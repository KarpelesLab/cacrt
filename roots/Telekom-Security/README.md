# Telekom Security

Deutsche Telekom Security GmbH (Telekom Security) is a wholly-owned subsidiary of Deutsche Telekom AG, headquartered in Bonn, Germany, with its Trust Center operations at Netphen (Untere Industriestrasse 20). It operates multiple public and qualified PKIs as a Trust Service Provider under eIDAS and serves as the Web PKI CA for the Deutsche Telekom Group. This folder consolidates roots from two legal entities: Deutsche Telekom Security GmbH (the current operator, established 2020) and T-Systems Enterprise Services GmbH (former operator of the "T-TeleSec GlobalRoot" roots, which transferred to Deutsche Telekom Security GmbH in 2020).

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| Telekom Security TLS ECC Root 2020 | `ddcda989.0` | ECC P-384 | 2045-08-25 | `57:8A:F4:DE:D0:85:3F:4E:59:98:DB:4A:EA:F9:CB:EA:8D:94:5F:60:B6:20:A3:8D:1A:3C:13:B2:BC:7B:A8:E1` |
| Telekom Security TLS RSA Root 2023 | `7fa05551.0` | RSA 4096 | 2048-03-27 | `EF:C6:5C:AD:BB:59:AD:B6:EF:E8:4D:A2:23:11:B3:56:24:B7:1B:3B:1E:A0:DA:8B:66:55:17:4E:C8:97:86:46` |
| T-TeleSec GlobalRoot Class 2 | `1e09d511.0` | RSA 2048 | 2033-10-01 | `91:E2:F5:78:8D:58:10:EB:A7:BA:58:73:7D:E1:54:8A:8E:CA:CD:01:45:98:BC:0B:14:3E:04:1B:17:05:25:52` |
| T-TeleSec GlobalRoot Class 3 | `5443e9e3.0` | RSA 2048 | 2033-10-01 | `FD:73:DA:D3:1C:64:4F:F1:B4:3B:EF:0C:CD:DA:96:71:0B:9C:D9:87:5E:CA:7E:31:70:7A:F3:E9:6D:52:2B:BD` |

## Rationale for inclusion

All four roots are trusted for TLS server authentication (serverAuth EKU) and are present in the Mozilla, Apple, and Microsoft root stores. The two T-TeleSec GlobalRoot certificates have been trusted since Firefox 23/29 (Class 3 and Class 2, respectively) and across Apple OS X 10.9+ and Microsoft Windows Vista+. The two newer Telekom Security roots (ECC Root 2020 and RSA Root 2023) were approved for inclusion in Mozilla's root store in January 2024 following a six-week CCADB public discussion that closed December 13, 2023. ([Mozilla Bugzilla #1820592][bz1820592]; [CCADB public discussion][ccadb-discuss])

T-TeleSec GlobalRoot Class 3 is additionally EV-enabled in Mozilla and Microsoft root stores (EV Policy OID `1.3.6.1.4.1.7879.13.24.1`). ([Mozilla Bugzilla #760313][bz760313])

Telekom Security is a qualified trust service provider under EU Regulation No. 910/2014 (eIDAS), with qualified status granted for website authentication (Server.ID) and electronic signatures (Qualified.ID).

## CA/Browser Forum compliance

Telekom Security is audited under the ETSI framework, not WebTrust. Audits are conducted by TÜV Informationstechnik GmbH against ETSI EN 319 411-1 and ETSI EN 319 411-2. The audit submitted for Mozilla inclusion covered April 8, 2022 – April 7, 2023, with the audit letter dated June 21, 2023. ([CCADB public discussion][ccadb-discuss])

The CA publishes a Certificate Policy (CP) and a public Certification Practice Statement (CPS Public), both versioned and maintained in the PKI Repository at telesec.de. As of 2026, CP v9.00 and CPS Public v12.00 are current. The CP/CPS commits to CA/Browser Forum Baseline Requirements and ETSI EN 319 401/411 for all publicly trusted TLS certificates. ([PKI Repository][pki-repo]; [Telekom Security CP v9.00][cp-latest])

Domain-validated TLS certificates are issued via ACME (RFC 8555) through the Public Certificate Service Platform (PCSP), and ACME-based automated revocation is also supported. ([Server.ID DV product page][serverid-dv])

All publicly trusted TLS certificates are submitted to Certificate Transparency logs per CA/Browser Forum policy; the CPS Public explicitly states that pre-certificates are submitted to a sufficient number of CT logs before final issuance. ([Telekom Security CPS Public][cps-public])

CCADB disclosure is required and implemented: new CA certificates are published in the CCADB within 7 days of issuance. Test websites (valid, expired, revoked) are operated for all TLS-issuing roots. ([CPS Public][cps-public])

## Past non-compliance

The following incidents are publicly documented in Mozilla Bugzilla and CCADB. No distrust action has been taken against any Telekom Security root to date.

**1. Undisclosed intermediate CA (2018) — [Bug #1455137][bz1455137]**
T-Systems failed to disclose the intermediate "Deutsche Telekom AG Issuing CA 01" (issued July 2016, active from November 2016) in the CCADB for approximately 18 months, violating Mozilla Root Store Policy §5.3. The CA misunderstood that disclosure required completed audit results. Remediation included staff replacement, dedicated training, and a dual-control (four-eyes) principle for CCADB changes. Mozilla reviewers noted residual concern that systemic issues may not have been fully resolved.

**2. Improper use of domain validation method (2023) — [Bug #1825780][bz1825780]**
Telekom Security used validation directly with DENIC (the .de ccTLD registry) rather than a CA/Browser Forum-approved domain validation method. This was the single incident listed in the CCADB inclusion request. A community reviewer found the incident response adversarial and lacking detail, and raised concerns about the CA's understanding of Baseline Requirements.

**3. CRL "unrevocation" during maintenance (2020) — [Bug #1655698][bz1655698]**
A software defect during a planned maintenance window caused an automated script to delete active customer accounts, triggering an unplanned CRL issuance that incorrectly listed 907 valid certificates from "TeleSec ServerPass Class 2 CA" as revoked. The database was then restored from backup, which removed the incorrect revocation entries — technically constituting an "unrevocation." Reviewers drew comparisons to DigiNotar and India CCA incidents. All affected certificates were ultimately revoked by October 1, 2020. The CA acknowledged the decision to unrevoke was made without compliance team involvement and added compliance staffing.

**4. Key encipherment in ECC TLS certificates (2021) — [Bug #1703528][bz1703528]**
Two ECC SAN TLS certificates were mis-issued with Key Usage `keyEncipherment`, caused by a template misconfiguration introduced during a CT log extension change. Both were revoked within 24 hours.

**5. basicConstraints not marked critical and delayed revocation (2024) — [Bug #1875820][bz1875820] / [Bug #1877388][bz1877388]**
TLS certificates were issued without `basicConstraints` marked as critical, violating Baseline Requirements. 336 certificates (approximately 41% of those affected) were not revoked within the required 5-day window; affected subscribers included a Belgian toll system, banking IT, Bavarian police IT, and municipal data centers. The CA cited "critical infrastructure" as justification for delayed revocation. Mozilla and Chrome root program representatives rejected this rationale, noting it violated BR section 4.9.1.1 and was not an acceptable exception. A Chrome Root Program representative warned that repeat non-compliance feeds ongoing evaluation that may result in removal from the Chrome Root Store. All affected certificates were reported revoked by June 2, 2024.

## Transparency

- **CP/CPS**: Published at the telesec.de PKI Repository. The CPS Public is the authoritative document for all publicly trusted TLS certificates. Both German and English versions are maintained at the same version number; the German version is authoritative in case of dispute. ([PKI Repository][pki-repo])
- **CCADB**: Telekom Security is enrolled in the CCADB (case 00001269). New intermediate CAs are disclosed within 7 days of issuance, as required by CCADB policy and the CPS Public. ([CCADB public discussion][ccadb-discuss])
- **Incident self-reporting**: Incidents are tracked and responded to in Mozilla Bugzilla. Incident reports for the 2024 basicConstraints misissuance and delayed revocation were scrutinized for adequacy; reviewers found responses in some cases insufficiently detailed. ([Bug #1877388][bz1877388])
- **Certificate Transparency**: All publicly trusted TLS certificates include SCTs from at least two CT logs before issuance, consistent with CA/Browser Forum Baseline Requirements and root program policies. ([CPS Public][cps-public])

## Sources

- [Mozilla Bugzilla #1820592 — Add Telekom Security Root Certificates][bz1820592]
- [Mozilla Bugzilla #935687 — Add T-TeleSec GlobalRoot Class 2 to NSS][bz935687]
- [Mozilla Bugzilla #760297 — Add T-TeleSec GlobalRoot Class 3 to NSS][bz760297]
- [Mozilla Bugzilla #760313 — Enable T-TeleSec GlobalRoot Class 3 for EV][bz760313]
- [Mozilla Bugzilla #1455137 — T-Systems: Undisclosed Intermediate Certificate][bz1455137]
- [Mozilla Bugzilla #1825780 — Telekom Security: Improper domain validation][bz1825780]
- [Mozilla Bugzilla #1655698 — Telekom Security: CRL contained unrevoked certificates][bz1655698]
- [Mozilla Bugzilla #1703528 — Telekom Security: Key Encipherment in ECC TLS certificates][bz1703528]
- [Mozilla Bugzilla #1875820 — Telekom Security: TLS certificates with basicConstraints not marked critical][bz1875820]
- [Mozilla Bugzilla #1877388 — Telekom Security: Revocation delay for TLS certificates][bz1877388]
- [CCADB Public Discussion — Deutsche Telekom Security CA Inclusion Request (mail-archive.com)][ccadb-discuss]
- [CCADB Public Discussion — Google Groups][ccadb-google]
- [Telekom Security PKI Repository (telesec.de)][pki-repo]
- [Telekom Security CP v9.00 (EN/DE)][cp-latest]
- [Telekom Security CPS Public v12.00 (EN)][cps-public]
- [Telekom Security Root Program Overview][root-program]
- [Telekom Security Root Certificates page][root-certs]
- [Telekom Security Root Compatibility page][root-compat]
- [Server.ID DV (ACME) product page][serverid-dv]
- [Corporate PKI CP/CPS page][corp-pki]

[bz1820592]: https://bugzilla.mozilla.org/show_bug.cgi?id=1820592
[bz935687]: https://bugzilla.mozilla.org/show_bug.cgi?id=935687
[bz760297]: https://bugzilla.mozilla.org/show_bug.cgi?id=760297
[bz760313]: https://bugzilla.mozilla.org/show_bug.cgi?id=760313
[bz1455137]: https://bugzilla.mozilla.org/show_bug.cgi?id=1455137
[bz1825780]: https://bugzilla.mozilla.org/show_bug.cgi?id=1825780
[bz1655698]: https://bugzilla.mozilla.org/show_bug.cgi?id=1655698
[bz1703528]: https://bugzilla.mozilla.org/show_bug.cgi?id=1703528
[bz1875820]: https://bugzilla.mozilla.org/show_bug.cgi?id=1875820
[bz1877388]: https://bugzilla.mozilla.org/show_bug.cgi?id=1877388
[ccadb-discuss]: https://www.mail-archive.com/public@ccadb.org/msg00223.html
[ccadb-google]: https://groups.google.com/a/ccadb.org/g/public/c/yiJ-bkv-Ftg/m/JsbbxpZJBAAJ
[pki-repo]: https://www.telesec.de/de/service/downloads/pki-repository/
[cp-latest]: https://www.telesec.de/assets/downloads/PKI-Repository/Telekom-Security-CP-EN-DE-V9.0.pdf
[cps-public]: https://www.telesec.de/assets/downloads/PKI-Repository/Telekom-Security-CPS-Public-EN-V12.0.pdf
[root-program]: https://telesec.de/en/root-program/root-program/overview
[root-certs]: https://telesec.de/en/root-program/informations-about-ca-certificates/root-certificates
[root-compat]: https://telesec.de/en/root-program/informations-about-ca-certificates/root-compatibility
[serverid-dv]: https://www.telesec.de/de/produkte/serverpass/details/serverid-dv
[corp-pki]: https://corporate-pki.telekom.de/cps/cps.htm
