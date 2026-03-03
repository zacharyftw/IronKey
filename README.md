<h1 align="center">
<img width="200px" src="assets/logo.png" />

# IronKey

[![CI][s0]][l0] [![crates][s1]][l1] ![MIT][s2] [![UNSAFE][s3]][l3] [![deps][s4]][l4]

[s0]: https://github.com/KekmaTime/IronKey/workflows/CI/badge.svg
[l0]: https://github.com/KekmaTime/IronKey/actions
[s1]: https://img.shields.io/crates/v/ironkey.svg
[l1]: https://crates.io/crates/ironkey
[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[s3]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[l3]: https://github.com/rust-secure-code/safety-dance/
[s4]: https://deps.rs/repo/github/KekmaTime/IronKey/status.svg
[l4]: https://deps.rs/repo/github/KekmaTime/IronKey

</h1>

<h5 align="center">An encrypted terminal password manager — AES-256-GCM vault, Argon2id key derivation, fully keyboard-driven TUI</h5>

![demo](assets/demo.gif)

## Table of Contents

1. [Features](#features)
2. [Security Model](#security-model)
3. [Installation](#installation)
4. [Build](#build)
5. [Usage](#usage)
6. [Configuration](#configuration)
7. [Contributing](#contributing)

---

## 1. Features

- **Encrypted Vault** — AES-256-GCM encryption, key derived via Argon2id from your master password
- **Master Password** — vault locked on every launch; guided setup on first run
- **Full Entry Management** — add, edit, delete entries with title, username, password, URL, and notes
- **Password Generator** — configurable length and character types; accessible standalone or inline while editing
- **Live Search** — `/` to filter entries by title, username, or URL as you type
- **Clipboard Safety** — auto-clears clipboard after 30 seconds (configurable) with a live countdown; clears on exit
- **Idle Lock** — automatically locks vault after a configurable period of inactivity
- **Wayland & X11** — clipboard support for both display servers

---

## 2. Security Model

IronKey is a local, offline password manager. No data ever leaves your machine.

### Encryption

- **Algorithm:** AES-256-GCM (authenticated encryption — detects tampering)
- **Key derivation:** Argon2id with a random 16-byte salt generated on every save
- **Storage:** `~/.ironkey/vault.json` — base64-encoded salt, nonce, and ciphertext
- **File permissions:** vault file is written with `0600` permissions on Unix (owner read/write only)

### In-memory protections

- Master password is stored as `Zeroizing<String>` — memory is zeroed on drop (lock or exit)
- Derived key bytes are zeroized immediately after encrypt/decrypt

### Vault writes

- Writes go to a `.tmp` file first, then atomically renamed — a crash mid-save cannot corrupt your vault

### What IronKey does NOT protect against

- A compromised OS or keylogger capturing your master password at input
- Swap/hibernate files — consider enabling full-disk encryption on your machine
- Clipboard contents before the auto-clear timer fires (default: 30 seconds)

---

## 3. Installation

### From crates.io

```sh
cargo install ironkey
```

### From source

Requires Rust (see [Build](#build) for minimum version).

```sh
git clone https://github.com/KekmaTime/IronKey.git
cd IronKey
cargo build --release
./target/release/ironkey
```

### Wayland clipboard support

```sh
cargo install ironkey --features wayland_support
# or from source:
cargo build --release --features wayland_support
```

---

## 4. Build

**Minimum supported Rust version: 1.75**

Install Rust via [rustup](https://rustup.rs) if you don't have it:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```sh
cargo build            # debug
cargo build --release  # optimized
cargo test             # run tests
cargo clippy           # lint
```

---

## 5. Usage

```sh
ironkey
```

On first launch you will be prompted to set a master password. The vault is created at `~/.ironkey/vault.json`.

### Vault List

| Key | Action |
|---|---|
| `↑` / `↓` | Navigate entries |
| `Enter` | View entry |
| `a` | Add new entry |
| `g` | Open password generator |
| `/` | Search / filter |
| `Esc` | Clear search |
| `l` | Lock vault |
| `q` | Quit |

### Entry Detail

| Key | Action |
|---|---|
| `Space` | Reveal / hide password |
| `c` | Copy password to clipboard |
| `u` | Copy username to clipboard |
| `e` | Edit entry |
| `d` | Delete entry |
| `l` | Lock vault |
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

---

## 6. Configuration

IronKey reads `~/.ironkey/config.toml` on startup. All keys are optional and fall back to defaults.

```toml
vault_path = "~/.ironkey/vault.json"  # custom vault location
clipboard_timeout_secs = 30           # seconds before clipboard auto-clears
default_password_length = 20          # default length in the generator
lock_on_idle_secs = 300               # lock after N seconds of inactivity (omit to disable)
```

---

## 7. Contributing

Contributions are welcome — bug reports, feature requests, and pull requests.

1. Fork the repo and create a branch from `master`
2. Make your changes — run `cargo fmt` and `cargo clippy` before committing
3. Open a pull request with a short description of what you changed and why

Please follow [conventional commits](https://www.conventionalcommits.org/) style for commit messages (`feat:`, `fix:`, `chore:`, etc.).

If you find a **security issue**, please open a GitHub issue marked `[security]` rather than a public PR.
