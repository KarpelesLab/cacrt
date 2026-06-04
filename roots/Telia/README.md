# Telia

Telia Finland Oyj (Business ID 1475607-9, Helsinki, Finland) is a subsidiary of the Swedish telecommunications group Telia Company AB, and is the legal entity responsible for the Telia Certificate Authority. The CA delivers publicly trusted TLS server certificates to organisations in Finland, Sweden, Norway, Denmark, Estonia, Latvia, and Lithuania; it succeeds earlier roots issued under the TeliaSonera and Sonera brands (merged 2002). Registration Authority services are split between Telia Finland Oyj (FI, EE, LT) and Cygate AB (SE, NO, DK), both wholly owned subsidiaries of Telia Company AB.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| Telia Root CA v2 | 8f103249.0 | RSA 4096 | 2043-11-29 | `24:2B:69:74:2F:CB:1E:5B:2A:BF:98:89:8B:94:57:21:87:54:4E:5B:4D:99:11:78:65:73:62:1F:6A:74:B8:2C` |

## Rationale for inclusion

Telia Root CA v2 is included in all four major root programmes: Mozilla NSS ([bug 1751298](https://bugzilla.mozilla.org/show_bug.cgi?id=1751298)), Google Chrome ([issue 230517132](https://issuetracker.google.com/issues/230517132)), Apple, and Microsoft. The root replaced the predecessor TeliaSonera Root CA v1 (valid to 2032) to update the legal entity name from `O=TeliaSonera` to `O=Telia Finland Oyj` and extend validity to 2043 in line with CA/Browser Forum Baseline Requirements. The CCADB record ([case 00000660](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000660)) records trust bits for Websites and Email (S/MIME). The root covers TLS server authentication as its primary Web-PKI use.

## CA/Browser Forum compliance

- **Audit framework:** ETSI EN 319 411-1 (TLS BR) and ETSI EN 319 401 (Standard), audited by datenschutz cert GmbH. The most recent audit period is 1 November 2024 – 31 October 2025, with statements dated 17 December 2025 and no deviations reported. Earlier years were audited to WebTrust for CAs v2.2 and WebTrust for BRs v2.4 criteria by KPMG.
- **Baseline Requirements:** The CP/CPS explicitly states that in the event of any inconsistency with the CA/Browser Forum Baseline Requirements, the Requirements take precedence.
- **CCADB disclosure:** All publicly trusted CA certificates are published to CCADB within 7 days of issuance per the CP/CPS.
- **ACME:** Telia operates an ACME service supporting `http-01` (BR 3.2.2.4.19 / RFC 8555 §8.3) for DV and OV certificates; wildcard names are excluded.
- **Certificate Transparency:** OV and DV certificates are submitted to CT logs as required by Chromium policy; the CP/CPS requires CT logging for TLS certificates.
- **CP/CPS:** Published at [cps.trust.telia.com](https://cps.trust.telia.com/TLSv65/teliacatlscpcps.html) (TLS) and in the [Telia repository](https://repository.trust.teliasonera.com/).

## Past non-compliance

Several Baseline Requirements violations have been documented against the prior TeliaSonera/Telia hierarchy (under TeliaSonera Root CA v1) and were the subject of Mozilla Bugzilla incident bugs. The main incidents are:

- **[Bug 1524567](https://bugzilla.mozilla.org/show_bug.cgi?id=1524567)** – Certificates with invalid IP values in the SAN dNSName field, issued 2016; not revoked promptly despite the underlying code defect having been fixed in January 2017.
- **[Bug 1524050](https://bugzilla.mozilla.org/show_bug.cgi?id=1524050)** – Mis-issued certificate with a space in the dNSName; revocation occurred more than five days after the incident report, violating BR §4.9.5. Mozilla commented that the response implied Telia considered BR compliance optional.
- **[Bug 1528261](https://bugzilla.mozilla.org/show_bug.cgi?id=1528261)** – Certificate with a bare hostname (no TLD); validation code had detected the invalidity in logs but incorrectly accepted the CSR.
- **[Bug 1551372](https://bugzilla.mozilla.org/show_bug.cgi?id=1551372)** – `stateOrProvinceName` field contained the literal string `Some-State` or the invalid value `Finland`; Mozilla raised concerns about Telia's reliance on single-person manual review and lack of an adequate incident response process.
- **[Bug 1689589](https://bugzilla.mozilla.org/show_bug.cgi?id=1689589)** – Leaf certificate using the disallowed ECDSA P-521 curve, caused by a gap between the ACME code path and the curve-filtering logic.
- **[Bug 1736020](https://bugzilla.mozilla.org/show_bug.cgi?id=1736020)** – Incorrect email address used for domain validation in multi-SAN requests; related to [bug 1737808](https://bugzilla.mozilla.org/show_bug.cgi?id=1737808) for delayed revocation.
- **[Bug 1475115](https://bugzilla.mozilla.org/show_bug.cgi?id=1475115)** / **[Bug 1565270](https://bugzilla.mozilla.org/show_bug.cgi?id=1565270)** – Qualified WebTrust audit statements for 2018 and 2019, with multiple qualifications in the BR report; Mozilla required point-in-time and then 6-month-cycle period-of-time audits during remediation.

All of the above incidents were listed in Attachment B of the WebTrust BR audit report covering the 2020–2021 period and were reported as resolved at the time of the Telia Root CA v2 inclusion request. The current CCADB audit record shows no deviations. No distrust action has been taken against Telia Root CA v2 itself. For a full incident search see: [Bugzilla Telia CA issues](https://bugzilla.mozilla.org/buglist.cgi?query_format=advanced&product=CA%20Program&component=CA%20Certificate%20Compliance&status_whiteboard=Telia&status_whiteboard_type=allwordssubstr).

## Transparency

- **CP/CPS:** Publicly available at [cps.trust.telia.com](https://cps.trust.telia.com/TLSv65/teliacatlscpcps.html) (current TLS version) and archived versions at [repository.trust.teliasonera.com](https://repository.trust.teliasonera.com/). Documents follow RFC 3647 structure with no blank sections.
- **CCADB:** All subordinate CA certificates are disclosed to CCADB within 7 days of issuance; the root record is [case 00000660](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000660).
- **Incident self-reporting:** Incidents listed above were filed and tracked in Mozilla Bugzilla under the CA Program product; the CA provided incident reports and corrective-action timelines in each bug, though Mozilla noted delayed updates in some cases.
- **Certificate Transparency:** TLS certificates are logged to publicly operated CT logs per Chromium CT policy; certificate issuance activity is visible via [crt.sh](https://crt.sh/?CAName=Telia+Finland+Oyj) and Cloudflare Radar.

## Sources

- [Mozilla Bugzilla 1664161 – Add Telia CA root certificate (assessment)](https://bugzilla.mozilla.org/show_bug.cgi?id=1664161)
- [Mozilla Bugzilla 1751298 – Add Telia Root CA v2 to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=1751298)
- [Mozilla dev-security-policy: Public Discussion: Inclusion of Telia Root CA v2](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/52Gfr4dnJD8)
- [Google Chrome issue tracker 230517132 – Telia Root CA v2 inclusion request](https://issuetracker.google.com/issues/230517132)
- [CCADB case 00000660 – Telia Root CA v2](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000660)
- [Telia CP/CPS for TLS Server Certificates (current)](https://cps.trust.telia.com/TLSv65/teliacatlscpcps.html)
- [Telia Certificate Services Repository](https://repository.trust.teliasonera.com/)
- [Telia Root CPS v2.7 (PDF)](https://repository.trust.teliasonera.com/Telia_Root_CPS_v2.7.pdf)
- [Mozilla Bugzilla 1524567 – Invalid IP in SAN DNS field](https://bugzilla.mozilla.org/show_bug.cgi?id=1524567)
- [Mozilla Bugzilla 1524050 – Mis-issued certificate: invalid dnsName](https://bugzilla.mozilla.org/show_bug.cgi?id=1524050)
- [Mozilla Bugzilla 1528261 – FQDN without domain part](https://bugzilla.mozilla.org/show_bug.cgi?id=1528261)
- [Mozilla Bugzilla 1551372 – "Some-State" in stateOrProvinceName](https://bugzilla.mozilla.org/show_bug.cgi?id=1551372)
- [Mozilla Bugzilla 1689589 – Disallowed P-521 curve in leaf certificate](https://bugzilla.mozilla.org/show_bug.cgi?id=1689589)
- [Mozilla Bugzilla 1736020 – Invalid email used for domain validation](https://bugzilla.mozilla.org/show_bug.cgi?id=1736020)
- [Mozilla Bugzilla 1475115 – Qualified audit statements 2018](https://bugzilla.mozilla.org/show_bug.cgi?id=1475115)
- [Mozilla Bugzilla 1565270 – Qualified BR audit statement 2019](https://bugzilla.mozilla.org/show_bug.cgi?id=1565270)
- [Telia Company – Wikipedia](https://en.wikipedia.org/wiki/Telia_Company)
