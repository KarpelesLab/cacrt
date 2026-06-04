# SSL.com

SSL Corporation (operating as SSL.com) is a publicly trusted certificate authority headquartered in Houston, Texas, USA, founded in 2002. The company issues TLS/SSL, code signing, S/MIME, and document signing certificates to customers in over 180 countries and participates as a full voting member of the CA/Browser Forum. All roots in this folder are issued by SSL Corporation; no separate legal entities or acquired brands are merged here, though SSL.com acquired the VikingCloud digital certificate customer portfolio in November 2025.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| SSL.com EV Root Certification Authority ECC | `f0c70a8d.0` | ECC P-384 | 2041-02-12 | `22:A2:C1:F7:BD:ED:70:4C:C1:E7:01:B5:F4:08:C3:10:88:0F:E9:56:B5:DE:2A:4A:44:F9:9C:87:3A:25:A7:C8` |
| SSL.com EV Root Certification Authority RSA R2 | `06dc52d5.0` | RSA 4096 | 2042-05-30 | `2E:7B:F1:6C:C2:24:85:A7:BB:E2:AA:86:96:75:07:61:B0:AE:39:BE:3B:2F:E9:D0:CC:6D:4E:F7:34:91:42:5C` |
| SSL.com Root Certification Authority ECC | `0bf05006.0` | ECC P-384 | 2041-02-12 | `34:17:BB:06:CC:60:07:DA:1B:96:1C:92:0B:8A:B4:CE:3F:AD:82:0E:4A:A3:0B:9A:CB:C4:A7:4E:BD:CE:BC:65` |
| SSL.com Root Certification Authority RSA | `6fa5da56.0` | RSA 4096 | 2041-02-12 | `85:66:6A:56:2E:E0:BE:5C:E9:25:C1:D8:89:0A:6F:76:A8:7E:C1:6D:4D:7D:5F:29:EA:74:19:CF:20:12:3B:69` |
| SSL.com TLS ECC Root CA 2022 | `865fbdf9.0` | ECC P-384 | 2046-08-19 | `C3:2F:FD:9F:46:F9:36:D1:6C:36:73:99:09:59:43:4B:9A:D6:0A:AF:BB:9E:7C:F3:36:54:F1:44:CC:1B:A1:43` |
| SSL.com TLS RSA Root CA 2022 | `a89d74c2.0` | RSA 4096 | 2046-08-19 | `8F:AF:7D:2E:2C:B4:70:9B:B8:E0:B3:36:66:BF:75:A5:DD:45:B5:DE:48:0F:8E:A8:D4:BF:E6:BE:BC:17:F2:ED` |

## Rationale for inclusion

SSL.com's roots are included in all four major root programs:

- **Mozilla NSS / Firefox**: The four 2016-era roots were included following a public CCADB discussion. The 2022 TLS roots (RSA and ECC) were added in NSS 3.92 / Firefox 117 (EV enabled in Firefox 118) following Mozilla bug 1799533. [[1]](https://bugzilla.mozilla.org/show_bug.cgi?id=1799533)
- **Chrome Root Store**: SSL.com TLS RSA Root CA 2022 and SSL.com TLS ECC Root CA 2022 were approved for Server Authentication use, effective approximately March 19, 2024. [[2]](https://groups.google.com/a/ccadb.org/g/public/c/5yTz3rg-SrY)
- **Apple and Microsoft**: SSL.com roots appear in the Apple and Microsoft root stores; Apple's program requires CCADB disclosure and annual audits. [[3]](https://www.apple.com/certificateauthority/ca_program.html)

All roots in this folder carry the TLS server authentication extended key usage or were issued with EV treatment; they are scoped to the public Web PKI.

## CA/Browser Forum compliance

SSL.com undergoes annual WebTrust audits conducted by BDO (an AICPA/CPA Canada accredited auditor), covering: WebTrust for CAs, Baseline Requirements for TLS, Extended Validation SSL, Code Signing, S/MIME, Network Security, and Verified Mark Certificates. [[4]](https://www.ssl.com/web-trust/)

SSL.com has been a full voting member of the CA/Browser Forum since August 3, 2017 (associate member from October 2016). Its CP/CPS (current version publicly available at `https://legal.ssl.com/documents/SSLcom-CP-CPS.pdf`, versioned archive from v1.0 through v1.30 at `https://www.ssl.com/repository/`) covers all certificate types and is publicly disclosed in the CCADB. [[5]](https://www.ssl.com/repository/) [[6]](https://legal.ssl.com/documents/SSLcom-CP-CPS.pdf)

SSL.com supports ACME (RFC 8555) for automated DV certificate issuance and renewal. [[7]](https://www.ssl.com/guide/ssl-tls-certificate-issuance-and-revocation-with-acme/) All publicly-trusted TLS certificates are submitted to Certificate Transparency logs per Baseline Requirements Section 2.2.

## Past non-compliance

**2025 — DCV mis-issuance (BR 3.2.2.4.14): Mozilla bug 1961406** [[8]](https://bugzilla.mozilla.org/show_bug.cgi?id=1961406)

In April 2025 a security researcher reported a flaw in SSL.com's implementation of DCV method 3.2.2.4.14 ("Email to DNS TXT Contact"). SSL.com's system incorrectly treated the domain portion of the approver's contact email address as the validated domain, rather than the domain in the certificate request. A code path introduced on 2024-02-12 propagated the email domain into the validation record, and a switch from method 3.2.2.4.2 to 3.2.2.4.14 on 2024-12-02 brought the bug into active use. The researcher demonstrated the flaw by obtaining a certificate for a domain belonging to Alibaba Cloud (aliyun.com). SSL.com disabled the affected DCV method within approximately two hours of the report, revoked 11 mis-issued certificates (including ones for *.medinet.ca, help.gurusoft.com.sg, and banners.betvictor.com), and deployed a patch on 2025-05-09. The bug was marked RESOLVED FIXED. No evidence of malicious exploitation was reported. [[9]](https://securityonline.info/ssl-com-discloses-mis-issuance-of-digital-certificates-due-to-dcv-flaw/) [[10]](https://www.theregister.com/2025/04/22/ssl_com_validation_flaw/)

**2024 — Certificate problem-reporting usability: Mozilla bug 1942270** [[11]](https://bugzilla.mozilla.org/show_bug.cgi?id=1942270)

A researcher found SSL.com's revocation request form difficult to use (ambiguous "Certificate Thumbprint" field) when attempting to report a compromised key. SSL.com confirmed the certificate was revoked within the required timeline, attributed the confusion to internal miscommunication, and improved the form to accept multiple fingerprint formats. The bug was RESOLVED INVALID; no BR violation was confirmed.

No distrust actions by any root program have been publicly documented against SSL.com as of the date this file was last updated. Searches of Mozilla's Bugzilla and CCADB reveal no additional formal mis-issuance or revocation-delay incidents beyond those described above. [[12]](https://bugzilla.mozilla.org/buglist.cgi?quicksearch=ssl.com+mis-issuance)

## Transparency

- **CP/CPS**: Publicly available at `https://legal.ssl.com/documents/SSLcom-CP-CPS.pdf`; full version history at `https://www.ssl.com/repository/`. [[5]](https://www.ssl.com/repository/)
- **CCADB disclosure**: SSL.com discloses all intermediate CAs and audit reports through the CCADB, as required by Mozilla, Chrome, and Apple root programs.
- **Incident self-reporting**: The 2025 DCV mis-issuance was disclosed via a CCADB/Bugzilla incident report and detailed post-incident commitments, including SDLC improvements and expanded test coverage. [[8]](https://bugzilla.mozilla.org/show_bug.cgi?id=1961406)
- **Certificate Transparency**: SSL.com embeds Signed Certificate Timestamps (SCTs) in all publicly-trusted TLS certificates per the CA/Browser Forum Baseline Requirements.
- **Annual audits**: WebTrust audit reports are published at `https://www.ssl.com/web-trust/`. [[4]](https://www.ssl.com/web-trust/)

## Sources

- [1] [Mozilla Bugzilla 1799533 — Add SSL.com 2022 TLS Root CA Certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=1799533)
- [2] [CCADB Public — Chrome Root Store Inclusion Application Decision for SSL.com](https://groups.google.com/a/ccadb.org/g/public/c/5yTz3rg-SrY)
- [3] [Apple Root Certificate Program](https://www.apple.com/certificateauthority/ca_program.html)
- [4] [SSL.com — Current WebTrust Audit Reports](https://www.ssl.com/web-trust/)
- [5] [SSL.com — Repository (CP/CPS archive)](https://www.ssl.com/repository/)
- [6] [SSL.com CP/CPS (current)](https://legal.ssl.com/documents/SSLcom-CP-CPS.pdf)
- [7] [SSL.com — SSL/TLS Certificate Issuance and Revocation with ACME](https://www.ssl.com/guide/ssl-tls-certificate-issuance-and-revocation-with-acme/)
- [8] [Mozilla Bugzilla 1961406 — SSL.com: DCV bypass and issue fake certificates for any MX hostname](https://bugzilla.mozilla.org/show_bug.cgi?id=1961406)
- [9] [SecurityOnline — SSL.com Discloses Mis-issuance of Digital Certificates Due to DCV Flaw](https://securityonline.info/ssl-com-discloses-mis-issuance-of-digital-certificates-due-to-dcv-flaw/)
- [10] [The Register — Bug hunter obtains an SSL cert for Alibaba Cloud in 5 steps](https://www.theregister.com/2025/04/22/ssl_com_validation_flaw/)
- [11] [Mozilla Bugzilla 1942270 — SSL.com: Revocation process requires submission to a form that is unusable](https://bugzilla.mozilla.org/show_bug.cgi?id=1942270)
- [12] [Mozilla Bugzilla — SSL.com incident search](https://bugzilla.mozilla.org/buglist.cgi?quicksearch=ssl.com+mis-issuance)
- [13] [SSL.com — About](https://www.ssl.com/about/)
- [14] [SSL.com — ACME protocol support](https://www.ssl.com/products/website-security/acme/)
- [15] [Mozilla Bugzilla 1799703 — Add SSL.com 2022 Client Root CA Certificates](https://bugzilla.mozilla.org/show_bug.cgi?id=1799703)
