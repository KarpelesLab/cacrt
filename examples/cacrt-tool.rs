//! Maintenance tooling for the curated root set. Not part of the published
//! library API — run with `cargo run --example cacrt-tool -- <cmd>`.
//!
//! Subcommands:
//!   import <certdata.txt>   Seed/refresh roots/ from a Mozilla NSS certdata.txt.
//!   verify                  Re-check every roots/*.pem against curation rules.
//!   diff <certdata.txt>     Report adds/removes vs the current roots/ set.
//!   hash <cert.pem|der>     Print the OpenSSL subject hash of one certificate.
//!
//! The roots/ directory is the source of truth; `import` proposes changes, a
//! human reviews and commits them.

#[path = "../build_support/mod.rs"]
mod build_support;

use build_support::{curation, octal, pem, sha1, subject_hash};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        Some("import") => cmd_import(args.get(1)),
        Some("verify") => cmd_verify(),
        Some("diff") => cmd_diff(args.get(1)),
        Some("hash") => cmd_hash(args.get(1)),
        _ => {
            eprintln!("usage: cacrt-tool <import|verify|diff|hash> [path]");
            std::process::exit(2);
        }
    };
    if let Err(e) = result {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn roots_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("roots")
}

// ---------------------------------------------------------------------------
// certdata.txt model
// ---------------------------------------------------------------------------

#[derive(Default, Debug)]
struct CertObj {
    label: String,
    value: Vec<u8>,       // DER certificate (CKA_VALUE)
    distrust_after: bool, // CKA_NSS_SERVER_DISTRUST_AFTER is set
}

#[derive(Default, Debug)]
struct TrustObj {
    sha1: Vec<u8>,       // CKA_CERT_SHA1_HASH
    server_auth: String, // CKA_TRUST_SERVER_AUTH enum
}

struct Certdata {
    certs: Vec<CertObj>,
    trust: Vec<TrustObj>,
}

/// Parse the subset of certdata.txt we care about.
fn parse_certdata(text: &str) -> Result<Certdata, String> {
    let mut certs = Vec::new();
    let mut trust = Vec::new();

    let mut lines = text.lines().peekable();
    // Attributes of the object currently being assembled.
    let mut class = String::new();
    let mut attrs: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    let mut str_attrs: BTreeMap<String, String> = BTreeMap::new();

    let flush = |class: &str,
                 attrs: &BTreeMap<String, Vec<u8>>,
                 str_attrs: &BTreeMap<String, String>,
                 certs: &mut Vec<CertObj>,
                 trust: &mut Vec<TrustObj>| {
        match class {
            "CKO_CERTIFICATE" => certs.push(CertObj {
                label: str_attrs.get("CKA_LABEL").cloned().unwrap_or_default(),
                value: attrs.get("CKA_VALUE").cloned().unwrap_or_default(),
                distrust_after: attrs.contains_key("CKA_NSS_SERVER_DISTRUST_AFTER"),
            }),
            "CKO_NSS_TRUST" => trust.push(TrustObj {
                sha1: attrs.get("CKA_CERT_SHA1_HASH").cloned().unwrap_or_default(),
                server_auth: str_attrs
                    .get("CKA_TRUST_SERVER_AUTH")
                    .cloned()
                    .unwrap_or_default(),
            }),
            _ => {}
        }
    };

    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed == "BEGINDATA" {
            continue;
        }
        let mut it = trimmed.splitn(3, ' ');
        let name = it.next().unwrap_or("");
        if !name.starts_with("CKA_") {
            continue;
        }
        let ty = it.next().unwrap_or("");
        let rest = it.next().unwrap_or("");

        // A new object begins at CKA_CLASS — flush the previous one.
        if name == "CKA_CLASS" {
            if !class.is_empty() {
                flush(&class, &attrs, &str_attrs, &mut certs, &mut trust);
            }
            class = rest.trim().to_string();
            attrs.clear();
            str_attrs.clear();
            continue;
        }

        if ty == "MULTILINE_OCTAL" {
            let mut body = String::new();
            for l in lines.by_ref() {
                if l.trim() == "END" {
                    break;
                }
                body.push_str(l);
                body.push('\n');
            }
            attrs.insert(name.to_string(), octal::decode(&body)?);
        } else if ty == "UTF8" || ty.starts_with("CK_") {
            // Store the human/enum value; strip surrounding quotes for UTF8.
            let v = rest.trim();
            let v = v
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .unwrap_or(v);
            str_attrs.insert(name.to_string(), v.to_string());
        }
    }
    if !class.is_empty() {
        flush(&class, &attrs, &str_attrs, &mut certs, &mut trust);
    }
    Ok(Certdata { certs, trust })
}

// ---------------------------------------------------------------------------
// import
// ---------------------------------------------------------------------------

fn cmd_import(path: Option<&String>) -> Result<(), String> {
    let path = path.ok_or("import needs a path to certdata.txt")?;
    let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let data = parse_certdata(&text)?;

    // Index trust by the SHA-1 of the certificate it refers to.
    let mut trust_by_sha1: BTreeMap<Vec<u8>, &TrustObj> = BTreeMap::new();
    for t in &data.trust {
        if !t.sha1.is_empty() {
            trust_by_sha1.insert(t.sha1.clone(), t);
        }
    }

    let now = curation::now_yyyymmddhhmmss();
    let dir = roots_dir();
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let mut used_names: BTreeMap<String, u32> = BTreeMap::new();
    let (mut written, mut skipped) = (0u32, 0u32);

    for cert in &data.certs {
        let fp = sha1::sha1(&cert.value);
        let server_auth = trust_by_sha1
            .get(fp.as_slice())
            .map(|t| t.server_auth.as_str());

        match curation::evaluate(
            cert.distrust_after,
            &cert.label,
            server_auth,
            &cert.value,
            now,
        ) {
            curation::Decision::Accept => {}
            curation::Decision::Reject(reason) => {
                skipped += 1;
                eprintln!("skip  {:<45} {reason}", truncate(&cert.label, 45));
                continue;
            }
        }

        let hash = subject_hash(&cert.value).map_err(|e| e.to_string())?;
        let mut base = sanitize(&cert.label);
        let n = used_names.entry(base.clone()).or_insert(0);
        if *n > 0 {
            base = format!("{base}-{n}");
        }
        *used_names.get_mut(&sanitize(&cert.label)).unwrap() += 1;

        let file = dir.join(format!("{base}.pem"));
        std::fs::write(&file, render_pem(&cert.label, hash, &fp, &cert.value))
            .map_err(|e| e.to_string())?;
        written += 1;
        println!("ok    {:08x}  {}", hash, cert.label);
    }

    eprintln!("\nimported {written} roots, skipped {skipped} (see reasons above)");
    Ok(())
}

fn render_pem(label: &str, hash: u32, fp_sha1: &[u8; 20], der: &[u8]) -> String {
    let fp = fp_sha1
        .iter()
        .map(|b| format!("{b:02X}"))
        .collect::<Vec<_>>()
        .join(":");
    let mut out = String::new();
    out.push_str(&format!("# Label: {label}\n"));
    out.push_str(&format!("# OpenSSL subject hash: {hash:08x}\n"));
    out.push_str(&format!("# SHA1 fingerprint: {fp}\n"));
    out.push_str("# Source: Mozilla NSS certdata.txt (CKT_NSS_TRUSTED_DELEGATOR, server auth)\n");
    out.push_str(&pem::write_certificate(der));
    out
}

// ---------------------------------------------------------------------------
// verify
// ---------------------------------------------------------------------------

fn cmd_verify() -> Result<(), String> {
    let mut problems = 0u32;
    let mut count = 0u32;
    for (path, der) in load_roots()? {
        count += 1;
        // Re-run the machine-checkable rules (key strength, expiry, structure).
        if let Err(reason) = curation::check_cert(&der, curation::now_yyyymmddhhmmss()) {
            problems += 1;
            eprintln!("FAIL {}: {reason}", path.display());
        }
        if subject_hash(&der).is_err() {
            problems += 1;
            eprintln!("FAIL {}: cannot compute subject hash", path.display());
        }
    }
    if problems > 0 {
        return Err(format!("{problems} problem(s) across {count} roots"));
    }
    println!("verified {count} roots, all pass curation rules");
    Ok(())
}

// ---------------------------------------------------------------------------
// diff
// ---------------------------------------------------------------------------

fn cmd_diff(path: Option<&String>) -> Result<(), String> {
    let path = path.ok_or("diff needs a path to certdata.txt")?;
    let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let data = parse_certdata(&text)?;

    let mut trust_by_sha1: BTreeMap<Vec<u8>, &TrustObj> = BTreeMap::new();
    for t in &data.trust {
        trust_by_sha1.insert(t.sha1.clone(), t);
    }
    let now = curation::now_yyyymmddhhmmss();
    let mut incoming: BTreeMap<[u8; 20], String> = BTreeMap::new();
    for cert in &data.certs {
        let fp = sha1::sha1(&cert.value);
        let server_auth = trust_by_sha1
            .get(fp.as_slice())
            .map(|t| t.server_auth.as_str());
        if matches!(
            curation::evaluate(
                cert.distrust_after,
                &cert.label,
                server_auth,
                &cert.value,
                now
            ),
            curation::Decision::Accept
        ) {
            incoming.insert(fp, cert.label.clone());
        }
    }

    let mut current: BTreeMap<[u8; 20], PathBuf> = BTreeMap::new();
    for (path, der) in load_roots()? {
        current.insert(sha1::sha1(&der), path);
    }

    for (fp, label) in &incoming {
        if !current.contains_key(fp) {
            println!("+ {label}");
        }
    }
    for (fp, path) in &current {
        if !incoming.contains_key(fp) {
            println!("- {}", path.display());
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// hash
// ---------------------------------------------------------------------------

fn cmd_hash(path: Option<&String>) -> Result<(), String> {
    let path = path.ok_or("hash needs a path to a cert (PEM or DER)")?;
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    let der = match std::str::from_utf8(&bytes) {
        Ok(text) if text.contains("-----BEGIN CERTIFICATE-----") => {
            pem::read_one_certificate(text)?
        }
        _ => bytes,
    };
    println!("{:08x}", subject_hash(&der).map_err(|e| e.to_string())?);
    Ok(())
}

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

fn load_roots() -> Result<Vec<(PathBuf, Vec<u8>)>, String> {
    let dir = roots_dir();
    let mut out = Vec::new();
    let entries = match std::fs::read_dir(&dir) {
        Ok(e) => e,
        Err(_) => return Ok(out),
    };
    for e in entries {
        let path = e.map_err(|e| e.to_string())?.path();
        if path.extension().and_then(|s| s.to_str()) != Some("pem") {
            continue;
        }
        let text = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let der =
            pem::read_one_certificate(&text).map_err(|e| format!("{}: {e}", path.display()))?;
        out.push((path, der));
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(out)
}

fn sanitize(label: &str) -> String {
    let mut s: String = label
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    while s.contains("__") {
        s = s.replace("__", "_");
    }
    s.trim_matches('_').to_string()
}

fn truncate(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        s.to_string()
    } else {
        let prefix: String = s.chars().take(n.saturating_sub(1)).collect();
        format!("{prefix}…")
    }
}
