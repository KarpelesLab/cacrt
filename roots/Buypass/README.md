# Buypass

> **Status: DISABLED (removed from the compiled store as of 2026-06-04).**
> Buypass discontinued public TLS/SSL issuance — last issuance 2025-10-31, and
> all Buypass TLS/SSL certificates expired no later than 2026-04-15. As that date
> has passed, no valid Buypass-issued TLS server certificate remains in the wild,
> so these roots serve no purpose as trust anchors. The certificates are retained
> here as `*.disabled` (excluded from the build) to preserve the audit trail and
> this rationale. See [`CHANGELOG-roots.md`](../../CHANGELOG-roots.md).

Buypass AS (org. no. 983 163 327) is a Norwegian publicly-owned corporation headquartered in Oslo, Norway, established in 2001. It operates as a Certification Authority and Qualified Trust Service Provider (QTSP) under the eIDAS Regulation, issuing digital certificates for TLS/SSL, electronic identification, electronic signatures, and electronic seals. Both roots in this folder are issued directly by Buypass AS under the same legal entity; no additional brands or legal entities are merged here.

**Note:** Buypass announced on 18 August 2025 that it is discontinuing its public TLS/SSL certificate issuance business, with the final issuance date of 31 October 2025 and all Buypass TLS/SSL certificates expiring no later than 15 April 2026. The roots remain in the Web PKI trust stores until they expire in 2040 or are explicitly distrusted.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| Buypass Class 2 Root CA | `54657681.0` | RSA 4096 | 2040-10-26 | `9A:11:40:25:19:7C:5B:B9:5D:94:E6:3D:55:CD:43:79:08:47:B6:46:B2:3C:DF:11:AD:A4:A0:0E:FF:15:FB:48` |
| Buypass Class 3 Root CA | `e8de2f56.0` | RSA 4096 | 2040-10-26 | `ED:F7:EB:BC:A2:7A:2A:38:4D:38:7B:7D:40:10:C6:66:E2:ED:B4:84:3E:4C:29:B4:AE:1D:5B:93:32:E6:B2:4D` |

Subject for both roots: `C=NO, O=Buypass AS-983163327`. The Class 2 root was used primarily for DV/OV TLS certificates; the Class 3 root for OV, EV (EV Policy OID `2.16.578.1.26.1.3.3`), QWAC, and PSD2 QWAC certificates.

## Rationale for inclusion

Both roots were accepted into the Mozilla NSS root store following two separate public reviews: the original Class 2 CA 1 / Class 3 CA 1 pair via [Bugzilla #477028](https://bugzilla.mozilla.org/show_bug.cgi?id=477028) (approved 2010), and the renewed roots via [Bugzilla #685128](https://bugzilla.mozilla.org/show_bug.cgi?id=685128) (approved 2012, shipped in Firefox 16/19). Both roots are also trusted by Microsoft (Windows CTL), Google (Android CA store), and Apple. The Websites trust bit is enabled for both roots; EV is enabled for the Class 3 root. Buypass is a member of the CA/Browser Forum.

## CA/Browser Forum compliance

Buypass is audited under **ETSI EN 319 411-1** (general certificates, including TLS) and **ETSI EN 319 411-2** (EU Qualified Certificates / eIDAS), which are accepted audit frameworks under the CA/Browser Forum Baseline Requirements. Earlier audits (prior to 2012) were conducted by KPMG under the WebTrust for CAs and WebTrust EV SSL criteria. Buypass additionally holds ISO 27001 (information security), ISO 9001 (quality management), and PCI DSS certifications. It is a Qualified Trust Service Provider (QTSP) supervised by the Norwegian Communications Authority (Nkom). CP/CPS documents are publicly available at [buypass.com/security/ca-documentation-legal](https://www.buypass.com/security/ca-documentation-legal), and a PKI Disclosure Statement is available at [buypass.no/pds/pds_en.pdf](https://www.buypass.no/pds/pds_en.pdf). Buypass discloses audit information and intermediate certificates in the CCADB. Certificate Transparency logging was required for publicly-trusted TLS certificates and was offered via the Buypass Go SSL (ACME) service, launched in 2018.

## Past non-compliance

Several publicly-documented incidents have been filed in Mozilla Bugzilla:

- **[Bug 1595113](https://bugzilla.mozilla.org/show_bug.cgi?id=1595113) (Nov 2019):** Eight intermediate CAs capable of issuing TLS certificates were not listed in Buypass's audit reports. Buypass became aware through a mozilla.dev.security.policy discussion and remediated the disclosure.

- **[Bug 1626078](https://bugzilla.mozilla.org/show_bug.cgi?id=1626078) (Mar 2020):** PSD2 QWACs were missing the NCA identifier in the `cabfOrganizationIdentifier` field, violating the EV Guidelines. Reviewers required revocation; Buypass initially resisted but ultimately revoked the affected certificates.

- **[Bug 1632632](https://bugzilla.mozilla.org/show_bug.cgi?id=1632632) (Apr 2020):** One PSD2 QWAC was mis-issued with an illegal value (`UN`) in the Subject Business Category field.

- **[Bug 1654216](https://bugzilla.mozilla.org/show_bug.cgi?id=1654216) (Jul 2020):** A PSD2 QWAC was issued with an RSA modulus not divisible by 8, violating BR requirements. Buypass had incorrectly assumed the check was covered by linting tools.

- **[Bug 1838421](https://bugzilla.mozilla.org/show_bug.cgi?id=1838421) (Jun 2023):** One TLS certificate was mis-issued using an impermissible domain validation method (the `iodef` property of a CAA record rather than the `contactemail` property). A reviewer noted that use of externally-operated DNS tools constitutes delegation to a third party under BR §3.2.2.4.

- **[Bug 1864204](https://bugzilla.mozilla.org/show_bug.cgi?id=1864204) (Nov 2023):** 591 TLS certificates (479 OV, 101 EV, 11 QWAC) were issued with Subject attributes in incorrect relative order, violating BR §7.1.4.2. Issuance was halted on discovery; the issue was corrected and restarted.

- **[Bug 1865368](https://bugzilla.mozilla.org/show_bug.cgi?id=1865368) (Nov 2023):** Arising from the subject-order mis-issuance above, Buypass failed to revoke 553 of 591 affected certificates within the 5-day BR deadline. Buypass acknowledged the delay was "far beyond requirements," citing potential disruption to Norwegian healthcare and critical infrastructure as justification. This reasoning was strongly rejected by Mozilla reviewers, who noted that a public CA cannot selectively waive revocation requirements based on subscriber criticality, and raised concerns about a "huge gap in knowledge on the Certificate Authority Policy side."

- **[Bug 1872371](https://bugzilla.mozilla.org/show_bug.cgi?id=1872371):** Buypass's ACME service had used external recursive DNS resolvers for domain validation since 2017, constituting reliance on a Delegated Third Party without proper controls under the BRs. Buypass acknowledged the oversight in retrospect.

No distrust action has been taken by any major root program against Buypass's roots as of the time of writing. The root store records can be searched at [ccadb.org](https://www.ccadb.org/) and in [Mozilla Bugzilla CA component](https://bugzilla.mozilla.org/buglist.cgi?product=CA%20Program&component=CA%20Certificate%20Root%20Program&list_id=17131362).

## Transparency

- CP/CPS documents are publicly published at [buypass.com/security/ca-documentation-legal](https://www.buypass.com/security/ca-documentation-legal).
- A PKI Disclosure Statement (PDS) is available in English at [buypass.no/pds/pds_en.pdf](https://www.buypass.no/pds/pds_en.pdf).
- Intermediate certificates and audit reports are disclosed in the CCADB (the 2019 incident in [Bug 1595113](https://bugzilla.mozilla.org/show_bug.cgi?id=1595113) identified and remediated a gap in this disclosure).
- Buypass self-reported incidents in Mozilla Bugzilla and the CA Program component.
- Certificate Transparency was supported for TLS certificates issued under these roots; Buypass operated the Buypass Go SSL ACME service which supported CT logging.

## Sources

- [Buypass CA Documentation (Legal)](https://www.buypass.com/security/ca-documentation-legal)
- [Buypass Certification / Audit page](https://www.buypass.com/the-company/certification)
- [Buypass PKI Disclosure Statement (EN)](https://www.buypass.no/pds/pds_en.pdf)
- [Buypass TLS/SSL discontinuation announcement](https://www.buypass.com/products/tls-ssl-certificates/discontinues-issuance-of-tls-ssl-certificates)
- [Bugzilla #477028 — Add Buypass AS root certificates (first inclusion)](https://bugzilla.mozilla.org/show_bug.cgi?id=477028)
- [Bugzilla #499712 — Add Buypass Class 2 CA 1 and Class 3 CA 1 to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=499712)
- [Bugzilla #685128 — Add Buypass Class 2 Root CA and Class 3 Root CA (renewed roots)](https://bugzilla.mozilla.org/show_bug.cgi?id=685128)
- [mozilla.dev.security.policy — Buypass Root Inclusion Request for Renewed Roots](https://groups.google.com/g/mozilla.dev.security.policy/c/h0aEnEUa-ic)
- [Bugzilla #1595113 — Buypass: Intermediate certificates not listed in audit reports](https://bugzilla.mozilla.org/show_bug.cgi?id=1595113)
- [Bugzilla #1626078 — Buypass: Missing NCA identifier in cabfOrganizationIdentifier in PSD2 QWACs](https://bugzilla.mozilla.org/show_bug.cgi?id=1626078)
- [Bugzilla #1632632 — Buypass: Illegal Business Category in a PSD2 QWAC](https://bugzilla.mozilla.org/show_bug.cgi?id=1632632)
- [Bugzilla #1654216 — Buypass: PSD2 QWAC with RSA modulus not divisible by 8](https://bugzilla.mozilla.org/show_bug.cgi?id=1654216)
- [Bugzilla #1838421 — Buypass: Domain validation method using not allowed domain contact](https://bugzilla.mozilla.org/show_bug.cgi?id=1838421)
- [Bugzilla #1864204 — Buypass: TLS certificates with incorrect Subject attribute order](https://bugzilla.mozilla.org/show_bug.cgi?id=1864204)
- [Bugzilla #1865368 — Buypass: TLS certificates not revoked within 5 days](https://bugzilla.mozilla.org/show_bug.cgi?id=1865368)
- [Bugzilla #1872371 — Buypass: Using an external DNS Resolver for DNS lookups](https://bugzilla.mozilla.org/show_bug.cgi?id=1872371)
- [Microsoft Trusted Root Program — August 2025 Deployment Notice](https://learn.microsoft.com/en-us/security/trusted-root/2025/august-2025)
- [Android CA certificates — Google Git (Buypass roots)](https://android.googlesource.com/platform/system/ca-certificates/+/162afc579a4e05933db8ee63f79cc40a7b62cd49)
- [Buypass — PKI Consortium member page](https://pkic.org/members/buypass/)
- [nixsanctuary.com — Root CA BuyPass Ends TLS/SSL Certificate Issuance](https://nixsanctuary.com/root-ca-buypass-ends-tls-ssl-certificate-issuance-including-free-gossl-via-acme/)
