# iTrusChina

iTrusChina Co., Ltd. (天威诚信数字技术有限公司) is a Chinese certificate authority founded in 2000, headquartered in Haidian District, Beijing, China. Approved by the Chinese Ministry of Industry and Information Technology and the State Cryptography Administration, it is one of the earliest publicly trusted CAs in China. Its publicly trusted TLS roots are issued under the **vTrus** brand.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| vTrus Root CA | `7a3adc42.0` | RSA 4096 | 2043-07-31 | `8A:71:DE:65:59:33:6F:42:6C:26:E5:38:80:D0:0D:88:A1:8D:A4:C6:A9:1F:0D:CB:61:94:E2:06:C5:C9:63:87` |
| vTrus ECC Root CA | `ed858448.0` | ECC P-384 | 2043-07-31 | `30:FB:BA:2C:32:23:8E:2A:98:54:7A:F9:79:31:E5:50:42:8B:9B:3F:1C:8E:EB:66:33:DC:FA:86:C5:B2:7D:D3` |

Both roots were generated on 2018-07-31 and are self-signed.

## Rationale for inclusion

Both vTrus roots are included in the **Mozilla/NSS root store** (shipped in NSS 3.74 / Firefox 97, approved via [Bugzilla #1554846](https://bugzilla.mozilla.org/show_bug.cgi?id=1554846) and added to NSS via [Bugzilla #1740095](https://bugzilla.mozilla.org/show_bug.cgi?id=1740095)). Both roots carry the Websites trust bit and are EV-enabled (EV Policy OID `2.23.140.1.1`) per [Bugzilla #1740099](https://bugzilla.mozilla.org/show_bug.cgi?id=1740099). The CCADB inclusion case is [Case #00000431](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000431).

No independent public documentation of explicit inclusion in the Apple, Microsoft, or Chrome root stores was found at the time of writing; coverage in those programs could not be confirmed from available sources.

The roots are scoped to TLS server authentication (the Websites trust bit) and Extended Validation. No externally operated subordinate CAs or external registration authorities are disclosed in the CCADB record.

## CA/Browser Forum compliance

iTrusChina is audited annually under the **WebTrust for CAs**, **WebTrust for CAs — SSL Baseline with Network Security**, and **WebTrust for CAs — Extended Validation SSL** schemes. The most recently reported audit period per the CCADB record is 2025-01-09 through 2026-01-08, with an audit statement dated 2026-03-11 by **SunRise CPAs / DFK International**. Earlier audits (period ending March 2021) were performed by **PricewaterhouseCoopers Zhong Tian LLP** [[public discussion](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/4bWqkOwhzCc)].

The combined Certificate Policy and Certification Practice Statement (CP/CPS v1.6.3, effective 2026-03-12) is publicly available at the [iTrusChina repository](https://www.itrus.com.cn/repository). iTrusChina commits to the CA/Browser Forum Baseline Requirements and EV Guidelines.

Certificate Transparency: iTrusChina embeds SCTs in issued certificates as required by the CA/Browser Forum Baseline Requirements and Mozilla policy. No company-operated CT log was identified in public sources.

## Past non-compliance

Three publicly documented issues have been filed in Mozilla Bugzilla since inclusion:

1. **CRL/ARL signature verification errors (2021)** — During the root inclusion public discussion, Andrew Ayer reported that iTrusChina's Authority Revocation Lists (ARLs) failed signature verification, reproducible via `openssl` and crt.sh. Root cause was a design bug in iTrusChina's in-house CA software, which omitted extensions from the signed TBSCertList. iTrusChina remediated the software and added a monitoring system. The issue was tracked and resolved in [Bugzilla #1712664](https://bugzilla.mozilla.org/show_bug.cgi?id=1712664).

2. **Missing 2018 KGC and GAP-period audit reports (identified 2024)** — In April 2024, the Chrome Root Program team identified that iTrusChina had not obtained a Key Generation Ceremony audit (2018-07-31) or a GAP-period audit (2018-07-31 to 2018-10-07), violating TLS BR §§6.1.1.1 and 8.1 which require consecutive audit coverage from key generation. Root cause was inexperience with WebTrust requirements at the time of key generation in 2018. Supplementary KGC and GAP audit reports were completed by July and September 2024 respectively. Tracked in [Bugzilla #1923279](https://bugzilla.mozilla.org/show_bug.cgi?id=1923279).

3. **Issuance of certificates using previously reported compromised keys (2024)** — On 2024-10-26, Google notified iTrusChina that it had issued certificates reusing public keys previously flagged as compromised (BR §6.1.1.3). Investigation confirmed 41 certificates were affected: 8 keys had been incorrectly marked as `keyCompromise` due to a system bug (pre-September 2022), then their public keys were later reused for test website certificates. The keys were not genuinely compromised. All 41 certificates were revoked; staff training and system updates were implemented. Tracked in [Bugzilla #1927384](https://bugzilla.mozilla.org/show_bug.cgi?id=1927384).

No distrust action has been taken against any iTrusChina/vTrus root in any public root program at the time of writing.

## Transparency

- **CP/CPS**: Publicly available in English and Chinese at [https://www.itrus.com.cn/repository](https://www.itrus.com.cn/repository). Current version: v1.6.3 (effective 2026-03-12). The document notes that neither the English nor Chinese version is subordinate; both are maintained to be materially equivalent (a condition required during Mozilla inclusion review).
- **CCADB disclosure**: Both roots are disclosed in the CCADB; [Case #00000431](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000431) is publicly viewable.
- **Audit statements**: Four WebTrust seals are published on the iTrusChina repository page, verified via CPA Canada at `https://www.cpacanada.ca/webtrustseal?sealid=11954` through `...11957`.
- **Incident reporting**: iTrusChina has filed incident reports in Mozilla Bugzilla for all three issues noted above, consistent with Mozilla policy requirements.
- **Certificate problem reports**: iTrusChina publishes a problem-report endpoint at [https://www.itrus.com.cn/baogao/index.htm](https://www.itrus.com.cn/baogao/index.htm).

## Sources

- [Mozilla Bugzilla #1554846 — Add iTrusChina root certificate(s)](https://bugzilla.mozilla.org/show_bug.cgi?id=1554846)
- [Mozilla Bugzilla #1740095 — Add iTrusChina root certificates to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=1740095)
- [Mozilla Bugzilla #1740099 — Enable EV Treatment for iTrusChina's vTrus Root CA and vTrus ECC Root CA](https://bugzilla.mozilla.org/show_bug.cgi?id=1740099)
- [Mozilla Bugzilla #1712664 — iTrusChina: verification errors for the roots' CRLs (ARL)](https://bugzilla.mozilla.org/show_bug.cgi?id=1712664)
- [Mozilla Bugzilla #1923279 — iTrusChina: lacking 2018 KGC and GAP period audit report](https://bugzilla.mozilla.org/show_bug.cgi?id=1923279)
- [Mozilla Bugzilla #1927384 — iTrusChina: Issuance of certificates using keys previously reported as compromised](https://bugzilla.mozilla.org/show_bug.cgi?id=1927384)
- [Mozilla dev-security-policy: Public Discussion re: Inclusion of the iTrusChina Root CAs](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/4bWqkOwhzCc)
- [CCADB Case #00000431 — iTrusChina Co., Ltd.](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000431)
- [iTrusChina Certificate Repository (CP/CPS, audit seals)](https://www.itrus.com.cn/repository)
- [iTrusChina Global-Trust CP and CPS v1.6.3 (PDF)](https://www.itrus.com.cn/upload/file/20260311/1773211207826059054.pdf)
