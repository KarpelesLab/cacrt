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

## 2026-06-04 — disabled Buypass (TLS wind-down complete)

Disabled both **Buypass** roots (`Buypass Class 2 Root CA`, `Buypass Class 3 Root
CA`) by renaming them to `*.disabled`, so they are excluded from the compiled
store but retained for the audit trail.

Rationale: Buypass announced (2025-08-18) it is discontinuing public TLS/SSL
certificate issuance — last issuance 2025-10-31, with all Buypass TLS/SSL
certificates expiring no later than **2026-04-15**. That date has now passed, so
no valid Buypass-issued TLS server certificate can exist in the wild and the
roots serve no purpose as trust anchors. The certificate files and the company
`README.md` (with full rationale and sources) are kept in `roots/Buypass/`.
See <https://www.buypass.com/products/tls-ssl-certificates/discontinues-issuance-of-tls-ssl-certificates>.
