# certSIGN

certSIGN (operating as CERTSIGN SA) is a Romanian trust-service provider headquartered in Bucharest, Romania, founded in 2006. It is an eIDAS-qualified trust service provider (QTSP) and operates a public key infrastructure whose Web PKI root — certSIGN ROOT CA G2 — is trusted for TLS server authentication by Mozilla, Microsoft, and Apple. This folder consolidates roots operated under the single legal entity CERTSIGN SA.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| certSIGN ROOT CA G2 | `5f618aec.0` | RSA 4096 | 2042-02-06 | `65:7C:FE:2F:A7:3F:AA:38:46:25:71:F3:32:A2:36:3A:46:FC:E7:02:09:51:71:07:02:CD:FB:B6:EE:DA:33:05` |

## Rationale for inclusion

certSIGN ROOT CA G2 was approved for inclusion in the Mozilla root store (Websites trust bit, EV-enabled) following a public review on mozilla.dev.security.policy in 2020 ([Bug 1403453](https://bugzilla.mozilla.org/show_bug.cgi?id=1403453)). The root was added to NSS 3.54 / Firefox 79, and EV treatment was enabled via [Bug 1645192](https://bugzilla.mozilla.org/show_bug.cgi?id=1645192). The root is also included in the Microsoft Trusted Root Program and Apple root store, as recorded in the CCADB included-CA report. The root is scoped to TLS server authentication (Websites trust bit); EV policy OID 2.23.140.1.1 applies.

certSIGN is a [qualified trust service provider](https://www.certsign.ro/en/about-us/) under eIDAS Regulation 910/2014, supervised by the Romanian National Authority for Management and Regulation in Communications (ANCOM), and its services are listed on the EU Trusted List.

## CA/Browser Forum compliance

certSIGN is audited annually under ETSI EN 319 411-1 and EN 319 411-2 by LSTI (most recent audit statement: 2026-05-08, as recorded in the [CCADB](https://ccadb.my.salesforce-sites.com/mozilla/IncludedCACertificateReport)). It also holds WebTrust for CAs (Standard and BR) audits, confirmed on the [certSIGN certifications page](https://www.certsign.ro/en/about-us/certifications/).

The CPS for certSIGN ROOT CA G2 ([CPS ROOT CA G2 v2.28, Jan 2026](https://www.certsign.ro/en/document/certsign-root-ca-g2-certification-practice-statement/)) and subordinate issuing CAs explicitly commit to the CA/Browser Forum Baseline Requirements, Mozilla Root Store Policy, Apple Root Certificate Program, Microsoft Trusted Root Program, and Chrome Root Program Policy. All issued TLS certificates are logged to public Certificate Transparency logs as required by browser policy. certSIGN discloses its full PKI hierarchy in the CCADB and publishes CP/CPS documents in its [public repository](https://www.certsign.ro/en/repository/).

## Past non-compliance

The following publicly documented incidents are on record in Mozilla Bugzilla. No distrust action has been taken against certSIGN by any major root program.

**Bug [1390979](https://bugzilla.mozilla.org/show_bug.cgi?id=1390979) — Non-BR-Compliant Certificate Issuance (2017):** certSIGN issued 40 certificates containing BR violations in 10 distinct categories (e.g., CNs not matching SANs, missing/invalid locality fields, leading spaces in dNSName). The root cause was that pre-issuance technical controls did not enforce BR compliance, and a general-purpose email inbox caused problem reports to be detected late. All 40 certificates were either revoked or expired by the time of remediation. certSIGN created a dedicated revocation address (revokecsgn@certsign.ro), enforced pre-issuance linting controls, and updated its CPS.

**Bug [1551375](https://bugzilla.mozilla.org/show_bug.cgi?id=1551375) — "Some-State" in stateOrProvinceName (2019):** A single certificate was issued containing OpenSSL's default placeholder value "Some-State" in the stateOrProvinceName field, indicating the field was not validated. The certificate was revoked the same day it was reported. Remediation included blocking default placeholder values, increasing RA training frequency, raising internal audit sampling, and deploying a CSR checker validated against ISO 3166-2. The Mozilla reviewer noted the initial fix addressed the symptom rather than the root cause and requested further analysis.

**Bug [1674886](https://bugzilla.mozilla.org/show_bug.cgi?id=1674886) — OV pre-certificate issued instead of DV (2020):** An RA operator selected the wrong certificate profile (OV instead of DV), and a second operator's cross-check failed to catch the error. Pre-issuance linting (cablint) did not flag the wrong policy OID. One certificate was affected and revoked within 24 hours of external notification. certSIGN integrated zlint and added policy OID pre-issuance checks. Mozilla reviewers also noted that certSIGN incorrectly closed the Bugzilla bug twice before a Mozilla representative reviewed it.

**Bug [1762707](https://bugzilla.mozilla.org/show_bug.cgi?id=1762707) — Subscriber precertificate without Certificate Policies extension (2022):** A software update introduced a linting endpoint misconfiguration combined with faulty exception handling lacking a catch-all condition, causing pre-issuance linting to fail silently and be skipped. One pre-certificate was issued without the required Certificate Policies extension and revoked within 24 hours of issuance. The fix added a catch-all exception handler and changed linting so that warnings are treated as errors.

**Bug [1965808](https://bugzilla.mozilla.org/show_bug.cgi?id=1965808) — 2025 ETSI Audit Finding: conflicting information in DV CPS (2025):** ETSI auditors identified conflicting information in the certSIGN Web CA DV CPS (chapter 3.2.2.1, ETSI EN 319 411-1 REG-6.2.2-03A). No certificates were affected. certSIGN published a corrected CPS version (v1.9) the following day and improved its internal CPS review process.

A search of [Mozilla Bugzilla for all certSIGN bugs](https://bugzilla.mozilla.org/buglist.cgi?quicksearch=certsign&list_id=17266801) confirms no distrust or removal action has been taken against this CA.

## Transparency

- **CP/CPS:** All current CP and CPS documents are publicly available at the [certSIGN repository](https://www.certsign.ro/en/repository/). The Root CA G2 CPS is [CPS ROOT CA G2 v2.28 (Jan 2026)](https://www.certsign.ro/en/document/certsign-root-ca-g2-certification-practice-statement/). The general CP is [CP Law v1.20 (Jan 2026)](https://www.certsign.ro/en/document/certsign-certification-policy/).
- **CCADB disclosure:** certSIGN discloses audit reports, CP/CPS URLs, and PKI hierarchy information in the CCADB; the CCADB entry for this root shows audit statement date 2026-05-08 by LSTI under ETSI EN 319 411.
- **Incident self-reporting:** The incidents described above were disclosed in Mozilla Bugzilla. certSIGN provides incident reports with root-cause analysis and remediation timelines.
- **Certificate Transparency:** All TLS certificates issued under this root are logged to public CT logs as required by Mozilla Root Store Policy and Chrome's CT policy; certificates are visible via [crt.sh](https://crt.sh/?caid=&o=CERTSIGN).

## Sources

- [certSIGN official website](https://www.certsign.ro/en/)
- [certSIGN About Us / Our Story](https://www.certsign.ro/en/about-us/our-story/)
- [certSIGN Certifications page](https://www.certsign.ro/en/about-us/certifications/)
- [certSIGN public repository (CP/CPS)](https://www.certsign.ro/en/repository/)
- [CPS ROOT CA G2 v2.28 (Jan 2026)](https://www.certsign.ro/en/document/certsign-root-ca-g2-certification-practice-statement/)
- [CP Law v1.20 (Jan 2026)](https://www.certsign.ro/en/document/certsign-certification-policy/)
- [Mozilla Bugzilla #1403453 — Add certSIGN Root CA G2 certificate](https://bugzilla.mozilla.org/show_bug.cgi?id=1403453)
- [Mozilla Bugzilla #1645186 — Add certSIGN Root CA G2 to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=1645186)
- [Mozilla Bugzilla #1645192 — Enable EV treatment for certSIGN Root CA G2](https://bugzilla.mozilla.org/show_bug.cgi?id=1645192)
- [Mozilla Bugzilla #470756 — Original certSIGN ROOT CA inclusion (2009)](https://bugzilla.mozilla.org/show_bug.cgi?id=470756)
- [Mozilla Bugzilla #1390979 — certSIGN: Non-BR-Compliant Certificate Issuance](https://bugzilla.mozilla.org/show_bug.cgi?id=1390979)
- [Mozilla Bugzilla #1551375 — certSIGN: "Some-State" in stateOrProvinceName](https://bugzilla.mozilla.org/show_bug.cgi?id=1551375)
- [Mozilla Bugzilla #1674886 — certSIGN: OV/DV mis-issuance](https://bugzilla.mozilla.org/show_bug.cgi?id=1674886)
- [Mozilla Bugzilla #1762707 — certSIGN: Subscriber precertificate without Certificate Policies](https://bugzilla.mozilla.org/show_bug.cgi?id=1762707)
- [Mozilla Bugzilla #1965808 — certSIGN: 2025 ETSI Audit Finding #5 – Conflicting info in CPS](https://bugzilla.mozilla.org/show_bug.cgi?id=1965808)
- [CCADB Included CA Certificate Report](https://ccadb.my.salesforce-sites.com/mozilla/IncludedCACertificateReport)
- [mozilla.dev.security.policy — Request to Include certSIGN Root CA G2](https://groups.google.com/g/mozilla.dev.security.policy/c/ynPfdYtz0Ag/m/Nfbq64TyBwAJ)
