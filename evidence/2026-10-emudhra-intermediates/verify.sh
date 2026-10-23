#!/bin/sh
# Verify the archived eMudhra evidence with OpenSSL only:
#   - each private key matches the public key of both certificates that carry it;
#   - each key produces a signature that verifies against that public key;
#   - each certificate's signature verifies against its archived issuer.
# Run from anywhere: sh verify.sh
set -eu
cd "$(dirname "$0")"
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
echo "cacrt evidence check" > "$tmp/msg"
status=0
for c in certs/*.crt.pem; do
  b=$(basename "$c")
  k="keys/${b%%.*}.key.pem"
  case "$b" in
    *Sectigo_AAA*) i=issuers/AAA_Certificate_Services.crt.pem ;;
    *_CS_CA_*)     i=issuers/emSign_Root_CS_CA_G3.crt.pem ;;
    *)             i=issuers/emSign_Root_TLS_CA_G3.crt.pem ;;
  esac
  openssl ec -in "$k" -pubout 2>/dev/null > "$tmp/pub"
  openssl x509 -in "$c" -noout -pubkey > "$tmp/certpub"
  if cmp -s "$tmp/pub" "$tmp/certpub" &&
     openssl dgst -sha384 -sign "$k" -out "$tmp/sig" "$tmp/msg" &&
     openssl dgst -sha384 -verify "$tmp/pub" -signature "$tmp/sig" "$tmp/msg" >/dev/null &&
     openssl verify -partial_chain -no_check_time -purpose any -CAfile "$i" "$c" >/dev/null 2>&1
  then
    echo "OK    $b"
  else
    echo "FAIL  $b"
    status=1
  fi
done
exit $status
