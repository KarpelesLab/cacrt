# D-Trust

D-Trust GmbH is a Berlin-based trust service provider and certificate authority, wholly owned by Bundesdruckerei Group GmbH, which is in turn fully owned by the German federal government. Founded as Germany's first "Trust Center," D-Trust issues publicly trusted TLS certificates (OV, DV, and EV) and is a qualified trust service provider under the EU eIDAS regulation. All roots in this folder are operated by D-Trust GmbH; no separate brands or legal entities are merged here.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| D-TRUST BR Root CA 1 2020 | `9ef4a08a.0` | ECC P-384 | 2035-02-11 | `E5:9A:AA:81:60:09:C2:2B:FF:5B:25:BA:D3:7D:F3:06:F0:49:79:7C:1F:81:D8:5A:B0:89:E6:57:BD:8F:00:44` |
| D-TRUST BR Root CA 2 2023 | `ffdd40f9.0` | RSA 4096 | 2038-05-09 | `05:52:E6:F8:3F:DF:65:E8:FA:96:70:E6:66:DF:28:A4:E2:13:40:B5:10:CB:E5:25:66:F9:7C:4F:B9:4B:2B:D1` |
| D-TRUST EV Root CA 1 2020 | `5931b5bc.0` | ECC P-384 | 2035-02-11 | `08:17:0D:1A:A3:64:53:90:1A:2F:95:92:45:E3:47:DB:0C:8D:37:AB:AA:BC:56:B8:1A:A1:00:DC:95:89:70:DB` |
| D-TRUST EV Root CA 2 2023 | `a09a51ae.0` | RSA 4096 | 2038-05-09 | `8E:82:21:B2:E7:D4:00:78:36:A1:67:2F:0D:CC:29:9C:33:BC:07:D3:16:F1:32:FA:1A:20:6D:58:71:50:F1:CE` |
| D-TRUST Root Class 3 CA 2 2009 | `c28a8a30.0` | RSA 2048 | 2029-11-05 | `49:E7:A4:42:AC:F0:EA:62:87:05:00:54:B5:25:64:B6:50:E4:F4:9E:42:E3:48:D6:AA:38:E0:39:E9:57:B1:C1` |
| D-TRUST Root Class 3 CA 2 EV 2009 | `d4dae3dd.0` | RSA 2048 | 2029-11-05 | `EE:C5:49:6B:98:8C:E9:86:25:B9:34:09:2E:EC:29:08:BE:D0:B0:F3:16:C2:D4:73:0C:84:EA:F1:F3:D3:48:81` |

The 2020 and 2023 roots are planned rollovers for the 2009 roots: the BR pair replaces D-TRUST Root Class 3 CA 2 2009, and the EV pair replaces D-TRUST Root Class 3 CA 2 EV 2009. D-Trust has communicated a target replacement date of approximately September 1, 2026 for the 2009 roots in active programs. [[1836258]](https://bugzilla.mozilla.org/show_bug.cgi?id=1836258)

## Rationale for inclusion

All six roots are included in the Mozilla NSS root store (and thus Firefox). The 2009 roots have been trusted for many years; the 2020 ECC roots were approved following a public discussion in January 2022 and shipped in NSS/Firefox thereafter ([Bugzilla #1754890](https://bugzilla.mozilla.org/show_bug.cgi?id=1754890)); the 2023 RSA roots completed public discussion in September–December 2024 and shipped in NSS 3.108 / Firefox 136, with EV treatment enabled in Firefox 137 ([Bugzilla #1836258](https://bugzilla.mozilla.org/show_bug.cgi?id=1836258)).

The 2023 roots are also included in the Google Chrome Root Store (v30) and the Microsoft Trusted Root Program. As of the time of writing, Apple has not included the 2023 roots; the older 2009 roots carry trust on Apple platforms. Trust bits are set for Server Authentication (TLS); EV treatment applies to the EV-designated roots.

D-Trust is a member of the [PKI Consortium](https://pkic.org/members/dtrust/) and a qualified trust service provider registered with the German Federal Network Agency (Bundesnetzagentur) under eIDAS since 2016.

## CA/Browser Forum compliance

D-Trust uses the **ETSI** audit framework. Annual audits are conducted by TÜV Informationstechnik GmbH (TÜViT), a subsidiary of TÜV NORD, against the following standards:

- ETSI EN 319 411-1 (Certificate Policy for CA issuing public certificates)
- ETSI EN 319 401 (General Policy Requirements)
- CA/Browser Forum TLS Baseline Requirements (BR)
- CA/Browser Forum Extended Validation Guidelines (EVG) (for EV roots)

The most recent audit cycle covers the period October 8, 2024 – October 7, 2025, with an audit statement date of January 15, 2026. No deviations were reported for the Standard, TLS BR, or TLS EV audit areas. [CCADB Case #00000689](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000689)

D-Trust discloses all CA records in the CCADB, including intermediate certificates, audit statements, and CP/CPS links. Certificate Transparency (CT) is enforced: D-Trust-issued TLS certificates include Signed Certificate Timestamps (SCTs) as required by browser policies, and certificates appear in public CT logs. No ACME endpoint operated by D-Trust has been identified in public documentation.

## Past non-compliance

The following compliance incidents are publicly documented in Mozilla Bugzilla. No distrust action has been taken against any D-Trust root by any major root program as of the time of writing.

**[Bug 2029013](https://bugzilla.mozilla.org/show_bug.cgi?id=2029013) — Missing pre-signing linting (2025–2026, ongoing)**
CA/Browser Forum Ballot SC-075, effective March 15, 2025, introduced a mandatory requirement (Section 4.3.1.2 of the TLS Baseline Requirements) for a pre-sign linting step before issuing certificates. D-Trust incorrectly assessed its existing RA-side and CA-side configuration checks as satisfying this requirement. The deficiency was discovered during investigation of a related incident (Bug 2023458) in April 2026. D-Trust estimates 57,565 certificates were issued non-compliantly between March 15, 2025 and April 2, 2026. Issuance was halted on April 2, 2026 at 08:45 UTC; a ZLint/PKILint-based pre-sign linting solution was deployed and issuance resumed at 15:40 UTC the same day. All affected still-valid certificates were revoked by April 7, 2026 at 06:05 UTC. A broader review of all CA/Browser Forum requirement interpretations, supported by third-party sample audits, is due by September 30, 2026. The bug remains open pending that review.

**[Bug 2023458](https://bugzilla.mozilla.org/show_bug.cgi?id=2023458) — TLS precertificates exceeding maximum validity (March 2026)**
Following the reduction of maximum TLS certificate validity to 200 days under Ballot SC-081 (effective March 15, 2026), D-Trust mis-issued 19 OV TLS precertificates with a validity period of 203 days. The root cause was a legacy calendar-based duration calculation introduced in 2014 that expanded to 203 days for March start dates. The precertificates were signed by the issuing CA and submitted to CT logs before post-sign linting blocked final certificate generation. All 19 precertificates were revoked on March 15, 2026. Resolved fixed.

**[Bug 1896190](https://bugzilla.mozilla.org/show_bug.cgi?id=1896190) — EV certificate postalCode/localityName mixup (2024)**
D-Trust issued one EV certificate in which the subject's postalCode and localityName fields were swapped. The error was reported by a third party on May 9, 2024. The certificate had been active since February 2023 and passed an annual recheck in October 2023 without the error being detected. The certificate was revoked on May 13, 2024, within the 5-day window. A programmatic postcode validation check was subsequently implemented. Resolved fixed.

**[Bug 2007116](https://bugzilla.mozilla.org/show_bug.cgi?id=2007116) — CRL URL mismatch in CCADB disclosure**
Four intermediate CA certificate records in the CCADB contained CRL URLs that did not exactly match the URLs encoded in the corresponding certificates, violating CCADB Policy disclosure requirements. The issue was identified via third-party notification. Resolved fixed.

**[Bug 1939809](https://bugzilla.mozilla.org/show_bug.cgi?id=1939809) — QCStatement with HTTP link in PDS**
D-Trust issued 25 EV TLS certificates that also qualified as Qualified Certificates for Website Authentication (QWACs) under eIDAS. These contained a QCStatement extension with an HTTP (not HTTPS) link to the PKI Disclosure Statement, contrary to ETSI EN 319 412-5. Resolved fixed.

**[Bug 1563772](https://bugzilla.mozilla.org/show_bug.cgi?id=1563772) — Precertificate OU field exceeding 64 characters (2019)**
An older incident where internal QA detected precertificates with an OU attribute exceeding the 64-character limit. Issuance was stopped promptly. Resolved fixed.

No Mozilla distrust action, root removal, or formal warning has been issued against D-Trust roots. A search of [Mozilla Bugzilla CA incidents for D-Trust](https://bugzilla.mozilla.org/buglist.cgi?product=CA%20Program&component=CA%20Certificate%20Compliance&short_desc=D-Trust&short_desc_type=allwordssubstr) shows no outstanding unresolved incidents beyond Bug 2029013 (awaiting the September 2026 review milestone).

## Transparency

D-Trust publishes its CP, TSPS, and all product-specific CPS documents publicly at [https://www.d-trust.net/en/support/repository](https://www.d-trust.net/en/support/repository), available in English and German at no charge with no access restrictions. Archived versions are retained at [https://www.d-trust.net/en/support/repository-archiv](https://www.d-trust.net/en/support/repository-archiv). Current key documents include:

- CP of D-Trust GmbH (valid from 2026-05-13)
- TSPS of D-Trust (valid from 2026-05-13)
- CPS of D-Trust Root PKI (valid from 2024-11-24)
- CPS of D-Trust CSM PKI (valid from 2026-04-20)

D-Trust maintains full CCADB disclosure for all publicly-trusted CAs, including intermediate CA records, audit statement links, and CP/CPS pointers. Incident reports are filed in Mozilla Bugzilla under the CA Program product. TLS certificates issued under these roots are subject to mandatory Certificate Transparency log submission as required by browser root program policies.

## Sources

- [D-Trust GmbH official website](https://www.d-trust.net/en)
- [D-Trust repository (CP/CPS)](https://www.d-trust.net/en/support/repository)
- [Bundesdruckerei Group — group structure](https://www.bundesdruckerei.de/en/group/group-structure)
- [CCADB Case #00000689 — D-TRUST EV Root CA 1 2020 inclusion](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000689)
- [Bugzilla #1754890 — Add D-TRUST BR/EV Root CA 1 2020 to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=1754890)
- [Bugzilla #1836258 — Add D-TRUST BR/EV Root CA 2 2023](https://bugzilla.mozilla.org/show_bug.cgi?id=1836258)
- [CCADB public discussion — D-Trust TLS CA inclusion request (2023 roots)](https://groups.google.com/a/ccadb.org/g/public/c/EPVczE_6oCc)
- [CCADB public discussion — D-Trust SBR Root CA inclusion (S/MIME)](https://groups.google.com/a/ccadb.org/g/public/c/gbPfqACMfRw)
- [Mozilla dev-security-policy — D-Trust root inclusion requests (2020)](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/0Ljc_EkPsiQ)
- [Bugzilla #2029013 — D-Trust: Missing pre-signing linting for TLS issuance](https://bugzilla.mozilla.org/show_bug.cgi?id=2029013)
- [Bugzilla #2023458 — D-Trust: TLS precertificates exceeding maximum validity](https://bugzilla.mozilla.org/show_bug.cgi?id=2023458)
- [Bugzilla #1896190 — D-Trust: EV certificate postalCode/localityName mixup](https://bugzilla.mozilla.org/show_bug.cgi?id=1896190)
- [Bugzilla #2007116 — D-Trust: CRL URL disclosure mismatch](https://bugzilla.mozilla.org/show_bug.cgi?id=2007116)
- [Bugzilla #1939809 — D-Trust: QCStatement with HTTP PDS link](https://bugzilla.mozilla.org/show_bug.cgi?id=1939809)
- [Bugzilla #1563772 — D-Trust: Precertificate OU > 64 characters](https://bugzilla.mozilla.org/show_bug.cgi?id=1563772)
- [TÜViT — Certification Authorities according to ETSI](https://www.tuvit.de/en/services/certification/certification-authorities-according-to-etsi/)
- [D-Trust — PKI Consortium member profile](https://pkic.org/members/dtrust/)
