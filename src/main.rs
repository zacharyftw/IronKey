mod mods;
use crossterm::terminal::{self, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use mods::auth::auth;
use mods::entry_detail::{self, DetailAction};
use mods::state::{AppState, Screen};
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

    let mut state = AppState::new();

    loop {
        let screen = state.screen.clone();
        match screen {
            Screen::Auth => {
                let (password, vault) = auth(&mut term)?;
                state.master_password = password;
                state.vault = vault;
                state.screen = Screen::VaultList;
            }
            Screen::VaultList => {
                term.clear()?;
                match vault_list::show(&mut term, &state.vault)? {
                    VaultListAction::Quit => break,
                    VaultListAction::View(i) => state.screen = Screen::EntryDetail(i),
                }
            }
            Screen::EntryDetail(i) => {
                term.clear()?;
                match entry_detail::show(&mut term, &state.vault.entries[i])? {
                    DetailAction::Back => state.screen = Screen::VaultList,
                }
            }
            Screen::AddEntry | Screen::EditEntry(_) => {
                // Phase 5
                state.screen = Screen::VaultList;
            }
        }
    }

    let _ = stdout().execute(LeaveAlternateScreen);
    terminal::disable_raw_mode()?;
    Ok(())
}
