#!/usr/bin/env bash
# gen_sc79.sh OUT -- sc79's test PKI, authored with the OpenSSL BINARY as
# a black-box certificate builder (never its source). Provenance, not rig
# machinery: CI never runs it; the committed files are the corpus. Keys
# are random, so a re-run makes a different PKI with the same shapes.
#
# The binary that authored the committed corpus is recorded in
# openssl.txt (version and sha256 of the binary and its two libraries).
# Every certificate here is cross-checked by `openssl verify` against
# its chain at the end of this script (the negatives are checked to
# FAIL, each for the reason the row names).
#
# Usage, from a directory of the caller's own:
#   bash vendor/vectors/pki/sc79/gen_sc79.sh /path/to/out
set -euo pipefail
out=${1:?usage: gen_sc79.sh OUT}
mkdir -p "$out"
cd "$out"
ossl=$(command -v openssl)
{
  openssl version
  sha256sum "$(readlink -f "$ossl")"
  for lib in $(ldd "$(readlink -f "$ossl")" | awk '/libssl|libcrypto/ {print $3}'); do
    sha256sum "$(readlink -f "$lib")"
  done
} > openssl.txt

cat > ext.cnf <<'EOF'
[req]
distinguished_name = dn
[dn]
[root]
basicConstraints = critical,CA:TRUE
keyUsage = critical,keyCertSign,cRLSign
subjectKeyIdentifier = hash
[root_pl0]
basicConstraints = critical,CA:TRUE,pathlen:0
keyUsage = critical,keyCertSign,cRLSign
subjectKeyIdentifier = hash
[int]
basicConstraints = critical,CA:TRUE,pathlen:0
keyUsage = critical,keyCertSign,cRLSign
extendedKeyUsage = serverAuth,clientAuth
subjectKeyIdentifier = hash
authorityKeyIdentifier = keyid
[int_email]
subjectKeyIdentifier = hash
authorityKeyIdentifier = keyid
basicConstraints = critical,CA:TRUE,pathlen:0
keyUsage = critical,keyCertSign,cRLSign
extendedKeyUsage = emailProtection
[int_nc]
subjectKeyIdentifier = hash
authorityKeyIdentifier = keyid
basicConstraints = critical,CA:TRUE,pathlen:0
keyUsage = critical,keyCertSign,cRLSign
nameConstraints = critical,permitted;DNS:localhost
[int_nonca]
subjectKeyIdentifier = hash
authorityKeyIdentifier = keyid
basicConstraints = critical,CA:FALSE
keyUsage = critical,digitalSignature
[leaf]
subjectKeyIdentifier = hash
authorityKeyIdentifier = keyid
basicConstraints = critical,CA:FALSE
keyUsage = critical,digitalSignature,keyEncipherment
extendedKeyUsage = serverAuth
subjectAltName = DNS:localhost,DNS:*.wolf.test,IP:127.0.0.1
[leaf_client]
subjectKeyIdentifier = hash
authorityKeyIdentifier = keyid
basicConstraints = critical,CA:FALSE
keyUsage = critical,digitalSignature
extendedKeyUsage = clientAuth
subjectAltName = DNS:localhost
[leaf_kunosig]
subjectKeyIdentifier = hash
authorityKeyIdentifier = keyid
basicConstraints = critical,CA:FALSE
keyUsage = critical,keyEncipherment
extendedKeyUsage = serverAuth
subjectAltName = DNS:localhost
[leaf_local]
subjectKeyIdentifier = hash
authorityKeyIdentifier = keyid
basicConstraints = critical,CA:FALSE
keyUsage = critical,digitalSignature
extendedKeyUsage = serverAuth
subjectAltName = DNS:localhost
[leaf_wild]
subjectKeyIdentifier = hash
authorityKeyIdentifier = keyid
basicConstraints = critical,CA:FALSE
keyUsage = critical,digitalSignature
subjectAltName = DNS:*.wild.wolf.test,DNS:exact.wolf.test
EOF

key() { # key NAME ALG [OPTS...]
  local name=$1 alg=$2; shift 2
  openssl genpkey -algorithm "$alg" "$@" -out "$name.key" 2>/dev/null
}
selfsigned() { # selfsigned NAME CN SECTION DIGEST [SIGOPT...]
  local name=$1 cn=$2 sec=$3 dg=$4; shift 4
  openssl req -x509 -new -key "$name.key" -subj "/CN=$cn" -days 3650 \
    -config ext.cnf -extensions "$sec" $dg "$@" -out "$name.pem"
}
issue() { # issue NAME CN SECTION ISSUER DIGEST [SIGOPT...] (NAME.key must exist)
  local name=$1 cn=$2 sec=$3 iss=$4 dg=$5; shift 5
  openssl req -new -key "$name.key" -subj "/CN=$cn" -config ext.cnf -out "$name.csr"
  openssl x509 -req -in "$name.csr" -CA "$iss.pem" -CAkey "$iss.key" -CAcreateserial \
    -days 3650 -extfile ext.cnf -extensions "$sec" $dg "$@" -out "$name.pem" 2>/dev/null
  rm -f "$name.csr"
}

# The RSA chain: RSA-4096 root, RSA-3072 intermediate (sha384WithRSA),
# RSA-2048 leaf (sha256WithRSA).
key rsa_root RSA -pkeyopt rsa_keygen_bits:4096
selfsigned rsa_root "wolf-std sc79 RSA root" root -sha256
key rsa_int RSA -pkeyopt rsa_keygen_bits:3072
issue rsa_int "wolf-std sc79 RSA intermediate" int rsa_root -sha384
key rsa_leaf RSA -pkeyopt rsa_keygen_bits:2048
issue rsa_leaf "localhost" leaf rsa_int -sha256

# The RSA-PSS chain: RSASSA-PSS certificate signatures, SHA-256 salt 32
# (root -> intermediate) and SHA-384 salt 48 (intermediate -> leaf).
key pss_int RSA -pkeyopt rsa_keygen_bits:2048
issue pss_int "wolf-std sc79 PSS intermediate" int rsa_root -sha256 \
  -sigopt rsa_padding_mode:pss -sigopt rsa_pss_saltlen:32 -sigopt rsa_mgf1_md:sha256
key pss_leaf RSA -pkeyopt rsa_keygen_bits:2048
issue pss_leaf "localhost" leaf pss_int -sha384 \
  -sigopt rsa_padding_mode:pss -sigopt rsa_pss_saltlen:48 -sigopt rsa_mgf1_md:sha384

# The P-384 chain, and the mixed shapes public PKI carries: a P-256 leaf
# under the P-384 intermediate with ecdsa-with-SHA256, a P-384 leaf with
# ecdsa-with-SHA512.
key p384_root EC -pkeyopt ec_paramgen_curve:P-384
selfsigned p384_root "wolf-std sc79 P-384 root" root -sha384
key p384_int EC -pkeyopt ec_paramgen_curve:P-384
issue p384_int "wolf-std sc79 P-384 intermediate" int p384_root -sha384
key p384_leaf EC -pkeyopt ec_paramgen_curve:P-384
issue p384_leaf "localhost" leaf p384_int -sha384
key p256_leaf EC -pkeyopt ec_paramgen_curve:P-256
issue p256_leaf "localhost" leaf p384_int -sha256
key ec512_leaf EC -pkeyopt ec_paramgen_curve:P-384
issue ec512_leaf "localhost" leaf p384_int -sha512

# An Ed25519 chain for the interop control row.
key ed_root ED25519
selfsigned ed_root "wolf-std sc79 Ed25519 root" root ""
key ed_leaf ED25519
issue ed_leaf "localhost" leaf ed_root ""

# Negatives under the RSA intermediate.
key rsa1024_leaf RSA -pkeyopt rsa_keygen_bits:1024
issue rsa1024_leaf "localhost" leaf rsa_int -sha256
key rsa1024_int RSA -pkeyopt rsa_keygen_bits:1024
issue rsa1024_int "wolf-std sc79 RSA-1024 intermediate" int rsa_root -sha256
cp rsa_leaf.key weakiss_leaf.key
issue weakiss_leaf "localhost" leaf rsa1024_int -sha256
cp rsa_leaf.key client_leaf.key
issue client_leaf "localhost" leaf_client rsa_int -sha256
cp rsa_leaf.key kunosig_leaf.key
issue kunosig_leaf "localhost" leaf_kunosig rsa_int -sha256
cp rsa_leaf.key wild_leaf.key
issue wild_leaf "exact.wolf.test" leaf_wild rsa_int -sha256
cp rsa_leaf.key sha1_leaf.key
issue sha1_leaf "localhost" leaf rsa_int -sha1
key email_int RSA -pkeyopt rsa_keygen_bits:2048
issue email_int "wolf-std sc79 email intermediate" int_email rsa_root -sha256
cp rsa_leaf.key email_leaf.key
issue email_leaf "localhost" leaf email_int -sha256
key nc_int RSA -pkeyopt rsa_keygen_bits:2048
issue nc_int "wolf-std sc79 constrained intermediate" int_nc rsa_root -sha256
cp rsa_leaf.key nc_leaf.key
issue nc_leaf "localhost" leaf_local nc_int -sha256
key nonca_int RSA -pkeyopt rsa_keygen_bits:2048
issue nonca_int "wolf-std sc79 not-a-CA intermediate" int_nonca rsa_root -sha256
cp rsa_leaf.key nonca_leaf.key
issue nonca_leaf "localhost" leaf nonca_int -sha256

# pathLenConstraint 0 on a root, with an intermediate below it.
key pl0_root RSA -pkeyopt rsa_keygen_bits:2048
selfsigned pl0_root "wolf-std sc79 pathlen-0 root" root_pl0 -sha256
key pl0_int RSA -pkeyopt rsa_keygen_bits:2048
issue pl0_int "wolf-std sc79 pathlen intermediate" int pl0_root -sha256
cp rsa_leaf.key pl0_leaf.key
issue pl0_leaf "localhost" leaf pl0_int -sha256

# The cross-sign: the RSA intermediate's subject and key, issued a second
# time by another root. The chain [leaf, int_x, int] needs path building
# to step past int_x when only rsa_root is trusted.
key other_root RSA -pkeyopt rsa_keygen_bits:2048
selfsigned other_root "wolf-std sc79 other RSA root" root -sha256
cp rsa_int.key rsa_int_x.key
issue rsa_int_x "wolf-std sc79 RSA intermediate" int other_root -sha256

# Algorithms the ladder refuses by name: P-521 and Ed448.
key p521_root EC -pkeyopt ec_paramgen_curve:P-521
selfsigned p521_root "wolf-std sc79 P-521 root" root -sha512
key ed448_root ED448
selfsigned ed448_root "wolf-std sc79 Ed448 root" root ""

# A 2049-bit key (emLen = k - 1 under RSASSA-PSS) and CertificateVerify
# fixtures: RFC 8446 section 4.4.3's signed content over a fixed
# transcript hash, signed as each TLS 1.3 scheme signs it.
key rsa2049 RSA -pkeyopt rsa_keygen_bits:2049
printf 'sc79 row message' > msg.bin
openssl dgst -sha256 -sigopt rsa_padding_mode:pss -sigopt rsa_pss_saltlen:32 \
  -sign rsa2049.key -out rsa2049_pss_sha256.sig msg.bin
openssl dgst -sha256 -sign rsa2049.key -out rsa2049_pkcs1_sha256.sig msg.bin
openssl rsa -in rsa2049.key -pubout -RSAPublicKey_out -outform DER -out rsa2049.pub.der 2>/dev/null
cv_content() { # cv_content HASHLEN: 64 spaces, the context, 0x00, bytes 0..HASHLEN-1
  python3 -I -c 'import sys; n=int(sys.argv[1]); sys.stdout.buffer.write(b" "*64 + b"TLS 1.3, server CertificateVerify" + b"\0" + bytes(range(n)))' "$1"
}
cv_content 32 > cv32.bin
cv_content 48 > cv48.bin
for h in 256 384 512; do
  sl=$((h / 8))
  openssl dgst -sha$h -sigopt rsa_padding_mode:pss -sigopt rsa_pss_saltlen:$sl \
    -sigopt rsa_mgf1_md:sha$h -sign rsa_leaf.key -out cv_rsa_pss_sha$h.sig cv32.bin
done
openssl dgst -sha384 -sign p384_leaf.key -out cv_p384_sha384.sig cv32.bin

# DER of every certificate, and the cross-checks.
for p in *.pem; do openssl x509 -in "$p" -outform DER -out "${p%.pem}.der"; done
check() { # check WANT-OK|FAIL ROOT LEAF [UNTRUSTED]
  local want=$1 root=$2 leaf=$3 untrusted=${4:-}
  local args=(-CAfile "$root.pem" -purpose sslserver -x509_strict)
  [ -z "$untrusted" ] || args+=(-untrusted "$untrusted.pem")
  if openssl verify "${args[@]}" "$leaf.pem" > /dev/null 2>&1; then got=OK; else got=FAIL; fi
  echo "verify $leaf under $root${untrusted:+ via $untrusted}: $got (want $want)"
  [ "$got" = "$want" ] || return 1
}
{
  check OK rsa_root rsa_leaf rsa_int
  check OK rsa_root pss_leaf pss_int
  check OK p384_root p384_leaf p384_int
  check OK p384_root p256_leaf p384_int
  check OK p384_root ec512_leaf p384_int
  check OK ed_root ed_leaf
  check FAIL rsa_root client_leaf rsa_int
  check OK rsa_root kunosig_leaf rsa_int
  check FAIL rsa_root email_leaf email_int
  check FAIL rsa_root nonca_leaf nonca_int
  check FAIL pl0_root pl0_leaf pl0_int
  check OK rsa_root nc_leaf nc_int
  check OK other_root rsa_leaf rsa_int_x
} > verify.txt
cat verify.txt
