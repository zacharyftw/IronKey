use super::utils::{centered_rect, clear_clipboard, set_clipboard_content};
use super::vault::VaultEntry;
use crossterm::event::{poll, read, Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::Alignment;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Terminal;
use std::error::Error;
use std::io::Stdout;
use std::time::{Duration, Instant};

pub enum DetailAction {
    Back,
    Edit(String),
    Delete(String),
    Lock,
}

pub fn show(
    term: &mut Terminal<CrosstermBackend<Stdout>>,
    entry: &VaultEntry,
    clipboard_timeout_secs: u64,
    idle_timeout_secs: Option<u64>,
) -> Result<DetailAction, Box<dyn Error>> {
    let mut reveal = false;
    let mut status = String::new();
    let mut copy_time: Option<Instant> = None;
    let mut last_activity = Instant::now();

    loop {
        // idle lock check
        if let Some(timeout) = idle_timeout_secs {
            if last_activity.elapsed().as_secs() >= timeout {
                clear_clipboard();
                return Ok(DetailAction::Lock);
            }
        }

        // auto-clear clipboard when timeout expires
        if let Some(t) = copy_time {
            if t.elapsed().as_secs() >= clipboard_timeout_secs {
                clear_clipboard();
                copy_time = None;
                status = "Clipboard cleared.".to_string();
            }
        }

        let password_display = if reveal {
            entry.password.clone()
        } else {
            "*".repeat(entry.password.chars().count())
        };

        // build status with countdown if active
        let status_display = if let Some(t) = copy_time {
            let remaining = clipboard_timeout_secs.saturating_sub(t.elapsed().as_secs());
            format!("{}  (clears in {}s)", status, remaining)
        } else {
            status.clone()
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

            if !status_display.is_empty() {
                let status_para = Paragraph::new(status_display.as_str())
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

        // poll with 1s timeout so the countdown updates every second
        if poll(Duration::from_millis(1000))? {
            if let Event::Key(event) = read()? {
                last_activity = Instant::now();
                match event.code {
                    KeyCode::Esc => {
                        clear_clipboard();
                        return Ok(DetailAction::Back);
                    }
                    KeyCode::Char('l') => {
                        clear_clipboard();
                        return Ok(DetailAction::Lock);
                    }
                    KeyCode::Char(' ') => {
                        reveal = !reveal;
                        status.clear();
                    }
                    KeyCode::Char('c') => match set_clipboard_content(&entry.password) {
                        Ok(_) => {
                            copy_time = Some(Instant::now());
                            status = "Password copied!".to_string();
                        }
                        Err(e) => status = e,
                    },
                    KeyCode::Char('u') => match set_clipboard_content(&entry.username) {
                        Ok(_) => {
                            copy_time = Some(Instant::now());
                            status = "Username copied!".to_string();
                        }
                        Err(e) => status = e,
                    },
                    KeyCode::Char('e') => {
                        clear_clipboard();
                        return Ok(DetailAction::Edit(entry.id.clone()));
                    }
                    KeyCode::Char('d') => {
                        if confirm_delete(term, &entry.title)? {
                            clear_clipboard();
                            return Ok(DetailAction::Delete(entry.id.clone()));
                        }
                    }
                    _ => {}
                }
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
