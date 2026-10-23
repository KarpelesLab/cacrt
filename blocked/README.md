# Blocked certificates

CA certificates that must never be trusted, wherever they are found. The build
fingerprints every `*.pem` here, together with every disabled root
(`roots/**/*.disabled`), into the blocklist the crate exposes as
`cacrt::blocked()`. See [`CURATION.md`](../CURATION.md) "Blocking intermediates".

| Directory | What | Certificates |
|---|---|---|
| [`2026-10-emudhra-intermediates/`](2026-10-emudhra-intermediates/) | eMudhra intermediates with trivially small private keys | 16 (8 keys) |

Each file needs a `# Blocked: <reason>` header. Never put private keys here:
this directory ships in the crate.
