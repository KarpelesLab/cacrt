# Curation rules

This document defines which CA root certificates `cacrt` accepts and why. The
rules are enforced in code (`build_support/curation.rs`, exercised by `build.rs`
and the `cacrt-tool verify` command) and are intended to track current
CA/Browser Forum and root-program best practice.

The source of truth for the trusted set is the collection of per-root PEM files
in [`roots/`](roots/). Those files are seeded from Mozilla NSS and then
maintained by hand; this document explains the policy applied when deciding what
belongs there.

## Source

- Roots are seeded from **Mozilla's NSS `certdata.txt`** root store — the
  de-facto trust store used by `ca-certificates`, curl, and most of the open
  Linux ecosystem.
- `certdata.txt` is distributed by Mozilla under the **Mozilla Public License
  2.0**. The certificates themselves are public data.

## Acceptance rules

A certificate is included only if it passes **all** of the following. Rules 1, 3,
4 are machine-checkable and re-verified on every build / in CI.

1. **TLS server-auth trust anchor.** The certificate must have an NSS trust
   record with `CKA_TRUST_SERVER_AUTH = CKT_NSS_TRUSTED_DELEGATOR`. Roots marked
   `CKT_NSS_MUST_VERIFY_TRUST` (e.g. e-mail/SMIME- or code-signing-only roots)
   are **excluded** — they are not trust anchors for TLS.

   *Rationale:* CA/Browser Forum Baseline Requirements and the browser root
   programs scope trust by purpose; a root trusted only for S/MIME must not be a
   TLS anchor.

2. **No active phase-out.** Certificates carrying NSS
   `CKA_NSS_SERVER_DISTRUST_AFTER` (a graceful server-auth distrust date) are
   **excluded**.

   *Rationale:* a distrust-after date is the root programs signalling that the CA
   is being wound down for TLS. A curated, forward-looking store should not ship
   anchors already on the way out. See `CHANGELOG-roots.md` for the specific
   roots this removes.

3. **Not expired.** Certificates whose `notAfter` is in the past at build /
   verification time are **excluded**.

   *Rationale:* an expired root is dead weight and can mask configuration errors.

4. **Adequate key strength.** RSA keys must be **≥ 2048 bits**; ECC keys must be
   on **P-256, P-384, or P-521**. DSA and unrecognized key algorithms are
   **excluded**.

   *Rationale:* CA/Browser Forum BR §6.1.5 minimum key sizes; DSA is obsolete for
   the Web PKI.

5. **Self-signature algorithm is recorded but NOT used to exclude.** A root's own
   self-signature may use SHA-1 and the root is still eligible.

   *Rationale:* a root's self-signature is **not validated** by relying parties —
   trust comes from the root being in the store, not from its signature. The
   SHA-1 prohibition in the BRs and root-program policies applies to subscriber
   and intermediate certificates, not to root self-signatures. Excluding SHA-1
   self-signed roots would wrongly drop many still-trusted, widely-deployed roots.

6. **Explicit deny list.** Specific CAs may be excluded beyond the above via a
   documented deny list (`DENY_LABEL_SUBSTRINGS` in `build_support/curation.rs`).
   Mozilla already removes fully-distrusted CAs from `certdata.txt`, so this list
   is for deliberate, additional removals. Every entry must have a matching note
   in `CHANGELOG-roots.md`. The list is currently empty.

7. **Dedicated-TLS preference (guidance, not auto-enforced).** Modern root
   programs (notably the Chrome Root Program) favor hierarchies dedicated to TLS
   server authentication. This cannot be fully derived from `certdata.txt`, so it
   is applied as reviewer guidance when adding roots rather than a hard automated
   gate.

## Disabling a root

To retire a root we have already shipped, **rename its `.pem` file to
`.disabled`** (e.g. `Buypass_Class_3_Root_CA.pem` → `Buypass_Class_3_Root_CA.disabled`).
The build only compiles `*.pem`, so a `.disabled` file is excluded from the
generated store — but it stays in the repository, preserving the certificate
itself, its company `README.md`, and the recorded rationale. This keeps the audit
trail intact (we can always see what was once trusted and why it was dropped)
without deleting history. Record every such change in `CHANGELOG-roots.md`, and
add a status banner to the company's `README.md`.

Typical reasons to disable rather than keep a root: the CA has stopped issuing
TLS certificates and all of its end-entity certificates have since expired (so the
anchor can no longer serve any live chain), or a root program has distrusted it.

## Auditability

Every addition or removal of a root is recorded in
[`CHANGELOG-roots.md`](CHANGELOG-roots.md) with a date and a reason, so the
provenance of the trusted set is reviewable over time.

## References

- CA/Browser Forum, *Baseline Requirements for the Issuance and Management of
  Publicly-Trusted TLS Server Certificates* — <https://cabforum.org/working-groups/server/baseline-requirements/requirements/>
- Mozilla Root Store Policy — <https://www.mozilla.org/en-US/about/governance/policies/security-group/certs/policy/>
- Mozilla CA Certificate Change Requests (inclusion/removal process) — <https://wiki.mozilla.org/CA/Certificate_Change_Requests>
- Mozilla *Required or Recommended Practices* — <https://wiki.mozilla.org/CA/Required_or_Recommended_Practices>
- Chrome Root Program Policy — <https://googlechrome.github.io/chromerootprogram/>
- Common CA Database (CCADB) — <https://www.ccadb.org/>
- OpenSSL subject hash: `crypto/x509/x_name.c` (`x509_name_canon`), `crypto/x509/x509_cmp.c` (`X509_NAME_hash_ex`)
