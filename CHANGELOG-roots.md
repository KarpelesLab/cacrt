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

## 2026-10-24 — blocklist: disabled roots and eMudhra intermediates

Added a blocklist that ships in the crate (`cacrt::blocked()`, see `CURATION.md`
"Blocking intermediates"). It starts with 19 certificates:

- the 3 disabled roots: **emSign ECC Root CA - G3**, **Buypass Class 2 Root CA**
  and **Buypass Class 3 Root CA**;
- the 16 compromised eMudhra intermediate certificates (8 keys, each issued by
  emSign Root TLS/CS CA - G3 and cross-signed by Sectigo AAA Certificate
  Services), in `blocked/2026-10-emudhra-intermediates/`.

Callers that check every certificate in their chain now reject these even when
their anchor comes from an OS store that still trusts them.

## 2026-10-21 — disabled emSign ECC Root CA - G3 (intermediate key compromise)

Disabled **emSign ECC Root CA - G3** (eMudhra Technologies Limited, SHA-256
`86A1ECBA089C4A8D3BBE2734C612BA341D813E043CF9E8A862CD5C57A36BBE6B`) by renaming
it to `*.disabled`.

Rationale: on 2026-10-17 the private keys of eight eMudhra intermediate CA
certificates were reported to the root programs as compromised. The reporter
proved it by signing a test message with each key. The keys were not stolen:
eMudhra generated them badly. Each intermediate's P-384 private scalar is a tiny
integer, so anyone can recover it by computing `k·G` for small `k` and comparing
the result with the certificate's public key:

| Intermediate | Private key |
|---|---|
| EM OV TLS CA - G3C | 4 |
| EM EV TLS CA - G3C | 56 |
| EM OV CS CA - G3B | 66 |
| EM EV TLS CA - G3B | 115 |
| EM OV TLS CA - G3B | 155 |
| EM DV TLS CA - G3B | 156 |
| EM DV TLS CA - G3C | 230 |
| EM OV CS CA - G3C | 237 |

We checked each value against the public keys in the incident's
`impacted-intermediates.json`. The keys date back to issuance (April 2025), so
the intermediates were never secure. All 16 certificates (each key was certified
by its eMudhra root and cross-signed by Sectigo) and the 8 private keys are
archived, with a verification script, in
[`evidence/2026-10-emudhra-intermediates/`](evidence/2026-10-emudhra-intermediates/).

These intermediates chain to this root. Mozilla blocked them through OneCRL and
is removing the root in
NSS 3.131.1. Chrome blocked them with a CRLSet and is removing the root in Chrome
Root Store version 43. Apple and Microsoft acted on 2026-10-19 and 2026-10-20.
`cacrt` has no revocation channel of its own, so we follow Mozilla's guidance for
downstream stores: distrust the root.

The other roots named in the incident's `roots.json` are not in this store. Those
are `emSign Root TLS CA - G3` and `emSign Root CS CA - G3` (never in Mozilla),
plus Sectigo's `AAA Certificate Services` (removed from Mozilla earlier), which
cross-signed the affected keys. The other three eMudhra roots (G1, C1, C3) are
not named in the incident and stay trusted.

- <https://blog.mozilla.org/security/2026/10/20/protecting-firefox-users-following-the-compromise-of-emudhra-intermediate-certificates/>
- <https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/0R3XLJL6FuE/m/0XofdebrAAAJ>
- <https://emudhra.we-have-a-problem.com/>
