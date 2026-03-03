use super::generator;
use super::utils::centered_rect;
use super::vault::{new_entry, VaultEntry};
use crossterm::event::{read, Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::Alignment;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Terminal;
use std::error::Error;
use std::io::Stdout;

const LABELS: [&str; 5] = ["Title", "Username", "Password", "URL", "Notes"];

pub fn show_add(
    term: &mut Terminal<CrosstermBackend<Stdout>>,
    default_length: usize,
) -> Result<Option<VaultEntry>, Box<dyn Error>> {
    show_form(term, None, default_length)
}

pub fn show_edit(
    term: &mut Terminal<CrosstermBackend<Stdout>>,
    entry: &VaultEntry,
    default_length: usize,
) -> Result<Option<VaultEntry>, Box<dyn Error>> {
    show_form(term, Some(entry), default_length)
}

fn show_form(
    term: &mut Terminal<CrosstermBackend<Stdout>>,
    existing: Option<&VaultEntry>,
    default_length: usize,
) -> Result<Option<VaultEntry>, Box<dyn Error>> {
    let mut fields: [String; 5] = if let Some(e) = existing {
        [
            e.title.clone(),
            e.username.clone(),
            e.password.clone(),
            e.url.clone(),
            e.notes.clone(),
        ]
    } else {
        Default::default()
    };

    let mut focused: usize = 0;
    let mut status = String::new();
    let title = if existing.is_some() {
        " Edit Entry "
    } else {
        " New Entry "
    };

    loop {
        term.draw(|f| {
            let size = f.area();
            let form_area = centered_rect(60, 90, size);

            let outer = Block::default()
                .title(title)
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Green));
            f.render_widget(outer, form_area);

            let inner = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Min(0),
                ])
                .split(form_area);

            for (i, label) in LABELS.iter().enumerate() {
                let is_focused = i == focused;
                let value = &fields[i];
                let display = if *label == "Password" {
                    "*".repeat(value.chars().count())
                } else {
                    value.clone()
                };

                let block = Block::default()
                    .title(format!(" {} ", label))
                    .borders(Borders::ALL)
                    .style(Style::default().fg(if is_focused {
                        Color::Green
                    } else {
                        Color::DarkGray
                    }));

                let para = Paragraph::new(display).block(block).style(
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(if is_focused {
                            Modifier::BOLD
                        } else {
                            Modifier::empty()
                        }),
                );
                f.render_widget(para, inner[i]);
            }

            if !status.is_empty() {
                let status_para = Paragraph::new(status.as_str())
                    .style(Style::default().fg(Color::Red))
                    .alignment(Alignment::Center);
                f.render_widget(status_para, inner[5]);
            }

            let hint = if focused == 2 {
                " Tab next   Esc cancel   Enter save   g generate password "
            } else {
                " Tab next   Esc cancel   Enter save "
            };
            let hint_para = Paragraph::new(hint)
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center);
            f.render_widget(hint_para, inner[6]);
        })?;

        if let Event::Key(event) = read()? {
            if event.kind != crossterm::event::KeyEventKind::Press {
                continue;
            }
            match event.code {
                KeyCode::Esc => return Ok(None),
                KeyCode::Tab => {
                    focused = (focused + 1) % LABELS.len();
                    status.clear();
                }
                KeyCode::BackTab => {
                    focused = (focused + LABELS.len() - 1) % LABELS.len();
                    status.clear();
                }
                KeyCode::Enter => {
                    if fields[0].trim().is_empty() {
                        status = "Title is required.".to_string();
                    } else {
                        let entry = if let Some(e) = existing {
                            let mut updated = new_entry(
                                &fields[0], &fields[1], &fields[2], &fields[3], &fields[4],
                            );
                            updated.id = e.id.clone();
                            updated.created_at = e.created_at.clone();
                            updated
                        } else {
                            new_entry(&fields[0], &fields[1], &fields[2], &fields[3], &fields[4])
                        };
                        return Ok(Some(entry));
                    }
                }
                KeyCode::Char('g') if focused == 2 => {
                    if let Some(pw) = generator::show(term, default_length)? {
                        fields[2] = pw;
                        status = "Password generated!".to_string();
                    }
                }
                KeyCode::Char(c) => {
                    fields[focused].push(c);
                    status.clear();
                }
                KeyCode::Backspace if !fields[focused].is_empty() => {
                    fields[focused].pop();
                    status.clear();
                }
                _ => {}
            }
        }
    }
}
