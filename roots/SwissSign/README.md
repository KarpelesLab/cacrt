# SwissSign

SwissSign AG is a Swiss certificate authority and trust service provider headquartered at Sägereistrasse 25, Glattbrugg ZH 8152, Switzerland. It is a wholly owned subsidiary of Swiss Post, following a consolidation in 2021 in which Swiss Post acquired SwissSign Group AG (itself a 2018 consortium of Swiss banks, insurers, and state enterprises) and a 2022 merger that folded SwissSign Group AG into SwissSign AG. SwissSign is the largest trust service provider in Switzerland and operates in the Web PKI as both a TLS and S/MIME certificate authority under the CA/Browser Forum Baseline Requirements.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| SwissSign RSA TLS Root CA 2022 - 1 | `9e654b62.0` | RSA 4096 | 2047-06-08 | `19:31:44:F4:31:E0:FD:DB:74:07:17:D4:DE:92:6A:57:11:33:88:4B:43:60:D3:0E:27:29:13:CB:E6:60:CE:41` |

## Rationale for inclusion

SwissSign RSA TLS Root CA 2022 - 1 is trusted for TLS server authentication (Websites) and was approved for EV by Mozilla in June 2025 following a six-week public discussion (CCADB Case 00001460) that drew no objections; NSS inclusion landed in NSS 3.114 / Firefox 142 ([Mozilla Bugzilla #1845047][bz1845047]). Microsoft included this root in its November 2023 deployment notice ([Microsoft Trusted Root Program, November 2023][ms2023nov]). The root was already in the Apple trust store prior to the Mozilla request ([SwissSign CCADB public discussion][ccadb-discuss]).

The 2022 roots were created to replace the aging SwissSign Gold CA – G2 and Silver CA – G2 roots (using outdated key material) and to enforce the CA/Browser Forum requirement for separation of TLS and S/MIME hierarchies.

## CA/Browser Forum compliance

SwissSign is audited under the ETSI framework by **TÜV TRUST IT GmbH / TÜV AUSTRIA CERT** (eIDAS Conformity Assessment Body). Applicable standards as of the September 2024 audit cycle:

- ETSI EN 319 411-1 V1.3.1 (policies: EVCP, OVCP, DVCP, NCP, LCP)
- ETSI EN 319 401 V2.3.1
- CA/Browser Forum TLS Baseline Requirements (BRG), EV Guidelines, Network and Certificate System Security Requirements
- CA/Browser Forum S/MIME Baseline Requirements (SMIME BRG)

The latest audit attestations (AA2024090401 standard, AA2024090403 TLS EV, AA2024090404 S/MIME BR) are published by TÜV TRUST IT ([TÜV TRUST IT – SwissSign][tuvit-swisssign]).

SwissSign's CPS commits to the Mozilla Root Store Policy, CCADB Policy, and CA/Browser Forum requirements "in their latest published versions" ([SwissSign CPS TLS][cps-tls]). Certificate Transparency is required for all TLS certificates; the CPS mandates submission to the required number of CT logs per IETF RFC 6962 during issuance. As of 2026-03-02 the CPS also requires successful DNSSEC validation to the IANA root trust anchor for domain validation.

CCADB disclosure is current; audit documents are linked from CCADB Case 00001460.

## Past non-compliance

SwissSign has a documented history of compliance incidents handled through Mozilla Bugzilla. No distrust action has been taken against any SwissSign root by any major root store program. The incidents below are drawn entirely from cited sources.

- **2017 – Non-BR-compliant certificate issuance** ([Bugzilla #1391066][bz1391066]): Several certificates were not revoked after non-compliance was identified; improper OU field handling and IDN encoding errors. Remediated in a software release on 2017-11-04; organizational management restructuring also announced.

- **2017 – Invalid DNSName in SAN** ([Bugzilla #1428877][bz1428877]): Follow-on to #1391066; SwissSign detected further invalid DNS name encoding issues via its post-issuance cablint checker.

- **2017 – Two certificates issued with same issuer and serial number** ([Bugzilla #1404403][bz1404403]).

- **2018 – Misissuance of intermediate certificates (incorrect organizationIdentifier)** ([Bugzilla #1506607][bz1506607]): Six non-production intermediate CAs were created with an incorrect organizationIdentifier due to human error during GUI data entry at a key ceremony. No leaf certificates were affected.

- **2018 – Undisclosed intermediate certificates** ([Bugzilla #1455132][bz1455132]): Five intermediate CA certificates issued in 2016–2017 were not disclosed in CCADB within the seven-day window required by Mozilla Root Store Policy §5.3. SwissSign disclosed all five the day after notification (2018-10-10). The delay was attributed to staff turnover disrupting the disclosure process.

- **2019 – CP/CPS certificate profile issue** ([Bugzilla #1558552][bz1558552]).

- **2020 – Failure to provide preliminary incident report within 24 hours** ([Bugzilla #1636141][bz1636141], [Bugzilla #1671113][bz1671113]): Two separate instances in 2020 where SwissSign failed to acknowledge a certificate problem report within the required 24-hour window. Remediated by introducing a dedicated mis-issuance email address monitored by a Manager on Call team and updating internal training.

- **2021 – Certificate with key length 4098 bit** ([Bugzilla #1691704][bz1691704]): A certificate was issued with a 4098-bit RSA key rather than a standard 4096-bit key.

- **2024 audit cycle non-conformities**: The September 2024 ETSI audit (AA2024090401, AA2024090403, AA2024090404) identified non-conformities including: absence of dual-control in the DSS, mis-issuance reports not routed exclusively through the dedicated mailbox, missing block-list filter for the commonName field, and S/MIME Mailbox-Validated certificates issued with unpermitted KeyUsage values. All major non-conformities were resolved before attestation issuance or addressed by a documented action plan ([TÜV TRUST IT audit attestations][tuvit-swisssign]).

A CCADB/Bugzilla search for SwissSign incidents: [Bugzilla CA Compliance – SwissSign][bz-search].

## Transparency

- **CP/CPS repository**: All certificate policies, CPSes, and certificate profiles are published at [repository.swisssign.com][repo] (CPS TLS, CP DV/OV/EV, CPR TLS, CPS S/MIME, TSPS).
- **CCADB**: SwissSign discloses all intermediate CAs in the Common CA Database; the current inclusion request is CCADB Case 00001460.
- **Certificate Transparency**: SwissSign submits all publicly trusted TLS certificates to CT logs as required by browser policies and the CA/Browser Forum BRG. CT support is described on the [SwissSign compatibility page][compat].
- **Incident self-reporting**: SwissSign uses [Bugzilla][bz-search] and posts incident reports to the mozilla.dev.security.policy list. A dedicated mis-issuance report address (certificatemisuse@swisssign.com) was introduced following the 2020 reporting-delay incidents.
- **Audit attestations**: Published by TÜV TRUST IT at [it-tuv.com][tuvit-swisssign]; also linked from CCADB.

## Sources

- [SwissSign website](https://www.swisssign.com/en/)
- [SwissSign certificate repository (CP/CPS documents)][repo]
- [SwissSign CPS TLS (current)][cps-tls]
- [SwissSign compatibility / root store page][compat]
- [Mozilla Bugzilla #1845047 – Add SwissSign RSA TLS Root CA 2022 - 1 and SwissSign RSA SMIME Root CA 2022 - 1][bz1845047]
- [CCADB public discussion – SwissSign CA Inclusion Request][ccadb-discuss]
- [Mozilla Bugzilla #343756 – Original SwissSign root inclusion (2006)][bz343756]
- [Mozilla Bugzilla #1391066 – Non-BR-Compliant Certificate Issuance][bz1391066]
- [Mozilla Bugzilla #1404403 – Two certs issued with same issuer and serial number][bz1404403]
- [Mozilla Bugzilla #1428877 – Invalid DNSName in SAN][bz1428877]
- [Mozilla Bugzilla #1506607 – Misissuance of Intermediate Certificates (organizationIdentifier)][bz1506607]
- [Mozilla Bugzilla #1455132 – Undisclosed Intermediate Certificates][bz1455132]
- [Mozilla Bugzilla #1558552 – CP/CPS certificate profile issue][bz1558552]
- [Mozilla Bugzilla #1636141 – Failure to provide preliminary report within 24 hours][bz1636141]
- [Mozilla Bugzilla #1671113 – Failure to provide preliminary report within 24 hours][bz1671113]
- [Mozilla Bugzilla #1691704 – Certificate with key length 4098 bit][bz1691704]
- [Mozilla Bugzilla CA compliance search – SwissSign][bz-search]
- [TÜV TRUST IT – SwissSign audit attestations (2024)][tuvit-swisssign]
- [Microsoft Trusted Root Program – November 2023 Deployment Notice][ms2023nov]
- [SwissSign new CA roots page][new-roots]
- [SwissSign EverybodyWiki / company background](https://en.everybodywiki.com/SwissSign)
- [SwissSign PKI Consortium member profile](https://pkic.org/members/swisssign/)

[repo]: https://repository.swisssign.com/
[cps-tls]: https://repository.swisssign.com/SwissSign_CPS_TLS.pdf
[compat]: https://www.swisssign.com/en/support/kompatibilitaet.html
[bz1845047]: https://bugzilla.mozilla.org/show_bug.cgi?id=1845047
[ccadb-discuss]: https://www.mail-archive.com/public@ccadb.org/msg00455.html
[bz343756]: https://bugzilla.mozilla.org/show_bug.cgi?id=343756
[bz1391066]: https://bugzilla.mozilla.org/show_bug.cgi?id=1391066
[bz1404403]: https://bugzilla.mozilla.org/show_bug.cgi?id=1404403
[bz1428877]: https://bugzilla.mozilla.org/show_bug.cgi?id=1428877
[bz1506607]: https://bugzilla.mozilla.org/show_bug.cgi?id=1506607
[bz1455132]: https://bugzilla.mozilla.org/show_bug.cgi?id=1455132
[bz1558552]: https://bugzilla.mozilla.org/show_bug.cgi?id=1558552
[bz1636141]: https://bugzilla.mozilla.org/show_bug.cgi?id=1636141
[bz1671113]: https://bugzilla.mozilla.org/show_bug.cgi?id=1671113
[bz1691704]: https://bugzilla.mozilla.org/show_bug.cgi?id=1691704
[bz-search]: https://bugzilla.mozilla.org/buglist.cgi?product=CA%20Program&component=CA%20Certificate%20Compliance&status_whiteboard=SwissSign&resolution=---
[tuvit-swisssign]: https://it-tuv.com/en/referenzen/swisssign-ag-swisssign-group-company-receives-trust-services-certification/
[ms2023nov]: https://learn.microsoft.com/en-us/security/trusted-root/2023/nov2023
[new-roots]: https://www.swisssign.com/en/support/ca-prod/new-roots-2021.html
