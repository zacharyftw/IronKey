mod mods;
use crossterm::terminal::{self, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use mods::auth::auth;
use mods::entry_detail::{self, DetailAction};
use mods::vault_list::{self, VaultListAction};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::stdout;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
    term.clear()
        .map_err(|e| format!("Failed to clear terminal: {}", e))?;

    let (_master_password, vault) = auth(&mut term)?;

    term.clear()?;

    loop {
        match vault_list::show(&mut term, &vault)? {
            VaultListAction::Quit => break,
            VaultListAction::View(i) => {
                term.clear()?;
                match entry_detail::show(&mut term, &vault.entries[i])? {
                    DetailAction::Back => {}
                }
                term.clear()?;
            }
        }
    }

    let _ = stdout().execute(LeaveAlternateScreen);
    terminal::disable_raw_mode()?;
    Ok(())
}
