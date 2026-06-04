# FNMT-RCM

Fábrica Nacional de Moneda y Timbre – Real Casa de la Moneda (FNMT-RCM) is Spain's national mint and a government-owned public corporation attached to the Ministry of Economy, headquartered in Madrid. Through its CERES (Spanish Certification) project, FNMT-RCM has operated as a publicly-trusted certification authority since the 1990s, issuing TLS server certificates, qualified electronic signature/seal certificates, and citizen identity certificates. All roots in this folder belong to the same legal entity; no external brand mergers are involved.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| AC RAIZ FNMT-RCM | `cd8c0d63.0` | RSA 4096 | 2030-01-01 | `EB:C5:57:0C:29:01:8C:4D:67:B1:AA:12:7B:AF:12:F7:03:B4:61:1E:BC:17:B7:DA:B5:57:38:94:17:9B:93:FA` |
| AC RAIZ FNMT-RCM SERVIDORES SEGUROS | `b81b93f0.0` | ECC P-384 | 2043-12-20 | `55:41:53:B1:3D:2C:F9:DD:B7:53:BF:BE:1A:4E:0A:E0:8D:0A:A4:18:70:58:FE:60:A2:B8:62:B2:E4:B8:7B:CB` |

## Rationale for inclusion

Both roots are trusted for the Websites (server authentication / TLS) trust bit across the major root programs:

- **Mozilla NSS:** AC RAIZ FNMT-RCM was added in NSS 3.28.1 / Firefox 51 (Mozilla bug [#1299951](https://bugzilla.mozilla.org/show_bug.cgi?id=1299951)); AC RAIZ FNMT-RCM SERVIDORES SEGUROS was approved in bug [#1559342](https://bugzilla.mozilla.org/show_bug.cgi?id=1559342) and added in bug [#1683738](https://bugzilla.mozilla.org/show_bug.cgi?id=1683738), with EV enabled (OID 2.23.140.1.1).
- **Microsoft Trusted Root Program:** AC RAIZ FNMT-RCM (B8651) was tracked in Microsoft's August 2021 deployment notice; a newer generation root (AC RAIZ FNMT-RCM G2) appeared in the June 2025 notice. ([Microsoft Learn – August 2021](https://learn.microsoft.com/en-us/security/trusted-root/2021/august2021), [June 2025](https://learn.microsoft.com/en-us/security/trusted-root/2025/june-2025))
- **Apple root store:** Both roots appear in Apple's list of trusted certificates for iOS/macOS. ([Apple Support – iOS 17 / macOS 14](https://support.apple.com/en-mt/105116))
- **Google Chrome Root Program:** Inclusion is subject to Chrome Root Program policy; FNMT-RCM roots are present in the Chrome Root Store.

FNMT-RCM is also a Qualified Trust Service Provider (QTSP) under EU Regulation 910/2014 (eIDAS), supervised by Spain's national supervisory body, and is listed on the EU Trusted Lists. ([FNMT Quality page](https://www.cert.fnmt.es/en/que-es-ceres/calidad))

## CA/Browser Forum compliance

FNMT-RCM's TLS hierarchies are audited annually under **ETSI EN 319 411-1** and **ETSI EN 319 411-2** (and previously also under WebTrust for CAs and WebTrust Baseline Requirements during the original 2016 inclusion). The current auditor for AC RAIZ FNMT-RCM is **AENOR INTERNACIONAL, S.A. (Unipersonal)**; the current auditor for AC RAIZ FNMT-RCM SERVIDORES SEGUROS is **AENOR CONFIA, S.A. (Unipersonal)**. The most recent SERVIDORES SEGUROS audit covers the period 2025-01-13 to 2026-01-12, with an audit statement date of 2026-04-01, and reports no deviations. ([CCADB Case #00000418](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000418))

The CA commits to the CA/Browser Forum Baseline Requirements for TLS Server Certificates and EV Guidelines. Domain validation uses CA/B Forum-permitted methods (including DNS change and constructed e-mail to domain contact). CAA record checking has been implemented since at least May 2017. Certificate profiles are validated against crt.sh linters (cablint, x509lint) post-issuance. FNMT-RCM discloses all intermediates and audit results in CCADB.

## Past non-compliance

The following publicly-documented compliance issues have been reported and remediated through Mozilla's Bugzilla process. No distrust action has been taken against FNMT-RCM by any root program.

1. **2019 audit findings – domain validation, CAA, document age, CRL ([Bug 1544586](https://bugzilla.mozilla.org/show_bug.cgi?id=1544586)):** The 2019 audit found that deprecated domain-validation method 3.2.2.4.1 was in use (not listed in the CPS), verification evidence older than 825 days was accepted in some cases, CAA checking was not uniformly applied, suspended certificates were incorrectly included in CRLs (5 non-TLS certs), and the Validation Specialist role lacked formal definition. FNMT updated its CPS and practices, revoked the affected certificates, and the bug was resolved fixed.

2. **2021 mis-issuance – missing CA/B Forum policy OIDs ([Bug 1696872](https://bugzilla.mozilla.org/show_bug.cgi?id=1696872)):** FNMT discovered that 240 EV certificates (from AC Administración Pública, issued 2020-01-10 to 2021-03-03) and 248 OV certificates (from AC Componentes Informáticos, issued 2020-09-30 to 2021-03-01) were missing the required CA/B Forum reserved policy OIDs (2.23.140.1.1 and 2.23.140.1.2.2 respectively). Root cause: reduced compliance-team activity from August 2020 due to personal circumstances, combined with pre- and post-issuance linting (cablint, x509lint) failing to detect the omission. Issuance was suspended on 2021-03-08, certificates were revoked by 2021-03-13, and a peer-review compliance protocol was introduced. Bug resolved fixed.

3. **2025 service disruption – CRL/OCSP and CP/CPS availability ([Bug 1963778](https://bugzilla.mozilla.org/show_bug.cgi?id=1963778)):** The April 28, 2025 nationwide electricity blackout in Spain caused FNMT's backup-datacenter failover to fail (a provider communications misconfiguration), resulting in approximately 30 hours of disruption to OCSP, CRL distribution, revocation-request acceptance, and CP/CPS publication — violating BR sections 4.10.2 and 4.9.3. No certificates were misissued and no revocation delays occurred. FNMT self-reported, remediated (increased generator autonomy, cloud-based DNS, remote datacenter management, updated BCP), and the bug was resolved fixed.

4. **2025 test-website publication error ([Bug 1947207](https://bugzilla.mozilla.org/show_bug.cgi?id=1947207)):** An expired certificate was shown as the "Test Website – Valid" entry in CCADB due to a load-balancer node not being updated. Root cause was a bug in the TLS termination cluster; remediated.

## Transparency

- **CP/CPS:** Publicly available in English at [sede.fnmt.gob.es](https://www.sede.fnmt.gob.es/en/normativa/declaracion-de-practicas-de-certificacion). The General Declaration of Trust Services Practices (DGPC) applies to all hierarchies; a dedicated CPS for SERVIDORES SEGUROS consolidates all BR compliance commitments in one document. ([CPS – SERVIDORES SEGUROS](https://www.sede.fnmt.gob.es/documents/10445900/10536309/dpc_ss_english.pdf), [General CPS](https://www.sede.fnmt.gob.es/documents/10445900/10536309/dgpc_english.pdf))
- **CCADB:** Both roots and their intermediate CAs are disclosed in the Common CA Database. FNMT-RCM maintains CCADB self-assessment documents and links audit statements. ([CCADB Case #00000418](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000418))
- **Incident self-reporting:** All known incidents listed above were either self-reported or promptly responded to via Mozilla Bugzilla; FNMT has not been cited for failure to self-disclose.
- **Certificate Transparency:** As a publicly-trusted CA, FNMT-RCM must embed SCTs in all issued TLS certificates per Chrome (since 2018) and Firefox (since February 2025) requirements. FNMT-RCM does not operate its own CT log; certificates are logged to public CT logs operated by third parties. No specific CT log policy beyond BR/browser requirements has been publicly documented by FNMT-RCM.

## Sources

- [Mozilla Bugzilla #435736 – Add Spanish FNMT root certificate](https://bugzilla.mozilla.org/show_bug.cgi?id=435736)
- [Mozilla Bugzilla #1299951 – Add AC RAIZ FNMT-RCM root to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=1299951)
- [Mozilla Bugzilla #1559342 – Add AC RAIZ FNMT-RCM SERVIDORES SEGUROS root](https://bugzilla.mozilla.org/show_bug.cgi?id=1559342)
- [Mozilla Bugzilla #1683738 – Add AC RAIZ FNMT-RCM SERVIDORES SEGUROS to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=1683738)
- [Mozilla Bugzilla #1544586 – FNMT: 2019 audit findings](https://bugzilla.mozilla.org/show_bug.cgi?id=1544586)
- [Mozilla Bugzilla #1696872 – FNMT: mis-issuance of TLS certs without CA/B Forum policy OID](https://bugzilla.mozilla.org/show_bug.cgi?id=1696872)
- [Mozilla Bugzilla #1947207 – FNMT: incorrect test website publication](https://bugzilla.mozilla.org/show_bug.cgi?id=1947207)
- [Mozilla Bugzilla #1963778 – FNMT: CRL/OCSP and CP/CPS disruption (2025)](https://bugzilla.mozilla.org/show_bug.cgi?id=1963778)
- [CCADB Case #00000418 – AC RAIZ FNMT-RCM SERVIDORES SEGUROS](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000418)
- [mozilla.dev.security.policy – FNMT Root Inclusion Request](https://groups.google.com/g/mozilla.dev.security.policy/c/7wIZmwp4qGQ)
- [mozilla.dev.security.policy – FNMT Public Discussion (SERVIDORES SEGUROS)](https://groups.google.com/g/mozilla.dev.security.policy/c/T5bYrPznCXQ)
- [FNMT-RCM – Digital Certification (CERES)](https://www.fnmt.es/en/ceres)
- [FNMT-RCM – Institutional Information](https://www.fnmt.es/en/institucion/informacion-institucional)
- [FNMT-RCM – Quality & Certifications](https://www.cert.fnmt.es/en/que-es-ceres/calidad)
- [FNMT-RCM – General CPS (English)](https://www.sede.fnmt.gob.es/documents/10445900/10536309/dgpc_english.pdf)
- [FNMT-RCM – CPS for SERVIDORES SEGUROS (English)](https://www.sede.fnmt.gob.es/documents/10445900/10536309/dpc_ss_english.pdf)
- [FNMT-RCM – CPS for Componentes Informáticos (English)](https://www.sede.fnmt.gob.es/documents/10445900/10536309/dpc_componentes_english.pdf)
- [FNMT-RCM – Audit Attestation Letter 2019 (AENOR)](https://www.cert.fnmt.es/documents/10446703/10724392/AuditAttestationLetter_FNMT_RCM.pdf)
- [Microsoft Trusted Root – August 2021 Deployment Notice](https://learn.microsoft.com/en-us/security/trusted-root/2021/august2021)
- [Microsoft Trusted Root – June 2025 Deployment Notice](https://learn.microsoft.com/en-us/security/trusted-root/2025/june-2025)
- [Apple Support – Trusted root certificates in iOS 17 / macOS 14](https://support.apple.com/en-mt/105116)
- [Wikipedia – Royal Mint (Spain)](https://en.wikipedia.org/wiki/Royal_Mint_(Spain))
