# UniTrust

UniTrust is the public certification-authority brand of **Shanghai Electronic
Certification Authority Co., Ltd. (SHECA)**, a Shanghai-based commercial CA and
one of the largest CAs in China, operating under China's Electronic Signature
Law. "UCA" (UniTrust Certification Authority) is a registered trademark of SHECA;
the roots in this folder use the UCA brand.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| UCA Global G2 Root | `c01eb047.0` | RSA 4096 | 2040-12-31 | `9BEA11C976FE014764C1BE56A6F914B5A560317ABD9988393382E5161AA0493C` |
| UCA Extended Validation Root | `0f5dc4f3.0` | RSA 4096 | 2038-12-31 | `D43AF9B35473755C9684FC06D7D8CB70EE5C28E773FB294EB41EE71722924D24` |

## Rationale for inclusion

Both roots are owned by SHECA and were added to the Mozilla NSS root store with
the Websites (TLS server authentication) trust bit following a public inclusion
review ([Bugzilla #1496214](https://bugzilla.mozilla.org/show_bug.cgi?id=1496214);
EV enablement in [#1496215](https://bugzilla.mozilla.org/show_bug.cgi?id=1496215)).
The UCA Extended Validation Root carries the CA/Browser Forum EV policy OID
`2.23.140.1.1`. They are distributed in the major root programs (Mozilla, and the
Microsoft program in which SHECA has been present since 2008).

## CA/Browser Forum compliance

SHECA commits to the CA/Browser Forum Baseline Requirements in its
[UniTrust Certification Practice Statement](https://assets-cdn.sheca.com/documents/UniTrust%20Certification%20Practice%20Statement%20v3.8.0.pdf)
and EV CP/CPS, and is disclosed in the CCADB. WebTrust audits have been performed
by PwC (PricewaterhouseCoopers); the inclusion record references WebTrust for CAs
/ BR / EV audit periods beginning 2015 and continuing in subsequent years.

## Past non-compliance

No public **distrust** action against SHECA's UCA roots is known. During the
Mozilla inclusion review several CP/CPS and operational deficiencies were raised
and resolved before approval, including: test-website/revocation status pages
returning incorrect results; CP/CPS wording that did not adequately forbid the CA
generating subscriber key pairs (fixed by an explicit prohibition); and CP/CPS
clauses on email validation, "test" certificates, and renewal-without-revalidation
that were corrected. SHECA also revoked its SHECA Global G3 and Extended
Validation code-signing subordinate CAs during the process. These were
pre-inclusion review findings rather than post-trust incidents; see the inclusion
discussion ([mozilla.dev.security.policy thread](https://groups.google.com/g/mozilla.dev.security.policy/c/2RCO0P-gX0g)).
For any later incidents, see the
[CCADB / Mozilla CA incident records](https://wiki.mozilla.org/CA/Incident_Dashboard).

## Transparency

SHECA publishes its CP/CPS documents (including English-language versions) at
<https://www.sheca.com> / its document CDN, is disclosed with audit history in the
CCADB, and its TLS issuance is logged to Certificate Transparency as required by
the browser root programs.

## Sources

- [Bugzilla #1496214 — Add SHECA UCA Global G2 and UCA EV roots to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=1496214)
- [Bugzilla #1496215 — Enable EV for UCA Extended Validation Root](https://bugzilla.mozilla.org/show_bug.cgi?id=1496215)
- [Bugzilla #1309797 — Add SHECA root certificate(s)](https://bugzilla.mozilla.org/show_bug.cgi?id=1309797)
- [mozilla.dev.security.policy — SHECA inclusion discussion](https://groups.google.com/g/mozilla.dev.security.policy/c/2RCO0P-gX0g)
- [SHECA UniTrust Certification Practice Statement v3.8.0](https://assets-cdn.sheca.com/documents/UniTrust%20Certification%20Practice%20Statement%20v3.8.0.pdf)
- [Mozilla CA Incident Dashboard](https://wiki.mozilla.org/CA/Incident_Dashboard)
