//! A deliberately small DER/TLV reader, enough to pull the fields we need out
//! of an X.509 `Certificate` at build time. Not a general-purpose parser: it
//! assumes definite-length DER (which conforming certificates always use).

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Truncated,
    IndefiniteLength,
    LengthOverflow,
    Trailing,
    Unexpected(&'static str),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}

// Common universal tags.
pub const SEQUENCE: u8 = 0x30;
pub const SET: u8 = 0x31;
pub const INTEGER: u8 = 0x02;
pub const OID: u8 = 0x06;
pub const BIT_STRING: u8 = 0x03;
pub const UTC_TIME: u8 = 0x17;
pub const GENERALIZED_TIME: u8 = 0x18;
/// Context-specific, constructed, tag 0 — the explicit `version` of TBSCertificate.
pub const CONTEXT_0: u8 = 0xA0;

/// A parsed Tag-Length-Value element borrowing from the input.
#[derive(Debug, Clone, Copy)]
pub struct Tlv<'a> {
    pub tag: u8,
    /// Tag + length octets only.
    pub header: &'a [u8],
    /// Value octets.
    pub content: &'a [u8],
    /// The whole element: header + content.
    pub raw: &'a [u8],
}

/// Read one TLV from the front of `input`, returning it and the remaining bytes.
pub fn read_tlv(input: &[u8]) -> Result<(Tlv<'_>, &[u8]), Error> {
    if input.len() < 2 {
        return Err(Error::Truncated);
    }
    let tag = input[0];
    let len_byte = input[1];
    let (content_len, header_len) = if len_byte < 0x80 {
        (len_byte as usize, 2)
    } else if len_byte == 0x80 {
        return Err(Error::IndefiniteLength);
    } else {
        let n = (len_byte & 0x7f) as usize;
        if n > 4 || input.len() < 2 + n {
            return Err(Error::LengthOverflow);
        }
        let mut len = 0usize;
        for &b in &input[2..2 + n] {
            len = (len << 8) | b as usize;
        }
        (len, 2 + n)
    };
    let total = header_len + content_len;
    if input.len() < total {
        return Err(Error::Truncated);
    }
    let tlv = Tlv {
        tag,
        header: &input[..header_len],
        content: &input[header_len..total],
        raw: &input[..total],
    };
    Ok((tlv, &input[total..]))
}

/// Parse the contents of a constructed element into its child TLVs, in order.
pub fn children(content: &[u8]) -> Result<Vec<Tlv<'_>>, Error> {
    let mut rest = content;
    let mut out = Vec::new();
    while !rest.is_empty() {
        let (tlv, next) = read_tlv(rest)?;
        out.push(tlv);
        rest = next;
    }
    Ok(out)
}

/// Read one TLV and require it consumes the whole input (no trailing bytes).
fn read_single<'a>(input: &'a [u8], expect: u8, what: &'static str) -> Result<Tlv<'a>, Error> {
    let (tlv, rest) = read_tlv(input)?;
    if !rest.is_empty() {
        return Err(Error::Trailing);
    }
    if tlv.tag != expect {
        return Err(Error::Unexpected(what));
    }
    Ok(tlv)
}

/// Return the `TBSCertificate` element (full TLV) of a DER `Certificate`.
pub fn tbs_certificate(cert: &[u8]) -> Result<Tlv<'_>, Error> {
    let outer = read_single(cert, SEQUENCE, "Certificate is not a SEQUENCE")?;
    let (tbs, _rest) = read_tlv(outer.content)?;
    if tbs.tag != SEQUENCE {
        return Err(Error::Unexpected("TBSCertificate is not a SEQUENCE"));
    }
    Ok(tbs)
}

/// Selectable fields of `TBSCertificate`, in their on-the-wire order.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TbsField {
    SerialNumber,
    Signature,
    Issuer,
    Validity,
    Subject,
    Spki,
}

/// Extract one field of a parsed `TBSCertificate`. Returns the raw bytes of the
/// field's TLV (tag + length + value).
pub fn tbs_field<'a>(tbs: Tlv<'a>, field: TbsField) -> Result<&'a [u8], Error> {
    let kids = children(tbs.content)?;
    // The optional `[0] EXPLICIT Version` shifts everything by one when present.
    let base = if kids.first().map(|t| t.tag) == Some(CONTEXT_0) {
        1
    } else {
        0
    };
    let idx = base
        + match field {
            TbsField::SerialNumber => 0,
            TbsField::Signature => 1,
            TbsField::Issuer => 2,
            TbsField::Validity => 3,
            TbsField::Subject => 4,
            TbsField::Spki => 5,
        };
    kids.get(idx)
        .map(|t| t.raw)
        .ok_or(Error::Unexpected("TBSCertificate field missing"))
}

/// A calendar time parsed from an X.509 `Time`, normalized for comparison as
/// `YYYYMMDDHHMMSS`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time(pub u64);

/// Parse an X.509 `Time` (UTCTime or GeneralizedTime), assuming Z (UTC) form as
/// required by RFC 5280.
pub fn parse_time(time_tlv: &[u8]) -> Result<Time, Error> {
    let (tlv, rest) = read_tlv(time_tlv)?;
    if !rest.is_empty() {
        return Err(Error::Trailing);
    }
    let s = std::str::from_utf8(tlv.content).map_err(|_| Error::Unexpected("Time not ASCII"))?;
    let digits = |slice: &str| -> Result<u64, Error> {
        slice
            .parse::<u64>()
            .map_err(|_| Error::Unexpected("Time has non-digit"))
    };
    let (yyyy, mmddhhmmss) = match tlv.tag {
        UTC_TIME => {
            // YYMMDDHHMMSSZ — RFC 5280: YY < 50 => 20YY, else 19YY.
            if s.len() < 13 {
                return Err(Error::Unexpected("short UTCTime"));
            }
            let yy = digits(&s[0..2])?;
            let year = if yy < 50 { 2000 + yy } else { 1900 + yy };
            (year, &s[2..12])
        }
        GENERALIZED_TIME => {
            // YYYYMMDDHHMMSSZ
            if s.len() < 15 {
                return Err(Error::Unexpected("short GeneralizedTime"));
            }
            (digits(&s[0..4])?, &s[4..14])
        }
        _ => return Err(Error::Unexpected("not a Time")),
    };
    Ok(Time(yyyy * 10_000_000_000 + digits(mmddhhmmss)?))
}

/// Public-key summary used for curation key-strength checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyInfo {
    /// RSA modulus size in bits.
    Rsa { bits: usize },
    /// Named elliptic curve (by friendly label) and field size in bits.
    Ec { curve: &'static str, bits: usize },
    /// Anything we don't classify (DSA, Ed25519, unknown OID).
    Other { oid: Vec<u8> },
}

const OID_RSA: &[u8] = &[0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x01];
const OID_EC: &[u8] = &[0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01];
const OID_P256: &[u8] = &[0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07];
const OID_P384: &[u8] = &[0x2b, 0x81, 0x04, 0x00, 0x22];
const OID_P521: &[u8] = &[0x2b, 0x81, 0x04, 0x00, 0x23];

/// Classify the public key in a `SubjectPublicKeyInfo` TLV.
pub fn spki_key_info(spki: &[u8]) -> Result<KeyInfo, Error> {
    let spki = read_single(spki, SEQUENCE, "SPKI is not a SEQUENCE")?;
    let parts = children(spki.content)?;
    let alg = parts
        .first()
        .ok_or(Error::Unexpected("SPKI missing algorithm"))?;
    let alg_parts = children(alg.content)?;
    let oid = alg_parts
        .first()
        .filter(|t| t.tag == OID)
        .ok_or(Error::Unexpected("algorithm missing OID"))?;

    if oid.content == OID_RSA {
        // subjectPublicKey BIT STRING wraps RSAPublicKey ::= SEQUENCE { modulus, .. }
        let bitstr = parts
            .get(1)
            .filter(|t| t.tag == BIT_STRING)
            .ok_or(Error::Unexpected("RSA SPKI missing BIT STRING"))?;
        // First content byte of a BIT STRING is the count of unused bits.
        let inner = bitstr
            .content
            .get(1..)
            .ok_or(Error::Unexpected("empty BIT STRING"))?;
        let rsa = read_single(inner, SEQUENCE, "RSAPublicKey not a SEQUENCE")?;
        let modulus = children(rsa.content)?
            .into_iter()
            .next()
            .filter(|t| t.tag == INTEGER)
            .ok_or(Error::Unexpected("RSA modulus missing"))?;
        // Strip a single leading 0x00 sign byte, then count bits.
        let mag = match modulus.content.first() {
            Some(0) => &modulus.content[1..],
            _ => modulus.content,
        };
        let bits = match mag.first() {
            Some(&first) => (mag.len() - 1) * 8 + (8 - first.leading_zeros() as usize),
            None => 0,
        };
        return Ok(KeyInfo::Rsa { bits });
    }

    if oid.content == OID_EC {
        let curve = alg_parts.get(1).filter(|t| t.tag == OID);
        let (curve, bits) = match curve.map(|t| t.content) {
            Some(c) if c == OID_P256 => ("P-256", 256),
            Some(c) if c == OID_P384 => ("P-384", 384),
            Some(c) if c == OID_P521 => ("P-521", 521),
            _ => ("unknown", 0),
        };
        return Ok(KeyInfo::Ec { curve, bits });
    }

    Ok(KeyInfo::Other {
        oid: oid.content.to_vec(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_long_form_length() {
        // SEQUENCE, long-form length 0x0102 = 258, with 258 content bytes.
        let mut buf = vec![SEQUENCE, 0x82, 0x01, 0x02];
        buf.extend(std::iter::repeat_n(0u8, 258));
        let (tlv, rest) = read_tlv(&buf).unwrap();
        assert_eq!(tlv.tag, SEQUENCE);
        assert_eq!(tlv.content.len(), 258);
        assert!(rest.is_empty());
    }

    #[test]
    fn rejects_indefinite_length() {
        assert_eq!(
            read_tlv(&[SEQUENCE, 0x80]).unwrap_err(),
            Error::IndefiniteLength
        );
    }
}
