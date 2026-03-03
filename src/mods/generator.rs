use super::passgen::passgen;
use super::utils::{centered_rect, set_clipboard_content};
use crossterm::event::{read, Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::Alignment;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Terminal;
use std::error::Error;
use std::io::Stdout;

const OPT_LABELS: [&str; 4] = ["Uppercase", "Lowercase", "Numbers", "Special Characters"];

// Returns Some(password) when confirmed, None on cancel.
pub fn show(
    term: &mut Terminal<CrosstermBackend<Stdout>>,
    default_length: usize,
) -> Result<Option<String>, Box<dyn Error>> {
    let mut selected = [true, true, true, true];
    let mut length_input = default_length.to_string();
    let mut focused: usize = 0; // 0-3 char types, 4 length
    let mut generated: Option<String> = None;
    let mut status = String::new();

    loop {
        term.draw(|f| {
            let size = f.area();
            let form_area = centered_rect(55, 90, size);

            let outer = Block::default()
                .title(" Generate Password ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Green));
            f.render_widget(outer, form_area);

            let rows = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3), // Uppercase
                    Constraint::Length(3), // Lowercase
                    Constraint::Length(3), // Numbers
                    Constraint::Length(3), // Special
                    Constraint::Length(3), // Length
                    Constraint::Length(2), // generated password
                    Constraint::Length(1), // status
                    Constraint::Length(1), // hint
                    Constraint::Min(0),
                ])
                .split(form_area);

            for (i, label) in OPT_LABELS.iter().enumerate() {
                let is_focused = focused == i;
                let checkbox = if selected[i] { "[✓]" } else { "[ ]" };
                let block = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(if is_focused {
                        Color::Green
                    } else {
                        Color::DarkGray
                    }));
                let para = Paragraph::new(format!(" {} {}", checkbox, label))
                    .block(block)
                    .style(
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(if is_focused {
                                Modifier::BOLD
                            } else {
                                Modifier::empty()
                            }),
                    );
                f.render_widget(para, rows[i]);
            }

            let len_block = Block::default()
                .title(" Length ")
                .borders(Borders::ALL)
                .style(Style::default().fg(if focused == 4 {
                    Color::Green
                } else {
                    Color::DarkGray
                }));
            let len_para = Paragraph::new(length_input.as_str())
                .block(len_block)
                .style(Style::default().fg(Color::Green));
            f.render_widget(len_para, rows[4]);

            if let Some(ref pw) = generated {
                let pw_display = format!(" {}", pw);
                let pw_para = Paragraph::new(pw_display)
                    .style(
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    )
                    .alignment(Alignment::Center);
                f.render_widget(pw_para, rows[5]);
            }

            if !status.is_empty() {
                let status_para = Paragraph::new(status.as_str())
                    .style(Style::default().fg(Color::Red))
                    .alignment(Alignment::Center);
                f.render_widget(status_para, rows[6]);
            }

            let hint = if generated.is_some() {
                " Enter use   r regenerate   c copy   Esc cancel "
            } else {
                " Tab next   Space toggle   Enter generate   Esc cancel "
            };
            let hint_para = Paragraph::new(hint)
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center);
            f.render_widget(hint_para, rows[7]);
        })?;

        if let Event::Key(event) = read()? {
            if event.kind != crossterm::event::KeyEventKind::Press {
                continue;
            }
            match event.code {
                KeyCode::Esc => return Ok(None),
                KeyCode::Enter => {
                    if let Some(ref pw) = generated {
                        return Ok(Some(pw.clone()));
                    }
                    let len = length_input.parse().unwrap_or(20);
                    match passgen(selected, len) {
                        Ok(pw) => {
                            generated = Some(pw);
                            status.clear();
                        }
                        Err(e) => status = e.to_string(),
                    }
                }
                KeyCode::Char('r') => {
                    let len = length_input.parse().unwrap_or(20);
                    match passgen(selected, len) {
                        Ok(pw) => {
                            generated = Some(pw);
                            status.clear();
                        }
                        Err(e) => status = e.to_string(),
                    }
                }
                KeyCode::Char('c') => {
                    if let Some(ref pw) = generated {
                        match set_clipboard_content(pw) {
                            Ok(_) => status = "Copied!".to_string(),
                            Err(e) => status = e,
                        }
                    }
                }
                KeyCode::Tab => {
                    focused = (focused + 1) % 5;
                    status.clear();
                }
                KeyCode::BackTab => {
                    focused = (focused + 4) % 5;
                    status.clear();
                }
                KeyCode::Char(' ') if focused < 4 => {
                    selected[focused] = !selected[focused];
                }
                KeyCode::Char(c) if focused == 4 && c.is_ascii_digit() => {
                    length_input.push(c);
                }
                KeyCode::Backspace if focused == 4 && !length_input.is_empty() => {
                    length_input.pop();
                }
                _ => {}
            }
        }
    }
}
