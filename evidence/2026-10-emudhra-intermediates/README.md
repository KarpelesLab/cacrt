# Evidence: eMudhra intermediate CA key compromise (October 2026)

> **These private keys are public on purpose. Never trust anything signed by
> them.** All 16 certificates have been revoked for `cACompromise`. Firefox,
> Chrome, Apple and Microsoft block them. This directory is an archive and is
> not part of the root store: `build.rs` reads only `roots/`, and `/evidence`
> is excluded from the published crate.

This directory keeps the evidence behind the decision to disable
**emSign ECC Root CA - G3** (see [`CHANGELOG-roots.md`](../../CHANGELOG-roots.md)
and [`roots/eMudhra/README.md`](../../roots/eMudhra/README.md)). Anyone can
check it with OpenSSL alone.

## What is here

| Path | Contents |
|---|---|
| `certs/` | All 16 CA-signed intermediate certificates: each of the 8 keys was certified once by its eMudhra root and once by Sectigo's AAA Certificate Services. |
| `keys/` | The 8 private keys (SEC1 `EC PRIVATE KEY`, P-384), one per intermediate. Both certificates of a pair share a key. |
| `issuers/` | The issuing roots: emSign Root TLS CA - G3, emSign Root CS CA - G3 and AAA Certificate Services, plus emSign ECC Root CA - G3, which `roots.json` lists as a cross-signer in the chain. That root is the only one of these in Mozilla's store and was in `cacrt`. |
| `source/` | `impacted-intermediates.json` and `roots.json` as published at <https://emudhra.we-have-a-problem.com/>, fetched 2026-10-21 and again on 2026-10-24 (identical). Every PEM here was extracted from these files and its SHA-256 checked against the published fingerprint. |
| `verify.sh` | Re-checks everything below. |

## The keys

Each private scalar `d` is a tiny integer, at most 255. A properly generated
P-384 key is a uniformly random value in `[1, n−1]` with `n ≈ 2^384`. The
chance of a random key being ≤ 255 is about 2^-376. All eight keys fit in one
byte, which points to a key generator that used about 8 bits of entropy.

| Intermediate | Subject O | Use | d | SPKI SHA-256 |
|---|---|---|---|---|
| EM OV TLS CA - G3C | eMudhra Inc | tls | 4 | `63ECEFEE431D49523BC1136C4E72D0206A477086CAF606CD5CF025686C109CF8` |
| EM EV TLS CA - G3C | eMudhra Inc | tls | 56 | `5C0660A2B164910D8FF589C18E14552E9E0EC237DCEFE2CB736D41A4811F81A0` |
| EM OV CS CA - G3B | eMudhra BV | code-signing | 66 | `EC4FFA1AD0AC88081314B7AAFFEF6A91F97CD57291BB5BA686C5415F5EB4F6B6` |
| EM EV TLS CA - G3B | eMudhra BV | tls | 115 | `E0254C7FCDC265E2F0F8A428DABB75AB4AE039B4872D62F5EE57F0877D5A8A41` |
| EM OV TLS CA - G3B | eMudhra BV | tls | 155 | `43BFFA19D6EF651451F6841E019931E96F6F2EAEFF28AB4B6F1CE4576DE2717B` |
| EM DV TLS CA - G3B | eMudhra BV | tls | 156 | `EBED6475C2C2D80FCB199F3FE8DD6E6B3CE60B77EB15D4E42C3DEC0C6F5B5105` |
| EM DV TLS CA - G3C | eMudhra Inc | tls | 230 | `49DD353DC2B919A3770645117D0E60C16115E7D1BC259EEFE734B00EA434FE5F` |
| EM OV CS CA - G3C | eMudhra Inc | code-signing | 237 | `92C063D67E8F14DB71D288E11266FFDAF69CBF115F7BAB14F3BC2F68D25E0E30` |

## The certificates

| Certificate file | Issuer | Serial | Cert SHA-256 | Revoked (CRL) |
|---|---|---|---|---|
| `certs/EM_OV_TLS_CA_G3C.emSign.crt.pem` | emSign Root TLS CA - G3 | `296926C227A3A1526392B19482A504` | `CB6E15165AC802E9B3F993F16C25D41F4E81C21EBDE71AE1B012AC00BF9C7C3F` | 2026-10-18 |
| `certs/EM_OV_TLS_CA_G3C.Sectigo_AAA.crt.pem` | Sectigo AAA Certificate Services | `8D5A4811939EF447D77111CDD9849AE9` | `7485B51C0ED192878FA1D2F948FB529AE1842CC5086C4B67F7BE5B2039DE9489` | 2026-10-19 |
| `certs/EM_EV_TLS_CA_G3C.emSign.crt.pem` | emSign Root TLS CA - G3 | `8009928A64107601AF05FA2F19F109` | `432EC25F0D28DE9006D97BF752F17BBB2CD41F669933FCB072D9A1148D0261F3` | 2026-10-18 |
| `certs/EM_EV_TLS_CA_G3C.Sectigo_AAA.crt.pem` | Sectigo AAA Certificate Services | `05F078752DECE7A1BB8595D303167171` | `FD9D5C2DBBD4995AB69B8D50C439D9527B00529199C2FD01B2174EB58F07021F` | 2026-10-19 |
| `certs/EM_OV_CS_CA_G3B.emSign.crt.pem` | emSign Root CS CA - G3 | `7818933F8F85B4619A2B35FBA83D0A` | `9E39DE2881273ACC225A149C72BA3E0F51424963669AB633EE1DC7DF5DB067B4` | 2026-10-18 |
| `certs/EM_OV_CS_CA_G3B.Sectigo_AAA.crt.pem` | Sectigo AAA Certificate Services | `41E32B73AC185D6CE0D7480BACF84E69` | `A496E23AC0CAB8155FC9B117B97EAE368EFCBB9041AEC8804D5A308D2D01C2D8` | 2026-10-19 |
| `certs/EM_EV_TLS_CA_G3B.emSign.crt.pem` | emSign Root TLS CA - G3 | `C7C41EFE92D825F05EAE841A4A82C1` | `E7568FE6E12331F4FD7DE61DA4C7559D0F8EB192A218D4741A3BA75184333838` | 2026-10-18 |
| `certs/EM_EV_TLS_CA_G3B.Sectigo_AAA.crt.pem` | Sectigo AAA Certificate Services | `D8F8685E4F2BD9D82402A85A1C1EB731` | `579480AEA166290BD3C643589E795FC0D0E54E86505D180F1DFF99EFD3199512` | 2026-10-19 |
| `certs/EM_OV_TLS_CA_G3B.emSign.crt.pem` | emSign Root TLS CA - G3 | `6CADA367FB1C100088120EA202D568` | `98B896625C0C230ADEEC3EB2CF6AAD5001ECF9156A68644E82374CE5833BC25B` | 2026-10-18 |
| `certs/EM_OV_TLS_CA_G3B.Sectigo_AAA.crt.pem` | Sectigo AAA Certificate Services | `E94223BDE92F914E5AB9ACABFDAC3710` | `5F1DED4DD590AF74631290CF073B122A2AAC6A1FA479778A0BD43579BD832699` | 2026-10-19 |
| `certs/EM_DV_TLS_CA_G3B.emSign.crt.pem` | emSign Root TLS CA - G3 | `A4AEE678F23362B4C0E460B772F214` | `42C4ACCD12C4F59597925E1A08A7A90102608543809308D50EC427E118BA1493` | 2026-10-18 |
| `certs/EM_DV_TLS_CA_G3B.Sectigo_AAA.crt.pem` | Sectigo AAA Certificate Services | `A024FDF4BA665C785CE4A020483858B7` | `45A883F349AEDFD5F3A649B5498FFB121712BA8B448F3F2A91DBC5A766FCDDBE` | 2026-10-19 |
| `certs/EM_DV_TLS_CA_G3C.emSign.crt.pem` | emSign Root TLS CA - G3 | `588B2E3C2BB8CDA13E1A1209254321` | `0164D7CEAE5BB909C272AF151FF44B1177AE9B47D9424B94F1D9558C30934D19` | 2026-10-18 |
| `certs/EM_DV_TLS_CA_G3C.Sectigo_AAA.crt.pem` | Sectigo AAA Certificate Services | `91F347F1A5B7D39962B95E0EE4E299D1` | `62B7F9868263F72F6911FC78CEAF72C1979158B7368BF648683BB2AA39B02E72` | 2026-10-19 |
| `certs/EM_OV_CS_CA_G3C.emSign.crt.pem` | emSign Root CS CA - G3 | `544B7F922C31AE220A4A922916F6E6` | `4B2460A466F95FD40113BCDCCA2CEB29B3B093921DE45474DA7DF48874C630B1` | 2026-10-18 |
| `certs/EM_OV_CS_CA_G3C.Sectigo_AAA.crt.pem` | Sectigo AAA Certificate Services | `D6A5EF613B390F32EF9CDF349F772841` | `84112972B8E59D14560AE7E7119654474AAD119990CF3722BFC1E3B91615E1FD` | 2026-10-19 |

The emSign-issued certificates are valid until 2040-04-28 and the Sectigo
cross-signs until 2028-12-31, so the revocation is the only thing that stops them
from working.

## How to verify

```sh
sh evidence/2026-10-emudhra-intermediates/verify.sh
```

For each certificate the script checks that:

1. the key's public point equals the certificate's public key;
2. the key signs a message that verifies under that public key;
3. the certificate's signature verifies against its archived issuer (expiry
   is ignored and revocation is not checked).

To check the scalar by hand:

```sh
openssl ec -in keys/EM_OV_TLS_CA_G3C.key.pem -noout -text   # priv: 47 zero bytes, then 04
```

## Sources

- <https://emudhra.we-have-a-problem.com/>
- <https://blog.mozilla.org/security/2026/10/20/protecting-firefox-users-following-the-compromise-of-emudhra-intermediate-certificates/>
- <https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/0R3XLJL6FuE/m/0XofdebrAAAJ>
