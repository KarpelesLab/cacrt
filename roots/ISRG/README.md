# ISRG

The Internet Security Research Group (ISRG) is a California 501(c)(3) public-benefit nonprofit corporation, headquartered in San Francisco, CA, USA, founded in May 2013. ISRG operates **Let's Encrypt**, the world's largest publicly trusted certificate authority, which issues free, automated TLS server-authentication certificates. This folder covers only ISRG's own self-signed root certificates; IdenTrust's DST Root CA X3 (a historic cross-signer used during bootstrapping) is tracked separately under that operator.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| ISRG Root X1 | `4042bcee.0` | RSA 4096 | 2035-06-04 | `96:BC:EC:06:26:49:76:F3:74:60:77:9A:CF:28:C5:A7:CF:E8:A3:C0:AA:E1:1A:8F:FC:EE:05:C0:BD:DF:08:C6` |
| ISRG Root X2 | `0b9bc432.0` | ECC P-384 | 2040-09-17 | `69:72:9B:8E:15:A8:6E:FC:17:7A:57:AF:B7:17:1D:FC:64:AD:D2:8C:2F:CA:8C:F1:50:7E:34:45:3C:CB:14:70` |

Both roots are self-signed with `C=US, O=Internet Security Research Group`.

## Rationale for inclusion

ISRG Root X1 (RSA 4096) is trusted by all major root programs. Mozilla (NSS/Firefox 50, 2016) and Apple were the first; Microsoft completed the set [as of end of July 2018](https://letsencrypt.org/2018/08/06/trusted-by-all-major-root-programs), alongside Google, Oracle, and BlackBerry. The Mozilla inclusion process used the standard CA Application Process tracked in [Bugzilla #1204656](https://bugzilla.mozilla.org/show_bug.cgi?id=1204656) and [#1289889](https://bugzilla.mozilla.org/show_bug.cgi?id=1289889). Trust bit granted: Websites (TLS server authentication); EV was not requested.

ISRG Root X2 (ECDSA P-384) was submitted to Apple, Chrome, Google, Microsoft, Mozilla, and Oracle root programs in April 2021. Mozilla public discussion ran September–October 2021 ([Bugzilla #1701317](https://bugzilla.mozilla.org/show_bug.cgi?id=1701317)); NSS inclusion landed in September 2022 ([Bugzilla #1738805](https://bugzilla.mozilla.org/show_bug.cgi?id=1738805)). Trust stores for other programs were updated by approximately mid-2022.

Both roots carry the Websites (TLS server authentication) trust bit only.

## CA/Browser Forum compliance

ISRG undergoes annual [WebTrust for CAs](https://letsencrypt.org/repository/) audits performed by an independent, licensed WebTrust practitioner (Schellman & Company, LLC). The audit scope covers:

- **WebTrust Principles and Criteria for Certification Authorities** (v2.2.2, as of the 2025 audit cycle)
- **WebTrust for CAs — SSL Baseline with Network Security** (SSL Baseline v2.8, Network Security v1.7, as of the 2025 audit cycle)

Audit reports have been published without gap since 2015 and are available at [letsencrypt.org/repository/](https://letsencrypt.org/repository/). The combined CP/CPS (currently v6.1, May 2026) is publicly available and updated frequently; separate CP and CPS documents were maintained from 2015 until they were merged into a single document with v5.0 in May 2023. All root and subordinate CA certificates are disclosed in the [CCADB](https://ccadb.org/) public record.

Let's Encrypt is the originating implementer and primary driver of the **ACME** (Automatic Certificate Management Environment) protocol, standardized as [RFC 8555](https://datatracker.ietf.org/doc/html/rfc8555) (March 2019). All certificates are logged to public **Certificate Transparency** (CT) logs on a best-effort basis per ISRG's CP/CPS.

## Past non-compliance

ISRG has a public track record of self-disclosing incidents to Mozilla Bugzilla. Significant documented incidents include:

- **Incorrect OCSP responses under certain conditions (2019)** — Under a specific POST request pattern, the OCSP cache could return a valid but incorrect response, violating RFC 6960. [Bugzilla #1576789](https://bugzilla.mozilla.org/show_bug.cgi?id=1576789).
- **Expired OCSP signing certificate (2020)** — The ISRG Root OCSP X1 delegated signing certificate expired on 2020-06-04 and was not replaced for approximately five days, causing OCSP validation errors for chains to ISRG Root X1. Root cause: missing expiration monitoring. [Bugzilla #1645276](https://bugzilla.mozilla.org/show_bug.cgi?id=1645276).
- **Certificate lifetimes 90 days plus one second (2021)** — Certificates were issued with a validity of 90 days + 1 second, exceeding the permitted maximum, due to a discrepancy between CP and CPS. [Bugzilla #1715455](https://bugzilla.mozilla.org/show_bug.cgi?id=1715455).
- **Failure to revoke for the certificate lifetime incident (2021)** — ISRG decided not to mass-revoke the affected certificates, citing operational risk and the lack of automated replacement tooling in the Web PKI; this prompted development of the ACME Renewal Information (ARI) extension. [Bugzilla #1715672](https://bugzilla.mozilla.org/show_bug.cgi?id=1715672).
- **TLS-ALPN-01 challenge certificate contained additional identifiers (2022)** — A certificate validated via TLS-ALPN-01 was non-compliant with RFC 8737 because the challenge certificate's `subjectAltName` contained an IP address in addition to the `dNSName`. Revoked within hours of report; fix deployed the next day. [Bugzilla #1752670](https://bugzilla.mozilla.org/show_bug.cgi?id=1752670).
- **Duplicate serial numbers (2023)** — During a certificate profile configuration rollout, 645 certificates were issued with duplicate serial numbers due to routing requests to backend instances with different configurations during deployment. [Bugzilla #1838667](https://bugzilla.mozilla.org/show_bug.cgi?id=1838667).
- **No meaningful subject distinguished name (2024)** — 443,453 certificates issued between 2023-11-29 and 2024-09-27 had an empty Subject field, compliant with BR §7.1 but non-compliant with CP/CPS §3.1.2. 133,613 unexpired certificates were revoked. [Bugzilla #1921573](https://bugzilla.mozilla.org/show_bug.cgi?id=1921573).
- **Gen Y cross-certified subordinate CAs missing serverAuth EKU (2025)** — Three certificates in the new Generation Y hierarchy were missing required Extended Key Usage extensions and had non-compliant Subject Organization fields; revoked and replaced. [Bugzilla #2038351](https://bugzilla.mozilla.org/show_bug.cgi?id=2038351).

No distrust action has been taken against ISRG by any major root program. For a complete list of open and resolved incidents, see [Mozilla Bugzilla CA incidents for Let's Encrypt](https://bugzilla.mozilla.org/buglist.cgi?product=CA%20Program&component=CA%20Certificate%20Compliance&short_desc=Let%27s+Encrypt&short_desc_type=allwordssubstr).

## Transparency

- **CP/CPS**: The combined ISRG CP/CPS (currently v6.1) is publicly available at [letsencrypt.org/repository/](https://letsencrypt.org/repository/). Versions have been published without gap since May 2015.
- **WebTrust audits**: Annual reports are publicly posted in the same repository; the 2025 audit cycle covers WebTrust for CAs v2.2.2, SSL Baseline v2.8, and Network Security v1.7.
- **CCADB**: All ISRG root and intermediate certificates are disclosed in the Mozilla CCADB. Records are publicly searchable at [ccadb.org](https://www.ccadb.org/cas/mozilla).
- **Incident self-reporting**: ISRG self-discloses incidents to Mozilla Bugzilla (CA Program / CA Certificate Compliance component). A Compliance Rotation assigns weekly review of open Bugzilla tickets and mailing list posts.
- **Certificate Transparency**: All subscriber certificates are submitted to public CT logs per ISRG CP/CPS. ISRG also operates public CT logs.
- **Certificate repository**: Valid, expired, and revoked test pages are listed at [letsencrypt.org/certificates/](https://letsencrypt.org/certificates/).

## Sources

- [Let's Encrypt — Chains of Trust](https://letsencrypt.org/certificates/)
- [Let's Encrypt — Policy and Legal Repository (CP/CPS and audits)](https://letsencrypt.org/repository/)
- [ISRG CP/CPS v6.1](https://letsencrypt.org/documents/isrg-cp-cps-v6.1/)
- [Let's Encrypt trusted by all major root programs (2018)](https://letsencrypt.org/2018/08/06/trusted-by-all-major-root-programs)
- [Let's Encrypt — Generation Y hierarchy announcement (2025)](https://letsencrypt.org/2025/11/24/gen-y-hierarchy)
- [ISRG 10th Anniversary](https://letsencrypt.org/2023/05/24/isrg-10th-anniversary)
- [About ISRG — abetterinternet.org](https://www.abetterinternet.org/about/)
- [Let's Encrypt — Wikipedia](https://en.wikipedia.org/wiki/Let%27s_Encrypt)
- [Internet Security Research Group — Wikipedia](https://en.wikipedia.org/wiki/Internet_Security_Research_Group)
- [Mozilla Bugzilla #1204656 — Add ISRG / Let's Encrypt root certificate](https://bugzilla.mozilla.org/show_bug.cgi?id=1204656)
- [Mozilla Bugzilla #1289889 — Add ISRG Root X1 to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=1289889)
- [Mozilla Bugzilla #1701317 — ISRG Root X2 inclusion request](https://bugzilla.mozilla.org/show_bug.cgi?id=1701317)
- [Mozilla Bugzilla #1738805 — Add ISRG Root X2 root certificate to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=1738805)
- [Mozilla dev-security-policy — ISRG/Let's Encrypt inclusion request discussion](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/D8coPL0eU3k/m/bE_aRuWxCAAJ)
- [ISRG Root X2 submitted to root programs — Let's Encrypt Community](https://community.letsencrypt.org/t/isrg-root-x2-submitted-to-root-programs/149385)
- [Mozilla Bugzilla #1576789 — Incorrect OCSP responses (2019)](https://bugzilla.mozilla.org/show_bug.cgi?id=1576789)
- [Mozilla Bugzilla #1645276 — Expired ISRG Root OCSP X1 Certificate (2020)](https://bugzilla.mozilla.org/show_bug.cgi?id=1645276)
- [Mozilla Bugzilla #1715455 — Certificate lifetimes 90 days plus one second (2021)](https://bugzilla.mozilla.org/show_bug.cgi?id=1715455)
- [Mozilla Bugzilla #1715672 — Failure to revoke for Certificate Lifetime Incident (2021)](https://bugzilla.mozilla.org/show_bug.cgi?id=1715672)
- [Mozilla Bugzilla #1752670 — TLS-ALPN-01 allows additional identifiers (2022)](https://bugzilla.mozilla.org/show_bug.cgi?id=1752670)
- [Mozilla Bugzilla #1838667 — Duplicate Serial Numbers (2023)](https://bugzilla.mozilla.org/show_bug.cgi?id=1838667)
- [Mozilla Bugzilla #1921573 — No Meaningful Subject Distinguished Name (2024)](https://bugzilla.mozilla.org/show_bug.cgi?id=1921573)
- [Mozilla Bugzilla #2038351 — Gen Y Cross-Certified Subordinate CAs missing serverAuth EKU (2025)](https://bugzilla.mozilla.org/show_bug.cgi?id=2038351)
- [RFC 8555 — Automatic Certificate Management Environment (ACME)](https://datatracker.ietf.org/doc/html/rfc8555)
- [CCADB — Mozilla CA Certificates in Root Store](https://www.ccadb.org/cas/mozilla)
- [Let's Encrypt — Certificate Compatibility](https://letsencrypt.org/docs/certificate-compatibility/)
