# SECOM Trust Systems

SECOM Trust Systems CO., LTD. is a Japanese information-security company headquartered in Mitaka, Tokyo, and a wholly owned subsidiary of SECOM CO., LTD. Founded in 1985, it operates public root certification authorities under the "Security Communication" brand and was the first company in Japan to obtain WebTrust Certification (2004). It is the only Japanese company that participates in the CA/Browser Forum and runs a public root CA from its own data center in Japan. All roots in this folder are operated by SECOM Trust Systems CO., LTD.; no separate legal entities or merged brands are represented.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| Security Communication ECC RootCA1 | `5860aaa6.0` | ECC P-384 | 2038-01-18 | `E7:4F:BD:A5:5B:D5:64:C4:73:A3:6B:44:1A:A7:99:C8:A6:8E:07:74:40:E8:28:8B:9F:A1:E5:0E:4B:BA:CA:11` |
| Security Communication RootCA2 | `cd58d51e.0` | RSA 2048 | 2029-05-29 | `51:3B:2C:EC:B8:10:D4:CD:E5:DD:85:39:1A:DF:C6:C2:DD:60:D8:7B:B7:36:D2:B5:21:48:4A:A4:7A:0E:BE:F6` |

## Rationale for inclusion

Both roots are included in all four major root programs:

- **Mozilla NSS / Firefox:** Security Communication RootCA2 has been trusted since Firefox 11 (first included circa 2011, originally as a SHA-256 successor to the earlier SHA-1 RootCA1). Security Communication ECC RootCA1 was approved in August 2022 and shipped in NSS 3.83 / Firefox 106, with website and email trust bits. The email trust bit for ECC RootCA1 was subsequently removed in NSS 3.108 (January 2025, [Bug 1939086](https://bugzilla.mozilla.org/show_bug.cgi?id=1939086)). Both roots are currently included for TLS server authentication. ([Bug 1313982](https://bugzilla.mozilla.org/show_bug.cgi?id=1313982))
- **Microsoft Trusted Root Program:** Both roots appear in Microsoft deployment notices, including the [October 2021](https://learn.microsoft.com/en-us/security/trusted-root/2021/october2021) and [August 2025](https://learn.microsoft.com/en-us/security/trusted-root/2025/august-2025) notices.
- **Apple and Google Chrome:** SECOM confirmed both roots were already embedded in Apple and Chrome products at the time of the Mozilla EV review process.

Trust scope is TLS server authentication (and historically email/S/MIME). No EV designation was requested for the ECC RootCA1 or RootCA3 inclusion; RootCA2 has EV enabled ([Bug 1096205](https://bugzilla.mozilla.org/show_bug.cgi?id=1096205)).

## CA/Browser Forum compliance

- **Audit framework:** SECOM conducts annual WebTrust for CAs and WebTrust for Baseline Requirements audits. The auditor has been KPMG in recent cycles (confirmed for the period ending June 6, 2021); earlier cycles used PricewaterhouseCoopers Aarata LLC. ([Bug 1717044](https://bugzilla.mozilla.org/show_bug.cgi?id=1717044), [Bug 1313982](https://bugzilla.mozilla.org/show_bug.cgi?id=1313982))
- **Baseline Requirements:** The CP/CPS states explicitly that it conforms to the CA/Browser Forum Baseline Requirements and that, in any inconsistency, the BRs take precedence. ([SECOM Publicly Trusted TLS CP/CPS](https://repo1.secomtrust.net/spcpp/publicly-trusted-cpcps/SECOM_Publicly_Trusted_TLS_CP-CPS_EN.html))
- **CCADB disclosure:** SECOM maintains records in the CCADB and resolved CCADB ALV (Audit Letter Validation) errors identified in 2021. ([Bug 1717044](https://bugzilla.mozilla.org/show_bug.cgi?id=1717044))
- **ACME:** SECOM introduced an ACME-based automation mechanism for certificate management and revocation as a remediation measure following the 2020 delayed-revocation incident. ([Bug 1652610](https://bugzilla.mozilla.org/show_bug.cgi?id=1652610))
- **Certificate Transparency:** CT logging is required for publicly trusted TLS certificates under the BR and root-program policies; SECOM's CP/CPS framework is consistent with these requirements, though individual CT log configurations are not itemized in the publicly available documentation reviewed.
- **CA/Browser Forum membership:** SECOM participates in the CA/Browser Forum as a CA member.

## Past non-compliance

The following incidents are publicly documented in Mozilla Bugzilla. No distrust action has been taken against SECOM Trust Systems roots; however, the pattern of delayed revocations drew sustained criticism from Mozilla reviewers.

1. **Mis-issued EV certificates (2019) — [Bug 1576133](https://bugzilla.mozilla.org/show_bug.cgi?id=1576133):** SECOM mis-issued 12 EV certificates with incorrect values in the `serialNumber` field of the Subject DN (test/operation-confirmation certificates with placeholder registration numbers instead of actual corporate registration numbers, violating EV Guidelines §9.2.5). All certificates were revoked within approximately two days of notification (August 19–21, 2019). Remediation included multi-person verification of all Subject DN fields. Marked RESOLVED FIXED.

2. **Incorrect OCSP Delegated Responder Certificate (2020) — [Bug 1649962](https://bugzilla.mozilla.org/show_bug.cgi?id=1649962):** SECOM issued OCSP Delegated Responder certificates without the `id-pkix-ocsp-nocheck` extension, as required by the Baseline Requirements. Discovered July 2, 2020. Revocation was addressed under Bug 1652610.

3. **Delayed Revocation of CA Certificates with OCSP EKU Issue (2020) — [Bug 1652610](https://bugzilla.mozilla.org/show_bug.cgi?id=1652610):** SECOM failed to revoke affected intermediate CA certificates (primarily JPRS Organization Validation Authority - G3 and JPRS Domain Validation Authority - G3) within the BR-required 7-day window. SECOM's revocation plan extended approximately 8–9 months, with the final revocations and key destructions completed March 2021. The delay was attributed to manual certificate replacement processes at hosting providers. Mozilla reviewers noted the plan was "substantially longer than almost every other CA affected." SECOM subsequently implemented ACME-based automation. Marked RESOLVED FIXED.

4. **Outdated audit statements for intermediate certificates (2021) — [Bug 1695993](https://bugzilla.mozilla.org/show_bug.cgi?id=1695993):** Two JPRS intermediate CA certificates issued July 10, 2020, were omitted from the audit scope because SECOM had decided not to use them due to a serial-number entropy concern. The certificates were revoked on March 9, 2021, after the issue was reported on March 3, 2021. Whiteboard: `[ca-compliance] [audit-failure]`. Marked RESOLVED FIXED.

5. **Intermediate CA Certificates Missing from Audit Reports (2021) — [Bug 1717044](https://bugzilla.mozilla.org/show_bug.cgi?id=1717044):** SECOM discovered 17 intermediate CA certificates were missing from its 2020 WebTrust audit reports due to a misunderstanding of audit scope requirements. SECOM had only confirmed TLS-issuing CAs and excluded CAs that had not issued serverAuth certificates in the prior year. Four certificates were revoked and keys destroyed; approximately 12 were re-issued with constraining EKUs. A new KPMG WebTrust audit covering all certificates was completed and submitted to CPA Canada on September 3, 2021. Marked RESOLVED FIXED.

6. **Delayed Revocation of non-technically-constrained FUJIFILM Certificates (2021) — [Bug 1707229](https://bugzilla.mozilla.org/show_bug.cgi?id=1707229):** 127 end-entity certificates issued by FUJIFILM Fnet CA required revocation; SECOM missed the 5-day deadline, revoking 10 days after the deadline began (April 26, 2021, versus a deadline of April 21, 2021). The delay was attributed to a policy restricting certificate maintenance to weekends. Mozilla reviewers noted this policy made BR-compliant revocation structurally impossible. Remediation included FUJIFILM migrating to a non-public CA (operational March 7, 2022). Marked RESOLVED FIXED.

No distrust of SECOM roots has been enacted by any major root program as of the date of this writing. For an exhaustive current view, see the [CCADB incident search](https://ccadb.my.salesforce-sites.com/mozilla/AllProblemReportingMechanismsReport) and [Mozilla Bugzilla CA compliance queries](https://bugzilla.mozilla.org/buglist.cgi?product=CA%20Program&component=CA%20Certificate%20Compliance&status_whiteboard=SECOM).

## Transparency

- **CP/CPS:** SECOM publishes CP/CPS documents publicly in English. As of May 26, 2025, legacy policy documents (including the Security Communication RootCA CPS Ver. 6.06 and SECOM CA Service CPS Ver. 2.22) were consolidated into a single integrated [SECOM Publicly Trusted TLS CP/CPS](https://repo1.secomtrust.net/spcpp/publicly-trusted-cpcps/SECOM_Publicly_Trusted_TLS_CP-CPS_EN.html). The CP/CPS is reviewed and revised at least annually.
- **Root CA repositories:** Certificate, CRL, and policy documents for Security Communication RootCA2 are at [repository.secomtrust.net/SC-Root2/](https://repository.secomtrust.net/SC-Root2/) and for ECC RootCA1 at [repository.secomtrust.net/SC-ECC-Root1/](https://repository.secomtrust.net/SC-ECC-Root1/index.html).
- **CCADB:** SECOM maintains entries in CCADB for its included roots and has resolved past ALV errors. The externally operated S/MIME subordinate CA (Cybertrust Japan SureMail CA G5 under SECOM's hierarchy) was disclosed to CCADB and underwent a public discussion period in December 2024 – January 2025. ([CCADB public discussion](https://www.mail-archive.com/public@ccadb.org/msg00400.html))
- **Incident self-reporting:** All incidents listed above were filed in Mozilla Bugzilla with incident reports from SECOM. The 2021 audit-scope issues (Bugs 1695993 and 1717044) were partly self-discovered via CCADB ALV error notifications.
- **Certificate Transparency:** Certificates issued under these roots are subject to CT logging requirements under CA/Browser Forum Baseline Requirements and root-program CT policies.
- **Test websites:** SECOM operates test URLs for ECC RootCA1: valid at `sr5v.secom-cert.jp`, revoked at `sr5r.secom-cert.jp`, expired at `sr5e.secom-cert.jp`.

## Sources

- [SECOM Trust Systems — Digital Certificate Services](https://www.secomtrust.net/english/certificate_authority.html)
- [SECOM Publicly Trusted TLS CP/CPS (EN)](https://repo1.secomtrust.net/spcpp/publicly-trusted-cpcps/SECOM_Publicly_Trusted_TLS_CP-CPS_EN.html)
- [SECOM CA Service CPS Ver. 2.22](https://repo1.secomtrust.net/spcpp/cps/SECOM-CPS-EN.pdf)
- [Security Communication RootCA CPS Ver. 6.06](https://repository.secomtrust.net/SC-Root/SCRootCPS-EN.pdf)
- [Security Communication RootCA2 Repository](https://repository.secomtrust.net/SC-Root2/)
- [Security Communication ECC RootCA1 Repository](https://repository.secomtrust.net/SC-ECC-Root1/index.html)
- [Mozilla Bugzilla Bug 1313982 — Add SECOM root certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=1313982)
- [Mozilla Bugzilla Bug 1576133 — SECOM: Mis-issued EV Certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=1576133)
- [Mozilla Bugzilla Bug 1649962 — SECOM: Incorrect OCSP Delegated Responder Certificate](https://bugzilla.mozilla.org/show_bug.cgi?id=1649962)
- [Mozilla Bugzilla Bug 1652610 — SECOM: Delayed Revocation of CA Certificate with OCSP EKU Issue](https://bugzilla.mozilla.org/show_bug.cgi?id=1652610)
- [Mozilla Bugzilla Bug 1695993 — SECOM: Outdated audit statements for intermediate certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=1695993)
- [Mozilla Bugzilla Bug 1707229 — SECOM: Delayed Revocation of non-technically constrained FUJIFILM Certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=1707229)
- [Mozilla Bugzilla Bug 1717044 — SECOM: Intermediate CA Certificates Missing from Audit Reports](https://bugzilla.mozilla.org/show_bug.cgi?id=1717044)
- [Mozilla Bugzilla Bug 1939086 — Security Communication ECC RootCA1 email trust bit removal](https://bugzilla.mozilla.org/show_bug.cgi?id=1939086)
- [Mozilla Bugzilla Bug 1410544 — Remove "Security Communication EV RootCA1" root cert](https://bugzilla.mozilla.org/show_bug.cgi?id=1410544)
- [Mozilla Bugzilla Bug 1096205 — Enable EV for Security Communication RootCA2](https://bugzilla.mozilla.org/show_bug.cgi?id=1096205)
- [Microsoft Trusted Root Program — October 2021 Deployment Notice](https://learn.microsoft.com/en-us/security/trusted-root/2021/october2021)
- [Microsoft Trusted Root Program — August 2025 Deployment Notice](https://learn.microsoft.com/en-us/security/trusted-root/2025/august-2025)
- [CCADB Public Discussion — SECOM Externally-Operated S/MIME CA (December 2024)](https://www.mail-archive.com/public@ccadb.org/msg00400.html)
- [SECOM Root Inclusion Request — mozilla.dev.security.policy](https://groups.google.com/g/mozilla.dev.security.policy/c/oGEsyvsuhP8)
- [SECOM CO., LTD. — Corporate History / Milestones](https://www.secom.co.jp/english/corporate/vision/history.html)
