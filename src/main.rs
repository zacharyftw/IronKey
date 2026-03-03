mod mods;
use crossterm::terminal::{self, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use mods::auth::auth;
use mods::entry_detail::{self, DetailAction};
use mods::entry_form;
use mods::state::{AppState, Screen};
use mods::utils::clear_clipboard;
use mods::vault::{self, add_entry, delete_entry, update_entry};
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
                let (password, loaded_vault) = auth(&mut term)?;
                state.master_password = password;
                state.vault = loaded_vault;
                state.screen = Screen::VaultList;
            }
            Screen::VaultList => {
                term.clear()?;
                match vault_list::show(&mut term, &state.vault)? {
                    VaultListAction::Quit => break,
                    VaultListAction::View(i) => state.screen = Screen::EntryDetail(i),
                    VaultListAction::Add => state.screen = Screen::AddEntry,
                }
            }
            Screen::EntryDetail(i) => {
                term.clear()?;
                let entry = state.vault.entries[i].clone();
                match entry_detail::show(&mut term, &entry)? {
                    DetailAction::Back => state.screen = Screen::VaultList,
                    DetailAction::Edit(id) => state.screen = Screen::EditEntry(id),
                    DetailAction::Delete(id) => {
                        delete_entry(&mut state.vault, &id);
                        vault::save(&vault::vault_path(), &state.master_password, &state.vault)?;
                        state.screen = Screen::VaultList;
                    }
                }
            }
            Screen::AddEntry => {
                term.clear()?;
                if let Some(entry) = entry_form::show_add(&mut term)? {
                    add_entry(&mut state.vault, entry);
                    vault::save(&vault::vault_path(), &state.master_password, &state.vault)?;
                }
                state.screen = Screen::VaultList;
            }
            Screen::EditEntry(id) => {
                term.clear()?;
                if let Some(pos) = state.vault.entries.iter().position(|e| e.id == id) {
                    let existing = state.vault.entries[pos].clone();
                    if let Some(updated) = entry_form::show_edit(&mut term, &existing)? {
                        update_entry(&mut state.vault, &id, updated);
                        vault::save(&vault::vault_path(), &state.master_password, &state.vault)?;
                    }
                }
                state.screen = Screen::VaultList;
            }
        }
    }

    clear_clipboard();
    let _ = stdout().execute(LeaveAlternateScreen);
    terminal::disable_raw_mode()?;
    Ok(())
}
