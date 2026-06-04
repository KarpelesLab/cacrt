# IdenTrust

IdenTrust (legally IdenTrust Services, LLC) is a United States-based commercial certificate authority headquartered in Salt Lake City, Utah. Founded in 1999 as a banking consortium (originally named Identrus), it was acquired by ASSA ABLOY via its HID Global subsidiary in 2014 and now operates as "IdenTrust – Part of HID Global." IdenTrust operates publicly trusted TLS roots under its TrustID and IdenTrust Global Common (IGC) programs, and is historically notable for having cross-signed Let's Encrypt's ISRG Root X1 intermediate via the now-expired DST Root CA X3.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| IdenTrust Commercial Root CA 1 | `ef954a4e.0` | RSA 4096 | 2034-01-16 | `5D:56:49:9B:E4:D2:E0:8B:CF:CA:D0:8A:3E:38:72:3D:50:50:3B:DE:70:69:48:E4:2F:55:60:30:19:E5:28:AE` |
| IdenTrust Public Sector Root CA 1 | `1e08bfd1.0` | RSA 4096 | 2034-01-16 | `30:D0:89:5A:9A:44:8A:26:20:91:63:55:22:D1:F5:20:10:B5:86:7A:CA:E1:2C:78:EF:95:8F:D4:F4:38:9F:2F` |

Both roots are self-signed (subject equals issuer), issued January 16, 2014, with a 20-year validity period.

**Note:** IdenTrust's DST Root CA X3 (expired 2021-09-30) is not included here as it is no longer valid. That root was used to cross-sign Let's Encrypt's ISRG Root X1 intermediate to bootstrap trust on older devices; the cross-signed chain arrangement ended when DST Root CA X3 expired. ([Let's Encrypt documentation](https://letsencrypt.org/docs/dst-root-ca-x3-expiration-september-2021/))

## Rationale for inclusion

Both roots are included in all four major root programs:

- **Mozilla NSS / Firefox:** Approved via [Bugzilla #1037590](https://bugzilla.mozilla.org/show_bug.cgi?id=1037590) (websites + email trust bits), with the underlying NSS changes tracked in [Bug #1118020](https://bugzilla.mozilla.org/show_bug.cgi?id=1118020). EV treatment for the Commercial Root CA 1 was subsequently enabled per [Bug #1658596](https://bugzilla.mozilla.org/show_bug.cgi?id=1658596).
- **Microsoft:** Included in the Microsoft Trusted Root Certificate Program.
- **Apple:** Included in the Apple Root Certificate Program.
- **Chrome Root Store:** Included (confirmed via [Cloudflare Radar CT data](https://radar.cloudflare.com/certificate-transparency/ca/5D56499BE4D2E08BCFCAD08A3E38723D50503BDE706948E42F55603019E528AE)).

The Commercial Root CA 1 is scoped to TLS server authentication (websites) and S/MIME (email). The Public Sector Root CA 1 serves the same trust purposes with a focus on U.S. public sector issuance.

## CA/Browser Forum compliance

IdenTrust is audited annually by an accredited WebTrust auditor (Schellman & Company, LLC has conducted audits in recent years). Audit scope covers:

- WebTrust for CAs (Principles and Criteria for Certification Authorities)
- WebTrust for CAs – SSL Baseline with Network Security (CA/Browser Forum Baseline Requirements)
- WebTrust for CAs – Extended Validation SSL

The governing document for publicly trusted TLS certificates is the **TrustID TLS CP/CPS**, currently at version 5.0.2, available at `https://www.identrust.com/sites/default/files/resources/identrust_trustid_cpcps_tls-5.0.2.pdf`. The CP/CPS states that CA/Browser Forum Baseline Requirements take precedence in the event of conflict, and IdenTrust commits to an annual CP/CPS review.

IdenTrust discloses root and intermediate CA certificates in the CCADB. Its TLS certificates embed Signed Certificate Timestamps (SCTs) to satisfy Certificate Transparency requirements; CT compliance is documented in the TLS CP/CPS. ([IdenTrust TLS CP/CPS v5.0.2](https://www.identrust.com/sites/default/files/resources/identrust_trustid_cpcps_tls-5.0.2.pdf))

An IdenTrust WebTrust audit report covering the period July 1, 2022 through June 30, 2023 was submitted to the CCADB public list; see [CCADB public discussion thread](https://groups.google.com/a/ccadb.org/g/public/c/4h4w1VObR-A).

## Past non-compliance

IdenTrust has a documented record of compliance incidents in Mozilla Bugzilla and on the mozilla.dev.security.policy forum. All incidents listed below are sourced from public Bugzilla records; no distrust of either root has occurred.

| Year | Incident | Bugzilla |
|---|---|---|
| 2017 | Non-BR-compliant certificate issuance (pathLenConstraint with CA:False) | [#1391000](https://bugzilla.mozilla.org/show_bug.cgi?id=1391000) |
| 2018 | Improper encoding of wildcard certificates; pre-issuance linting not in place; slow revocation (1 week delay) | [#1446121](https://bugzilla.mozilla.org/show_bug.cgi?id=1446121) |
| 2019 | Missing SHA-256 thumbprints for intermediate CAs in annual WebTrust audit reports; Mozilla directed IdenTrust to revoke, seek OneCRL addition, or obtain corrected auditor reports | [#1588213](https://bugzilla.mozilla.org/show_bug.cgi?id=1588213) |
| 2020 | Incorrect subject details (DBA name) in EV SSL certificate for HydrantId sub-CA | [#1635279](https://bugzilla.mozilla.org/show_bug.cgi?id=1635279) |
| 2020 | Two certificates exceeding 398-day validity by 1 second due to inclusive-period interpretation | [#1663080](https://bugzilla.mozilla.org/show_bug.cgi?id=1663080) |
| 2020 | DNS/OCSP service degradation impacting ~25% of validation traffic | [#1677239](https://bugzilla.mozilla.org/show_bug.cgi?id=1677239) |
| 2021 | OV SSL certificates with "Not Applicable" in stateOrProvinceName field; incident report criticised for insufficient depth | [#1718552](https://bugzilla.mozilla.org/show_bug.cgi?id=1718552) |
| 2021 | 124 EV certificates issued without re-validating vetting documentation after 398 days (stale vetting via API path); delayed filing | [#1734917](https://bugzilla.mozilla.org/show_bug.cgi?id=1734917) |
| 2022 | OCSP responder returning "Unauthorized" for 3 valid ICA certificates due to missed post-issuance configuration step | [#1758213](https://bugzilla.mozilla.org/show_bug.cgi?id=1758213) |
| 2023 | Inaccurate CRL details for 8 issuing CAs in CCADB; incorrect URL source used during data entry | [#1818833](https://bugzilla.mozilla.org/show_bug.cgi?id=1818833) |
| 2024 | Temporary errors in test website certificates for Public Sector root (wrong chain, expired revoked certificate) | [#1883792](https://bugzilla.mozilla.org/show_bug.cgi?id=1883792) |

A recurring theme in reviewer commentary is that incidents were frequently surfaced by external reporters (community tooling such as ZLint, third parties such as Entrust and Andrew Ayer) rather than IdenTrust's own monitoring, and that some incident reports were criticised for lacking sufficient root-cause analysis and for delayed filing. IdenTrust's stated remediations have progressively emphasized pre-issuance linting, automation, and expanded compliance review processes.

IdenTrust's initial application to enable EV treatment in Firefox (first filed as [Bug #1339292](https://bugzilla.mozilla.org/show_bug.cgi?id=1339292)) was recommended for denial by Mozilla's reviewer due to an unconstrained sub-CA outside EV audit scope and outstanding compliance issues; the request was later re-filed and ultimately approved ([Bug #1658596](https://bugzilla.mozilla.org/show_bug.cgi?id=1658596)) after the issues were addressed.

No formal distrust action against either root in this folder has been recorded in Mozilla, Chrome, Apple, or Microsoft root programs as of the date of this file.

## Transparency

- **CP/CPS:** IdenTrust publishes CP and CPS documents publicly at `https://www.identrust.com/support/documents`. The governing TLS document is the TrustID TLS CP/CPS; the IGC program is governed by the IGC CPS (current version: 1.5.6, dated 2023-03-31). ([IGC CPS v1.5.6](https://www.identrust.com/sites/default/files/resources/IGC_CPS_v1.5.6_2023_0331_1.pdf))
- **CCADB disclosure:** IdenTrust discloses root and intermediate CA certificates, audit statements, and CRL/OCSP URLs in the CCADB, as required by Mozilla Root Store Policy and CCADB Policy. ([CCADB Policy](https://www.ccadb.org/policy))
- **Incident reporting:** IdenTrust files incident reports in Mozilla Bugzilla; the record above reflects public filings. Timeliness of some filings has been questioned by Mozilla reviewers.
- **Certificate Transparency:** IdenTrust's TLS CP/CPS (v5.0.2) documents CT compliance; issued TLS certificates embed SCTs from qualified CT logs as required by Chrome, Firefox (from v135), and Apple CT policies.

## Sources

- [IdenTrust official website](https://www.identrust.com/)
- [IdenTrust – HID Global product page](https://www.hidglobal.com/identrust)
- [IdenTrust Wikipedia article](https://en.wikipedia.org/wiki/IdenTrust)
- [Mozilla Bugzilla #1037590 – Add Renewal IdenTrust root certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=1037590)
- [Mozilla Bugzilla #1118020 – Add 2 IdenTrust renewal root certificates to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=1118020)
- [Mozilla Bugzilla #1339292 – Enable EV for IdenTrust Commercial Root CA 1 (first attempt)](https://bugzilla.mozilla.org/show_bug.cgi?id=1339292)
- [Mozilla Bugzilla #1551703 – Enable EV for IdenTrust Commercial Root CA 1 (second attempt)](https://bugzilla.mozilla.org/show_bug.cgi?id=1551703)
- [Mozilla Bugzilla #1658596 – Enable EV Treatment for IdenTrust Commercial Root CA 1](https://bugzilla.mozilla.org/show_bug.cgi?id=1658596)
- [Mozilla Bugzilla #1391000 – IdenTrust: Non-BR-Compliant Certificate Issuance](https://bugzilla.mozilla.org/show_bug.cgi?id=1391000)
- [Mozilla Bugzilla #1446121 – IdenTrust: Improper encoding of wildcard certificate](https://bugzilla.mozilla.org/show_bug.cgi?id=1446121)
- [Mozilla Bugzilla #1588213 – IdenTrust: Missing Thumbprints for Intermediate CA certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=1588213)
- [Mozilla Bugzilla #1635279 – IdenTrust: Incorrect Subject Details for HydrantId](https://bugzilla.mozilla.org/show_bug.cgi?id=1635279)
- [Mozilla Bugzilla #1663080 – IdenTrust: Issuance of certificates greater than 398 days](https://bugzilla.mozilla.org/show_bug.cgi?id=1663080)
- [Mozilla Bugzilla #1677239 – IdenTrust: Service Degradation](https://bugzilla.mozilla.org/show_bug.cgi?id=1677239)
- [Mozilla Bugzilla #1718552 – IdenTrust: Certificates with Invalid values for stateOrProvinceName](https://bugzilla.mozilla.org/show_bug.cgi?id=1718552)
- [Mozilla Bugzilla #1734917 – IdenTrust: Mis-Issued EV Certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=1734917)
- [Mozilla Bugzilla #1758213 – IdenTrust: Failure to provide OCSP responses for valid ICA certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=1758213)
- [Mozilla Bugzilla #1818833 – IdenTrust: Inaccurate CRL Details in CCADB](https://bugzilla.mozilla.org/show_bug.cgi?id=1818833)
- [Mozilla Bugzilla #1883792 – IdenTrust: Temporary Errors in Test Website Certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=1883792)
- [IdenTrust TrustID TLS CP/CPS v5.0.2](https://www.identrust.com/sites/default/files/resources/identrust_trustid_cpcps_tls-5.0.2.pdf)
- [IdenTrust Global Common CPS v1.5.6](https://www.identrust.com/sites/default/files/resources/IGC_CPS_v1.5.6_2023_0331_1.pdf)
- [Let's Encrypt: DST Root CA X3 Expiration (September 2021)](https://letsencrypt.org/docs/dst-root-ca-x3-expiration-september-2021/)
- [CCADB Policy](https://www.ccadb.org/policy)
- [CCADB public list: IdenTrust WebTrust Audit Report 2022–2023](https://groups.google.com/a/ccadb.org/g/public/c/4h4w1VObR-A)
- [mozilla.dev.security.policy: Identrust Commercial Root CA 1 EV Request](https://groups.google.com/g/mozilla.dev.security.policy/c/fTeHAGGTBqg/m/l51Nt5ijAgAJ)
- [IdenTrust CA Certificate Compatibility page](https://www.identrust.com/ca-certificate-compatibility)
