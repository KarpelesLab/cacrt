# Certigna

Certigna is a public certificate authority operated by **Dhimyotis**, a French company headquartered in Villeneuve-d'Ascq, France. Dhimyotis is one of France's largest CAs, issuing certificates for the general public, private companies, and public administrations (including French government teleservices). This folder covers the newer **Certigna Root CA** root (SHA-256 root, 2013–2033), which supersedes the earlier SHA-1 "Certigna" root (2007–2027, not included here as it is nearing end-of-life and is out of scope for new issuance).

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| Certigna Root CA | `f51bb24c.0` | RSA 4096 | 2033-10-01 | `D4:8D:3D:23:EE:DB:50:A4:59:E5:51:97:60:1C:27:77:4B:9D:7B:18:C9:4D:5A:05:95:11:A1:02:50:B9:31:68` |

Subject: `C=FR, O=Dhimyotis, OU=0002 48146308100036, CN=Certigna Root CA`

## Rationale for inclusion

**Certigna Root CA** was approved by Mozilla and included in NSS 3.41 / Firefox 65 (shipped early 2019), with both Websites and Email trust bits enabled. [Mozilla Bugzilla #1265683][bz1265683] tracks the inclusion request; [NSS bug #1505614][bz1505614] tracks the code change. The root is also included in the Mozilla Firefox root store as confirmed by the [CCADB CA Certificates In Firefox report][ccadb-firefox].

The CA is audited by [LSTI][lsti], a French accredited Conformity Assessment Body (CAB), against ETSI EN 319 411 and the CA/Browser Forum TLS Baseline Requirements. An audit attestation letter covering TLS BR compliance (audit period ending 2024) is published on the LSTI website. The root is listed as serving "France, Europe" with a geographic focus on French-speaking markets.

The earlier "Certigna" root (2007, 2048-bit RSA / SHA-1) was accepted into Apple's and Microsoft's root programs circa 2007–2008 [Bugzilla #393166][bz393166].

## CA/Browser Forum compliance

- **Audit framework:** ETSI EN 319 411-1 (LCP, NCP+) and ETSI EN 319 411-2 (QCP+, eIDAS), audited by [LSTI][lsti] under ISO/IEC 17065:2012 and ETSI EN 319 403. Audit attestation letters are published on the LSTI website as required by CCADB policy.
- **Baseline Requirements:** The CA is committed to the CA/B Forum TLS Baseline Requirements; annual audit scope covers TLS-BR, TLS-EV, CS-BR, S/MIME BR, and Network and Certificate System Security Requirements (NCSSRs).
- **CCADB disclosure:** Intermediate certificates are disclosed in the CCADB. A prior failure to disclose intermediates was corrected (see Past non-compliance below).
- **Certificate Transparency:** All publicly-trusted TLS certificates must be logged to CT logs per BR requirements; the CP/CPS documents reference CT obligations.
- **ACME support:** Noted as accepted for certificate issuance in the Certigna Multipurpose CA CP/CPS (v1.1, 2024-06-25) and subsequent revisions. [CPS reference][cps-multipurpose]
- **CP/CPS:** Published at [cps.certigna.com][cps-root] and [politique.certigna.fr][politique], with usage-specific documents for TLS, S/MIME, Code Signing, and Multipurpose CAs.

## Past non-compliance

The following publicly-documented compliance incidents are on record. No distrust or removal actions have been taken against Certigna in any major root program as of the time of writing.

**1. CAA misissuance — one certificate (2018)**
During the Certigna Root CA inclusion review, Certigna self-reported that one TLS certificate had been issued despite a DNS CAA validation failure. A Registration Officer had manually authorized the issuance in contradiction to the CAA check result. Mozilla reviewers noted that CAA failure is misissuance regardless of whether the organization consented through other means. Certigna remediated by making the CAA control a blocking, non-overridable step in its RA portal. Discussed on [mozilla.dev.security.policy][mdsp-caa] and in [Bugzilla #1265683][bz1265683].

**2. Certificates with validity > 398 days; delayed revocation (2020)**
[Bugzilla #1674082][bz1674082] (and duplicate [#1667744][bz1667744]): 76 certificates and pre-certificates were issued between September 1–28, 2020 with a validity of 398 days + 1 second due to an off-by-one error in the certificate generation service (the notAfter computation used an inclusive-end convention). Revocation of the affected certificates spanned from October 2 to October 22, 2020, substantially exceeding the required 5-day revocation window. The delay was attributed to operational impact on French government services; Certigna also initially failed to file a required incident report for the delayed revocation. Mozilla reviewer Ryan Sleevi noted the CA "could have revoked the existing certificates as scheduled." Resolved FIXED; no distrust action taken.

**3. Undisclosed intermediate certificates (2018)**
[Bugzilla #1451949][bz1451949]: One or more intermediate CA certificates chaining to the Certigna root were not disclosed in the CCADB, violating Mozilla's Root Store Policy. The intermediates were promptly added to CCADB after the bug was filed. Resolved FIXED; whiteboard tagged `[disclosure-failure]`.

**4. clientAuth-only subscriber certificates (2025)**
[Bugzilla #1983955][bz1983955]: 3,438 certificates were issued under the legacy "Certigna Services CA" intermediate with only the `clientAuth` EKU, lacking the `serverAuth` EKU and the required CA/Browser Forum reserved policy OID. The non-compliance dated from 2015-11-25. The issue was raised by Sectigo. Certigna initially contended these certificates were outside TLS BR scope; Mozilla and Chrome reviewers disagreed. Issuance was suspended on 2025-08-19; 692 still-valid certificates were revoked or expired by 2025-08-24. Certigna deployed a new dedicated intermediate for client authentication and updated CP/CPS to address policy chaining. Resolved FIXED; whiteboard tagged `[ov-misissuance]`.

For a current CCADB/Bugzilla search covering open or future incidents, see: [CCADB CA Certificates In Firefox][ccadb-firefox] and [Bugzilla CA Program][bz-ca-program].

## Transparency

- **CP/CPS:** Publicly available in French and English at [cps.certigna.com][cps-root] and [politique.certigna.fr][politique]. Usage-specific CPS documents for TLS ([DPCCertignaTLSCA][cps-tls]), S/MIME, Code Signing, and Multipurpose are maintained separately and updated regularly (most recent versions dated 2025–2026).
- **CCADB disclosure:** Certigna's root and all non-technically-constrained intermediates are disclosed in the CCADB per Mozilla Root Store Policy requirements.
- **Incident self-reporting:** Certigna has self-reported incidents (e.g., the 2018 CAA misissuance and the 2020 validity-period issue). A procedural gap in filing the 2020 delayed-revocation incident report was acknowledged and corrected.
- **Certificate Transparency:** All BR-covered TLS certificates are subject to CT logging as required by the CA/B Forum Baseline Requirements and enforced by browser root programs.
- **Audit attestation letters:** Published on the [LSTI website][lsti] and referenced in CCADB records, as required by CCADB policy for ETSI-audited CAs.

## Sources

- [Bugzilla #393166 — Add Certigna certificates to Mozilla root CA list (2007)][bz393166]
- [Bugzilla #1265683 — Add Certigna Root CA root certificate(s)][bz1265683]
- [Bugzilla #1505614 — Add Certigna Root CA to NSS (inclusion code change)][bz1505614]
- [Bugzilla #1451949 — Dhimyotis/Certigna: Intermediate Cert(s) not disclosed in CCADB][bz1451949]
- [Bugzilla #1667744 — Dhimyotis/Certigna: Certificates issued with validity periods greater than 398 days][bz1667744]
- [Bugzilla #1674082 — Dhimyotis/Certigna: Certificates issued with validity periods greater than 398 days (main bug)][bz1674082]
- [Bugzilla #1983955 — Certigna: Subscriber certificate with EKU clientAuth only (2025)][bz1983955]
- [mozilla.dev.security.policy — Incident Report: Misissuance of one certificate without DNS CAA authorization][mdsp-caa]
- [mozilla.dev.security.policy — Certigna Root Renewal Request][mdsp-renewal]
- [CCADB — CA Certificates In Firefox Report][ccadb-firefox]
- [Certigna CP/CPS page (certigna.com)][cps-page]
- [Certigna Root CA CPS v4.9 (EN, archived)][cps-root]
- [Certigna TLS CA CPS (EN)][cps-tls]
- [Certigna Multipurpose CA CPS (EN)][cps-multipurpose]
- [LSTI — Auditor website][lsti]
- [LSTI — Audit Attestation Letter TLS BR v1.0 for Certigna (2024)][lsti-aal]

[bz393166]: https://bugzilla.mozilla.org/show_bug.cgi?id=393166
[bz1265683]: https://bugzilla.mozilla.org/show_bug.cgi?id=1265683
[bz1505614]: https://bugzilla.mozilla.org/show_bug.cgi?id=1505614
[bz1451949]: https://bugzilla.mozilla.org/show_bug.cgi?id=1451949
[bz1667744]: https://bugzilla.mozilla.org/show_bug.cgi?id=1667744
[bz1674082]: https://bugzilla.mozilla.org/show_bug.cgi?id=1674082
[bz1983955]: https://bugzilla.mozilla.org/show_bug.cgi?id=1983955
[mdsp-caa]: https://groups.google.com/g/mozilla.dev.security.policy/c/mVD1QoGXBOQ
[mdsp-renewal]: https://groups.google.com/g/mozilla.dev.security.policy/c/z7iDk9CdTFo
[ccadb-firefox]: https://ccadb.my.salesforce-sites.com/mozilla/CACertificatesInFirefoxReport
[cps-page]: https://www.certigna.com/autorites/index.xhtml
[cps-root]: https://cps.certigna.com/archives/DPC_Certigna_Root_CA_v4.9_en.pdf
[cps-tls]: https://cps.certigna.com/DPCCertignaTLSCA.pdf
[cps-multipurpose]: https://cps.certigna.com/DPCCertignaMultipurposeCA.pdf
[politique]: https://politique.certigna.fr/en/DPCunique.pdf
[lsti]: https://www.lsti-certification.fr
[lsti-aal]: https://www.lsti-certification.fr/wp-content/uploads/2024/07/23-1713-Audit-Attestation-Letter-TLS-BR-V1.0_Certigna_SS.pdf
[bz-ca-program]: https://bugzilla.mozilla.org/buglist.cgi?product=CA%20Program&component=CA%20Certificate%20Compliance&resolution=---
