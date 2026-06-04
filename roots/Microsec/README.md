# Microsec

Microsec Ltd. is a Hungarian private corporation headquartered in Budapest (Graphisoft Park, Ángel Sanz Briz út 13, H-1033) that operates the **e-Szignó** public-key infrastructure and is the largest certificate authority in Hungary. Founded in 1984, the company is a Qualified Trust Service Provider (QTSP) under eIDAS and serves as the IT service provider for the Hungarian Ministry of Justice. All three roots in this folder are issued by Microsec Ltd. under the e-Szignó brand; no mergers of separate legal entities are involved.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| Microsec e-Szigno Root CA 2009 | `8160b96c.0` | RSA 2048 | 2029-12-30 | `3C:5F:81:FE:A5:FA:B8:2C:64:BF:A2:EA:EC:AF:CD:E8:E0:77:FC:86:20:A7:CA:E5:37:16:3D:F3:6E:DB:F3:78` |
| e-Szigno Root CA 2017 | `e868b802.0` | ECC P-256 | 2042-08-22 | `BE:B0:0B:30:83:9B:9B:C3:2C:32:E4:44:79:05:95:06:41:F2:64:21:B1:5E:D0:89:19:8B:51:8A:E2:EA:1B:99` |
| e-Szigno TLS Root CA 2023 | `f44703f1.0` | ECC P-521 | 2038-07-17 | `B4:91:41:50:2D:00:66:3D:74:0F:2E:7E:C3:40:C5:28:00:96:26:66:12:1A:36:D0:9C:F7:DD:2B:90:38:4F:B4` |

## Rationale for inclusion

**Microsec e-Szigno Root CA 2009** is included in Apple, Google Chrome, Microsoft, and Mozilla root stores with Server Authentication trust enabled. It was approved by Mozilla in 2010 with Websites, Email, and Code Signing trust bits ([Bug 557904](https://bugzilla.mozilla.org/show_bug.cgi?id=557904), [Bug 510506](https://bugzilla.mozilla.org/show_bug.cgi?id=510506)).

**e-Szigno Root CA 2017** was approved for inclusion in Mozilla/NSS ([Bug 1645174](https://bugzilla.mozilla.org/show_bug.cgi?id=1645174)) with Websites and Email trust bits; Mozilla denied EV treatment citing compliance concerns. It was added to the Microsoft Trusted Root Program in [May 2021](https://learn.microsoft.com/en-us/security/trusted-root/2021/may2021) and is also included in Apple and Google Chrome per CCADB records.

**e-Szigno TLS Root CA 2023** was created specifically to satisfy Chrome Root Program requirements for a dedicated TLS-only hierarchy. It is currently included in the Microsoft root store. A CCADB public discussion (Case 00001692, November–December 2025) concluded with no objections; final decisions by Apple, Google, and Mozilla are pending as of June 2026 ([CCADB public discussion](https://groups.google.com/a/ccadb.org/g/public/c/95h0Cjq8A0k)).

All three roots are scoped to TLS Server Authentication. Microsec is supervised by the Hungarian National Media and Infocommunications Authority (NMHH) as a Qualified Trust Service Provider.

## CA/Browser Forum compliance

Microsec is audited annually under **ETSI EN 319 411** by Hungarian conformity assessment bodies — historically Hunguard and TÜViT (TÜV Nord Group). Audit attestation letters (AALs) are required to cover CA/Browser Forum Baseline Requirements (BRs) compliance and are filed in CCADB. The e-Szigno TLS Root CA 2023 hierarchy completed its first conformity assessment in Q4 2023, covering ETSI EN 319 411-1/-2 and the CA/B Forum TLS BRs.

All end-entity TLS certificates issued under these roots must comply with the CA/B Forum TLS Baseline Requirements. Microsec's CP/CPS documents for TLS are published in English at [https://e-szigno.hu/en/pki-services/ca-certificates.html](https://e-szigno.hu/en/pki-services/ca-certificates.html) and at the static document repository (e.g., `https://static.e-szigno.hu/docs/szsz--min--ssl--EN--v3.03.pdf`). Certificate Transparency is implemented; Cloudflare Radar lists CT logs for Microsec-issued certificates ([CT record](https://radar.cloudflare.com/certificate-transparency/ca/327A3D761ABADEA034EB998406275CB1A4776EFDAE2FDF6D0168EA1C4F5567D0)).

Mozilla denied EV treatment for both the 2009 and 2017 roots in 2020, citing the need for Microsec to first demonstrate improved compliance with the BRs and EV Guidelines ([dev.security.policy discussion](https://groups.google.com/g/mozilla.dev.security.policy/c/jRKOr4nvOfY/m/KPFhaOJaAQAJ)).

## Past non-compliance

Multiple publicly-documented incidents are on record:

1. **2018 — Misissuance of Cisco VPN server certificates** ([dev.security.policy](https://groups.google.com/g/mozilla.dev.security.policy/c/Pcc1_luzwNs), [follow-up](https://groups.google.com/g/mozilla.dev.security.policy/c/FJA4ViMkbvs/m/tu8pzY1lAQAJ)): Microsec issued three certificates with three-year validity under a policy not aligned with the TLS BRs, reasoning that Cisco VPN server certificates were not in scope. Mozilla reviewers found the response "highly disconcerting" and noted systemic failure to track BR applicability. A recurrence occurred in 2020.

2. **2019–2020 — Doppelganger subordinate CA certificates and CCADB ALV failures** ([dev.security.policy](https://groups.google.com/g/mozilla.dev.security.policy/c/QqYm4BhFMHs)): Automated CCADB Audit Letter Validation found 9 discrepancies against Microsec's audit letters, partly from PDF formatting (SHA-256 fingerprints split by page breaks) and partly from undisclosed historical sub-CA versions ("doppelganger" certificates). These were resolved by revoking the deprecated sub-CA certificates on 2019-11-29.

3. **2020 — Incorrect OCSP Delegated Responder Certificate and delayed ICA revocation** ([Bug 1649947](https://bugzilla.mozilla.org/show_bug.cgi?id=1649947), [Bug 1651632](https://bugzilla.mozilla.org/show_bug.cgi?id=1651632)): Microsec issued TSA intermediate CA certificates that included the OCSP Signing EKU based on a misreading of a Mozilla forum discussion. Revocation of two of the four affected ICAs was delayed beyond 7 days because immediate revocation would have disrupted time-stamping services; the delay was disclosed and the root cause was a gap in internal EKU chain validation.

4. **2024 — Misissuance of EV TLS certificates without CPSuri** ([Bug 1886257](https://bugzilla.mozilla.org/show_bug.cgi?id=1886257)): A profile update misread CABF BR 2.2.2 (marking policyQualifiers as NOT RECOMMENDED) as applying to EV certificates, which still require CPSuri per the EV Guidelines. A total of 45 EV TLS certificates were affected. Two PSD2 QWAC certificates subject to the same mis-issuance were not revoked within the required 5-day window ([Bug 1887110](https://bugzilla.mozilla.org/show_bug.cgi?id=1887110)), as Microsec prioritised operational continuity for financial-sector customers; this was acknowledged as a BR violation and the certificates were subsequently revoked.

No distrust or removal action has been taken by any root program against Microsec roots as of June 2026. Searches of Mozilla Bugzilla ([Microsec bugs](https://bugzilla.mozilla.org/buglist.cgi?quicksearch=microsec)) and the CCADB show no open distrust proceedings.

## Transparency

- **CP/CPS**: Published in Hungarian and English at [https://e-szigno.hu/en/pki-services/ca-certificates.html](https://e-szigno.hu/en/pki-services/ca-certificates.html) and via static document links (e.g., `szsz--min--ssl--EN--v3.03.pdf` for TLS CPS). Documents are versioned and dated.
- **CCADB**: Microsec is enrolled in CCADB; intermediate CA certificates and audit records are disclosed. Case 00001692 covers the 2023 TLS root inclusion request.
- **Incident self-reporting**: Incidents have been self-reported on mozilla.dev.security.policy and via Mozilla Bugzilla. Response timeliness has been criticised in some cases (see Past non-compliance above).
- **Certificate Transparency**: Microsec logs issued TLS certificates to public CT logs; coverage is tracked at [Cloudflare Radar](https://radar.cloudflare.com/certificate-transparency/ca/327A3D761ABADEA034EB998406275CB1A4776EFDAE2FDF6D0168EA1C4F5567D0).

## Sources

- [Microsec / e-Szignó official website](https://e-szigno.hu)
- [e-Szignó CA certificates and hierarchy](https://e-szigno.hu/en/pki-services/ca-certificates.html)
- [Mozilla Bugzilla 510506 — Add Microsec e-Szigno Root CA 2009](https://bugzilla.mozilla.org/show_bug.cgi?id=510506)
- [Mozilla Bugzilla 557904 — Add Microsec e-Szigno Root CA 2009 to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=557904)
- [Mozilla Bugzilla 1445364 — Microsec new (ECC) Root Inclusion Request](https://bugzilla.mozilla.org/show_bug.cgi?id=1445364)
- [Mozilla Bugzilla 1645174 — Add e-Szigno Root CA 2017 to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=1645174)
- [mozilla.dev.security.policy — Request to Include e-Szigno Root CA 2017 and EV-enable 2009](https://groups.google.com/g/mozilla.dev.security.policy/c/jRKOr4nvOfY/m/KPFhaOJaAQAJ)
- [mozilla.dev.security.policy — Incident report: Misissuance of Cisco VPN server certificates](https://groups.google.com/g/mozilla.dev.security.policy/c/Pcc1_luzwNs)
- [mozilla.dev.security.policy — Misissuance of 2 Cisco VPN server auth certificates (recurrence)](https://groups.google.com/g/mozilla.dev.security.policy/c/FJA4ViMkbvs/m/tu8pzY1lAQAJ)
- [mozilla.dev.security.policy — Revoked subordinate CA certificates under 2009 root](https://groups.google.com/g/mozilla.dev.security.policy/c/QqYm4BhFMHs)
- [Mozilla Bugzilla 1649947 — Microsec: Incorrect OCSP Delegated Responder Certificate](https://bugzilla.mozilla.org/show_bug.cgi?id=1649947)
- [Mozilla Bugzilla 1651632 — Microsec: Failure to revoke noncompliant ICA within 7 days](https://bugzilla.mozilla.org/show_bug.cgi?id=1651632)
- [Mozilla Bugzilla 1886257 — Microsec: Misissuance of EV TLS certificate without CPSuri](https://bugzilla.mozilla.org/show_bug.cgi?id=1886257)
- [Mozilla Bugzilla 1887110 — Microsec: Delayed revocation of misissued certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=1887110)
- [CCADB Public Discussion — Microsec Ltd. CA Inclusion Request (2023 TLS root)](https://groups.google.com/a/ccadb.org/g/public/c/95h0Cjq8A0k)
- [Microsoft Trusted Root Program — May 2021 Deployment Notice](https://learn.microsoft.com/en-us/security/trusted-root/2021/may2021)
- [Cloudflare Radar — CT for Microsec e-Szigno Root CA](https://radar.cloudflare.com/certificate-transparency/ca/327A3D761ABADEA034EB998406275CB1A4776EFDAE2FDF6D0168EA1C4F5567D0)
