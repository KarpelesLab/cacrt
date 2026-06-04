//! The curation rules, in code. Used by `examples/cacrt-tool.rs` to decide what
//! to import, and by `build.rs` to keep the committed `roots/` honest.
//!
//! The full rationale for each rule lives in `CURATION.md`. Keep the two in sync.

use super::der;

/// Outcome of evaluating a candidate root for inclusion.
#[derive(Debug, PartialEq, Eq)]
pub enum Decision {
    Accept,
    Reject(String),
}

/// Minimum acceptable RSA modulus size, in bits (CA/Browser Forum BR §6.1.5).
pub const MIN_RSA_BITS: usize = 2048;
/// Minimum acceptable ECC field size, in bits (P-256 and up).
pub const MIN_EC_BITS: usize = 256;

/// Labels (case-insensitive substring match) we exclude beyond what the source
/// already drops. Mozilla's certdata.txt already removes fully-distrusted CAs,
/// so this list is for additional, deliberately documented removals. Each entry
/// MUST have a matching note in CURATION.md / CHANGELOG-roots.md.
pub const DENY_LABEL_SUBSTRINGS: &[(&str, &str)] = &[
    // (substring, reason) — empty by default; add with a documented rationale.
];

/// Full inclusion decision for a candidate certificate.
///
/// * `distrust_after` — the NSS `CKA_NSS_SERVER_DISTRUST_AFTER` attribute is set
///   (the CA is being phased out for server auth).
/// * `server_auth` — the value of `CKA_TRUST_SERVER_AUTH`, if a trust object
///   was found for this certificate.
pub fn evaluate(
    distrust_after: bool,
    label: &str,
    server_auth: Option<&str>,
    der_bytes: &[u8],
    now: u64,
) -> Decision {
    // Rule 1: must be an explicit TLS server-auth trust anchor.
    match server_auth {
        Some("CKT_NSS_TRUSTED_DELEGATOR") => {}
        Some(other) => {
            return Decision::Reject(format!("not a server-auth trust anchor ({other})"))
        }
        None => return Decision::Reject("no server-auth trust record".into()),
    }

    // Rule 2: honor NSS server distrust-after (phase-out).
    if distrust_after {
        return Decision::Reject("NSS server distrust-after is set (CA being phased out)".into());
    }

    // Rule 3: explicit deny list (documented manual removals).
    let lc = label.to_ascii_lowercase();
    for (needle, reason) in DENY_LABEL_SUBSTRINGS {
        if lc.contains(&needle.to_ascii_lowercase()) {
            return Decision::Reject(format!("deny list: {reason}"));
        }
    }

    // Rules 4 & 5: machine-checkable certificate properties.
    if let Err(reason) = check_cert(der_bytes, now) {
        return Decision::Reject(reason);
    }

    Decision::Accept
}

/// Machine-checkable certificate rules: structural validity, key strength, and
/// expiry. Returns `Err(reason)` on the first violation.
pub fn check_cert(der_bytes: &[u8], now: u64) -> Result<(), String> {
    let tbs = der::tbs_certificate(der_bytes).map_err(|e| format!("malformed certificate: {e}"))?;

    // Expiry (rule: drop certs already past notAfter).
    let validity = der::tbs_field(tbs, der::TbsField::Validity).map_err(|e| e.to_string())?;
    let v = der::read_tlv(validity).map_err(|e| e.to_string())?.0;
    let times = der::children(v.content).map_err(|e| e.to_string())?;
    let not_after = times
        .get(1)
        .ok_or_else(|| "validity missing notAfter".to_string())?;
    let exp = der::parse_time(not_after.raw).map_err(|e| e.to_string())?;
    if exp.0 < now {
        return Err(format!("expired (notAfter {})", exp.0));
    }

    // Key strength (rule: RSA >= 2048, ECC >= P-256, no DSA/unknown).
    let spki = der::tbs_field(tbs, der::TbsField::Spki).map_err(|e| e.to_string())?;
    match der::spki_key_info(spki).map_err(|e| e.to_string())? {
        der::KeyInfo::Rsa { bits } if bits >= MIN_RSA_BITS => {}
        der::KeyInfo::Rsa { bits } => return Err(format!("RSA key too small ({bits} bits)")),
        der::KeyInfo::Ec { curve, bits } if bits >= MIN_EC_BITS => {
            let _ = curve;
        }
        der::KeyInfo::Ec { curve, bits } => {
            return Err(format!("EC curve too weak ({curve}, {bits} bits)"))
        }
        der::KeyInfo::Other { oid } => {
            return Err(format!("unsupported key algorithm (OID bytes {oid:02x?})"))
        }
    }

    Ok(())
}

/// Current UTC time as `YYYYMMDDHHMMSS` packed into a u64, matching
/// [`der::Time`]'s ordering so the two can be compared directly.
pub fn now_yyyymmddhhmmss() -> u64 {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let (y, mo, d, h, mi, s) = civil_from_unix(secs as i64);
    (y as u64) * 10_000_000_000
        + (mo as u64) * 100_000_000
        + (d as u64) * 1_000_000
        + (h as u64) * 10_000
        + (mi as u64) * 100
        + s as u64
}

/// Convert Unix seconds to a civil UTC date/time (Howard Hinnant's algorithm).
fn civil_from_unix(secs: i64) -> (i64, u32, u32, u32, u32, u32) {
    let days = secs.div_euclid(86_400);
    let rem = secs.rem_euclid(86_400);
    let (h, mi, s) = (rem / 3600, (rem % 3600) / 60, rem % 60);

    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m as u32, d as u32, h as u32, mi as u32, s as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn now_is_in_a_sane_range() {
        let n = now_yyyymmddhhmmss();
        assert!(n > 20_200_000_000_000, "got {n}"); // after 2020-01-01
        assert!(n < 21_000_000_000_000, "got {n}"); // before 2100-01-01
    }

    #[test]
    fn rejects_non_anchor() {
        let d = evaluate(false, "X", Some("CKT_NSS_MUST_VERIFY_TRUST"), &[], 0);
        assert!(matches!(d, Decision::Reject(_)));
    }
}
