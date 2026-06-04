# NetLock

NetLock Kft. (full legal name: NETLOCK Informatikai és Hálózatbiztonsági Szolgáltató Kft.) is a Hungarian certificate authority founded in 1997 and headquartered in Budapest, recognized as Hungary's first qualified trust service provider. It issues TLS/SSL, qualified electronic signature, and timestamp certificates under Hungarian and EU eIDAS regulation, supervised by the National Media and Infocommunications Authority (NMHH). Since 2013 it has been a member of the Docler Holding group.

## Roots in this folder

| Certificate | OpenSSL hash | Key | Valid until | SHA-256 fingerprint |
|---|---|---|---|---|
| NetLock Arany (Class Gold) Főtanúsítvány | `988a38cb.0` | RSA 2048 | 2028-12-06 | `6C:61:DA:C3:A2:DE:F0:31:50:6B:E0:36:D2:A6:FE:40:19:94:FB:D1:3D:F9:C8:D4:66:59:92:74:C4:46:EC:98` |

## Rationale for inclusion

The NetLock Arany (Class Gold) Főtanúsítvány root was added to Mozilla NSS in January 2010 (shipped in NSS 3.12.6 / Firefox 3.6.2) with trust flags for websites, email, and code signing. [[Bug 532201]](https://bugzilla.mozilla.org/show_bug.cgi?id=532201) Mozilla subsequently approved Extended Validation (EV) treatment for this root. [[Bug 1579454]](https://bugzilla.mozilla.org/show_bug.cgi?id=1579454)

The root is currently included in the Mozilla, Microsoft, and Apple root stores and issues TLS server authentication certificates under the CA/Browser Forum Baseline Requirements. Note that Google Chrome announced removal of default trust for new certificates issued after July 31, 2025 (see [Past non-compliance](#past-non-compliance) below). [[Google Blog, May 2025]](https://security.googleblog.com/2025/05/sustaining-digital-certificate-security-chrome-root-store-changes.html)

## CA/Browser Forum compliance

NetLock is audited under the **ETSI EN 319 411** framework (covering ETSI EN 319 401, EN 319 411, and EN 319 412) by **MÁTRIX Ltd.** (Auditing, Evaluating and Certification Ltd.), a conformity assessment body. The most recent publicly available audit attestation letter (I-NL23T5\_TAN-AAL-01, issued August 25, 2023, covering the period September 1, 2022 – August 22, 2023) reported no major or minor non-conformities. [[Audit attestation letter]](https://netlock.hu/app/uploads/dokumentumok/COMPLIANCE/Tanusitasok/2023/I-NL23T5_TAN-AAL-01_v1.0_signed-1.pdf) The company also maintains ISO 9001:2015 and ISO 27001:2022 certifications and performs WebTrust for CA methodology-based reviews. [[NetLock About]](https://netlock.hu/en/about-us/)

NetLock publishes CP and CPS documents at [netlock.hu/aktualis-szabalyzatok](https://netlock.hu/aktualis-szabalyzatok/#english) in Hungarian (official) and English (translation). Certificates are logged to public Certificate Transparency logs per BR requirements.

## Past non-compliance

NetLock has a substantial and publicly documented history of compliance issues in Mozilla's Bugzilla. The following incidents are all cited from public Bugzilla records:

- **2017 – Non-BR-compliant certificate issuance** [[Bug 1391056]](https://bugzilla.mozilla.org/show_bug.cgi?id=1391056): TLS certificates issued that did not conform to the Baseline Requirements.

- **2019 – Non-compliant intermediate issuance** [[Bug 1586795]](https://bugzilla.mozilla.org/show_bug.cgi?id=1586795): Intermediate CA certificates issued after 2019-01-01 were found to have missing or inconsistent Extended Key Usage extensions contrary to Mozilla Policy 2.6.1. Misissuance was blocked November 18, 2019; affected intermediates were replaced.

- **2020 – Failure to revoke non-compliant ICAs within 7 days** [[Bug 1656882]](https://bugzilla.mozilla.org/show_bug.cgi?id=1656882): Following discovery of the Bug 1586795 intermediates, NetLock failed to revoke them within the BR-required 7-day window; revocations were completed May–June 2020.

- **2021 – Intermediate CA certificate missing from audit reports** [[Bug 1716874]](https://bugzilla.mozilla.org/show_bug.cgi?id=1716874): An intermediate CA was not included in the relevant audit reports.

- **2022–2023 – SSL certificates with OU field, revocation delay** [[Bug 1822809]](https://bugzilla.mozilla.org/show_bug.cgi?id=1822809): Certificates issued with prohibited OU fields were not revoked within the BR 5-day window, with NetLock citing customer requests as justification for the delay.

- **2024 – Policy qualifier misissuance and delayed revocation** [[Bug 1891331]](https://bugzilla.mozilla.org/show_bug.cgi?id=1891331) / [[Bug 1947691]](https://bugzilla.mozilla.org/show_bug.cgi?id=1947691): TLS certificates issued with User Notice policy qualifiers (prohibited since BR 2.0 effective September 15, 2023) across three intermediates. Required revocation within 5 days was not met for some certificates, again citing customer requests.

- **2024 – Intermediate CA certificates not disclosed to CCADB** [[Bug 1904041]](https://bugzilla.mozilla.org/show_bug.cgi?id=1904041): Intermediate CA certificates issued in May 2024 were not disclosed to the CCADB within the required 7-day window due to an administrative error. The Chrome Root Program characterized NetLock's incident report handling as "not acceptable" and noted a "concerning pattern" relative to Chrome Root Program Policy factors.

- **2024 – Failure to respond to Certificate Problem Report within 24 hours** [[Bug 1905509]](https://bugzilla.mozilla.org/show_bug.cgi?id=1905509): NetLock did not respond to a CPR within the BR-required 24-hour window, attributed to misaligned contact information between CCADB and the CP/S.

**Google Chrome distrust (effective August 1, 2025):** On May 30, 2025, Google announced that Chrome 139+ would no longer trust TLS certificates chaining to the NetLock Arany root whose earliest SCT is dated after July 31, 2025 11:59:59 PM UTC, citing "a pattern of compliance failures, unmet improvement commitments, and the absence of tangible, measurable progress in response to publicly disclosed incident reports." [[Google Online Security Blog]](https://security.googleblog.com/2025/05/sustaining-digital-certificate-security-chrome-root-store-changes.html) [[BleepingComputer coverage]](https://www.bleepingcomputer.com/news/security/google-chrome-to-distrust-chunghwa-telecom-netlock-certificates-in-august/)

No publicly documented distrust by Mozilla or Microsoft has been found as of the date of this writing. A search of the Mozilla Bugzilla CA-incident tracker and CCADB is recommended for the most current status: [[Bugzilla NetLock bugs]](https://bugzilla.mozilla.org/buglist.cgi?query_format=specific&order=relevance+desc&bug_status=__all__&product=&content=netlock).

## Transparency

- **CP/CPS:** Published at [netlock.hu/aktualis-szabalyzatok](https://netlock.hu/aktualis-szabalyzatok/#english). Documents are authoritative in Hungarian; English translations are provided. Versions covering qualified (QC), non-qualified (C), and EVGL certificate services are maintained separately.
- **CCADB disclosure:** NetLock is listed in the CCADB. Incidents noted above (Bug 1904041, Bug 1891331) document instances where CCADB entries for certificate policies were found to be out of date and intermediate certificates were not timely disclosed.
- **Audit attestation:** Published on the NetLock website; latest available is the 2023 ETSI attestation by MÁTRIX Ltd. [[Link]](https://netlock.hu/app/uploads/dokumentumok/COMPLIANCE/Tanusitasok/2023/I-NL23T5_TAN-AAL-01_v1.0_signed-1.pdf)
- **Certificate Transparency:** Certificates issued under this root are logged to CT logs per BR requirements. CT log inclusion was referenced in the Chrome distrust action (SCT cutoff date used as the enforcement boundary).
- **Incident self-reporting:** Mozilla Bugzilla records indicate several instances where incidents were reported externally before NetLock self-disclosed, and periods of non-responsiveness to required weekly incident updates (Bug 1905509, Bug 1904041).

## Sources

- [Mozilla Bugzilla Bug 532201 – Add NetLock Arany root to NSS](https://bugzilla.mozilla.org/show_bug.cgi?id=532201)
- [Mozilla Bugzilla Bug 1579454 – Enable EV Treatment for NetLock Arany](https://bugzilla.mozilla.org/show_bug.cgi?id=1579454)
- [Mozilla Bugzilla Bug 1391056 – NetLock: Non-BR-Compliant Certificate Issuance](https://bugzilla.mozilla.org/show_bug.cgi?id=1391056)
- [Mozilla Bugzilla Bug 1586795 – NetLock: Issuance of intermediates after 2019-01-01 that do not comply with Mozilla Policy](https://bugzilla.mozilla.org/show_bug.cgi?id=1586795)
- [Mozilla Bugzilla Bug 1656882 – NetLock: Failure to revoke noncompliant ICA within 7 days](https://bugzilla.mozilla.org/show_bug.cgi?id=1656882)
- [Mozilla Bugzilla Bug 1716874 – NetLock: Intermediate CA Certificate Missing from Audit Reports](https://bugzilla.mozilla.org/show_bug.cgi?id=1716874)
- [Mozilla Bugzilla Bug 1822809 – NETLOCK: SSL certificates with OU field – revocation delay](https://bugzilla.mozilla.org/show_bug.cgi?id=1822809)
- [Mozilla Bugzilla Bug 1891331 – NETLOCK: Policy Qualifiers other than id-qt-cps is included in TLS certificates – delayed revocation](https://bugzilla.mozilla.org/show_bug.cgi?id=1891331)
- [Mozilla Bugzilla Bug 1947691 – NETLOCK: Bug 1891331 replacement – delayed revocation](https://bugzilla.mozilla.org/show_bug.cgi?id=1947691)
- [Mozilla Bugzilla Bug 1904041 – NETLOCK: Intermediate CA Certificate not disclosed to CCADB](https://bugzilla.mozilla.org/show_bug.cgi?id=1904041)
- [Mozilla Bugzilla Bug 1905509 – NETLOCK: CPR was not responded to in 24 hours](https://bugzilla.mozilla.org/show_bug.cgi?id=1905509)
- [Mozilla Bugzilla Bug 1794583 – Add NETLOCK ECC Root (withdrawn)](https://bugzilla.mozilla.org/show_bug.cgi?id=1794583)
- [Google Online Security Blog – Sustaining Digital Certificate Security: Upcoming Changes to the Chrome Root Store (May 30, 2025)](https://security.googleblog.com/2025/05/sustaining-digital-certificate-security-chrome-root-store-changes.html)
- [BleepingComputer – Google Chrome to distrust Chunghwa Telecom, Netlock certificates in August](https://www.bleepingcomputer.com/news/security/google-chrome-to-distrust-chunghwa-telecom-netlock-certificates-in-august/)
- [NetLock About Us](https://netlock.hu/en/about-us/)
- [NetLock Certificate Issuers / CRL page](https://www.netlock.hu/USEREN/html/cacrl.html)
- [NetLock Audit Attestation Letter 2023 (ETSI, MÁTRIX Ltd.)](https://netlock.hu/app/uploads/dokumentumok/COMPLIANCE/Tanusitasok/2023/I-NL23T5_TAN-AAL-01_v1.0_signed-1.pdf)
- [NetLock CP/CPS documents (current)](https://netlock.hu/aktualis-szabalyzatok/#english)
