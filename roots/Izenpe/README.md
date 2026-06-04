# Izenpe

Izenpe S.A. is a public limited company established in 2002 and co-owned by the Basque Government and the three Basque Provincial Councils (through their IT subsidiaries EJIE, LANTIK, IZFE, and CCASA), headquartered in Vitoria-Gasteiz, Spain. Its primary mission is to provide qualified and non-qualified digital certificates and trust services to Basque public administrations, civil servants, citizens, and businesses, and it also operates as a Qualified Trust Service Provider under the EU eIDAS regulation. All Web PKI roots in this folder are issued under the single legal entity IZENPE S.A.; no mergers or brand aliases apply.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| Izenpe_com.pem | `cc450945.0` | RSA 4096 | 2037-12-13 | `25:30:CC:8E:98:32:15:02:BA:D9:6F:9B:1F:BA:1B:09:9E:2D:29:9E:0F:45:48:BB:91:4F:36:3B:C0:D4:53:1F` |

## Rationale for inclusion

The "Izenpe.com" root (SHA-256 `2530…531F`) is included in the Mozilla Root Program (websites and EV trust bits enabled; email trust bit not requested), Apple's trust store, and the Microsoft Trusted Root Program. It was added to Mozilla NSS following community discussion and approval in 2010 ([Bugzilla #578491](https://bugzilla.mozilla.org/show_bug.cgi?id=578491)), with EV status enabled shortly afterwards ([Bugzilla #578499](https://bugzilla.mozilla.org/show_bug.cgi?id=578499)). The EV policy OID is `1.3.6.1.4.1.14777.6.1.1`. The root issues TLS server-authentication certificates to public-sector and commercial entities in the Basque Country and is a member of the [PKI Consortium](https://pkic.org/members/izenpe/).

## CA/Browser Forum compliance

Izenpe is audited annually under the **ETSI EN 319 411-1 / 319 411-2** framework (not WebTrust) by **AENOR Internacional, S.A.U.** The most recent available audit attestation letter (period 2023-10-31 to 2024-10-30) covers the Izenpe.com root and reports no major or minor non-conformities against ETSI EN 319 411-2 v2.5.1, ETSI EN 319 401, CA/Browser Forum EV Guidelines v2.0.1, Baseline Requirements for TLS Server Certificates v2.0.8, and Network and Certificate System Security Requirements v1.7 ([AENOR audit attestation, 2024](https://eidas.aenor.com/www.aenor.com/Certificacion_Documentos/eiDas/3-AENOR-AAL_PSC20200004-2024-IZENPE_TLS-EV_Audit.pdf)).

Izenpe's CP/CPS is structured per RFC 3647. The combined CP/CPS for Website Authentication Certificates (v2.7) commits to CA/BF Baseline Requirements and EV Guidelines, mandates linting (zlint, pkimetal) effective 2025-03-15, sets TLS validity to 395 days, and requires Certificate Transparency pre-certificate logging for all issued TLS certificates ([CP/CPS v2.7](https://www.izenpe.eus/contenidos/informacion/cps_ssl/en_def/adjuntos/cp_cps_2.7.pdf)). CA records are disclosed in the CCADB ([Mozilla Included CA Certificate Report](https://ccadb.my.salesforce-sites.com/mozilla/includedcacertificatereport)).

## Past non-compliance

Several publicly documented incidents exist:

**2016 — Issuance to a non-public TLD after prohibition.** Izenpe issued a certificate for a domain ending in `.jaso`, which is not a real TLD, after the 2012 Baseline Requirements prohibited issuance to reserved/internal names. This was raised as part of the broader 2020 incident discussion ([Bugzilla #1651026](https://bugzilla.mozilla.org/show_bug.cgi?id=1651026), comment context).

**2019 — Intermediate CAs not disclosed in audit report ([Bugzilla #1596744](https://bugzilla.mozilla.org/show_bug.cgi?id=1596744)).** Izenpe failed to include SHA-1 certificate identifiers for several sub-CAs in its audit report, discovered via CCADB Audit Letter Validation. Three SSL/TLS-issuing sub-CAs were revoked within days; four non-SSL sub-CAs (widely deployed for citizen qualified signatures) required a delayed revocation plan tracked separately in [Bugzilla #1598608](https://bugzilla.mozilla.org/show_bug.cgi?id=1598608). Bug closed as RESOLVED FIXED with `[ca-compliance] [audit-failure]` tags.

**2020 — Certificate issued to an internal domain ([Bugzilla #1651026](https://bugzilla.mozilla.org/show_bug.cgi?id=1651026)).** An operator used the wrong CSR and issued a TLS certificate containing internal domain names. The certificate was detected and revoked within minutes. Investigation also revealed that a February 2020 software patch had silently disabled an internal-domain check, and that CAA validation for four manually-processed certificates was performed by an operator using Google's online dig tool, which reviewers (Ryan Sleevi, Andrew Ayer) identified as reliance on an undisclosed delegated third party, making those certificates misissued. The email contingency issuance path was eliminated; automated validation and CA/BF-compliant CAA checking were introduced. Bug closed as RESOLVED FIXED.

**2021 — Incorrectly encoded SubjectPublicKeyInfo in sub-CAs ([Bugzilla #1685767](https://bugzilla.mozilla.org/show_bug.cgi?id=1685767)).** Seven sub-CAs issued in October 2010 (before BR 1.0 took effect July 2012) had the ASN.1 NULL parameter omitted from their `sha256WithRSAEncryption` algorithm identifier, violating RFC 4055. Izenpe remediated by restructuring its CA hierarchy: non-SSL CAs were moved to a new tree, leaving only two sub-CAs notified to root programs. Bug resolved as a duplicate of [Bugzilla #1667846](https://bugzilla.mozilla.org/show_bug.cgi?id=1667846) (undisclosed certificates in CCADB).

**2016 — CT log key reuse, log disqualified by Chrome.** Izenpe operated a Certificate Transparency log at `ct.izenpe.com` that shared its key with a test log instance, causing it to present two conflicting views of the Merkle tree. Chrome's CT policy team disqualified the log effective 2016-05-30; no SCTs from that log (past or future) count toward CT requirements ([Chromium CT Policy list, May 2016](https://groups.google.com/a/chromium.org/g/ct-policy/c/qOorKuhL1vA)). Izenpe was permitted to reapply with a new key at a new URL (`ct.izenpe.eus`).

No distrust or removal of the Izenpe.com root from any major root program has been documented. To check for any open compliance bugs, see the [Mozilla Bugzilla CA: Izenpe query](https://bugzilla.mozilla.org/buglist.cgi?product=CA%20Program&component=CA%20Certificate%20Compliance&short_desc=Izenpe&short_desc_type=allwordssubstr) and the [CCADB public record](https://ccadb.my.salesforce-sites.com/mozilla/includedcacertificatereport).

## Transparency

- **CP/CPS:** Published in English at [izenpe.eus](https://www.izenpe.eus/contenidos/informacion/cps_ssl/en_def/adjuntos/cp_cps_2.7.pdf) (combined CP/CPS v2.7 for website authentication certificates; separate CPS v7.2 for the overall PKI hierarchy at [DOC_P_CPS_v7.2.pdf](https://www.izenpe.eus/contenidos/informacion/dpc_izenpe/en_def/adjuntos/DOC_P_CPS_v7.2.pdf)).
- **CCADB disclosure:** Izenpe discloses root and intermediate CA certificates in the CCADB as required by the Mozilla Root Store Policy.
- **Incident self-reporting:** Incidents documented in publicly accessible Mozilla Bugzilla CA compliance tickets (see bugs linked above); Izenpe has filed self-reports and responded to community-raised issues.
- **Certificate Transparency:** All issued TLS certificates include SCTs. Izenpe previously operated its own CT log (`ct.izenpe.com`), which was disqualified in 2016 (see above). A replacement log (`ct.izenpe.eus`) was submitted for qualification ([ct-policy inclusion request](https://groups.google.com/g/certificate-transparency/c/Au0Pq1bmMWY)). Current TLS issuance relies on third-party CT logs.

## Sources

- [Mozilla Bugzilla #361957 — Add Izenpe CA EV root certificate](https://bugzilla.mozilla.org/show_bug.cgi?id=361957)
- [Mozilla Bugzilla #578491 — Add Izenpe.com root certificate to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=578491)
- [Mozilla Bugzilla #578499 — Enable Izenpe.com root certificate for EV in PSM](https://bugzilla.mozilla.org/show_bug.cgi?id=578499)
- [Mozilla Bugzilla #1432807 — Disable Izenpe.com root cert (expiring non-included root)](https://bugzilla.mozilla.org/show_bug.cgi?id=1432807)
- [Mozilla Bugzilla #1596744 — Izenpe: Intermediate CA certificates not listed in audit report](https://bugzilla.mozilla.org/show_bug.cgi?id=1596744)
- [Mozilla Bugzilla #1651026 — Izenpe: certificate issued to internal domain](https://bugzilla.mozilla.org/show_bug.cgi?id=1651026)
- [Mozilla Bugzilla #1685767 — Izenpe: Multiple sub CAs with incorrectly encoded SubjectPublicKeyInfo algorithm](https://bugzilla.mozilla.org/show_bug.cgi?id=1685767)
- [Chromium CT Policy — Upcoming CT Log Removal: Izenpe (2016)](https://groups.google.com/a/chromium.org/g/ct-policy/c/qOorKuhL1vA)
- [Chromium CT Policy — Izenpe test and production logs use the same key](https://groups.google.com/a/chromium.org/g/ct-policy/c/g0WWgUodpOk)
- [CT Policy — Izenpe CT log inclusion request (ct.izenpe.eus)](https://groups.google.com/g/certificate-transparency/c/Au0Pq1bmMWY)
- [AENOR Audit Attestation Letter for Izenpe TLS EV (2024)](https://eidas.aenor.com/www.aenor.com/Certificacion_Documentos/eiDas/3-AENOR-AAL_PSC20200004-2024-IZENPE_TLS-EV_Audit.pdf)
- [Izenpe CP/CPS v2.7 — Website Authentication Certificates](https://www.izenpe.eus/contenidos/informacion/cps_ssl/en_def/adjuntos/cp_cps_2.7.pdf)
- [Izenpe CPS v7.2 (2022)](https://www.izenpe.eus/contenidos/informacion/dpc_izenpe/en_def/adjuntos/DOC_P_CPS_v7.2.pdf)
- [Izenpe — PKI Consortium member page](https://pkic.org/members/izenpe/)
- [Mozilla Included CA Certificate Report (CCADB)](https://ccadb.my.salesforce-sites.com/mozilla/includedcacertificatereport)
- [mozilla.dev.security.policy — Izenpe Root Inclusion Request](https://groups.google.com/g/mozilla.dev.security.policy/c/BgVw_Ii-2X8)
