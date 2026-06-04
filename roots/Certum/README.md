# Certum

Certum is the public CA brand of **Asseco Data Systems S.A.** (formerly **Unizeto Technologies S.A.**), headquartered in Szczecin, Poland. Founded in 1998, it is the oldest public commercial CA in Poland and one of the earliest public-key certification authorities in Europe. This folder merges roots issued under both the Unizeto and Asseco legal entities, which are the same operational CA following corporate rebranding in 2016.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| Certum EC-384 CA | `9482e63a.0` | ECC P-384 | 2043-03-26 | `6B:32:80:85:62:53:18:AA:50:D1:73:C9:8D:8B:DA:09:D5:7E:27:41:3D:11:4C:F7:87:A0:F5:D0:6C:03:0C:F6` |
| Certum Trusted Network CA 2 | `40193066.0` | RSA 4096 | 2046-10-06 | `B6:76:F2:ED:DA:E8:77:5C:D3:6C:B0:F6:3C:D1:D4:60:39:61:F4:9E:62:65:BA:01:3A:2F:03:07:B6:D0:B8:04` |
| Certum Trusted Network CA | `48bec511.0` | RSA 2048 | 2029-12-31 | `5C:58:46:8D:55:F5:8E:49:7E:74:39:82:D2:B5:00:10:B6:D1:65:37:4A:CF:83:A7:D4:A3:2D:B7:68:C4:40:8E` |
| Certum Trusted Root CA | `e35234b1.0` | RSA 4096 | 2043-03-16 | `FE:76:96:57:38:55:77:3E:37:A9:5E:7A:D4:D9:CC:96:C3:01:57:C1:5D:31:76:5B:A9:B1:57:04:E1:AE:78:FD` |

**Issuer O field notes:** `Certum Trusted Network CA` and `Certum Trusted Network CA 2` carry `O=Unizeto Technologies S.A.`; `Certum EC-384 CA` and `Certum Trusted Root CA` carry `O=Asseco Data Systems S.A.`, reflecting the 2016 corporate rename.

## Rationale for inclusion

Certum's roots are included in all major root programs:

- **Mozilla NSS** — original roots added 2002 ([bug 167572](https://bugzilla.mozilla.org/show_bug.cgi?id=167572)); Certum Trusted Network CA added via [bug 532377](https://bugzilla.mozilla.org/show_bug.cgi?id=532377); Certum Trusted Network CA 2 added via [bug 999378](https://bugzilla.mozilla.org/show_bug.cgi?id=999378) (EV enabled in NSS 3.23); Certum Trusted Root CA and Certum EC-384 CA approved April 2021, included in NSS 3.66 / Firefox 90 ([bug 1598577](https://bugzilla.mozilla.org/show_bug.cgi?id=1598577), [bug 1707097](https://bugzilla.mozilla.org/show_bug.cgi?id=1707097)).
- **Microsoft Trusted Root Program** — all four roots are listed as trusted; the EC-384 and Trusted Root CA roots were added in the [July 2020 deployment](https://learn.microsoft.com/nb-no/security/trusted-root/2020/july2020).
- **Apple** — roots are present in macOS and iOS trust stores.
- **Chrome** — roots appear in the Chrome trusted root listing.

Trust bits for the two newer roots (Certum EC-384 CA, Certum Trusted Root CA) cover TLS server authentication and S/MIME (email). EV treatment is enabled for both in Mozilla. Note: Chrome and Firefox are phasing out root certificates older than 15 years for TLS, which will eventually affect Certum Trusted Network CA (expires 2029) and Certum Trusted Network CA 2 (expires 2046, but issued 2011).

## CA/Browser Forum compliance

Certum conducts annual audits against the **WebTrust for CAs** and **WebTrust for SSL Baseline Requirements** schemes. It was the first Polish CA to receive a WebTrust seal (2002) and has held WebTrust EV SSL and WebTrust EV Code Signing seals. Auditors have included Ernst & Young and BDO. Certum also undergoes **eIDAS** conformity assessments (audited by TÜV Informationstechnik GmbH) for its qualified trust services under EU Regulation 910/2014.

Certum participates in CA/Browser Forum working groups (representatives listed in SCWG meeting minutes, e.g., Kateryna Aleksieieva and Aleksandra Kurosz of Asseco Data Systems). It discloses intermediate CA certificates and audit reports in the [CCADB](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000519) (Case 00000519).

Certum supports the **ACME protocol** (RFC 8555) via `https://acme.certum.pl/directory`, enabling automated certificate issuance and renewal ([ACME documentation](https://support.certum.eu/en/how-to-activate-a-certum-ssl-certificate-using-acme/)). All publicly-trusted TLS certificates are logged to Certificate Transparency logs as required by the Baseline Requirements.

CP and CPS documents are published at `files.certum.eu` in accordance with RFC 3647; the current TLS CPS is v7.11 (March 2025): [CCP-DK02-ZK02-CPS v7.11](https://www.certum.eu/en/wp-content/uploads/2025/03/CCP-DK02-ZK02-Certification-Practice-Statement-of-Certum-Certification-Services_v7.11.pdf).

## Past non-compliance

The following publicly-documented incidents are on record in Mozilla Bugzilla. No distrust action has been taken against Certum's roots.

**CAA mis-issuance — CNAME to restrictive record (2017–2018)** — [Bug 1409766](https://bugzilla.mozilla.org/show_bug.cgi?id=1409766): Certum's CAA validation failed for domains with a `www`-prefixed CNAME pointing to a domain with a restrictive CAA record. One certificate was confirmed mis-issued. The immediate fix was delayed roughly a year due to architectural complexity; the permanent fix was deployed 2018-09-11.

**CAA mis-issuance — mixed wildcard/non-wildcard SAN (2017–2018)** — [Bug 1420860](https://bugzilla.mozilla.org/show_bug.cgi?id=1420860): Certum validated CAA for the wildcard SAN only and did not re-check the base domain. After approximately six months without a substantive response, Mozilla warned that continued silence could lead to sanctions including root removal. Certum attributed the problem to misinterpretation of RFC 6844 and deployed a fix.

**Delayed revocation — Debian weak keys (2018)** — [Bug 1435770](https://bugzilla.mozilla.org/show_bug.cgi?id=1435770): Two certificates containing known-weak Debian keys were not revoked within the 24-hour BR deadline. Root cause was a fingerprint mismatch in Certum's stored hashes (the Modulus prefix and line endings differed between storage and the presented CSR).

**EV certificate mis-issuance (2018)** — [Bug 1451228](https://bugzilla.mozilla.org/show_bug.cgi?id=1451228): An EV certificate was issued for `zegarownia.pl` with the domain-name word in the O field rather than the legal company name. Attributed to human error during manual data entry.

**Corrupted CRL (2018)** — [Bug 1511459](https://bugzilla.mozilla.org/show_bug.cgi?id=1511459): A CRL with a corrupted signature was served for approximately 19 days. Certum prioritised fixing the problem before reporting to Mozilla, which itself was flagged as a disclosure delay.

**Intermediate CAs not listed in audit report (2019)** — [Bug 1598277](https://bugzilla.mozilla.org/show_bug.cgi?id=1598277): 18 certificates failed Audit Letter Validation; Certum determined 15 were genuine non-compliances and revoked all 15.

**Cross-signed non-EV-audited roots enabled for EV (2022–2023)** — [Bug 1815355](https://bugzilla.mozilla.org/show_bug.cgi?id=1815355): Certum had cross-signed SSL.com's subordinate CAs from an EV-enabled root (Certum Trusted Network CA) in 2018, without those sub-CAs having EV audits, and with `anyPolicy` in the cross-certificate contrary to BR 7.1.6.3. Auditor BDO added retrospective EV audit scope; the cross-certificates expired September 2023 and were not renewed with the same constraints. Mozilla closed the bug as FIXED in June 2023.

**S/MIME SAN errors (2024)** — [Bug 1879845](https://bugzilla.mozilla.org/show_bug.cgi?id=1879845): 96 S/MIME certificates were issued with invalid SubjectAlternativeName content between 16 January and 12 February 2024. Issuance was halted, a fix was deployed 13 February 2024, and all affected certificates were revoked.

For a broader search of open/closed Certum issues, see the [Bugzilla query for "Asseco DS / Certum"](https://bugzilla.mozilla.org/buglist.cgi?short_desc=Asseco+DS+%2F+Certum&short_desc_type=allwordssubstr).

## Transparency

- **CP/CPS**: Published at [files.certum.eu](https://files.certum.eu/documents/repsitory/) per RFC 3647; current TLS CPS is v7.11.
- **CCADB**: Certum discloses roots and intermediates in the Common CA Database ([CCADB case 00000519](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000519)); CAA identifiers (`certum.pl`, `certum.eu`) are listed in the [All CAA Identifiers Report](https://ccadb.my.salesforce-sites.com/ccadb/AllCAAIdentifiersReport).
- **Incident self-reporting**: Incidents have been self-reported to Mozilla's dev-security-policy list and tracked in Bugzilla, though Bug 1511459 and Bug 1420860 noted delays in reporting.
- **Certificate Transparency**: TLS certificates are submitted to public CT logs as required by the Baseline Requirements. Certum operates CT monitoring via [crt.sh](https://crt.sh/?o=Asseco+Data+Systems+S.A.) and integrated CT disclosure checks after Bug 1815355.

## Sources

- [Certum — About Certum by Asseco](https://www.certum.eu/en/certum-by-asseco/)
- [Certum — Root Certificates page](https://www.certum.eu/en/cert_expertise_root_certificates/)
- [Certum — CPS v7.11 (March 2025)](https://www.certum.eu/en/wp-content/uploads/2025/03/CCP-DK02-ZK02-Certification-Practice-Statement-of-Certum-Certification-Services_v7.11.pdf)
- [Certum — ACME documentation](https://support.certum.eu/en/how-to-activate-a-certum-ssl-certificate-using-acme/)
- [CCADB — Certum case 00000519](https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000519)
- [Mozilla Bugzilla 167572 — Add Unizeto CERTUM CA certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=167572)
- [Mozilla Bugzilla 999378 — Add CERTUM's SHA2 root certificate (Trusted Network CA 2)](https://bugzilla.mozilla.org/show_bug.cgi?id=999378)
- [Mozilla Bugzilla 1598577 — Add Asseco DS / Certum root certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=1598577)
- [Mozilla Bugzilla 1707097 — Add Certum root certs to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=1707097)
- [Mozilla Bugzilla 1409766 — CAA mis-issuance on CNAME to restrictive CAA record](https://bugzilla.mozilla.org/show_bug.cgi?id=1409766)
- [Mozilla Bugzilla 1420860 — CAA mis-issuance on mixed wildcard/non-wildcard SAN](https://bugzilla.mozilla.org/show_bug.cgi?id=1420860)
- [Mozilla Bugzilla 1435770 — Non-BR-compliant issuance, Debian weak keys](https://bugzilla.mozilla.org/show_bug.cgi?id=1435770)
- [Mozilla Bugzilla 1451228 — EV certificate mis-issuance](https://bugzilla.mozilla.org/show_bug.cgi?id=1451228)
- [Mozilla Bugzilla 1511459 — Corrupted CRL](https://bugzilla.mozilla.org/show_bug.cgi?id=1511459)
- [Mozilla Bugzilla 1598277 — Intermediate CAs not listed in audit report](https://bugzilla.mozilla.org/show_bug.cgi?id=1598277)
- [Mozilla Bugzilla 1815355 — Cross-signed non-EV-audited root with EV-enabled root](https://bugzilla.mozilla.org/show_bug.cgi?id=1815355)
- [Mozilla Bugzilla 1879845 — S/MIME certificates with error in subjectAlternativeName](https://bugzilla.mozilla.org/show_bug.cgi?id=1879845)
- [mozilla.dev.security.policy — Public Discussion of Asseco's Root Inclusion Request](https://groups.google.com/g/mozilla.dev.security.policy/c/_A7OX4Tz65k)
- [Microsoft Trusted Root Program — July 2020 Deployment Notice](https://learn.microsoft.com/nb-no/security/trusted-root/2020/july2020)
- [CCADB — All CAA Identifiers Report](https://ccadb.my.salesforce-sites.com/ccadb/AllCAAIdentifiersReport)
