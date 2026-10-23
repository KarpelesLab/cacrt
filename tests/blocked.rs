//! Blocklist behavior: the disabled roots and the compromised eMudhra
//! intermediates are listed, lookups by key and by certificate hash work, and
//! (with the `check` feature) whole DER chains are checked, root included.

use cacrt::BlockedKind;

fn unhex(s: &str) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (i, b) in out.iter_mut().enumerate() {
        *b = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).unwrap();
    }
    out
}

/// SPKI SHA-256 of the eight compromised eMudhra intermediate keys, as
/// published in the incident's `impacted-intermediates.json`.
const EMUDHRA_SPKI: [&str; 8] = [
    "EBED6475C2C2D80FCB199F3FE8DD6E6B3CE60B77EB15D4E42C3DEC0C6F5B5105", // EM DV TLS CA - G3B
    "43BFFA19D6EF651451F6841E019931E96F6F2EAEFF28AB4B6F1CE4576DE2717B", // EM OV TLS CA - G3B
    "E0254C7FCDC265E2F0F8A428DABB75AB4AE039B4872D62F5EE57F0877D5A8A41", // EM EV TLS CA - G3B
    "49DD353DC2B919A3770645117D0E60C16115E7D1BC259EEFE734B00EA434FE5F", // EM DV TLS CA - G3C
    "63ECEFEE431D49523BC1136C4E72D0206A477086CAF606CD5CF025686C109CF8", // EM OV TLS CA - G3C
    "5C0660A2B164910D8FF589C18E14552E9E0EC237DCEFE2CB736D41A4811F81A0", // EM EV TLS CA - G3C
    "EC4FFA1AD0AC88081314B7AAFFEF6A91F97CD57291BB5BA686C5415F5EB4F6B6", // EM OV CS CA - G3B
    "92C063D67E8F14DB71D288E11266FFDAF69CBF115F7BAB14F3BC2F68D25E0E30", // EM OV CS CA - G3C
];

#[test]
fn blocklist_is_sorted_and_complete() {
    let all = cacrt::blocked();
    for w in all.windows(2) {
        let a = (w[0].spki_sha256(), w[0].cert_sha256());
        let b = (w[1].spki_sha256(), w[1].cert_sha256());
        assert!(a < b, "not strictly ordered");
    }
    let roots = all.iter().filter(|b| b.kind() == BlockedKind::Root).count();
    let inter = all
        .iter()
        .filter(|b| b.kind() == BlockedKind::Intermediate)
        .count();
    // 3 disabled roots (emSign ECC G3, Buypass Class 2 and 3); 16 eMudhra
    // intermediates (8 keys, each issued by eMudhra and cross-signed by Sectigo).
    assert_eq!((roots, inter), (3, 16));
    for b in all {
        assert!(!b.reason().is_empty(), "{} has no reason", b.label());
    }
}

#[test]
fn compromised_emudhra_keys_are_blocked() {
    for spki in EMUDHRA_SPKI {
        let b =
            cacrt::blocked_by_spki(&unhex(spki)).unwrap_or_else(|| panic!("{spki} not blocked"));
        assert_eq!(b.kind(), BlockedKind::Intermediate);
        let certs = cacrt::blocked()
            .iter()
            .filter(|e| *e.spki_sha256() == unhex(spki))
            .count();
        assert_eq!(
            certs, 2,
            "{spki}: expected the eMudhra cert and the Sectigo cross-sign"
        );
    }
}

#[test]
fn disabled_root_is_blocked_by_key_and_cert() {
    // emSign ECC Root CA - G3.
    let spki = unhex("8D417DB2DD8BF5E3084D1E3F196D583849D81BDD4C00C70B9D39369E96B8C782");
    let cert = unhex("86A1ECBA089C4A8D3BBE2734C612BA341D813E043CF9E8A862CD5C57A36BBE6B");
    let b = cacrt::blocked_by_spki(&spki).expect("emSign ECC Root CA - G3 key blocked");
    assert_eq!(b.kind(), BlockedKind::Root);
    assert_eq!(b.label(), "emSign ECC Root CA - G3");
    assert!(core::ptr::eq(b, cacrt::blocked_by_cert(&cert).unwrap()));
}

#[test]
fn unknown_hashes_are_not_blocked() {
    assert!(cacrt::blocked_by_spki(&[0; 32]).is_none());
    assert!(cacrt::blocked_by_spki(&[0xff; 32]).is_none());
    assert!(cacrt::blocked_by_cert(&[0; 32]).is_none());
}

// PEM decoding for the DER-level tests (`pem` reaches `base64` via `super::`).
#[cfg(feature = "check")]
#[path = "../build_support/base64.rs"]
mod base64;
#[cfg(feature = "check")]
#[path = "../build_support/pem.rs"]
mod pem;

#[cfg(feature = "check")]
mod check {
    use super::pem;
    use std::path::{Path, PathBuf};

    fn files(dir: &Path, ext: &str, out: &mut Vec<PathBuf>) {
        for e in std::fs::read_dir(dir).unwrap() {
            let path = e.unwrap().path();
            if path.is_dir() {
                files(&path, ext, out);
            } else if path.extension().and_then(|s| s.to_str()) == Some(ext) {
                out.push(path);
            }
        }
    }

    fn der(path: &Path) -> Vec<u8> {
        pem::read_one_certificate(&std::fs::read_to_string(path).unwrap()).unwrap()
    }

    fn blocked_files() -> Vec<PathBuf> {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut v = Vec::new();
        files(&root.join("roots"), "disabled", &mut v);
        files(&root.join("blocked"), "pem", &mut v);
        assert_eq!(v.len(), 19);
        v
    }

    #[test]
    fn every_blocked_file_is_caught_from_der() {
        for path in blocked_files() {
            let d = der(&path);
            assert!(
                cacrt::blocked_by_der(&d).unwrap().is_some(),
                "{} not blocked",
                path.display()
            );
        }
    }

    #[test]
    fn trusted_roots_pass() {
        let chain: Vec<&[u8]> = cacrt::all().iter().map(|c| c.der()).collect();
        cacrt::check_chain(&chain).expect("no embedded root may be blocked");
    }

    #[test]
    fn chain_with_os_supplied_disabled_root_is_rejected() {
        // A chain whose anchor came from an OS store that still trusts emSign
        // ECC Root CA - G3: the trusted root before it passes, the root fails.
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let disabled = der(&root.join("roots/eMudhra/emSign_ECC_Root_CA_G3.disabled"));
        let ok = cacrt::lookup("062cdee6.0").unwrap().der();
        match cacrt::check_chain(&[ok, &disabled]) {
            Err(cacrt::ChainError::Blocked { index: 1, entry }) => {
                assert_eq!(entry.label(), "emSign ECC Root CA - G3")
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn malformed_der_is_reported() {
        let ok = cacrt::lookup("062cdee6.0").unwrap().der();
        let truncated = &ok[..ok.len() - 1];
        for bad in [
            &[][..],
            &[0x30, 0x00][..],
            truncated,
            &[0x30, 0x85, 1, 1, 1, 1, 1][..],
        ] {
            assert!(cacrt::blocked_by_der(bad).is_err());
        }
        assert!(matches!(
            cacrt::check_chain(&[ok, truncated]),
            Err(cacrt::ChainError::Malformed { index: 1 })
        ));
    }
}
