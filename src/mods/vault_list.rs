use super::generator;
use super::utils::{centered_rect, clear_clipboard, navigate_list, set_clipboard_content};
use super::vault::Vault;
use crossterm::event::{poll, read, Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::Alignment;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use ratatui::Terminal;
use std::error::Error;
use std::io::Stdout;
use std::time::{Duration, Instant};

pub enum VaultListAction {
    Quit,
    View(usize),
    Add,
    Lock,
}

pub fn show(
    term: &mut Terminal<CrosstermBackend<Stdout>>,
    vault: &Vault,
    default_length: usize,
    idle_timeout_secs: Option<u64>,
) -> Result<VaultListAction, Box<dyn Error>> {
    let mut list_state = ListState::default();
    let mut status = String::new();
    let mut search_query = String::new();
    let mut search_mode = false;
    let mut last_activity = Instant::now();

    if !vault.entries.is_empty() {
        list_state.select(Some(0));
    }

    loop {
        // idle lock check
        if let Some(timeout) = idle_timeout_secs {
            if last_activity.elapsed().as_secs() >= timeout {
                clear_clipboard();
                return Ok(VaultListAction::Lock);
            }
        }

        let filtered: Vec<(String, usize)> = vault
            .entries
            .iter()
            .enumerate()
            .filter(|(_, e)| {
                if search_query.is_empty() {
                    return true;
                }
                let q = search_query.to_lowercase();
                e.title.to_lowercase().contains(&q)
                    || e.username.to_lowercase().contains(&q)
                    || e.url.to_lowercase().contains(&q)
            })
            .map(|(i, e)| (format!("  {}  ·  {}", e.title, e.username), i))
            .collect();

        if filtered.is_empty() {
            list_state.select(None);
        } else if list_state.selected().is_none_or(|s| s >= filtered.len()) {
            list_state.select(Some(0));
        }

        term.draw(|f| {
            let size = f.area();

            let block = Block::default()
                .title(" IronKey — Vault ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Green));
            f.render_widget(block, centered_rect(80, 85, size));

            let search_display = if search_mode {
                format!(" / {}_", search_query)
            } else if !search_query.is_empty() {
                format!(" / {}  (Esc to clear)", search_query)
            } else {
                String::new()
            };
            if !search_display.is_empty() {
                let search_para = Paragraph::new(search_display.as_str())
                    .style(Style::default().fg(Color::Yellow));
                f.render_widget(search_para, centered_rect(74, 8, size));
            }

            if filtered.is_empty() {
                let msg = if vault.entries.is_empty() {
                    "No entries yet. Press 'a' to add one."
                } else {
                    "No matches."
                };
                let empty = Paragraph::new(msg)
                    .style(Style::default().fg(Color::DarkGray))
                    .alignment(Alignment::Center);
                f.render_widget(empty, centered_rect(60, 30, size));
            } else {
                let items: Vec<ListItem> = filtered
                    .iter()
                    .map(|(line, _)| ListItem::new(line.clone()))
                    .collect();

                let list = List::new(items)
                    .style(Style::default().fg(Color::Green))
                    .highlight_style(
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol("❯ ");

                f.render_stateful_widget(list, centered_rect(74, 65, size), &mut list_state);
            }

            if !status.is_empty() {
                let status_para = Paragraph::new(status.as_str())
                    .style(Style::default().fg(Color::Yellow))
                    .alignment(Alignment::Center);
                f.render_widget(status_para, centered_rect(80, 6, size));
            }

            let hint = if search_mode {
                " Type to filter   Esc exit search "
            } else {
                " ↑↓ navigate   Enter view   a add   g generate   / search   l lock   q quit "
            };
            let hint_para = Paragraph::new(hint)
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center);
            f.render_widget(hint_para, centered_rect(80, 5, size));
        })?;

        if poll(Duration::from_millis(1000))? {
            if let Event::Key(event) = read()? {
                if event.kind != crossterm::event::KeyEventKind::Press {
                    continue;
                }
                last_activity = Instant::now();
                if search_mode {
                    match event.code {
                        KeyCode::Esc => {
                            search_mode = false;
                            search_query.clear();
                            if !vault.entries.is_empty() {
                                list_state.select(Some(0));
                            }
                        }
                        KeyCode::Backspace if !search_query.is_empty() => {
                            search_query.pop();
                        }
                        KeyCode::Enter => {
                            search_mode = false;
                        }
                        KeyCode::Char(c) => {
                            search_query.push(c);
                            list_state.select(if filtered.is_empty() { None } else { Some(0) });
                        }
                        _ => {}
                    }
                } else {
                    match event.code {
                        KeyCode::Char('q') => return Ok(VaultListAction::Quit),
                        KeyCode::Char('a') => return Ok(VaultListAction::Add),
                        KeyCode::Char('l') => {
                            clear_clipboard();
                            return Ok(VaultListAction::Lock);
                        }
                        KeyCode::Char('/') => {
                            search_mode = true;
                            status.clear();
                        }
                        KeyCode::Char('g') => {
                            if let Some(pw) = generator::show(term, default_length)? {
                                match set_clipboard_content(&pw) {
                                    Ok(_) => status = "Password generated and copied!".to_string(),
                                    Err(_) => status = "Password generated.".to_string(),
                                }
                            }
                            term.clear()?;
                        }
                        KeyCode::Up | KeyCode::Down => {
                            navigate_list(&mut list_state, filtered.len(), event.code);
                        }
                        KeyCode::Enter => {
                            if let Some(i) = list_state.selected() {
                                if let Some((_, original_idx)) = filtered.get(i) {
                                    return Ok(VaultListAction::View(*original_idx));
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
