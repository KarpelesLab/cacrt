//! Decoder for NSS `certdata.txt` `MULTILINE_OCTAL` blocks.
//!
//! Such a block is a run of lines where each byte is written as a backslash
//! followed by three octal digits, e.g. `\060\202\005`.

/// Decode the body lines of a `MULTILINE_OCTAL` value (between the attribute
/// line and the terminating `END`). Returns the raw bytes.
pub fn decode(body: &str) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    let mut chars = body.chars().peekable();
    while let Some(&c) = chars.peek() {
        if c == '\\' {
            chars.next();
            let mut val: u16 = 0;
            for _ in 0..3 {
                let d = chars
                    .next()
                    .ok_or_else(|| "truncated octal escape".to_string())?;
                let digit = d
                    .to_digit(8)
                    .ok_or_else(|| format!("invalid octal digit {d:?}"))?;
                val = val * 8 + digit as u16;
            }
            if val > 255 {
                return Err(format!("octal value {val} out of range"));
            }
            out.push(val as u8);
        } else {
            // Whitespace / newlines between escapes are ignored.
            if !c.is_whitespace() {
                return Err(format!("unexpected character {c:?} in octal block"));
            }
            chars.next();
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::decode;

    #[test]
    fn decodes_multiline() {
        // \060\127 = 0x30 0x57
        let got = decode("\\060\\127\n\\013\\060").unwrap();
        assert_eq!(got, vec![0x30, 0x57, 0x0b, 0x30]);
    }
}
