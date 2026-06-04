//! Re-implementation of OpenSSL's `X509_NAME_canon` and the subject-name hash.
//!
//! OpenSSL computes the value in `openssl x509 -subject_hash` (the `062cdee6`
//! part of a hash-dir filename) as:
//!
//!   1. Canonicalize the subject `Name`: re-encode it so every directory-string
//!      attribute value becomes a `UTF8String` whose text is converted to UTF-8,
//!      lower-cased (ASCII), with leading/trailing spaces removed and internal
//!      runs of spaces collapsed to one. Non-string values are copied verbatim.
//!   2. SHA-1 the canonical DER.
//!   3. Take the first four bytes as a little-endian `u32`.
//!
//! See OpenSSL `crypto/x509/x_name.c` (`x509_name_canon`, `asn1_string_canon`)
//! and `crypto/x509/x509_cmp.c` (`X509_NAME_hash_ex`).

use super::der::{self, Tlv};
use super::sha1::sha1;

/// Compute the OpenSSL subject hash from a subject `Name` TLV (full SEQUENCE).
pub fn name_hash(subject_name: &[u8]) -> u32 {
    let canon = canon_name(subject_name).unwrap_or_default();
    let d = sha1(&canon);
    u32::from_le_bytes([d[0], d[1], d[2], d[3]])
}

/// Produce the canonical DER encoding of a subject `Name`.
///
/// Matches OpenSSL's `x509_name_canon`/`i2d_name_canon`: the result is the
/// concatenation of each RDN encoded as a `SET OF` AttributeTypeAndValue, with
/// **no outer `SEQUENCE` wrapper**. (`X509_NAME_hash` then SHA-1s this directly.)
pub fn canon_name(subject_name: &[u8]) -> Result<Vec<u8>, der::Error> {
    let (name, _) = der::read_tlv(subject_name)?;
    if name.tag != der::SEQUENCE {
        return Err(der::Error::Unexpected("Name is not a SEQUENCE"));
    }
    let mut out = Vec::new();
    for rdn in der::children(name.content)? {
        if rdn.tag != der::SET {
            return Err(der::Error::Unexpected("RDN is not a SET"));
        }
        // DER `SET OF`: members are sorted by their encoding. (Single-valued
        // RDNs dominate, but multi-valued ones must be ordered to match.)
        let mut avas: Vec<Vec<u8>> = der::children(rdn.content)?
            .into_iter()
            .map(canon_ava)
            .collect::<Result<_, _>>()?;
        avas.sort();
        let mut avas_enc = Vec::new();
        for a in avas {
            avas_enc.extend(a);
        }
        out.extend(wrap(der::SET, &avas_enc));
    }
    Ok(out)
}

/// Canonicalize one `AttributeTypeAndValue` (SEQUENCE { type OID, value }).
fn canon_ava(ava: Tlv<'_>) -> Result<Vec<u8>, der::Error> {
    if ava.tag != der::SEQUENCE {
        return Err(der::Error::Unexpected("AVA is not a SEQUENCE"));
    }
    let parts = der::children(ava.content)?;
    let oid = parts
        .first()
        .filter(|t| t.tag == der::OID)
        .ok_or(der::Error::Unexpected("AVA missing type OID"))?;
    let value = parts
        .get(1)
        .ok_or(der::Error::Unexpected("AVA missing value"))?;

    let value_enc = if is_canon_string(value.tag) {
        let utf8 = to_utf8(value.tag, value.content);
        let canon = canon_string(&utf8);
        wrap(0x0c, &canon) // re-tag as UTF8String
    } else {
        value.raw.to_vec()
    };

    let mut inner = oid.raw.to_vec();
    inner.extend(value_enc);
    Ok(wrap(der::SEQUENCE, &inner))
}

/// String tags OpenSSL subjects to canonicalization (`ASN1_MASK_CANON`).
fn is_canon_string(tag: u8) -> bool {
    matches!(
        tag,
        0x0c // UTF8String
        | 0x12 // NumericString
        | 0x13 // PrintableString
        | 0x14 // T61String / TeletexString
        | 0x15 // VideotexString
        | 0x16 // IA5String
        | 0x1a // VisibleString / ISO646String
        | 0x1b // GeneralString
        | 0x1c // UniversalString
        | 0x1e // BMPString
    )
}

/// Convert a string value's content bytes to UTF-8, per its tag.
fn to_utf8(tag: u8, content: &[u8]) -> Vec<u8> {
    match tag {
        0x1e => {
            // BMPString: UTF-16BE.
            let mut out = Vec::new();
            for pair in content.chunks_exact(2) {
                let cp = u16::from_be_bytes([pair[0], pair[1]]);
                push_utf8(&mut out, cp as u32);
            }
            out
        }
        0x1c => {
            // UniversalString: UTF-32BE.
            let mut out = Vec::new();
            for quad in content.chunks_exact(4) {
                let cp = u32::from_be_bytes([quad[0], quad[1], quad[2], quad[3]]);
                push_utf8(&mut out, cp);
            }
            out
        }
        0x14 | 0x15 => {
            // T61/Videotex: approximate as Latin-1 (high bytes rarely appear in
            // CA subjects; the openssl cross-check test guards real cases).
            let mut out = Vec::new();
            for &b in content {
                push_utf8(&mut out, b as u32);
            }
            out
        }
        // UTF8String / Printable / IA5 / Visible / Numeric / General are already
        // ASCII or UTF-8 byte sequences.
        _ => content.to_vec(),
    }
}

fn push_utf8(out: &mut Vec<u8>, cp: u32) {
    match char::from_u32(cp) {
        Some(c) => {
            let mut b = [0u8; 4];
            out.extend_from_slice(c.encode_utf8(&mut b).as_bytes());
        }
        None => out.push(b'?'),
    }
}

/// Apply `asn1_string_canon`: strip leading/trailing ASCII spaces, collapse
/// internal space runs to one, lower-case ASCII; copy non-ASCII verbatim.
fn canon_string(utf8: &[u8]) -> Vec<u8> {
    let start = utf8.iter().position(|&b| b != b' ').unwrap_or(utf8.len());
    let end = utf8
        .iter()
        .rposition(|&b| b != b' ')
        .map(|p| p + 1)
        .unwrap_or(start);
    let trimmed = &utf8[start..end];

    let mut out = Vec::with_capacity(trimmed.len());
    let mut i = 0;
    while i < trimmed.len() {
        let b = trimmed[i];
        if b >= 0x80 {
            out.push(b);
            i += 1;
        } else if b == b' ' {
            out.push(b' ');
            while i < trimmed.len() && trimmed[i] == b' ' {
                i += 1;
            }
        } else {
            out.push(b.to_ascii_lowercase());
            i += 1;
        }
    }
    out
}

/// Wrap `content` in a DER TLV with the given tag and definite length.
fn wrap(tag: u8, content: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(content.len() + 4);
    out.push(tag);
    let len = content.len();
    if len < 0x80 {
        out.push(len as u8);
    } else {
        let bytes = len.to_be_bytes();
        let first = bytes.iter().position(|&b| b != 0).unwrap();
        let sig = &bytes[first..];
        out.push(0x80 | sig.len() as u8);
        out.extend_from_slice(sig);
    }
    out.extend_from_slice(content);
    out
}

#[cfg(test)]
mod tests {
    use super::canon_string;

    #[test]
    fn collapses_and_lowercases() {
        assert_eq!(canon_string(b"  Hello   World  "), b"hello world");
        assert_eq!(canon_string(b"ACME"), b"acme");
        assert_eq!(canon_string(b"   "), b"");
    }
}
