//! Minimal PEM handling for `CERTIFICATE` blocks.

use super::base64;

/// Extract the DER bytes from the single `CERTIFICATE` block in `text`.
/// Errors if there are zero or more than one certificate blocks.
pub fn read_one_certificate(text: &str) -> Result<Vec<u8>, String> {
    const BEGIN: &str = "-----BEGIN CERTIFICATE-----";
    const END: &str = "-----END CERTIFICATE-----";

    let mut blocks = text.match_indices(BEGIN);
    let first = blocks.next().ok_or("no CERTIFICATE block found")?;
    if blocks.next().is_some() {
        return Err("more than one CERTIFICATE block".into());
    }
    let after_begin = &text[first.0 + BEGIN.len()..];
    let end = after_begin
        .find(END)
        .ok_or("CERTIFICATE block has no END marker")?;
    base64::decode(&after_begin[..end])
}

/// Render DER bytes as a PEM `CERTIFICATE` block (64-char lines).
pub fn write_certificate(der: &[u8]) -> String {
    let b64 = base64::encode(der);
    let mut out = String::from("-----BEGIN CERTIFICATE-----\n");
    for line in b64.as_bytes().chunks(64) {
        out.push_str(std::str::from_utf8(line).unwrap());
        out.push('\n');
    }
    out.push_str("-----END CERTIFICATE-----\n");
    out
}

#[cfg(test)]
mod tests {
    use super::{read_one_certificate, write_certificate};

    #[test]
    fn roundtrip() {
        let der: Vec<u8> = (0..200u32).map(|i| i as u8).collect();
        let pem = write_certificate(&der);
        assert_eq!(read_one_certificate(&pem).unwrap(), der);
    }
}
