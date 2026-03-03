use rand::RngCore;
use std::{fs, path::PathBuf};

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use argon2::Argon2;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

#[derive(Serialize, Deserialize, Clone)]
pub struct VaultEntry {
    pub id: String,
    pub title: String,
    pub username: String,
    pub password: String,
    pub url: String,
    pub notes: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Vault {
    pub entries: Vec<VaultEntry>,
}

#[derive(Serialize, Deserialize)]
struct VaultFile {
    salt: String,
    nonce: String,
    ciphertext: String,
}

pub fn vault_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("could not find home directory");
    path.push(".ironkey");
    path.push("vault.json");
    path
}

pub fn load(path: &PathBuf, master_password: &str) -> std::io::Result<Vault> {
    let data = fs::read_to_string(path)?;
    let vault_file: VaultFile = serde_json::from_str(&data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    let salt = BASE64
        .decode(&vault_file.salt)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
    let nonce = BASE64
        .decode(&vault_file.nonce)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
    let ciphertext = BASE64
        .decode(&vault_file.ciphertext)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;

    let mut key = derive_key(master_password, &salt);
    let plaintext = decrypt(&key, &nonce, &ciphertext).map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, "wrong master password")
    })?;
    key.zeroize();

    let vault: Vault = serde_json::from_slice(&plaintext)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    Ok(vault)
}

pub fn save(path: &PathBuf, master_password: &str, vault: &Vault) -> std::io::Result<()> {
    let plaintext = serde_json::to_vec(vault)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    let mut salt = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt);
    let mut key = derive_key(master_password, &salt);
    let (nonce, ciphertext) = encrypt(&key, &plaintext);
    key.zeroize();

    let vault_file = VaultFile {
        salt: BASE64.encode(salt),
        nonce: BASE64.encode(&nonce),
        ciphertext: BASE64.encode(&ciphertext),
    };

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(&vault_file)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    fs::write(path, json)
}

fn derive_key(master_password: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(master_password.as_bytes(), salt, &mut key)
        .expect("argon2 key derivation failed");
    key
}

fn encrypt(key_bytes: &[u8; 32], plaintext: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key_bytes));
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .expect("encryption failed");
    (nonce.to_vec(), ciphertext)
}

fn decrypt(
    key_bytes: &[u8; 32],
    nonce: &[u8],
    ciphertext: &[u8],
) -> Result<Vec<u8>, aes_gcm::Error> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key_bytes));
    cipher.decrypt(Nonce::from_slice(nonce), ciphertext)
}
