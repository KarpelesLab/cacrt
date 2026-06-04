//! `cacrt` — curated, `no_std`/no-`alloc` access to DER-encoded CA root
//! certificates by their OpenSSL subject hash.
//!
//! The crate embeds a curated set of trusted root CA certificates (see
//! `CURATION.md` for the acceptance rules) and exposes a tiny, allocation-free
//! API to look them up and iterate over them. All parsing, hashing and curation
//! happen at build time; at runtime this crate is pure `&'static` data and
//! integer comparisons.
//!
//! # OpenSSL hash names
//!
//! Certificates are addressable by the same name OpenSSL's `c_rehash`/`openssl
//! rehash` uses for files in a hash directory, e.g. `"062cdee6.0"`. The eight
//! hex digits are the subject-name hash (SHA-1 over the canonicalized subject,
//! first four bytes little-endian) and the trailing number disambiguates
//! certificates that share a subject hash.
//!
//! ```
//! // Look one up by its OpenSSL hash name.
//! if let Some(ca) = cacrt::lookup("062cdee6.0") {
//!     assert_eq!(ca.subject_hash(), 0x062cdee6);
//!     let _der: &[u8] = ca.der();
//! }
//!
//! // Iterate over every embedded CA.
//! for ca in cacrt::all() {
//!     let _ = (ca.subject_hash(), ca.label());
//! }
//! ```
//!
//! # Building certificate chains
//!
//! To find the issuer of a certificate without hashing at runtime, match the
//! issuer's raw DER `Name` against each CA's subject with [`find_by_subject`].

#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// A single embedded CA root certificate.
///
/// All accessors are zero-cost: the fields point into data baked into the
/// binary at build time.
pub struct Cert {
    subject_hash: u32,
    seq: u8,
    der: &'static [u8],
    subject: &'static [u8],
    label: &'static str,
}

impl Cert {
    /// Construct an entry. Only called by generated code in `certs.rs`.
    const fn new(
        subject_hash: u32,
        seq: u8,
        der: &'static [u8],
        subject: &'static [u8],
        label: &'static str,
    ) -> Self {
        Cert {
            subject_hash,
            seq,
            der,
            subject,
            label,
        }
    }

    /// The full DER-encoded `Certificate`.
    pub const fn der(&self) -> &'static [u8] {
        self.der
    }

    /// The raw DER of the certificate's subject `Name` (the `SEQUENCE` TLV).
    pub const fn subject_der(&self) -> &'static [u8] {
        self.subject
    }

    /// The OpenSSL subject hash (the `062cdee6` part of a hash name).
    pub const fn subject_hash(&self) -> u32 {
        self.subject_hash
    }

    /// The collision sequence number (the `0` in `062cdee6.0`).
    pub const fn seq(&self) -> u8 {
        self.seq
    }

    /// A human-readable label (the NSS `CKA_LABEL`).
    pub const fn label(&self) -> &'static str {
        self.label
    }

    /// The OpenSSL hash name (`"062cdee6.0"`) as a [`Display`](core::fmt::Display)
    /// value — formatting it needs no allocation.
    pub const fn hash_name(&self) -> HashName {
        HashName {
            hash: self.subject_hash,
            seq: self.seq,
        }
    }
}

impl core::fmt::Debug for Cert {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Cert")
            .field("name", &format_args!("{}", self.hash_name()))
            .field("label", &self.label)
            .field("der_len", &self.der.len())
            .finish()
    }
}

/// The OpenSSL hash name of a certificate, formatted on demand as `"%08x.%d"`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct HashName {
    hash: u32,
    seq: u8,
}

impl core::fmt::Display for HashName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:08x}.{}", self.hash, self.seq)
    }
}

impl core::fmt::Debug for HashName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(self, f)
    }
}

// Generated: `pub(crate) static CERTS: &[Cert]`, sorted by (subject_hash, seq).
include!(concat!(env!("OUT_DIR"), "/certs.rs"));

/// All embedded CA certificates, sorted by subject hash then sequence number.
pub fn all() -> &'static [Cert] {
    CERTS
}

/// Number of embedded CA certificates.
pub fn len() -> usize {
    CERTS.len()
}

/// Every certificate sharing the given subject hash, as a contiguous slice
/// (empty if none). Within the slice, entries are ordered by sequence number.
pub fn lookup_by_hash(hash: u32) -> &'static [Cert] {
    let lo = CERTS.partition_point(|c| c.subject_hash < hash);
    let hi = CERTS.partition_point(|c| c.subject_hash <= hash);
    &CERTS[lo..hi]
}

/// Look up a certificate by its OpenSSL hash name, e.g. `"062cdee6.0"`.
///
/// Returns `None` if the name is malformed or no such certificate is embedded.
pub fn lookup(name: &str) -> Option<&'static Cert> {
    let (hash_str, seq_str) = name.split_once('.')?;
    if hash_str.len() != 8 {
        return None;
    }
    let hash = u32::from_str_radix(hash_str, 16).ok()?;
    let seq: u8 = seq_str.parse().ok()?;
    lookup_by_hash(hash).iter().find(|c| c.seq == seq)
}

/// All certificates whose subject `Name` exactly equals `subject_der` (the raw
/// DER `SEQUENCE`). Useful for resolving an issuer during chain building without
/// hashing at runtime.
pub fn find_by_subject<'a>(subject_der: &'a [u8]) -> impl Iterator<Item = &'static Cert> + 'a {
    CERTS.iter().filter(move |c| c.subject == subject_der)
}
