# WISeKey

WISeKey is a Swiss cybersecurity and digital-identity company (SIX: WIHN; Nasdaq: WKEY) headquartered in Geneva, Switzerland. Its roots in the Web PKI are owned by the OISTE Foundation — a Geneva-based non-profit with UN ECOSOC consultative status founded in 1998 — and operated commercially by WISeKey SA under a contractual Trust Framework. This folder merges all TLS-capable OISTE/WISeKey roots regardless of whether the `O=` field reads `OISTE Foundation` or `WISeKey`.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| OISTE WISeKey Global Root GB CA | `e73d606e.0` | RSA 2048 | 2039-12-01 | `6B:9C:08:E8:6E:B0:F7:67:CF:AD:65:CD:98:B6:21:49:E5:49:4A:67:F5:84:5E:7B:D1:ED:01:9F:27:B8:6B:D6` |
| OISTE WISeKey Global Root GC CA | `773e07ad.0` | ECC P-384 | 2042-05-09 | `85:60:F9:1C:36:24:DA:BA:95:70:B5:FE:A0:DB:E3:6F:F1:1A:83:23:BE:94:86:85:4F:B3:F3:4A:55:71:19:8D` |
| OISTE Server Root ECC G1 | `6805c744.0` | ECC P-384 | 2048-05-24 | `EE:C9:97:C0:C3:0F:21:6F:7E:3B:8B:30:7D:2B:AE:42:41:2D:75:3F:C8:21:9D:AF:D1:52:0B:25:72:85:0F:49` |
| OISTE Server Root RSA G1 | `30e1580d.0` | RSA 4096 | 2048-05-24 | `9A:E3:62:32:A5:18:9F:FD:DB:35:3D:FD:26:52:0C:01:53:95:D2:27:77:DA:C5:9D:B5:7B:98:C0:89:A6:51:E6` |

Notes:
- **OISTE WISeKey Global Root GB CA** (`O=WISeKey`): SHA-256 RSA root included in Mozilla, Microsoft, Apple, and Chrome root stores. EV-enabled. Replaced the older SHA-1 GA root for TLS use. [[Mozilla Bug 1172819]](https://bugzilla.mozilla.org/show_bug.cgi?id=1172819)
- **OISTE WISeKey Global Root GC CA** (`O=WISeKey`): ECC P-384 root included in Mozilla (Websites trust bit), Microsoft, Apple, and Chrome. The Email trust bit was removed at WISeKey's own request in NSS 3.98. [[Mozilla Bug 1403591]](https://bugzilla.mozilla.org/show_bug.cgi?id=1403591), [[Bug 1879945]](https://bugzilla.mozilla.org/show_bug.cgi?id=1879945)
- **OISTE Server Root ECC G1** and **OISTE Server Root RSA G1** (`O=OISTE Foundation`): New dedicated TLS server-authentication roots created May 2023. As of mid-2025 these roots were under CCADB public discussion for inclusion in major root programs (CCADB Case 00001946); individual root-store decisions are pending. [[CCADB public discussion]](https://www.mail-archive.com/public@ccadb.org/msg00476.html)

## Rationale for inclusion

The OISTE WISeKey Global Root GB CA and GC CA are included in all four major root programs (Mozilla, Microsoft, Apple, and Chrome/Android). The OISTE Foundation's ownership structure — a Swiss non-profit regulated under Swiss Civil Code Articles 80 et seq., with OISTE holding UN ECOSOC consultative status — provides a governance layer above the commercial operator WISeKey SA.

The newer OISTE Server Root ECC G1 and RSA G1 roots were created as dedicated single-purpose TLS roots consistent with CA/B Forum best-practice guidance on purpose-limited hierarchies. Public discussion in the CCADB program concluded August 2025 with no objections. [[CCADB Case 00001946 discussion]](https://www.mail-archive.com/public@ccadb.org/msg00476.html) [[CCADB discussion close]](http://www.mail-archive.com/public@ccadb.org/msg00489.html)

## CA/Browser Forum compliance

WISeKey undergoes annual third-party audits by **Auren** (formerly Auditoria) under the **WebTrust** framework, covering:

- WebTrust for Certification Authorities
- WebTrust for Baseline Requirements (SSL/TLS)
- WebTrust for Extended Validation (GB root)
- WebTrust for S/MIME Baseline Requirements (since 2024)
- WebTrust for Network Security (added 2025)

WISeKey publicly announced renewal of its WebTrust seal in August 2024 and July 2025. [[2024 renewal]](https://www.globenewswire.com/news-release/2024/08/06/2924724/0/en/WISeKey-Renews-WebTrust-Compliance-for-OISTE-WISeKey-Global-Root-of-Trust.html) [[2025 renewal]](https://www.globenewswire.com/news-release/2025/07/29/3122960/0/en/WISeKey-Renews-WebTrust-Compliance-for-OISTE-WISeKey-Global-Root-of-Trust.html)

The OISTE/WISeKey CP/CPS (v4.0, signed January 2025) is publicly available. From January 2025, the authoritative copy is maintained on GitHub at [github.com/OISTE/repository](https://github.com/OISTE/repository) with versioned releases; PDF copies are also published at [wisekey.com/repository](https://www.wisekey.com/repository/). [[CP/CPS v4.0 PDF]](https://cdn.wisekey.com/osite/uploads/20250120112333/Signed_OWGTMCPS_v4.0.pdf)

Certificate Transparency is required for all publicly-trusted TLS certificates. Intermediate CA retirement in 2019 was explicitly tied to enabling CT and automated CAA on the remaining hierarchy. [[Bug 1549308]](https://bugzilla.mozilla.org/show_bug.cgi?id=1549308)

## Past non-compliance

**2017 — Non-BR-compliant certificate issuance ([Bug 1391089](https://bugzilla.mozilla.org/show_bug.cgi?id=1391089)):** Two problematic certificates were identified. The first contained an internal/local DNS name caused by a bug in the portal's automated TLD validation that failed under certain SAN configurations; it was revoked on August 17, 2017. The second was a pre-certificate artifact generated during EV CT-log testing via a malfunctioning EJBCA patch that was not stored in the database; it was revoked on August 8, 2017. WISeKey also failed to respond to a problem report within 24 hours, though the cause (the email apparently not reaching their inbox) was not fully explained. Remediation included patching the TLD validator, introducing 100% manual review of SSL certificates, and migrating SSL issuance to the QuoVadis platform. The bug was resolved FIXED.

**2019 — Intermediate CA revocations ([Bug 1549308](https://bugzilla.mozilla.org/show_bug.cgi?id=1549308)):** WISeKey proactively revoked five intermediate CAs that lacked Certificate Transparency and automated CAA controls. Filed as an informational transparency notice; Mozilla closed it as INVALID because the revocations were not security- or compliance-driven and only required CCADB updates.

**2020 — Delayed revocation of intermediate CA with OCSP Signing EKU ([Bug 1651730](https://bugzilla.mozilla.org/show_bug.cgi?id=1651730)):** WISeKey failed to revoke an intermediate CA certificate containing an improper OCSP Signing EKU within the required 7-day window, citing a high impact on a healthcare customer. The delay was publicly noted as a compliance issue.

**2024 — OCSP "Unauthorized" for issued TLS certificate ([Bug 1903823](https://bugzilla.mozilla.org/show_bug.cgi?id=1903823)):** WISeKey's OCSP responder returned "Unauthorized" for a valid TLS pre-certificate due to a race condition in EJBCA during simultaneous certificate requests for the same user, leaving the certificate unpublished in the OCSP database. The community determined this was a compliance violation under the Mozilla Root Store Policy and CA/B Forum BRs. Remediated by manual publication, EJBCA configuration changes, and improved application-level concurrency handling. Resolved FIXED.

**2025 — S/MIME certificate issuance without proper validation ([Bug 1949755](https://bugzilla.mozilla.org/show_bug.cgi?id=1949755)):** A code update deployed January 10, 2025 to WISeKey's free S/MIME service broke server-side re-validation of the email address field, allowing DOM manipulation to issue a certificate to a different mailbox than the one validated. Four certificates were affected (all issued by the researcher). WISeKey disabled new issuances immediately, deployed a patch the same day, revoked all affected certificates, and committed to systematic peer code review for all changes. Resolved FIXED.

No distrust actions by any major root program have been taken against any WISeKey/OISTE root. For a current CCADB incident search, see: [https://www.ccadb.org/cas/updates](https://www.ccadb.org/cas/updates).

## Transparency

- **CP/CPS:** Publicly available at [wisekey.com/repository/cps-versions/](https://www.wisekey.com/repository/cps-versions/) and on [GitHub (OISTE/repository)](https://github.com/OISTE/repository) with full version history since January 2025.
- **Root certificates and CRL locations:** Published by the OISTE Foundation at [oiste.org/repository/root-ca-certificates-crl/](https://oiste.org/repository/root-ca-certificates-crl/).
- **CCADB disclosure:** OISTE Foundation is the CA Owner in CCADB; audits and intermediate CA records are updated annually. Intermediate CA revocations are disclosed in CCADB (see Bug 1549308 above).
- **Incident self-reporting:** WISeKey has self-reported all known incidents to Mozilla Bugzilla, including the 2024 OCSP bug and 2025 S/MIME issuance flaw.
- **Certificate Transparency:** Required for all TLS certificates. CAA domain identifiers include `wisekey.com` and `hightrusted.com`.

## Sources

- [OISTE Foundation — About](https://oiste.org/about-us/)
- [OISTE Foundation — Root Certificates & CRL Locations](https://oiste.org/repository/root-ca-certificates-crl/)
- [WISeKey — PKI Documentation / Repository](https://www.wisekey.com/repository/)
- [WISeKey — CPS Version History](https://www.wisekey.com/repository/cps-versions/)
- [OISTE/WISeKey Global Trust Model CP/CPS v4.0 (PDF, January 2025)](https://cdn.wisekey.com/osite/uploads/20250120112333/Signed_OWGTMCPS_v4.0.pdf)
- [Mozilla Bugzilla 1172819 — Add OISTE WISeKey Global Root GB CA](https://bugzilla.mozilla.org/show_bug.cgi?id=1172819)
- [Mozilla Bugzilla 1213042 — NSS inclusion of OISTE WISeKey Global Root GB CA](https://bugzilla.mozilla.org/show_bug.cgi?id=1213042)
- [Mozilla Bugzilla 1213044 — Enable OISTE WISeKey Global Root GB CA for EV](https://bugzilla.mozilla.org/show_bug.cgi?id=1213044)
- [Mozilla Bugzilla 1403591 — Add OISTE WISeKey Global Root GC CA](https://bugzilla.mozilla.org/show_bug.cgi?id=1403591)
- [Mozilla Bugzilla 1483924 — NSS inclusion of OISTE WISeKey Global Root GC CA](https://bugzilla.mozilla.org/show_bug.cgi?id=1483924)
- [Mozilla dev-security-policy — OISTE WISeKey Global Root GC CA Inclusion Request](https://groups.google.com/g/mozilla.dev.security.policy/c/36t-jbTQnTY)
- [Mozilla Bugzilla 1391089 — WISeKey: Non-BR-Compliant Certificate Issuance (2017)](https://bugzilla.mozilla.org/show_bug.cgi?id=1391089)
- [Mozilla Bugzilla 1549308 — WISeKey: Revocation of multiple intermediate CAs (2019)](https://bugzilla.mozilla.org/show_bug.cgi?id=1549308)
- [Mozilla Bugzilla 1651730 — WISeKey: Failure to revoke ICA with OCSP EKU within 7 days (2020)](https://bugzilla.mozilla.org/show_bug.cgi?id=1651730)
- [Mozilla Bugzilla 1653092 — Turn off Websites Trust Bit for OISTE WISeKey Global Root GA CA](https://bugzilla.mozilla.org/show_bug.cgi?id=1653092)
- [Mozilla Bugzilla 1879945 — Remove Email trust bit from OISTE WISeKey Global Root GC CA](https://bugzilla.mozilla.org/show_bug.cgi?id=1879945)
- [Mozilla Bugzilla 1903823 — WISeKey: OCSP "Unauthorized" for TLS certificate (2024)](https://bugzilla.mozilla.org/show_bug.cgi?id=1903823)
- [Mozilla Bugzilla 1949755 — WISeKey: S/MIME issuance without proper validation (2025)](https://bugzilla.mozilla.org/show_bug.cgi?id=1949755)
- [CCADB Public Discussion — OISTE CA Inclusion Request (Case 00001946, June 2025)](https://www.mail-archive.com/public@ccadb.org/msg00476.html)
- [CCADB — Close of OISTE CA Inclusion Public Discussion (August 2025)](http://www.mail-archive.com/public@ccadb.org/msg00489.html)
- [WISeKey — WebTrust Renewal Announcement (August 2024)](https://www.globenewswire.com/news-release/2024/08/06/2924724/0/en/WISeKey-Renews-WebTrust-Compliance-for-OISTE-WISeKey-Global-Root-of-Trust.html)
- [WISeKey — WebTrust Renewal Announcement (July 2025)](https://www.globenewswire.com/news-release/2025/07/29/3122960/0/en/WISeKey-Renews-WebTrust-Compliance-for-OISTE-WISeKey-Global-Root-of-Trust.html)
