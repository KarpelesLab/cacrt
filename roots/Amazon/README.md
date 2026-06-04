# Amazon

Amazon Trust Services LLC is a certificate authority operated by Amazon Web Services, Inc., headquartered in Seattle, Washington, USA. It issues publicly trusted TLS certificates primarily through AWS Certificate Manager (ACM) and participates in the Web PKI as a member of the CA/Browser Forum. All four roots in this folder are operated under the Amazon Trust Services brand; no distinct sub-brands or separate legal entities are merged here (the related Starfield Services Root CA – G2, acquired from Starfield Technologies, is tracked separately).

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| Amazon Root CA 1 | `ce5e74ef.0` | RSA 2048 | 2038-01-17 | `8E:CD:E6:88:4F:3D:87:B1:12:5B:A3:1A:C3:FC:B1:3D:70:16:DE:7F:57:CC:90:4F:E1:CB:97:C6:AE:98:19:6E` |
| Amazon Root CA 2 | `6d41d539.0` | RSA 4096 | 2040-05-26 | `1B:A5:B2:AA:8C:65:40:1A:82:96:01:18:F8:0B:EC:4F:62:30:4D:83:CE:C4:71:3A:19:C3:9C:01:1E:A4:6D:B4` |
| Amazon Root CA 3 | `8cb5ee0f.0` | ECC P-256 | 2040-05-26 | `18:CE:6C:FE:7B:F1:4E:60:B2:E3:47:B8:DF:E8:68:CB:31:D0:2E:BB:3A:DA:27:15:69:F5:03:43:B4:6D:B3:A4` |
| Amazon Root CA 4 | `de6d66f3.0` | ECC P-384 | 2040-05-26 | `E3:5D:28:41:9E:D0:20:25:CF:A6:90:38:CD:62:39:62:45:8D:A5:C6:95:FB:DE:A3:C2:2B:0B:FB:25:89:70:92` |

All four roots are self-signed and were created on 2015-05-26. All four were cross-signed by Starfield Services Root Certificate Authority – G2 to establish an immediate trust path while the new roots propagated into operating-system and browser trust stores.

## Rationale for inclusion

Amazon Trust Services roots were approved for inclusion in the Mozilla root store following a public discussion period (Mozilla Bugzilla [Bug 1172401](https://bugzilla.mozilla.org/show_bug.cgi?id=1172401), closed 2016). The roots are included in the Mozilla NSS store, the Chrome Root Store, the Microsoft Trusted Root Program, and the Apple Root Certificate Program; ACM-issued certificates are trusted by Chrome, Firefox, Safari, and Edge by default. The scope covers TLS server authentication and Extended Validation SSL; EV treatment was approved alongside root inclusion. Amazon Trust Services is a CA/Browser Forum member. [[amazontrust.com](https://www.amazontrust.com/)] [[AWS blog](https://aws.amazon.com/blogs/security/how-to-prepare-for-aws-move-to-its-own-certificate-authority/)]

## CA/Browser Forum compliance

Amazon Trust Services undergoes annual independent WebTrust audits covering:

- WebTrust Principles and Criteria for Certification Authorities (WTCA)
- WebTrust for Baseline Requirements (TLS)
- WebTrust for Extended Validation SSL
- WebTrust for Network Security
- WebTrust for Code Signing Baseline Requirements
- WebTrust for S/MIME

Audit reports (period-of-time) dating back to 2015 are publicly available in the ATS repository, with the most recent covering 2025. Audits were initially filed as Point-in-Time Readiness Assessments (EY as auditor) and transitioned to full period-of-time reports. The CP/CPS (currently v2.6 as a combined document) is publicly available and updated; historical versions are archived. All intermediate CA certificates are disclosed in CCADB. ATS logs all publicly trusted TLS certificates to Certificate Transparency logs (at least two logs per certificate) as required by Chrome policy since April 2018, with opt-out available per-certificate via the ACM API. [[repository](https://www.amazontrust.com/repository/)] [[CT blog post](https://aws.amazon.com/blogs/security/how-to-get-ready-for-certificate-transparency/)]

## Past non-compliance

Several publicly documented compliance incidents exist in Mozilla's CA compliance tracker. None resulted in distrust of the Amazon roots; all were resolved:

- **Bug [1743935](https://bugzilla.mozilla.org/show_bug.cgi?id=1743935) – Misissuance of subordinate CA per CPS (2021):** A subordinate CA certificate issued in 2015 (operated by DigiCert on ATS's behalf) had a `notAfter` date extending beyond its issuer, violating the ATS CPS. ATS had corrected the issuance software in November 2015 but failed to retroactively audit previously issued certificates. Three additional affected certificates from the same 2015 ceremony were identified. All four were revoked by December 8, 2021 (none had issued end-entity certificates, so there was no subscriber impact).

- **Bug [1743943](https://bugzilla.mozilla.org/show_bug.cgi?id=1743943) – Delayed revocation of subordinate CA (2021–2023):** One of the intermediates from the above ceremony (the "2040 intermediate") had been included in ATS's public repository and preloaded via CCADB. Because revoking within the BR-required 7-day window would have implicitly revoked over 24 million active end-entity certificates, ATS notified browser root program operators (Mozilla, Chrome, Apple, Microsoft, Cisco) and delayed revocation. Reviewers (Ryan Sleevi, Ryan Dickson) criticized the timeline and lack of revocation readiness. ATS migrated issuance to a pool of rotating short-lived intermediates, replaced affected certificates in phased waves, and ultimately revoked the certificates on May 24, 2023. Mozilla required the incident to be documented in ATS's annual audit report.

- **Bug [1713668](https://bugzilla.mozilla.org/show_bug.cgi?id=1713668) – ALV errors / undisclosed intermediates (2020–2021):** Two intermediate certificates created in October 2015 were not listed in ATS's audit reports, causing Audit Letter Validation (ALV) failures. ATS had incorrectly interpreted the scoping requirement as satisfied by one certificate per key pair. Mozilla reviewers noted the incident report failed to adequately explain how ATS missed a long-standing policy requirement. ATS revoked both certificates by June 30, 2021 and implemented monthly policy-review meetings.

- **Bug [1713978](https://bugzilla.mozilla.org/show_bug.cgi?id=1713978) – Forbidden/ambiguous domain validation method 3.2.2.4.6 (2021):** ATS's CP and CPS gave contradictory statements about use of BR domain validation method 3.2.2.4.6 (which was prohibited). Resolved by updated CP and CPS published July 23, 2021.

- **Bug [1713976](https://bugzilla.mozilla.org/show_bug.cgi?id=1713976) – CP/CPS did not specify key-compromise methods (2021):** Section 4.9.12 of the BR requires CPs/CPSes to specify methods parties may use to demonstrate private-key compromise; neither ATS document did so. Resolved in the same July 2021 update.

- **Bug [1746945](https://bugzilla.mozilla.org/show_bug.cgi?id=1746945) – Missing CAA check for test-website certificates (2021):** Self-reported by ATS: a CAA check was omitted for certificates issued for test websites on December 8, 2021. ATS noted it does not operate its own online intermediates for day-to-day issuance; DigiCert operates those intermediates.

- **Bug [1525710](https://bugzilla.mozilla.org/show_bug.cgi?id=1525710) – Test-revoked certificates with invalid validity period (2019):** When Ballot 193 shortened the maximum certificate validity to 825 days, ATS did not update the validity period or guardrail for test-revoked certificates, resulting in certificates with 39-month validity and incorrectly formatted EV subjects.

No distrust actions against any Amazon Trust Services root have been taken by any major root program as of the date of this writing. For a current view of open or resolved compliance bugs, see the [Mozilla Bugzilla CA compliance tracker](https://bugzilla.mozilla.org/buglist.cgi?product=CA%20Program&component=CA%20Certificate%20Compliance&status_whiteboard=AmazonTrust&query_format=advanced).

## Transparency

- **CP/CPS:** Combined CP/CPS v2.6 (and full version history back to May 2015) is publicly available at [amazontrust.com/repository](https://www.amazontrust.com/repository/). The document underwent a structural consolidation in July 2025, merging the previously separate CP and CPS.
- **CCADB:** Amazon Trust Services discloses all intermediate CA certificates (including externally operated ones) in the Common CA Database. ALV checks against CCADB are incorporated into ATS's monthly compliance review agenda (remediation adopted after Bug 1713668).
- **Incident reporting:** ATS self-reports compliance incidents to Mozilla's CA compliance tracker (Bugzilla) and includes material incidents in annual WebTrust audit reports as required by Mozilla policy.
- **Certificate Transparency:** All publicly trusted TLS certificates have been logged to at least two CT logs since April 24, 2018. SCTs are embedded in issued certificates. [[AWS CT blog post](https://aws.amazon.com/blogs/security/how-to-get-ready-for-certificate-transparency/)]
- **Audit reports:** WebTrust audit reports (WTCA, BR, EV, Network Security, Code Signing, S/MIME) are published at [amazontrust.com/repository](https://www.amazontrust.com/repository/) with reports covering 2015–2025.

## Sources

- [Amazon Trust Services – Repository (CP/CPS, audit reports, root certificates)](https://www.amazontrust.com/repository/)
- [Amazon Trust Services – Homepage](https://www.amazontrust.com/)
- [Mozilla Bugzilla Bug 1172401 – Add Amazon root certificates (inclusion request)](https://bugzilla.mozilla.org/show_bug.cgi?id=1172401)
- [Mozilla Bugzilla Bug 1743935 – Amazon Trust Services: Misissuance of Subordinate Per CPS](https://bugzilla.mozilla.org/show_bug.cgi?id=1743935)
- [Mozilla Bugzilla Bug 1743943 – Amazon Trust Services: Delayed Revocation of Subordinate CA](https://bugzilla.mozilla.org/show_bug.cgi?id=1743943)
- [Mozilla Bugzilla Bug 1713668 – Amazon Trust Services: ALV Errors](https://bugzilla.mozilla.org/show_bug.cgi?id=1713668)
- [Mozilla Bugzilla Bug 1713978 – Amazon Trust Services: Forbidden Domain Validation Method 3.2.2.4.6](https://bugzilla.mozilla.org/show_bug.cgi?id=1713978)
- [Mozilla Bugzilla Bug 1713976 – Amazon Trust Services: CP/CPS does not specify key compromise methods](https://bugzilla.mozilla.org/show_bug.cgi?id=1713976)
- [Mozilla Bugzilla Bug 1746945 – Amazon Trust Services: Missing CAA Check For Test Website Certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=1746945)
- [Mozilla Bugzilla Bug 1525710 – Amazon Trust Services: Test revoked certificates with invalid validity period](https://bugzilla.mozilla.org/show_bug.cgi?id=1525710)
- [mozilla.dev.security.policy – Amazon Root Inclusion Request](https://groups.google.com/g/mozilla.dev.security.policy/c/zZ5RHXCkpGM)
- [AWS Security Blog – How to Prepare for AWS's Move to Its Own Certificate Authority](https://aws.amazon.com/blogs/security/how-to-prepare-for-aws-move-to-its-own-certificate-authority/)
- [AWS Security Blog – Preparing for ACM Support of Certificate Transparency](https://aws.amazon.com/blogs/security/how-to-get-ready-for-certificate-transparency/)
- [AWS Security Blog – How to configure and verify ACM certificates with trust stores](https://aws.amazon.com/blogs/security/how-to-configure-and-verify-acm-certificates-with-trust-stores/)
- [TechTarget – What is Amazon Trust Services?](https://www.techtarget.com/searchaws/definition/Amazon-Trust-Services)
- [Chrome Root Program Policy, Version 1.8](https://googlechrome.github.io/chromerootprogram/)
- [CCADB – Included CA Certificate Report (Mozilla)](https://ccadb.my.salesforce-sites.com/mozilla/IncludedCACertificateReport)
