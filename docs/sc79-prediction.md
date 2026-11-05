# sc79 prediction: the public-PKI TLS client

Written before any vector, row or change under `std/`. Base: trunk `3111d39`. Toolchain unchanged: wolf 0.2.36, lupin 0.1.58.

The work: `std.x.crypto.rsa` (RSASSA-PKCS1-v1_5 and RSASSA-PSS verification, RFC 8017), `std.x.crypto.p384` (ECDSA P-384 verification, FIPS 186-5, SEC 1 v2, SP 800-186), X.509 path validation against a caller-supplied root set in `std.x.tls.cert` (RFC 5280, RFC 9525), and the TLS 1.3 wiring in `std.x.tls.client` (RFC 8446 §4.2.3, §4.4.2, §4.4.3). wolf-std#101 is the issue.

## P1. Wycheproof RSA: which cases accept and which reject

Vendored from Wycheproof `testvectors_v1`, the files whose hash is SHA-256, SHA-384 or SHA-512 and whose modulus is 2048 to 4096 bits:

- RSASSA-PKCS1-v1_5: `rsa_signature_{2048,3072,4096}_{sha256,sha384,sha512}_test.json`, nine files, 2330 cases (68 `valid`, 9 `acceptable`, 2253 `invalid`).
- RSASSA-PSS with MGF1 over the same hash: `rsa_pss_2048_sha256_mgf1_0`, `rsa_pss_2048_sha256_mgf1_32`, `rsa_pss_2048_sha384_mgf1_48`, `rsa_pss_3072_sha256_mgf1_32`, `rsa_pss_4096_sha256_mgf1_32`, `rsa_pss_4096_sha384_mgf1_48`, `rsa_pss_4096_sha512_mgf1_32`, `rsa_pss_4096_sha512_mgf1_64`, eight files, 1065 cases (704 `valid`, 361 `invalid`).

Not vendored, named: SHA-1, SHA-224, SHA-512/224, SHA-512/256, SHA-3 and SHAKE files (no such hash in the ladder); MGF1 under a different hash than the message hash (no TLS 1.3 scheme uses it); the `_params` files (they differ only in carrying RSASSA-PSS parameters in the key, which this module does not take); `rsa_pss_misc`; the 8192-bit files (refused by name, below); OAEP and PKCS#1 encryption files.

The predicted verdicts, by rule:

- Every `valid` case verifies, and every `invalid` case does not. The invalid classes are the files' own: BER-encoded and modified padding, invalid ASN.1 in the DigestInfo, wrong hash, short padding, no hash, signatures out of range, the wrong primitive.
- The nine `acceptable` cases are each file's one `MissingNull` case (a DigestInfo without the NULL parameters). **They are rejected.** Verification builds the expected encoding per RFC 8017 §8.2.2 steps 3 and 4 and compares it with the recovered one, and §9.2 Note 1's DigestInfo carries the NULL.
- The cases flagged `SmallPublicKey` use e = 3. **They verify**, as `valid`. The exponent rule is RFC 8017 §3.1's: e odd, 3 ≤ e < n, with e < 2^256 as the upper bound (FIPS 186-5's limit, and the CA/Browser Forum's).
- So exactly the nine `MissingNull` tcIds differ from a "Wycheproof result = accept unless invalid" reading; the independent checker lists them by file and tcId.

## P2. Wycheproof ECDSA P-384

Vendored: `ecdsa_secp384r1_sha384_test.json` (504: 194 `valid`, 310 `invalid`), `ecdsa_secp384r1_sha384_p1363_test.json` (280: 193, 87), `ecdsa_secp384r1_sha256_test.json` (472: 162, 310) and `ecdsa_secp384r1_sha512_test.json` (542: 231, 311). 1798 cases (780 `valid`, 1018 `invalid`), none `acceptable`.

Every `valid` case verifies and every `invalid` case does not: strict DER (X.690) for the DER files, r and s in [1, n), the public key on the curve (SEC 1 §3.2.2), the hash truncated to the leftmost 384 bits for SHA-512 and used whole for SHA-256 (FIPS 186-5 §6.4.2's bits2int). No case changes verdict.

## P3. The local handshakes, against an OpenSSL binary as the peer

The peer and the certificate builder are one pinned OpenSSL binary, recorded by version and digest. The rows run on Linux x86-64.

| server | certificates | CertificateVerify | trunk | after |
|---|---|---|---|---|
| RSA | RSA-2048 leaf under an RSA root, sha256WithRSAEncryption | `rsa_pss_rsae_sha256` | `complete=unsupported_alg` | complete, application data read |
| RSA, SHA-384 and SHA-512 schemes | the same chain, the server held to one scheme | `rsa_pss_rsae_sha384`, `_sha512` | `complete=unsupported_alg` | complete |
| RSA-PSS certificates | an RSA-4096 root signing an intermediate with RSASSA-PSS, a leaf under it | `rsa_pss_rsae_sha256` | `complete=unsupported_alg` | complete |
| ECDSA P-384 chain | P-384 root, P-384 intermediate, P-384 leaf, ecdsa-with-SHA384 | `ecdsa_secp384r1_sha384` | `complete=unsupported_alg` | complete |
| Ed25519 and P-256 | today's profiles | `ed25519`, `ecdsa_secp256r1_sha256` | complete | complete (unchanged) |

## P4. github.com:443

From Linux, with the distribution's root bundle converted to concatenated DER and supplied explicitly, one outbound connection per run. Trunk: `complete=unsupported_alg` (as wolf-std#101 records). After: the handshake completes. Expected chain: a P-256 leaf, an intermediate signed with ecdsa-with-SHA384 by a P-384 root that is in the bundle; if the server sends its RSA chain instead, RSA-2048 under RSA-4096, which also completes. The row records the algorithms it saw.

## P5. The cost (ratios, evidence only)

Native, the rig's default build, on one Linux x86-64 host under `taskset`. A calibration run of `p256.verify` measured about 16.6 ms per call.

| verify | predicted ratio to `p256.verify` |
|---|---|
| RSA-2048, PKCS#1 v1.5, SHA-256, e = 65537 | 0.1–0.4 |
| RSA-4096, PKCS#1 v1.5, SHA-256, e = 65537 | 0.3–1.2 |
| P-384, SHA-384 | 1–4 |

The reasoning: Montgomery multiplication over 24-bit limbs costs about 2k² limb steps for k limbs, RSA with e = 65537 is 19 of them, and a P-384 verify is about 8000 field multiplications over 16 limbs.

## P6. What moves in the existing rows

- `tests/x/tls/cert/unsupported_alg_row.lu`: its RSA certificate validates now, so the row is rewritten to name an algorithm that is still refused.
- `tests/x/tls/client/rfc8448_parse_vectors.lu`: RFC 8448's CertificateVerify is under a 1024-bit RSA key; the answer moves from `unsupported_alg` to the refusal of a small modulus by name.
- Every other existing chain, client and crypto row keeps its verdict and its ledger word.

## P7. The plant

One hunk: RSASSA-PSS verification with the salt length off by one. The PSS rows that carry a real salt go red by name; the PKCS#1 v1.5 rows stay green.
