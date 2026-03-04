# Contributing to IronKey

Thanks for your interest in contributing! Here's how to get started.

## Getting Started

1. Fork the repo and clone your fork
2. Install Rust (stable toolchain): https://rustup.rs
3. Build and run tests:

```sh
cargo build
cargo test
cargo clippy --all-features
cargo fmt --check
```

## Making Changes

1. Create a branch from `master`
2. Make your changes
3. Ensure all checks pass:
   - `cargo test` — all tests pass
   - `cargo clippy --all-features` — no warnings
   - `cargo fmt` — code is formatted
4. Write clear, concise commit messages using [conventional commits](https://www.conventionalcommits.org/) (e.g. `fix: resolve clipboard crash on wayland`)
5. Open a pull request against `master`

## Wayland Support

To build with Wayland clipboard support:

```sh
cargo build --features wayland_support
```

## Project Structure

```
src/
├── main.rs              # Entry point, terminal init, screen dispatch
└── mods/
    ├── auth.rs          # Master password authentication screen
    ├── config.rs        # Config file (~/.ironkey/config.toml)
    ├── entry_detail.rs  # Single entry view
    ├── entry_form.rs    # Add/edit entry form
    ├── generator.rs     # Password generator screen
    ├── passgen.rs       # Password generation logic
    ├── state.rs         # AppState, Screen enum, VaultEntry
    ├── utils.rs         # Shared helpers
    ├── vault.rs         # Encrypted vault (AES-256-GCM + Argon2id)
    └── vault_list.rs    # Vault list with search/filter
```

## Reporting Bugs

Open an issue with:
- Steps to reproduce
- Expected vs actual behavior
- OS and Rust version (`rustc --version`)

## Feature Requests

Check the [roadmap](roadmap.md) first. If your idea isn't listed, open an issue describing the use case.

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).
