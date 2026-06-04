# TrustAsia

TrustAsia Technologies, Inc. is a Shanghai-based certificate authority incorporated in China in April 2013, operating under the legal name TrustAsia Technologies, Inc. (亚洲诚信). It issues publicly trusted TLS server authentication certificates primarily for the Chinese and Asia-Pacific markets, holds an Electronic Authentication Service License from China's State Cryptography Administration (OSCCA, licence no. 0060) and a License for Electronic Certification Service Provider from the Ministry of Industry and Information Technology (no. ECP31010421056), and is also one of a small number of independent Google Certificate Transparency log operators.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| TrustAsia Global Root CA G3 | `9bf03295.0` | RSA 4096 | 2046-05-19 | `E0:D3:22:6A:EB:11:63:C2:E4:8F:F9:BE:3B:50:B4:C6:43:1B:E7:BB:1E:AC:C5:C3:6B:5D:5E:C5:09:03:9A:08` |
| TrustAsia Global Root CA G4 | `1cef98f5.0` | ECC P-384 | 2046-05-19 | `BE:4B:56:CB:50:56:C0:13:6A:52:6D:F4:44:50:8D:AA:36:A0:B5:4F:42:E4:AC:38:F7:2A:F4:70:E4:79:65:4C` |
| TrustAsia TLS ECC Root CA | `2ccbdda3.0` | ECC P-384 | 2044-05-15 | `C0:07:6B:9E:F0:53:1F:B1:A6:56:D6:7C:4E:BE:97:CD:5D:BA:A4:1E:F4:45:98:AC:C2:48:98:78:C9:2D:87:11` |
| TrustAsia TLS RSA Root CA | `b0d5255e.0` | RSA 4096 | 2044-05-15 | `06:C0:8D:7D:AF:D8:76:97:1E:B1:12:4F:E6:7F:84:7E:C0:C7:A1:58:D3:EA:53:CB:E9:40:E2:EA:97:91:F4:C3` |

## Rationale for inclusion

**TrustAsia Global Root CA G3 and G4** were approved for inclusion in Mozilla's root store (websites + email trust bits, EV-enabled) following a six-week public discussion that closed on 16 August 2023 with no objections. The roots were added to NSS 3.95, shipping in Firefox 121 (December 2023). Mozilla Bugzilla [bug 1688854](https://bugzilla.mozilla.org/show_bug.cgi?id=1688854) tracks the inclusion request.

**TrustAsia TLS RSA Root CA and TrustAsia TLS ECC Root CA** were approved by the Chrome Root Program on 24 June 2025 following a six-week CCADB public discussion (Case 00002095, opened 21 April 2025, closed 2 June 2025) that received no objections. Chrome Root Store inclusion was scheduled for the release of approximately 28 July 2025. The Chrome Root Program decision was announced via the CCADB public list ([announcement](https://www.mail-archive.com/public@ccadb.org/msg00467.html)).

All four roots are audited under WebTrust criteria and are limited to TLS server authentication (and, for G3/G4, S/MIME). TrustAsia is a member of the PKI Consortium.

## CA/Browser Forum compliance

TrustAsia's CA operations are audited annually by **Anthony Kam & Associates Ltd.** (Hong Kong) under the following WebTrust criteria:

- WebTrust for Certification Authorities
- WebTrust for CAs — Network Security
- WebTrust for CAs — SSL/TLS Baseline Requirements (CA/B Forum BR)
- WebTrust for CAs — SSL/TLS Extended Validation Guidelines (EVG)
- WebTrust for CAs — S/MIME Baseline Requirements

The most recent audit period on record covers **1 August 2023 to 31 July 2024**, with a Key Generation ceremony report dated **15 May 2024**. The CCADB self-assessment (v1.4.1, October 2024) reported no Bugzilla incidents in the prior 24 months at the time of submission.

The CP&CPS for the global hierarchy (current version V1.6.1) is publicly available at [repository.trustasia.com](https://repository.trustasia.com/repo/cps/TrustAsia-Global-CP-CPS_EN_V1.6.1.pdf) and commits to the CA/Browser Forum Baseline Requirements for TLS and S/MIME. TrustAsia supports ACME-based certificate issuance and requires Certificate Transparency logging for all issued TLS certificates.

## Past non-compliance

One publicly documented incident is on record:

**ACME Authorization Reuse Non-Compliance (2026-01-21) — [Bugzilla #2011713](https://bugzilla.mozilla.org/show_bug.cgi?id=2011713)**
A code defect deployed on 2025-12-29 caused domain validation records in TrustAsia's LiteSSL ACME service to be reused across different ACME accounts, in violation of TLS BR §3.2.2.4. The flaw was identified on 2026-01-21 via a third-party community report on v2ex.com. TrustAsia immediately suspended the ACME issuance service, revoked all 143 affected DV certificates (within the required window), and deployed a fix enforcing mandatory ACME account isolation. The bug was resolved as FIXED approximately two months after opening. Root causes included missing account-isolation enforcement in the validation-record retrieval query and insufficient adversarial QA test coverage.

No distrust actions by any root program have been taken against TrustAsia's roots. No other incidents or mis-issuance events are documented in publicly available Bugzilla or dev-security-policy archives as of June 2026. For a current search, see the [Mozilla CA Incident Dashboard](https://wiki.mozilla.org/CA/Incident_Dashboard) and [Bugzilla CA:TrustAsia](https://bugzilla.mozilla.org/buglist.cgi?product=NSS&component=CA%20Certificates&summary=TrustAsia).

## Transparency

- **CP&CPS:** Publicly available at [repository.trustasia.com](https://repository.trustasia.com/); the global CP&CPS is versioned and updated when policy changes occur.
- **CCADB:** TrustAsia maintains entries in the Common CA Database for all roots and their issuing intermediates.
- **Audit reports:** WebTrust audit seals and full assurance reports are published via the CCADB and on CPA Canada's WebTrust portal.
- **Incident self-reporting:** The 2026 ACME incident (Bugzilla #2011713) was self-reported to Mozilla and a full incident report was filed.
- **Certificate Transparency:** TrustAsia operates one of the five independent Google-recognized CT logs ([ct.trustasia.com](https://ct.trustasia.com/blog/english/)) in addition to requiring CT logging for certificates it issues as a CA.

## Sources

- [TrustAsia PKI Consortium member profile](https://pkic.org/members/trustasia/)
- [TrustAsia Global Trusted CA Service Repository](https://repository.trustasia.com/)
- [TrustAsia Global CP&CPS V1.6.1 (PDF)](https://repository.trustasia.com/repo/cps/TrustAsia-Global-CP-CPS_EN_V1.6.1.pdf)
- [Mozilla Bugzilla #1688854 — Add TrustAsia root certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=1688854)
- [CCADB Public Discussion of TrustAsia CA Inclusion Request (Mozilla, 2023)](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/3mAiS5jIkNw)
- [CCADB Public Discussion of TrustAsia CA Inclusion Request (2025)](https://groups.google.com/a/ccadb.org/g/public/c/clTruHb98og)
- [Chrome Root Store Inclusion Decision — TrustAsia Technologies, Inc. (June 2025)](https://www.mail-archive.com/public@ccadb.org/msg00467.html)
- [Mozilla Bugzilla #2011713 — TrustAsia: ACME Authorization Reuse Non-Compliance](https://bugzilla.mozilla.org/show_bug.cgi?id=2011713)
- [NSS 3.95 Release Notes (TrustAsia G3/G4 added)](https://firefox-source-docs.mozilla.org/security/nss/releases/nss_3_95.html)
- [TrustAsia CT Log](https://ct.trustasia.com/blog/english/)
- [Cloudflare Radar — TrustAsia TLS RSA Root CA CT transparency](https://radar.cloudflare.com/certificate-transparency/ca/06C08D7DAFD876971EB1124FE67F847EC0C7A158D3EA53CBE940E2EA9791F4C3)
- [Mozilla CA Incident Dashboard](https://wiki.mozilla.org/CA/Incident_Dashboard)
