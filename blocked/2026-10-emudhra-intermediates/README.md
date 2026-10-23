# eMudhra intermediate key compromise (October 2026)

The 8 eMudhra intermediate CA keys below have trivially small P-384 private
keys (d ≤ 255) and are public. Each key was certified twice: once by its emSign
G3 root and once by Sectigo's AAA Certificate Services, so all 16 certificates
are here. Blocking by key (SPKI SHA-256) also catches any copy not listed.

| Intermediate | SPKI SHA-256 |
|---|---|
| EM OV TLS CA - G3C | `63ECEFEE431D49523BC1136C4E72D0206A477086CAF606CD5CF025686C109CF8` |
| EM EV TLS CA - G3C | `5C0660A2B164910D8FF589C18E14552E9E0EC237DCEFE2CB736D41A4811F81A0` |
| EM OV CS CA - G3B | `EC4FFA1AD0AC88081314B7AAFFEF6A91F97CD57291BB5BA686C5415F5EB4F6B6` |
| EM EV TLS CA - G3B | `E0254C7FCDC265E2F0F8A428DABB75AB4AE039B4872D62F5EE57F0877D5A8A41` |
| EM OV TLS CA - G3B | `43BFFA19D6EF651451F6841E019931E96F6F2EAEFF28AB4B6F1CE4576DE2717B` |
| EM DV TLS CA - G3B | `EBED6475C2C2D80FCB199F3FE8DD6E6B3CE60B77EB15D4E42C3DEC0C6F5B5105` |
| EM DV TLS CA - G3C | `49DD353DC2B919A3770645117D0E60C16115E7D1BC259EEFE734B00EA434FE5F` |
| EM OV CS CA - G3C | `92C063D67E8F14DB71D288E11266FFDAF69CBF115F7BAB14F3BC2F68D25E0E30` |

Source: <https://emudhra.we-have-a-problem.com/impacted-intermediates.json>.
The private keys and a verification script are archived (outside the crate) in
[`evidence/2026-10-emudhra-intermediates/`](../../evidence/2026-10-emudhra-intermediates/).
Rationale: [`CHANGELOG-roots.md`](../../CHANGELOG-roots.md), 2026-10-21.
