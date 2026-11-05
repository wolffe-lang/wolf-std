# sc79 — wolf-std: the public-PKI TLS client (RSA, ECDSA P-384, path validation)

Footprint: module

`std.x.tls.client` completes a TLS 1.3 handshake with github.com:443 when its caller supplies the distribution's root bundle as DER (wolf-std#101). Three pieces make it work. `std.x.crypto.rsa` verifies RSASSA-PKCS1-v1_5 and RSASSA-PSS signatures (RFC 8017). `std.x.crypto.p384` verifies ECDSA P-384 signatures (FIPS 186-5, SEC 1 v2, SP 800-186). `std.x.tls.cert.validate_path` builds a path to a caller-supplied root set and validates it. The client offers and verifies six schemes: `ed25519`, `ecdsa_secp256r1_sha256`, `ecdsa_secp384r1_sha384`, and `rsa_pss_rsae_sha256`, `_sha384` and `_sha512`.

## Forbidden

No cryptographic library's source was read: no OpenSSL, BoringSSL, LibreSSL, rustls, ring, Go, Bouncy Castle, mbedTLS, wolfSSL, NSS or other. No libc source was read either. The sources were the standards: RFC 8017, FIPS 186-5, SEC 1 v2, SP 800-186, RFC 5280, RFC 4055, RFC 5480, RFC 5758, RFC 9525, RFC 8446 (vendored at `vendor/vectors/rfc/rfc8446.txt`), and the Wycheproof vectors. The two Python checkers were written from those standards alone. In each one, `hashlib` is the only call with cryptographic meaning, and it serves as the SHA-2 oracle.

OpenSSL took part only as a binary: a certificate builder and a black-box TLS peer. It was OpenSSL 3.0.13 from a Linux x86-64 container image (Ubuntu 24.04). `vendor/vectors/pki/sc79/openssl.txt` pins its version and the digests of its binary and two libraries, and the interop rows check that digest before they run. The one external connection was github.com:443, from the row that needs it and from one sparse `git` clone of the Wycheproof repository (R-sc79-1). The lane connected to no other external host.

Nothing changed in the toolchain pins (wolf 0.2.36, lupin 0.1.58), in CI, or in any consumer repository. No label, merge, tag or close was made. The lane filed four follow-on issues: #126, #127, #128, #129.

## Inputs verified

- **Base:** wolf-std trunk `3111d39`, branch `sc79`.
- **Toolchain:** the release archives match the release API's digests, `d383c74c…` (wolf 0.2.36, Linux x86-64) and `34f52792…` (lupin 0.1.58). The binaries are `f7b73fba…` (wolf), `47c646a6…` (lupin) and `47fb9ad0…` (`libwolf_rt.a`), the same digests sc77 recorded.
- **Wycheproof:** `testvectors_v1` at C2SP/wycheproof `d17cc89f`. All 21 files are vendored with sha256 values in `vendor/vectors/README.md`.
- **Today's refusal:** den's vc00 probe logs `complete=unsupported_alg` for an RSA server and for github.com. At trunk, this lane's interop rows reproduce it for every RSA and P-384 profile (below).
- **Read first:** `API-CONVENTIONS.md` §1–§4, §10–§13 and the later amendments; `std/x/README.md`'s crypto and tls rows; the headers and public surfaces of `tls/{cert,client,handshake,record}`, `crypto/{p256,sha2,chacha20,curve25519}` and `jose`; sc77's note and gauntlet.

## Prediction

The prediction was committed first, at `ba4c946` (`docs/sc79-prediction.md`), before any vector, row or `std/` change.

- **P1, Wycheproof RSA:** held. All 3395 cases came out as predicted: 772 accepted, 2623 rejected. Exactly the nine `MissingNull` acceptables (tcId 8 in each PKCS#1 file) were rejected, and the five e = 3 valid cases were accepted. The checker computed every verdict independently (`tests/x/crypto/rsa/rsa_check.txt`).
- **P2, Wycheproof P-384:** held. 1798 cases: 780 accepted, 1018 rejected, with no verdict changed.
- **P3, local handshakes:** the green half held, and the red half only in part. Every profile completes at the head. At trunk, the RSA, RSA-PSS-certificate and P-256-under-P-384 rows fail as predicted, with `complete=unsupported_alg`. The `rsa_pss_rsae_sha384`, `rsa_pss_rsae_sha512` and P-384-chain rows also fail at trunk, but with `complete=handshake_failure`, not the predicted `unsupported_alg`. Trunk did not offer those schemes, so the OpenSSL peer refused during the handshake, before any certificate reached the client.
- **P4, github.com:** half held. The handshake completes, and the client reads application data. I predicted the row would record the chain's algorithms. It cannot, without a second connection, because the client does not expose the peer's chain (R-sc79-10).
- **P5, cost:** held at the edge. RSA-4096 (0.42) and P-384 (1.58) fell inside their predicted ranges. RSA-2048 came in at 0.099, against a predicted 0.1–0.4, at the range's floor.
- **P6, existing rows:** held. `unsupported_alg_row.lu` now names Ed448, because its RSA certificate validates. `rfc8448_parse_vectors.lu`'s answer moves from `unsupported_alg` to `weak_key`, because the trace's key is RSA-1024. Every other existing row keeps its ledger word under the final record's `std-test`.
- **P7, the plant:** see the evidence below.

## Evidence index

### The modules

- `std/x/crypto/rsa/rsa.lu`: the key policy (each refusal has a name), DER `RSAPublicKey`, a bignum in 24-bit limbs with CIOS Montgomery multiplication, R² by doubling and squaring, square-and-multiply, and the two verifiers exactly as RFC 8017 writes them.
- `std/x/crypto/p384/p384.lu`: Montgomery field and scalars in 16 × 24-bit limbs, Jacobian points with every exceptional case explicit, Shamir's joint multiplication, and the X = r·Z² test with no field inversion.
- `std/x/tls/cert/cert.lu`: the signature table (`verify_signed_by`), RSASSA-PSS parameters (RFC 4055), the EC named curve, extendedKeyUsage, nameConstraints detection, `san_matches` (RFC 9525), and `validate_path`, which searches depth first with backtracking, tries roots first, and stops at 8 certificates and 16 link attempts. `validate_chain` is now its one-anchor case.
- `std/x/tls/client/client.lu`: `signature_schemes`, three new scheme constants, `verify_certificate_verify` for the new schemes, and `complete` over `validate_path`.

### Vectors, checkers, rows

| what | where | lanes |
|---|---|---|
| RSA checker, from RFC 8017 | `tests/x/crypto/rsa/rsa_check.py`, output `rsa_check.txt` | gauntlet `rsa-checker` (output must match byte for byte) |
| P-384 checker, from the standards | `tests/x/crypto/p384/p384_check.py`, output `p384_check.txt` | gauntlet `p384-checker` |
| Wycheproof RSA, 17 files, generated | `tests/x/crypto/rsa/wycheproof_*.lu` (`cargo xtask gen-vectors`) | native; lupin and checked `unsupported` (step budgets); the 4-case smoke file runs on lupin |
| Wycheproof P-384, 4 files in 17 parts | `tests/x/crypto/p384/wycheproof_*.lu` | native; lupin and checked `unsupported` |
| key policy, early refusals, selector trap | `tests/x/crypto/rsa/{key_policy_rows,verify_reject_early,hash_selector_trap}.lu`, `tests/x/crypto/p384/verify_reject_early.lu` | all three |
| 2049-bit PSS (emLen = k − 1) | `tests/x/crypto/rsa/rsa2049_rows.lu` | native |
| P-384 group self-test | `tests/x/crypto/p384/self_test.lu` | native |
| RSA, RSA-PSS, P-384 and mixed-EC chains | `tests/x/tls/cert/{rsa_chain_valid,p384_chain_valid}.lu` | native |
| root set, cross-sign, unparseable root | `tests/x/tls/cert/root_set_rows.lu` | native |
| every refusal by name | `tests/x/tls/cert/public_pki_refusals.lu` | native |
| RFC 9525 matching | `tests/x/tls/cert/san_matches_rows.lu` | all three |
| CertificateVerify schemes and refusals | `tests/x/tls/client/cert_verify_schemes.lu` | native |
| the test PKI | `vendor/vectors/pki/sc79/` (built by `gen_sc79.sh`, checked by `openssl verify`, `verify.txt`) | — |
| hex provenance | `docs/sc79-witness/hex_provenance.py` | gauntlet `hex-provenance` |

The refusals, each from `public_pki_refusals.lu`, `root_set_rows.lu`, `key_policy_rows.lu` or `cert_verify_schemes.lu`:

| refusal | certificates or keys |
|---|---|
| `weak_key` | a 1024-bit leaf; a 1024-bit issuer; a 1024-bit key in CertificateVerify; RFC 8448's own RSA-1024 trace key |
| `bad_usage` | a clientAuth-only leaf; a leaf whose keyUsage lacks digitalSignature (OpenSSL's sslserver purpose accepts it); an emailProtection-only intermediate |
| `name_constraints` | an intermediate with nameConstraints (OpenSSL accepts the chain) |
| `not_ca` | an intermediate with cA FALSE; a root with pathLenConstraint 0 above an intermediate |
| `unsupported_alg` | sha1WithRSAEncryption, P-521, Ed448; the rsa_pkcs1 and rsa_pss_pss schemes |
| `bad_signature` | one flipped signature byte under RSA, RSA-PSS and P-384; a scheme that does not fit its key |
| `expired`, `not_yet_valid`, `name_mismatch`, `issuer_mismatch` | the RSA chain at 2037 and at 2026-01-01; `example.com`; a wildcard two labels deep; the bare parent of a wildcard; the wrong root; a cross-sign alone |
| `small_modulus`, `large_modulus`, `bad_modulus`, `bad_exponent`, `der` | the RSA key policy |

**Red first.** The Wycheproof rows were committed at `4e9ae98`, before either module existed: they read `fail(E0301)` on the compiler lanes (`docs/sc79-witness/red-4e9ae98.log`). The public-PKI rows were committed at `6a4daee` against trunk's cert and client: they read `fail(E0301)`, and RFC 8448's row still read `unsupported_alg` (`red-6a4daee.log`). `9b4ccb5` and `b593700` turn the rows green.

### Interop

`docs/sc79-witness/interop/run.py` builds `client/client.lu`. It serves each chain from the pinned OpenSSL `s_server`, held to a single scheme, gives the probe that chain's root and the time 2027-06-01, and requires a completed handshake plus one HTTP/1.0 exchange.

| row | at trunk `3111d39` | at the head |
|---|---|---|
| `interop-rsa-pss-sha256` | `complete=unsupported_alg` | complete |
| `interop-rsa-pss-sha384` | `complete=handshake_failure` | complete |
| `interop-rsa-pss-sha512` | `complete=handshake_failure` | complete |
| `interop-rsa-pss-certificates` | `complete=unsupported_alg` | complete |
| `interop-p384-chain` | `complete=handshake_failure` | complete |
| `interop-p256-under-p384` | `complete=unsupported_alg` | complete |
| `interop-ed25519-control` | complete | complete |
| `github-handshake` (120 distribution roots) | `unsupported_alg` (den's vc00) | complete, 5679 application bytes in 0.3 s |

The trunk run is `docs/sc79-witness/interop-trunk-3111d39.log`. The final record's `interop-local` and `github-handshake` steps give the head's results. If github.com does not resolve, `github-handshake` is a named SKIP, and the row makes no connection.

### The cost

`docs/sc79-witness/bench/bench.lu` runs one warm-up batch and five interleaved batches of 32 calls, and takes the median per call. Its inputs are the first valid case of each vendored Wycheproof file. It ran natively on one Linux x86-64 block of six CPUs under `taskset`, holding the block's two locks and its light lock, so nothing ran beside it. The run was at `b22ada4`, whose `std/` is identical to the head's (log: `docs/sc79-witness/cost-b22ada4.log`). That is the pre-rewrite name of `514cfcd`: before the first push, three witness logs were rewritten in place to drop the scheduler's lock-helper lines, so every later commit was replayed, and the trees are otherwise identical.

| verify | default build (the rig's lane) | ratio to `p256.verify` | predicted | `--release` | ratio |
|---|---|---|---|---|---|
| `p256.verify` (the baseline) | 16.35 ms | 1 | — | 7.11 ms | 1 |
| RSA-2048, PKCS#1 v1.5, SHA-256 | 1.63 ms | 0.099 | 0.1–0.4 | 0.64 ms | 0.090 |
| RSA-2048, PSS, SHA-256 | 1.67 ms | 0.101 | — | 0.65 ms | 0.091 |
| RSA-4096, PKCS#1 v1.5, SHA-256 | 6.83 ms | 0.417 | 0.3–1.2 | 2.95 ms | 0.415 |
| P-384, SHA-384 | 25.82 ms | 1.58 | 1–4 | 8.27 ms | 1.16 |

RSA-2048 lands at the bottom of its predicted range, 0.099 against a predicted 0.1. The other two land inside theirs. The final record's `cost-witness` step repeats the measurement, and the PR body quotes it.

### The plant

`sc79-plant-pss-salt` is pushed without a PR. It is one commit, one file and one hunk on the final head, and it reads the PSS salt one octet short. Its record names `pss-2049-sha256-salt32` and `rsa_pss_2048_sha256_mgf1_32:tc1`, each a `FAIL` line in the nonzero `sc79-rows-native` step. PKCS#1 v1.5 rows and salt-length-0 PSS rows stay green.

### The gauntlet

`docs/sc79-witness/gauntlet.sh` runs these steps in order:

1. the toolchain, and the sc79 rows natively;
2. both checkers, hex provenance, the local interop rows, and the github.com row;
3. every `cargo xtask ci` step: `time-oracle`, `fmt`, `clippy`, `test`, `sync-pin`, `doctor`, `ledger-check`, `lint-conventions`, `fmt-lu`, `gen-vectors`, `std-test` (624 files, three lanes), `doc-examples` and `ulp`;
4. the cost witness.

The local-gate recorder writes the green and plant records. The PR body gives their paths and every `END` line.

## Rulings asked (R-list)

- **R-sc79-1: fetching the vectors.** The network clause makes github.com:443 the only external host, reached "only from the row that needs it", while item 3 asks for the Wycheproof files to be vendored by digest. Done: one sparse `git` clone from github.com, and no other host contacted. The RFC texts were not fetched, because rfc-editor.org is a second host. Options: (A) accept this reading, and let future contracts say that a vendoring fetch from github.com is allowed; (B) have the orchestrator vendor inputs before a lane starts. **Recommended: A**, plus a follow-on that vendors RFC 8017, 5280, 4055 and 9525 into `vendor/vectors/rfc/` from a host the orchestrator names.
- **R-sc79-2: the exponent floor.** RFC 8017 §3.1 allows e ≥ 3, while FIPS 186-5 asks for e > 2^16. Implemented: odd, 3 ≤ e < n and e < 2^256, which is the CA/Browser Forum's MUST. The e = 3 Wycheproof valid cases are accepted. Encode-and-compare closes the e = 3 forgery class that parsed padding opened. Options: (A) as built; (B) refuse e ≤ 2^16 by name. **Recommended: A.**
- **R-sc79-3: MissingNull.** Wycheproof marks a DigestInfo without its NULL as `acceptable`. Implemented: reject, because RFC 8017 §9.2 Note 1 includes the NULL. **Recommended: keep.**
- **R-sc79-4: extendedKeyUsage on issuers.** RFC 5280 does not chain EKU. Implemented: an issuer that carries EKU must allow serverAuth or any purpose, so a CA restricted to email does not issue TLS servers. **Recommended: keep.**
- **R-sc79-5: roots held to the profile.** A root that ends a path is checked like any certificate: validity, cA, keyCertSign, EKU and key policy. This is stricter than RFC 5280 §6.1.1(d), and it continues sc22's reading. A v1 root without basicConstraints would be refused. **Recommended: keep.** No such root is in the distribution bundle the github.com row used.
- **R-sc79-6: the row vocabulary grew.** `weak_key`, `bad_usage` and `name_constraints` join `validate_chain`, `validate_path`, `verify_signed_by` and `client.complete`, and `named`/`row_name` handle them. An exhaustive downstream `match` on those rows stops compiling. In the org, the one consumer of `complete` goes through `named`, and `host_matches` stays exact for its server-side caller. **Recommended: accept** (this is the x tier, and the CHANGELOG names the break).
- **R-sc79-7: `signature_algorithms_cert`.** It is not sent, so a server that chooses its chain by `signature_algorithms` sees no PKCS#1 v1.5 scheme, even though the client verifies sha256/384/512WithRSA certificates. Options: (A) leave it; (B) send `signature_algorithms_cert` listing the certificate algorithms `verify_signed_by` accepts. **Recommended: B**, as a follow-on with a row against an OpenSSL peer holding two chains.
- **R-sc79-8: the modulus ceiling.** Implemented at 4096 bits, as the contract sized it. Over 4096 bits is `large_modulus` in the rsa module and `unsupported_alg` in a path. **Recommended: keep until a consumer names an 8192-bit root.**
- **R-sc79-9: P-256 under SHA-384 or SHA-512.** `unsupported_alg`, because p256's surface hashes with SHA-256 itself. **Recommended: add `p256.verify_digest_der` when a consumer meets such a certificate.** It is rare in public PKI.
- **R-sc79-10: the peer chain is not visible.** The github.com row records success, not the chain the server sent, because `Client` does not expose it and a second connection is not allowed. **Recommended: a follow-on that returns the validated path (or the leaf and its issuer names) from `complete`, for diagnostics.**

## Done-when

- [x] Branch `sc79` from trunk `3111d39`; pins unchanged.
- [x] Prediction committed first (`ba4c946`), every line checked above.
- [x] `std.x.crypto.rsa`: PKCS#1 v1.5 and PSS with SHA-256/384/512, every refusal named, every Wycheproof case by tcId, and an independent RFC 8017 checker.
- [x] `std.x.crypto.p384`: verify only, mirroring p256, every Wycheproof case, and an independent checker.
- [x] `std.x.tls.cert`: v3 extensions, root-set path building, the full signature table, basicConstraints, pathLen, keyUsage, EKU, the supplied time, RFC 9525 names, and name constraints refused by name; rows red first.
- [x] `std.x.tls.client`: six schemes offered and verified, roots and time taken from the caller; OpenSSL peers red at trunk and green after; github.com:443 completing with the distribution's roots.
- [x] `std/x/README.md`, the module headers, `README.md` (newest surfaces, 624 tests) and `CHANGELOG.md` under Unreleased.
- [x] Follow-ons filed: #126 (P-521), #127 (RSA signing), #128 (revocation), #129 (name constraints).
- [ ] The green record and the plant record at the final head (paths and `END` lines in the PR body).
