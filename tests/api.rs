//! Public-API behavior tests for the embedded, curated root store.

extern crate alloc;
use alloc::format;

#[test]
fn store_is_non_empty_and_sorted() {
    let all = cacrt::all();
    assert!(!all.is_empty(), "expected embedded roots");
    assert_eq!(all.len(), cacrt::len());

    // Sorted by (subject_hash, seq), which the lookups rely on.
    for w in all.windows(2) {
        let a = (w[0].subject_hash(), w[0].seq());
        let b = (w[1].subject_hash(), w[1].seq());
        assert!(a < b, "not strictly ordered: {a:?} !< {b:?}");
    }
}

#[test]
fn lookup_by_openssl_name() {
    // GlobalSign Root CA - R3 is the canonical 062cdee6.0 example.
    let ca = cacrt::lookup("062cdee6.0").expect("062cdee6.0 present");
    assert_eq!(ca.subject_hash(), 0x062c_dee6);
    assert_eq!(ca.seq(), 0);
    assert!(!ca.der().is_empty());
    assert_eq!(format!("{}", ca.hash_name()), "062cdee6.0");
}

#[test]
fn malformed_names_return_none() {
    assert!(cacrt::lookup("062cdee6").is_none(), "missing .seq");
    assert!(
        cacrt::lookup("62cdee6.0").is_none(),
        "hash not 8 hex digits"
    );
    assert!(cacrt::lookup("zzzzzzzz.0").is_none(), "non-hex");
    assert!(cacrt::lookup("062cdee6.x").is_none(), "non-numeric seq");
    assert!(cacrt::lookup("ffffffff.0").is_none(), "no such hash");
}

#[test]
fn lookup_by_hash_groups_match_all() {
    for ca in cacrt::all() {
        let group = cacrt::lookup_by_hash(ca.subject_hash());
        assert!(
            group.iter().any(|c| core::ptr::eq(c, ca)),
            "{} not in its own hash group",
            ca.hash_name()
        );
        // Group is contiguous and shares the hash.
        assert!(group.iter().all(|c| c.subject_hash() == ca.subject_hash()));
    }
    assert!(cacrt::lookup_by_hash(0xffff_ffff).is_empty());
}

#[test]
fn lookup_matches_lookup_by_hash() {
    for ca in cacrt::all() {
        let by_name = cacrt::lookup(&format!("{}", ca.hash_name())).unwrap();
        assert!(core::ptr::eq(by_name, ca));
    }
}

#[test]
fn find_by_subject_resolves_each_root() {
    for ca in cacrt::all() {
        let found: alloc::vec::Vec<_> = cacrt::find_by_subject(ca.subject_der()).collect();
        assert!(
            found.iter().any(|c| core::ptr::eq(*c, ca)),
            "{} not found by its own subject",
            ca.hash_name()
        );
    }
    assert_eq!(cacrt::find_by_subject(b"not a real subject").count(), 0);
}

#[test]
fn subject_der_is_a_prefix_free_sequence_inside_der() {
    // Every stored subject must be a DER SEQUENCE and appear verbatim in the
    // full certificate (it is a slice of the TBSCertificate).
    for ca in cacrt::all() {
        assert_eq!(ca.subject_der().first(), Some(&0x30), "{}", ca.hash_name());
        let der = ca.der();
        let sub = ca.subject_der();
        assert!(
            der.windows(sub.len()).any(|w| w == sub),
            "{} subject not contained in its DER",
            ca.hash_name()
        );
    }
}
