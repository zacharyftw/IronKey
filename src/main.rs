mod mods;
use crossterm::terminal::{self, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use mods::auth::auth;
use mods::config;
use mods::entry_detail::{self, DetailAction};
use mods::entry_form;
use mods::state::{AppState, Screen};
use mods::utils::clear_clipboard;
use mods::vault::{self, add_entry, delete_entry, update_entry};
use mods::vault_list::{self, VaultListAction};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::stdout;
use zeroize::Zeroizing;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
    term.clear()
        .map_err(|e| format!("Failed to clear terminal: {}", e))?;

    let cfg = config::load();
    let mut state = AppState::new(cfg);

    loop {
        let screen = state.screen.clone();
        match screen {
            Screen::Auth => {
                let vault_path = state.config.vault_path();
                let (password, loaded_vault) = auth(&mut term, &vault_path)?;
                state.master_password = Zeroizing::new(password);
                state.vault = loaded_vault;
                state.screen = Screen::VaultList;
            }
            Screen::VaultList => {
                term.clear()?;
                let default_length = state.config.default_password_length;
                let idle = state.config.lock_on_idle_secs;
                match vault_list::show(&mut term, &state.vault, default_length, idle)? {
                    VaultListAction::Quit => break,
                    VaultListAction::View(i) => state.screen = Screen::EntryDetail(i),
                    VaultListAction::Add => state.screen = Screen::AddEntry,
                    VaultListAction::Lock => {
                        state.vault = Default::default();
                        state.master_password = Zeroizing::new(String::new());
                        state.screen = Screen::Auth;
                    }
                }
            }
            Screen::EntryDetail(i) => {
                term.clear()?;
                let Some(entry) = state.vault.entries.get(i).cloned() else {
                    state.screen = Screen::VaultList;
                    continue;
                };
                let timeout = state.config.clipboard_timeout_secs;
                let idle = state.config.lock_on_idle_secs;
                match entry_detail::show(&mut term, &entry, timeout, idle)? {
                    DetailAction::Back => state.screen = Screen::VaultList,
                    DetailAction::Edit(id) => state.screen = Screen::EditEntry(id),
                    DetailAction::Delete(id) => {
                        delete_entry(&mut state.vault, &id);
                        vault::save(
                            &state.config.vault_path(),
                            &state.master_password,
                            &state.vault,
                        )?;
                        state.screen = Screen::VaultList;
                    }
                    DetailAction::Lock => {
                        state.vault = Default::default();
                        state.master_password = Zeroizing::new(String::new());
                        state.screen = Screen::Auth;
                    }
                }
            }
            Screen::AddEntry => {
                term.clear()?;
                let default_length = state.config.default_password_length;
                if let Some(entry) = entry_form::show_add(&mut term, default_length)? {
                    add_entry(&mut state.vault, entry);
                    vault::save(
                        &state.config.vault_path(),
                        &state.master_password,
                        &state.vault,
                    )?;
                }
                state.screen = Screen::VaultList;
            }
            Screen::EditEntry(id) => {
                term.clear()?;
                let default_length = state.config.default_password_length;
                if let Some(existing) = state.vault.entries.iter().find(|e| e.id == id).cloned() {
                    if let Some(updated) =
                        entry_form::show_edit(&mut term, &existing, default_length)?
                    {
                        update_entry(&mut state.vault, &id, updated);
                        vault::save(
                            &state.config.vault_path(),
                            &state.master_password,
                            &state.vault,
                        )?;
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
