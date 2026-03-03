use super::utils::{centered_rect, navigate_list};
use super::vault::Vault;
use crossterm::event::{read, Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::Alignment;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use ratatui::Terminal;
use std::error::Error;
use std::io::Stdout;

pub enum VaultListAction {
    Quit,
    View(usize),
    Add,
}

pub fn show(
    term: &mut Terminal<CrosstermBackend<Stdout>>,
    vault: &Vault,
) -> Result<VaultListAction, Box<dyn Error>> {
    let mut list_state = ListState::default();
    if !vault.entries.is_empty() {
        list_state.select(Some(0));
    }

    loop {
        term.draw(|f| {
            let size = f.size();

            let block = Block::default()
                .title(" IronKey — Vault ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Green));
            f.render_widget(block, centered_rect(80, 85, size));

            if vault.entries.is_empty() {
                let empty = Paragraph::new("No entries yet. Press 'a' to add one.")
                    .style(Style::default().fg(Color::DarkGray))
                    .alignment(Alignment::Center);
                f.render_widget(empty, centered_rect(60, 30, size));
            } else {
                let items: Vec<ListItem> = vault
                    .entries
                    .iter()
                    .map(|e| {
                        let line = format!("  {}  ·  {}", e.title, e.username);
                        ListItem::new(line)
                    })
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

                f.render_stateful_widget(list, centered_rect(74, 70, size), &mut list_state);
            }

            let hint = Paragraph::new(" ↑↓ navigate   Enter view   a add   q quit ")
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center);
            f.render_widget(hint, centered_rect(80, 8, size));
        })?;

        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Char('q') => return Ok(VaultListAction::Quit),
                KeyCode::Char('a') => return Ok(VaultListAction::Add),
                KeyCode::Up | KeyCode::Down => {
                    navigate_list(&mut list_state, vault.entries.len(), event.code);
                }
                KeyCode::Enter => {
                    if let Some(i) = list_state.selected() {
                        if !vault.entries.is_empty() {
                            return Ok(VaultListAction::View(i));
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
