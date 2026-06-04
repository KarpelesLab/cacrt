# BJCA

Beijing Certificate Authority Co., Ltd. (BJCA; Chinese: 北京数字认证股份有限公司) is a state-owned certificate authority established in February 2001 and headquartered in Beijing, China. It is one of the first CAs granted an electronic certification service licence by the Chinese government and operates a publicly-trusted "global certification system" that issues TLS/SSL and code-signing certificates in accordance with CA/Browser Forum Baseline Requirements and WebTrust standards. Both roots in this folder are owned by the same legal entity (Beijing Certificate Authority Co., Ltd.) under a single CA programme; no additional brands or subsidiaries are merged here.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| BJCA Global Root CA1 | `0179095f.0` | RSA 4096 | 2044-12-12 | `F3:89:6F:88:FE:7C:0A:88:27:66:A7:FA:6A:D2:74:9F:B5:7A:7F:3E:98:FB:76:9C:1F:A7:B0:9C:2C:44:D5:AE` |
| BJCA Global Root CA2 | `3e359ba6.0` | ECC P-384 | 2044-12-12 | `57:4D:F6:93:1E:27:80:39:66:7B:72:0A:FD:C1:60:0F:C2:7E:B6:6D:D3:09:29:79:FB:73:85:64:87:21:28:82` |

Both roots share the subject `C=CN, O=BEIJING CERTIFICATE AUTHORITY` and were generated on 2019-12-19.

## Rationale for inclusion

BJCA's roots were approved for Mozilla's NSS root store (CCADB Case #615, Bugzilla [#1647181](https://bugzilla.mozilla.org/show_bug.cgi?id=1647181)), with inclusion landing in NSS 3.89.1 / Firefox 114 (2023). EV treatment was subsequently enabled via [Bug #1822924](https://bugzilla.mozilla.org/show_bug.cgi?id=1822924). Because Chrome and Android derive their default trust from NSS/Mozilla's list, the roots are also trusted in those environments; Android's CA certificate bundle was updated to include them at the NSS 3.91 RTM update ([AOSP commit 60163cca](https://android.googlesource.com/platform/system/ca-certificates/+/60163ccad5880d940c93d78e5e4ae7fa22634912)). The roots are **not** confirmed as included in the Apple or Microsoft root programs as of the date of this writing — Apple's iOS 18 / macOS 15 trust store (version 2024051500) does not list BJCA, and no public announcement from the Microsoft Trusted Root Program has been found.

Trust bits approved by Mozilla: TLS server authentication (Websites) and S/MIME (Email). Scope is TLS server certificates globally, not restricted to Chinese domains.

## CA/Browser Forum compliance

BJCA's global certification system is audited annually against:

- **WebTrust Principles and Criteria for Certification Authorities** (baseline CA audit)
- **WebTrust Principles and Criteria for CAs — SSL Baseline with Network Security** (covering TLS issuance)
- **WebTrust Principles and Criteria for CAs — Code Signing Baseline Requirements** (separate scope)

Audits for the period 10 March 2023 – 9 March 2024 were conducted by Anthony Kam & Associates, Ltd. (Hong Kong) against WebTrust SSL Baseline with Network Security criteria v2.6, confirming that BJCA disclosed SSL certificate lifecycle practices in its CP/CPS and provided services in conformity with CA/Browser Forum Baseline Requirements. [[WebTrust seal, CPA Canada](https://www.cpacanada.ca/api/getPDFWebTrust?attachmentId=fc7e75ff-2a6c-4287-9944-d5783b8676e4)] [[WebTrust seal for prior period](http://download.bjca.org.cn/download/WTCA_SealFile/BJCA_WTCA_SSLBR_SealFile_2021.pdf)]

For the period 10 March 2024 – 9 March 2025, a code-signing audit was performed by an auditor based in Taipei against WebTrust Code Signing Baseline Requirements v3.2. [[WebTrust seal, CPA Canada](https://www.cpacanada.ca/api/getPDFWebTrust?attachmentId=2237365d-b594-4ed3-bd2a-e89dd10527da)]

BJCA discloses its **BJCA Global Certificate Policy (CP)** and **BJCA Global Certification Practice Statement (CPS)** on its website and in CCADB, as required. The CP/CPS in force at the time of inclusion was v1.0.6 (dated 2022-07-25). [[CCADB public discussion](https://groups.google.com/a/ccadb.org/g/public/c/o9lbCbr92Ug/m/lPkqrHF1DQAJ)]

During Mozilla's policy review (Bug #1647181, Comments 7 and 24), Ben Wilson identified numerous CP/CPS gaps (email verification, validity periods exceeding 397 days, missing pre-issuance linting, network security detail, EV fields). BJCA addressed these across CPS revisions v1.0.2 through v1.0.6 and all flagged items were marked "Fixed" before approval. The process of iterative remediation is documented in the bug thread.

## Past non-compliance

No certificate mis-issuance incident, revocation failure, or distrust action by a major root program has been publicly documented for BJCA as of the date of this writing. The following concerns were raised during the Mozilla inclusion review and are publicly documented:

**1. Silent root certificate installation by associated software (2022–2023)**
During the public discussion, BJCA acknowledged that its "certificate environment" software installed root certificates without explicit user confirmation, stating the software "chooses to skip user confirmation during the installation process." Community members including Prof. Joel Reardon characterised this as inconsistent with the responsibility expected of a globally-trusted CA. BJCA committed to adding user-confirmation options in future software versions. This was a concern about non-CA-software practice, not a BR violation by the CA itself. [[CCADB public list](https://groups.google.com/a/ccadb.org/g/public/c/o9lbCbr92Ug/m/lPkqrHF1DQAJ)] [[dev-security-policy thread](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/loH2352Ik6E)]

**2. "Beijing One Pass" spyware allegation (2021, raised during 2022–2023 inclusion review)**
Recorded Future's Insikt Group published report CTA-CN-2021-0729 finding that the "Beijing One Pass" employee benefits application — developed by BJCA — exhibited characteristics consistent with spyware, including keystroke capture, screenshot recording, clipboard access, persistence mechanisms, and a backdoor driver (wmControl.exe). BJCA stated the flagged behaviours were attributed to a USB token driver from a third-party manufacturer, that the driver had been removed in software ≥ v3.6.8, and that the application belonged to a separate "national trusted source certification system" with no shared infrastructure with the globally-trusted roots. Mozilla's root programme manager concluded there was insufficient evidence to classify One Pass as malware. The inclusion request was approved. [[Recorded Future report](https://www.recordedfuture.com/research/beijing-one-pass-benefits-software-spyware)] [[The Record news article](https://therecord.media/spyware-features-found-in-chinese-state-benefits-app)] [[dev-security-policy thread](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/loH2352Ik6E)]

For completeness: a [Bugzilla search for BJCA incidents](https://bugzilla.mozilla.org/buglist.cgi?short_desc=BJCA&query_format=specific&order=Importance&no_redirect=1) and the [CCADB public list search](https://groups.google.com/a/ccadb.org/g/public/search?q=BJCA) return no post-inclusion incident reports as of this writing.

## Transparency

- **CP/CPS**: BJCA publishes its Global CP and Global CPS on its website ([bjca.cn](https://www.bjca.cn/en/)) and discloses URLs in CCADB, as required by Mozilla policy.
- **CCADB disclosure**: BJCA is a participating CA in CCADB (Case #615). Audit reports, CP/CPS URLs, and intermediate CA records are required to be kept current.
- **WebTrust seal files**: Publicly accessible via CPA Canada's website (links in the CA/B Forum compliance section above) and BJCA's own download server.
- **Certificate Transparency**: TLS certificates issued from the publicly-trusted hierarchy are subject to CT logging as mandated by the CA/Browser Forum Baseline Requirements (BR §4.9.12 / SC-62). No BJCA-operated CT log has been identified; BJCA relies on third-party logs as is standard practice.
- **OCSP/CRL**: Revocation services are operated for all issued certificates per BR requirements. OCSP and CRL endpoints are embedded in issued certificates.

## Sources

- [Mozilla Bugzilla #1647181 — BJCA root inclusion request](https://bugzilla.mozilla.org/show_bug.cgi?id=1647181)
- [Mozilla Bugzilla #1822921 — Add BJCA roots to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=1822921)
- [CCADB public discussion of BJCA inclusion request](https://groups.google.com/a/ccadb.org/g/public/c/o9lbCbr92Ug/m/lPkqrHF1DQAJ)
- [Mozilla dev-security-policy: Public Discussion re: Beijing CA (BJCA)](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/loH2352Ik6E)
- [BJCA official website](https://www.bjca.cn/en/)
- [WebTrust assurance report 2023–2024 (CPA Canada)](https://www.cpacanada.ca/api/getPDFWebTrust?attachmentId=fc7e75ff-2a6c-4287-9944-d5783b8676e4)
- [WebTrust SSL BR seal file 2021 (bjca.org.cn)](http://download.bjca.org.cn/download/WTCA_SealFile/BJCA_WTCA_SSLBR_SealFile_2021.pdf)
- [WebTrust code signing assurance report 2024–2025 (CPA Canada)](https://www.cpacanada.ca/api/getPDFWebTrust?attachmentId=2237365d-b594-4ed3-bd2a-e89dd10527da)
- [Recorded Future / Insikt Group: "Beijing One Pass" Employee Benefits Software Exhibits Spyware Characteristics (CTA-CN-2021-0729)](https://www.recordedfuture.com/research/beijing-one-pass-benefits-software-spyware)
- [The Record: Spyware features found in Chinese state benefits app](https://therecord.media/spyware-features-found-in-chinese-state-benefits-app)
- [AOSP ca-certificates commit adding BJCA roots (NSS 3.91)](https://android.googlesource.com/platform/system/ca-certificates/+/60163ccad5880d940c93d78e5e4ae7fa22634912)
- [Apple iOS 18 / macOS 15 trusted root certificate list](https://support.apple.com/en-us/121672)
- [Beijing Certificate Authority — Cloud Signature Consortium member page](https://cloudsignatureconsortium.org/member/beijing-certificate-authority/)
