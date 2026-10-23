# cacrt

[![Crates.io](https://img.shields.io/crates/v/cacrt.svg)](https://crates.io/crates/cacrt)
[![Docs.rs](https://docs.rs/cacrt/badge.svg)](https://docs.rs/cacrt)
[![CI](https://github.com/KarpelesLab/cacrt/actions/workflows/ci.yml/badge.svg)](https://github.com/KarpelesLab/cacrt/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

Curated, **`no_std` / no-`alloc`** access to DER-encoded CA root certificates,
addressable by their OpenSSL subject hash (e.g. `062cdee6.0`).

The crate embeds a curated set of trusted TLS root CAs and exposes a tiny,
allocation-free API to look them up and iterate over them. All parsing, hashing
and curation happen at **build time**; at runtime the crate is nothing but
`&'static` data and integer comparisons. It has **zero runtime dependencies**.

## Why

Embedded and `no_std` projects often need a trust anchor set but can't pull in a
full PKI stack or an allocator. `cacrt` bakes a vetted root store directly into
the binary and lets you fetch a root by the same name OpenSSL's hash directories
use, or resolve an issuer by its raw DER `Name` while building a chain.

## Usage

```rust
// Look up a root by its OpenSSL hash name ("<subject-hash>.<seq>").
if let Some(ca) = cacrt::lookup("062cdee6.0") {
    assert_eq!(ca.subject_hash(), 0x062cdee6);
    let der: &[u8] = ca.der();          // DER-encoded certificate
    let _ = (ca.label(), ca.hash_name()); // "GlobalSign Root CA - R3", 062cdee6.0
}

// Iterate over every embedded CA.
for ca in cacrt::all() {
    println!("{}  {}", ca.hash_name(), ca.label());
}

// Resolve an issuer during chain building — no runtime hashing required.
let issuer_der_name: &[u8] = /* the `issuer` Name from a leaf cert */;
# let issuer_der_name = b"";
for candidate in cacrt::find_by_subject(issuer_der_name) {
    let _ = candidate.der();
}
```

### API

| Function | Purpose |
|---|---|
| `cacrt::all() -> &'static [Cert]` | Every CA, sorted by `(subject_hash, seq)`. |
| `cacrt::len() -> usize` | Number of embedded CAs. |
| `cacrt::lookup(name) -> Option<&'static Cert>` | By OpenSSL hash name, `"062cdee6.0"`. |
| `cacrt::lookup_by_hash(u32) -> &'static [Cert]` | All CAs sharing a subject hash. |
| `cacrt::find_by_subject(&[u8]) -> impl Iterator` | Exact subject-DER match. |

Each `Cert` exposes `der()`, `subject_der()`, `subject_hash()`, `seq()`,
`label()`, and `hash_name()` (a `Display` value — no allocation needed).

## Blocklist: check the whole chain

Leaving a CA out of the store is not enough when your software also trusts
another store, such as the operating system's. So `cacrt` also ships a
**blocklist** of certificates that must never be trusted, wherever they come
from:

- every root `cacrt` has disabled (`roots/**/*.disabled`), and
- compromised intermediates (`blocked/`), such as the eMudhra intermediates
  whose private keys were trivially small integers.

Entries are keyed by the SHA-256 of the certificate **and** of its
`SubjectPublicKeyInfo`. Matching on the key catches every certificate that
carries it: cross-signs, reissues and copies we have never seen.

After your verifier has built and accepted a chain, **check every certificate in
it, leaf to root, including a root that came from the OS store.** A root that
`cacrt` disabled then stays rejected on systems that still trust it.

```rust
// Zero-cost lookups: hash with your own crypto, compare against static data.
if let Some(b) = cacrt::blocked_by_spki(&spki_sha256) {
    return Err(format!("blocked: {} ({})", b.label(), b.reason()));
}

// Or, with the `check` feature, pass the DER chain and let cacrt parse and hash.
cacrt::check_chain(&[leaf_der, intermediate_der, root_der])?;
```

| Function | Purpose |
|---|---|
| `cacrt::blocked() -> &'static [Blocked]` | Every blocked certificate, with `kind()`, `label()`, `reason()` and both hashes. |
| `cacrt::blocked_by_spki(&[u8; 32]) -> Option<&Blocked>` | Is this **key** blocked? (SHA-256 of the DER SPKI.) |
| `cacrt::blocked_by_cert(&[u8; 32]) -> Option<&Blocked>` | Is this **certificate** blocked? (SHA-256 of its DER.) |
| `cacrt::blocked_by_der(&[u8])` *(feature `check`)* | Both checks on one DER certificate. |
| `cacrt::check_chain(&[&[u8]])` *(feature `check`)* | Check every certificate of a chain and report the first blocked one. |

The blocklist only helps verifiers that call it. Generic TLS stacks (rustls,
OpenSSL) know nothing about it. Hook it in after normal verification, for
example from a rustls `ServerCertVerifier` wrapper.

## The `062cdee6.0` naming

This is exactly OpenSSL's hash-directory filename (`c_rehash` / `openssl
rehash`): the eight hex digits are the **subject-name hash** — SHA-1 over the
*canonicalized* subject `Name`, first four bytes taken little-endian — and the
trailing number disambiguates certificates that share a subject hash. `cacrt`
reproduces OpenSSL's `X509_NAME_canon` at build time, and CI cross-checks every
embedded root against the system `openssl x509 -subject_hash`.

## How the root set is maintained

The source of truth is **one PEM file per root**, organized **one folder per
operating company** under [`roots/`](roots/) (e.g. `roots/DigiCert/`,
`roots/Sectigo/`). Each PEM carries a metadata header (label, subject hash,
SHA-1 fingerprint, source), and each company folder has a `README.md` describing
that CA's roots, the rationale for inclusion, CA/Browser Forum compliance, any
past non-compliance, and transparency. The set was seeded from Mozilla's NSS
`certdata.txt` and is now a frozen baseline maintained by hand against the rules
in [`CURATION.md`](CURATION.md). At build time, `build.rs` walks the tree,
converts every PEM to DER, computes its subject hash, and emits a sorted, indexed
static table.

Maintenance tooling lives in an example:

```sh
# Seed/refresh roots/ from a fresh certdata.txt
cargo run --example cacrt-tool -- import path/to/certdata.txt

# Re-check the committed roots against the curation rules (used in CI)
cargo run --example cacrt-tool -- verify

# Show what would be added/removed vs a newer certdata.txt
cargo run --example cacrt-tool -- diff path/to/certdata.txt

# Print one certificate's OpenSSL subject hash
cargo run --example cacrt-tool -- hash path/to/cert.pem
```

## `no_std` / no-`alloc` guarantee

The library is `#![no_std]`, `#![forbid(unsafe_code)]`, and never references
`alloc`, with or without the `check` feature. CI builds both for
`thumbv7em-none-eabi` (a bare-metal target with no std and no default allocator)
to prove it.

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at
your option.

The embedded root certificates are third-party data distributed by Mozilla under
the terms of the Mozilla Public License 2.0; see [`CURATION.md`](CURATION.md).
