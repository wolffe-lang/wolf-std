# vendor/vectors/pki/sc79 — the public-PKI test corpus (RSA, RSA-PSS, P-384)

Authored by `gen_sc79.sh` with an OpenSSL binary used as a black-box
certificate builder: OpenSSL 3.0.13 (30 Jan 2024), Ubuntu 24.04's build,
inside the Linux x86-64 container image the lane's gauntlet runs in. The
binary and its two libraries are recorded by sha256 in `openssl.txt`. No
OpenSSL source was read. Keys are random, so re-running the script makes
a different PKI of the same shapes; the committed files are the corpus,
as the RFC texts are. `sha256sums.txt` holds every file's digest.

Every certificate is valid from 2026-11-05 for 3650 days (to
2036-11-02). The rows judge it at a supplied time, never the clock.
`verify.txt` is OpenSSL's own verdict on each chain
(`openssl verify -purpose sslserver -x509_strict`), so a builder mistake
and a `std.x.tls.cert` mistake cannot agree by construction.

The private keys of the five leaves the local TLS servers present are
committed (`*_leaf.key`): they are test keys for loopback rows and sign
nothing else. No other private key is kept.

## The chains

| chain | root | intermediate | leaf | signatures |
|---|---|---|---|---|
| RSA | `rsa_root` RSA-4096 | `rsa_int` RSA-3072 | `rsa_leaf` RSA-2048 | sha384WithRSA, sha256WithRSA |
| RSA-PSS | `rsa_root` | `pss_int` RSA-2048 | `pss_leaf` RSA-2048 | RSASSA-PSS SHA-256 salt 32, SHA-384 salt 48 |
| P-384 | `p384_root` P-384 | `p384_int` P-384 | `p384_leaf` P-384 | ecdsa-with-SHA384 |
| mixed EC | `p384_root` | `p384_int` | `p256_leaf` P-256 | ecdsa-with-SHA256 by a P-384 key |
| mixed EC | `p384_root` | `p384_int` | `ec512_leaf` P-384 | ecdsa-with-SHA512 |
| Ed25519 | `ed_root` | none | `ed_leaf` | Ed25519 |

Leaves carry basicConstraints cA FALSE, keyUsage digitalSignature and
keyEncipherment, extendedKeyUsage serverAuth, and subjectAltName
`DNS:localhost`, `DNS:*.wolf.test`, `IP:127.0.0.1`. Intermediates carry
pathLenConstraint 0 and extendedKeyUsage serverAuth, clientAuth.

## The negatives (each refused by the name in the second column)

| file | refusal | why |
|---|---|---|
| `rsa1024_leaf` under `rsa_int` | `weak_key` | the leaf's modulus is 1024 bits |
| `weakiss_leaf` under `rsa1024_int` | `weak_key` | the issuer's modulus is 1024 bits |
| `client_leaf` under `rsa_int` | `bad_usage` | extendedKeyUsage clientAuth only |
| `kunosig_leaf` under `rsa_int` | `bad_usage` | keyUsage without digitalSignature (RFC 8446 §4.4.2.2; OpenSSL's sslserver purpose accepts it, for TLS 1.2's RSA key transport) |
| `email_leaf` under `email_int` | `bad_usage` | the intermediate's extendedKeyUsage is emailProtection only |
| `nc_leaf` under `nc_int` | `name_constraints` | the intermediate carries nameConstraints (OpenSSL accepts the chain; the refusal is this library's named limit) |
| `nonca_leaf` under `nonca_int` | `not_ca` | the intermediate's basicConstraints says cA FALSE |
| `pl0_leaf` under `pl0_int` under `pl0_root` | `not_ca` | the root's pathLenConstraint is 0 |
| `sha1_leaf` under `rsa_int` | `unsupported_alg` | sha1WithRSAEncryption |
| `p521_root` | `unsupported_alg` | a P-521 key and ecdsa-with-SHA512 |
| `ed448_root` | `unsupported_alg` | Ed448 |
| `wild_leaf` under `rsa_int` | the SAN rows | `DNS:*.wild.wolf.test` and `DNS:exact.wolf.test` |

`rsa_int_x` has `rsa_int`'s subject and key, issued by `other_root`: the
chain `rsa_leaf, rsa_int_x, rsa_int` trusted to `rsa_root` alone needs
path building to step past the cross-sign.

## The signature fixtures

- `rsa2049.pub.der`: a 2049-bit RSAPublicKey, so RSASSA-PSS's encoded
  message is one octet shorter than the modulus (emBits = 2048). OpenSSL
  3.0 rounds an odd key size down, so the script draws this key's primes
  itself (Miller-Rabin over Python's integers) and hands OpenSSL the
  RSAPrivateKey to sign with;
  `rsa2049_pss_sha256.sig` (salt 32) and `rsa2049_pkcs1_sha256.sig` sign
  the 16 bytes `sc79 row message`.
- `cv_rsa_pss_sha{256,384,512}.sig` (by `rsa_leaf.key`, salt = digest
  length, MGF1 over the same hash) and `cv_p384_sha384.sig` (by
  `p384_leaf.key`, DER) sign RFC 8446 §4.4.3's content: 64 spaces,
  `TLS 1.3, server CertificateVerify`, 0x00, then the 32 bytes 0x00..0x1f
  standing in for the transcript hash.
