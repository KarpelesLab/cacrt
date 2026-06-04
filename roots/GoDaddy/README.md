# GoDaddy

GoDaddy, Inc. is a publicly traded US technology company headquartered in Scottsdale, Arizona, founded in 1997 as Jomax Technologies and renamed GoDaddy in 1999. In addition to being the world's largest domain registrar, GoDaddy operates a public Certificate Authority under two brands—**GoDaddy** (OID arc `2.16.840.1.114413`) and **Starfield Technologies** (OID arc `2.16.840.1.114414`), a wholly-owned subsidiary—both managed under a single CP/CPS governed by the Starfield Governance and Policy Committee. Both brands are combined in this folder.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| Go Daddy Root Certificate Authority - G2 | `cbf06781.0` | RSA 2048 | 2037-12-31 | `45:14:0B:32:47:EB:9C:C8:C5:B4:F0:D7:B5:30:91:F7:32:92:08:9E:6E:5A:63:E2:74:9D:D3:AC:A9:19:8E:DA` |
| Starfield Root Certificate Authority - G2 | `4bfab552.0` | RSA 2048 | 2037-12-31 | `2C:E1:CB:0B:F9:D2:F9:E1:02:99:3F:BE:21:51:52:C3:B2:DD:0C:AB:DE:1C:68:E5:31:9B:83:91:54:DB:B7:F5` |
| Starfield Services Root Certificate Authority - G2 | `09789157.0` | RSA 2048 | 2037-12-31 | `56:8D:69:05:A2:C8:87:08:A4:B3:02:51:90:ED:CF:ED:B1:97:4A:60:6A:13:C6:E5:29:0F:CB:2A:E6:3E:DA:B5` |

All three roots are self-signed, issued 2009-09-01, and expire 2037-12-31.

## Rationale for inclusion

All three G2 roots are included in all four major root programs:

- **Mozilla NSS / Firefox** — included following the public review of the Go Daddy Root Inclusion Request on mozilla.dev.security.policy and corresponding Bugzilla approval. Websites and Code Signing trust bits are enabled; EV is enabled for Go Daddy G2 and Starfield G2. The Starfield Services G2 root was initially held pending a concrete use-plan and was later approved.
- **Microsoft Windows** — distributed through the Microsoft Trusted Root Program.
- **Apple (macOS / iOS)** — present in Apple's shared Root Store (Trust Store version 2023071300 and later), including EV OID `2.16.840.1.114413.1.7.23.3` / `2.16.840.1.114414.1.7.24.3`.
- **Google Chrome** — Chrome relies on the OS trust store on most platforms and also ships its own Chrome Root Store; GoDaddy roots are included.

All three roots are scoped to TLS server authentication (and historically code signing). Starfield Services G2 is also the trust anchor for Amazon Trust Services' intermediate hierarchy following a root transfer.

## CA/Browser Forum compliance

GoDaddy/Starfield operates under the **WebTrust for CAs** audit framework, with separate annual engagements covering:

- WebTrust Principles and Criteria for Certification Authorities (base)
- WebTrust for CAs – SSL Baseline Requirements
- WebTrust for CAs – Extended Validation SSL
- WebTrust for CAs – Network and Certificate System Security Requirements (NSR)

Audits are conducted by **BDO International Limited** at Starfield's Phoenix, AZ and Ashburn, VA data centers. The most recent publicly available EV audit covers the period January 1, 2025 to December 31, 2025 (WebTrust EV SSL v1.8); the NSR audit covers July 1, 2024 to June 30, 2025 (NSR v1.7). Both reports are published at [certs.godaddy.com/repository](https://certs.godaddy.com/repository).

The combined **TLS Certificate Policy and Certification Practice Statement (CP/CPS)** applies to all CA hierarchies under both brands. It is publicly maintained on GitHub at [github.com/godaddy/cp-cps](https://github.com/godaddy/cp-cps) with prior PDF versions archived at `certs.godaddy.com`. The CP/CPS explicitly asserts conformance with the current CA/Browser Forum Baseline Requirements for TLS and EV Guidelines; the BRs take precedence over the CP/CPS in case of conflict.

GoDaddy supports the **ACME protocol** (RFC 8555) for automated certificate lifecycle management via External Account Binding (EAB) credentials, and embeds **Signed Certificate Timestamps (SCTs)** from multiple CT logs in all issued TLS certificates per the requirements of Chrome and other root programs.

GoDaddy's CP/CPS is disclosed in the **CCADB** (Common CA Database), and all externally-operated subordinate CAs are required to be disclosed there as well. A notable cross-sign: GoDaddy's Starfield Root G2 was used to issue two external subordinate CAs for **Certainly, LLC** (a Fastly subsidiary), processed through Mozilla's Section 8 CA Operational Changes procedure and tracked in [Bugzilla #1755851](https://bugzilla.mozilla.org/show_bug.cgi?id=1755851).

## Past non-compliance

GoDaddy has self-reported or been the subject of several documented compliance incidents. No major distrust or root removal action has been taken against GoDaddy by any major root program. The incidents below are drawn from publicly filed Bugzilla reports.

| Year | Bug | Summary |
|---|---|---|
| 2014 | [#988633](https://bugzilla.mozilla.org/show_bug.cgi?id=988633) | Improperly encoded certificate (Basic Constraints extension included default-false `cA` value). |
| 2018 | [#1484766](https://bugzilla.mozilla.org/show_bug.cgi?id=1484766) | Random Value vulnerability: validation controls for Methods 3.2.2.4.6/7 could be bypassed via Method 3.2.2.4.2; affected 865 certificates. Discovered via self-audit. |
| 2019 | [#1533774](https://bugzilla.mozilla.org/show_bug.cgi?id=1533774) | Insufficient serial number entropy (unsigned 64-bit integer misread as satisfying BR 7.1). |
| 2019 | [#1567061](https://bugzilla.mozilla.org/show_bug.cgi?id=1567061) | Inconsistent CCADB disclosure of externally-operated intermediates (cross-certificates associated with Amazon Trust Services root transfer). Resolved after CCADB clarification. |
| 2019 | [#1605804](https://bugzilla.mozilla.org/show_bug.cgi?id=1605804) | Domain validation reuse: timeframe calculated from validation to request rather than to issuance. Identified in a 3% internal audit. |
| 2020 | [#1647030](https://bugzilla.mozilla.org/show_bug.cgi?id=1647030) | Agreed-Upon Website domain validation bypass: a developer omitted a variable, allowing prior-FQDN checks to be skipped. |
| 2024 | [#1904749](https://bugzilla.mozilla.org/show_bug.cgi?id=1904749) | CAA check failure: "contains" match rather than exact match for issuer-domain-name per RFC 8659. First mis-issuance 2017-09-23; 168 active certificates revoked on 2024-06-28. |
| 2024 | [#1905419](https://bugzilla.mozilla.org/show_bug.cgi?id=1905419) | Intermittent OCSP "unauthorized" response for newly-issued certificates due to propagation lag; root programs queried whether the BRs covered this case. |
| 2025 | [#1942877](https://bugzilla.mozilla.org/show_bug.cgi?id=1942877) | Delayed revocation of 35 certificates with keys from the Fortinet leak: email malware filters silently blocked .key/.crt attachments to `practices@starfieldtech.com`, delaying revocation 3–4 days. Resolved with a web-based reporting form (sec.godaddy.com/report-certificate). |
| 2025 | [#1969296](https://bugzilla.mozilla.org/show_bug.cgi?id=1969296) | Two certificates contained invalid embedded SCT signatures from DigiCert Yeti2025 CT log; GoDaddy had validated response structure but not the signature value. Both certificates revoked within the 5-day window. |

No distrust or removal of GoDaddy roots from any major root program has been found in public records as of June 2026. For a full list of open and resolved issues, see the [Mozilla Bugzilla CA Program search for GoDaddy](https://bugzilla.mozilla.org/buglist.cgi?product=CA%20Program&component=GoDaddy).

## Transparency

- **CP/CPS**: Published publicly on GitHub ([github.com/godaddy/cp-cps](https://github.com/godaddy/cp-cps)) and archived at [certs.godaddy.com/repository](https://certs.godaddy.com/repository). Both the GoDaddy and Starfield hierarchies operate under the same document.
- **CCADB**: GoDaddy/Starfield discloses root and intermediate certificate records in the CCADB, including externally-operated subordinate CAs. Audit reports and CP/CPS links are maintained there and updated at least annually per Mozilla Root Store Policy.
- **Incident self-reporting**: GoDaddy self-reports compliance issues to Mozilla Bugzilla. Multiple incidents listed above were discovered through GoDaddy's own 3% self-audit program before external discovery.
- **Certificate Transparency**: Per CP/CPS Section 4.4.2, all TLS certificates are submitted to publicly accessible CT logs, and SCTs are embedded in issued certificates. GoDaddy's CT practices were highlighted in [Bug #1969296](https://bugzilla.mozilla.org/show_bug.cgi?id=1969296) (2025), where inadequate SCT signature validation led to two certificates with malformed SCTs being issued and subsequently revoked.
- **CRL/OCSP**: CRL distribution points for all root and intermediate hierarchies are published at `certs.godaddy.com/repository` and `certs.starfieldtech.com`.

## Sources

- [GoDaddy/Starfield CP/CPS on GitHub (TLS_CPCPS.md)](https://github.com/godaddy/cp-cps/blob/main/cpcps/TLS_CPCPS.md)
- [GoDaddy Certificate Repository](https://certs.godaddy.com/repository)
- [WebTrust EV SSL Audit Report — Starfield Technologies (BDO, 2025)](https://certs.godaddy.com/repository/webtrust/en/WebTrustPrinciplesAndCriteriaEV.pdf)
- [WebTrust NSR Audit Report — Starfield Technologies (BDO)](https://certs.starfieldtech.com/repository/webtrust/en/WebTrustPrinciplesAndCriteriaNSR.pdf)
- [Mozilla Bugzilla #527056 — Add Go Daddy G2 CA Certs to root store](https://bugzilla.mozilla.org/show_bug.cgi?id=527056)
- [mozilla.dev.security.policy — Go Daddy Root Inclusion Request](https://groups.google.com/g/mozilla.dev.security.policy/c/jnp-TvHtkCw)
- [Mozilla Bugzilla #1755851 — GoDaddy cross-signing two Certainly Intermediate Certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=1755851)
- [mozilla.dev.security.policy — Public Discussion of GoDaddy cross-signing Certainly](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/bEnn98Dajzc/m/4DnwaOBnAwAJ)
- [Mozilla Bugzilla #1567061 — GoDaddy: inconsistent disclosure of externally-operated intermediate](https://bugzilla.mozilla.org/show_bug.cgi?id=1567061)
- [Mozilla Bugzilla #988633 — GoDaddy: improperly encoded certificate](https://bugzilla.mozilla.org/show_bug.cgi?id=988633)
- [Mozilla Bugzilla #1484766 — GoDaddy: Random Value Vulnerability in Domain Validation](https://bugzilla.mozilla.org/show_bug.cgi?id=1484766)
- [Mozilla Bugzilla #1533774 — GoDaddy: Insufficient serial number entropy](https://bugzilla.mozilla.org/show_bug.cgi?id=1533774)
- [Mozilla Bugzilla #1605804 — GoDaddy: Domain Validation Reuse Issue](https://bugzilla.mozilla.org/show_bug.cgi?id=1605804)
- [Mozilla Bugzilla #1647030 — GoDaddy: Agreed-Upon Website Domain Validation Method Issue](https://bugzilla.mozilla.org/show_bug.cgi?id=1647030)
- [Mozilla Bugzilla #1904749 — GoDaddy: CAA checks passed when records contained incorrect variants](https://bugzilla.mozilla.org/show_bug.cgi?id=1904749)
- [Mozilla Bugzilla #1905419 — GoDaddy: Intermittent unauthorized OCSP response](https://bugzilla.mozilla.org/show_bug.cgi?id=1905419)
- [Mozilla Bugzilla #1942877 — GoDaddy: Delayed revocation](https://bugzilla.mozilla.org/show_bug.cgi?id=1942877)
- [Mozilla Bugzilla #1969296 — GoDaddy: Certificates with invalid embedded SCT signatures](https://bugzilla.mozilla.org/show_bug.cgi?id=1969296)
- [Mozilla Bugzilla CA Program — full GoDaddy incident list](https://bugzilla.mozilla.org/buglist.cgi?product=CA%20Program&component=GoDaddy)
- [Apple Root Store — iOS 17 / macOS 14 trusted certificates list](https://support.apple.com/en-us/105116)
- [CCADB — Add and Update Root Certificate Records](https://www.ccadb.org/cas/updates)
- [GoDaddy ACME Setup documentation](https://www.godaddy.com/help/set-up-my-ssl-certificate-with-acme-40393)
- [SSLMate Blog — Apple CT policy and GoDaddy certificates (2021)](https://sslmate.com/blog/post/apples_new_ct_policy)
