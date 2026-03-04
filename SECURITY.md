# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.2.x   | Yes       |
| < 0.2   | No        |

## Security Model

IronKey encrypts all vault data at rest using:
- **AES-256-GCM** for authenticated encryption
- **Argon2id** for master password key derivation
- **Random 16-byte salts** per encryption operation

The master password is held in memory using `Zeroizing<String>` and is cleared when no longer needed. Vault files are written atomically (tmp + rename) with `0600` permissions on Unix.

## Reporting a Vulnerability

If you discover a security vulnerability, please **do not** open a public issue.

Instead, report it privately by emailing the maintainer or using [GitHub's private vulnerability reporting](https://github.com/zacharyftw/IronKey/security/advisories/new).

Please include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact

You can expect an initial response within 72 hours.

## Known Limitations

- Clipboard contents are not auto-cleared (planned for a future release)
- No vault auto-lock on idle timeout yet
- Vault file path is not configurable (always `~/.ironkey/vault.json`)
