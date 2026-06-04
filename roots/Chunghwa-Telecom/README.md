# Chunghwa Telecom

Chunghwa Telecom Co., Ltd. (中華電信) is Taiwan's largest integrated telecommunications operator, headquartered in the Zhongzheng District of Taipei and partially state-owned via the Ministry of Transportation and Communications. The company operates two public certificate authority brands — **ePKI** and **HiPKI** — issuing publicly-trusted TLS certificates under its own name; this folder covers only the HiPKI brand (HiPKI Root CA – G1). Chunghwa Telecom also operates GTLSCA (Government TLS CA) on behalf of Taiwan's Ministry of Digital Affairs (MODA), though GTLSCA is a subordinate CA and not represented by a separate root in this folder.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| HiPKI Root CA – G1 | `90c5a3c8.0` | RSA 4096 | 2037-12-31 | `F0:15:CE:3C:C2:39:BF:EF:06:4B:E9:F1:D2:C4:17:E1:A0:26:4A:0A:94:BE:1F:0C:8D:12:18:64:EB:69:49:CC` |

Full subject: `C=TW, O=Chunghwa Telecom Co., Ltd., CN=HiPKI Root CA - G1`
Self-signed (root); not-before 2019-02-22.

## Rationale for inclusion

HiPKI Root CA – G1 was added to the Mozilla NSS root store in the NSS 3.74 / Firefox 97 release cycle (late 2021) following the standard Mozilla CA inclusion process, with the Websites trust bit and EV treatment enabled. [Mozilla Bugzilla #1563417](https://bugzilla.mozilla.org/show_bug.cgi?id=1563417) and [#1733012](https://bugzilla.mozilla.org/show_bug.cgi?id=1733012) document the review and landing. The root covers TLS server authentication for websites operated by Chunghwa Telecom's HiPKI hierarchy, including EV certificates.

The root is also trusted by Apple (macOS/iOS) and Microsoft (Windows), and has historically been trusted by Google Chrome. However, as of Chrome 139 (targeting stable release 2025-08-05), **Google removed default Chrome Root Store trust** for certificates issued by HiPKI Root CA – G1 (and the companion ePKI root) with an earliest SCT dated after 2025-07-31. Mozilla Firefox, Apple Safari, and Microsoft Edge were not part of that action and continued to trust the root under their own policies. [Bleeping Computer, June 2025](https://www.bleepingcomputer.com/news/security/google-chrome-to-distrust-chunghwa-telecom-netlock-certificates-in-august/)

## CA/Browser Forum compliance

Chunghwa Telecom's HiPKI hierarchy is audited annually under the WebTrust scheme:

- WebTrust for Certification Authorities (WebTrust for CAs)
- WebTrust for CAs – SSL Baseline Requirements (TLS BR)
- WebTrust for CAs – Extended Validation SSL

Audit seals are published on the [CHT PKI repository](https://chtca.hinet.net/en/repository.html). The CP and CPS documents are structured per RFC 3647 and explicitly commit to CA/Browser Forum Baseline Requirements and Network and Certificate System Security Requirements, with the BR taking precedence over CPS in the event of conflict. [HiPKI CPS v1.1 (PDF)](https://chtca.hinet.net/assets/en/file/TLS/HiPKICA-CPS-v1.1-en.pdf); [HiPKI CP v1.3 (PDF)](https://chtca.hinet.net/assets/en/file/TLS/HiPKI-CP-v1.3-en.pdf)

Certificate Transparency is required and implemented; ACME was deployed in the HiPKI production environment as of 2024 for automated certificate renewal of the test website, with plans to extend it more broadly. [Bugzilla #1904038](https://bugzilla.mozilla.org/show_bug.cgi?id=1904038)

CCADB disclosure is maintained for the HiPKI Root CA – G1 record and its subordinate CAs, including annual audit statements. The CA's linting tooling uses a self-developed module based on the open-source ZLint, supplemented by checks against blocklists and phishing databases, though reviewers have noted that certain lint failures went undetected due to outdated ZLint versions. [Bugzilla #1916392](https://bugzilla.mozilla.org/show_bug.cgi?id=1916392)

## Past non-compliance

Several publicly-documented incidents were filed in Mozilla Bugzilla and discussed on the mozilla.dev.security.policy list between 2023 and 2025. All involve Chunghwa Telecom's GTLSCA subordinate CA (operated for Taiwan's Ministry of Digital Affairs), which chains to ePKI and not to the HiPKI root in this folder — however, Google cited these incidents as part of the pattern that led to the Chrome distrust of HiPKI Root CA – G1 as well. The incidents are therefore directly relevant to this root's standing.

**1. EKU misissuance and delayed revocation (2024) — [Bugzilla #1887096](https://bugzilla.mozilla.org/show_bug.cgi?id=1887096), [#1892419](https://bugzilla.mozilla.org/show_bug.cgi?id=1892419)**
GTLSCA issued 6,453 certificates with the extKeyUsage extension incorrectly marked critical (non-compliant with TLS BR v2.0.0). The CA was notified on 2024-03-19; the BR-mandated 5-day revocation deadline was 2024-03-24. Revocation proceeded in batches over approximately two months, completing on 2024-05-13. Chunghwa Telecom cited government-agency bureaucracy, lack of a bulk-reissuance program, and operator pressure as justification for the delay. Mozilla reviewers rejected those justifications as CA-side decision-making failures and characterized the response as inadequate; the bug was ultimately closed RESOLVED FIXED on 2025-02-12.

**2. Delayed revocation — SubjectDirectoryAttributes extension (2024) — [Bugzilla #1903066](https://bugzilla.mozilla.org/show_bug.cgi?id=1903066)**
A follow-on incident required revocation of 12,911 certificates containing a controversial SubjectDirectoryAttributes (OID 2.5.29.9) extension. The CA again failed to revoke within the BR's 5-day window, citing scheduling conflicts with the previous batch-revocation cycle. Mozilla community members reiterated that subscriber-side or operational constraints are not valid grounds for exceeding revocation deadlines.

**3. Duplicate LocalityName misissuance (2024) — [Bugzilla #1916392](https://bugzilla.mozilla.org/show_bug.cgi?id=1916392)**
247 TLS certificates were found to contain two LocalityName (L) values in the SubjectDN, violating the Baseline Requirements. The root cause was an automated DN-compilation routine that generated two L= fields for sub-municipal government entities. All 247 certificates were revoked by 2024-09-06. Reviewers noted the issue could have been caught by pre/post-issuance linting.

**4. CCADB self-assessment not submitted (2024) — [Bugzilla #1946414](https://bugzilla.mozilla.org/show_bug.cgi?id=1946414)**
GTLSCA failed to submit its first-ever required CCADB annual policy self-assessment for 2023, in violation of Chrome Root Program Policy Section 6. The root cause was a personnel reassignment without a proper handover. Chrome's Ryan Dickson found the initial remediation plan insufficient; the bug was closed RESOLVED FIXED after CHT revised its internal tracking and submitted the 2024 assessment.

**5. Expired CCADB test website URL (2024) — [Bugzilla #1904038](https://bugzilla.mozilla.org/show_bug.cgi?id=1904038)**
The "Test Website – Valid" URL disclosed in CCADB pointed to a certificate that had been expired for 278 days, violating TLS BR Section 2.2. Remediated by implementing ACME-based automated renewal for the test endpoint.

**6. Google Chrome distrust (effective 2025-08-01) — [BleepingComputer](https://www.bleepingcomputer.com/news/security/google-chrome-to-distrust-chunghwa-telecom-netlock-certificates-in-august/)**
Google announced on 2025-05-30 that the Chrome Root Store would cease to trust TLS certificates from HiPKI Root CA – G1 and ePKI Root Certification Authority (and Netlock) whose earliest SCT post-dates 2025-07-31, effective Chrome 139 (stable ~2025-08-05). Google cited "patterns of compliance failures, unmet improvement commitments, and the absence of tangible, measurable progress in response to publicly disclosed incident reports." Chunghwa Telecom disputed the framing, stating the decision was not related to any cybersecurity issues. Mozilla, Apple, and Microsoft did not take parallel distrust action as of this writing. Chunghwa Telecom has stated it aims to regain Chrome default trust by March 2026.

No incidents of distrust by Mozilla, Apple, or Microsoft were found. For a comprehensive search of Mozilla Bugzilla incidents filed against this CA, see [Bugzilla CA: Chunghwa Telecom](https://bugzilla.mozilla.org/buglist.cgi?product=CA%20Program&component=CA%20Certificate%20Compliance&short_desc=Chunghwa+Telecom&short_desc_type=allwordssubstr).

## Transparency

- **CP/CPS**: Published in English and Chinese at [chtca.hinet.net/en/repository.html](https://chtca.hinet.net/en/repository.html). Documents follow RFC 3647 structure. Current versions include HiPKI CP v1.3 and HiPKICA CPS v1.1.
- **CCADB disclosure**: Root and subordinate CA records are maintained in the CCADB, including audit statements. Disclosure lapses were documented in Bugzilla #1946414 (see above).
- **Certificate Transparency**: CT logging is required for all issued certificates; enforced by browser CT policies.
- **Incident self-reporting**: CHT has filed incident reports for the events listed above. Community reviewers have at times found the initial reports and action items insufficient in depth or root-cause analysis.
- **ACME**: Deployed in the HiPKI production environment for test certificate automation as of late 2024; broader subscriber ACME support is noted in public discussion. [Bugzilla #1904038](https://bugzilla.mozilla.org/show_bug.cgi?id=1904038)

## Sources

- [Mozilla Bugzilla #1563417 – Add Chunghwa Telecom's HiPKI Root CA -G1 Certificate](https://bugzilla.mozilla.org/show_bug.cgi?id=1563417)
- [Mozilla Bugzilla #1733012 – Add HiPKI Root CA - G1 to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=1733012)
- [Mozilla Bugzilla #1733014 – Enable EV Treatment for HiPKI Root CA - G1](https://bugzilla.mozilla.org/show_bug.cgi?id=1733014)
- [Mozilla Bugzilla #1887096 – Wrong Extended Key Usage setting by GTLSCA](https://bugzilla.mozilla.org/show_bug.cgi?id=1887096)
- [Mozilla Bugzilla #1892419 – Chunghwa Telecom: Delayed Revocation Due to GTLSCA EKU Misissuance](https://bugzilla.mozilla.org/show_bug.cgi?id=1892419)
- [Mozilla Bugzilla #1903066 – Chunghwa Telecom: Delayed Revocation with Controversial Extension](https://bugzilla.mozilla.org/show_bug.cgi?id=1903066)
- [Mozilla Bugzilla #1904038 – Chunghwa Telecom: "Test Website - Valid" URL disclosed to CCADB is expired](https://bugzilla.mozilla.org/show_bug.cgi?id=1904038)
- [Mozilla Bugzilla #1916392 – Chunghwa Telecom: TLS Certificates Contains two LocalityName Values in SubjectDN by GTLSCA](https://bugzilla.mozilla.org/show_bug.cgi?id=1916392)
- [Mozilla Bugzilla #1946414 – Chunghwa Telecom: Failure to Submit Annual CCADB Self-Assessment 2023 by GTLSCA](https://bugzilla.mozilla.org/show_bug.cgi?id=1946414)
- [mozilla.dev.security.policy – Public Discussion of Chunghwa Telecom's Root Inclusion Request](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/uvdF8RTRFPc)
- [BleepingComputer – Google Chrome to distrust Chunghwa Telecom, Netlock certificates in August](https://www.bleepingcomputer.com/news/security/google-chrome-to-distrust-chunghwa-telecom-netlock-certificates-in-august/)
- [The Hacker News – Google Chrome to Distrust Two Certificate Authorities Over Compliance and Conduct Issues](https://thehackernews.com/2025/06/google-chrome-to-distrust-two.html)
- [CHT PKI Repository (chtca.hinet.net)](https://chtca.hinet.net/en/repository.html)
- [HiPKI CPS v1.1 (PDF)](https://chtca.hinet.net/assets/en/file/TLS/HiPKICA-CPS-v1.1-en.pdf)
- [HiPKI CP v1.3 (PDF)](https://chtca.hinet.net/assets/en/file/TLS/HiPKI-CP-v1.3-en.pdf)
- [Wikipedia – Chunghwa Telecom](https://en.wikipedia.org/wiki/Chunghwa_Telecom)
