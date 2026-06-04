//! Cross-checks our build-time subject-hash implementation against the system
//! `openssl`, and re-verifies the committed root set against the curation rules.
//!
//! Including `build_support` here (a test crate, so `cfg(test)` is set) also runs
//! the helpers' own unit tests (SHA-1 vectors, base64 round-trips, etc.).

#[path = "../build_support/mod.rs"]
mod build_support;

use build_support::{curation, pem, subject_hash};
use std::path::{Path, PathBuf};
use std::process::Command;

fn roots() -> Vec<PathBuf> {
    let mut v = Vec::new();
    collect_pems(&Path::new(env!("CARGO_MANIFEST_DIR")).join("roots"), &mut v);
    v.sort();
    v
}

/// Recursively gather `*.pem` paths under `dir` (roots are nested per company).
fn collect_pems(dir: &Path, out: &mut Vec<PathBuf>) {
    for e in std::fs::read_dir(dir).expect("roots dir") {
        let path = e.unwrap().path();
        if path.is_dir() {
            collect_pems(&path, out);
        } else if path.extension().and_then(|s| s.to_str()) == Some("pem") {
            out.push(path);
        }
    }
}

fn openssl_available() -> bool {
    Command::new("openssl")
        .arg("version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

#[test]
fn subject_hash_matches_openssl() {
    if !openssl_available() {
        eprintln!("skipping: openssl not available");
        return;
    }
    let mut checked = 0;
    for path in roots() {
        let text = std::fs::read_to_string(&path).unwrap();
        let der = pem::read_one_certificate(&text).unwrap();
        let ours = format!("{:08x}", subject_hash(&der).unwrap());

        let out = Command::new("openssl")
            .args(["x509", "-subject_hash", "-noout", "-in"])
            .arg(&path)
            .output()
            .expect("run openssl");
        let theirs = String::from_utf8(out.stdout).unwrap().trim().to_string();

        assert_eq!(ours, theirs, "subject hash mismatch for {}", path.display());
        checked += 1;
    }
    assert!(checked > 0, "no roots were cross-checked");
    eprintln!("cross-checked {checked} roots against openssl");
}

#[test]
fn committed_roots_pass_curation() {
    let now = curation::now_yyyymmddhhmmss();
    let mut problems = Vec::new();
    for path in roots() {
        let text = std::fs::read_to_string(&path).unwrap();
        let der = pem::read_one_certificate(&text).unwrap();
        if let Err(reason) = curation::check_cert(&der, now) {
            problems.push(format!("{}: {reason}", path.display()));
        }
    }
    assert!(
        problems.is_empty(),
        "curation failures:\n{}",
        problems.join("\n")
    );
}
