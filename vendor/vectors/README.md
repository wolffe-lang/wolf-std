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

Totals: sc16 — 1210 vectors vendored and asserted (all green on the
native lane; 365 of them — the short sets, the RFC files and the
per-digest Wycheproof smoke subsets — differential under the
interpreter). sc17 — 344 assertions from 353 accounted-for vendored
vectors (28 RFC 8439 + 256 valid + 60 invalid Wycheproof, all green on
the native lane, +9 trap-omissions named above; the 28 RFC vectors and
the 20 smoke-subset vectors are differential under the interpreter,
and the Poly1305 file is three-lane — the checked tier runs it too).
