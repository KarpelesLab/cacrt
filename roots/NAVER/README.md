# NAVER

NAVER Cloud Trust Services Corp. is a South Korean certificate authority subsidiary of NAVER Cloud Corp. (itself a subsidiary of NAVER Corporation, the dominant South Korean internet conglomerate headquartered in Seongnam, Gyeonggi-do). The root in this folder was originally operated by NAVER BUSINESS PLATFORM Corp., then by NAVER Cloud Corp. (from 11 September 2017), and since 5 June 2023 by NAVER Cloud Trust Services Corp.; it issues publicly-trusted TLS server-authentication certificates under a single RSA root for NAVER services and external subscribers.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| NAVER Global Root Certification Authority | `3fb36b73.0` | RSA 4096 | 2037-08-18 | `88:F4:38:DC:F8:FF:D1:FA:8F:42:91:15:FF:E5:F8:2A:E1:E0:6E:0C:70:C3:75:FA:AD:71:7B:34:A4:9E:72:65` |

## Rationale for inclusion

The NAVER Global Root Certification Authority is trusted for TLS server authentication by all four major root programs:

- **Mozilla NSS / Firefox** — included in NSS 3.60, shipping with Firefox 85 (December 2020). Inclusion approved via [Bugzilla #1404221](https://bugzilla.mozilla.org/show_bug.cgi?id=1404221) following a public comment period opened October 2020 on [mozilla.dev.security.policy](https://groups.google.com/g/mozilla.dev.security.policy/c/edkFKcdJWZU).
- **Microsoft** — the first major program to include the root; present before the Mozilla inclusion.
- **Google Android / Chrome** — included prior to October 2022.
- **Apple iOS, macOS, iPadOS, watchOS, tvOS** — included October 2022, completing a five-year inclusion journey across all major platforms. ([NAVER Cloud Medium announcement](https://medium.com/naver-cloud/news-naver-clouds-root-certificate-authority-root-ca-now-loaded-on-major-oss-browsers-94c871817896))

The root is scoped to TLS server-authentication (websites trust bit); no e-mail or code-signing trust bits are asserted.

## CA/Browser Forum compliance

- **Audit framework:** WebTrust for Certification Authorities and WebTrust for Baseline Requirements, performed annually since 2018. The auditor through at least the 2019 audit period was Deloitte. NAVER Cloud Trust Services holds active WebTrust seals (three seal IDs) via CPA Canada. ([navercloudtrust.com](https://navercloudtrust.com/))
- **Baseline Requirements:** The CP/CPS explicitly defers to the CA/Browser Forum Baseline Requirements in case of conflict. NAVER is a member of the CA/Browser Forum. ([NAVER Cloud Trust Services CPS v1.0.3](https://ssl.pstatic.net/static/rootca/CPS/NAVER_Cloud_Trust_Services_CPS_v1.0.3.pdf))
- **CCADB disclosure:** NAVER has been disclosing to the CCADB since at least the Microsoft root program inclusion; NAVER Cloud later requested CCADB API access to manage subordinate CA records ([Bugzilla #1779692](https://bugzilla.mozilla.org/show_bug.cgi?id=1779692)).
- **Certificate Transparency:** No independent CT log is operated by NAVER Cloud Trust Services; issued certificates are expected to be logged in public CT logs per BR requirements. No CT-specific exclusion or exception has been publicly documented.

## Past non-compliance

One documented compliance incident has been publicly filed:

**DV misissuance — `test.com` (2023-11-24, [Bugzilla #1866448](https://bugzilla.mozilla.org/show_bug.cgi?id=1866448)):** During QA testing of the newly launched NCP Certificate Manager service, a single DV certificate was improperly issued for `test.com`. A QA engineer exploited excessive access rights to the internal outbound mailer to retrieve and use a domain-validation approval link intended for the subscriber, bypassing proper control of the domain. The certificate was identified and revoked on the same day (within approximately 40 minutes of issuance). Three corrective actions were completed between 2023-11-24 and 2023-12-21, including removal of internal mailer access rights, blocking the validation approval page from internal tools, and requiring the random value to be submitted only through the subscriber's authenticated NCP CM account. Email validation was suspended until the third action was complete. The bug was resolved as FIXED with a whiteboard tag of `[ca-compliance] [dv-misissuance]`.

**Audit-statement timing (audit period ending 2019-11-30):** During the Mozilla inclusion review, Mozilla noted that NAVER's audit statement for the period ending 30 November 2019 was dated 28 April 2020 — more than 93 days after the audit period end date, exceeding the 90-day expectation. NAVER explained this resulted from uncertainty about CCADB notification procedures while the root was only in the Microsoft root program. No formal sanction was issued. ([Bugzilla #1404221](https://bugzilla.mozilla.org/show_bug.cgi?id=1404221))

**Lint errors during Mozilla review:** Two certificates found to have non-BR-compliant fields were discovered and revoked during the Mozilla inclusion review; both were issued to NAVER-controlled domains. No public incident report beyond the inclusion bug exists for these.

No distrust action has been taken against NAVER Cloud Trust Services by any root program. For completeness: a search of Mozilla's [CA Incident Dashboard](https://wiki.mozilla.org/CA/Incident_Dashboard) and Bugzilla returns Bugzilla #1866448 as the sole post-inclusion compliance bug for this CA.

## Transparency

- **CP/CPS:** Publicly available in full. Current version is CPS v1.0.8 (dated 27 November 2025), published at [navercloudtrustservices.github.io/PolicyDocument/](https://navercloudtrustservices.github.io/PolicyDocument/) and [navercloudtrust.com](https://navercloudtrust.com/). Prior CPS versions under earlier operating entities are also archived ([NBP CPS v1.2](https://ssl.pstatic.net/static/rootca/CPS/NBP_CPS_v1.2.pdf); [NAVER Cloud Trust Services CPS v1.0.3](https://ssl.pstatic.net/static/rootca/CPS/NAVER_Cloud_Trust_Services_CPS_v1.0.3.pdf)).
- **CCADB:** The NAVER Global Root Certification Authority is listed in the CCADB under the owner "NAVER Cloud Trust Services" with annual audit updates. ([CA Certificates in Firefox report](https://ccadb.my.salesforce-sites.com/mozilla/CACertificatesInFirefoxReport))
- **Incident self-reporting:** The 2023 misissuance (Bugzilla #1866448) was self-reported with a preliminary incident report posted the same day as issuance and revocation.
- **Certificate Transparency:** Publicly-trusted certificates issued by this CA are expected to appear in public CT logs per the CA/Browser Forum Baseline Requirements; no CT log is operated by NAVER Cloud Trust Services itself.

## Sources

- [Mozilla Bugzilla #1404221 — Add Root certificate of NAVER Business Platform](https://bugzilla.mozilla.org/show_bug.cgi?id=1404221)
- [Mozilla Bugzilla #1678166 — Add NAVER Global Root Certification Authority root cert to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=1678166)
- [Mozilla Bugzilla #1866448 — NAVER Cloud Trust Services: DV Certificate issued with improperly validated domain](https://bugzilla.mozilla.org/show_bug.cgi?id=1866448)
- [Mozilla Bugzilla #1779692 — NAVER Cloud — Request for access to CCADB API](https://bugzilla.mozilla.org/show_bug.cgi?id=1779692)
- [NAVER: Public Discussion of Root Inclusion Request — mozilla.dev.security.policy](https://groups.google.com/g/mozilla.dev.security.policy/c/edkFKcdJWZU)
- [NAVER Cloud Trust Services — official CA portal](https://navercloudtrust.com/)
- [NAVER Cloud Trust Services Policy Document repository](https://navercloudtrustservices.github.io/PolicyDocument/)
- [NAVER Cloud Trust Services CPS v1.0.3 (PDF)](https://ssl.pstatic.net/static/rootca/CPS/NAVER_Cloud_Trust_Services_CPS_v1.0.3.pdf)
- [NAVER BUSINESS PLATFORM Corp. CPS v1.2 (PDF)](https://ssl.pstatic.net/static/rootca/CPS/NBP_CPS_v1.2.pdf)
- [NAVER Cloud Medium — Root CA now loaded on major OSs/browsers worldwide](https://medium.com/naver-cloud/news-naver-clouds-root-certificate-authority-root-ca-now-loaded-on-major-oss-browsers-94c871817896)
- [CCADB — CA Certificates in Firefox Report](https://ccadb.my.salesforce-sites.com/mozilla/CACertificatesInFirefoxReport)
- [Mozilla CA Incident Dashboard](https://wiki.mozilla.org/CA/Incident_Dashboard)
- [Naver Corporation — Wikipedia](https://en.wikipedia.org/wiki/Naver_Corporation)
