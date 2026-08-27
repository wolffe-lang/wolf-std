# vendor/vectors — the crypto vector corpora (sc16, D53's first rung)

Public test vectors, vendored VERBATIM with provenance, consumed by
`cargo xtask gen-vectors` (which emits the committed
`tests/x/crypto/sha2/cavp_*` and `wycheproof_*` files and is
drift-checked in ci) and quoted by the hand-written
`tests/x/crypto/sha2/rfc4231_hmac.lu` / `rfc5869_hkdf.lu`. Never edit a
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

Total: 1210 vectors vendored and asserted (all green on the native
lane; 365 of them — the short sets, the RFC files and the per-digest
Wycheproof smoke subsets — differential under the interpreter).
