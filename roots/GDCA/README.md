# GDCA

GDCA (Guang Dong Certificate Authority), now formally named **Global Digital Cybersecurity Authority Co., Ltd.** (数安时代科技股份有限公司), is a Chinese publicly-trusted certificate authority headquartered in Guangdong province, China, operating under China's Electronic Signature Law. It serves business corporations, government agencies, and individuals registered in mainland China, and holds Web PKI trust through Mozilla, Microsoft, and Google Android root programs. The single folder entry covers both the former "GUANG DONG CERTIFICATE AUTHORITY CO.,LTD." and the post-2016 "Global Digital Cybersecurity Authority Co., Ltd." legal entity, which are the same organisation after a corporate rename.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| GDCA TrustAUTH R5 ROOT | `0f6fa695.0` | RSA 4096 | 2040-12-31 | `BF:FF:8F:D0:44:33:48:7D:6A:8A:A6:0C:1A:29:76:7A:9F:C2:BB:B0:5E:42:0F:71:3A:13:B9:92:89:1D:38:93` |

## Rationale for inclusion

The GDCA TrustAUTH R5 ROOT is included in the **Mozilla NSS / Firefox** trust store (first shipped in NSS 3.34 / Firefox 58, with EV enabled in Firefox 60), in the **Microsoft Trusted Root Certificate Program** (added July 2020), and in the **Google Android** system CA store (Android 9 / Pie). As of the CCADB CA Certificates in Firefox report (checked 2026-06-04), the root remains trusted for TLS server authentication and EV in Firefox. The EV Policy OID is `1.2.156.112559.1.1.6.1`. The root was not found on the CA/Root CA Lifecycles distrust schedule at the time of writing. No confirmed inclusion in the Apple root store was found; sources from 2018 indicated an application had been submitted but not yet completed.

**Sources:** [Mozilla Bugzilla #1128392](https://bugzilla.mozilla.org/show_bug.cgi?id=1128392) · [Mozilla Bugzilla #1385063](https://bugzilla.mozilla.org/show_bug.cgi?id=1385063) · [Mozilla Bugzilla #1385065](https://bugzilla.mozilla.org/show_bug.cgi?id=1385065) · [CCADB CA Certificates in Firefox](https://ccadb.my.salesforce-sites.com/mozilla/CACertificatesInFirefoxReport) · [Microsoft Trusted Root July 2020](https://learn.microsoft.com/en-us/security/trusted-root/2020/july2020)

## CA/Browser Forum compliance

Audits are performed annually by **PricewaterhouseCoopers Zhong Tian LLP** under the **WebTrust for CAs**, **WebTrust for SSL Baseline Requirements**, and **WebTrust EV SSL** criteria. The most recently referenced audit period end date available in CCADB audit reminder emails was 2021-02-28. GDCA's CPS commits to the CA/Browser Forum Baseline Requirements and states that in the event of a conflict, the Baseline Requirements govern. CP and CPS documents (including EV CP/CPS) are published on the GDCA website at `https://www.gdca.com.cn/customer_service/knowledge_universe/cp_cps/`; as of February 2025 the EV CPS was at v3.0 and the standard CPS at v4.4. CCADB disclosure is maintained under the CA owner name "Global Digital Cybersecurity Authority Co., Ltd. (Formerly Guang Dong Certificate Authority (GDCA))". No ACME or public CT-policy disclosures were found in the sources reviewed; Certificate Transparency is required by BR/Chrome policy for all certificates issued from this root.

**Sources:** [CCADB Included CA Certificate List](https://ccadb.my.salesforce-sites.com/mozilla/includedcacertificatereport) · [CCADB Audit Reminder Email (dev-security-policy)](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/FYkfVYnBK5I) · [GDCA CP/CPS index](https://www.gdca.com.cn/customer_service/knowledge_universe/cp_cps/)

## Past non-compliance

Two publicly-documented mis-issuance incidents are on record; no distrust action has been identified in the sources reviewed.

1. **RSA-1024 key-size mis-issuance (2018) — [Mozilla Bugzilla #1467414](https://bugzilla.mozilla.org/show_bug.cgi?id=1467414):** Seven DV TLS certificates containing 1024-bit RSA keys (below the BR minimum of 2048 bits) were issued between December 2017 and June 2018. Root cause was a configuration mismatch between test and production environments introduced during a system upgrade. GDCA revoked all affected certificates within two days of being notified (June 7–8, 2018), fixed the production template, and deployed pre-issuance linting (cablint, x509lint, zlint). Resolved FIXED by Mozilla reviewer Wayne Thayer.

2. **Incorrect organizationName in EV certificate (2020) — [Mozilla Bugzilla #1662382](https://bugzilla.mozilla.org/show_bug.cgi?id=1662382):** One EV certificate was issued on August 25, 2020, with a mis-entered organizationName value. GDCA identified it through a routine internal audit the following day and revoked it within roughly 35 minutes of identification. Root cause was sole reliance on manual validation; remediation was a Certificate Management System feature cross-checking the field against a Qualified Government Information Source. Mozilla reviewer Ryan Sleevi described it as "an exemplary incident report." Resolved FIXED.

No distrust or removal action against GDCA was found in the Mozilla Root CA Lifecycles schedule, Mozilla Additional Trust Changes wiki, or recent root-program announcements reviewed. For a live check, see the [CCADB Included CA Certificate List](https://ccadb.my.salesforce-sites.com/mozilla/includedcacertificatereport) and [Mozilla Root CA Lifecycles](https://wiki.mozilla.org/CA/Root_CA_Lifecycles).

## Transparency

- **CP/CPS:** Published in Chinese (primary) and English at [gdca.com.cn](https://www.gdca.com.cn/customer_service/knowledge_universe/cp_cps/); EV CPS v3.0 and standard CPS v4.4 were available as of February 2025.
- **CCADB:** Root is disclosed under CA owner "Global Digital Cybersecurity Authority Co., Ltd. (Formerly Guang Dong Certificate Authority (GDCA))"; Standard, BR, and EV audit records are tracked there.
- **Incident self-reporting:** Both documented incidents (Bugzilla #1467414 and #1662382) include GDCA-authored incident reports with timelines, root-cause analyses, and remediation steps.
- **Certificate Transparency:** Certificates from this root are subject to the CT logging requirements of the CA/Browser Forum Baseline Requirements and Chrome CT Policy.

## Sources

- [Mozilla Bugzilla #1128392 — Add GDCA Root Certificate (inclusion request)](https://bugzilla.mozilla.org/show_bug.cgi?id=1128392)
- [Mozilla Bugzilla #1385063 — Add GDCA TrustAUTH R5 ROOT to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=1385063)
- [Mozilla Bugzilla #1385065 — Enable GDCA TrustAUTH R5 ROOT for EV in PSM](https://bugzilla.mozilla.org/show_bug.cgi?id=1385065)
- [Mozilla Bugzilla #1467414 — GDCA: Misissuance of certificates with small RSA keys](https://bugzilla.mozilla.org/show_bug.cgi?id=1467414)
- [Mozilla Bugzilla #1662382 — GDCA: Incorrect Value in organizationName Field](https://bugzilla.mozilla.org/show_bug.cgi?id=1662382)
- [CCADB CA Certificates in Firefox Report](https://ccadb.my.salesforce-sites.com/mozilla/CACertificatesInFirefoxReport)
- [CCADB Included CA Certificate List](https://ccadb.my.salesforce-sites.com/mozilla/includedcacertificatereport)
- [CCADB Audit Reminder Email Summary — dev-security-policy](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/FYkfVYnBK5I)
- [Mozilla Root CA Lifecycles wiki](https://wiki.mozilla.org/CA/Root_CA_Lifecycles)
- [Mozilla Additional Trust Changes wiki](https://wiki.mozilla.org/CA/Additional_Trust_Changes)
- [mozilla.dev.security.policy — GDCA root inclusion request (Google Groups)](https://groups.google.com/g/mozilla.dev.security.policy/c/kB2JrygK7Vk/m/kULupXMFCQAJ)
- [Microsoft Trusted Root Program — July 2020 Deployment Notice](https://learn.microsoft.com/en-us/security/trusted-root/2020/july2020)
- [GDCA website — CP/CPS index (gdca.com.cn)](https://www.gdca.com.cn/customer_service/knowledge_universe/cp_cps/)
