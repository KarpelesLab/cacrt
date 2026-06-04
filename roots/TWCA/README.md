# TWCA

Taiwan-CA Inc. (TWCA) is a commercial certificate authority headquartered in Taipei, Taiwan, formed as a joint venture by Taiwan Stock Exchange Corporation (TWSE), Taiwan Depository and Clearing Corporation (TDCC), Financial Information Service Corporation (FISC), and HiTrust Inc. It provides TLS/SSL, S/MIME, and identity certificates primarily for Taiwan's financial sector and the broader public internet, and operates in the Web PKI under the brand name "TAIWAN-CA."

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| TWCA Root Certification Authority | b7a5b843.0 | RSA 2048 | 2030-12-31 | `BF:D8:8F:E1:10:1C:41:AE:3E:80:1B:F8:BE:56:35:0E:E9:BA:D1:A6:B9:BD:51:5E:DC:5C:6D:5B:87:11:AC:44` |
| TWCA Global Root CA | 5f15c80c.0 | RSA 4096 | 2030-12-31 | `59:76:90:07:F7:68:5D:0F:CD:50:87:2F:9F:95:D5:75:5A:5B:2B:45:7D:81:F3:69:2B:61:0A:98:67:2F:0E:1B` |
| TWCA CYBER Root CA | b8d25de6.0 | RSA 4096 | 2047-11-22 | `3F:63:BB:28:14:BE:17:4E:C8:B6:43:9C:F0:8D:6D:56:F0:B7:C4:05:88:3A:56:48:A3:34:42:4D:6B:3E:C5:58` |

## Rationale for inclusion

All three roots are included in the **Mozilla** root store. The original "TWCA Root Certification Authority" (RSA 2048) was approved in [Bug 518503](https://bugzilla.mozilla.org/show_bug.cgi?id=518503) with websites and email trust bits, and later enabled for EV ([Bug 823770](https://bugzilla.mozilla.org/show_bug.cgi?id=823770)). "TWCA Global Root CA" (RSA 4096) was approved in [Bug 810133](https://bugzilla.mozilla.org/show_bug.cgi?id=810133) with websites, email, and code-signing trust bits plus EV. "TWCA CYBER Root CA" was approved in [Bug 1849702](https://bugzilla.mozilla.org/show_bug.cgi?id=1849702) (CCADB case #1244) with websites trust bit and EV, landing in NSS 3.104 / Firefox 131 (August 2024).

"TWCA CYBER Root CA" and "TWCA Global Root CA G2" (not present in this folder) were also added to the **Microsoft** Trusted Root Certificate Program in the [November 2023 deployment](https://learn.microsoft.com/en-us/security/trusted-root/2023/nov2023). The original "TWCA Root Certification Authority" has been present in **Apple**'s trust store since iOS 5/iOS 6 ([Apple support note](https://support.apple.com/en-gw/103612)).

TWCA issues OV-level TLS certificates and EV SSL certificates; it does not issue DV-level certificates. Audits are performed annually by KPMG under WebTrust criteria.

## CA/Browser Forum compliance

TWCA is audited annually against:

- **WebTrust for Certification Authorities** (Principles & Criteria for CAs)
- **WebTrust for CAs – SSL Baseline Requirements** (CA/Browser Forum Baseline Requirements)
- **WebTrust for CAs – Extended Validation SSL** (for roots with EV trust)

Audit reports from 2008 through 2025 are publicly available from TWCA's [repository](https://www.twca.com.tw/repository?lang=en). Historical audits were performed by SunRise CPAs' Firm (a DFK member firm); recent reports list KPMG. TWCA is enrolled in CCADB and discloses intermediate CAs, CP/CPS documents, and audit reports through that system. Issued TLS certificates are logged to Certificate Transparency logs; TWCA upgraded its CA software to embed SCTs and check CAA records following the 2017 non-compliance incident (see below). ACME with ARI support was on TWCA's development roadmap as of 2024 ([Bug 1886110](https://bugzilla.mozilla.org/show_bug.cgi?id=1886110)).

## Past non-compliance

TWCA has accumulated several publicly-documented compliance incidents in Mozilla's Bugzilla:

**2017 – Non-BR-compliant certificate issuance ([Bug 1391068](https://bugzilla.mozilla.org/show_bug.cgi?id=1391068)):** TWCA issued a certificate that did not comply with the CA/Browser Forum Baseline Requirements. The mis-issued certificate was promptly revoked. Mozilla reviewers noted that TWCA's initial response addressed only the immediate problem without describing systemic changes; TWCA subsequently committed to system controls to prevent high-risk domain issuance and to scan the certificate database for prohibited domain names.

**2021 – Intermediate CA certificate missing from audit reports ([Bug 1716670](https://bugzilla.mozilla.org/show_bug.cgi?id=1716670)):** A cross-signed CA certificate (created in 2014 to bridge the RSA 2048 and RSA 4096 roots) was inadvertently omitted from TWCA's WebTrust audit reports for several years. Mozilla found TWCA's initial remediation plan — to fix the issue in the next scheduled annual audit — unacceptable and required more prompt action.

**2021 – Policy OID documentation gap ([Bug 1738778](https://bugzilla.mozilla.org/show_bug.cgi?id=1738778)):** A community member identified that TWCA's CP/CPS did not clearly state that Device Certificates bearing a specific policy OID were managed in accordance with the Baseline Requirements. TWCA acknowledged the documentation deficiency and updated its CPS in late 2021 and 2022; Mozilla considered this a documentation issue rather than a misissuance.

**2024 – Delayed revocation of TLS certificates with non-critical basicConstraints ([Bug 1886110](https://bugzilla.mozilla.org/show_bug.cgi?id=1886110)):** Of 16,481 affected TLS certificates whose `basicConstraints` extension was not marked critical (a BR violation), TWCA did not complete revocation within the BR-mandated 5-day window for all certificates. Mozilla's reviewer was critical of TWCA's remediation response, noting that statements such as "we commit to adhering to the BRs" did not constitute a firm policy change. TWCA ultimately committed to stricter user-agreement requirements and acknowledged CRL publication frequency as a factor; the bug was closed in February 2025. During the public discussion for the 2024 root inclusion request, at least one community member suggested Mozilla consider a motion of distrust given this and other open bugs, though Mozilla proceeded with inclusion after review ([dev-security-policy thread](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/NqQQcki4fqs)).

No Mozilla distrust action has been taken against any TWCA root. For a comprehensive list of open and resolved bugs, see the [TWCA component in Mozilla Bugzilla](https://bugzilla.mozilla.org/buglist.cgi?product=CA%20Program&component=CA%20Certificate%20Compliance&short_desc=TWCA&short_desc_type=allwordssubstr).

## Transparency

TWCA's CP and CPS documents are publicly available in English and Chinese at [twca.com.tw/repository](https://www.twca.com.tw/repository?lang=en). Current documents include:

- **CP** v2.6.3 (2025-11-15)
- **TWCA Global CA CPS** v3.0.1 (2025-11-15)
- **CYBER CA CPS** v2.0.1 (2025-11-15)

Annual WebTrust audit reports are posted to the same repository (2008–2025) and disclosed in CCADB. TWCA filed a CCADB Self-Assessment for its 2024 root inclusion request. Issued TLS certificates appear in public Certificate Transparency logs (visible, e.g., via [Cloudflare Radar for TWCA Secure SSL CA](https://radar.cloudflare.com/certificate-transparency/ca/1A2C75FD096E0499E9FF6AC74E526F61EAAE3EDFC8C2EA4436FEE0C24D8B7D0E)).

## Sources

- [TWCA Repository (CP, CPS, Audit Reports)](https://www.twca.com.tw/repository?lang=en)
- [TWCA CYBER CA CPS (PDF, 2023)](https://www.twca.com.tw/upload/saveArea/filePage/20231020/767ceae91ed443359ec4dd38fc9f78fb/767ceae91ed443359ec4dd38fc9f78fb.pdf)
- [Mozilla Bugzilla Bug 518503 – Add TWCA Root Certificate](https://bugzilla.mozilla.org/show_bug.cgi?id=518503)
- [Mozilla Bugzilla Bug 810133 – Add TWCA Global Root CA](https://bugzilla.mozilla.org/show_bug.cgi?id=810133)
- [Mozilla Bugzilla Bug 823770 – Enable TWCA Root CA for EV](https://bugzilla.mozilla.org/show_bug.cgi?id=823770)
- [Mozilla Bugzilla Bug 1849702 – Add Taiwan-CA Inc. (TWCA) root certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=1849702)
- [Mozilla Bugzilla Bug 1908004 – Add Taiwan CA Roots to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=1908004)
- [Mozilla Bugzilla Bug 1391068 – Taiwan-CA: Non-BR-Compliant Certificate Issuance](https://bugzilla.mozilla.org/show_bug.cgi?id=1391068)
- [Mozilla Bugzilla Bug 1716670 – TWCA: Intermediate CA Certificate Missing from Audit Reports](https://bugzilla.mozilla.org/show_bug.cgi?id=1716670)
- [Mozilla Bugzilla Bug 1738778 – TWCA: Policy OID not set to indicate assurance level](https://bugzilla.mozilla.org/show_bug.cgi?id=1738778)
- [Mozilla Bugzilla Bug 1886110 – TWCA: Revocation delay for TLS certificates with non-critical basicConstraints](https://bugzilla.mozilla.org/show_bug.cgi?id=1886110)
- [Mozilla dev-security-policy: Approval of Taiwan CA's Root Inclusion Request](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/NqQQcki4fqs)
- [Mozilla dev-security-policy: TWCA Root Inclusion Request (discussion)](https://groups.google.com/g/mozilla.dev.security.policy/c/VxTA29IDBZs/m/6PN70sqZ3KcJ)
- [Microsoft Trusted Root Program – November 2023 Deployment Notice](https://learn.microsoft.com/en-us/security/trusted-root/2023/nov2023)
- [Apple – iOS 5/6 Trusted Root Certificates](https://support.apple.com/en-gw/103612)
- [Cloudflare Radar – CT for TWCA Secure SSL CA](https://radar.cloudflare.com/certificate-transparency/ca/1A2C75FD096E0499E9FF6AC74E526F61EAAE3EDFC8C2EA4436FEE0C24D8B7D0E)
- [CPA Canada – TWCA WebTrust Audit Report (example)](https://www.cpacanada.ca/api/getPDFWebTrust?attachmentId=207c5d90-2aaf-4807-8299-9d38b5d03591)
