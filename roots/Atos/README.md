# Atos

Atos SE is a French multinational IT services company (headquarters: Bezons, France) that operates the **Atos Trustcenter** PKI service, branded and run through its subsidiary **Eviden Germany GmbH** (formerly Atos Germany, Lohberg 10, 49716 Meppen, Germany). The Trustcenter issues publicly trusted TLS server certificates and S/MIME certificates under the **Atos TrustedRoot** brand and participates in the Web PKI through Mozilla NSS, Chrome Root Store, and Microsoft CTL. This folder merges the Atos/Eviden legal-entity roots; the operator has rebranded subordinate CAs from "Atos Trustcenter" to "Eviden Trustcenter" following the April 2023 Eviden spin-out from Atos SE.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| Atos TrustedRoot 2011 | `e36a6752.0` | RSA 2048 | 2030-12-31 | `F3:56:BE:A2:44:B7:A9:1E:B3:5D:53:CA:9A:D7:86:4A:CE:01:8E:2D:35:D5:F8:F9:6D:DF:68:A6:F4:1A:A4:74` |
| Atos TrustedRoot Root CA ECC TLS 2021 | `fb717492.0` | ECC P-384 | 2041-04-17 | `B2:FA:E5:3E:14:CC:D7:AB:92:12:06:47:01:AE:27:9C:1D:89:88:FA:CB:77:5F:A8:A0:08:91:4E:66:39:88:A8` |
| Atos TrustedRoot Root CA RSA TLS 2021 | `9b46e03d.0` | RSA 4096 | 2041-04-17 | `81:A9:08:8E:A5:9F:B3:64:C5:48:A6:F8:55:59:09:9B:6F:04:05:EF:BF:18:E5:32:4E:C9:F4:57:BA:00:11:2F` |

## Rationale for inclusion

**Atos TrustedRoot 2011** was included in Mozilla NSS in Firefox 29 following two rounds of public discussion (Bugzilla [#711366](https://bugzilla.mozilla.org/show_bug.cgi?id=711366), [#938814](https://bugzilla.mozilla.org/show_bug.cgi?id=938814)). It carries trust bits for server authentication, email protection, and code signing, and is also present in the Microsoft CTL. This root anchors the legacy hierarchy and continues to issue subordinate CAs (e.g., Eviden TrustedRoot Client-CA G2 2023) through its 2030 expiry.

**Atos TrustedRoot Root CA RSA TLS 2021** and **Atos TrustedRoot Root CA ECC TLS 2021** were approved for Mozilla NSS in Firefox 117 / NSS 3.92 (Bugzilla [#1782092](https://bugzilla.mozilla.org/show_bug.cgi?id=1782092)) and added to the Chrome Root Store in the September 2023 release cycle following the Chrome Root Program's detailed review ([CCADB announcement, 2023-07-28](https://groups.google.com/a/ccadb.org/g/public/c/xVi4Gq5yvQ8)). Both 2021 roots are scoped exclusively to TLS server authentication. The operator confirmed to Mozilla that none of the roots are used for DLP or network surveillance purposes.

## CA/Browser Forum compliance

Audits for all three roots are performed under the **ETSI EN 319 411-1** framework (formerly ETSI TS 102 042). For the 2023 inclusion request, the primary auditor was **datenschutz cert GmbH**; the audit letter cited an issuance date of 15 June 2022 covering a period ending 27 April 2022, with no incidents in the preceding 24 months ([CCADB public discussion](https://groups.google.com/a/ccadb.org/g/public/c/v5yFBHjuBRo/m/rN_6EmbXAgAJ)). The CPS is structured per RFC 3647 and states explicitly that where inconsistencies exist between the CPS and the CA/Browser Forum Baseline Requirements, the BRs take precedence. CCADB disclosure is maintained and CA records are publicly listed in the CCADB. Certificate Transparency is required for all publicly trusted TLS certificates issued from the 2021 TLS roots. The operator does not currently advertise ACME support in public-facing documentation.

## Past non-compliance

The following incidents are documented in Mozilla Bugzilla:

1. **Duplicate serial numbers on intermediate CAs (pre-BR)** — Two pairs of intermediate CAs ("Atos TrustedRoot Server-CA 2011" and "Atos TrustedRoot CodeSigning-CA 2011") shared identical Issuer/Serial Number combinations. The certificates predated the Baseline Requirements effective date of 2012-07-01 and had already been revoked by the time the issue was reported; the bug was closed as a duplicate of the original inclusion request. ([Bugzilla #1329223](https://bugzilla.mozilla.org/show_bug.cgi?id=1329223))

2. **Insufficient serial number entropy (2019)** — Certificates were issued with only 63 bits of effective entropy in serial numbers due to a misinterpretation of BR §7.1 (highest bit forced to zero). Approximately 322 TLS certificates and ~200,000 S/MIME certificates were affected. Atos revoked all software-based affected certificates, increased entropy to 95 bits (96-bit field), and added technical validator controls. ([Bugzilla #1540961](https://bugzilla.mozilla.org/show_bug.cgi?id=1540961))

3. **Incorrect OCSP delegated responder certificates (2020)** — Four intermediate CA certificates contained the `id-kp-OCSPSigning` EKU without the required `id-pkix-ocsp-nocheck` extension, violating BRs. All four ICAs were revoked within seven days; private keys were destroyed in the presence of an auditor on 2020-07-14; ICA profiles were updated and a creation checklist checkpoint was added. ([Bugzilla #1649963](https://bugzilla.mozilla.org/show_bug.cgi?id=1649963))

4. **COVID-19 audit delay (2020)** — The 2020 ETSI surveillance audit for Atos TrustedRoot 2011 was conducted as a fully remote session due to pandemic travel restrictions. Mozilla raised concerns about the reduced assurance of remote audits. ([Bugzilla #1626355](https://bugzilla.mozilla.org/show_bug.cgi?id=1626355))

No distrust action has been taken against any Atos/Eviden root by Mozilla, Chrome, Apple, or Microsoft. For a current view of open or resolved reports, see the [CCADB problem reporting search](https://ccadb.my.salesforce-sites.com/mozilla/CACertificatesInFirefoxReport) and [Mozilla Bugzilla CA component](https://bugzilla.mozilla.org/buglist.cgi?product=NSS&component=CA+Certificates).

## Transparency

- **CP/CPS:** Published at [https://pki.atos.net/trustcenter/en/download/trusted-root-ca](https://pki.atos.net/trustcenter/en/download/trusted-root-ca). The current CPS for issuing CAs is version 2.7.5 (2023-06-28), branded as "Eviden TrustedRoot CA Issuing CA – CPS". A combined CP/CPS approach is used (no separate CP document); the document is structured per RFC 3647 and considers ETSI EN 319 411-1 and CA/B Forum BR requirements.
- **CCADB disclosure:** All three roots are disclosed in the CCADB under the Eviden operator record. Root certificates, audit information, and CP/CPS URLs are maintained there.
- **Incident self-reporting:** Incidents in the Bugzilla list above were reported by Atos to Mozilla. The CPS specifies a 24×7 contact mechanism for reporting certificate misuse, private key compromise, and service outages.
- **Certificate Transparency:** CT logging is required for all publicly trusted TLS end-entity certificates issued from the 2021 TLS roots, consistent with CA/B Forum BR and Chrome CT Policy requirements.

## Sources

- [Bugzilla #711366 – Add Atos Trustcenter CA cert to trusted root CA cert list](https://bugzilla.mozilla.org/show_bug.cgi?id=711366)
- [Bugzilla #938814 – Add Atos TrustedRoot 2011 root certificate to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=938814)
- [Bugzilla #1329223 – Atos: duplicate serial numbers](https://bugzilla.mozilla.org/show_bug.cgi?id=1329223)
- [Bugzilla #1540961 – Atos: Insufficient Serial Number Entropy](https://bugzilla.mozilla.org/show_bug.cgi?id=1540961)
- [Bugzilla #1626355 – Atos: Tracking bug for possible audit delays](https://bugzilla.mozilla.org/show_bug.cgi?id=1626355)
- [Bugzilla #1649963 – Atos: Incorrect OCSP Delegated Responder Certificate](https://bugzilla.mozilla.org/show_bug.cgi?id=1649963)
- [Bugzilla #1782092 – Add Atos Roots (Mozilla inclusion request, 2022–2023)](https://bugzilla.mozilla.org/show_bug.cgi?id=1782092)
- [CCADB Public Discussion of Atos CA Inclusion Request](https://groups.google.com/a/ccadb.org/g/public/c/v5yFBHjuBRo/m/rN_6EmbXAgAJ)
- [CCADB: Chrome Root Store Inclusion Decision – Eviden (formerly Atos Trustcenter), 2023-07-28](https://groups.google.com/a/ccadb.org/g/public/c/xVi4Gq5yvQ8)
- [Atos Root Inclusion Request – mozilla.dev.security.policy](https://groups.google.com/g/mozilla.dev.security.policy/c/zEfkTiu_Uqk)
- [Eviden Trustcenter website](https://pki.atos.net/trustcenter/en)
- [Eviden TrustedRoot CPS v2.7.5 (via PSW Group mirror)](https://www.psw-group.de/fileadmin/user_upload/Downloads/Eviden_TrustedRoot_CPS_IssuingCAs.pdf)
- [Atos – Wikipedia](https://en.wikipedia.org/wiki/Atos)
- [Eviden PKI Consortium member profile](https://pkic.org/members/eviden/)
- [datenschutz cert GmbH – Zertifikatsliste (Atos ATCA)](https://www.datenschutz-cert.de/zertifikatslisten/zertifikatsliste?tx_dsnordreferenzen_pi1%5Bzertifikate%5D=551&cHash=ac2736682f0aebcda035f1cb8f31a99e)
- [CCADB Mozilla CA Certificates in Firefox Report](https://ccadb.my.salesforce-sites.com/mozilla/CACertificatesInFirefoxReport)
