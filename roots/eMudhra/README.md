# eMudhra

eMudhra is a digital identity and PKI services company headquartered in Bangalore, India, incorporated in 2008 and publicly listed on Indian exchanges since 2022. It operates two legal entities in the Web PKI: **eMudhra Technologies Limited** (India) and **eMudhra Inc** (USA), both issuing publicly trusted TLS certificates under the **emSign** brand. eMudhra is the first CA based in India to achieve inclusion in the major browser root programs globally.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| emSign Root CA - G1 | `2923b3f9.0` | RSA 2048 | 2043-02-18 | `40:F6:AF:03:46:A9:9A:A1:CD:1D:55:5A:4E:9C:CE:62:C7:F9:63:46:03:EE:40:66:15:83:3D:C8:C8:D0:03:67` |
| emSign ECC Root CA - G3 | `14bc7599.0` | ECC P-384 | 2043-02-18 | `86:A1:EC:BA:08:9C:4A:8D:3B:BE:27:34:C6:12:BA:34:1D:81:3E:04:3C:F9:E8:A8:62:CD:5C:57:A3:6B:BE:6B` |
| emSign Root CA - C1 | `406c9bb1.0` | RSA 2048 | 2043-02-18 | `12:56:09:AA:30:1D:A0:A2:49:B9:7A:82:39:CB:6A:34:21:6F:44:DC:AC:9F:39:54:B1:42:92:F2:E8:C8:60:8F` |
| emSign ECC Root CA - C3 | `4b718d9b.0` | ECC P-384 | 2043-02-18 | `BC:4D:80:9B:15:18:9D:78:DB:3E:1D:8C:F4:F9:72:6A:79:5D:A1:64:3C:A5:F1:35:8E:1D:DB:0E:DC:0D:7E:B3` |

G1 and G3 are operated by eMudhra Technologies Limited (C=IN); C1 and C3 are operated by eMudhra Inc (C=US). Two additional roots (G2, C2) exist for code signing and time stamping only and are not included here as they are not used for TLS server authentication.

## Rationale for inclusion

All four roots are included in the **Mozilla (NSS)** root store with Email and Websites (TLS server authentication) trust bits, shipped in NSS 3.43 / Firefox 67 (2019). Extended Validation treatment (EV Policy OID `2.23.140.1.1`) was subsequently enabled for all four roots in Firefox 68 via [Bug 1515465](https://bugzilla.mozilla.org/show_bug.cgi?id=1515465). The roots are also included in the **Microsoft Trusted Root Program** (which was the earliest inclusion, predating Mozilla), the **Apple** root store (macOS, iOS, iPadOS, Safari), and the **Android** trust store. [[Mozilla NSS inclusion bug]](https://bugzilla.mozilla.org/show_bug.cgi?id=1515457) [[Mozilla dev-security-policy discussion]](https://groups.google.com/g/mozilla.dev.security.policy/c/dmARDUq_rPw)

All four roots are scoped to TLS server authentication (and S/MIME). Subordinate CA issuance under these roots is restricted to entities controlled by eMudhra; the SSL/TLS CP/CPS explicitly prohibits third-party-operated issuing CAs for publicly trusted TLS certificates. [[SSL/TLS CP/CPS]](https://repository.emsign.com/cps/ssl/SSL-TLS-CP-CPS-v1.02.pdf)

## CA/Browser Forum compliance

eMudhra undergoes annual **WebTrust for CAs** audits (including WebTrust SSL Baseline with Network Security, WebTrust EV SSL, and WebTrust Code Signing), conducted by BDO. Audit seals and links to CPA Canada seal verification are published in the emSign repository. The annual audit period runs June 1 – May 31. [[emSign repository]](https://repository.emsign.com/)

The CP/CPS is structured around RFC 3647 and explicitly commits to conformance with the CA/Browser Forum Baseline Requirements (BR) and EV Guidelines. The CAA issuer domain is `emsign.com`. The CP/CPS is publicly versioned and archived; the current general CP/CPS is v1.23 and the SSL/TLS-specific CP/CPS is v1.05 as of early 2026. [[CP/CPS v1.22]](https://repository.emsign.com/cps/CP-CPS-v1.22.pdf) [[SSL/TLS CP/CPS v1.02]](https://repository.emsign.com/cps/ssl/SSL-TLS-CP-CPS-v1.02.pdf)

eMudhra's certificates are logged to publicly operated Certificate Transparency logs as required by BR and browser policies. Test URLs for TLS CAs are published in the emSign repository.

## Past non-compliance

No mis-issuance event resulting in distrust by any root program has been identified in public records. Three compliance incidents have been documented in Mozilla Bugzilla:

1. **OCSP Responder Time Inconsistency (Bug 1917459, September 2024):** An IST-to-UTC conversion error in the OCSP responder caused timestamps for certificates revoked between 12:00 and 23:59 IST to differ from CRL timestamps by 12 hours. The revocation *status* itself was correct in all cases; only the displayed timestamp was wrong. eMudhra identified the root cause within one day, deployed a fix within two days, and added enhanced monitoring and maker/checker controls. The bug was resolved fixed. [[Bug 1917459]](https://bugzilla.mozilla.org/show_bug.cgi?id=1917459)

2. **CRL URL Mismatch in CCADB Disclosure (Bug 2007297, December 2025):** An external party reported that CRL Distribution Point URLs embedded in 100 CA certificates did not byte-for-byte match the CRL URLs disclosed in CCADB, as required by CCADB Policy v2.0 §6.2. The differences were formatting-only (e.g., trailing slash, http vs https scheme normalization) and did not affect CRL availability or revocation status. eMudhra corrected CCADB records by 2025-12-25, added a manual verification step by 2025-12-31, and implemented automated CRLDP-to-CCADB validation by 2026-01-10. The bug was resolved fixed. [[Bug 2007297]](https://bugzilla.mozilla.org/show_bug.cgi?id=2007297)

3. **Audit Delay (Bug 1728790, 2021):** WebTrust seal delivery from CPA Canada was delayed past the Mozilla policy deadline of three months after the audit period end (May 31, 2021), with the delay attributed to COVID-19 disruptions (second-wave lockdowns in India). Interim audit reports were attached to the bug; final seals were received approximately one week later than estimated. CCADB records were updated December 1, 2021 and the bug was resolved fixed. A similar first-seal delay had occurred during the original 2018 root inclusion. [[Bug 1728790]](https://bugzilla.mozilla.org/show_bug.cgi?id=1728790)

No other public incident records were found. For a live search of open Bugzilla CA-compliance bugs for eMudhra, see the [Mozilla Bugzilla CA compliance queue](https://bugzilla.mozilla.org/buglist.cgi?product=NSS&component=CA%20Certificate%20Compliance&resolution=---).

## Transparency

- **CP/CPS:** Publicly versioned and archived at [repository.emsign.com](https://repository.emsign.com/), covering general PKI, SSL/TLS, code signing, and InCommon-specific CPS documents.
- **CCADB:** eMudhra discloses root and intermediate CA certificates, audit statements, and CRL/OCSP URLs in CCADB as required by Mozilla and Microsoft root program policies.
- **Audit statements:** WebTrust audit seals with CPA Canada verification links are published in the emSign repository; individual audit reports have been attached to Mozilla Bugzilla bugs during compliance reviews.
- **Certificate Transparency:** Certificates issued under these roots are logged to public CT logs per CA/Browser Forum Baseline Requirements.
- **Incident reporting:** All three known incidents were disclosed via Mozilla Bugzilla with root-cause analysis and remediation timelines.

## Sources

- [eMudhra / emSign root inclusion request — Mozilla Bugzilla Bug 1442337](https://bugzilla.mozilla.org/show_bug.cgi?id=1442337)
- [Add eMudhra Technologies Limited root certificates to NSS — Bug 1515457](https://bugzilla.mozilla.org/show_bug.cgi?id=1515457)
- [Enable EV Treatment for eMudhra Technologies Limited roots — Bug 1515465](https://bugzilla.mozilla.org/show_bug.cgi?id=1515465)
- [Mozilla dev-security-policy: Request to Include emSign roots](https://groups.google.com/g/mozilla.dev.security.policy/c/dmARDUq_rPw)
- [eMudhra emSign PKI Services: OCSP Responder Time Inconsistency — Bug 1917459](https://bugzilla.mozilla.org/show_bug.cgi?id=1917459)
- [eMudhra emSign PKI Services: CRL URL Mismatch Between CCADB Disclosure and Certificates — Bug 2007297](https://bugzilla.mozilla.org/show_bug.cgi?id=2007297)
- [eMudhra: Audit Delay — Bug 1728790](https://bugzilla.mozilla.org/show_bug.cgi?id=1728790)
- [emSign Repository (CP/CPS, audit seals, certificates)](https://repository.emsign.com/)
- [emSign CP/CPS v1.22](https://repository.emsign.com/cps/CP-CPS-v1.22.pdf)
- [emSign SSL/TLS CP/CPS v1.02](https://repository.emsign.com/cps/ssl/SSL-TLS-CP-CPS-v1.02.pdf)
- [eMudhra company profile — Tracxn](https://tracxn.com/d/companies/emudhra/__UQqhb4UAixEXiU_gLgb8nYCFuVFfJ659NC0R8UqaGpc)
