use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub vault_path: Option<String>,
    pub clipboard_timeout_secs: u64,
    pub default_password_length: usize,
    pub lock_on_idle_secs: Option<u64>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            vault_path: None,
            clipboard_timeout_secs: 30,
            default_password_length: 20,
            lock_on_idle_secs: None,
        }
    }
}

impl Config {
    pub fn vault_path(&self) -> PathBuf {
        match &self.vault_path {
            Some(p) => PathBuf::from(p),
            None => {
                let mut path = dirs::home_dir().expect("could not find home directory");
                path.push(".ironkey");
                path.push("vault.json");
                path
            }
        }
    }
}

pub fn config_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("could not find home directory");
    path.push(".ironkey");
    path.push("config.toml");
    path
}

pub fn load() -> Config {
    let path = config_path();
    if !path.exists() {
        return Config::default();
    }
    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return Config::default(),
    };
    toml::from_str(&content).unwrap_or_default()
}

pub fn save(config: &Config) -> std::io::Result<()> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let content = toml::to_string_pretty(config)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    std::fs::write(path, content)
}
