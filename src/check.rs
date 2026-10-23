//! Optional (`check` feature) runtime blocklist checks on DER certificates.
//!
//! A tiny, allocation-free DER walk finds the `SubjectPublicKeyInfo`, and the
//! certificate and its key are hashed with the bundled SHA-256. Nothing here
//! verifies signatures or builds paths: run it on a chain your verifier has
//! already accepted.

use crate::sha256::sha256;
use crate::{blocked_by_cert, blocked_by_spki, Blocked};

/// The certificate is not DER this parser can read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParseError;

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("malformed DER certificate")
    }
}

/// Why [`check_chain`] rejected a chain. `index` is the position in the slice.
#[derive(Clone, Copy, Debug)]
pub enum ChainError {
    /// The certificate at `index` (or its key) is on the blocklist.
    Blocked {
        /// Position of the certificate in the chain.
        index: usize,
        /// The matching blocklist entry.
        entry: &'static Blocked,
    },
    /// The certificate at `index` could not be parsed.
    Malformed {
        /// Position of the certificate in the chain.
        index: usize,
    },
}

impl core::fmt::Display for ChainError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ChainError::Blocked { index, entry } => write!(
                f,
                "certificate {index} is blocked: {} ({})",
                entry.label(),
                entry.reason()
            ),
            ChainError::Malformed { index } => write!(f, "certificate {index} is malformed"),
        }
    }
}

/// The blocklist entry matching this DER certificate, by its key or by the
/// certificate itself.
pub fn blocked_by_der(cert_der: &[u8]) -> Result<Option<&'static Blocked>, ParseError> {
    let spki = spki(cert_der)?;
    Ok(blocked_by_spki(&sha256(spki)).or_else(|| blocked_by_cert(&sha256(cert_der))))
}

/// Check every DER certificate of a chain (leaf, intermediates **and root**)
/// against the blocklist. Fails on the first blocked or unparsable certificate.
///
/// Pass the root too, even when it came from another trust store: that is how
/// a root `cacrt` has disabled stays rejected on systems that still trust it.
pub fn check_chain(chain: &[&[u8]]) -> Result<(), ChainError> {
    for (index, der) in chain.iter().enumerate() {
        match blocked_by_der(der) {
            Ok(None) => {}
            Ok(Some(entry)) => return Err(ChainError::Blocked { index, entry }),
            Err(ParseError) => return Err(ChainError::Malformed { index }),
        }
    }
    Ok(())
}

/// The raw `SubjectPublicKeyInfo` TLV of a DER `Certificate`.
fn spki(cert: &[u8]) -> Result<&[u8], ParseError> {
    let (outer, rest) = read_tlv(cert, 0x30)?;
    if !rest.is_empty() {
        return Err(ParseError);
    }
    let (tbs, _) = read_tlv(outer, 0x30)?;
    let mut rest = tbs;
    // Skip the optional `[0] EXPLICIT Version`.
    if rest.first() == Some(&0xA0) {
        rest = read_tlv(rest, 0xA0)?.1;
    }
    // serialNumber, signature, issuer, validity, subject.
    for tag in [0x02, 0x30, 0x30, 0x30, 0x30] {
        rest = read_tlv(rest, tag)?.1;
    }
    let (content, after) = read_tlv(rest, 0x30)?;
    let len = rest.len() - after.len();
    debug_assert!(content.len() < len);
    Ok(&rest[..len])
}

/// Read one definite-length TLV with the expected tag, returning its content
/// and the bytes after it.
fn read_tlv(input: &[u8], tag: u8) -> Result<(&[u8], &[u8]), ParseError> {
    let (&t, rest) = input.split_first().ok_or(ParseError)?;
    if t != tag {
        return Err(ParseError);
    }
    let (&first, mut rest) = rest.split_first().ok_or(ParseError)?;
    let len = if first < 0x80 {
        first as usize
    } else {
        let n = (first & 0x7f) as usize;
        if n == 0 || n > 4 || rest.len() < n {
            return Err(ParseError);
        }
        let mut len = 0usize;
        for &b in &rest[..n] {
            len = (len << 8) | b as usize;
        }
        rest = &rest[n..];
        len
    };
    if rest.len() < len {
        return Err(ParseError);
    }
    Ok(rest.split_at(len))
}
