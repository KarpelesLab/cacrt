# Cybertrust Japan

Cybertrust Japan Co., Ltd. is a Tokyo-based trust services company and subsidiary of SB Technology Corp., headquartered at ARK Mori Building 31F, Akasaka, Minato-ku, Tokyo, Japan. Founded in 2000 as Japan's first commercial certificate authority, the company operates public TLS and identity PKI under the **SecureSign** brand, and also absorbed the JCSI (Japan Certification Services, Inc.) root CA infrastructure after JCSI was liquidated in 2015. The roots in this folder are the SecureSign generation issued in 2020 to replace the older SecureSign RootCA11 (2009).

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| SecureSign Root CA12 | `616816f6.0` | RSA 2048 | 2040-04-08 | `3F:03:4B:B5:70:4D:44:B2:D0:85:45:A0:20:57:DE:93:EB:F3:90:5F:CE:72:1A:CB:C7:30:C0:6D:DA:EE:90:4E` |
| SecureSign Root CA14 | `878d9bca.0` | RSA 4096 | 2045-04-08 | `4B:00:9C:10:34:49:4F:9A:B5:6B:BA:3B:A1:D6:27:31:FC:4D:20:D8:95:5A:DC:EC:10:A9:25:60:72:61:E3:38` |
| SecureSign Root CA15 | `6a9bdba3.0` | ECC P-384 | 2045-04-08 | `E7:78:F0:F0:95:FE:84:37:29:CD:1A:00:82:17:9E:53:14:A9:C2:91:44:28:05:E1:FB:1D:8F:B6:B8:88:6C:3A` |

All three roots are self-signed and were generated on 2020-04-08.

## Rationale for inclusion

All three roots were approved for the **Mozilla root store** (websites trust bit, EV-enabled) in July 2024 following a six-week CCADB public discussion (Case 00000585, opened 2020) and a seven-day last-call period; no objections were raised. They shipped in NSS 3.104 / Firefox 131 (2024-08-28), with EV enabled in Firefox 132. CA12 and CA14 are also included in the **Microsoft Trusted Root Program**. CA14 and CA15 are not listed as included in Chrome or Apple root stores as of the time of writing; the Chrome inclusion request focused on CA12 for TLS only. [[CCADB Case 585](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000585)] [[Mozilla Bug 1658793](https://bugzilla.mozilla.org/show_bug.cgi?id=1658793)] [[Mozilla dev-security-policy intent to approve](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/pJHiCL07E-Q)]

The scope of the roots in the Web PKI is TLS server authentication (OV and EV). CA12 issues under the "SureServer" brand (intermediates Cybertrust Japan SureServer CA G7 / EV CA G7). CA14 and CA15 have broader stated EKUs (S/MIME, code signing, timestamping) but Cybertrust committed to single-purpose operation before issuing subscriber certificates under those roots.

## CA/Browser Forum compliance

Cybertrust Japan is audited annually by **KPMG Japan** under the **WebTrust for CAs**, **WebTrust for TLS Baseline Requirements**, and **WebTrust for EV TLS** frameworks (CCADB Case 585, audit period 2024-08-24 – 2025-08-23, statements dated 2025-10-30). A key-generation point-in-time audit was conducted for the April 2020 ceremony. The CP/CPS explicitly states that the CA/B Forum Baseline Requirements take precedence over the CPS in case of conflict and that Cybertrust commits to their latest published version. [[CCADB Case 585](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000585)] [[TLS CP/CPS repository](https://www.cybertrust.ne.jp/ssl/repository_rt/)]

The EV Policy OID published in these roots is 2.23.140.1.1 (CA/B Forum EV guideline). CAA domain names recognised are `cybertrust.co.jp` and `cybertrust.ne.jp`.

## Past non-compliance

Two publicly-documented incidents have been recorded against the Cybertrust Japan CA in Mozilla Bugzilla:

**1. Duplicate serial numbers (Bug 1330924, resolved 2017).** CTJ's Certificate Transparency implementation issued a pre-certificate and then re-issued the final certificate with the same serial number, treating both as "the same certificate." This violated RFC 5280 and BR §7.1. Approximately 24,700 certificate pairs across ~2,000 customers were affected. CTJ patched the system in March–April 2017 to generate genuine pre-certificates. Mozilla accepted the resolution and closed the bug as RESOLVED FIXED. [[Bugzilla 1330924](https://bugzilla.mozilla.org/show_bug.cgi?id=1330924)]

**2. CRL signature algorithm encoding error (Bug 1827490, resolved 2023).** CRL Watch detected that two CTJ CRLs (under ECDSA roots) included an extraneous named-curve parameter in the outer AlgorithmIdentifier, violating RFC 5480 and Mozilla Root Store Policy. Remediated CRLs were published on 2023-04-11 and a procedural check was added. The bug was closed RESOLVED FIXED. [[Bugzilla 1827490](https://bugzilla.mozilla.org/show_bug.cgi?id=1827490)]

No distrust action has been taken against any Cybertrust Japan root in Mozilla or other major root programs. A search of the Mozilla CA Incident Dashboard and CCADB shows no additional open or unresolved incidents as of the writing of this file. [[Mozilla CA Incident Dashboard](https://wiki.mozilla.org/CA/Incident_Dashboard)]

## Transparency

**CP/CPS:** The combined TLS Certificate Policy and Certification Practice Statement is published in English at `https://www.cybertrust.ne.jp/ssl/repository_rt/TLS_CTJCPCPS_English.pdf` (and in markdown at `TLS_CTJCPCPS_English.md`), with the full repository at `https://www.cybertrust.ne.jp/ssl/repository_rt/`. Versions reviewed by Mozilla during inclusion were CP 1.15 / CPS 1.04; the current published version is 2.09 (May 2026). [[CP/CPS repository](https://www.cybertrust.ne.jp/ssl/repository_rt/)]

**CCADB disclosure:** Cybertrust Japan maintains current audit statements, intermediate CA records, and CP/CPS URLs in CCADB. [[CCADB Case 585](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000585)]

**Incident reporting:** The problem-reporting address is `evc-report@cybertrust.ne.jp`, published in CCADB.

**Certificate Transparency:** CT logging is required and implemented; Cloudflare Radar shows active logging for CTJ-issued certificates. Cybertrust has operated a CT log (SureTime CA G3) and has participated in CT since 2015.

## Sources

- [Cybertrust Japan corporate website](https://www.cybertrust.co.jp/english/)
- [CCADB Case 00000585 — Root Inclusion Request](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000585)
- [Mozilla Bugzilla 1658793 — Add Cybertrust Japan SecureSign Roots (CA12, CA14, CA15)](https://bugzilla.mozilla.org/show_bug.cgi?id=1658793)
- [CCADB public discussion of Cybertrust Japan / JCSI CA Inclusion Request (mail-archive)](https://www.mail-archive.com/public@ccadb.org/msg00281.html)
- [CCADB public discussion (Google Groups)](https://groups.google.com/a/ccadb.org/g/public/c/4OuyyOD-7ng)
- [Mozilla dev-security-policy — Intent to Approve Cybertrust / JCSI Japan Root Inclusions](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/pJHiCL07E-Q)
- [Mozilla Bugzilla 1330924 — CyberTrust Japan: issuance of pairs of certificates with same name and serial number](https://bugzilla.mozilla.org/show_bug.cgi?id=1330924)
- [Mozilla Bugzilla 1827490 — Cybertrust Japan: CRL signature algorithm encoding error](https://bugzilla.mozilla.org/show_bug.cgi?id=1827490)
- [Mozilla CA Incident Dashboard](https://wiki.mozilla.org/CA/Incident_Dashboard)
- [Cybertrust Japan TLS CP/CPS repository](https://www.cybertrust.ne.jp/ssl/repository_rt/)
- [JCSI Root CA CPS v1.4 (English)](https://www.cybertrust.co.jp/jcsi/CPS_JCSIRoot_Ver1.4_EN.pdf)
- [CyberTrust — Wikipedia](https://en.wikipedia.org/wiki/CyberTrust)
