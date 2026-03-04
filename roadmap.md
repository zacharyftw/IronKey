# IronKey Roadmap

## Phase 1 — Clipboard & Lock Security

- [ ] Clipboard auto-clear after N seconds (configurable via `config.toml`)
- [ ] Vault auto-lock after idle timeout — re-prompt master password

## Phase 2 — Password Intelligence

- [ ] Password strength meter — show entropy/strength rating when generating or viewing passwords
- [ ] Duplicate/weak password detection — warn if same password used across entries
- [ ] Password age tracking — flag old/stale passwords that should be rotated

## Phase 3 — TOTP / 2FA

- [ ] Store TOTP secrets in vault entries
- [ ] Generate and display 6-digit TOTP codes with countdown timer

## Phase 4 — Import & Export

- [ ] Import from Bitwarden CSV
- [ ] Import from KeePass XML
- [ ] Import from 1Password export
- [ ] Export vault to encrypted backup format

## Phase 5 — Organization

- [ ] Categories/tags for entries
- [ ] Fuzzy search (replace exact substring match)
- [ ] Customizable keybindings via config

## Phase 6 — CLI Mode

- [ ] Non-interactive CLI — `ironkey get github --field password`
- [ ] Shell completions (fish/bash/zsh)

## Phase 7 — Advanced

- [ ] Multiple vaults support
- [ ] Breach check — query HaveIBeenPwned API for compromised passwords
