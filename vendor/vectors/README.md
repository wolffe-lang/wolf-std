# vendor/vectors — the crypto vector corpora (sc16/sc17, D53's ladder)

Public test vectors, vendored VERBATIM with provenance, consumed by
`cargo xtask gen-vectors` (which emits the committed
`tests/x/crypto/sha2/cavp_*`/`wycheproof_*` and
`tests/x/crypto/chacha20/wycheproof_*` files and is drift-checked in
ci) and quoted by the hand-written
`tests/x/crypto/sha2/rfc4231_hmac.lu` / `rfc5869_hkdf.lu` and
`tests/x/crypto/chacha20/known_answers.lu` / `rfc8439_a*.lu`. Never edit a
vendored file; re-vendor from the source and re-run the generator, each
in its own commit. The spec documents themselves (FIPS 180-4, FIPS
198-1, the RFC texts, the CAVP zip) are fetched into the planning repo's
`refs/specs/` per the report-11 provenance discipline; what lives here
is exactly the vector data the rig consumes.

## Sources (retrieved 2026-08-26)

- `cavp/SHA{256,384,512}{ShortMsg,LongMsg,Monte}.rsp` — NIST CAVP
  Secure Hash Standard byte-oriented test vectors, from
  `https://csrc.nist.gov/CSRC/media/Projects/Cryptographic-Algorithm-Validation-Program/documents/shs/shabytetestvectors.zip`
  (files dated 2011-05-12 inside the archive, CAVS 11.0/11.1).
- `wycheproof/hkdf_sha{256,384,512}_test.json` — Project Wycheproof
  (C2SP), `testvectors_v1` at
  `https://github.com/C2SP/wycheproof/blob/master/testvectors_v1/`.
- `rfc/rfc4231.txt`, `rfc/rfc5869.txt` — the RFC Editor's canonical
  texts (`https://www.rfc-editor.org/rfc/`).

## Sources (retrieved 2026-08-26, sc17 — the AEAD rung)

- `rfc/rfc8439.txt` — the RFC Editor's canonical text
  (`https://www.rfc-editor.org/rfc/rfc8439.txt`), the implementation
  authority for `std.x.crypto.chacha20` and the source its hand-written
  vector tests quote.
- `wycheproof/chacha20_poly1305_test.json` — Project Wycheproof (C2SP),
  `testvectors_v1` at
  `https://github.com/C2SP/wycheproof/blob/master/testvectors_v1/`.

sha256 of every vendored file, as retrieved:

```
6fac36f37360bcf74ffcf4465c18e30d6d5a04cc90885b901fc3130c16060974  cavp/SHA256LongMsg.rsp
29ea30c6bb4b84e425fb8c1d731c6bb852dac935825f2bd1143e5d3c4f10bfb9  cavp/SHA256Monte.rsp
75e1cb83994638481808e225b9eb0c1ebd0c232d952ac42b61abce6363be283c  cavp/SHA256ShortMsg.rsp
536171765a4278c000ac3c9913edb2eed0ca7ccd5a10b72ed79fdfe7901a6d6a  cavp/SHA384LongMsg.rsp
4270099431ff52ee1686dc472351e681c26c507433df8f107c7de203b771424e  cavp/SHA384Monte.rsp
7ea7bcf00fadc20949fae63703e40681ddf288fea808471cb3cbc95f3ec16811  cavp/SHA384ShortMsg.rsp
b1f3f05d5c209777954d49521d7ea1349447c36a0c52849e044bc397a27dd410  cavp/SHA512LongMsg.rsp
8ca78659286c2f01667a98fc7accd32fc171ae7b24ac00f1a8ce6b77770247fa  cavp/SHA512Monte.rsp
e53a36c03609e5a3e3cc4b6e117a499db7864c23ec825c6cec99503a45f40764  cavp/SHA512ShortMsg.rsp
bb2b462a38b251cb52a2aede706d6d4b62b26864f4e80c95497507ddb07c5f1e  wycheproof/hkdf_sha256_test.json
69ff6ea3657bb9c1b8cdffbbb4e7832353d08fd15c0d9997b03f7a6b180e3678  wycheproof/hkdf_sha384_test.json
bb9a21f4e86041caf5d7792b030349f8ff289087f195b2fbc0fc0afc39deca6f  wycheproof/hkdf_sha512_test.json
72178527ce93500e730bc8eb182b857e583096d652b64ece0879c52ba1df973b  rfc/rfc4231.txt
7a40eb3835b35fc947eb12a2ed614db079d43b26e50dbc537c31fba16397089c  rfc/rfc5869.txt
25bef70fbf7a07ff45c2fe4cb7c6ce954eac687413d8610603268b4e4415324c  rfc/rfc8439.txt
fe61d25f90e1bde4461d00eafe61049e5f29bd999f36b766df9cda90906ad53d  wycheproof/chacha20_poly1305_test.json
```

## Sources (retrieved 2026-08-26, sc18 — the curve and the signature)

- `rfc/rfc7748.txt`, `rfc/rfc8032.txt` — the RFC Editor's canonical
  texts (`https://www.rfc-editor.org/rfc/`): the implementation
  authorities for `std.x.crypto.curve25519` (X25519 and Ed25519) and
  the source its hand-written vector tests quote. RFC 8446 §7.4.2 (the
  reject-on-zero requirement the module cites) is fetched to the
  planning repo's `refs/specs/` but not vendored here: it contributes a
  cited RULE, not vector data.
- `wycheproof/x25519_test.json`, `wycheproof/ed25519_test.json` —
  Project Wycheproof (C2SP), `testvectors_v1` at
  `https://github.com/C2SP/wycheproof/blob/master/testvectors_v1/`.
  NAMING NOTE: the sc18 contract calls the second file
  `eddsa_test.json`, which is its LEGACY-directory name; in
  `testvectors_v1` the Ed25519 file is `ed25519_test.json` (the legacy
  path 404s from raw.githubusercontent). The v1 file is the one
  vendored, its `schema` field still says `eddsa_verify_schema_v1.json`.

sha256 of the sc18 files, as retrieved:

```
279ca0ecc5e92e2962e27b846986aeb74729d9dd34bd4a04a362f80dcb596ad3  rfc/rfc7748.txt
ed63657ff389301282b169b0abde9b5dd2c7e4d524fdfa5da6ff3094fc93c4c3  rfc/rfc8032.txt
35c3f5231cf25cc640b524d403461deee9e49441d5d915a3a25b2c8ff5adbe7d  wycheproof/x25519_test.json
752d2ea7d7c6cf4736381b6cbacb61f8182b126ab7cd9b058f00c50084975536  wycheproof/ed25519_test.json
```

## Sources (retrieved 2026-08-27, sc20 — the record layer)

- `rfc/rfc8446.txt` — the RFC Editor's canonical text
  (`https://www.rfc-editor.org/rfc/rfc8446.txt`), TLS 1.3: the
  implementation authority for `std.x.tls.record` (§5 the record
  protocol, §7.1 HKDF-Expand-Label/Derive-Secret, §7.3 traffic key
  calculation). Vendored for the normative spec text the module cites,
  not for vector data.
- `rfc/rfc8448.txt` — the RFC Editor's canonical text
  (`https://www.rfc-editor.org/rfc/rfc8448.txt`), "Example Handshake
  Traces for TLS 1.3": the VECTOR SPINE. §3 (Simple 1-RTT Handshake,
  cipher suite TLS_AES_128_GCM_SHA256) publishes every intermediate
  secret, traffic key, IV and — crucially — the `info` (HkdfLabel)
  bytes and transcript-hash contexts, all quoted into the hand-written
  `tests/x/tls/record/` tests and cross-checked by an independent
  reference (hashlib/hmac HKDF, from-scratch ChaCha20-Poly1305)
  reproducing §3 byte-for-byte before any hex landed in a `.lu`.

sha256 of the sc20 files, as retrieved:

```
47871bc8820a2c3b6ea89f061055577058862cf543686b82d10131239702b3bd  rfc/rfc8446.txt
6564d1376d1ec744fc7a9993da15ebc1b9be361908b166091f47ef605c537fba  rfc/rfc8448.txt
```

## Sources (authored 2026-08-27, sc22 — the certificate)

- `pki/` — the sc22 Ed25519 test PKI: a self-signed root + leaf +
  the negative battery (wrong-anchor, non-CA-signer, unknown-critical)
  + an 18-variant malformed-encoding conformance set + an
  openssl-generated RSA/ECDSA parse-only pair. AUTHORED by
  `pki/gen_pki.py` (from-scratch RFC 8032 Ed25519 + hand-built X.690
  DER, deterministic seeds) and CROSS-CHECKED with openssl 3.6.3
  (`openssl verify` validates the chain; `openssl x509 -text`
  re-parses every field) — the extractor+reference discipline,
  inverted: no public X.509 corpus is shaped for an Ed25519-scoped
  validator, so the reference authors and the independent tool checks.
  Every file, clause map and omission is documented in
  `pki/README.md`; sha256s in `pki/sha256sums.txt`.

## Sources (retrieved 2026-08-27, sc23 — the Weierstrass curve)

- `rfc/rfc6979.txt` — the RFC Editor's canonical text
  (`https://www.rfc-editor.org/rfc/rfc6979.txt`), "Deterministic Usage
  of DSA and ECDSA": the implementation authority for
  `std.x.crypto.p256`'s deterministic nonces and the source of the
  A.2.5 (P-256, SHA-256) vectors its hand-written test quotes.
- `wycheproof/ecdsa_secp256r1_sha256_test.json` — Project Wycheproof
  (C2SP), `testvectors_v1` at
  `https://github.com/C2SP/wycheproof/blob/master/testvectors_v1/`.
- `cavp/SigVer.rsp`, `cavp/SigGen.txt` — NIST CAVP FIPS 186-4 ECDSA
  test vectors, from
  `https://csrc.nist.gov/CSRC/media/Projects/Cryptographic-Algorithm-Validation-Program/documents/dss/186-4ecdsatestvectors.zip`
  (archive dated 2020-01-14; the two files carry every curve/hash
  pairing — the generator consumes exactly the `[P-256,SHA-256]`
  sections, the rest is omitted by name below).

sha256 of the sc23 files, as retrieved:

```
456e8f17558fdbd206f968b96fc6f1b4a71ea331ab30ad17f711ab3adaa7d701  rfc/rfc6979.txt
182db4f3e230f6f9fa9f800d2a614dede30284b8e8438bbfe1171905402e9332  wycheproof/ecdsa_secp256r1_sha256_test.json
e9841c3d12e323042751460d0b8ef4bc59c2640105ec7da4852775f80ab10191  cavp/SigVer.rsp
95d0a19c03f6d2dc16e9e18fd5c3318e1952e0bc88a45127c996c35b4a15f4e2  cavp/SigGen.txt
```

Every sc23 vector was reproduced by a from-scratch big-int Python
reference (P-256 + FIPS 186-5 ECDSA + RFC 6979 + strict-DER signature
parsing) BEFORE vendoring: RFC 6979 A.2.5 `k`/`r`/`s` byte-exact, all
15 CAVP SigGen `(R,S)` regenerated from the published `(d, k)`, all 15
CAVP SigVer P/F verdicts, and all 484 Wycheproof verdicts — so a
transcription error and an implementation error cannot agree (the
sc17/sc18 discipline).

## Sources (retrieved 2026-11-05, sc79 — RSA and ECDSA P-384)

- `wycheproof/rsa_signature_{2048,3072,4096}_{sha256,sha384,sha512}_test.json`
  (RSASSA-PKCS1-v1_5), `wycheproof/rsa_pss_*_test.json` (eight RSASSA-PSS
  files, MGF1 over the message hash), and
  `wycheproof/ecdsa_secp384r1_{sha384,sha384_p1363,sha256,sha512}_test.json`
  — Project Wycheproof (C2SP), `testvectors_v1` at
  `https://github.com/C2SP/wycheproof/tree/master/testvectors_v1/`, from
  a sparse clone of the repository at commit
  `d17cc89fa5c8f4f4d378b5385ebf75da5d0926c9` (2026-10-23).
- The specifications behind the checkers are RFC 8017 (PKCS #1 v2.2),
  FIPS 186-5, SEC 1 v2 and SP 800-186. They contribute rules and the
  P-384 domain parameters, not vector data, and are not vendored.

Every case in these 21 files was decided by an independent Python
checker written from the specifications alone before a line of the wolf
modules existed: `tests/x/crypto/rsa/rsa_check.py` (RFC 8017's RSAVP1,
EMSA-PKCS1-v1_5 and EMSA-PSS with MGF1) and
`tests/x/crypto/p384/p384_check.py` (the curve from SP 800-186, FIPS
186-5 §6.4.2 verification, strict X.690 signature parsing). Python's
`hashlib` is the one library call with cryptographic meaning, used as
the SHA-2 oracle.

sha256 of the sc79 files, as retrieved:

```
660c948e5433279ecdf966e8a1386b69ecec618efc1ff00269e85aa9dc84f0a6  wycheproof/ecdsa_secp384r1_sha256_test.json
e27344bf6daae75fa19663620883809f91c79fc4b4420eff9eae8036b34476be  wycheproof/ecdsa_secp384r1_sha384_p1363_test.json
8a5b3ae1760975143414811f13588c24d951d9d8c904195087ba327591dfe9cc  wycheproof/ecdsa_secp384r1_sha384_test.json
f44bae32e419fc75c5b5af751cff4e35145cfbcc24944e378d5f30889d310513  wycheproof/ecdsa_secp384r1_sha512_test.json
b22a8d9a2e7e47f681d0ff1d1a55455daaf325d989c64ad005f848bd50b9b4c0  wycheproof/rsa_pss_2048_sha256_mgf1_0_test.json
7f6efafc160f4816b96cbf1c12188a31051d7e3f001e27505d9edb5f2a0e325c  wycheproof/rsa_pss_2048_sha256_mgf1_32_test.json
66d464778b0b2f683a1d1a20e94f77e9472b8cc393b45f423fa08a48e677063c  wycheproof/rsa_pss_2048_sha384_mgf1_48_test.json
cca48433e6d1accb57f65b3396c4f4dd93c776467101c3f0948e74c283fd4749  wycheproof/rsa_pss_3072_sha256_mgf1_32_test.json
e627cbf5139a01e0a446c4fe15911f6268224fdd3ef4a1eb714b3942b5686410  wycheproof/rsa_pss_4096_sha256_mgf1_32_test.json
d632649a7630f7c267eda576714ca11cf0ff068d6622f576b8701f923c112577  wycheproof/rsa_pss_4096_sha384_mgf1_48_test.json
af1e1c1c593edf91be3e9880927231d57f9df082f2d8caec2a61d9874ade101f  wycheproof/rsa_pss_4096_sha512_mgf1_32_test.json
c93ceaa56a190c9fd4707441c5c6a75839f108202d88aec891697f9623042547  wycheproof/rsa_pss_4096_sha512_mgf1_64_test.json
94a917b01ff50fb874cfc05bf29b4af44868d944a6558201cf18380da93fb393  wycheproof/rsa_signature_2048_sha256_test.json
c571c105d261c0ff588a2888a529f152563fb3b77894b7620d4e4f8f934f2c1d  wycheproof/rsa_signature_2048_sha384_test.json
16ea24b039905d054bdb6004f5fd179374e150b7b6d73a7ba654b6d00eab12ef  wycheproof/rsa_signature_2048_sha512_test.json
0f5f18cabfaad3e2792e82f7e9882f8999049b456714de924b8a5e202f61ca43  wycheproof/rsa_signature_3072_sha256_test.json
73499ee649d8fb703b3f0f455fa36f7682fde677c590a736958ebce05ab27dae  wycheproof/rsa_signature_3072_sha384_test.json
09aa3336fdb7be216180ce944ca7b2a052e1de38c210435c19506327d4a36ebd  wycheproof/rsa_signature_3072_sha512_test.json
957aca128e30bd02c8982f8ca482d6521d16683f4bbc3f193cdfd4c8f04bc667  wycheproof/rsa_signature_4096_sha256_test.json
bc26f16b9fc9a90628864d3d1111ef48a4bd00b9d4870bfab88ff326e309e073  wycheproof/rsa_signature_4096_sha384_test.json
9d6e67b1eb0d7040db411835238efd20e9302399a258bdff82de3f38e0f2855a  wycheproof/rsa_signature_4096_sha512_test.json
```

## What is taken, what is omitted (named, never silent)

- **CAVP short-message sets**: ALL vectors (65 + 129 + 129), run on
  every lane the module executes on (differential: interpreter +
  native).
- **CAVP long-message sets**: ALL vectors (64 + 128 + 128), native
  lane; the interpreter column is `slow` (the sc16 ledger word — see
  tests/ledger.toml's header).
- **CAVP Monte Carlo**: ALL 100 checkpoints per family, native lane;
  interpreter `slow`.
- **Omitted CAVP files**: SHA-1, SHA-224, SHA-512/224 and SHA-512/256
  sets — the algorithms are outside D53's ladder (SHA-384 is the only
  truncation the contract names). The bit-oriented vector sets are also
  not vendored: wolf's byte currency makes a bit-length message
  unrepresentable, and the generator hard-errors on a `Len` that is not
  a multiple of 8.
- **Wycheproof HKDF**: every `result: "valid"` vector (83 + 80 + 80).
  The nine non-valid vectors (three per digest, all flagged
  `SizeTooLarge`, L = 255 * HashLen + 1) are omitted from the generated
  tables because the API answers that shape with a documented `assert`
  trap, held by `tests/x/crypto/sha2/hkdf_expand_cap_trap.lu`; the
  generator hard-errors on any OTHER non-valid flavour, so a
  re-vendored file cannot silently shrink coverage.
- **RFC 4231**: all seven test cases for HMAC-SHA-256/384/512 (21
  vectors; case 5's published output is truncated to 128 bits and is
  asserted as the RFC publishes it). The HMAC-SHA-224 column is
  omitted: SHA-224 is outside the ladder.
- **RFC 5869**: the three SHA-256 cases (A.1-A.3), PRK and OKM both
  asserted. A.4-A.7 are omitted: HMAC-SHA-1.
- **RFC 8439 (sc17)**: the COMPLETE published set, 28 vectors, all
  asserted in the hand-written `tests/x/crypto/chacha20/` files: the
  five Section-2 worked examples (§2.3.2 block, §2.4.2 cipher both
  directions, §2.5.2 Poly1305, §2.6.2 key generation, §2.8.2 AEAD
  seal+open), A.1 block (5), A.2 encryption (3, each also
  round-tripped), A.3 Poly1305 (11, the reduction edge cases included),
  A.4 key generation (3), A.5 AEAD decryption (1, also re-sealed).
  §2.1.1/§2.2.1's quarter-round vectors are held transitively: a wrong
  rotation fails every block vector (the QR is not `pub`).
- **Wycheproof ChaCha20-Poly1305 (sc17)**: all 325 vectors accounted
  for, none silently. 256 `valid` (each SEALED to the published
  ciphertext||tag and OPENED back, 8 generated part files), 60
  `invalid` — every one `ModifiedTag` — asserted to answer the `tag`
  row (2 generated part files), and the 9 `InvalidNonceSize` vectors
  (nonce lengths 0/8/11/13/14/16/20/24/32 bytes) omitted by name
  because the API answers a wrong-size nonce with the documented
  `assert` trap held by `nonce_len_trap.lu` (the sc16 SizeTooLarge
  precedent). The file contains NO `acceptable` results — there was
  nothing to decide — and the generator hard-errors if a re-vendored
  file grows one or any other undecided flavour. Two deterministic
  smoke subsets (10 valid + 10 invalid) keep the interpreter's
  differential column while the part files ride the native lane
  (lupin `slow`, F-0093).

- **RFC 7748 (sc18)**: the complete published X25519 set, asserted in
  the hand-written `tests/x/crypto/curve25519/` files: the two §5.2
  one-shot vectors, the §5.2 iteration chain at 1 and 1,000 iterations
  (the 1,000,000-iteration checkpoint is omitted by name: it is 1000x
  the chain the rig already runs against a 60s per-test ceiling, and
  it gates no code path the 1,000 chain does not), and
  the §6.1 Diffie-Hellman exchange (both public keys, both directions
  of the shared secret). §5's decode masks and clamping are cited at
  the code that implements them.
- **RFC 8032 (sc18)**: all five §7.1 Ed25519 test vectors (TEST 1,
  TEST 2, TEST 3, TEST 1024 — the 1023-byte message — and TEST
  SHA(abc)): public key derived from the secret, signature asserted
  byte-for-byte, and every signature verified. Ed25519ctx/Ed25519ph
  (§7.2/§7.3) are omitted by name: the variants are outside the sc18
  contract (no TLS 1.3 or ACME consumer needs them; they join the
  ladder when a consumer names them).
- **Wycheproof X25519 (sc18)**: all 518 vectors accounted for, none
  silently, every flag DECIDED: 264 `valid` (shared secret asserted
  byte-for-byte), 223 `acceptable` with a nonzero shared secret
  (asserted byte-for-byte the same way — their flags are `Twist`,
  `NonCanonicalPublic`, `SpecialPublicKey` and friends, and RFC 7748
  requires masking the high bit and does not require twist or
  canonicity rejection, so the ladder's answer is the conformant one),
  and 31 `acceptable` flagged `ZeroSharedSecret` (small-order/
  low-order public keys), for which `x25519` must answer the `zero`
  row — the RFC 8446 §7.4.2 abort, pinned as this module's behaviour.
  The `ZeroSharedSecret` flag exactly characterizes the all-zero
  outputs in the vendored file (verified at vendoring time), and the
  generator hard-errors on any result/flag combination these rules
  have not named.
- **Wycheproof Ed25519 (sc18)**: all 151 vectors accounted for: 88
  `valid` (verify answers true) and 63 `invalid` (verify answers
  false — wrong-length signatures, truncated/garbage encodings, S >= L
  malleability, swapped halves, all decided by the single cofactorless
  verify with strict S < L and canonical decompression). The generator
  hard-errors on any other result flavour.
- **RFC 8448 §3 (sc20)**: the Simple 1-RTT Handshake trace, asserted in
  the hand-written `tests/x/tls/record/` files. TAKEN, byte-for-byte:
  the whole key schedule (early/handshake/master secrets via
  HKDF-Extract; the "derived"/"c hs traffic"/"s hs traffic"/"c ap
  traffic"/"s ap traffic"/"exp master"/"res master" secrets via
  Derive-Secret; the client+server finished keys) — 13 secrets; the
  `info`/HkdfLabel dumps for every one of those plus "key"/"iv"/
  "resumption" — 11 encodings (pitfall #1, the exact `"tls13 "` prefix);
  the four traffic key/iv pairs (server+client, handshake+application) —
  8 values (§7.3); and the header/AAD of the trace's actual encrypted
  records (§5.2 framing). **The AES-GCM reconciliation (F-0094):** RFC
  8448's records are AES-128-GCM, a cipher on its own rung (out of the
  sc20 contract — the MTI ChaCha20-Poly1305 is what this ladder has). So
  the key schedule / nonce / header / AAD — all AEAD-INDEPENDENT — are
  asserted against RFC 8448 directly, and the record AEAD seal/open
  byte-match uses ChaCha20-Poly1305 fixtures derived from RFC 8448's
  REAL server-handshake-traffic secret (a 32-byte `"key"` Expand-Label
  where the trace takes 16), generated by the independent reference and
  pinned. OMITTED by name: §4 (0-RTT), §5 (HelloRetryRequest), §6
  (Client Authentication), §7 (Compatibility Mode) — later handshake
  sprints; the AES-128-GCM record bodies — the AES-GCM rung's own
  D-question. Nothing about the trace is consumed silently: the
  extractor warns on any block whose parsed length disagrees with its
  header.

- **RFC 6979 (sc23)**: the A.2.5 P-256/SHA-256 vectors — the private
  key's public key (Ux, Uy) and both signatures ("sample", "test"),
  `k`, `r` and `s` asserted byte-exact in the hand-written
  `tests/x/crypto/p256/rfc6979_p256.lu`. The other-hash columns
  (SHA-1/224/384/512) and the other curves of A.2 are omitted by name:
  ES256 is SHA-256 and the ladder's curve is P-256 (a second hash
  joins when a consumer names it).
- **CAVP ECDSA (sc23)**: the `[P-256,SHA-256]` sections whole — all 15
  SigVer cases (P/F verdicts, the R/S/Q/Msg tamper classes) and all 15
  SigGen cases. SigGen's published `(R,S)` were made with the file's
  own random `k`, and std's sign is RFC 6979 deterministic (a
  DIFFERENT, equally valid `k`), so the generator asserts SigGen
  through VERIFY (Q against Msg/R/S — 15 more verify vectors) while
  the sign path is pinned byte-exact by RFC 6979 and round-tripped
  sign->verify; the reference reproduced all 15 `(R,S)` from the
  published `(d, k)` at vendoring time, so the file's self-consistency
  is checked even though std never uses a caller-supplied nonce. All
  other curve/hash sections are omitted by name (outside the ladder).
- **Wycheproof ECDSA secp256r1/SHA-256 (sc23)**: all 484 vectors
  accounted for, none silently: 174 `valid` and 310 `invalid` (the
  DER manipulations, r/s = 0 / = n / > n, the point-duplication and
  arithmetic-edge classes, wrong hashes), every one decided by the
  single strict-DER + FIPS 186-5 verify; the generator hard-errors on
  any result flavour other than valid/invalid, so a re-vendored file
  cannot grow an undecided case.

Totals: sc16 — 1210 vectors vendored and asserted (all green on the
native lane; 365 of them — the short sets, the RFC files and the
per-digest Wycheproof smoke subsets — differential under the
interpreter). sc17 — 344 assertions from 353 accounted-for vendored
vectors (28 RFC 8439 + 256 valid + 60 invalid Wycheproof, all green on
the native lane, +9 trap-omissions named above; the 28 RFC vectors and
the 20 smoke-subset vectors are differential under the interpreter,
and the Poly1305 file is three-lane — the checked tier runs it too).

- **Wycheproof RSA (sc79)**: 3395 cases accounted for, every one by
  tcId. RSASSA-PKCS1-v1_5, nine files, 2330 cases: 68 `valid` (verify
  answers true), 2253 `invalid` (false) and 9 `acceptable`, each file's
  one `MissingNull` case, which verification REJECTS: RFC 8017 §8.2.2
  builds the expected encoding and compares it, and §9.2 Note 1's
  DigestInfo carries the NULL. The `SmallPublicKey` cases (e = 3) are
  `valid` and verify: the exponent rule is RFC 8017 §3.1's, odd and at
  least 3, below n and below 2^256. RSASSA-PSS, eight files, 1065 cases:
  704 `valid` and 361 `invalid`, the salt length taken from each group's
  `sLen`. OMITTED by name: the SHA-1, SHA-224, SHA-512/224, SHA-512/256,
  SHA-3 and SHAKE files (outside the ladder), MGF1 under a hash other
  than the message hash (no TLS 1.3 scheme uses it), the `_params` files
  (RSASSA-PSS parameters carried in the key, which the module does not
  take), `rsa_pss_misc`, the 8192-bit files (the module refuses a
  modulus above 4096 bits by name; a hand row holds that refusal), and
  every OAEP and PKCS#1 encryption file.
- **Wycheproof ECDSA P-384 (sc79)**: 1798 cases accounted for: the
  SHA-384 file in DER (504) and in IEEE P1363 r || s form (280), and
  the SHA-256 (472) and SHA-512 (542) DER files, which exercise FIPS
  186-5's bits2int with a digest shorter and longer than the order.
  780 `valid`, 1018 `invalid`, no `acceptable`. The SHA-3 and SHAKE
  files are omitted by name (outside the ladder).
