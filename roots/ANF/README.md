# ANF

ANF Autoridad de Certificación (ANF AC) is a Spanish Qualified Trust Service Provider (QTSP) headquartered in Barcelona, Spain, accredited under the EU eIDAS Regulation (EU 910/2014) and in operation since approximately 2000. It operates under the legal entity ANF Autoridad de Certificacion (CIF G63287510) and serves customers in Spain and over 30 countries, issuing TLS/SSL, electronic-signature, seal, and timestamping certificates. This folder consolidates the one Web-PKI root operated under the ANF Autoridad de Certificacion brand.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| ANF Secure Server Root CA | b433981b.0 | RSA 4096 | 2039-08-30 | `FB:8F:EC:75:91:69:B9:10:6B:1E:51:16:44:C6:18:C5:13:04:37:3F:6C:06:43:08:8D:8B:EF:FD:1B:99:75:99` |

## Rationale for inclusion

The ANF Secure Server Root CA is included in the **Mozilla NSS** root store with the Websites (TLS server authentication) trust bit enabled, and Extended Validation (EV) treatment was also granted (deployed in Firefox 90 / NSS 3.66, June 2021). The root is likewise present in the **Microsoft Trusted Root Certificate Program** and the **Apple** root store, and it appears in the **Chrome Root Store** (as confirmed in `root_store.md` at the Chromium source repository).

Mozilla's inclusion was approved in Bugzilla bug [#1585951](https://bugzilla.mozilla.org/show_bug.cgi?id=1585951) and carried through by [#1703942](https://bugzilla.mozilla.org/show_bug.cgi?id=1703942) (NSS) and [#1703944](https://bugzilla.mozilla.org/show_bug.cgi?id=1703944) (EV treatment). The scope of trust is TLS server authentication (DVCP, OVCP, EVCP).

## CA/Browser Forum compliance

ANF AC is audited under the **ETSI EN 319 411-1** (NCP+/OVCP/EVCP) and **ETSI EN 319 411-2** (QCP-w for qualified certificates) frameworks, with audits performed by CSQA (an accredited conformity assessment body). Annual audit attestation letters are disclosed in the CCADB. The CP/CPS documents are publicly available on the ANF website; the current CPS is version 3.7 (published September 2025).

ANF AC's CPS commits to CA/B Forum Baseline Requirements compliance and EV Guidelines compliance for TLS issuance. During the Mozilla inclusion review, the auditor updated the attestation to explicitly confirm EVCP coverage alongside QCP-w (Mozilla requires explicit EVCP, which is not implied by QCP-w alone). ANF AC has stated it subscribes to Mozilla Bugzilla CA compliance notifications and employs automated pre-issuance linting (cablint, x509lint, zlint). Certificate Transparency (SCT embedding) is required for publicly-trusted TLS certificates per the Baseline Requirements and browser CT policies.

Sources: [Mozilla Bug #1585951](https://bugzilla.mozilla.org/show_bug.cgi?id=1585951); [CPS v3.7](https://anf.es/wp-content/uploads/2025/09/Certification-Practices-Statement-ANF-AC-v.3.7.pdf)

## Past non-compliance

**2019 misissuance (old hierarchy — never Mozilla-trusted):** During the public discussion for bug [#1585951](https://bugzilla.mozilla.org/show_bug.cgi?id=1585951), community reviewers documented that ANF misissued dozens of certificates from its older hierarchy (tracked in the earlier [Bug #555156](https://bugzilla.mozilla.org/show_bug.cgi?id=555156) inclusion attempt), violating the CA/B Forum Baseline Requirements. The incident report revealed multiple PKI misunderstandings: incorrect use of the CA/B Forum Test Certificate OID, confusion between subject serial number and certificate serial number entropy requirements, and inadequate domain validation controls. That older hierarchy was never included in the Mozilla root store; ANF withdrew the earlier inclusion request and built a new hierarchy for the current application. Reviewers, including Ryan Sleevi, nonetheless raised serious concerns about whether root causes had been addressed before approving the new hierarchy. Mozilla ultimately approved inclusion after ANF demonstrated organizational changes (new technical team, automated controls).

**2025 ETSI audit finding — information security policy version mismatch:** In [Bugzilla Bug #1970565](https://bugzilla.mozilla.org/show_bug.cgi?id=1970565), ANF AC reported a non-conformity identified during its annual ETSI EN 319 401 conformity assessment audit (finding identified March 13, 2025). A website maintenance rollback on February 26, 2025 had accidentally restored an outdated version (v1.6) of the Information Security Policy instead of the current approved version (v1.7). The correct version was restored on March 14, 2025. The issue was documentation-only, with no impact on certificate issuance. ANF AC implemented post-rollback verification procedures and automated policy-version comparison controls. The bug was resolved and closed.

No distrust actions by any major root program have been identified for the ANF Secure Server Root CA. For a current view of open compliance bugs, see the [Mozilla Bugzilla CA Certificate Compliance component](https://bugzilla.mozilla.org/buglist.cgi?product=CA%20Certificate%20Compliance&component=ANF%20AC).

## Transparency

- **CP/CPS:** Publicly available at [https://anf.es/en/legal-repository/](https://anf.es/en/legal-repository/) and directly at [CPS v3.7](https://anf.es/wp-content/uploads/2025/09/Certification-Practices-Statement-ANF-AC-v.3.7.pdf). Documents are structured per ETSI standards and versioned.
- **CCADB disclosure:** ANF AC is listed in the Common CA Database (CCADB) with audit attestation letters uploaded and current. The root inclusion and EV treatment were managed through the CCADB workflow.
- **Incident self-reporting:** The 2025 documentation finding (Bug #1970565) was self-reported by ANF AC to Mozilla within the required window, and remediation details were published in the bug.
- **Certificate Transparency:** TLS certificates issued under this root are logged in public CT logs per CA/B Forum BRs and browser CT enforcement requirements. During the 2021 inclusion review, it was noted that the new hierarchy had issued zero end-entity certificates known to CT at that time; current production certificates are expected to carry embedded SCTs.

## Sources

- [ANF AC official website](https://anf.es/en/)
- [ANF AC root certificates page](https://anf.es/en/ac-root/)
- [ANF AC CPS v3.7 (September 2025)](https://anf.es/wp-content/uploads/2025/09/Certification-Practices-Statement-ANF-AC-v.3.7.pdf)
- [Mozilla Bugzilla #555156 — Add ANF root certificate (old hierarchy, withdrawn)](https://bugzilla.mozilla.org/show_bug.cgi?id=555156)
- [Mozilla Bugzilla #1585951 — Add ANF AC root certificates (inclusion approved)](https://bugzilla.mozilla.org/show_bug.cgi?id=1585951)
- [Mozilla Bugzilla #1703942 — Add ANF Secure Server Root CA to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=1703942)
- [Mozilla Bugzilla #1703944 — Enable EV Treatment for ANF Secure Server Root CA](https://bugzilla.mozilla.org/show_bug.cgi?id=1703944)
- [Mozilla Bugzilla #1970565 — ANF AC: Finding #2 ETSI Audit - Information security policy not updated](https://bugzilla.mozilla.org/show_bug.cgi?id=1970565)
- [Public discussion: Inclusion of the ANF Secure Server Root CA (mozilla.dev.security.policy)](https://groups.google.com/g/mozilla.dev.security.policy/c/MLyRWsLHAdA)
- [Chrome Root Store root_store.md](https://chromium.googlesource.com/chromium/src/+/main/net/data/ssl/chrome_root_store/root_store.md)
- [ANF AC — PKI Consortium member profile](https://pkic.org/members/anf-ac/)
- [CCADB resources](https://www.ccadb.org/resources)
