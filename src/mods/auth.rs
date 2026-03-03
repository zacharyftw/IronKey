use super::utils::centered_rect;
use super::vault::{self, Vault};
use crossterm::event::{read, Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::Alignment;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Terminal;
use std::error::Error;
use std::io::Stdout;

pub fn auth(
    term: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(String, Vault), Box<dyn Error>> {
    let path = vault::vault_path();
    if path.exists() {
        unlock_vault(term)
    } else {
        create_vault(term)
    }
}

fn draw_input(
    term: &mut Terminal<CrosstermBackend<Stdout>>,
    title: &str,
    input: &str,
    status: &str,
) -> Result<(), Box<dyn Error>> {
    let masked = "*".repeat(input.chars().count());
    term.draw(|f| {
        let size = f.size();

        let title_block = Block::default()
            .title("IronKey")
            .title_alignment(Alignment::Center)
            .borders(Borders::NONE)
            .style(Style::default().fg(Color::Green));
        f.render_widget(title_block, centered_rect(60, 50, size));

        let input_block = Block::default()
            .title(title)
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Green));
        let area = centered_rect(50, 20, size);
        f.render_widget(input_block, area);

        let input_paragraph =
            Paragraph::new(masked.as_str()).style(Style::default().fg(Color::Green));
        f.render_widget(input_paragraph, centered_rect(46, 10, size));

        if !status.is_empty() {
            let status_paragraph = Paragraph::new(status)
                .style(Style::default().fg(Color::Red))
                .alignment(Alignment::Center);
            f.render_widget(status_paragraph, centered_rect(50, 5, size));
        }
    })?;
    Ok(())
}

fn read_password(
    term: &mut Terminal<CrosstermBackend<Stdout>>,
    title: &str,
    status: &str,
) -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    loop {
        draw_input(term, title, &input, status)?;
        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Char(c) => input.push(c),
                KeyCode::Backspace if !input.is_empty() => {
                    input.pop();
                }
                KeyCode::Enter => break,
                _ => {}
            }
        }
    }
    Ok(input)
}

fn create_vault(
    term: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(String, Vault), Box<dyn Error>> {
    loop {
        let password = read_password(term, " Set Master Password ", "")?;
        if password.is_empty() {
            continue;
        }
        let confirm = read_password(term, " Confirm Master Password ", "")?;
        if password != confirm {
            read_password(
                term,
                " Confirm Master Password ",
                "Passwords do not match. Press Enter to retry.",
            )?;
            continue;
        }
        let vault = Vault::default();
        let path = vault::vault_path();
        vault::save(&path, &password, &vault)?;
        return Ok((password, vault));
    }
}

fn unlock_vault(
    term: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(String, Vault), Box<dyn Error>> {
    let mut status = String::new();
    loop {
        let password = read_password(term, " Enter Master Password ", &status)?;
        let path = vault::vault_path();
        match vault::load(&path, &password) {
            Ok(vault) => return Ok((password, vault)),
            Err(_) => {
                status = "Wrong password. Try again.".to_string();
            }
        }
    }
}
