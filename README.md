# IronKey

IronKey is a Terminal User Interface (TUI) based encrypted password manager written in Rust.

## Features

- **Encrypted Vault** — passwords stored with AES-256-GCM encryption, key derived via Argon2id from your master password
- **Master Password** — vault locked behind a master password; create on first run, unlock on every launch
- **Full Entry Management** — add, edit, and delete entries with title, username, password, URL, and notes
- **Password Generator** — configurable generator (length, character types) accessible standalone or inline when editing entries
- **Live Search** — `/` to filter entries by title, username, or URL as you type
- **Clipboard Safety** — auto-clears clipboard after 30 seconds (configurable) with a live countdown; also clears on exit
- **Wayland & X11** — clipboard support for both display servers

## Installation

Requires Rust and Git.

```sh
git clone https://github.com/KekmaTime/IronKey.git
cd IronKey
cargo build --release
```

or

```sh
cargo install ironkey
```

## Usage

```sh
ironkey
```

On first launch you will be prompted to set a master password and a vault will be created at `~/.ironkey/vault.json`.

### Vault List

| Key | Action |
|---|---|
| `↑` / `↓` | Navigate entries |
| `Enter` | View entry |
| `a` | Add new entry |
| `g` | Open password generator |
| `/` | Search / filter |
| `Esc` | Clear search |
| `q` | Quit |

### Entry Detail

| Key | Action |
|---|---|
| `Space` | Reveal / hide password |
| `c` | Copy password to clipboard |
| `u` | Copy username to clipboard |
| `e` | Edit entry |
| `d` | Delete entry |
| `Esc` | Back to vault list |

### Entry Form (Add / Edit)

| Key | Action |
|---|---|
| `Tab` / `Shift+Tab` | Move between fields |
| `g` | Generate password (on password field) |
| `Enter` | Save |
| `Esc` | Cancel |

### Password Generator

| Key | Action |
|---|---|
| `Tab` / `Shift+Tab` | Move between options |
| `Space` | Toggle character type |
| `Enter` | Generate → confirm |
| `r` | Regenerate |
| `c` | Copy to clipboard |
| `Esc` | Cancel |

## Configuration

IronKey reads `~/.ironkey/config.toml` on startup. Missing keys fall back to defaults.

```toml
vault_path = "~/.ironkey/vault.json"  # custom vault location
clipboard_timeout_secs = 30           # seconds before clipboard auto-clears
default_password_length = 20          # default length in the generator
lock_on_idle_secs = 300               # idle lock (optional)
```
