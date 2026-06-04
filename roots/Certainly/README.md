# Certainly

Certainly LLC is a wholly owned subsidiary of Fastly, Inc., a content delivery network company headquartered in San Francisco, California, USA. Certainly operates as Fastly's own publicly trusted Certification Authority, issuing Domain Validation (DV) TLS server certificates exclusively to Fastly customers for websites and API endpoints served through the Fastly CDN. Both roots (RSA and ECDSA hierarchies) are operated under the single legal entity Certainly LLC; no separate brands are merged here.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| Certainly Root E1 | `8508e720.0` | ECC P-384 | 2046-04-01 | `B4:58:5F:22:E4:AC:75:6A:4E:86:12:A1:36:1C:5D:9D:03:1A:93:FD:84:FE:BB:77:8F:A3:06:8B:0F:C4:2D:C2` |
| Certainly Root R1 | `7a780d93.0` | RSA 4096 | 2046-04-01 | `77:B8:2C:D8:64:4C:43:05:F7:AC:C5:CB:15:6B:45:67:50:04:03:3D:51:C6:0C:62:02:A8:E0:C3:34:67:D3:A0` |

## Rationale for inclusion

Both roots carry the **websites (TLS server authentication) trust bit only**; no S/MIME or code-signing trust is requested or granted. They were approved for inclusion in the Mozilla NSS root store and shipped in **Firefox 103** (released July 2022) following an uncontested public discussion. Fastly has confirmed inclusion in the **Apple** and **Google Chrome** root stores as well. The roots bootstrap an ACME-driven DV-only hierarchy; intermediates were additionally cross-signed by GoDaddy (Starfield Root Certificate Authority – G2) to extend backward compatibility while the own roots propagated.

- Mozilla inclusion bug: [Bug 1727941](https://bugzilla.mozilla.org/show_bug.cgi?id=1727941)
- Public discussion (dev-security-policy): [Certainly's Root Inclusion Request](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/EhXhiHfWGC8)
- GoDaddy cross-sign discussion: [Bug 1755851](https://bugzilla.mozilla.org/show_bug.cgi?id=1755851)
- Fastly announcement: [Announcing Certainly](https://www.fastly.com/blog/announcing-certainly-fastlys-own-tls-certification-authority)

## CA/Browser Forum compliance

Certainly operates under the **WebTrust for Certification Authorities** audit framework:

- Initial point-in-time audits (dated 30 June 2021) were conducted by **Schellman & Company** under WebTrust Principles and Criteria for CAs v. 2.2.1 and WebTrust SSL Baseline with Network Security v. 2.5.
- Annual period-of-time audits have continued; the 2023–2024 period audit (covering July 1, 2023 – June 30, 2024) was conducted under WebTrust Principles and Criteria for CAs v. 2.2.2.
- Audit reports are published at [certainly.com/repository](https://www.certainly.com/repository/).

Certainly's CP/CPS (current version 2.1) is publicly available and commits to the CA/Browser Forum **Baseline Requirements** for TLS and to Mozilla Root Store Policy requirements. All records are disclosed in the **CCADB** (Case 829). Certificates are issued via **ACME** using the open-source **Boulder** CA software (maintained by Let's Encrypt). All issued certificates are logged to public **Certificate Transparency** logs as required by the BRs and browser policies. Certainly issues DV certificates only, with a default **30-day validity period** (the shortest default in the industry at launch) to reduce exposure from compromised certificates.

- CP/CPS: [certainly.com/repository/CertainlyCP-CPS.pdf](https://www.certainly.com/repository/CertainlyCP-CPS.pdf)
- Audit index: [certainly.com/repository/audit/](https://www.certainly.com/repository/audit/index.html)
- CCADB included certificate report: [Mozilla CA Certificates in Firefox Report](https://ccadb.my.salesforce-sites.com/mozilla/CACertificatesInFirefoxReport)

## Past non-compliance

Two publicly documented compliance incidents have been resolved; no distrust action has been taken against Certainly.

**1. Root CRL validity period exceeds maximum by one second (2021)**
Certainly discovered that both root CRLs (valid April 1, 2021 – April 1, 2022) exceeded the Baseline Requirements maximum validity period by one second. Root cause was an overly simplistic interpretation of the 12-month limit. Corrected CRLs with a 364-day interval were published October 29, 2021. No certificates were misissued. Certainly also helped draft CAB Forum ballot SC52 to clarify the BR language.
- Source: [Bug 1732745](https://bugzilla.mozilla.org/show_bug.cgi?id=1732745)

**2. TLS-ALPN-01 validation bug (January 2022)**
Certainly, running Boulder, was affected by the same TLS-ALPN-01 validation flaw disclosed by Let's Encrypt (Bug 1751984), in which Boulder did not verify the TLS version used during validation. Certainly declared an incident on January 25, 2022, disabled new TLS-ALPN-01 validations within ~100 minutes, deployed the Boulder fix, and revoked all 337,621 affected unexpired certificates within 24 hours of declaring the incident. During remediation, 8 additional certificates were inadvertently issued from pre-existing (pre-patch) authorizations before those authorizations were revoked; all 8 were promptly revoked. Certainly subsequently documented formal procedures for faster issuance halts and revocation of existing authorizations. All affected certificates were from internal system testing and were not issued to third-party customers. Bug resolved February 16, 2022.
- Source: [Bug 1752452](https://bugzilla.mozilla.org/show_bug.cgi?id=1752452)

**3. CRL intermixing issue (discovered May 2024)**
The 2023–2024 WebTrust audit documented an incident in which the E1 and R1 CRLs became intermixed: Certainly sometimes served the wrong CRL (R1 when E1 was expected) and also served CRLs that, while valid, did not contain the most recent revocations. The issue was discovered on May 29, 2024 and corrected approximately two hours later, with affected CRL shards manually updated.
- Source: [WebTrust for CAs Audit Report 2023–2024](https://www.cpacanada.ca/api/getPDFWebTrust?attachmentId=43fedd00-a8c7-4aa1-80e4-05bc4ccd57d6)

A search of Mozilla Bugzilla for open or unresolved Certainly incidents found no additional reported issues beyond those above. No distrust or removal action has been taken by any root program.

## Transparency

- **CP/CPS** is publicly available at [certainly.com/repository](https://www.certainly.com/repository/) (current version 2.1; prior versions archived).
- **CCADB disclosure**: Certainly is fully disclosed in the CCADB under Case 829; records include audit reports, CP/CPS versions, and intermediate CA certificates.
- **Incident self-reporting**: Both known incidents (Bugs 1732745 and 1752452) were self-reported to Mozilla Bugzilla in a timely manner.
- **Certificate Transparency**: All issued certificates are submitted to CT logs as required; Certainly runs Boulder, which implements CT precertificate submission natively.
- **Problem reporting**: cert-prob-reports@certainly.com (published in the repository).

## Sources

- [Certainly – Fastly's Certification Authority (certainly.com)](https://www.certainly.com/)
- [Certainly repository / CP-CPS](https://www.certainly.com/repository/)
- [Mozilla Bug 1727941 – Add Certainly R1 and E1 Root Certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=1727941)
- [Mozilla Bug 1755851 – GoDaddy cross-signing two Certainly Intermediate Certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=1755851)
- [Mozilla Bug 1732745 – Certainly: Root CRL validity period exceeds maximum by one second](https://bugzilla.mozilla.org/show_bug.cgi?id=1732745)
- [Mozilla Bug 1752452 – Certainly: TLS Using ALPN TLS Version and OID](https://bugzilla.mozilla.org/show_bug.cgi?id=1752452)
- [dev-security-policy: Public Discussion of Certainly's Root Inclusion Request](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/EhXhiHfWGC8)
- [dev-security-policy: Public Discussion of GoDaddy cross-signing Certainly Intermediates](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/bEnn98Dajzc/m/4DnwaOBnAwAJ)
- [Fastly Blog: Announcing Certainly, Fastly's own TLS Certification Authority](https://www.fastly.com/blog/announcing-certainly-fastlys-own-tls-certification-authority)
- [CCADB: Mozilla CA Certificates in Firefox Report](https://ccadb.my.salesforce-sites.com/mozilla/CACertificatesInFirefoxReport)
- [WebTrust for CAs Audit Report 2023–2024 (CPA Canada)](https://www.cpacanada.ca/api/getPDFWebTrust?attachmentId=43fedd00-a8c7-4aa1-80e4-05bc4ccd57d6)
