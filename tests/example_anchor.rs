//! Regression test for resolving the real trust anchor used by example.com and
//! example.org.
//!
//! As of 2026 both sites are served by Cloudflare and present this chain:
//!
//! ```text
//! leaf (CN=example.com / example.org)
//!   -> Cloudflare TLS Issuing ECC CA 3
//!   -> SSL.com TLS Transit ECC CA R2
//!   -> SSL.com TLS ECC Root CA 2022      <- the trust anchor
//! ```
//!
//! The last intermediate ("SSL.com TLS Transit ECC CA R2") names
//! "SSL.com TLS ECC Root CA 2022" as its issuer; a verifier resolves that
//! anchor by matching the issuer's raw DER `Name` against the embedded store.
//! This certificate is `865fbdf9.0`. (The cross-signed copy the server also
//! offers, issued by Comodo "AAA Certificate Services", shares the same subject
//! `Name` but is a different certificate — the modern anchor is the self-signed
//! root verified here by its DER.)

extern crate alloc;
use alloc::format;

/// The DER `Name` that example.com / example.org's last intermediate names as
/// its issuer — i.e. exactly what chain-building looks up. Captured verbatim
/// from the live chain; equal to the self-signed root's subject.
const ANCHOR_NAME_DER: &[u8] = &[
    48, 78, 49, 11, 48, 9, 6, 3, 85, 4, 6, 19, 2, 85, 83, 49, 24, 48, 22, 6, 3, 85, 4, 10, 12, 15,
    83, 83, 76, 32, 67, 111, 114, 112, 111, 114, 97, 116, 105, 111, 110, 49, 37, 48, 35, 6, 3, 85,
    4, 3, 12, 28, 83, 83, 76, 46, 99, 111, 109, 32, 84, 76, 83, 32, 69, 67, 67, 32, 82, 111, 111,
    116, 32, 67, 65, 32, 50, 48, 50, 50,
];

/// The full DER of the genuine self-signed "SSL.com TLS ECC Root CA 2022"
/// (SHA-256 `C3:2F:FD:9F:46:F9:36:D1:6C:36:73:99:09:59:43:4B:9A:D6:0A:AF:BB:9E:7C:F3:36:54:F1:44:CC:1B:A1:43`).
/// Resolving the anchor must yield exactly these bytes.
const ANCHOR_DER: &[u8] = &[
    48, 130, 2, 58, 48, 130, 1, 192, 160, 3, 2, 1, 2, 2, 16, 20, 3, 245, 171, 251, 55, 139, 23, 64,
    91, 226, 67, 178, 165, 209, 196, 48, 10, 6, 8, 42, 134, 72, 206, 61, 4, 3, 3, 48, 78, 49, 11,
    48, 9, 6, 3, 85, 4, 6, 19, 2, 85, 83, 49, 24, 48, 22, 6, 3, 85, 4, 10, 12, 15, 83, 83, 76, 32,
    67, 111, 114, 112, 111, 114, 97, 116, 105, 111, 110, 49, 37, 48, 35, 6, 3, 85, 4, 3, 12, 28,
    83, 83, 76, 46, 99, 111, 109, 32, 84, 76, 83, 32, 69, 67, 67, 32, 82, 111, 111, 116, 32, 67,
    65, 32, 50, 48, 50, 50, 48, 30, 23, 13, 50, 50, 48, 56, 50, 53, 49, 54, 51, 51, 52, 56, 90, 23,
    13, 52, 54, 48, 56, 49, 57, 49, 54, 51, 51, 52, 55, 90, 48, 78, 49, 11, 48, 9, 6, 3, 85, 4, 6,
    19, 2, 85, 83, 49, 24, 48, 22, 6, 3, 85, 4, 10, 12, 15, 83, 83, 76, 32, 67, 111, 114, 112, 111,
    114, 97, 116, 105, 111, 110, 49, 37, 48, 35, 6, 3, 85, 4, 3, 12, 28, 83, 83, 76, 46, 99, 111,
    109, 32, 84, 76, 83, 32, 69, 67, 67, 32, 82, 111, 111, 116, 32, 67, 65, 32, 50, 48, 50, 50, 48,
    118, 48, 16, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 5, 43, 129, 4, 0, 34, 3, 98, 0, 4, 69, 41,
    53, 115, 250, 194, 184, 35, 206, 20, 125, 168, 177, 77, 160, 91, 54, 238, 42, 44, 83, 195, 96,
    9, 53, 178, 36, 102, 38, 105, 192, 179, 149, 214, 93, 146, 64, 25, 14, 198, 165, 19, 112, 244,
    239, 18, 81, 40, 93, 231, 204, 189, 249, 60, 133, 193, 207, 148, 144, 201, 43, 206, 146, 66,
    88, 89, 103, 253, 148, 39, 16, 100, 140, 79, 4, 177, 77, 73, 228, 123, 79, 155, 245, 231, 8,
    248, 3, 136, 247, 167, 195, 146, 75, 25, 84, 129, 163, 99, 48, 97, 48, 15, 6, 3, 85, 29, 19, 1,
    1, 255, 4, 5, 48, 3, 1, 1, 255, 48, 31, 6, 3, 85, 29, 35, 4, 24, 48, 22, 128, 20, 137, 143, 47,
    163, 232, 43, 160, 20, 84, 123, 243, 86, 184, 38, 95, 103, 56, 11, 156, 208, 48, 29, 6, 3, 85,
    29, 14, 4, 22, 4, 20, 137, 143, 47, 163, 232, 43, 160, 20, 84, 123, 243, 86, 184, 38, 95, 103,
    56, 11, 156, 208, 48, 14, 6, 3, 85, 29, 15, 1, 1, 255, 4, 4, 3, 2, 1, 134, 48, 10, 6, 8, 42,
    134, 72, 206, 61, 4, 3, 3, 3, 104, 0, 48, 101, 2, 48, 85, 227, 34, 86, 233, 215, 146, 36, 88,
    79, 30, 148, 50, 15, 12, 2, 54, 194, 253, 172, 116, 50, 78, 225, 251, 28, 128, 136, 163, 204,
    251, 215, 235, 43, 255, 55, 125, 240, 237, 215, 158, 117, 106, 53, 118, 82, 69, 224, 2, 49, 0,
    199, 141, 111, 66, 32, 143, 190, 182, 77, 89, 237, 119, 77, 41, 196, 32, 32, 69, 100, 134, 58,
    80, 198, 196, 173, 45, 147, 245, 24, 125, 114, 237, 169, 207, 196, 172, 87, 54, 40, 8, 101,
    223, 60, 121, 102, 126, 160, 234,
];

/// Resolve the anchor by the issuer `Name` the chain points at — the way a
/// verifier building example.com / example.org's chain would.
#[test]
fn resolves_example_dot_com_and_org_anchor_by_issuer_name() {
    let anchor = cacrt::find_by_subject(ANCHOR_NAME_DER)
        .next()
        .expect("SSL.com TLS ECC Root CA 2022 must be embedded for example.com/.org");

    assert_eq!(format!("{}", anchor.hash_name()), "865fbdf9.0");
    assert_eq!(anchor.subject_hash(), 0x865f_bdf9);
    assert_eq!(anchor.seq(), 0);
    assert_eq!(anchor.label(), "SSL.com TLS ECC Root CA 2022");

    // The stored subject is byte-identical to the issuer Name in the chain,
    // and the resolved certificate is the genuine self-signed root.
    assert_eq!(anchor.subject_der(), ANCHOR_NAME_DER);
    assert_eq!(anchor.der(), ANCHOR_DER);
}

/// The same anchor is reachable by its OpenSSL hash name, and both routes
/// return the very same entry.
#[test]
fn anchor_reachable_by_openssl_hash_name() {
    let by_name = cacrt::lookup("865fbdf9.0").expect("865fbdf9.0 present");
    assert_eq!(by_name.der(), ANCHOR_DER);

    let by_subject = cacrt::find_by_subject(ANCHOR_NAME_DER).next().unwrap();
    assert!(core::ptr::eq(by_name, by_subject));

    // And it sits in its subject-hash group.
    let group = cacrt::lookup_by_hash(0x865f_bdf9);
    assert!(group.iter().any(|c| core::ptr::eq(c, by_name)));
}
