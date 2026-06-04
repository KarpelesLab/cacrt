# Sectigo

Sectigo (formerly COMODO CA Limited, rebranded November 1, 2018) is a commercial certificate authority headquartered in Roseland, New Jersey, USA, with roots in Salford, Greater Manchester, UK where the original Comodo CA was founded in 1998. It is one of the largest issuers of TLS server-authentication certificates in the Web PKI. This folder consolidates roots operated under three related legal/brand names: **Sectigo Limited**, **COMODO CA Limited**, and **The USERTRUST Network** (the USERTrust roots are Comodo/Sectigo infrastructure).

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| COMODO ECC Certification Authority | `eed8c118.0` | ECC P-384 | 2038-01-18 | `17:93:92:7A:06:14:54:97:89:AD:CE:2F:8F:34:F7:F0:B6:6D:0F:3A:E3:A3:B8:4D:21:EC:15:DB:BA:4F:AD:C7` |
| COMODO RSA Certification Authority | `d6325660.0` | RSA 4096 | 2038-01-18 | `52:F0:E1:C4:E5:8E:C6:29:29:1B:60:31:7F:07:46:71:B8:5D:7E:A8:0D:5B:07:27:34:63:53:4B:32:B4:02:34` |
| Sectigo Public Server Authentication Root E46 | `da0cfd1d.0` | ECC P-384 | 2046-03-21 | `C9:0F:26:F0:FB:1B:40:18:B2:22:27:51:9B:5C:A2:B5:3E:2C:A5:B3:BE:5C:F1:8E:FE:1B:EF:47:38:0C:53:83` |
| Sectigo Public Server Authentication Root R46 | `9046744a.0` | RSA 4096 | 2046-03-21 | `7B:B6:47:A6:2A:EE:AC:88:BF:25:7A:A5:22:D0:1F:FE:A3:95:E0:AB:45:C7:3F:93:F6:56:54:EC:38:F2:5A:06` |
| USERTrust ECC Certification Authority | `f30dd6ad.0` | ECC P-384 | 2038-01-18 | `4F:F4:60:D5:4B:9C:86:DA:BF:BC:FC:57:12:E0:40:0D:2B:ED:3F:BC:4D:4F:BD:AA:86:E0:6A:DC:D2:A9:AD:7A` |
| USERTrust RSA Certification Authority | `fc5a8f99.0` | RSA 4096 | 2038-01-18 | `E7:93:C9:B0:2F:D8:AA:13:E2:1C:31:22:8A:CC:B0:81:19:64:3B:74:9C:89:89:64:B1:74:6D:46:C3:D4:CB:D2` |

**Note on legacy roots:** The COMODO and USERTrust roots are legacy multi-purpose roots that are in the process of losing TLS trust in Chrome and Mozilla between 2025 and 2027 due to root-program policies limiting CA lifespans and enforcing single-purpose roots. Sectigo recommends migrating to the R46/E46 hierarchies. See [Sectigo's distrust timeline page](https://www.sectigo.com/resource-library/changes-to-root-ca-hierarchies-and-trust-status) for per-root dates.

## Rationale for inclusion

Sectigo roots are included in all major root programs:

- **Mozilla NSS / Firefox:** Legacy COMODO and USERTrust roots have been in the Mozilla root store for many years. The new single-purpose roots (R46, E46) were approved via CCADB Root Inclusion Case #1215 following a six-week public discussion period (April 24–June 9, 2023) with no objections, tracked in [Mozilla Bugzilla #1817340](https://bugzilla.mozilla.org/show_bug.cgi?id=1817340). EV treatment was enabled via [Bugzilla #1840432](https://bugzilla.mozilla.org/show_bug.cgi?id=1840432).
- **Chrome Root Store:** The Chrome Root Program separately approved inclusion of Sectigo Public Server Authentication Root E46 and R46, announced November 2, 2023, effective ~December 5, 2023. ([CCADB public list announcement](https://groups.google.com/a/ccadb.org/g/public/c/o0V6XfBJ2vw))
- **Apple and Microsoft:** The R46/E46 roots are also included in the Apple and Microsoft root stores according to Sectigo's published migration guidance.

The scope in this folder is TLS server authentication (Websites trust bit). The R46/E46 roots are also approved for EV treatment.

## CA/Browser Forum compliance

Sectigo undergoes annual **WebTrust for CAs** audits covering the WebTrust Principles and Criteria, the CA/Browser Forum Baseline Requirements, and Network Security. The most recent published cycle covers April 1, 2024 – March 31, 2025, with a Standard Audit Statement Date of June 17, 2025, performed by BDO International Limited. Sectigo publicly announced the completion of its 2023–24 WebTrust audit on July 1, 2024. ([Business Wire announcement](https://www.businesswire.com/news/home/20240701133093/en/Sectigo-Delivers-23-24-WebTrust-Audit-Confirms-Compliance-with-CABrowser-Forum-Baseline-Requirements))

Sectigo discloses audit results, CP/CPS documents, and intermediate CA details in the CCADB. After CPS v5.4.1, Sectigo migrated to a combined CP/CPS document structure, with separate documents per trust purpose (TLS, S/MIME, Code Signing, etc.). The current CP/CPS repository is published at [sectigo.com/cps-repository](https://www.sectigo.com/cps-repository).

Sectigo operates its own Certificate Transparency logs and also sponsors Let's Encrypt's high-volume "Oak" CT log to strengthen CT ecosystem resilience. Sectigo's Rob Stradling created [crt.sh](https://crt.sh), a widely used public CT log search tool. Sectigo supports ACME-based automated certificate management through its Certificates as a Service (CaaS) platform.

## Past non-compliance

The following incidents are publicly documented in Mozilla Bugzilla or the dev-security-policy forum. This list is not exhaustive of all minor compliance filings.

- **2011 – Registration Authority compromise** ([Mozilla Security Blog](https://blog.mozilla.org/security/2011/03/25/comodo-certificate-issue-follow-up/)): An attacker compromised a Comodo RA partner in Italy on March 15, 2011, causing nine fraudulent TLS certificates to be issued for high-value domains (google.com, yahoo.com, skype.com, addons.mozilla.org, live.com). Comodo detected and revoked the certificates quickly and disclosed the incident to browser vendors. Browser vendors responded with hard-coded distrust entries. Comodo changed its practices to require intermediate certificates for RA issuance. A self-proclaimed Iranian hacker later claimed sole responsibility.

- **2016 – Broken OCR mis-issuance** ([Bugzilla #1311713](https://bugzilla.mozilla.org/show_bug.cgi?id=1311713)): Comodo issued certificates to incorrect parties due to faulty optical character recognition applied to WHOIS data. Comodo disabled OCR-based domain validation in response.

- **2018 – Failure to revoke within 24 hours** ([Bugzilla #1492006](https://bugzilla.mozilla.org/show_bug.cgi?id=1492006)): Sectigo failed to revoke a certificate within the BR-required 24-hour window after a revocation request was received. The root cause was inadequate internal communication of urgency. Sectigo subsequently released a self-service revocation portal (August 2019).

- **2019 – Invalid OU field on DV certificates** ([Bugzilla #1593776](https://bugzilla.mozilla.org/show_bug.cgi?id=1593776)): Sectigo was found to include non-compliant organizationalUnitName fields in DV certificates. Sectigo ceased the practice on December 15, 2019 and submitted a CA/B Forum proposal on OU field handling.

- **2019 – Missing CPS changelog** ([Bugzilla #1545208](https://bugzilla.mozilla.org/show_bug.cgi?id=1545208)): Sectigo's CPS lacked a required changelog for policy changes since Mozilla Policy 2.4, identified during compliance review.

- **2019 – CCADB automated letter validation failure** ([Bugzilla #1597950](https://bugzilla.mozilla.org/show_bug.cgi?id=1597950)): Sectigo's "Ensured Root CA" intermediate failed CCADB automated letter validation.

- **2021 – Test certificates issued from trusted CA** ([Bugzilla #1712188](https://bugzilla.mozilla.org/show_bug.cgi?id=1712188)): Certificates, including some issued to unregistered domains (constituting a domain-control validation failure), were issued from a QA account under a trusted CA. All affected certificates were revoked. Sectigo attributed the root cause partly to inadequate onboarding of new staff following the company's post-2017 growth.

- **2025 – OCSP/CRL endpoints unavailable** ([Bugzilla #1991196](https://bugzilla.mozilla.org/show_bug.cgi?id=1991196)): OCSP, caIssuers, and CRL URLs in a subordinate CA became inaccessible because the domain (`comodoca4.com`) was not migrated during the 2017 Comodo Group carve-out and had expired. Sectigo revoked the affected subordinate CA and its leaf certificates.

No Mozilla or Chrome distrust action has been taken against Sectigo's root certificates as a direct sanction; the ongoing legacy-root distrust (COMODO, USERTrust) is a programmatic policy applying to all multi-purpose roots of sufficient age, not a targeted incident response.

## Transparency

- **CP/CPS:** Publicly available at [sectigo.com/cps-repository](https://www.sectigo.com/cps-repository); historical versions at [sectigo.com/certificate-practice-statement-archive](https://www.sectigo.com/certificate-practice-statement-archive).
- **CCADB disclosure:** Sectigo discloses all roots and subordinate CAs in the CCADB. The R46/E46 inclusion case is publicly visible as [CCADB Case #1215](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00001215) and in the [CCADB public discussion thread](https://groups.google.com/a/ccadb.org/g/public/c/1sKKdixUyFs).
- **Incident self-reporting:** Sectigo has reported incidents directly to Mozilla via Bugzilla (see above). The consistency of self-reporting has been mixed over time, as documented in several of the Bugzilla cases cited.
- **Certificate Transparency:** Sectigo embeds SCTs in all publicly-trusted TLS certificates as required by browser policies. Sectigo operates CT logs and sponsors Let's Encrypt's Oak log. The [CTLogs-AcceptedRoots repository](https://github.com/sectigo/CTLogs-AcceptedRoots) documents which roots Sectigo's CT logs accept.

## Sources

- [Sectigo official website](https://www.sectigo.com/)
- [Sectigo CP/CPS repository](https://www.sectigo.com/cps-repository)
- [Sectigo CPS archive](https://www.sectigo.com/certificate-practice-statement-archive)
- [Comodo CA rebrands as Sectigo (PR Newswire, 2018)](https://www.prnewswire.com/news-releases/comodo-ca-rebrands-as-sectigo-300741808.html)
- [Sectigo WebTrust 2023–24 audit announcement (Business Wire)](https://www.businesswire.com/news/home/20240701133093/en/Sectigo-Delivers-23-24-WebTrust-Audit-Confirms-Compliance-with-CABrowser-Forum-Baseline-Requirements)
- [Mozilla Bugzilla #1817340 – Add Sectigo R46/E46 Roots](https://bugzilla.mozilla.org/show_bug.cgi?id=1817340)
- [Mozilla Bugzilla #1840432 – Enable EV Treatment for Sectigo E46 and R46](https://bugzilla.mozilla.org/show_bug.cgi?id=1840432)
- [CCADB public discussion – Sectigo CA inclusion request](https://groups.google.com/a/ccadb.org/g/public/c/1sKKdixUyFs)
- [CCADB Case #1215 print view](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00001215)
- [Chrome Root Store inclusion announcement – Sectigo (CCADB public list)](https://groups.google.com/a/ccadb.org/g/public/c/o0V6XfBJ2vw)
- [Sectigo changes to root CA hierarchies and trust status](https://www.sectigo.com/resource-library/changes-to-root-ca-hierarchies-and-trust-status)
- [Sectigo public root CA migration guide](https://www.sectigo.com/sectigo-public-root-cas-migration)
- [Mozilla Security Blog – Comodo certificate issue follow-up (2011)](https://blog.mozilla.org/security/2011/03/25/comodo-certificate-issue-follow-up/)
- [Mozilla Bugzilla #1311713 – Comodo broken OCR mis-issuance](https://bugzilla.mozilla.org/show_bug.cgi?id=1311713)
- [Mozilla Bugzilla #1492006 – Sectigo failure to revoke within 24 hours](https://bugzilla.mozilla.org/show_bug.cgi?id=1492006)
- [Mozilla Bugzilla #1545208 – Sectigo missing CPS changelog](https://bugzilla.mozilla.org/show_bug.cgi?id=1545208)
- [Mozilla Bugzilla #1593776 – Sectigo invalid OU on DV certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=1593776)
- [Mozilla Bugzilla #1597950 – Sectigo CCADB failed ALV](https://bugzilla.mozilla.org/show_bug.cgi?id=1597950)
- [Mozilla Bugzilla #1712188 – Sectigo test certificates issued from trusted CA](https://bugzilla.mozilla.org/show_bug.cgi?id=1712188)
- [Mozilla Bugzilla #1991196 – Sectigo OCSP/CRL endpoints unavailable (2025)](https://bugzilla.mozilla.org/show_bug.cgi?id=1991196)
- [Sectigo CTLogs-AcceptedRoots (GitHub)](https://github.com/sectigo/CTLogs-AcceptedRoots)
- [Sectigo CT log sponsorship of Let's Encrypt Oak](https://www.sectigo.com/resource-library/sectigo-is-sponsoring-a-certificate-transparency-ct-log-from-lets-encrypt-heres-why)
