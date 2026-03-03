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
    Edit(String),
    Delete(String),
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

            let hint = Paragraph::new(
                " Space reveal   c copy pass   u copy user   e edit   d delete   Esc back ",
            )
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
                KeyCode::Char('e') => return Ok(DetailAction::Edit(entry.id.clone())),
                KeyCode::Char('d') => {
                    if confirm_delete(term, &entry.title)? {
                        return Ok(DetailAction::Delete(entry.id.clone()));
                    }
                }
                _ => {}
            }
        }
    }
}

fn confirm_delete(
    term: &mut Terminal<CrosstermBackend<Stdout>>,
    title: &str,
) -> Result<bool, Box<dyn Error>> {
    loop {
        term.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title(" Confirm Delete ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Red));
            let area = centered_rect(50, 25, size);
            f.render_widget(block, area);

            let msg = format!("Delete \"{}\"?\n\n  y  confirm     n  cancel", title);
            let para = Paragraph::new(msg)
                .style(Style::default().fg(Color::Red))
                .alignment(Alignment::Center);
            f.render_widget(para, centered_rect(44, 18, size));
        })?;

        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Char('y') => return Ok(true),
                KeyCode::Char('n') | KeyCode::Esc => return Ok(false),
                _ => {}
            }
        }
    }
}
