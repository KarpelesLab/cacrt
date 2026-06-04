# Actalis

Actalis S.p.A. is an Italian Certification Authority headquartered in Ponte San Pietro (BG), Italy, and owned by the Aruba Group since 2009. It is an eIDAS-qualified Trust Service Provider and CA/Browser Forum member, issuing publicly trusted TLS/SSL, S/MIME, and code-signing certificates under the Web PKI. This folder consolidates all root certificates operated by Actalis S.p.A.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| Actalis Authentication Root CA | `930ac5d2.0` | RSA 4096 | 2030-09-22 | `55:92:60:84:EC:96:3A:64:B9:6E:2A:BE:01:CE:0B:A8:6A:64:FB:FE:BC:C7:AA:B5:AF:C1:55:B3:7F:D7:60:66` |

## Rationale for inclusion

The Actalis Authentication Root CA is included in all four major root programs:

- **Mozilla NSS** — included since NSS 3.13.6 / Firefox 16 (2012); trust bits for Websites and Email. Approved via [Bugzilla #742525](https://bugzilla.mozilla.org/show_bug.cgi?id=742525).
- **Chrome Root Store** — confirmed included per [Cloudflare Radar CT CA data](https://radar.cloudflare.com/certificate-transparency/ca/55926084EC963A64B96E2ABE01CE0BA86A64FBFEBCC7AAB5AFC155B37FD76066).
- **Apple** and **Microsoft** root programs — listed as included across all four major root stores in CCADB reporting.

Actalis is the only Italian CA that is a [CA/Browser Forum](https://cabforum.org/) member and has participated since 2011. It is also a qualified Trust Service Provider under the EU eIDAS regulation, accredited by the Italian national supervisory body AgID. [[Source: Actalis company page](https://www.actalis.com/company)]

## CA/Browser Forum compliance

Actalis is audited under **ETSI EN 319 411** by TayllorCox PCEB (Czech Republic). The most recent audit statement (dated 2026-04-02) is publicly available and covers TLS BR, TLS EV BR, Code Signing BR, and S/MIME BR scopes. [[Audit Statement 2026 — TayllorCox PCEB](https://pceb.tayllorcox.cz/files/2026_Actalis_AAL_Standard_Audit_v2_signedTS.pdf)]

Actalis commits to the CA/Browser Forum Baseline Requirements for TLS certificates and publishes its Certification Practice Statement (CPS) at its [Legal Repository](https://www.actalis.com/legal-repository). The CPS is implemented for SSL/TLS Server and Code Signing certificates. Disclosure of sub-CA certificates and audit information is maintained in the CCADB. [[CCADB Included CA report](https://ccadb.my.salesforce-sites.com/mozilla/IncludedCACertificateReport)]

Actalis supports the **ACME** protocol (RFC 8555) for automated issuance of DV TLS certificates, with a public ACME endpoint at `https://acme-api.actalis.com/acme/directory`. [[Actalis ACME documentation](https://www.actalis.com/news/ssl-communications/the-acme-protocol-for-automated-certificate-management)]

All publicly trusted certificates are submitted to **Certificate Transparency** logs as required by Chrome and Apple policies (enforced since 2018). Actalis certificates can be monitored via crt.sh and Cloudflare Radar.

## Past non-compliance

Actalis has been the subject of several publicly documented compliance incidents, all tracked in Mozilla Bugzilla:

- **[Bug #1390974](https://bugzilla.mozilla.org/show_bug.cgi?id=1390974) (2017) — Non-BR-compliant certificate issuance:** Certificates issued for internal server names in violation of the Baseline Requirements.
- **[Bug #1405817](https://bugzilla.mozilla.org/show_bug.cgi?id=1405817) (2017) — Duplicate issuer/serial numbers:** Intermediate certificates issued with the same issuer and serial number, violating RFC 5280 §4.1.2.2 and the BRs.
- **[Bug #1523680](https://bugzilla.mozilla.org/show_bug.cgi?id=1523680) (2019) — Non-BR-compliant OCSP responder:** An OCSP responder returned "good" for unknown certificate serials rather than "unknown." Actalis corrected the configuration after the issue was filed.
- **[Bug #1572638](https://bugzilla.mozilla.org/show_bug.cgi?id=1572638) (2019) — Failure to revoke within BR timeframe:** A large-scale revocation event; approximately 88% (~215,000) of affected certificates were revoked within the first month, but full revocation was not completed within the BR-mandated window. The affected population included many Italian public-administration customers.
- **[Bug #1717357](https://bugzilla.mozilla.org/show_bug.cgi?id=1717357) (2021) — Non-compliant intermediate CAs:** Intermediates issued after 2020-08-20 for the AgID/PEC (Certified Electronic Mail) system did not comply with Mozilla Policy and the BRs regarding EKU extensions. The linter (ZLint) in use did not detect the issue. Actalis revoked affected TLS certificates.
- **[Bug #1883731](https://bugzilla.mozilla.org/show_bug.cgi?id=1883731) (2024) — Invalid RDN encoding order:** 263 EV TLS certificates were issued with Subject Attribute Encoding Order non-compliant with BR 2.0.0 §7.1.4.2. ZLint and Actalis's internal linter did not detect the issue; all 263 certificates were revoked with 0% missed at 5, 15, and 30 days.
- **[Bug #1982646](https://bugzilla.mozilla.org/show_bug.cgi?id=1982646) (2025) — Missing CCADB disclosure for new sub-CA:** The sub-CA "Actalis Domain Validated TLS Server RSA CA 2025" issued three leaf certificates (all on an internal test domain) before being disclosed in CCADB. Root cause: internal procedures had not been updated to reflect the CCADB policy effective 2025-07-15, and cross-certificate disclosure requirements were not recognized because no cross-certificates had been issued in the prior decade.

No distrust action has been taken against Actalis by any major root program. All incidents resulted in remediation and public incident reports. No additional major incidents were found beyond those listed above; see the [Actalis Bugzilla query](https://bugzilla.mozilla.org/buglist.cgi?query_format=advanced&product=CA%20Program&component=CA%20Certificate%20Compliance&status_whiteboard=Actalis&status_whiteboard_type=allwordssubstr) for a current view.

## Transparency

- **CP/CPS:** Published at [actalis.com/legal-repository](https://www.actalis.com/legal-repository). A separate CPS covers Qualified Certificates (eIDAS). The current CPS is described as the version implemented pending AgID approval.
- **CCADB disclosure:** Actalis discloses its root and sub-CA certificates to the CCADB. A 2025 incident (Bug #1982646) identified a gap in disclosure timing for a newly created sub-CA; Actalis acknowledged and remediated the issue.
- **Audit statements:** Annual ETSI EN 319 411 audit letters from TayllorCox PCEB are published on the [Actalis Legal Repository](https://www.actalis.com/legal-repository) and linked in CCADB.
- **Certificate Transparency:** All issued TLS certificates include SCTs as required. CT coverage for this root can be monitored via [crt.sh](https://crt.sh/?caid=&opt=&q=Actalis).
- **Incident reporting:** Actalis files public incident reports in [Mozilla Bugzilla](https://bugzilla.mozilla.org/buglist.cgi?query_format=advanced&product=CA%20Program&component=CA%20Certificate%20Compliance&status_whiteboard=Actalis&status_whiteboard_type=allwordssubstr) following the CA Community Forum incident-reporting template.

## Sources

- [Actalis company page — actalis.com/company](https://www.actalis.com/company)
- [Actalis legal repository — actalis.com/legal-repository](https://www.actalis.com/legal-repository)
- [Actalis ACME news announcement](https://www.actalis.com/news/ssl-communications/free-and-unlimited-dv-certificates-actalis-becomes-europes-reference-point-for-acme-based-web-security)
- [Actalis ACME guide — guide.actalis.com/ssl/activation/acme](https://guide.actalis.com/ssl/activation/acme)
- [Mozilla Bugzilla #520557 — original Actalis inclusion request](https://bugzilla.mozilla.org/show_bug.cgi?id=520557)
- [Mozilla Bugzilla #742525 — Add Actalis Authentication Root CA to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=742525)
- [Mozilla Bugzilla #1176188 — Enable email trust bit](https://bugzilla.mozilla.org/show_bug.cgi?id=1176188)
- [Mozilla Bugzilla #1390974 — Non-BR-compliant certificate issuance (2017)](https://bugzilla.mozilla.org/show_bug.cgi?id=1390974)
- [Mozilla Bugzilla #1405817 — Certs with same issuer and serial number (2017)](https://bugzilla.mozilla.org/show_bug.cgi?id=1405817)
- [Mozilla Bugzilla #1523680 — Non-BR-compliant OCSP responder (2019)](https://bugzilla.mozilla.org/show_bug.cgi?id=1523680)
- [Mozilla Bugzilla #1572638 — Failure to revoke within BR timeframe (2019)](https://bugzilla.mozilla.org/show_bug.cgi?id=1572638)
- [Mozilla Bugzilla #1717357 — Non-compliant intermediates (2021)](https://bugzilla.mozilla.org/show_bug.cgi?id=1717357)
- [Mozilla Bugzilla #1883731 — Invalid RDN encoding order (2024)](https://bugzilla.mozilla.org/show_bug.cgi?id=1883731)
- [Mozilla Bugzilla #1982646 — Missing CCADB disclosure for new sub-CA (2025)](https://bugzilla.mozilla.org/show_bug.cgi?id=1982646)
- [CCADB Included CA Certificate Report](https://ccadb.my.salesforce-sites.com/mozilla/IncludedCACertificateReport)
- [TayllorCox PCEB Audit Statement 2026](https://pceb.tayllorcox.cz/files/2026_Actalis_AAL_Standard_Audit_v2_signedTS.pdf)
- [PKI Consortium — Actalis member page](https://pkic.org/members/actalis/)
