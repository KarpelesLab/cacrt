# Firmaprofesional

Firmaprofesional S.A. (NIF A62634068) is a Spanish commercial Certification Authority and Qualified Trust Service Provider (QTSP), founded in 2001 and headquartered in Barcelona (Passeig de Gracia 50). It issues OV and EV TLS certificates primarily to professional associations, businesses, and public-sector entities in Spain and Europe, operating a network of more than 70 Registration Authorities across Spain. In October 2022 Firmaprofesional was acquired by Logalty Group; the legal entity and CA operations continue under the Firmaprofesional S.A. name.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| Autoridad de Certificacion Firmaprofesional CIF A62634068 | `3bde41ac.0` | RSA 4096 | 2036-05-05 | `57:DE:05:83:EF:D2:B2:6E:03:61:DA:99:DA:9D:F4:64:8D:EF:7E:E8:44:1C:3B:72:8A:FA:9B:CD:E0:F9:B2:6A` |

## Rationale for inclusion

This root is included in the Mozilla NSS / Firefox root store (trust bits: Websites and Email), with EV treatment enabled under OID `2.23.140.1.1`. It was first approved for inclusion via [Bugzilla #1102143](https://bugzilla.mozilla.org/show_bug.cgi?id=1102143) and landed in the NSS December 2021 batch ([Bugzilla #1741930](https://bugzilla.mozilla.org/show_bug.cgi?id=1741930), NSS 3.74). EV enablement was tracked in [Bugzilla #1741932](https://bugzilla.mozilla.org/show_bug.cgi?id=1741932). Firmaprofesional has been present in Mozilla's root program in various generations since 2006 ([Bugzilla #342426](https://bugzilla.mozilla.org/show_bug.cgi?id=342426)).

Firmaprofesional is authorized as a QTSP under the EU eIDAS Regulation (Reg. 910/2014) and appears in the EU Trusted List (EUTL). Its scope covers TLS server authentication (OV and EV).

No explicit evidence was found confirming current inclusion in the Chrome Root Store, Apple Root Certificate Program, or Microsoft Trusted Root Program for this specific root at the time of writing; the CCADB public discussion in early 2024 was specifically for the newer *FIRMAPROFESIONAL CA ROOT-A WEB* root ([CCADB Case 00001044](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00001044), [Bugzilla #1785215](https://bugzilla.mozilla.org/show_bug.cgi?id=1785215)), which received no objections and was approved for Mozilla inclusion in NSS 3.101 / Firefox 128.

## CA/Browser Forum compliance

Firmaprofesional is audited annually under **ETSI EN 319 411** (covering both the Baseline Requirements and Extended Validation Guidelines). Current auditor as of 2023–2025 is **DEKRA Testing and Certification, S.A.U.**, accredited by ENAC (Spain's national accreditation body). Earlier audits (2021–2022) were conducted by AENOR INTERNACIONAL, S.A. (Unipersonal) under ETSI EN 319 411. Prior to the ETSI transition, audits were performed by Ernst & Young under WebTrust for CAs and WebTrust for EV criteria.

CP/CPS documents are publicly available at the [Firmaprofesional certification policies repository](https://www.firmaprofesional.com/certification-policies-and-practices/). The current CPS and Website Authentication CP are maintained in English and Spanish, with versioned archives going back to 2013. The CA discloses intermediate certificates and audit statements in the CCADB as required by Mozilla policy. Certificate Transparency (SCT) inclusion is required by browser policy for all publicly trusted TLS certificates; no Firmaprofesional-operated CT log was identified.

## Past non-compliance

The following compliance incidents are publicly documented on Mozilla Bugzilla. All listed bugs have been resolved; no distrust action has been taken against Firmaprofesional by Mozilla or other major root programs.

- **[Bug 1398240](https://bugzilla.mozilla.org/show_bug.cgi?id=1398240) — Non-BR-Compliant OCSP Responders (2017):** Several Firmaprofesional OCSP responders returned "good" for non-existent serial numbers, violating BR §4.9.10. The issue had been flagged as a low-severity finding in a 2014–2015 WebTrust audit. Affected sub-CAs (CA1, AA.PP) had already stopped issuing TLS certificates; remaining valid certificates were revoked and the sub-CAs were added to Mozilla's OneCRL. Resolved fixed.

- **[Bug 1412950](https://bugzilla.mozilla.org/show_bug.cgi?id=1412950) — Insufficient Audit Statements (2017):** A gap of approximately ten months (March 2016 – January 2017) was not covered by any audit period, violating BR §8.1 and Mozilla policy requiring contiguous annual audits. Root cause: Firmaprofesional's initial eIDAS point-in-time audit did not cover the full prior period. Resolved after submission of remediated audit statements.

- **[Bug 1455119](https://bugzilla.mozilla.org/show_bug.cgi?id=1455119) / [Bug 1464335](https://bugzilla.mozilla.org/show_bug.cgi?id=1464335) — Undisclosed Intermediate Certificates (2018):** Two technically-constrained subordinate CAs (including the SIGNE sub-CA) were not disclosed to the CCADB as required. Root cause: no internal procedure mandated CCADB disclosure upon issuance. Procedure was updated and certificates disclosed. Resolved fixed.

- **[Bug 1606380](https://bugzilla.mozilla.org/show_bug.cgi?id=1606380) — 2019 Audit Report Findings:** AENOR auditors identified that revocation requests received by phone or email were not consistently recorded within 24 hours, and that entropy in SSL certificates issued by "AC Firmaprofesional - INFRASTRUCTURE" was 63 bits (below the 64-bit minimum). Both findings were remediated.

- **[Bug 1649943](https://bugzilla.mozilla.org/show_bug.cgi?id=1649943) — Incorrect OCSP Delegated Responder Certificate (2020):** Four intermediate CAs carried the OCSP Signing EKU without the required `id-pkix-ocsp-nocheck` extension, allowing them to technically act as OCSP delegated responders for the root. Firmaprofesional initially argued the risk was limited because all CAs were under its exclusive control; community feedback clarified this was still non-compliant. Remediation included reissuing all four intermediates with new key pairs, a key destruction ceremony for the affected keys, and addition to OneCRL. Resolved fixed; follow-on revocation timeline tracked in [Bug 1651637](https://bugzilla.mozilla.org/show_bug.cgi?id=1651637).

- **[Bug 1651637](https://bugzilla.mozilla.org/show_bug.cgi?id=1651637) — Failure to Revoke ICAs Within 7 Days (2020):** A dependency on the Spanish Trusted Services List (TSL) constrained Firmaprofesional's ability to revoke and replace intermediates within the BR-required 7-day window. Resolved as part of the broader OCSP EKU remediation above.

- **[Bug 1771715](https://bugzilla.mozilla.org/show_bug.cgi?id=1771715) — StateOrProvince Field (2022):** Approximately 80 TLS certificates contained invalid or inaccurate `stateOrProvince` values due to the absence of a technical validation control. The CA revoked all affected certificates and removed the field from future TLS certificates. Resolved fixed.

- **[Bug 1925293](https://bugzilla.mozilla.org/show_bug.cgi?id=1925293) — Incorrect Test Website URL in CCADB (2024–2025):** The "Test Website - Revoked" URL in the CCADB was misconfigured to point to an expired certificate because the same certificate was used for both the "Revoked" and "Expired" test URLs. Lower-severity administrative issue; resolved.

No distrust action or removal from the Mozilla root store has been taken against Firmaprofesional. A search of publicly known incidents is available via [Bugzilla product:NSS component:CA Certificates](https://bugzilla.mozilla.org/buglist.cgi?product=NSS&component=CA%20Certificates&resolution=---&short_desc=Firmaprofesional&short_desc_type=allwordssubstr).

## Transparency

- **CP/CPS:** Publicly available in English and Spanish at [https://www.firmaprofesional.com/certification-policies-and-practices/](https://www.firmaprofesional.com/certification-policies-and-practices/), with versioned archives. Current CPS effective 2026-02-26.
- **CCADB disclosure:** Firmaprofesional discloses intermediate certificates and audit statements in the CCADB as required. CCADB record available via [Case 00001044](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00001044).
- **Incident self-reporting:** Firmaprofesional has filed and responded to incident reports on Mozilla Bugzilla for the incidents listed above.
- **Certificate Transparency:** As a publicly trusted CA, Firmaprofesional-issued TLS certificates are required to include Signed Certificate Timestamps (SCTs) per browser policy (Chrome since 2018, Firefox since version 135). No Firmaprofesional-operated CT log was identified; certificates are submitted to third-party logs.

## Sources

- [Bugzilla #342426 — Add Firmaprofesional root CA certificate (Spain)](https://bugzilla.mozilla.org/show_bug.cgi?id=342426)
- [Bugzilla #521439 — Add renewed Firmaprofesional root CA cert](https://bugzilla.mozilla.org/show_bug.cgi?id=521439)
- [Bugzilla #1102143 — Add Renewed Autoridad de Certificacion Firmaprofesional root certificate](https://bugzilla.mozilla.org/show_bug.cgi?id=1102143)
- [Bugzilla #1398240 — Firmaprofesional: Non-BR-Compliant OCSP Responders](https://bugzilla.mozilla.org/show_bug.cgi?id=1398240)
- [Bugzilla #1412950 — Firmaprofesional: Insufficient Audit Statements](https://bugzilla.mozilla.org/show_bug.cgi?id=1412950)
- [Bugzilla #1455119 — Firmaprofesional: Undisclosed Intermediate certificate](https://bugzilla.mozilla.org/show_bug.cgi?id=1455119)
- [Bugzilla #1464335 — Firmaprofesional: Undisclosed Intermediate certificate SIGNE](https://bugzilla.mozilla.org/show_bug.cgi?id=1464335)
- [Bugzilla #1606380 — Firmaprofesional: 2019 Audit Report Findings](https://bugzilla.mozilla.org/show_bug.cgi?id=1606380)
- [Bugzilla #1649943 — Firmaprofesional: Incorrect OCSP Delegated Responder Certificate](https://bugzilla.mozilla.org/show_bug.cgi?id=1649943)
- [Bugzilla #1651637 — Firmaprofesional: Failure to revoke ICAs within 7 days: OCSP EKU](https://bugzilla.mozilla.org/show_bug.cgi?id=1651637)
- [Bugzilla #1741930 — Add renewed Autoridad de Certificacion Firmaprofesional CIF A62634068 root certificate to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=1741930)
- [Bugzilla #1741932 — Enable EV Treatment for the renewed Autoridad de Certificacion Firmaprofesional CIF A62634068 root cert](https://bugzilla.mozilla.org/show_bug.cgi?id=1741932)
- [Bugzilla #1771715 — Firmaprofesional: 2022 - StateOrProvince field](https://bugzilla.mozilla.org/show_bug.cgi?id=1771715)
- [Bugzilla #1785215 — Add "FIRMAPROFESIONAL CA ROOT-A WEB" Root Certificate](https://bugzilla.mozilla.org/show_bug.cgi?id=1785215)
- [Bugzilla #1925293 — Firmaprofesional: Incorrect publication of information for "Test Website - Revoked" URL in the CCADB](https://bugzilla.mozilla.org/show_bug.cgi?id=1925293)
- [CCADB Case 00001044 — Firmaprofesional CA ROOT-A WEB inclusion](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00001044)
- [CCADB Public Discussion of Firmaprofesional CA Inclusion Request](https://groups.google.com/a/ccadb.org/g/public/c/3TXrvZC0isw)
- [Mozilla dev-security-policy: Approval of Firmaprofesional CA Root-A Web](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/1KJZrE4oDV8)
- [Firmaprofesional certification policies and practices repository](https://www.firmaprofesional.com/certification-policies-and-practices/)
- [Logalty acquisition of Firmaprofesional (October 2022)](https://www.logalty.com/en/blog/2022/10/25/logaltys-identification-and-qualified-electronic-signature-solutions-are-boosted-by-its-acquisition-of-firmaprofesional/)
- [Firmaprofesional — PKI Consortium member profile](https://pkic.org/members/firmaprofesional/)
