# Google Trust Services

Google Trust Services (GTS) is the public certificate authority operated by Google Trust Services LLC (Mountain View, CA, USA), a wholly-owned subsidiary of Alphabet Inc. Established in 2016–2017, GTS issues publicly trusted TLS certificates for Google's own services and for general subscribers worldwide, and it also operates Google Trust Services Europe Ltd for EU-based subscribers. GTS bootstrapped its root infrastructure by acquiring two GlobalSign root CA certificates (GlobalSign R2 and R4) in 2016 and issuing four new self-signed roots (GTS Root R1–R4) modelled on those roots; together these form the current GTS hierarchy in the Web PKI.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| GTS Root R1 | `1001acf7.0` | RSA 4096 | 2036-06-22 | `D9:47:43:2A:BD:E7:B7:FA:90:FC:2E:6B:59:10:1B:12:80:E0:E1:C7:E4:E4:0F:A3:C6:88:7F:FF:57:A7:F4:CF` |
| GTS Root R3 | `0a775a30.0` | ECC P-384 | 2036-06-22 | `34:D8:A7:3E:E2:08:D9:BC:DB:0D:95:65:20:93:4B:4E:40:E6:94:82:59:6E:8B:6F:73:C8:42:6B:01:0A:6F:48` |
| GTS Root R4 | `a3418fda.0` | ECC P-384 | 2036-06-22 | `34:9D:FA:40:58:C5:E2:63:12:3B:39:8A:E7:95:57:3C:4E:13:13:C8:3F:E6:8F:93:55:6C:D5:E8:03:1B:3C:7D` |

All three roots were self-signed on 2016-06-22 and expire 2036-06-22. GTS Root R2 (also part of the original set) is not present in this folder. The corresponding OpenSSL symlink names follow the pattern `<hash>.0`.

## Rationale for inclusion

GTS Root R1, R3, and R4 are included in the Mozilla root store (approved via [Bugzilla #1325532](https://bugzilla.mozilla.org/show_bug.cgi?id=1325532) and later the replacement request in [Bugzilla #1496204](https://bugzilla.mozilla.org/show_bug.cgi?id=1496204) / [#1735407](https://bugzilla.mozilla.org/show_bug.cgi?id=1735407)), the Chrome Root Store, and Microsoft's Trusted Root Certificate Program. All roots are scoped to TLS server authentication (Websites EKU). GTS underwent the full Mozilla public-discussion and CCADB root-inclusion process for both its initial roots and the subsequent replacement roots (replacing R1–R4 due to a missing `digitalSignature` Key Usage bit, tracked in [Bugzilla #1652581](https://bugzilla.mozilla.org/show_bug.cgi?id=1652581)).

## CA/Browser Forum compliance

GTS is audited annually under the WebTrust for CAs and WebTrust Baseline Requirements (BR) schemes; its auditor of record is Ernst & Young, LLP ([CCADB case 00000666](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000666)). The audit scope covers TLS BR and Network and Certificate System Security Requirements (NCSSRs). Audit reports are disclosed to the CCADB and linked from the [GTS repository page](https://pki.goog/repository/).

GTS's CP/CPS (combined document, currently published at [pki.goog/repository/](https://pki.goog/repository/)) conforms to RFC 3647 and adopts CA/Browser Forum Baseline Requirements by reference. Notably:

- GTS issues exclusively via ACME and supports ACME Renewal Information (ARI) and Multi-Perspective Issuance Corroboration (MPIC) as described in the [GTS FAQ](https://pki.goog/faq/) and [Services page](https://pki.goog/services/).
- All publicly issued certificates are logged to Certificate Transparency logs at issuance; a race-condition mis-embedding of SCTs was reported and resolved in 2023 ([Bugzilla #1815874](https://bugzilla.mozilla.org/show_bug.cgi?id=1815874)).
- GTS stopped issuing OV TLS end-entity certificates as of 2021-12-05.
- CA private keys are stored in FIPS 140-2 Level 3 or higher HSMs per the [GTS CPS](https://pki.goog/repo/cps/4.14/GTS-CPS.html).

## Past non-compliance

The following incidents are publicly documented in Mozilla Bugzilla. All are resolved.

**Forbidden domain validation method (Bug [1706967](https://bugzilla.mozilla.org/show_bug.cgi?id=1706967), 2021):** GTS's CPS referenced BR domain validation method 3.2.2.4.10 (TLS Using a Random Number, retired by Ballot SC33 effective 2020-09-22) rather than the replacement method 3.2.2.4.20. Approximately 1,029,743 certificates were covered by the incorrect CPS reference; 248,139 non-expired certificates were revoked and re-issued. GTS also missed the BR 5-day revocation deadline (reported in [Bug 1715421](https://bugzilla.mozilla.org/show_bug.cgi?id=1715421)). Tagged `[ca-compliance] [policy-failure]`.

**Certificates not disclosed in CCADB (Bug [1667844](https://bugzilla.mozilla.org/show_bug.cgi?id=1667844), 2020–2021):** Six reissued GTS root and cross-signed root certificates (R1–R4, GlobalSign R2/R4 re-signs) were not disclosed in the CCADB. The root cause was a CCADB tooling limitation for re-issued roots and confusion about the correct disclosure path. Resolved after coordination with CCADB administrators. Tagged `[ca-compliance] [disclosure-failure]`.

**Incorrect SCT embedded in SXG certificates (Bug [1815874](https://bugzilla.mozilla.org/show_bug.cgi?id=1815874), 2023):** A 2016 shallow-copy bug in GTS's issuance code caused a race condition that embedded SCTs from one precertificate into a different final certificate for SignedHTTPExchange (SXG) certificates. 2,271 SXG certificates (0.13% of SXG issuance) were affected over 2021–2023; all were revoked by 2023-02-14 and the code defect was fixed. TLS certificates were not affected.

**OCSP serving issues (Bugs [1630040](https://bugzilla.mozilla.org/show_bug.cgi?id=1630040), [1634795](https://bugzilla.mozilla.org/show_bug.cgi?id=1634795), [1773556](https://bugzilla.mozilla.org/show_bug.cgi?id=1773556), [1882904](https://bugzilla.mozilla.org/show_bug.cgi?id=1882904), 2020–2023):** Multiple incidents where EJBCA-based legacy CAs served empty or incorrect OCSP responses, or where batch revocation data accidentally un-revoked CRLs for GTS Y3/Y4 subordinate CAs. GTS attributed recurring OCSP issues in part to maintaining dual infrastructure (EJBCA legacy + primary CA system). All incidents were resolved with remediation committed to primary-system migration.

**Invalid CRL reason code (Bug [1793467](https://bugzilla.mozilla.org/show_bug.cgi?id=1793467), 2022):** A CRL entry contained reason code 7, not a valid value per RFC 5280 § 5.3.1. Root cause was a Go code migration that introduced an incorrect enum mapping. No mis-issuance resulted.

**CRL validity period off by one second (Bug [1731164](https://bugzilla.mozilla.org/show_bug.cgi?id=1731164), 2021):** EJBCA-based secondary CA system generated CRLs with a validity period one second longer than required. Identified internally during periodic compliance review. No mis-issuance.

**Failure to provide timely incident updates (Bugs [1708516](https://bugzilla.mozilla.org/show_bug.cgi?id=1708516), [1770510](https://bugzilla.mozilla.org/show_bug.cgi?id=1770510)):** GTS was cited for not providing regular incident status updates (Bug 1708516) and for failing to deliver a preliminary incident report within 24 hours of a certificate-problem report (Bug 1770510). Both were resolved with process improvements.

**Incomplete CRL Distribution Point URLs in CCADB (Bug [2031164](https://bugzilla.mozilla.org/show_bug.cgi?id=2031164), 2022–2023):** GTS's CCADB entries for root CAs did not list the complete set of distinct HTTP CRL Distribution Point URLs present in unexpired certificates. Resolved after CCADB policy clarification.

No distrust action has been taken against GTS roots by any major root program. For a live search of open GTS incidents see [Mozilla Bugzilla CA:GTS](https://bugzilla.mozilla.org/buglist.cgi?component=CA%20Certificate%20Root%20Program&product=NSS&short_desc=Google+Trust+Services&short_desc_type=allwordssubstr).

## Transparency

- **CP/CPS:** Published at [pki.goog/repository/](https://pki.goog/repository/) (current TLS CP/CPS and historical version archive). Document follows RFC 3647 structure.
- **CCADB:** GTS discloses all root and intermediate CA certificates, audit reports, and CP/CPS URLs to the CCADB. The replacement root inclusion case is [CCADB case 00000666](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000666).
- **Certificate Transparency:** All publicly trusted TLS certificates are logged to CT logs at issuance; per GTS policy, SCTs are embedded in issued certificates. GTS operates the [Google Xenon CT log family](https://transparency.google/logs/).
- **Incident self-reporting:** GTS files incident reports in Mozilla Bugzilla and has committed (since 2021) to mirroring Mozilla Bugzilla CA bugs to its internal tracking system for SLA monitoring.
- **ACME API:** Certificates are available publicly via ACME at `dv.acme-v02.api.pki.goog` as described at [pki.goog/services/](https://pki.goog/services/).

## Sources

- [Google Trust Services — Home (pki.goog)](https://pki.goog/)
- [Google Trust Services — Repository (pki.goog/repository/)](https://pki.goog/repository/)
- [Google Trust Services — FAQ (pki.goog/faq/)](https://pki.goog/faq/)
- [Google Trust Services — Services (pki.goog/services/)](https://pki.goog/services/)
- [GTS TLS CP/CPS v6.0 (pki.goog)](https://pki.goog/repo/cp-cps-tls/6.0/GTS-CP-CPS-TLS.html)
- [GTS CPS v4.14 (pki.goog)](https://pki.goog/repo/cps/4.14/GTS-CPS.html)
- [Mozilla Bugzilla #1325532 — Add Google Root Certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=1325532)
- [Mozilla Bugzilla #1496204 — Add GTS LLC root certificates to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=1496204)
- [Mozilla Bugzilla #1652581 — GTS: digitalSignature KeyUsage not set](https://bugzilla.mozilla.org/show_bug.cgi?id=1652581)
- [Mozilla Bugzilla #1667844 — GTS: Certificates not disclosed in CCADB](https://bugzilla.mozilla.org/show_bug.cgi?id=1667844)
- [Mozilla Bugzilla #1706967 — GTS: Forbidden Domain Validation Method 3.2.2.4.10](https://bugzilla.mozilla.org/show_bug.cgi?id=1706967)
- [Mozilla Bugzilla #1708516 — GTS: Failure to provide regular and timely incident updates](https://bugzilla.mozilla.org/show_bug.cgi?id=1708516)
- [Mozilla Bugzilla #1731164 — GTS: CRL validity period set to expected value plus one second](https://bugzilla.mozilla.org/show_bug.cgi?id=1731164)
- [Mozilla Bugzilla #1735407 — Replace GTS LLC root certificates in NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=1735407)
- [Mozilla Bugzilla #1770510 — GTS: Failure to provide preliminary report within 24h](https://bugzilla.mozilla.org/show_bug.cgi?id=1770510)
- [Mozilla Bugzilla #1773556 — GTS: Incorrect OCSP responses for certain certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=1773556)
- [Mozilla Bugzilla #1793467 — GTS: Invalid CRL reason code](https://bugzilla.mozilla.org/show_bug.cgi?id=1793467)
- [Mozilla Bugzilla #1815874 — GTS: Incorrect SCT in certificate](https://bugzilla.mozilla.org/show_bug.cgi?id=1815874)
- [Mozilla Bugzilla #1882904 — GTS: Incorrect OCSP responses for new ICAs under test](https://bugzilla.mozilla.org/show_bug.cgi?id=1882904)
- [Mozilla Bugzilla #2031164 — GTS: Incomplete CRL Distribution Point URLs in CCADB](https://bugzilla.mozilla.org/show_bug.cgi?id=2031164)
- [Mozilla dev-security-policy — GTS root replacement discussion](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/lAu1_S48RAA)
- [Mozilla dev-security-policy — Replacement of GTS GlobalSign Root CA R2](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/ds3BLfZvRjg)
- [CCADB Case 00000666 (GTS replacement roots)](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000666)
- [Chrome Root Program Policy v1.8](https://googlechrome.github.io/chromerootprogram/)
- [The Hacker News — Google becomes its own Root Certificate Authority (2017)](https://thehackernews.com/2017/01/google-trust-services.html)
- [TechTarget — Google creates its own root certificate authority](https://www.techtarget.com/searchsecurity/news/450411906/Google-creates-its-own-root-certificate-authority)
