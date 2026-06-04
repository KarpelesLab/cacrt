# TunTrust

TunTrust is the public-facing brand of the **Agence Nationale de Certification Electronique (ANCE)**, a government-owned certificate authority established under Tunisian Law No. 2000-83 of 9 August 2000 on electronic exchanges and commerce, headquartered at Technopark El Ghazala, Ariana, Tunisia. ANCE operates Tunisia's national PKI and issues publicly-trusted TLS server certificates scoped to the `.tn` country-code top-level domain; the TunTrust Root CA in this folder is the sole Web-PKI-trusted root under that brand.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| TunTrust_Root_CA.pem | fd64f3fc.0 | RSA 4096 | 2044-04-26 | `2E:44:10:2A:B5:8C:B8:54:19:45:1C:8E:19:D9:AC:F3:66:2C:AF:BC:61:4B:6A:53:96:0A:30:F7:D0:E2:EB:41` |

Subject: `C=TN, O=Agence Nationale de Certification Electronique, CN=TunTrust Root CA`
Self-signed; valid 2019-04-26 – 2044-04-26.

## Rationale for inclusion

The TunTrust Root CA is trusted for **TLS server authentication (OV; Websites trust bit)** in the following root programs:

- **Mozilla Firefox** — approved in [Bugzilla #1587779](https://bugzilla.mozilla.org/show_bug.cgi?id=1587779); added to NSS in [Bugzilla #1728394](https://bugzilla.mozilla.org/show_bug.cgi?id=1728394) (2021).
- **Microsoft Windows** — added in the [March 2020 Trusted Root Program deployment](https://learn.microsoft.com/en-us/security/trusted-root/2020/march2020).
- **Google Chrome** (via Chrome Root Store) — listed as Included per [Cloudflare Radar CT data](https://radar.cloudflare.com/certificate-transparency/ca/2E44102AB58CB85419451C8E19D9ACF3662CAFBC614B6A53960A30F7D0E2EB41).
- **Apple** — **not included** as of the time of writing; no public explanation has been issued by Apple.

The issuing CA (TunTrust Services CA) is technically constrained in the CCADB to `.tn` domains owned by entities under Tunisian jurisdiction; Extended Validation (EV) trust is not asserted. The root itself does not encode name constraints, but Mozilla applied them at the root level via policy. The CA is a member of the [Cloud Signature Consortium](https://cloudsignatureconsortium.org/member/tuntrust/).

## CA/Browser Forum compliance

- **Audit framework:** WebTrust for CAs and WebTrust for Baseline Requirements, audited annually by **Deloitte** (Canada). The most recent completed audit period on record in CCADB covers 2024-10-01 – 2025-09-30 (statement date 2025-12-16). A point-in-time audit and Key Generation Ceremony (KGC) audit were performed at root creation (26 April 2019). [[CCADB Case 00000499](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000499)]
- **Baseline Requirements:** The CP/CPS explicitly states that CA/Browser Forum Baseline Requirements take precedence over the CP/CPS in case of conflict. [[TunTrust CP/CPS](https://www.tuntrust.tn/sites/default/files/Ressources/CPCPS-TunTrustPKI-Markdown.html)]
- **CCADB disclosure:** ANCE maintains CCADB records, including audit reports and CA compliance self-assessment spreadsheets (most recent: December 2024 and December 2025). [[CCADB Case 00000499](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000499)]
- **Pre-issuance linting:** TunTrust uses zlint, cablint, and certlint before certificate issuance, plus pre-configured EJBCA validators for CAA checking, blocklist checking, and weak-key checking. [[Mozilla dev-security-policy public discussion](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/dTTp4ZfUW34)]
- **Certificate Transparency:** Issued certificates are logged to public CT logs; the CA's CT issuance activity is visible via [Cloudflare Radar](https://radar.cloudflare.com/certificate-transparency/ca/2E44102AB58CB85419451C8E19D9ACF3662CAFBC614B6A53960A30F7D0E2EB41).

## Past non-compliance

**2020 OCSP availability outage (Bugzilla #1663953):** In September 2020, TunTrust self-reported a ~20-hour period during which OCSP was unreachable after a patch-management script accidentally deleted the OCSP virtual machine. TunTrust filed a timely incident report and remediated by engaging a third party for patch management and pursuing ISO 22301 (business continuity) certification. This incident arose during the Mozilla root-inclusion review and was reviewed as part of the public discussion. [[Mozilla dev-security-policy](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/dTTp4ZfUW34)]

No certificate misissuance was identified during the Mozilla inclusion review; the inclusion discussion found the issued certificates to be properly formed. The community did raise design-risk concerns regarding a government-operated, OV-only, name-constrained CA, which led Mozilla to document its "Quantifying Value" policy for new CA applicants. These concerns are on record but did not result in a distrust action. [[Mozilla dev-security-policy](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/dTTp4ZfUW34)]

No subsequent Mozilla CA incident bugs specific to TunTrust have been identified as of this writing. A search of open Mozilla CA Bugzilla issues can be performed at: [Bugzilla CA incident search](https://bugzilla.mozilla.org/buglist.cgi?product=NSS&component=CA%20Certificate%20Compliance&resolution=---).

## Transparency

- **CP/CPS:** Publicly available at the TunTrust repository ([https://www.tuntrust.tn/en/node/334](https://www.tuntrust.tn/en/node/334)). Current version (v06.1) is published at [https://www.tuntrust.tn/sites/default/files/Ressources/CPCPS-TunTrustPKI-Markdown.html](https://www.tuntrust.tn/sites/default/files/Ressources/CPCPS-TunTrustPKI-Markdown.html). Multiple prior versions (v5.6, v5.7, v5.9) remain accessible.
- **CCADB disclosure:** All intermediate CAs and audit reports are disclosed in CCADB; the root inclusion case (Case 00000499) is publicly readable. [[CCADB](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000499)]
- **Incident self-reporting:** TunTrust self-reported the 2020 OCSP outage to Mozilla Bugzilla (#1663953) and provided a root-cause analysis and remediation plan during the public discussion period. [[Mozilla dev-security-policy](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/dTTp4ZfUW34)]
- **Certificate Transparency:** Issued certificates are submitted to public CT logs. Issuance history is observable via [Cloudflare Radar](https://radar.cloudflare.com/certificate-transparency/ca/2E44102AB58CB85419451C8E19D9ACF3662CAFBC614B6A53960A30F7D0E2EB41) and [crt.sh](https://crt.sh/?caid=&opt=&q=tuntrust.tn).

## Sources

- [TunTrust official website (tuntrust.tn)](http://www.tuntrust.tn/en)
- [Mozilla Bugzilla #1587779 — Add TunTrust Root CA root certificate](https://bugzilla.mozilla.org/show_bug.cgi?id=1587779)
- [Mozilla Bugzilla #1728394 — Add TunTrust Root CA certificate to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=1728394)
- [Mozilla Bugzilla #1233645 — Add TunRootCA2 root certificate(s) (predecessor request)](https://bugzilla.mozilla.org/show_bug.cgi?id=1233645)
- [Mozilla dev-security-policy — Public Discussion re: Inclusion of the TunTrust Root CA](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/dTTp4ZfUW34)
- [CCADB Case 00000499 — TunTrust inclusion case](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000499)
- [Microsoft Trusted Root Program — March 2020 Deployment Notice](https://learn.microsoft.com/en-us/security/trusted-root/2020/march2020)
- [Cloudflare Radar — Certificate Transparency for TunTrust Root CA](https://radar.cloudflare.com/certificate-transparency/ca/2E44102AB58CB85419451C8E19D9ACF3662CAFBC614B6A53960A30F7D0E2EB41)
- [TunTrust CP/CPS v06.1](https://www.tuntrust.tn/sites/default/files/Ressources/CPCPS-TunTrustPKI-Markdown.html)
- [TunTrust audit reports page](https://www.tuntrust.tn/en/node/334)
- [Cloud Signature Consortium — TunTrust member profile](https://cloudsignatureconsortium.org/member/tuntrust/)
