# GlobalSign

GlobalSign (legal entity: GMO GlobalSign, Inc.; Belgian subsidiary: GlobalSign nv-sa) is a commercial certificate authority founded in Belgium in 1996 and acquired by Japan's GMO Internet Group in 2007, with headquarters in Brussels, Belgium. It operates as a publicly-trusted CA in the Web PKI, issuing TLS/SSL, S/MIME, code signing, and document signing certificates worldwide. This folder merges roots issued under two closely related legal entities — "GlobalSign" (the operating brand) and "GlobalSign nv-sa" (the Belgian registered entity) — both under the same management and audit scope.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| GlobalSign Root CA - R3 | `062cdee6.0` | RSA 2048 | 2029-03-18 | `CB:B5:22:D7:B7:F1:27:AD:6A:01:13:86:5B:DF:1C:D4:10:2E:7D:07:59:AF:63:5A:7C:F4:72:0D:C9:63:C5:3B` |
| GlobalSign Root CA - R6 | `dc4d6a89.0` | RSA 4096 | 2034-12-10 | `2C:AB:EA:FE:37:D0:6C:A2:2A:BA:73:91:C0:03:3D:25:98:29:52:C4:53:64:73:49:76:3A:3A:B5:AD:6C:CF:69` |
| GlobalSign Root R46 | `002c0b4f.0` | RSA 4096 | 2046-03-20 | `4F:A3:12:6D:8D:3A:11:D1:C4:85:5A:4F:80:7C:BA:D6:CF:91:9D:3A:5A:88:B0:3B:EA:2C:63:72:D9:3C:40:C9` |
| GlobalSign ECC Root CA - R4 | `b0e59380.0` | ECC P-256 | 2038-01-19 | `B0:85:D7:0B:96:4F:19:1A:73:E4:AF:0D:54:AE:7A:0E:07:AA:FD:AF:9B:71:DD:08:62:13:8A:B7:32:5A:24:A2` |
| GlobalSign ECC Root CA - R5 | `1d3472b9.0` | ECC P-384 | 2038-01-19 | `17:9F:BC:14:8A:3D:D0:0F:D2:4E:A1:34:58:CC:43:BF:A7:F5:9C:81:82:D7:83:A5:13:F6:EB:EC:10:0C:89:24` |
| GlobalSign Root E46 | `feffd413.0` | ECC P-384 | 2046-03-20 | `CB:B9:C4:4D:84:B8:04:3E:10:50:EA:31:A6:9F:51:49:55:D7:BF:D2:E2:C6:B4:93:01:01:9A:D6:1D:9F:50:58` |

Note: R3 and R4/R5 are legacy multi-purpose roots undergoing phased retirement. Mozilla is removing the TLS trust bit from R3 in April 2027; Google Chrome will apply an SCTNotAfter constraint to multi-purpose roots effective September 13, 2026. R46 and E46 are the current single-purpose TLS roots. [[1]](https://support.globalsign.com/ssl/upcoming-changes-tls-roots-and-certificate-profiles)

## Rationale for inclusion

GlobalSign roots are included in all major root programs:

- **Mozilla NSS / Firefox**: R3 included since the early 2000s; R6 included in NSS 3.39 / Firefox 63 (approved 2018) [[2]](https://bugzilla.mozilla.org/show_bug.cgi?id=1390803); R46 and E46 included in NSS 3.63 / Firefox 88 (approved 2021-02-02) [[3]](https://bugzilla.mozilla.org/show_bug.cgi?id=1570724).
- **Microsoft Trusted Root Program**: GlobalSign roots are distributed in Windows trust stores across all supported Windows versions. [[4]](https://support.globalsign.com/ca-certificates/general-information/globalsign-root-ubiquity)
- **Apple**: Roots are embedded in macOS and iOS trust stores. [[4]](https://support.globalsign.com/ca-certificates/general-information/globalsign-root-ubiquity)
- **Google Chrome**: Roots appear in Chrome's trusted CA list. [[4]](https://support.globalsign.com/ca-certificates/general-information/globalsign-root-ubiquity)

GlobalSign is a founding member of the CA/Browser Forum and an active participant in its Server Certificate and S/MIME working groups. It became a Qualified Trust Service Provider (QTSP) under eIDAS in the EU in 2018 and was approved as the UK's first QTSP in 2021. [[5]](https://en.wikipedia.org/wiki/GlobalSign)

All roots included here are scoped for TLS server authentication (the R46 and E46 roots are explicitly single-purpose, limited to ServerAuth EV). The CCADB case for R46/E46 (Case #00000469) confirms inclusion status as "Complete/Included" with audit coverage through March 31, 2025. [[6]](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000469)

## CA/Browser Forum compliance

GlobalSign is audited annually by KPMG Advisory N.V. (Netherlands) under the WebTrust scheme. The most recent publicly available audit covers the period **April 1, 2024 to March 31, 2025** (report dated June 27, 2025) and encompasses:

- WebTrust for Certification Authorities (CA operations)
- WebTrust for Baseline Requirements (TLS BR)
- WebTrust for Extended Validation Guidelines (TLS EVG)

The 2024–2025 audit report records no deviations for the R46 and E46 roots. [[6]](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000469) [[7]](https://www.cpacanada.ca/api/getPDFWebTrust?attachmentId=f7ac4da2-5c12-49a8-9867-f41871d17061)

Policy documents (CP and CPS) follow RFC 3647 format and are publicly available at `https://www.globalsign.com/en/repository`. The current versions as of mid-2025 are CP v7.6 and CPS v10.6 (both dated July 15, 2025), with subsequent versions (v7.7/v10.7, November 2025) also published. [[8]](https://www.globalsign.com/en/repository)

GlobalSign supports Certificate Transparency (CT) via SCTs embedded in certificates and OCSP responses, and offers ACME-based certificate automation (including for internal domains as of July 2024). [[9]](https://www.globalsign.com/en/company/news-events/news/gmo-globalsign-adds-acme-support-internal-domain-certificate-issuance) All issued TLS certificates are logged in CT logs as required by the Baseline Requirements and browser root program policies.

## Past non-compliance

Several compliance incidents have been publicly documented in Mozilla Bugzilla. No full distrust or removal action has been taken against GlobalSign by any major root program.

**CAA mis-issuance — mixed wildcard/non-wildcard SAN (2017, Bug [1420766](https://bugzilla.mozilla.org/show_bug.cgi?id=1420766))**: GlobalSign mis-issued a certificate where CAA was checked for the wildcard SAN but not for the base domain also included in the SAN.

**Invalid State/Province values with US country code (2019, Bug [1575880](https://bugzilla.mozilla.org/show_bug.cgi?id=1575880))**: Certificates were issued with US country codes and State/Province values not matching any approved list. GlobalSign halted affected issuance and implemented a compliance-team approval mechanism for all new values.

**Misissuance of QWAC certificates (2020, Bug [1623356](https://bugzilla.mozilla.org/show_bug.cgi?id=1623356))**: A bug in the RA platform caused malformed certificates to be issued when order data was edited after initial submission.

**OCSP service outage — HTTP 530 (2020, Bug [1622505](https://bugzilla.mozilla.org/show_bug.cgi?id=1622505))**: OCSP responders returned errors under traffic load, attributed to malicious traffic and infrastructure capacity issues. Remediated by moving to dedicated hardware and failing over to a second data centre.

**Certificate issued with RSASSA-PSS public key (2020, Bug [1630870](https://bugzilla.mozilla.org/show_bug.cgi?id=1630870))**: EJBCA's CSR input validation accepted an RSASSA-PSS key as a valid RSA key, resulting in one non-compliant certificate.

**Domain validation random value used beyond 30-day limit (2020, Bug [1654544](https://bugzilla.mozilla.org/show_bug.cgi?id=1654544))**: 78 domains were validated using random values that had exceeded the 30-day maximum, resulting in 101 non-compliant certificates. All were revoked by 2020-07-17.

**Incorrect OCSP Delegated Responder Certificate (2020–2022, Bug [1649937](https://bugzilla.mozilla.org/show_bug.cgi?id=1649937))**: Multiple issuing CAs had the OCSP Signing EKU included in their certificates, constituting a dangerous delegated responder configuration. This was the most significant incident of the period; GlobalSign produced a specialised ISAE 3000 Type II audit covering April 22, 2020 through April 21, 2021, destroyed affected key material under auditor witness, and committed to quarterly CA rotation on its Atlas platform.

**Failure to revoke noncompliant ICA within 7 days (2020, Bug [1651447](https://bugzilla.mozilla.org/show_bug.cgi?id=1651447))**: Related to the OCSP EKU issue above; an intermediate CA was not revoked within the 7-day window required by the BRs.

**CRL with invalid signature algorithm (2022, Bug [1793441](https://bugzilla.mozilla.org/show_bug.cgi?id=1793441))**: A CRL was generated with an incorrect signature algorithm. Identified via Bugzilla report; corrected promptly.

**Certificate issued to FQDN with malformed CAA record (2022, Bug [1759854](https://bugzilla.mozilla.org/show_bug.cgi?id=1759854))**: CAA validation logic stripped characters after a semicolon, leading to incorrect acceptance of records in the format `globalsign.com; digicert.com`.

No distrust action by Mozilla, Chrome, Apple, or Microsoft has been taken against GlobalSign. For a current view of open and closed incidents, see the [Mozilla CA Incident Dashboard](https://wiki.mozilla.org/CA/Incident_Dashboard).

## Transparency

- **CP/CPS**: Publicly available at [https://www.globalsign.com/en/repository](https://www.globalsign.com/en/repository). Current versions: CP v7.6, CPS v10.6 (July 2025), with v7.7/v10.7 (November 2025) also published. Both follow RFC 3647 format. [[8]](https://www.globalsign.com/en/repository)
- **CCADB disclosure**: GlobalSign maintains records in the CCADB for all root and intermediate CAs, with current audit documentation and policy URLs. CCADB case for R46/E46 (Case #00000469) shows ongoing updates with audit reports through March 2025. [[6]](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000469)
- **Incident self-reporting**: Incidents above were disclosed in Mozilla Bugzilla. In the most significant incident (Bug 1649937), GlobalSign provided a specialised third-party ISAE 3000 audit as supplementary evidence.
- **Certificate Transparency**: All TLS certificates are logged to public CT logs per BR requirements; SCTs are delivered via certificate embedding or OCSP stapling. [[10]](https://www.globalsign.com/en/blog/what-is-certificate-transparency)

## Sources

- [1] [GlobalSign: Upcoming Changes to TLS Roots and Certificate Profiles](https://support.globalsign.com/ssl/upcoming-changes-tls-roots-and-certificate-profiles)
- [2] [Mozilla Bugzilla Bug 1390803 — Add GlobalSign Root CA - R6](https://bugzilla.mozilla.org/show_bug.cgi?id=1390803)
- [3] [Mozilla Bugzilla Bug 1570724 — Add GlobalSign Root Certificates R46/E46](https://bugzilla.mozilla.org/show_bug.cgi?id=1570724)
- [4] [GlobalSign Root Ubiquity](https://support.globalsign.com/ca-certificates/general-information/globalsign-root-ubiquity)
- [5] [GlobalSign — Wikipedia](https://en.wikipedia.org/wiki/GlobalSign)
- [6] [CCADB Case 00000469 — GlobalSign Root R46/E46 Inclusion](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000469)
- [7] [KPMG WebTrust Audit Report — GlobalSign EV SSL (April 2024–March 2025)](https://www.cpacanada.ca/api/getPDFWebTrust?attachmentId=f7ac4da2-5c12-49a8-9867-f41871d17061)
- [8] [GlobalSign Repository (CP, CPS, and PKI documents)](https://www.globalsign.com/en/repository)
- [9] [GlobalSign ACME Support for Internal Domain Certificate Issuance (2024)](https://www.globalsign.com/en/company/news-events/news/gmo-globalsign-adds-acme-support-internal-domain-certificate-issuance)
- [10] [GlobalSign: What is Certificate Transparency?](https://www.globalsign.com/en/blog/what-is-certificate-transparency)
- [11] [Mozilla Bugzilla Bug 1420766 — GlobalSign/AlphaSSL CAA Mis-Issuance](https://bugzilla.mozilla.org/show_bug.cgi?id=1420766)
- [12] [Mozilla Bugzilla Bug 1575880 — GlobalSign SSL Certificates with US country code and invalid State/Prov](https://bugzilla.mozilla.org/show_bug.cgi?id=1575880)
- [13] [Mozilla Bugzilla Bug 1623356 — GlobalSign: Misissuance of QWAC Certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=1623356)
- [14] [Mozilla Bugzilla Bug 1622505 — GlobalSign: OCSP Status HTTP 530](https://bugzilla.mozilla.org/show_bug.cgi?id=1622505)
- [15] [Mozilla Bugzilla Bug 1630870 — GlobalSign: Certificate issued with RSASSA-PSS public key](https://bugzilla.mozilla.org/show_bug.cgi?id=1630870)
- [16] [Mozilla Bugzilla Bug 1654544 — GlobalSign: Use of Domain Validation Random Value for more than 30 days](https://bugzilla.mozilla.org/show_bug.cgi?id=1654544)
- [17] [Mozilla Bugzilla Bug 1649937 — GlobalSign: Incorrect OCSP Delegated Responder Certificate](https://bugzilla.mozilla.org/show_bug.cgi?id=1649937)
- [18] [Mozilla Bugzilla Bug 1651447 — GlobalSign: Failure to revoke noncompliant ICA within 7 days](https://bugzilla.mozilla.org/show_bug.cgi?id=1651447)
- [19] [Mozilla Bugzilla Bug 1793441 — GlobalSign: CRL contains invalid signature algorithm](https://bugzilla.mozilla.org/show_bug.cgi?id=1793441)
- [20] [Mozilla Bugzilla Bug 1759854 — GlobalSign: Certificate issued to FQDN with malformed CAA](https://bugzilla.mozilla.org/show_bug.cgi?id=1759854)
- [21] [Mozilla CA Incident Dashboard](https://wiki.mozilla.org/CA/Incident_Dashboard)
- [22] [Public Discussion of GlobalSign R46/E46/R45/E45 Root Inclusion (mozilla.dev.security.policy)](https://groups.google.com/g/mozilla.dev.security.policy/c/Lq0WgPiQJPk)
