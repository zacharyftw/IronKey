mod mods;
use crossterm::event::read;
use crossterm::terminal::{self, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use mods::auth::auth;
use mods::s1::*;
use mods::s2::*;
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::ListState;
use ratatui::Terminal;
use std::io::stdout;
use std::vec;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
    term.clear()
        .map_err(|e| format!("Failed to clear terminal: {}", e))?;

    // auth screen
    let (_master_password, _vault) = auth(&mut term)?;

    term.clear()?;

    let mut pass_len: usize = 0;
    let mut input = String::new();
    let options = vec![
        "Uppercase",
        "Lowercase",
        "Numbers",
        "Special Characters",
        "Password Length",
    ];
    let mut selected_options = vec![false; options.len()];

    let mut list_state = ListState::default();
    if !options.is_empty() {
        list_state.select(Some(0));
    }
    //1st screen
    s1(
        &mut term,
        options,
        &mut selected_options,
        &mut list_state,
        &mut input,
        &mut pass_len,
    )?;

    term.clear()?;

    //2nd screen
    s2(selected_options, pass_len, &mut term, &mut read)?;

    let _ = stdout().execute(LeaveAlternateScreen);
    terminal::disable_raw_mode()?;
    Ok(())
}
