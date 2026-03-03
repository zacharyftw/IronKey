use super::config::Config;
use super::vault::Vault;
use zeroize::Zeroizing;

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
    pub master_password: Zeroizing<String>,
    pub screen: Screen,
    pub config: Config,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            vault: Vault::default(),
            master_password: Zeroizing::new(String::new()),
            screen: Screen::Auth,
            config,
        }
    }
}
