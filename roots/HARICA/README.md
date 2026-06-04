# HARICA

HARICA (Hellenic Academic and Research Institutions Certification Authority) is a non-profit public-key infrastructure operated by the Greek Universities Network (GUnet) and hosted at the IT Center of Aristotle University of Thessaloniki, Athens, Greece. It serves as Greece's primary publicly-trusted CA for TLS server authentication and issues certificates globally under both the legacy "Hellenic Academic and Research Institutions Cert. Authority" legal name and the shorter "Hellenic Academic and Research Institutions CA" branding used for the 2021 hierarchy; all four roots in this folder are from the same operator.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| HARICA TLS ECC Root CA 2021 | `ecccd8db.0` | ECC P-384 | 2045-02-13 | `3F:99:CC:47:4A:CF:CE:4D:FE:D5:87:94:66:5E:47:8D:15:47:73:9F:2E:78:0F:1B:B4:CA:9B:13:30:97:D4:01` |
| HARICA TLS RSA Root CA 2021 | `9f727ac7.0` | RSA 4096 | 2045-02-13 | `D9:5D:0E:8E:DA:79:52:5B:F9:BE:B1:1B:14:D2:10:0D:32:94:98:5F:0C:62:D9:FA:BD:9C:D9:99:EC:CB:7B:1D` |
| Hellenic Academic and Research Institutions ECC RootCA 2015 | `7719f463.0` | ECC P-384 | 2040-06-30 | `44:B5:45:AA:8A:25:E6:5A:73:CA:15:DC:27:FC:36:D2:4C:1C:B9:95:3A:06:65:39:B1:15:82:DC:48:7B:48:33` |
| Hellenic Academic and Research Institutions RootCA 2015 | `32888f65.0` | RSA 4096 | 2040-06-30 | `A0:40:92:9A:02:CE:53:B4:AC:F4:F2:FF:C6:98:1C:E4:49:6F:75:5E:6D:45:FE:0B:2A:69:2B:CD:52:52:3F:36` |

## Rationale for inclusion

All four roots carry the TLS server-authentication trust bit and are included in Mozilla's NSS root store. HARICA has been a member of the Mozilla Root CA Program since 2012, the Microsoft Root Program since June 2013, the Apple Root CA Program since September 2013, the 360 Root Program since October 2019, and the Oracle Root Program since April 2021. According to HARICA's own disclosures it is currently the only Root CA operator based in Greece included in all major global root programs. The 2021 roots also have EV-policy OID 2.23.140.1.1 enabled for Extended Validation TLS. [[Mozilla public discussion, 2021]][mozilla-discussion-2021] [[CCADB case 00000730]][ccadb-case]

## CA/Browser Forum compliance

HARICA is audited annually by QMSCERT (Greece) against ETSI EN 319 411-1 and ETSI EN 319 411-2, covering the CA/Browser Forum Baseline Requirements (BR), Network Security Guidelines, and TLS Extended Validation Guidelines. The most recent audit period on record in CCADB for the 2021 TLS roots runs 2024-03-30 to 2025-03-29 with a statement date of 2025-06-27; no deviations were reported across Standard, NetSec, TLS BR, and TLS EVG criteria. [[CCADB case 00000730]][ccadb-case]

HARICA discloses all root and intermediate CA information in CCADB. Its CP/CPS (currently version 4.11, effective 2025-11-28) is publicly available at `https://repo.harica.gr/documents/CPS` and is structured per RFC 3647. The document explicitly references CCADB Policy 2.0 incorporation, DNSSEC validation for CAA and non-email DCV methods, a mass-certificate-revocation plan, and support for the ACME DNS Labeled with Account ID domain-validation method. [[HARICA CP/CPS repository]][cps-repo] [[CCADB case 00000730]][ccadb-case]

All TLS certificates are logged to public Certificate Transparency logs as required by Chrome and Safari root program policies. HARICA does not operate its own CT log. [[Chrome CT policy]][chrome-ct]

## Past non-compliance

Three publicly-documented compliance issues have been filed in Mozilla Bugzilla. None resulted in distrust or removal of any HARICA root.

**Bug 1649945 — Incorrect OCSP Delegated Responder Certificate (2020).** Reported July 2020 by Ryan Sleevi. Forty-one technically-constrained non-TLS sub-CA certificates were issued without the mandatory `id-pkix-ocsp-nocheck` extension, in violation of RFC 6960 and the Baseline Requirements. HARICA determined the affected CAs were never actually enabled to sign OCSP responses (cryptographically verifiable via HSM audit logs), limiting practical risk. Remediation included stopping issuance from affected sub-CAs (2020-07-06), migrating subscribers to new compliant hierarchies, revoking all 41 sub-CAs in two tranches (September and November 2020), and destroying all associated keys under external auditor supervision. Bug marked RESOLVED FIXED. [[Bug 1649945]][bug-1649945]

**Bug 1699796 — Certificates with invalid policy tree (2021).** HARICA self-identified 33 non-expired TLS end-entity certificates issued with a policy OID not present in the issuing CA's `certificatePolicies` extension, violating RFC 5280 path validation rules. Detection occurred 2021-03-18; a reviewer noted revocation on 2021-03-24 appeared to miss the 5-day deadline. Ryan Sleevi credited the proactive self-evaluation while noting future timelines should be date-and-time-stamped. Bug marked RESOLVED FIXED. [[Bug 1699796]][bug-1699796]

**Bug 1942130 — S/MIME certificate issuance without proper email validation (2025).** On 2025-01-15 a subscriber reported that a refactored REST API workflow introduced on 2025-01-08 omitted the final cross-check for email-address validation in the mailbox-validated S/MIME path, allowing issuance without confirmed validation. Five mis-issued certificates were identified (all to one subscriber) and revoked within 24 hours. A central pre-issuance validation checkpoint was implemented as a long-term fix (deployed 2025-03-21). Note: this affects S/MIME trust, not TLS; the TLS roots in this folder are not implicated. Bug marked RESOLVED FIXED. [[Bug 1942130]][bug-1942130]

**Bug 1963629 — Certificate Problem Report email alias not working (2025).** One of the two CPR email aliases listed in HARICA's CP/CPS and CCADB expired on 2025-04-18 due to an internal procedure failure (the alias had been set to expire in 2025 when created in 2005, and renewal reminders were not acted on). The alias was restored within 12 days; HARICA updated both aliases to expire in 2040, assigned two responsible persons for renewal alerts, and updated CCADB. Bug marked RESOLVED FIXED. [[Bug 1963629]][bug-1963629]

A search of CCADB and Mozilla Bugzilla finds no distrust actions, removal requests, or pending investigations against any HARICA root as of the writing of this file.

## Transparency

HARICA's CP/CPS is publicly versioned and permanently available at `https://repo.harica.gr/documents/CPS` (latest) and at versioned URLs such as `https://repo.harica.gr/documents/CPS-EN-4.11.pdf`. All root and intermediate CA certificates, CRLs, and OCSP endpoints are listed in the repository at `https://repo.harica.gr/`. HARICA files self-reported incident reports in Mozilla Bugzilla and discloses audit attestations in CCADB. It is a member of the PKI Consortium (since September 2019). Dimitris Zacharopoulos of HARICA has served as CA/Browser Forum Chair (2019–2020 and again from 2022). [[HARICA repository]][harica-repo] [[PKI Consortium]][pkic] [[CA/B Forum]][cabf-chair]

## Sources

- [HARICA website][harica-home]
- [HARICA PKI repository (CP/CPS, root certs, CRLs)][harica-repo]
- [HARICA CP/CPS (latest)][cps-repo]
- [CCADB case 00000730 — HARICA 2021 TLS roots][ccadb-case]
- [Mozilla dev-security-policy: Public Discussion of HARICA Root CA Inclusion Requests (2021)][mozilla-discussion-2021]
- [Mozilla Bugzilla Bug 711594 — Add Hellenic Academic and Research Institutions RootCA 2011][bug-711594]
- [Mozilla Bugzilla Bug 1759815 — Remove Hellenic Academic and Research Institutions RootCA 2011][bug-1759815]
- [Mozilla Bugzilla Bug 1649945 — HARICA: Incorrect OCSP Delegated Responder Certificate][bug-1649945]
- [Mozilla Bugzilla Bug 1699796 — HARICA: Certificates with invalid policy tree][bug-1699796]
- [Mozilla Bugzilla Bug 1942130 — HARICA: S/MIME certificate issuance without proper validation][bug-1942130]
- [Mozilla Bugzilla Bug 1963629 — HARICA: One of the two Certificate Problem Report email aliases not working][bug-1963629]
- [HARICA 2021 TLS roots announcement (news.harica.gr)][harica-news-2021]
- [PKI Consortium — HARICA member profile][pkic]
- [CA/B Forum — HARICA (Dimitris Zacharopoulos, chair)][cabf-chair]
- [Chrome Root Program Policy][chrome-ct]
- [Cloudflare Radar CT — Hellenic Academic and Research Institutions RootCA 2015][cf-radar]

[harica-home]: https://www.harica.gr/en/
[harica-repo]: https://repo.harica.gr/
[cps-repo]: https://repo.harica.gr/documents/CPS
[ccadb-case]: https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000730
[mozilla-discussion-2021]: https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/UAmBtcVvBKw
[bug-711594]: https://bugzilla.mozilla.org/show_bug.cgi?id=711594
[bug-1759815]: https://bugzilla.mozilla.org/show_bug.cgi?id=1759815
[bug-1649945]: https://bugzilla.mozilla.org/show_bug.cgi?id=1649945
[bug-1699796]: https://bugzilla.mozilla.org/show_bug.cgi?id=1699796
[bug-1942130]: https://bugzilla.mozilla.org/show_bug.cgi?id=1942130
[bug-1963629]: https://bugzilla.mozilla.org/show_bug.cgi?id=1963629
[harica-news-2021]: https://news.harica.gr/article/2021_harica_tls_roots/
[pkic]: https://pkic.org/members/harica/
[cabf-chair]: https://cabforum.org/
[chrome-ct]: https://googlechrome.github.io/chromerootprogram/
[cf-radar]: https://radar.cloudflare.com/certificate-transparency/ca/A040929A02CE53B4ACF4F2FFC6981CE4496F755E6D45FE0B2A692BCD52523F36
