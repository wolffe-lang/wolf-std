# sc77 — wolf-std: a strict Ed25519 verify that refuses small-order points

Footprint: module

`std.x.crypto.curve25519.verify_strict(pubkey, message, sig) -> bool` is new (wolf-std#120). It does everything `verify` does and also answers false when the public key A or the commitment R has small order, meaning `[8]P` is the identity. The check is three `ge_double`s and one `ge_eq`, both the module's own. `verify` is byte-for-byte unchanged, and its doc comment now names the difference and points to `verify_strict`. A downstream package manager's trust profile needs the refusal, and it must not grow point arithmetic of its own.

## Forbidden

No implementation source of any cryptographic library was read: no OpenSSL, BoringSSL, libsodium, NaCl, ref10, Go, RustCrypto, dalek or speccheck code, and no Python `cryptography` or `nacl`. The sources were RFC 8032's text (`vendor/vectors/rfc/rfc8032.txt`, §5.1, §5.1.7, §8), the Wycheproof vectors std already vendors, and this repository's own code. The small-order points come from our own Python generator, written from the curve equation. Its one library call with cryptographic meaning is SHA-512 through Python's `hashlib` API, used as an oracle.

Nothing else changed: no API or verdict of an existing function, no caller (`jose`, `tls/cert`, `tls/client`), no consumer repository, no toolchain pin (wolf 0.2.36, lupin 0.1.58), no CI file and no rig code. Linux work ran on kasumi CPUs 4–7 under one `flock` per take, `taskset` inside a 12 GiB container, with scratch under the lane's own directory and never in system temp. No Mac build was run. No label, merge, tag or issue was touched.

## Inputs, verified

- **Base:** wolf-std trunk `d407430`, branch `sc77`.
- **Toolchain:** release archives fetched on kasumi and hashed twice. Both match the release API digests: wolf `d383c74c8cd123b9d9dfaa268271d4f004294f85c2b230237d0ef07027d9500d` and lupin `34f52792bb59bc7565e0edc1235cd47d9a6ad13d5e6528993627f2e699b62193`. The wolf binary is `f7b73fba…` and lupin's is `47c646a6…`, the hashes sc75 recorded; `libwolf_rt.a` is `47fb9ad0…`. The version lines read `wolf 0.2.36 (wolfgang, pin 5adf3d9)` and `lupin 0.1.58 (… pin a5378e3)`.
- **What verify does today** (`curve25519.lu`, `pub fn verify`, unchanged since `5b3a599`): it checks the lengths, `S < L`, canonical decompression of A and R (`ge_decompress`, RFC 8032 §5.1.3), and the cofactorless equation `[S]B = R + [k]A`. It refuses no small-order point, and RFC 8032 §5.1.7 allows that.
- **`API-CONVENTIONS.md`** is at the repository root, not under `docs/`. Read §1–§4, §12–§13, and the sc72/sc75 amendments. The name `verify_strict` follows `verify`'s precedent: it is a bool-answering verifier, not an `is_` predicate. The new private helper is `ge_is_small_order`.
- **The module's register row** (`std/x/README.md`, `crypto/curve25519`) and its module header were read, and both now name `verify_strict` and the mixed-order follow-on. The README was read and refreshed: the newest-surface sentence and the test count, 561 → 569.
- **The byte-identity of verify** is a gauntlet step (`verify-unchanged`). It hashes `pub fn verify`'s body, and the bodies of `sign`, `public`, `x25519`, `x25519_base`, `field_self_test` and `to_hex`, against `d407430`. It also fails if any path under `std/x/jose`, `std/x/tls`, the other crypto modules, `vendor`, `xtask`, `ci`, `.github` or the toolchain file changed.

## Prediction

Committed first, at `d48889f` (`docs/sc77-prediction.md`), before the generator, any row, or any change to `std/`. Every line held.

- **P1, eight points and six non-canonical encodings:** held. The generator solves `[8]P = O` case by case from the doubling map (x = 0, y = 0, x² = −y²), and reaches the same set a second way: it projects 22 ordinary curve points through `[L]Q`. The encodings it computes equal the eight the prediction quoted from the literature's prose.
- **P2, the rows:** held. `verify` accepts exactly the A rows where ord(A) divides k, 7 of the 16, and exactly the two identity R rows. `verify_strict` refuses all 44.
- **P3, Wycheproof:** held. No case changes verdict, 0 of 151. tcId 10–19 and 60 carry a small-order R and were already invalid.
- **P4, cost:** held. The small-order check adds +0.9% to `verify`, against a predicted +0.5% to +1.5%. The `[L]A` prototype adds +41% (predicted +35% to +45%), and `[L]A` with `[L]R` adds +82% (predicted +70% to +90%).
- **P5, nothing else moves:** held. The strict rows run on all three lanes. The verify column and the strict Wycheproof parts are native-only. Every existing row keeps its ledger word in the final record's `std-test`.
- **P6, the plant:** held. Dropping the R check turns exactly `strict-r-identity-empty` and `strict-r-identity-72` red.

One thing the prediction did not foresee, now pinned as a row: under the cofactorless equation, a torsion-free A forces R into the prime-order subgroup. A mixed-order refusal therefore needs `[L]A` alone, not `[L]R` as well (R-sc77-2).

## Evidence index

### The generator and its output

- `tests/x/crypto/curve25519/small_order.py`: our own generator. It has its own field and affine group law, RFC 8032 §5.1.2/§5.1.3 encode and decode, and a reference cofactorless verify. It checks B against RFC 8032 Table 1 and `[L]B = O`, and TEST 1's key against the RFC. It emits nine files. `small_order.py --check` (gauntlet step `small-order-check`) regenerates them in memory through `wolf fmt -` and fails on any byte of drift.
- `tests/x/crypto/curve25519/small_order.txt`: the derivation record. It lists each point's x and y, its order, and the chain `[1..8]P`; the six non-canonical encodings with the reason decoding refuses each; every row's `verify` and `verify_strict` verdict; the mixed-order rows; and the Wycheproof classification.

### The vector table

The A rows use R = the identity and S = 0. The R rows sign under RFC 8032 §7.1 TEST 1's key with S = k·a mod L. The two messages are the empty message and the one byte `72`.

| point | order | canonical encoding | `verify` as A (empty, 72) | `verify` as R (empty, 72) | `verify_strict` |
|---|---|---|---|---|---|
| identity | 1 | `0100000000000000000000000000000000000000000000000000000000000000` | true, true | true, true | false (all 4) |
| order2 | 2 | `ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f` | false, true | false, false | false (all 4) |
| order4-0000x0 | 4 | `0000000000000000000000000000000000000000000000000000000000000000` | false, true | false, false | false (all 4) |
| order4-0000x1 | 4 | `0000000000000000000000000000000000000000000000000000000000000080` | true, true | false, false | false (all 4) |
| order8-26e8x0 | 8 | `26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05` | false, false | false, false | false (all 4) |
| order8-26e8x1 | 8 | `26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85` | false, false | false, false | false (all 4) |
| order8-c717x0 | 8 | `c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a` | true, false | false, false | false (all 4) |
| order8-c717x1 | 8 | `c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa` | false, false | false, false | false (all 4) |

An A row is accepted by `verify` exactly when k mod ord(A) = 0 (`small_order.txt` gives k mod ord(A) for each row). Under a real key, R + [k]A has a torsion part unless R is the identity, so only the identity R rows pass. Those two rows are the plant's named reds.

The six non-canonical encodings each appear as A and as R, and `verify` and `verify_strict` both answer false on all 12 rows:

| encoding | why RFC 8032 §5.1.3 refuses it |
|---|---|
| `edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f` | y = p ≥ p |
| `edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff` | y = p ≥ p |
| `0100000000000000000000000000000000000000000000000000000000000080` | x = 0 with x_0 = 1 |
| `eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f` | y = p + 1 ≥ p |
| `eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff` | y = p + 1 ≥ p, and x = 0 with x_0 = 1 |
| `ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff` | x = 0 with x_0 = 1 |

### Wycheproof differences by tcId

**None.** All 151 cases in `vendor/vectors/wycheproof/ed25519_test.json` (88 valid, 63 invalid) keep their verdict under `verify_strict`. The generator's own reference verify agrees with every `result`. No key among the 52 distinct keys (78 groups) has small order. A small-order R appears only in invalid cases, tcId 10, 11, 12, 13, 14 and 60 (`00…00`, order 4) and tcId 15, 16, 17, 18 and 19 (the identity), which `verify_strict` refuses on two grounds. The strict expected verdicts are in `wycheproof_ed25519_strict_p1.lu`–`p4.lu` and `_smoke.lu`.

### The rows and their lanes

Lanes were measured by hand at `7e1e93c` (`docs/sc77-witness/lanes-7e1e93c.log`), and the mixed-order file the same way when it was generated. The final record's `std-test` holds every word: a row that runs deeper or shallower than its ledger entry is red.

| file | rows | lupin | checked | native |
|---|---|---|---|---|
| `ed25519_strict_small_order.lu` | 44 (`verify_strict`, all false) | run (8.1 s) | run (2.4 s) | run |
| `ed25519_small_order_verify.lu` | 44 (`verify`, 9 true) | unsupported | unsupported | run |
| `ed25519_strict_mixed_order.lu` | 2 (both true: the limit) | run (15.1 s) | unsupported | run |
| `wycheproof_ed25519_strict_smoke.lu` | tc 1, tc 10 | run (4.4 s) | run (1.3 s) | run |
| `wycheproof_ed25519_strict_p1.lu`–`p4.lu` | 40 + 40 + 40 + 31 | unsupported | unsupported | run |

**Red first.** The rows were committed at `44012f0`, before `verify_strict` existed. There the strict file is `fail(E0301)` ("module `curve25519` has no item named `verify_strict`"), and the verify column is `exit(0)`: `verify` accepts its 9 rows (`docs/sc77-witness/red-44012f0.log`). `7e1e93c` adds `verify_strict`.

### The cost

Native, on kasumi under `taskset -c 4-7` (the host's other slots were busy, load about 12). Each function ran one warm-up batch of 32 RFC 8032 TEST 1 verifications and then five timed batches of 32, interleaved batch by batch. The figures are the median per call (`docs/sc77-witness/bench.lu`, `bench_full.lu`, log `cost-ac00335.log`).

| build | `verify` | `verify_strict` | strict / verify | `+[L]A` (prototype) | `+[L]A +[L]R` (prototype) |
|---|---|---|---|---|---|
| default (`wolf run`, the rig's native lane) | 1.693 ms | 1.708 ms | **+0.9%** | 2.396 ms, +41.0% | 3.087 ms, +81.7% |
| `--release` | 0.504 ms | 0.509 ms | **+1.0%** | 0.711 ms, +40.8% | 0.912 ms, +80.5% |

In the prototype columns, `verify` is the baseline from the same run (1.699 ms and 0.505 ms). The final gauntlet repeats the measurement as its `cost-witness` step. The prototype is `docs/sc77-witness/torsion-free.patch`, applied to a copy of `std/` and never committed there. It refuses the generator's mixed-order key, which `verify_strict` accepts.

### The plant

Branch `sc77-plant-r`, pushed without a PR. It is one commit, one file and one hunk on the final head, and it deletes `verify_strict`'s R small-order check. Its local record names `strict-r-identity-empty` and `strict-r-identity-72`, each a `FAIL` line in the nonzero `strict-rows-native` step. A dry run of the same break printed exactly those two lines and nothing else (`ed25519_strict_mixed_order`, `wycheproof_ed25519_strict_p1` and `_smoke` stayed green).

### The gauntlet

`docs/sc77-witness/gauntlet.sh` runs every `cargo xtask ci` step, then `small-order-check`, `verify-unchanged`, `strict-rows-native`, the full `std-test` (569 files, three lanes), `doc-examples`, `ulp` and `cost-witness`. The final head's green record and the plant's red record are written by the local-gate recorder, each to a new output path. Their paths and every `END` line are in the PR body.

## Rulings asked (R-list)

**R-sc77-1: should std's callers of `verify` move to `verify_strict`?** None was moved. These are every std call site of `verify`:

| site | what it verifies | recommendation |
|---|---|---|
| `std/x/jose/jose.lu:252` (`jws_verify_compact`) | a JWS under a caller-supplied key, such as an ACME account key carried in a `jwk` header | move |
| `std/x/tls/cert/cert.lu:1378` (`verify_signed_by`) | a child certificate's TBS bytes under its issuer's SPKI key | move |
| `std/x/tls/client/client.lu:730` (`verify_certificate_verify`) | TLS 1.3 CertificateVerify under the validated leaf's key | move |
| `std/x/crypto/curve25519/curve25519.lu:1491`, `:1532` (the `sign` and `verify` doc examples) | an honest signature | stay (they document `verify`) |
| `std/x/crypto/curve25519/curve25519.lu:1589` (the `verify_strict` doc example) | the identity forgery | stay (it shows the difference) |

Options: (A) move all three code sites in one follow-on lane, each with a row that refuses a small-order key at that site; (B) move only the two TLS sites, where a third party certifies the key; (C) keep `verify` everywhere and document it. **Recommended: A.** No honestly generated key or signature changes verdict, since RFC 8032 key generation never yields a small-order A, and the cost is about 1%. At each site a small-order key is one under which anyone can sign every message, so accepting it proves nothing about who signed. RFC 8032 §8.8's concern, implementations disagreeing about the set of valid signatures, argues for doing all three sites at once.

**R-sc77-2: mixed-order points.** A point with a prime-order part and a nonzero small-order part is not small order, so `[8]P` does not catch it. `ed25519_strict_mixed_order.lu` pins two such keys: TEST 1's A plus a point of order 2, and plus a point of order 8. Both `verify` and `verify_strict` accept them, for messages whose k is a multiple of the torsion part's order. Refusing them needs `[L]P = O`, which is a full scalar multiplication. Measured native: +41% per verify for `[L]A`, +82% with `[L]R` as well. Under the cofactorless equation `[L]R` is redundant: if A is torsion-free, `R = [S]B − [k]A` lies in the prime-order subgroup, so a mixed or non-identity small-order R can never verify. Options: (A) leave it a named follow-on, as built; (B) add `[L]A` to `verify_strict` now, at +41% on every call; (C) a separate key predicate, `curve25519.is_torsion_free(pubkey) -> bool`, paid once per key at load, not once per signature. **Recommended: A now, and C when a caller asks.** The forgery this lane closes (one signature valid for every message) needs a small-order A, and that is refused. Only the holder of a mixed-order key's prime-order secret can sign under it, and its effect is that cofactored and cofactorless verifiers disagree on some of its signatures. A trust profile that pins keys pays for C once per key.

## Done-when

- [x] Branch `sc77` from `d407430`, under the lane's own directory; no pin change.
- [x] Prediction committed first (`d48889f`), every line checked above.
- [x] The independent generator and its output committed (`260525b`, `04fb294`, `762ade5`), with the orders shown by its own arithmetic.
- [x] Rows red first (`44012f0`, `fail(E0301)`), then green with `verify_strict` (`7e1e93c`).
- [x] Wycheproof differences listed by tcId (none) and every vector run under `verify_strict`.
- [x] `verify` byte-for-byte unchanged; its doc comment names the difference (`c31bf05`); no caller moved; R-sc77-1 lists every call site.
- [x] Mixed-order cost measured; R-sc77-2 written; only the small-order check implemented.
- [x] `ct:` stated on `verify_strict` in the sc18 verify-is-public style.
- [x] Cost measured on kasumi under taskset (+0.9%).
- [x] Register row, module header, README and changelog refreshed.
- [ ] Final-head green record and plant red record: their paths and `END` lines are in the PR body, where CI is informational under the local gate.
