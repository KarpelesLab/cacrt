# Hongkong Post

Hongkong Post Certification Authority (HKPCA) is the public key infrastructure arm of Hongkong Post, the postal service of the Hong Kong Special Administrative Region. Established in 2000 as the first Recognised Certification Authority under Hong Kong's Electronic Transactions Ordinance (Cap. 553), HKPCA issues "e-Cert" digital certificates for identity authentication and TLS server authentication. HKPCA operations have been outsourced to Certizen Limited since 2023; the legal operator remains Hongkong Post (Postmaster General).

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| Hongkong Post Root CA 3 | `68dd7389.0` | RSA 4096 | 2042-06-03 | `5A:2F:C0:3F:0C:83:B0:90:BB:FA:40:60:4B:09:88:44:6C:76:36:18:3D:F9:84:6E:17:10:1A:44:7F:B8:EF:D6` |

## Rationale for inclusion

Hongkong Post Root CA 3 is included in all four major root programs:

- **Mozilla / NSS** — approved May 2019, with Websites trust bit and Extended Validation treatment (EV OID `2.23.140.1.1`); tracked in [Mozilla Bugzilla #1464306](https://bugzilla.mozilla.org/show_bug.cgi?id=1464306) and the CCADB public case 00000314. NSS inclusion landed in NSS 3.43 / Firefox 67; EV enabled in Firefox 68 ([Bug #1532753](https://bugzilla.mozilla.org/show_bug.cgi?id=1532753), [Bug #1532757](https://bugzilla.mozilla.org/show_bug.cgi?id=1532757)).
- **Microsoft** — included May 2019.
- **Google Chrome / Android** — included September 2020.
- **Apple** — included October 2021 (documented in [Apple's trusted root list for iOS 15.1 / macOS 12.1](https://support.apple.com/en-hk/103254)).

The CA is scoped to TLS server authentication (DV, OV, and EV) for the Hong Kong public sector and commercial organisations. It succeeded the expiring Hongkong Post Root CA 1, which had been trusted in Mozilla since 2010 ([Bugzilla #373537](https://bugzilla.mozilla.org/show_bug.cgi?id=373537)).

## CA/Browser Forum compliance

HKPCA is audited annually by PricewaterhouseCoopers (Hong Kong) against three WebTrust criteria:

- WebTrust Principles and Criteria for Certification Authorities
- WebTrust for SSL Baseline Requirements (CA/B Forum BRs)
- WebTrust for Extended Validation SSL (most recently v1.8, covering 1 December 2023 – 30 November 2024; audit report at [ecert.gov.hk/ev/Webtrust_EVSSL_2024.pdf](https://www.ecert.gov.hk/ev/Webtrust_EVSSL_2024.pdf))

The CA publishes a combined CP/CPS (no separate CP document). The current e-Cert (Server) CPS (v1.7.9 / updated July 2024) is available at [ecert.gov.hk/product/cps/ecert/index_text.html](https://www.ecert.gov.hk/product/cps/ecert/index_text.html) and bears OID `1.3.6.1.4.1.16030.1.7.21`. The CPS is structured in accordance with RFC 3647 and commits to compliance with the CA/B Forum Baseline Requirements.

HKPCA's root certificate records are disclosed in the CCADB. Certificate Transparency is required for all publicly-trusted TLS certificates issued under Root CA 3.

## Past non-compliance

**2016 — SHA-1 intermediate certificate (OneCRL)**
In August 2016 a SHA-1 end-entity certificate issued by the unconstrained intermediate "Hongkong Post e-Cert CA 1 - 10" was found in CT logs and serving live on a website, in violation of the CA/B Forum BR deadline of 1 January 2016. The intermediate used low-entropy serial numbers, creating a chosen-prefix collision risk. Mozilla added the two "Hongkong Post e-Cert CA 1 - 10" intermediates to OneCRL in September 2016 ([mozilla.dev.security.policy thread](https://groups.google.com/g/mozilla.dev.security.policy/c/Ng99HcqhZtI); [Bugzilla #1299579](https://bugzilla.mozilla.org/show_bug.cgi?id=1299579)). HKPCA committed to migrate remaining non-SSL issuance to a new SHA-256 constrained sub-CA.

**2016 — Missing Subject Alternative Name extension**
A separate Mozilla bug ([#1267332](https://bugzilla.mozilla.org/show_bug.cgi?id=1267332)) documented that HKPCA intermediates were issuing certificates without the required Subject Alternative Name extension.

**2018 — O/OU field length misissuance (undisclosed)**
During the Root CA 3 inclusion review, Mozilla noted a recent misissuance under Root CA 1 in which O and OU fields exceeded maximum length. The certificates were revoked, but no incident report had been filed. HKPCA addressed this for the new root's CPS but the absence of disclosure was noted as a concern.

**2024 — Delayed revocation of 1,105 TLS certificates ([Bugzilla #1886665](https://bugzilla.mozilla.org/show_bug.cgi?id=1886665))**
HKPCA identified 1,176 certificates (1,111 distinct) containing a defective Certificate Policies extension requiring revocation within 5 days under BR §4.9.1. Only 6 were revoked within the deadline; the remaining 1,105 were revoked late (final revocation completed 2024-05-29). The CA cited manual certificate management by government subscribers and the breadth of impact (over 70% of Hong Kong government websites). Mozilla required HKPCA to commit in writing to timely revocation, add pre-issuance linting (zlint upgrade + pkilint), obtain subscriber acknowledgement of revocation obligations, and ensure government entities have 24-hour replacement plans before issuance. All action items were marked complete; the bug was closed February 2025. No distrust action was taken against the root.

No broader distrust actions against Hongkong Post Root CA 3 have been identified in Mozilla, Chrome, Apple, or Microsoft root programs as of the date of this writing. A CCADB / Bugzilla search for additional incidents can be found at [bugzilla.mozilla.org](https://bugzilla.mozilla.org/buglist.cgi?query_format=advanced&short_desc=Hongkong+Post&short_desc_type=allwordssubstr&product=CA+Program).

## Transparency

- **CP/CPS**: Published at [ecert.gov.hk/product/cps/index.html](https://www.ecert.gov.hk/product/cps/index.html); updated versions released periodically (current Server CPS dated July 2024).
- **CCADB**: HKPCA discloses root and intermediate certificate records in the CCADB. The Root CA 3 inclusion case is CCADB case 00000314.
- **Audit reports**: WebTrust seal and audit reports are publicly linked from the CCADB and the ecert.gov.hk repository (e.g., [ecert.gov.hk/ev/Webtrust_EVSSL_2024.pdf](https://www.ecert.gov.hk/ev/Webtrust_EVSSL_2024.pdf)).
- **Incident self-reporting**: HKPCA has posted incident reports and remediation updates on Mozilla Bugzilla and dev-security-policy (see the 2024 delayed-revocation bug). The 2018 O/OU misissuance was not self-disclosed proactively.
- **Certificate Transparency**: Certificates issued under Root CA 3 are logged to publicly accessible CT logs, as required by the CA/B Forum BRs and Chrome CT policy. The 2016 SHA-1 issue was itself detected via CT log monitoring.

## Sources

- [Bugzilla #1464306 — Add Hongkong Post Root CA 3 (Mozilla approval)](https://bugzilla.mozilla.org/show_bug.cgi?id=1464306)
- [Bugzilla #1532753 — Add Hongkong Post Root CA 3 to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=1532753)
- [Bugzilla #1532757 — Enable EV treatment for Hongkong Post Root CA 3](https://bugzilla.mozilla.org/show_bug.cgi?id=1532757)
- [Bugzilla #1886665 — Hongkong Post: Delayed revocation of TLS certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=1886665)
- [Bugzilla #1299579 — Add Hongkong Post e-Cert CA 1 - 10 to OneCRL](https://bugzilla.mozilla.org/show_bug.cgi?id=1299579)
- [Bugzilla #1267332 — Hongkong Post e-Cert CA 1 - 10 issuing certs without SAN](https://bugzilla.mozilla.org/show_bug.cgi?id=1267332)
- [Bugzilla #373537 — Hongkong Post Root CA 1 inclusion](https://bugzilla.mozilla.org/show_bug.cgi?id=373537)
- [mozilla.dev.security.policy — Hongkong Post SHA-1 cert that could be used in TLS (2016)](https://groups.google.com/g/mozilla.dev.security.policy/c/Ng99HcqhZtI)
- [mozilla.dev.security.policy — New cross-certificate for Android compatibility (2022)](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/a2vWmLIKZy4)
- [Hongkong Post official announcement — Root CA3 browser inclusion (press/95)](https://www.hongkongpost.gov.hk/news/press/95_text.html)
- [Hongkong Post e-Cert CPS repository index](https://www.ecert.gov.hk/product/cps/index.html)
- [Hongkong Post e-Cert (Server) CPS v1.7.9 (PDF)](https://www.ecert.gov.hk/ev/e-Cert_(Server)_CPS-Eng-1.7.9.pdf)
- [WebTrust EV SSL Audit Report 2024 (PwC)](https://www.ecert.gov.hk/ev/Webtrust_EVSSL_2024.pdf)
- [Apple trusted root certificate list — iOS 15.1 / macOS 12.1](https://support.apple.com/en-hk/103254)
- [Hongkong Post e-Cert Service home page](https://www.hongkongpost.hk/en/services/ecert/index.html)
- [Hong Kong Digital Policy Office — Electronic Transactions Ordinance](https://www.digitalpolicy.gov.hk/en/our_work/digital_infrastructure/legal_framework/regulation/eto/)
