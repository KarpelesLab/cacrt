# CFCA

China Financial Certification Authority (CFCA) is a state-backed public key infrastructure provider established under the initiative of the People's Bank of China and jointly founded by major Chinese commercial banks, formally inaugurated in Beijing in 2000. Headquartered at Financial Street, Xicheng District, Beijing, China, CFCA is the largest digital authentication service organization in China, serving over 2,400 financial institutions. In the Web PKI it operates a single publicly-trusted RSA root (CFCA EV ROOT) covering TLS server authentication with EV treatment; its SM2-based and legacy SHA-1 roots are not included in any public root store.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| CFCA EV ROOT | `0b1b94ef.0` | RSA 4096 | 2029-12-31 | `5C:C3:D7:8E:4E:1D:5E:45:54:7A:04:E6:87:3E:64:F9:0C:F9:53:6D:1C:CC:2E:F8:00:F3:55:C4:C5:FD:70:FD` |

## Rationale for inclusion

The CFCA EV ROOT is included in the **Mozilla** (Firefox 38 / NSS 3.18, with EV treatment enabled in Firefox 40), **Microsoft**, **Apple**, and **Google** root stores with the Websites (TLS server authentication) trust bit enabled and EV status recognised. Mozilla's inclusion was approved following two public discussion periods on mozilla.dev.security.policy and is tracked in [Bugzilla #926029](https://bugzilla.mozilla.org/show_bug.cgi?id=926029) and the NSS addition in [Bugzilla #1131698](https://bugzilla.mozilla.org/show_bug.cgi?id=1131698). Microsoft added the root in May 2013; the August 2020 deployment notice records a `NotBefore` action limiting the Code Signing EKU on CFCA EV ROOT ([Microsoft Learn, August 2020](https://learn.microsoft.com/en-us/security/trusted-root/2020/august2020)).

The root issues EV TLS certificates through one internally-operated subordinate CA (CFCA EV OCA) and OV/DV TLS certificates through additional subordinate CAs. CFCA holds a valid License of Electronic Certification Services issued by China's Ministry of Industry and Information Technology (MIIT) and a License of Cryptography Use in Electronic Certification Services.

## CA/Browser Forum compliance

CFCA is audited annually against the **WebTrust for Certification Authorities** (CA), **WebTrust for CA — Baseline Requirements (BR)**, and **WebTrust for CA — Extended Validation SSL** criteria. The audit referenced during the EV inclusion request (2014–2015) was conducted by PricewaterhouseCoopers; more recent audit cycles (covering the period August 2021–July 2022 and later) were conducted by **Anthony Kam & Associates Ltd.**, a licensed WebTrust practitioner, with reports published via [CPA Canada](https://www.cpacanada.ca/). The 2024–2025 audit (period August 2024–July 2025) yielded an unqualified opinion.

CFCA discloses root and intermediate certificate records in the **CCADB** (Common CA Database) as required by Mozilla's CA Certificate Policy. It is a member of the CA/Browser Forum and the Asian PKI Forum. A 2021 request to add two next-generation roots (CFCA Global RSA ROOT, CFCA Global ECC ROOT) was tracked in [Bugzilla #1752685](https://bugzilla.mozilla.org/show_bug.cgi?id=1752685) but was **withdrawn** by CFCA in early 2024 (CCADB Case #980 closed).

CFCA added pre-issuance certificate linting (ZLint and PKIlint) to its issuance pipeline following the 2024 basicConstraints incident (see below); this change is documented in CP&CPS revision history (version dated August 2024). The CP&CPS states that CFCA-issued certificates are logged to Certificate Transparency logs as required by BR and EV requirements.

## Past non-compliance

Several publicly-documented compliance incidents have been filed against CFCA on Mozilla's Bugzilla. No formal distrust action has been taken by any major root program as of the writing of this document.

- **[Bug 1784820](https://bugzilla.mozilla.org/show_bug.cgi?id=1784820) (2022) — Delayed CCADB disclosure of an intermediate CA.** CFCA created the CFCA DV OCA subordinate CA in 2021 and failed to report it to the CCADB within the required 7-day window; the responsible person misunderstood the policy, believing reporting was required only after receiving an audit report. Resolved after Mozilla follow-up in June 2022.

- **[Bug 1798812](https://bugzilla.mozilla.org/show_bug.cgi?id=1798812) (2022) — Delayed CCADB disclosure of a revoked intermediate CA.** A follow-on to Bug 1784820: after CFCA DV OCA was revoked, CFCA did not update the CCADB revocation status within the required 24-hour / 7-day window. Resolved November 2022.

- **[Bug 1838371](https://bugzilla.mozilla.org/show_bug.cgi?id=1838371) (2023) — OV certificate with incorrect OrganizationName.** CFCA issued an OV TLS certificate for `www.hncdi.gov.cn` using a commercial software company's name as the O field rather than the government body listed as WHOIS registrant. Revocation of the mis-issued certificate was not completed until November 2023, approximately five months after the incident was filed, drawing criticism for the delayed response.

- **[Bug 1886135](https://bugzilla.mozilla.org/show_bug.cgi?id=1886135) (2024) — basicConstraints extension not marked critical.** When TLS BR 2.0.0 took effect (2023-09-15), CFCA's issuance pipeline did not mark the `basicConstraints` extension as critical as required by BR Section 7.1.2.7.6. CFCA's internal investigation identified 2,098 affected certificates. Per BR Section 4.9.1.1, all should have been revoked within five days; 840 were revoked in time. The remaining certificates were not revoked within the deadline — primarily certificates held by banks and critical infrastructure that cited regulatory and operational constraints — generating a separate delayed-revocation incident.

- **[Bug 1888882](https://bugzilla.mozilla.org/show_bug.cgi?id=1888882) (2024) — Delayed revocation following Bug 1886135.** Tracks the revocation of the remaining ~1,258 affected certificates. Mozilla reviewers also noted that a public report (Bug 1875820, filed January 2024) had already flagged the same basicConstraints issue weeks before CFCA's internal discovery, raising questions about CFCA's Bugzilla monitoring practices. CFCA acknowledged a monitoring gap and committed to reviewing all CA-community incident reports, not only those specific to CFCA. Closed September 2024.

- **[Bug 1949131](https://bugzilla.mozilla.org/show_bug.cgi?id=1949131) (2025) — Recurrence: basicConstraints not critical.** In February 2025, Google Root Program notified CFCA that three certificates still had the basicConstraints-not-critical defect. An internal audit found 47 additional missed certificates from the original 2023–2024 batch that had not been revoked. Root cause: the RA system's certificate records were incomplete (CTlog signature values missing from some records), combined with over-reliance on manual data tables for revocation reviews. All affected unexpired certificates were revoked by March 2025. Resolved May 2025.

A CCADB/Bugzilla search for open CFCA incidents can be performed at [Mozilla Bugzilla — CA: CFCA](https://bugzilla.mozilla.org/buglist.cgi?product=CA%20Program&component=CA%20Certificate%20Compliance&status_whiteboard=CFCA&list_id=17232049).

## Transparency

- **CP&CPS:** CFCA publishes its Certificate Policy and Certification Practice Statement for the Global-Trust System at `https://www.cfca.com.cn/` (current version v4.8, July 2025). Direct PDF: [`https://www.cfca.com.cn/upload/resources/file/2025/08/20/2446.pdf`](https://www.cfca.com.cn/upload/resources/file/2025/08/20/2446.pdf). Documents are updated at least annually; contact for policy inquiries is `cps@cfca.com.cn`.
- **CCADB:** CFCA EV ROOT and all its subordinate CAs are disclosed in the Common CA Database. Audit statements are uploaded to CCADB, referencing CPA Canada-hosted WebTrust seal URLs.
- **Incident self-reporting:** The incidents above were self-reported to Mozilla Bugzilla or, in some cases, raised by third parties with CFCA subsequently engaging. Post-2024 commitments include revised Bugzilla and risk-incident handling specifications and broader community monitoring.
- **Certificate Transparency:** CFCA's EV and TLS certificates are required to include SCTs and are submitted to public CT logs per BR and EV requirements. CFCA's CT management specifications were revised as part of the 2024 remediation commitments.

## Sources

- [Mozilla Bugzilla #926029 — CFCA root CA inclusion request](https://bugzilla.mozilla.org/show_bug.cgi?id=926029)
- [Mozilla Bugzilla #1131698 — Add CFCA EV ROOT to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=1131698)
- [Mozilla Bugzilla #1229288 — New EV Policy OID for CFCA EV root](https://bugzilla.mozilla.org/show_bug.cgi?id=1229288)
- [Mozilla Bugzilla #1752685 — Add CFCA Global Root Certificates (withdrawn)](https://bugzilla.mozilla.org/show_bug.cgi?id=1752685)
- [Mozilla Bugzilla #1784820 — CFCA: Delayed reporting of intermediate CA certificate](https://bugzilla.mozilla.org/show_bug.cgi?id=1784820)
- [Mozilla Bugzilla #1798812 — CFCA: Delayed reporting of revocation of an intermediate CA](https://bugzilla.mozilla.org/show_bug.cgi?id=1798812)
- [Mozilla Bugzilla #1838371 — CFCA: certificate with an incorrect OrganizationName](https://bugzilla.mozilla.org/show_bug.cgi?id=1838371)
- [Mozilla Bugzilla #1886135 — CFCA: certificate basicConstraints extension not marked as critical](https://bugzilla.mozilla.org/show_bug.cgi?id=1886135)
- [Mozilla Bugzilla #1888882 — CFCA: Delayed revocation of TLS certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=1888882)
- [Mozilla Bugzilla #1949131 — CFCA: BasicConstraints not marked critical — missed certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=1949131)
- [mozilla.dev.security.policy — CFCA Root Inclusion Request (first discussion)](https://groups.google.com/g/mozilla.dev.security.policy/c/2G6KuAT9Ekk)
- [mozilla.dev.security.policy — Second Discussion of CFCA Root Inclusion Request](https://groups.google.com/g/mozilla.dev.security.policy/c/aHxhbSVafUg)
- [mozilla.dev.security.policy — Something About CFCA](https://groups.google.com/g/mozilla.dev.security.policy/c/sANeulHFE8E)
- [Microsoft Trusted Root Program — August 2020 Deployment Notice](https://learn.microsoft.com/en-us/security/trusted-root/2020/august2020)
- [Microsoft Trusted Root Program — February 2024 Deployment Notice](https://learn.microsoft.com/en-us/security/trusted-root/2024/feb2024)
- [CCADB Included CA Certificate List](https://ccadb.my.salesforce-sites.com/mozilla/IncludedCACertificateReport)
- [CFCA official website](https://www.cfca.com.cn/)
- [CFCA CP&CPS (Global-Trust System, October 2024 version)](https://www.cfca.com.cn/upload/resources/file/2024/10/31/2050.pdf)
- [CFCA CP&CPS (Global-Trust System, August 2025 version)](https://www.cfca.com.cn/upload/resources/file/2025/08/20/2446.pdf)
- [Chinese Wikipedia — 中国金融认证中心](https://zh.wikipedia.org/wiki/%E4%B8%AD%E5%9B%BD%E9%87%91%E8%9E%8D%E8%AE%A4%E8%AF%81%E4%B8%AD%E5%BF%83)
