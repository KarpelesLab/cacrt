# Root set changelog

A record of additions to and removals from the curated root set in `roots/`, so
the provenance of every trust anchor is auditable. See `CURATION.md` for the
rules that drive these decisions.

## 2026-06-04 — initial seed from Mozilla NSS

Seeded `roots/` from Mozilla NSS `certdata.txt`.

- **119 roots included** — every certificate with
  `CKA_TRUST_SERVER_AUTH = CKT_NSS_TRUSTED_DELEGATOR` that also passed the
  key-strength and expiry rules.
- **Excluded: non-TLS roots** — all certificates marked
  `CKT_NSS_MUST_VERIFY_TRUST` (S/MIME- or code-signing-only roots) were dropped
  per rule 1.
- **Excluded: phased-out for server auth** (rule 2 —
  `CKA_NSS_SERVER_DISTRUST_AFTER` set):
  - `Entrust Root Certification Authority`
  - `ePKI Root Certification Authority`

No entries are on the explicit deny list at this time.
