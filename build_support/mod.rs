//! Host-side helpers shared by `build.rs` and the maintenance tooling in
//! `examples/`. Included via `#[path = "build_support/mod.rs"]`; never linked
//! into the published `no_std` library.
//!
//! Everything here is dependency-free and hand-rolled on purpose: the entire
//! build of a trust-store crate should be auditable without pulling in the
//! transitive dependency world.
#![allow(dead_code)] // not every consumer (build.rs vs examples) uses every helper

pub mod base64;
pub mod canon;
pub mod curation;
pub mod der;
pub mod octal;
pub mod pem;
pub mod sha1;

/// Compute the OpenSSL subject hash of a DER-encoded certificate.
///
/// This is the `062cdee6` part of an OpenSSL hash-dir filename: SHA-1 over the
/// canonicalized subject `Name`, first four bytes assembled little-endian.
pub fn subject_hash(cert_der: &[u8]) -> Result<u32, der::Error> {
    let tbs = der::tbs_certificate(cert_der)?;
    let subject = der::tbs_field(tbs, der::TbsField::Subject)?;
    Ok(canon::name_hash(subject))
}
