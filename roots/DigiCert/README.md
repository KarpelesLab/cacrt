# DigiCert

DigiCert, Inc. is a commercial certificate authority headquartered in Lehi, Utah, USA, founded in 2003 and a founding member of the CA/Browser Forum in 2005. It is one of the largest publicly trusted CA operators in the Web PKI, issuing TLS/SSL, code signing, S/MIME, and document-signing certificates under multiple brands. This folder merges roots operated by DigiCert, Inc. and QuoVadis Limited (acquired by DigiCert in January 2019 from WISeKey); note that the legacy Symantec PKI roots acquired in October 2017 were distrusted by browsers and are not represented here.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| DigiCert Assured ID Root G2 | `9d04f354.0` | RSA 2048 | 2038-01-15 | `7D:05:EB:B6:82:33:9F:8C:94:51:EE:09:4E:EB:FE:FA:79:53:A1:14:ED:B2:F4:49:49:45:2F:AB:7D:2F:C1:85` |
| DigiCert Assured ID Root G3 | `7f3d5d1d.0` | ECC P-384 | 2038-01-15 | `7E:37:CB:8B:4C:47:09:0C:AB:36:55:1B:A6:F4:5D:B8:40:68:0F:BA:16:6A:95:2D:B1:00:71:7F:43:05:3F:C2` |
| DigiCert Global Root G2 | `607986c7.0` | RSA 2048 | 2038-01-15 | `CB:3C:CB:B7:60:31:E5:E0:13:8F:8D:D3:9A:23:F9:DE:47:FF:C3:5E:43:C1:14:4C:EA:27:D4:6A:5A:B1:CB:5F` |
| DigiCert Global Root G3 | `dd8e9d41.0` | ECC P-384 | 2038-01-15 | `31:AD:66:48:F8:10:41:38:C7:38:F3:9E:A4:32:01:33:39:3E:3A:18:CC:02:29:6E:F9:7C:2A:C9:EF:67:31:D0` |
| DigiCert TLS ECC P384 Root G5 | `9846683b.0` | ECC P-384 | 2046-01-14 | `01:8E:13:F0:77:25:32:CF:80:9B:D1:B1:72:81:86:72:83:FC:48:C6:E1:3B:E9:C6:98:12:85:4A:49:0C:1B:05` |
| DigiCert TLS RSA4096 Root G5 | `d52c538d.0` | RSA 4096 | 2046-01-14 | `37:1A:00:DC:05:33:B3:72:1A:7E:EB:40:E8:41:9E:70:79:9D:2B:0A:0F:2C:1D:80:69:31:65:F7:CE:C4:AD:75` |
| DigiCert Trusted Root G4 | `75d1b2ed.0` | RSA 4096 | 2038-01-15 | `55:2F:7B:DC:F1:A7:AF:9E:6C:E6:72:01:7F:4F:12:AB:F7:72:40:C7:8E:76:1A:C2:03:D1:D9:D2:0A:C8:99:88` |
| QuoVadis Root CA 1 G3 | `749e9e03.0` | RSA 4096 | 2042-01-12 | `8A:86:6F:D1:B2:76:B5:7E:57:8E:92:1C:65:82:8A:2B:ED:58:E9:F2:F2:88:05:41:34:B7:F1:F4:BF:C9:CC:74` |
| QuoVadis Root CA 2 G3 | `064e0aa9.0` | RSA 4096 | 2042-01-12 | `8F:E4:FB:0A:F9:3A:4D:0D:67:DB:0B:EB:B2:3E:37:C7:1B:F3:25:DC:BC:DD:24:0E:A0:4D:AF:58:B4:7E:18:40` |
| QuoVadis Root CA 3 G3 | `e18bfb83.0` | RSA 4096 | 2042-01-12 | `88:EF:81:DE:20:2E:B0:18:45:2E:43:F8:64:72:5C:EA:5F:BD:1F:C2:D9:D2:05:73:07:09:C5:D8:B8:69:0F:46` |

## Rationale for inclusion

DigiCert root certificates are included in all major root programs: Mozilla NSS/Firefox, Google Chrome Root Store, Apple Root Certificate Program, and Microsoft Trusted Root Program. The G5 TLS roots (DigiCert TLS ECC P384 Root G5 and DigiCert TLS RSA4096 Root G5) were approved for Mozilla inclusion with EV enabled in NSS 3.80 / Firefox 104 following a public discussion that concluded 2022-03-31 ([Mozilla Bugzilla #1706228](https://bugzilla.mozilla.org/show_bug.cgi?id=1706228); [dev-security-policy discussion](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/z3-sYPyykqc)). Earlier roots (Assured ID G2/G3, Global Root G2/G3, Trusted Root G4) were included in prior inclusion requests going back to the original DigiCert roots in 2007 ([Mozilla Bugzilla #364568](https://bugzilla.mozilla.org/show_bug.cgi?id=364568)). QuoVadis roots are disclosed in the CCADB and included across root programs. All roots in this folder cover the TLS server authentication use case and are scoped to the Web PKI.

## CA/Browser Forum compliance

DigiCert undergoes annual WebTrust audits covering: WebTrust for Certification Authorities, WebTrust for Baseline Requirements, and WebTrust for Network and Certificate Systems Security. Audit letters are published at [digicert.com/webtrust-audits](https://www.digicert.com/webtrust-audits). The current consolidated CP/CPS (v7.x series, published 2024 onward) follows RFC 3647 and is publicly available at [digicert.com/legal-repository](https://www.digicert.com/legal-repository). DigiCert discloses all publicly trusted root and intermediate certificates in the CCADB ([ccadb.org](https://www.ccadb.org/)). DigiCert was an early adopter of Certificate Transparency, becoming the first CA to embed CT proofs in certificates in October 2013; all newly issued public TLS certificates are submitted to CT logs by default since February 2018. DigiCert supports the ACME protocol for automated certificate lifecycle management across its CertCentral platform.

## Past non-compliance

**2024 — DCV CNAME underscore prefix mass revocation ([Mozilla Bugzilla #1910322](https://bugzilla.mozilla.org/show_bug.cgi?id=1910322)):** DigiCert reported on July 28, 2024 that a flaw in its CNAME-based Domain Control Verification (DCV) process had caused it to omit the required underscore prefix in random validation values since August 2019, resulting in 83,267 mis-issued TLS certificates across 6,807 customers (~0.4% of applicable validations). CA/B Forum Baseline Requirements require revocation of non-compliant certificates within 24 hours. DigiCert issued revocation notices with less than 24 hours lead time; a customer (Alegeus) obtained a temporary court restraining order, and DigiCert engaged root programs to manage critical infrastructure exceptions, completing full revocation by August 3, 2024. A related delayed-revocation issue was tracked in [Bugzilla #1910805](https://bugzilla.mozilla.org/show_bug.cgi?id=1910805). An unrelated cease-and-desist letter from DigiCert regarding Bugzilla discourse was documented in [Bugzilla #1950144](https://bugzilla.mozilla.org/show_bug.cgi?id=1950144).

**2017–2018 — Symantec PKI distrust (historical context):** DigiCert acquired Symantec's TLS/SSL and PKI business in October 2017 after Google and Mozilla announced plans to distrust the legacy Symantec PKI roots due to years of compliance failures at Symantec (beginning 2015). The distrust affected Symantec-rooted certificates, not DigiCert's own roots. DigiCert issued replacement certificates for over 5 million affected customers under Symantec, GeoTrust, RapidSSL, Thawte, and VeriSign brands; final Chrome 70 distrust of legacy Symantec roots took effect October 2018. DigiCert's own root hierarchy was unaffected by this distrust action. ([DigiCert press release, January 2019](https://www.prnewswire.com/news-releases/digicert-works-with-its-customers-and-partners-to-successfully-move-past-googles-distrust-of-symantec-tls-certificates-300745385.html); [Wikipedia: DigiCert](https://en.wikipedia.org/wiki/DigiCert))

**2026 — Misissued code signing certificates ([Mozilla Bugzilla #2033170](https://bugzilla.mozilla.org/show_bug.cgi?id=2033170)):** DigiCert disclosed in April 2026 that a threat actor compromised its internal support portal via social engineering, obtaining 60 EV code signing certificates fraudulently. At least 11 certificates were used to sign malware (Zhong Stealer). DigiCert revoked all affected certificates within 24 hours of discovery. This incident involves code signing trust, not TLS server authentication roots; the TLS roots in this folder were not implicated.

No distrust of DigiCert's TLS roots by any major root program has been publicly documented as of the date of this writing.

## Transparency

DigiCert's CP/CPS documents are published at [digicert.com/legal-repository](https://www.digicert.com/legal-repository) and updated at minimum annually; version history and changelogs are maintained. The QuoVadis Europe legal repository is at [digicert.com/legal-repository/europe](https://www.digicert.com/legal-repository/europe). Annual WebTrust audit letters are posted at [digicert.com/webtrust-audits](https://www.digicert.com/webtrust-audits). All publicly trusted root and intermediate certificates are disclosed in the CCADB. DigiCert operates its own CT log and submits all public TLS issuances to external CT logs; SCTs are embedded in certificates by default. DigiCert's CCADB entries can be searched via the [Mozilla Included CA Certificate Report](https://ccadb.my.salesforce-sites.com/mozilla/CACertificatesInFirefoxReport). Compliance incidents are self-reported to Mozilla Bugzilla under the CA Certificate Compliance component.

## Sources

- [DigiCert — Wikipedia](https://en.wikipedia.org/wiki/DigiCert)
- [DigiCert Legal Repository (CP/CPS)](https://www.digicert.com/legal-repository)
- [DigiCert WebTrust Audits](https://www.digicert.com/webtrust-audits)
- [Mozilla Bugzilla #364568 — Add DigiCert CA Root Certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=364568)
- [Mozilla Bugzilla #1706228 — Add DigiCert root Certificates (G5)](https://bugzilla.mozilla.org/show_bug.cgi?id=1706228)
- [Mozilla dev-security-policy: DigiCert G5 inclusion public discussion](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/z3-sYPyykqc)
- [Mozilla Bugzilla #1910322 — DigiCert: Random value in CNAME without underscore prefix](https://bugzilla.mozilla.org/show_bug.cgi?id=1910322)
- [Mozilla Bugzilla #1910805 — DigiCert: Delayed revocation of 1910322](https://bugzilla.mozilla.org/show_bug.cgi?id=1910805)
- [Mozilla Bugzilla #1950144 — DigiCert: Threat of legal action to stifle Bugzilla discourse](https://bugzilla.mozilla.org/show_bug.cgi?id=1950144)
- [Mozilla Bugzilla #2033170 — DigiCert: Misissued code signing certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=2033170)
- [BleepingComputer: DigiCert mass-revoking TLS certificates due to domain validation bug](https://www.bleepingcomputer.com/news/security/digicert-mass-revoking-tls-certificates-due-to-domain-validation-bug/)
- [Censys: The DigiCert DCV Bug: Implications and Industry Impact](https://censys.com/blog/the-digicert-dcv-bug-implications-and-industry-impact/)
- [The Hacker News: DigiCert to Revoke 83,000+ SSL Certificates Due to Domain Validation Oversight](https://thehackernews.com/2024/07/digicert-to-revoke-83000-ssl.html)
- [DigiCert completes purchase of QuoVadis (PR Newswire, 2019)](https://www.prnewswire.com/news-releases/digicert-completes-purchase-of-quovadis-expands-european-presence-and-tls-pki-offerings-300779847.html)
- [DigiCert acquires Symantec CA — distrust completion (PR Newswire, 2019)](https://www.prnewswire.com/news-releases/digicert-works-with-its-customers-and-partners-to-successfully-move-past-googles-distrust-of-symantec-tls-certificates-300745385.html)
- [CCADB Mozilla Included CA Certificate Report](https://ccadb.my.salesforce-sites.com/mozilla/CACertificatesInFirefoxReport)
- [DigiCert Certificate Transparency FAQ](https://www.digicert.com/faq/certificate-transparency/what-is-certificate-transparency)
- [DigiCert: What Is ACME And Why Is It Important](https://www.digicert.com/blog/what-is-acme-and-why-is-it-important)
- [DigiCert Trusted Root Authority Certificates](https://knowledge.digicert.com/general-information/digicert-trusted-root-authority-certificates)
