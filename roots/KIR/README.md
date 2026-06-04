# KIR

Krajowa Izba Rozliczeniowa S.A. (KIR) is Poland's national interbank clearing house, founded in 1992 and headquartered in Warsaw. In addition to operating the ELIXIR and Express ELIXIR payment systems, KIR provides trust services under the **SZAFIR** brand (Centrum Obsługi Podpisu Elektronicznego / Electronic Signature Service Center), including qualified certificates, electronic seals, and TLS server certificates. The single root in this folder anchors the public-facing TLS hierarchy; an older root (SZAFIR ROOT CA, not included here) has been separately disabled by Microsoft.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| SZAFIR ROOT CA2 | `fe8a2cd8.0` | RSA 2048 | 2035-10-19 | `A1:33:9D:33:28:1A:0B:56:E5:57:D3:D3:2B:1C:E7:F9:36:7E:B0:94:BD:5F:A7:2A:7E:50:04:C8:DE:D7:CA:FE` |

## Rationale for inclusion

SZAFIR ROOT CA2 is included in the **Mozilla NSS** root store (trust bits: Websites, Email) following an inclusion request approved circa 2016. The root replaced the original SZAFIR ROOT CA, which was rejected from NSS due to an oversized serial number violating the 20-byte NSS limit; the CA2 replacement carries a SHA-256 self-signature and a conforming serial number.

The root is used to anchor TLS server authentication certificates (issued exclusively by the subordinate Szafir Trusted CA2). KIR's Mozilla CCADB entry lists it under CA Owner "Krajowa Izba Rozliczeniowa S.A." with the Websites trust bit enabled. The root does not appear to be independently included in the Apple or Chrome root programs as of this writing (no public announcement found); its Microsoft status differs—see below.

Sources: [Mozilla Bugzilla #817994][bz817994]; [Mozilla Bugzilla #1157375][bz1157375]; [mozilla.dev.security.policy inclusion discussion][mdsp1]; [CCADB public list][ccadb].

## CA/Browser Forum compliance

KIR's non-qualified trusted certificate hierarchy is audited annually by **Ernst & Young Poland** against **WebTrust for CAs** and **WebTrust for Baseline Requirements (SSL)**. WebTrust seals are published on the KIR certificate repository (elektronicznypodpis.pl) and verified via CPA Canada. The CPS explicitly commits to compliance with the CA/Browser Forum Baseline Requirements and Network and Certificate System Security Requirements.

TLS certificates are issued only by Szafir Trusted CA2; the CPS confirms Certificate Transparency logging for all TLS certificates in accordance with the requirements of certificate-transparency.org (section 4.4.2 of the CPS). The CPS also notes that KIR may use the ACME protocol for domain validation on TLS certificates.

Sources: [KIR CPS for trusted non-qualified certificates (April 2024)][cps2024]; [mozilla.dev.security.policy inclusion discussion][mdsp1].

## Past non-compliance

The following publicly documented incidents are on record; no distrust of SZAFIR ROOT CA2 has been found in Mozilla's root store.

**1. Multiple Baseline Requirements violations (2018) — [Bugzilla #1495497][bz1495497]**
In October 2018, automated linting identified approximately 46 certificates carrying BR violations (invalid postalAddress content, SANs containing e-mail addresses, missing stateOrProvince/locality fields). KIR had not implemented pre-issuance linting and was externally alerted. During the resolution period a further misissued pre-certificate was logged without disclosure to Mozilla, prompting additional criticism. KIR committed to pre-issuance linting and updated certificate templates.

**2. OCSP "unknown" responses (2019–2021) — [Bugzilla #1705657][bz1705657]; [Bugzilla #1525082][bz1525082]**
A significant number of issued certificates returned OCSP status "unknown" rather than "good". Root cause included a software bug that withheld OCSP responses for certificates whose notBefore was set in the future, combined with a policy inherited from an eIDAS auditor recommendation (T-Systems) to mark certificates as "unknown" until physically handed to the subscriber. Mozilla found neither justification adequate for a public TLS CA. KIR remediated on 2019-04-05.

**3. Intermediate CA Certificate Policies non-compliance (2024) — [Bugzilla #1921597][bz1921597]; [Bugzilla #1921598][bz1921598]**
Two intermediate CA certificates (SZAFIR Trusted CA4) issued on 16 September 2024 were missing the Reserved Certificate Policy Identifiers required by TLS Baseline Requirements in their Certificate Policies extension. The error resulted from an incorrect value in the CA generation procedure used during the issuance ceremony. A separate bug (#1921598) covers an analogous omission in a SZAFIR Trusted CA3 certificate related to S/MIME BR. KIR revised its CA generation procedure to include automated extension checks.

**4. SZAFIR ROOT CA disabled by Microsoft (February 2024)**
The *original* SZAFIR ROOT CA (SHA-1: `D3EEFBCBBCF49867838626E23BB59CA01E305DB7`, not present in this folder) was **Disabled** by Microsoft in the February 2024 Microsoft Trusted Root Program deployment. SZAFIR ROOT CA2 was not named in that release.

Source: [Microsoft Trusted Root Program — February 2024 Deployment Notice][ms-feb2024].

No Mozilla distrust action against SZAFIR ROOT CA2 has been found. For a live search, see [Mozilla Bugzilla CA:KIR query][bz-search] and the [CCADB public report][ccadb].

## Transparency

- **CP/CPS:** Published in English and Polish at [elektronicznypodpis.pl][kir-docs]. The English CPS (v1.19, April 2024) is publicly available.
- **CCADB disclosure:** KIR discloses root and intermediate certificate data in the CCADB as required by Mozilla policy. Audit periods and auditor information are maintained there.
- **Certificate Transparency:** CPS section 4.4.2 commits to CT logging for all TLS certificates; WebTrust seal links are maintained on the certificate repository page.
- **Incident self-reporting:** The 2024 CA4/CA3 incidents (bugs #1921597 and #1921598) were self-reported to Mozilla promptly after external notification, with root-cause analysis and updated procedures provided.

Sources: [KIR certificate repository][kir-certs]; [KIR CPS (EN, April 2024)][cps2024]; [Bugzilla #1921597][bz1921597].

## Sources

- [Mozilla Bugzilla #817994 — KIR S.A.'s application for inclusion in Mozilla Root Certificate Program][bz817994]
- [Mozilla Bugzilla #1157375 — Add SZAFIR ROOT CA root certificate to NSS][bz1157375]
- [Mozilla Bugzilla #1495497 — KIR S.A.: Certificates issued with multiple BR violations][bz1495497]
- [Mozilla Bugzilla #1525082 — KIR OCSP "unknown" status for revoked certificate][bz1525082]
- [Mozilla Bugzilla #1705657 — KIR S.A.: Many certificates with OCSP Unknown][bz1705657]
- [Mozilla Bugzilla #1921597 — KIR: Intermediate CA - SZAFIR Trusted CA4 - Certificate Policies extension - non-compliance][bz1921597]
- [Mozilla Bugzilla #1921598 — KIR: Intermediate CA - SZAFIR Trusted CA3 - Certificate Policies extension - non-compliance][bz1921598]
- [mozilla.dev.security.policy — KIR S.A. Root Inclusion Request][mdsp1]
- [mozilla.dev.security.policy — Second Discussion of KIR S.A. Root Inclusion Request][mdsp2]
- [KIR Certification Practice Statement for trusted non-qualified certificates v1.19 (EN, April 2024)][cps2024]
- [KIR certificate and CRL repository — elektronicznypodpis.pl][kir-certs]
- [Microsoft Trusted Root Program — February 2024 Deployment Notice][ms-feb2024]
- [CCADB public included CA certificate list][ccadb]
- [KIR corporate website — kir.pl][kir-web]

[bz817994]: https://bugzilla.mozilla.org/show_bug.cgi?id=817994
[bz1157375]: https://bugzilla.mozilla.org/show_bug.cgi?id=1157375
[bz1495497]: https://bugzilla.mozilla.org/show_bug.cgi?id=1495497
[bz1525082]: https://bugzilla.mozilla.org/show_bug.cgi?id=1525082
[bz1705657]: https://bugzilla.mozilla.org/show_bug.cgi?id=1705657
[bz1921597]: https://bugzilla.mozilla.org/show_bug.cgi?id=1921597
[bz1921598]: https://bugzilla.mozilla.org/show_bug.cgi?id=1921598
[mdsp1]: https://groups.google.com/g/mozilla.dev.security.policy/c/aNbK4zw_Zb8
[mdsp2]: https://groups.google.com/d/topic/mozilla.dev.security.policy/aTN3lkAt0HM
[cps2024]: https://www.elektronicznypodpis.pl/storage/file/core_files/2024/4/17/d8237a0ab84cdbba98326ed12967d5fa/20240403_certification_practice_statement_kir_trusted_nq_certificates_1_19-sig.pdf
[kir-certs]: https://www.elektronicznypodpis.pl/en/support/certificates-and-crls
[ms-feb2024]: https://learn.microsoft.com/en-us/security/trusted-root/2024/feb2024
[ccadb]: https://www.ccadb.org/
[kir-web]: https://www.kir.pl/en
[bz-search]: https://bugzilla.mozilla.org/buglist.cgi?query_format=advanced&product=NSS&component=CA%20Certificate%20Root%20Program&short_desc=KIR&short_desc_type=anywordssubstr
