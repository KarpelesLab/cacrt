# ACCV

ACCV (Agència de Tecnologia i Certificació Electrònica / Agencia de Tecnología y Certificación Electrónica) is a qualified trust service provider operated by Istec (Infraestructures i Serveis de Telecomunicacions i Certificació, SA), an instrumental public-sector company of the Generalitat Valenciana (regional government of the Valencian Community, Spain), headquartered in Valencia. It issues qualified certificates under the EU eIDAS framework (Regulation 910/2014) for citizens, public employees, companies, and government entities, and holds TLS server-authentication trust in all major root programs.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|-------------|-------------|-----|-------------|---------------------|
| ACCVRAIZ1 | a94d09e5.0 | RSA 4096 | 2030-12-31 | 9A:6E:C0:12:E1:A7:DA:9D:BE:34:19:4D:47:8A:D7:C0:DB:18:22:FB:07:1D:F1:29:81:49:6E:D1:04:38:41:13 |

## Rationale for inclusion

ACCVRAIZ1 is included in all major root programs with TLS server-authentication (website) trust:

- **Mozilla NSS / Firefox:** Approved via [Bug 811352](https://bugzilla.mozilla.org/show_bug.cgi?id=811352), added to NSS via [Bug 872279](https://bugzilla.mozilla.org/show_bug.cgi?id=872279), shipped in Firefox 27. Trust bits: Websites, Email, Code Signing.
- **Microsoft Trusted Root Program:** Listed as Government of Spain / ACCV under SHA-1 `93057A8815C64FCE882FFA9116522878BC536417`.
- **Apple:** Pre-installed in iOS, iPadOS, macOS, tvOS, and watchOS trust stores.
- **Google Chrome:** Relies on the platform trust store on most platforms; ACCV certificates are listed as trusted by ACCV's own documentation alongside Microsoft, Apple, Mozilla, and Google.

ACCV is also on the EU [Trusted List of qualified trust service providers](https://esignature.ec.europa.eu/efts/browser/) under eIDAS, giving certificates qualified legal standing across EU member states. Certificates are integrated into Spain's national @firma validation platform.

## CA/Browser Forum compliance

ACCV undergoes annual third-party conformity audits conducted by **AENOR** (Asociación Española de Normalización y Certificación), accredited by ENAC (Spain's national accreditation body) under ISO/IEC 17065 and ETSI EN 319 403-1. The audit framework is **ETSI EN 319 411-1 / EN 319 411-2**; an Audit Attestation Letter (AAL) from the 2023 standard audit is published at [eidas.aenor.com](https://eidas.aenor.com/www.aenor.com/Certificacion_Documentos/eiDas/AENOR-AAL_PSC20170010-2023-ACCV_Standard_Audit.pdf) under reference PSC-2017-0010.

ACCV's Certification Practice Statement (CPS, current version ACCV-CPS-V5.0.2-EN-2024, OID `1.3.6.1.4.1.8149.2.5.0`) follows RFC 3647. It commits to CA/Browser Forum Baseline Requirements for TLS server certificates; the dedicated server-authentication certificate policy is [ACCV-CP-36V2.0.1-EN-2024](https://www.accv.es/fileadmin/Archivos/Politicas_pdf/ACCV-CP-36V2.0.1-EN-2024.pdf) (OID `1.3.6.1.4.1.8149.3.36.2.0`).

ACCV discloses records in the [CCADB](https://www.ccadb.org/). TLS certificates include Signed Certificate Timestamps (SCTs) as required by Chrome and Firefox CT policies. ACCV publishes test pages for valid ([activo.accv.es](https://activo.accv.es)), revoked ([revocado.accv.es](https://revocado.accv.es)), and expired ([caducado.accv.es](https://caducado.accv.es)) end-entity certificates.

## Past non-compliance

Three publicly documented incidents are on record; ACCV was not distrusted by any root program as a result of any of them.

1. **Late audit statement (2018) — [Bug 1507862](https://bugzilla.mozilla.org/show_bug.cgi?id=1507862):** Audit statements were submitted approximately six months after the end of the audit period, violating Mozilla Root Store Policy §3.1.3. Root cause: administrative delays caused by the entry into force of Spain's new Public Sector Contracts Act in March 2018. Remediation: a multi-year public tendering process for audits. Mozilla accepted the response and resolved the bug.

2. **cRLIssuer mis-issuance and delayed revocation (2024) — [Bug 1884532](https://bugzilla.mozilla.org/show_bug.cgi?id=1884532):** 837 TLS certificates issued after the BR 2.0.1 effective date (2023-09-15) contained the prohibited `cRLIssuer` field in the CRL Distribution Points extension, inherited silently from EJBCA CA configuration. Pre-issuance linting (ZLint) did not detect the issue. All 837 certificates were revoked by 2024-03-14. Two related spin-off bugs were opened for the delayed problem-report response ([Bug 1886785](https://bugzilla.mozilla.org/show_bug.cgi?id=1886785)) and delayed revocation ([Bug 1886788](https://bugzilla.mozilla.org/show_bug.cgi?id=1886788)). Remediation included multi-reviewer profile change procedures, addition of pkilint alongside ZLint, post-deployment node-checksum verification, and expanded compliance-team staffing.

3. **userNotice policy qualifier mis-issuance (2024) — [Bug 1889567](https://bugzilla.mozilla.org/show_bug.cgi?id=1889567):** 13 unrevoked TLS certificates (plus approximately 379 already-revoked ones) were issued after 2023-09-15 with the prohibited `id-qt-unotice` (userNotice) policy qualifier. A deployment error on 2023-09-10 left the production node on the old, non-compliant profile; the error was silently self-corrected during a 2023-11-25 maintenance window. ZLint did not detect the issue until v3.6.0 (deployed 2024-01-22). Notified by Sectigo in April 2024. ACCV admitted that its initial evidence collection undercounted affected certificates. Remediation aligned with Bug 1884532 measures. Bug closed as resolved in August 2024.

For a broader search for any further ACCV incidents, see the [Mozilla Bugzilla CA-problem tracker](https://bugzilla.mozilla.org/buglist.cgi?component=CA%20Certificate%20Compliance&product=NSS&query_format=advanced&short_desc=ACCV&short_desc_type=allwordssubstr).

## Transparency

- **CP/CPS:** Published in English at [accv.es/pdf-politicas](https://www.accv.es/pdf-politicas); the current CPS is [ACCV-CPS-V5.0.2-EN-2024](https://www.accv.es/fileadmin/Archivos/Practicas_de_certificacion/ACCV-CPS-V5.0.2-EN-2024.pdf) and the TLS CP/CPS combined document is [ACCV-CPS-CP-V4.0.19-EN-2025](https://www.accv.es/en/fileadmin/Archivos/Practicas_de_certificacion/ACCV-CPS-CP-V4.0.19-EN-2025.pdf).
- **CCADB disclosure:** ACCV discloses root and intermediate CA records in the CCADB as required by the Mozilla Root Store Policy.
- **Incident self-reporting:** All three incidents above were reported or responded to via the Mozilla Bugzilla CA Compliance component; Bug 1884532 and Bug 1889567 include ACCV-authored incident reports.
- **Certificate Transparency:** ACCV TLS certificates carry embedded SCTs satisfying Chrome and Firefox CT policy requirements. Test domains are published for integration testing.
- **ETSI audit attestation:** AENOR publishes the Audit Attestation Letters on the AENOR eIDAS portal per CCADB requirements.

## Sources

- [ACCV official website](https://www.accv.es/en/)
- [ACCV — About us (Istec / Generalitat Valenciana)](https://www.accv.es/en/quienes-somos/)
- [ACCV certificate hierarchy download page](https://www.accv.es/en/servicios/ciudadanos-y-autonomos/descarga-de-certificados-jerarquia/)
- [ACCV CPS v5.0.2 (2024, English)](https://www.accv.es/fileadmin/Archivos/Practicas_de_certificacion/ACCV-CPS-V5.0.2-EN-2024.pdf)
- [ACCV TLS CP/CPS combined v4.0.19 (2025, English)](https://www.accv.es/en/fileadmin/Archivos/Practicas_de_certificacion/ACCV-CPS-CP-V4.0.19-EN-2025.pdf)
- [ACCV Server Authentication Certificate Policy v2.0.1 (2024)](https://www.accv.es/fileadmin/Archivos/Politicas_pdf/ACCV-CP-36V2.0.1-EN-2024.pdf)
- [Mozilla Bugzilla Bug 811352 — Additional Root CA for ACCV (ACCVRAIZ1 inclusion approval)](https://bugzilla.mozilla.org/show_bug.cgi?id=811352)
- [Mozilla Bugzilla Bug 872279 — Add ACCVRAIZ1 root certificate to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=872279)
- [mozilla.dev.security.policy — ACCV Root Inclusion Request](https://groups.google.com/g/mozilla.dev.security.policy/c/z6PXEzzt4mQ)
- [Mozilla Bugzilla Bug 1507862 — ACCV: Late Audit Statement (2018)](https://bugzilla.mozilla.org/show_bug.cgi?id=1507862)
- [Mozilla Bugzilla Bug 1884532 — ACCV: cRLIssuer in CRL Distribution Points (2024)](https://bugzilla.mozilla.org/show_bug.cgi?id=1884532)
- [Mozilla Bugzilla Bug 1889567 — ACCV: Certificates issued with Policy qualifiers other than id-qt-cps (2024)](https://bugzilla.mozilla.org/show_bug.cgi?id=1889567)
- [AENOR Audit Attestation Letter PSC-2017-0010 (2023)](https://eidas.aenor.com/www.aenor.com/Certificacion_Documentos/eiDas/AENOR-AAL_PSC20170010-2023-ACCV_Standard_Audit.pdf)
- [Mozilla Included CA Certificate Report (CCADB)](https://ccadb.my.salesforce-sites.com/mozilla/IncludedCACertificateReport)
