# TUBITAK

TÜBİTAK (Turkiye Bilimsel ve Teknolojik Arastirma Kurumu — the Scientific and Technological Research Council of Turkey) operates Kamu SM (Kamu Sertifikasyon Merkezi, "Government Certification Authority"), headquartered in Gebze, Kocaeli, Turkey. Established in 2005 under Electronic Signature Law No. 5070, Kamu SM is a government-owned CA that issues OV TLS certificates to Turkish government agencies and, following a 2022 regulatory change, to private-sector entities within the .tr ccTLD. In the Web PKI it operates as a technically-constrained CA whose name constraints limit issuance to the .tr ccTLD.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| TUBITAK Kamu SM SSL Kok Sertifikasi - Surum 1 | `ff34af3f.0` | RSA 2048 | 2043-10-25 | `46:ED:C3:68:90:46:D5:3A:45:3F:B3:10:4A:B8:0D:CA:EC:65:8B:26:60:EA:16:29:DD:7E:86:79:90:64:87:16` |

## Rationale for inclusion

The root is included in all four major root programs — Mozilla, Chrome, Apple, and Microsoft — with the Websites (TLS server authentication) trust bit enabled. It was added to Mozilla NSS in release 3.30.2 (Firefox 54) and NSS 3.28.5 (ESR 52.2), via [Mozilla Bugzilla #1349705](https://bugzilla.mozilla.org/show_bug.cgi?id=1349705). The inclusion was accompanied by name constraints, making this the first root included in NSS with domain restrictions applied from the outset.

The original constraints limited issuance to `gov.tr, k12.tr, pol.tr, mil.tr, tsk.tr, kep.tr, bel.tr, edu.tr, org.tr`. Following a public comment period on the [mozilla.dev.security.policy list](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/DotaWOS0v1E) (closed 2022-11-29, no objections), Mozilla approved expanding the constraint to the full `.tr` ccTLD; the change landed in NSS 3.89.1 / Firefox 114 via [Bug #1804505](https://bugzilla.mozilla.org/show_bug.cgi?id=1804505). The expansion was justified by a Turkish regulatory change that allowed Kamu SM to serve private-sector subscribers.

Apple lists this root in its trusted certificate catalogue ([iOS 16 / macOS 13 list](https://support.apple.com/en-us/103100)).

## CA/Browser Forum compliance

Kamu SM is audited annually by Kiwa Cermet Italia S.p.A. (Accredia-accredited) against:

- **ETSI EN 319 411-1** (LCP and OVCP profiles) and **ETSI EN 319 401**
- **CA/Browser Forum Baseline Requirements for TLS Server Certificates**
- **CA/Browser Forum Network and Certificate System Security Requirements**

The 2023 audit attestation (ACVPR 776, covering period 2022-09-19 to 2023-09-19, on-site 2023-10-19 to 2023-10-20) found **no major or minor non-conformities**; it was issued 2023-10-26 ([Kiwa audit attestation ACVPR 776](https://www.kiwa.com/globalassets/italy/eidas-certificates/tubitak-2023.pdf)). The CA's CP and CPS are published in English at `http://depo.kamusm.gov.tr/ilke/` and are disclosed in the CCADB. The 2023 audit was based on CP v1.0.5 (2023-08-09) and CPS v3.6.3 (2023-09-04).

The CA issues OV SSL certificates; it does not issue EV or DV certificates. Certificates are logged to public Certificate Transparency logs; Kamu SM's in-house linting tool (ARMA) was upgraded in 2023 to perform pre-issuance linting in addition to post-issuance checks ([Bug #1847193](https://bugzilla.mozilla.org/show_bug.cgi?id=1847193)).

## Past non-compliance

**2017 — Non-BR-compliant serial number entropy ([Bug #1390998](https://bugzilla.mozilla.org/show_bug.cgi?id=1390998)):** Between October 2016 and May 2017, Kamu SM issued 61 TLS certificates from its older root (TÜBİTAK UEKAE Kök Sertifika Hizmet Sağlayıcısı - Sürüm 3) with serial number entropy below the 64-bit minimum required by BR §7.1. The CA had continued using the old root because government customers on Android and iOS could not yet use the new root. A problem report sent by email on 2017-08-10 was not seen until 2017-08-17 (it landed in spam), also a BR violation (24-hour response requirement). Because the old root and all affected certificates were set to expire on 2017-08-21 (days away), the CA chose not to revoke; Mozilla's Ryan Sleevi resolved the bug Fixed with a note recommending pre-issuance linting integration. The 61 certificates were submitted to CT logs by the reporter.

**2019 — Non-compliant serial numbers in test certificates ([mozilla.dev.security.policy thread](https://groups.google.com/g/mozilla.dev.security.policy/c/JQrF7MepaWE)):** During a broader industry review triggered by the DarkMatter discussions (2019-02-26), Kamu SM identified two test certificates issued on 2023-02-03 with sub-64-bit serial entropy. No subscriber certificates were affected. The CA corrected its random number generator and the matter was tracked in Mozilla Bugzilla (#1539190); the CA confirmed its CCADB contact details in the process.

**2023 — commonName absent from SAN ([Bug #1847193](https://bugzilla.mozilla.org/show_bug.cgi?id=1847193)):** On 2023-08-04 Kamu SM issued one certificate in which the CN value was not included in the SAN extension, violating the Baseline Requirements. Post-issuance linting by ARMA detected the error and the certificate was revoked at 06:07 UTC, seven minutes after issuance. Root cause was human error during multi-domain request handling. Remediation (updated issuance checklist, CN-SAN software check, promotion of ARMA to pre-issuance linting) was operational in production by 2023-09-27; the bug was resolved Fixed on 2023-09-29. The 2023 Kiwa audit confirmed remediation was properly addressed.

No distrust actions by any major root program have been identified for this CA. For a full view of open or resolved Bugzilla issues, see the [Mozilla Bugzilla search for Kamu SM](https://bugzilla.mozilla.org/buglist.cgi?product=CA%20Program&component=CA%20Certificate%20Compliance&status_whiteboard=KamuSM&resolution=---).

## Transparency

- **CP/CPS:** Published in English at `http://depo.kamusm.gov.tr/ilke/` (versioned subdirectory `KamuSM_CPS/`); also mirrored at `https://kamusm.bilgem.tubitak.gov.tr/BilgiDeposu/KamuSM_CPS/KamuSM_CPS_En.pdf`. The PKI Disclosure Statement is at `https://kamusm.bilgem.tubitak.gov.tr/dosyalar/makaleler/Kamu_SM_PKI_Disclosure_Statement.pdf`.
- **CCADB:** The root and its intermediate are disclosed in the CCADB. Mozilla Applied Constraints (`.tr`) are reflected in the CCADB `IncludedCACertificateReportPEMCSV`. CCADB contact was verified during the 2019 incident response.
- **Incident reporting:** All three incidents above were self-reported or promptly responded to via the Mozilla dev-security-policy list and Bugzilla.
- **Certificate Transparency:** Certificates issued by the subordinate CA (TUBITAK Kamu SM SSL Sertifika Hizmet Saglayicisi - Surum 1, SHA-256: `BF32DA…CB67CE0`) are submitted to public CT logs. As of 2023-09-27, pre-issuance linting is operational in production. CT issuance activity is visible on [Cloudflare Radar](https://radar.cloudflare.com/certificate-transparency/ca/46EDC3689046D53A453FB3104AB80DCAEC658B2660EA1629DD7E867990648716).

## Sources

- [Mozilla Bugzilla #1262809 — TUBITAK Kamu Sertifikasyon Merkezi - New Root Certificate (original inclusion request)](https://bugzilla.mozilla.org/show_bug.cgi?id=1262809)
- [Mozilla Bugzilla #1349705 — Add "TUBITAK Kamu SM SSL Kok Sertifikasi - Surum 1" to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=1349705)
- [Mozilla Bugzilla #1390998 — Kamu SM: Non-BR-Compliant Certificate Issuance (2017 serial entropy)](https://bugzilla.mozilla.org/show_bug.cgi?id=1390998)
- [mozilla.dev.security.policy — Kamu SM: Information about non-compliant serial numbers (2019)](https://groups.google.com/g/mozilla.dev.security.policy/c/JQrF7MepaWE)
- [mozilla.dev.security.policy — KamuSM request to Expand to .tr ccTLD (2022)](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/DotaWOS0v1E)
- [Mozilla Bugzilla #1804505 — Change KamuSM's Root Certificate Technical Constraints (to .tr)](https://bugzilla.mozilla.org/show_bug.cgi?id=1804505)
- [Mozilla Bugzilla #1847193 — TÜBİTAK BİLGEM KAMU SM: commonName not in SAN (2023)](https://bugzilla.mozilla.org/show_bug.cgi?id=1847193)
- [Kiwa Cermet Italia — Audit Attestation ACVPR 776 for TÜBİTAK BİLGEM Kamu SM (2023-10-26)](https://www.kiwa.com/globalassets/italy/eidas-certificates/tubitak-2023.pdf)
- [Kamu SM SSL Certificate Policy and CPS (English, v1.0.5/3.6.3)](https://kamusm.bilgem.tubitak.gov.tr/BilgiDeposu/KamuSM_CPS/KamuSM_CPS_En.pdf)
- [Kamu SM PKI Disclosure Statement](https://kamusm.bilgem.tubitak.gov.tr/dosyalar/makaleler/Kamu_SM_PKI_Disclosure_Statement.pdf)
- [TÜBİTAK BİLGEM — Kamu SM Government Certification Authority](https://bilgem.tubitak.gov.tr/en/icerik/kamu-sm-government-certification-authority)
- [Apple trusted root certificates list — iOS 16 / macOS 13](https://support.apple.com/en-us/103100)
- [golang/go #61963 — TUBITAK Kamu SM SSL Kok Sertifikasi - Surum 1 should be constrained (Go bundle)](https://github.com/golang/go/issues/61963)
