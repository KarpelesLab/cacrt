# Microsoft

Microsoft Corporation (headquartered in Redmond, Washington, USA) operates its own publicly-trusted Certificate Authority under the brand "Microsoft PKI Services," separate from the Microsoft Trusted Root Program (which governs third-party CAs trusted in Windows). The two roots in this folder are the non-EV TLS issuance roots of that first-party CA hierarchy, used to secure Microsoft-owned domains including Azure service endpoints. All certificates issued under these roots are restricted to internal Microsoft subscribers and Microsoft-owned domains.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| Microsoft ECC Root Certificate Authority 2017 | `8d89cda1.0` | ECC P-384 | 2042-07-18 | `35:8D:F3:9D:76:4A:F9:E1:B7:66:E9:C9:72:DF:35:2E:E1:5C:FA:C2:27:AF:6A:D1:D7:0E:8E:4A:6E:DC:BA:02` |
| Microsoft RSA Root Certificate Authority 2017 | `bf53fb88.0` | RSA 4096 | 2042-07-18 | `C7:41:F7:0F:4B:2A:8D:88:BF:2E:71:C1:41:22:EF:53:EF:10:EB:A0:CF:A5:E6:4C:FA:20:F4:18:85:30:73:E0` |

Both certificates are self-signed, issued 2019-12-18, and carry the Websites (TLS server authentication) trust bit only. The matching EV roots (Microsoft EV RSA Root Certificate Authority 2017 and Microsoft EV ECC Root Certificate Authority 2017) were submitted to Mozilla but never completed inclusion; they are not in this folder.

## Rationale for inclusion

Both roots are included in all four major root programs:

- **Mozilla NSS / Firefox**: Added in NSS 3.54 / Firefox 79 (2020), tracked in [Bugzilla #1641716][bz1641716] following the public discussion in [Bugzilla #1448093][bz1448093]. Trust flags: Websites only.
- **Microsoft Trusted Root Program**: Included as first-party roots; the Microsoft Trusted Root Program governs both third-party and Microsoft's own roots distributed with Windows.
- **Apple Trust Store**: Both roots appear in Apple's trusted certificates list for macOS/iOS ([Apple Support HT213464][apple-ts]).
- **Chrome Root Store**: Both roots appear as Trusted Roots in the [Chrome Root Store][chrome-rs] (not constrained).

Issuance scope is explicitly restricted: per the CCADB case record and CPS, Microsoft PKI Services issues TLS certificates only to internal Microsoft subscribers for Microsoft-owned domains, with no externally operated subordinate CAs or Registration Authorities. [[CCADB Case #00000275][ccadb-case]]

## CA/Browser Forum compliance

Microsoft PKI Services operates under the following audit and policy framework:

- **Audit standard**: WebTrust for Certification Authorities, WebTrust for CAs — SSL Baseline with Network Security (WebTrust BR). Annual audit period runs 1 May to 30 April. The current auditor is Deloitte (Canada); audit statements for the 2024-05-01 to 2025-04-30 period are dated 2025-07-16. [[PKI Repository][pki-repo]]
- **Baseline Requirements**: The Public TLS CPS explicitly states conformance with the CA/Browser Forum Baseline Requirements. Annual CP/CPS review is required and performed. [[Public TLS CPS][tls-cps]]
- **CCADB disclosure**: The roots and their associated intermediates are disclosed in CCADB. Audit reports are published in the repository at `https://www.microsoft.com/pkiops/docs/`. [[CCADB Case #00000275][ccadb-case]]
- **Certificate Transparency**: TLS certificates issued under these roots embed SCTs per RFC 6962, as required by the CA/Browser Forum and browser CT policies. Microsoft's AD CS infrastructure supports the precertificate flow for CT log submission.
- **CP/CPS framework**: Documents follow RFC 3647 structure. The Public TLS CPS governs public issuance; a separate Corporate CPS governs internal non-public issuance. The current CP is v3.2.0; the current Public TLS CPS is v3.3.6. [[PKI Repository][pki-repo]]

## Past non-compliance

Several publicly-documented incidents have been filed on Mozilla Bugzilla against Microsoft PKI Services. All are self-reported or community-reported; no distrust action has been taken against these roots.

- **[Bug 1586847][bz1586847]** (2019): Ten intermediate CAs created 2019-05-14 and 2019-06-11 lacked the Client/Server Authentication EKUs required by Mozilla policy. Reported during the root inclusion review.
- **[Bug 1598390][bz1598390]** (2019): Null character introduced into intermediate certificates due to a tooling gap in Microsoft's offline certificate issuance suite.
- **[Bug 1644936][bz1644936]** (2020): Certificates mis-issued with a missing Locality field. Mozilla noted that more than a month elapsed between detection and reporting, characterising the delay as a violation of incident reporting guidance.
- **[Bug 1674561][bz1674561]** (2021): DV certificates were issued with OV policy OIDs due to an incorrect template selection in a rarely-used workflow introduced in a September 2020 change.
- **[Bug 1705419][bz1705419]** (2021): Certificates issued with underscore characters in SANs due to incomplete linting logic.
- **[Bug 1706860][bz1706860]** (2021): Three certificates contained a DNSName that was not a valid FQDN (trailing hyphen in a label), discovered during internal review following the underscore incident.
- **[Bug 1711147][bz1711147]** (2021): Eight intermediate CAs created 2021-03-11 were mis-issued without certificate policy extensions due to a syntax error in the ceremony scripts; post-issuance checks also failed to catch the omission.
- **[Bug 1962829][bz1962829] / [Bug 1962830][bz1962830]** (2025): A typographical error introduced in CPS 3.2.4 (2024-07-21) incorrectly stated that `keyEncipherment` was absent from RSA subscriber certificates. A related configuration change removed OCSP URIs from four issuing CAs' certificates without updating the CPS first.
- **[Bug 1965612][bz1965612]** (2025): Because the CPS typo constituted a misissuance under BR 4.9.1.1, approximately 75 million unexpired certificates technically required revocation within 5 days. Microsoft self-reported the delay and proposed staged batch revocation to avoid CRLs exceeding 600 MB; community criticism focused on the scale, the slow response timeline, and the lack of urgency. Microsoft committed to CRL partitioning, shorter certificate lifetimes, and formalising a process to monitor other CAs' Bugzilla incidents. The bug is resolved fixed.

No distrust action or formal root removal has been initiated by any root program against these roots as of June 2026. For a current search of open incidents, see the [Bugzilla CA:Microsoft query][bz-search].

## Transparency

- **CP/CPS**: Publicly available at the [Microsoft PKI Services repository][pki-repo]. The repository also contains current CRLs, audit reports, subscriber and relying party agreements, and test pages for valid, revoked, and expired certificates.
- **CCADB**: Microsoft discloses root and intermediate CA metadata, audit periods, and auditor information in CCADB. [[CCADB Case #00000275][ccadb-case]]
- **Incident reporting**: Incidents are self-reported to Mozilla Bugzilla (dev-security-policy component). The 2025 incidents prompted Microsoft to formalise a recurring process for monitoring and responding to community-filed bugs. [[Bug 1962829][bz1962829]]
- **Certificate Transparency**: All publicly-trusted TLS certificates are logged to CT logs; SCTs are embedded in issued certificates. CT compliance is audited as part of the annual WebTrust BR examination.
- **Problem reporting contact**: `certificateauthority@microsoft.com` (per CP/CPS).

## Sources

- [Microsoft PKI Services Repository (CP, CPS, audit reports)][pki-repo]
- [Microsoft PKI Services Public TLS CPS v3.3.4][tls-cps]
- [CCADB Case #00000275 — Microsoft Root Inclusion][ccadb-case]
- [Bugzilla #1448093 — Add 2 Microsoft Roots to Mozilla's Root Store][bz1448093]
- [Bugzilla #1641716 — Add Microsoft's non-EV root certificates to NSS][bz1641716]
- [Bugzilla #1582254 — Add Microsoft root certificates to NSS (earlier attempt)][bz1582254]
- [Bugzilla #1586847 — Intermediates issued after 2019-01-01 without required EKUs][bz1586847]
- [Bugzilla #1598390 — Null character bug in intermediate certificates][bz1598390]
- [Bugzilla #1644936 — Certificate mis-issuance: Locality missing][bz1644936]
- [Bugzilla #1674561 — DV certificate issued with OV fields][bz1674561]
- [Bugzilla #1705419 — Underscore in SAN][bz1705419]
- [Bugzilla #1706860 — DNSName is not FQDN (trailing hyphen)][bz1706860]
- [Bugzilla #1711147 — Malformed ICAs missing certificate policy extensions][bz1711147]
- [Bugzilla #1962829 — Policy document bug (CPS typographical error)][bz1962829]
- [Bugzilla #1962830 — Subscriber certificate change not compliant with CPS][bz1962830]
- [Bugzilla #1965612 — Failure to revoke in 5 days][bz1965612]
- [Mozilla dev-security-policy: Request to include 4 Microsoft Root CAs][mdsp]
- [Apple Trust Store — iOS 16 / macOS 13 trusted root certificates][apple-ts]
- [Chrome Root Store root_store.md][chrome-rs]
- [Bugzilla CA:Microsoft open incident search][bz-search]

[pki-repo]: https://www.microsoft.com/pkiops/docs/repository.htm
[tls-cps]: https://www.microsoft.com/pkiops/Docs/Content/policy/Microsoft_PKI_Services_public_tls_CPS_v3.3.4.pdf
[ccadb-case]: https://ccadb.my.salesforce-sites.com/mozilla/PrintViewForCase?CaseNumber=00000275
[bz1448093]: https://bugzilla.mozilla.org/show_bug.cgi?id=1448093
[bz1641716]: https://bugzilla.mozilla.org/show_bug.cgi?id=1641716
[bz1582254]: https://bugzilla.mozilla.org/show_bug.cgi?id=1582254
[bz1586847]: https://bugzilla.mozilla.org/show_bug.cgi?id=1586847
[bz1598390]: https://bugzilla.mozilla.org/show_bug.cgi?id=1598390
[bz1644936]: https://bugzilla.mozilla.org/show_bug.cgi?id=1644936
[bz1674561]: https://bugzilla.mozilla.org/show_bug.cgi?id=1674561
[bz1705419]: https://bugzilla.mozilla.org/show_bug.cgi?id=1705419
[bz1706860]: https://bugzilla.mozilla.org/show_bug.cgi?id=1706860
[bz1711147]: https://bugzilla.mozilla.org/show_bug.cgi?id=1711147
[bz1962829]: https://bugzilla.mozilla.org/show_bug.cgi?id=1962829
[bz1962830]: https://bugzilla.mozilla.org/show_bug.cgi?id=1962830
[bz1965612]: https://bugzilla.mozilla.org/show_bug.cgi?id=1965612
[mdsp]: https://groups.google.com/g/mozilla.dev.security.policy/c/Q2k_5eGXqmA
[apple-ts]: https://support.apple.com/en-us/103100
[chrome-rs]: https://chromium.googlesource.com/chromium/src/+/main/net/data/ssl/chrome_root_store/root_store.md
[bz-search]: https://bugzilla.mozilla.org/buglist.cgi?product=CA%20Program&component=CA%20Certificate%20Compliance&short_desc=Microsoft+PKI&short_desc_type=allwordssubstr
