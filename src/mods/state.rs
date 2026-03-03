use super::vault::Vault;

#[derive(Clone)]
pub enum Screen {
    Auth,
    VaultList,
    EntryDetail(usize),
    AddEntry,
    EditEntry(String),
}

pub struct AppState {
    pub vault: Vault,
    pub master_password: String,
    pub screen: Screen,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            vault: Vault::default(),
            master_password: String::new(),
            screen: Screen::Auth,
        }
    }
}
