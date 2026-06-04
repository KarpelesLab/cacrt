# Disig

Disig, a.s. is a Slovak joint-stock company headquartered in Bratislava, Slovakia, that has provided certification and trust services since 1 April 2006. It is an eIDAS-qualified trust service provider regulated by the Slovak National Security Authority (NSA) and operates the largest Slovak NSA-accredited certification authority (CA Disig), along with eID-card CAs (SVK eID ACA/PCA/SCA). Its publicly-trusted TLS-issuing hierarchy (CA Disig Root R2 → CA Disig R2I2) is the entity represented in this folder; no mergers or renamed legal entities are involved.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| CA_Disig_Root_R2.pem | 2ae6433e.0 | RSA 4096 | 2042-07-19 | E2:3D:4A:03:6D:7B:70:E9:F5:95:B1:42:20:79:D2:B9:1E:DF:BB:1F:B6:51:A0:63:3E:AA:8A:9D:C5:F8:07:03 |

## Rationale for inclusion

CA Disig Root R2 is trusted by Mozilla, Microsoft, Apple, and Google (Chrome) for TLS server authentication, as confirmed by Disig's own documentation and the Microsoft June 2020 trusted-root changelog. Mozilla inclusion was approved in Firefox 23 / NSS 3.15 (Bug [792377], shipped ~2013) following a public review; the accompanying SHA-1 root (CA Disig Root R1) was voluntarily removed in NSS 3.35 / Firefox 59 (Bug [1420855]) because Disig had consolidated all SSL issuance under R2. Disig is a member of the CA/Browser Forum. The active subordinate CA is CN=CA Disig R2I2 Certification Service (expires 2029-09-28).

[792377]: https://bugzilla.mozilla.org/show_bug.cgi?id=792377
[1420855]: https://bugzilla.mozilla.org/show_bug.cgi?id=1420855

## CA/Browser Forum compliance

Disig audits against **ETSI EN 319 411-1** (NCP / OVCP / LCP policies) conducted by **QSCert**, a Slovak/Czech accredited conformity assessment body. The most recently cited audit covers the period ending 22 April 2022 (statement dated 5 June 2022); the audit letter is hosted at `qscert.sk`. The CP/CPS (v7.1, effective 21 April 2026) states that in the event of any conflict, the CA/Browser Forum TLS Baseline Requirements prevail. Key practices documented in the CP/CPS include:

- **Certificate Transparency**: pre-certificates submitted to CT logs per RFC 6962; embedded SCTs in issued certificates; automated linting with `zlint` and `ctlint`.
- **Multi-Perspective Issuance Corroboration**: network perspective checks on domain validation contacts, per current BR requirements.
- **Maximum TLS validity**: 395 days.
- **ACME support**: HTTP challenge (RFC 8555 §8.3) is implemented; domain validation also supports DNS change (BR 3.2.2.4.7) and other BR-permitted methods.
- **CCADB disclosure**: Subordinate CA certificates identifying Disig as subject are disclosed in CCADB per the CCADB Policy.

## Past non-compliance

No formal Mozilla CA compliance incident (Bugzilla "CA Certificate Compliance" component) has been found for Disig a.s. in publicly available records. A targeted search of Bugzilla and CCADB incident records returned only Disig's routine root lifecycle bugs (inclusion of R1/R2, removal of the expired legacy root in Bug [1247711], voluntary removal of R1 in Bug [1420855]). No distrust action, mis-issuance report, or delayed-revocation filing attributed to Disig was identified.

Searchers wanting to verify this independently can query:
- Bugzilla component [CA Certificate Compliance](https://bugzilla.mozilla.org/buglist.cgi?product=CA%20Program&component=CA%20Certificate%20Compliance&resolution=---&list_id=17204488) filtered to "Disig".
- [CCADB public incident list](https://www.ccadb.org/cas/incident-report).

[1247711]: https://bugzilla.mozilla.org/show_bug.cgi?id=1247711

## Transparency

- **CP/CPS**: Published openly on GitHub at [disig/Policies](https://github.com/disig/Policies/blob/main/TLS/CP_CPS_Disig_TLS.md) and mirrored as a PDF at [eidas.disig.sk](https://eidas.disig.sk/pdf/cp-disig-en.pdf). The GitHub repository tracks version history with dated changelog entries.
- **CCADB disclosure**: Disig discloses intermediate CA certificates in CCADB per CCADB Policy §3 as stated in CP/CPS §3.2.6.
- **Incident self-reporting**: The CP/CPS (§9) requires the Provider to file a Mozilla Bugzilla incident report within 72 hours of discovering any non-compliance and to update it until resolved; contact: tspnotify@disig.sk.
- **Certificate Transparency**: All issued TLS certificates embed SCTs; CT log submission is automated and checked with `ctlint`.
- **ETSI audit letters**: Hosted publicly on the QSCert website (qscert.sk).

## Sources

- [Disig a.s. — Certification Authority overview](https://eidas.disig.sk/en/provider/certification-authority/)
- [Disig a.s. — About us](https://disig.sk/en/disig/about-us/)
- [Disig TLS CP/CPS v7.1 (GitHub)](https://github.com/disig/Policies/blob/main/TLS/CP_CPS_Disig_TLS.md)
- [Disig TLS CP/CPS PDF (eidas.disig.sk)](https://eidas.disig.sk/pdf/cp-disig-en.pdf)
- [Mozilla Bugzilla 792377 — Add CA Disig root certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=792377)
- [Mozilla Bugzilla 823753 — Add Disig Root R1 and Disig Root R2 to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=823753)
- [Mozilla Bugzilla 1247711 — Remove expiring CA Disig root certificate](https://bugzilla.mozilla.org/show_bug.cgi?id=1247711)
- [Mozilla Bugzilla 1420855 — Remove CA Disig Root R1](https://bugzilla.mozilla.org/show_bug.cgi?id=1420855)
- [mozilla.dev.security.policy — Disig Request to include Renewed Roots](https://groups.google.com/g/mozilla.dev.security.policy/c/Z-cp6etonlc/m/aIJ1CEzmCtwJ)
- [Microsoft Trusted Root Program — June 2020 changelog (includes CA Disig Root R2)](https://github.com/MicrosoftDocs/security/blob/main/security-docs/trusted-root/2020/June2020.md)
- [E2Encrypted — CA Disig Root R2 audit record](https://www.e2encrypted.com/certs/b561ebeaa4dee4254b691a98a55747c234c7d971/)
- [CCADB — Incident reporting guidelines](https://www.ccadb.org/cas/incident-report)
- [CA/Browser Forum — Audit criteria](https://cabforum.org/about/information/auditors-and-assessors/audit-criteria/)
