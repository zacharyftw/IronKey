use super::utils::{centered_rect, set_clipboard_content};
use super::vault::VaultEntry;
use crossterm::event::{read, Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::Alignment;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Terminal;
use std::error::Error;
use std::io::Stdout;

pub enum DetailAction {
    Back,
}

pub fn show(
    term: &mut Terminal<CrosstermBackend<Stdout>>,
    entry: &VaultEntry,
) -> Result<DetailAction, Box<dyn Error>> {
    let mut reveal = false;
    let mut status = String::new();

    loop {
        let password_display = if reveal {
            entry.password.clone()
        } else {
            "*".repeat(entry.password.chars().count())
        };

        term.draw(|f| {
            let size = f.size();

            let block = Block::default()
                .title(format!("  {}  ", entry.title))
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Green));
            f.render_widget(block, centered_rect(70, 85, size));

            let content = format!(
                "Title     {}\n\nUsername  {}\n\nPassword  {}\n\nURL       {}\n\nNotes     {}\n\nCreated   {}\nUpdated   {}",
                entry.title,
                entry.username,
                password_display,
                if entry.url.is_empty() { "—" } else { &entry.url },
                if entry.notes.is_empty() { "—" } else { &entry.notes },
                entry.created_at,
                entry.updated_at,
            );

            let detail = Paragraph::new(content).style(Style::default().fg(Color::Green));
            f.render_widget(detail, centered_rect(62, 65, size));

            if !status.is_empty() {
                let status_para = Paragraph::new(status.as_str())
                    .style(Style::default().fg(Color::Yellow))
                    .alignment(Alignment::Center);
                f.render_widget(status_para, centered_rect(70, 15, size));
            }

            let hint =
                Paragraph::new(" Space reveal   c copy password   u copy username   Esc back ")
                    .style(Style::default().fg(Color::DarkGray))
                    .alignment(Alignment::Center);
            f.render_widget(hint, centered_rect(70, 6, size));
        })?;

        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Esc => return Ok(DetailAction::Back),
                KeyCode::Char(' ') => {
                    reveal = !reveal;
                    status.clear();
                }
                KeyCode::Char('c') => match set_clipboard_content(&entry.password) {
                    Ok(_) => status = "Password copied to clipboard!".to_string(),
                    Err(e) => status = e,
                },
                KeyCode::Char('u') => match set_clipboard_content(&entry.username) {
                    Ok(_) => status = "Username copied to clipboard!".to_string(),
                    Err(e) => status = e,
                },
                _ => {}
            }
        }
    }
}
