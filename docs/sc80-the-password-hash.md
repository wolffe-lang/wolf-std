# sc80 — wolf-std: the password hash (scrypt, in crypt's `$7$` form)

Footprint: module

`std.x.crypto.scrypt` is scrypt (RFC 7914) with PBKDF2-HMAC-SHA-256 (RFC 8018 §5.2). `std.x.crypto.passwd` is the crypt(5) `$7$` string over it. `hash` writes the exact string Ubuntu 24.04's crypt writes for the same password, salt and cost. `verify` checks a stored string and compares the key in constant time. `gen_salt` encodes octets its caller drew from the OS random source. `parse` and `encode` are the string's two directions. Every other crypt method is refused by name as `unsupported`. The consumer is PAX's password lane: `passwd`, `login`, `su` and `sudo`.

## Forbidden

The lane read no crypto or crypt library's source: libxcrypt, glibc's crypt, OpenSSL, libsodium, Tarsnap's scrypt, Python's `_hashlib`, Go's x/crypto, BoringSSL or any other. It read no libc source either. The sources were RFC 7914, RFC 8018, RFC 2104, FIPS 180-4 (through `std.x.crypto.sha2`, unchanged) and the crypt(5) man page as Ubuntu 24.04 ships it. All four documents are vendored by digest under `vendor/vectors/` (`rfc/rfc7914.txt`, `rfc/rfc8018.txt`, `rfc/rfc2104.txt` and `man/crypt.5`).

crypt(5) gives the `$7$` string's regular expression and nothing of its inner encoding. The encoding was learned from Ubuntu's crypt run as a black box, in a container built from `ubuntu:24.04` pinned by digest (`sha256:68ee2200…`). That container holds libcrypt1 1:4.4.36-4build1, Python 3.12's `crypt` module, Perl 5.38's `crypt`, and `mkpasswd` from whois 5.5.22. Its strings were decoded under three hypotheses and checked against Python's `hashlib.scrypt`, used black box for raw scrypt output. One hypothesis held exactly (the module header and P0 below). The gauntlet runs in the org's Rust toolchain image (Ubuntu 24.04, the same libcrypt1, Perl and Python), with whois added (`localhost/sc80-gate:1`).

No password literal appears outside RFC 7914's vectors and generated test data. Every password a row uses is drawn at run time, either from a seeded generator, from SHA-256 of a numbered label, or from `secrets` in the interop harness. No password is written to disk. No default password exists anywhere.

The pins (wolf 0.2.36, lupin 0.1.58), CI and every consumer repository are unchanged. No label, merge or tag was made. Two follow-on issues were filed: #136 (yescrypt) and #137 (Argon2id).

## Inputs verified

- **Base:** wolf-std trunk `d4a4d4c`, branch `sc80`.
- **Toolchain:** the release archives match the release API's digests: `d383c74c…` (wolf 0.2.36, Linux x86-64), `34f52792…` (lupin 0.1.58, Linux x86-64), `b79d5584…` and `c1b02771…` (the macOS arm64 pair). The Linux binaries are `f7b73fba…`, `47c646a6…` and `libwolf_rt.a` `47fb9ad0…`, the digests sc79 recorded.
- **Read first:** sc79's note (the last crypto lane's gate); `API-CONVENTIONS.md` §1–§4 and §10–§13; `std/x/README.md`'s crypto rows; the headers and surfaces of `std/x/crypto/sha2` (HMAC), `std.rand`, `std.env`, `std.fs` and `std.io`. std has no OS random surface, so `gen_salt` takes the octets (R-sc80-1). Also read: pax's `notes/px103-set-id-and-sudo.md` §8.
- **P0, the black box, before the prediction:**
  - `mkpasswd -m scrypt -R c` writes N = 2^(c+7), r = 32, p = 1, for c from 6 to 11 (the default is 7); it refuses 5 and 12.
  - Its salt is 22 characters, from 16 random octets.
  - The salt characters themselves are scrypt's salt. The key is 32 octets in crypt's little-endian base 64.

## Prediction

The prediction was committed first, at `800574b` (`docs/sc80-prediction.md`), before any row or `std/` change.

- **P1, the RFC 7914 vectors:** held. Every vector RFC 7914 prints passes: §8, §9, §10, both §11 vectors, all four §12 vectors and §13's derived key. The lanes are as predicted. The building blocks, the c = 1 PBKDF2 and the first scrypt vector are three-lane. The c = 80000 PBKDF2 and vectors 2 and 3 are native only: the checked tier's "step budget exhausted", and lupin's step-budget `unsupported` after about 35 s. Before the modules existed, the compiler lanes read `fail(E0301)` (`docs/sc80-witness/red-3b146b6.log`).
- **P2, the cost: missed, by 10 to 30 times, on the slow side.** I predicted 0.3–1.0 s release on Linux at N = 2^14, and it is 33 ms. Every time figure in P2 was too slow, because I assumed under 1 G word operations a second. The release build (LLVM) runs the Salsa20/8 core at about 524,000 cores in 17 ms, and the default backend at about four times that. Memory came in at the edge: I predicted 16 MiB plus 2–10 MiB, and it is 16 MiB plus 1.7–1.9 MiB, just under the band's floor. The linear claims held: time doubles with N, and peak minus V is constant. N = 2^20 was predicted at 20–90 s release, and it is about 2.2 s per vector with a peak of 1 GiB plus 1.6 MiB.
- **P3, Ubuntu's crypt:** held, with two differences I did not predict.
  - Ubuntu's crypt accepts every string `hash` made, and `verify` accepts every string Ubuntu's crypt and `mkpasswd` made. Both hold at N = 2^14 and 2^15 (r = 8, p = 1) over 20 random passwords and salts per cost in each direction, and at `mkpasswd`'s own costs.
  - A wrong password is false on both sides. The zero-octet difference is as predicted.
  - **Not predicted:** Ubuntu's crypt also takes settings that crypt(5)'s regular expression excludes. It takes a salt longer than 86 characters (up to somewhere between 300 and 399), and an N not below 2^(16·r), against RFC 7914 §2 (N = 2^16 and 2^17 at r = 1). `passwd` refuses both by name (R-sc80-5, R-sc80-6).
- **P4, refusals:** held, every name as predicted. One change to the surface handed back early: `scrypt.memory` takes `dk_len` (R-sc80-4).
- **P5, the plant:** see the evidence below.

## Evidence index

### The modules

- `std/x/crypto/scrypt/scrypt.lu`:
  - The words are `wrapping[u32]`, four octets an element, so V is exactly 128·r·N octets (measured: peak minus V is constant across N).
  - BlockMix writes from a source block into a destination, and ROMix copies a block only to V. The inner loops allocate nothing, every PBKDF2 PRF call runs in its own `region`, and V's region ends before the final PBKDF2.
  - `scrypt` checks `memory(n_log2, r, p, dk_len)` against the caller's cap before it allocates.
- `std/x/crypto/passwd/passwd.lu`: the alphabet, the 30-bit fields, the 3-to-4 octet encoding with its canonical last character, the parser's classification of every crypt(5) form, `hash`, `verify`/`verify_capped` with a constant-time key comparison, and `gen_salt`. The header says what is and is not constant-time.

### Rows

| what | where | lanes |
|---|---|---|
| §8 Salsa20/8, §9 BlockMix, §10 ROMix | `tests/x/crypto/scrypt/rfc7914_core_rows.lu` | all three |
| §11 PBKDF2, c = 1 | `tests/x/crypto/scrypt/rfc7914_pbkdf2_c1.lu` | all three |
| §11 PBKDF2, c = 80000 (also holds the constant PRF footprint) | `tests/x/crypto/scrypt/rfc7914_pbkdf2_c80000.lu` | native |
| §12 vector 1 | `tests/x/crypto/scrypt/rfc7914_scrypt_vector1.lu` | all three |
| §12 vectors 2 and 3 | `tests/x/crypto/scrypt/rfc7914_scrypt_vectors_2_3.lu` | native |
| §12 vector 4 and §13's key (N = 2^20) | `docs/sc80-witness/rfc7914_n20/main.lu` | local record only, `prlimit --as=3221225472` |
| scrypt's refusals, each boundary from both sides | `tests/x/crypto/scrypt/parameter_refusals.lu` | all three |
| a block of the wrong length | `tests/x/crypto/scrypt/salsa_length_trap.lu` | all three, `trap(assert)` |
| the parser: every form refused by name | `tests/x/crypto/passwd/parse_refusals.lu` | all three |
| cost, gen_salt, hash, verify, verify_capped | `tests/x/crypto/passwd/hash_verify_rows.lu` | all three |
| 13 Ubuntu-made strings: verified, wrong password refused, `hash` byte-identical | `tests/x/crypto/passwd/ubuntu_strings.lu` | native |

`ubuntu_strings.lu` is generated by `docs/sc80-witness/interop/ubuntu_fixture.py` in the pinned image. The gauntlet's `ubuntu-fixture` step regenerates it and compares.

### Interop against Ubuntu's crypt

`docs/sc80-witness/interop/run.py` runs in the pinned image. It builds `interop/probe/main.lu` with the default build (the rig's lane), draws every password and salt from `secrets`, and decides 225 rows:

- `mine-*`: 40 strings `hash` made, salts from `gen_salt` over 16 random octets, at N = 2^14 and 2^15. Ubuntu's crypt accepts each one with its password and refuses it with a wrong one. The `gen-salt-*` rows show `gen_salt` encodes as crypt does.
- `ubuntu-*`: 40 strings Ubuntu's crypt made from random settings. `verify` accepts each and refuses it with a wrong password, and `hash` of the same inputs is byte-identical.
- `mkpasswd-*`: `mkpasswd -m scrypt` at its default cost and at `-R 6` (r = 32).
- `perl-*`: Perl's crypt over raw octets, including ones that are not UTF-8, in both directions.
- `salt-len-*`: salts of 0, 1, 43 and 86 characters agree. At 87, `passwd` refuses `bad_salt` and Ubuntu's crypt accepts (named).
- `method-*`: yescrypt, gost-yescrypt, bcrypt (`$2b$`, `$2a$`), sha512crypt, sha256crypt, sunmd5, md5crypt, bsdicrypt, descrypt and NT, each `unsupported` from `parse` and from `verify`.
- `malformed-*`: twelve mutations, each refused by name. For each one the harness records whether Ubuntu's crypt refuses the setting (n-zero, r-zero, p-zero, rp-2^30, salt-hyphen, short). Where it takes the setting, it records that the stored key fails there too (the key mutations and a bare setting). The two settings it takes and `passwd` refuses are named.
- `zero-octet-named-difference`: crypt(password with a zero octet) equals `hash` of the password cut at that octet, and `hash` of the whole password differs.
- `hashlib-*`: 24 random parameter sets (N to 2^12, r to 8, p to 4, dkLen to 128, random P and S) against `hashlib.scrypt`.

### The cost

`docs/sc80-witness/bench/cost.py` builds `bench/main.lu` with the default build and with `--release`. It runs five interleaved rounds per pair, each child from fork/exec with wait4, and takes the minimum wall and the largest peak. On Linux a child's `ru_maxrss` starts at its parent's resident set, so the floor row reads the parent. The program's own footprint is the peak minus V.

| host | build | N = 2^14, r = 8 | N = 2^15, r = 8 | peak at 2^14 / 2^15 | peak − V |
|---|---|---|---|---|---|
| the Mac (Apple M5 Pro, arm64) | release | 44 ms | 85 ms | 17.7 / 33.7 MiB | 1.7 MiB |
| the Mac | default | 149 ms | 294 ms | 17.7 / 33.7 MiB | 1.7 MiB |
| Linux x86-64 (i7-12700KF), one light CPU | release | 33 ms | 66 ms | 17.7 / 33.7 MiB | 1.8 MiB |
| Linux x86-64, one light CPU | default | 115 ms | 230 ms | 17.8 / 33.8 MiB | 1.9 MiB |

The Mac figures are from one take behind the Mac build lock, at `fb6456b`, whose `std/` is the head's (`docs/sc80-witness/cost-mac-fb6456b.log`); the Mac record's `cost-witness` step repeats them. The Linux figures are from a development run on a light slot. The final record's `cost-witness` step repeats them on a whole block, holding its light lock. N = 2^20 is about 2.2 s per vector (release, one CPU), with a peak of 1,050,224 KiB.

### The plant

`sc80-plant-blockmix-order` is pushed without a PR: one commit, one file, one hunk on the final head. scryptBlockMix writes the odd-indexed Salsa outputs to the first half of B' and the even ones to the second. The plant record names `rfc7914-s9-blockmix` and `rfc7914-s10-romix`, each a `FAIL` line in the nonzero `sc80-rows-native` step. The §8 Salsa row and both §11 PBKDF2 rows stay green.

### The gauntlet

`docs/sc80-witness/gauntlet.sh` runs these steps in order:

1. the toolchain;
2. the sc80 rows natively;
3. `ubuntu-fixture`, `ubuntu-interop` and `rfc7914-n20` (under the cap);
4. every `cargo xtask ci` step: `time-oracle`, `fmt`, `clippy`, `test`, `sync-pin`, `doctor`, `ledger-check`, `lint-conventions`, `fmt-lu`, `gen-vectors`, `std-test` (634 files, three lanes), `doc-examples` and `ulp`;
5. the cost witness.

`--mac` runs the toolchain, the rows and the cost witness. The local-gate recorder writes the green, plant and Mac records. The PR body gives their paths and every `END` line.

## Rulings asked (R-list)

- **R-sc80-1: `gen_salt` takes octets**, not a source handle. std has no OS random surface, and PAX reads getrandom. Accepted at the early hand-back.
- **R-sc80-2: the memory cap is a parameter.** It is `scrypt`'s seventh argument, and `passwd` holds stored costs to `default_cap()` (`memory(18, 32, 1, 32)`, the largest Ubuntu's gensalt writes); `verify_capped` takes the caller's own. Accepted.
- **R-sc80-3: `empty` and `locked` are named refusals**, so the pax side can show them. Accepted.
- **R-sc80-4: `scrypt.memory` takes `dk_len`.** This changes the signature handed back early (`memory(n_log2, r, p)`). The key `scrypt` returns is allocated too, and an RFC-legal dk_len up to (2^32 − 1)·32 would otherwise pass any cap and allocate without bound (a row found it). `passwd`'s surface is unchanged. Recommended as built.
- **R-sc80-5: salts over 86 characters.** crypt(5)'s format allows at most 86. Ubuntu's crypt takes up to somewhere between 300 and 399, and `passwd` refuses `bad_salt`. Options: (A) keep the documented bound; (B) accept up to Ubuntu's measured limit. **Recommended: A.** No tool writes them, and gensalt writes 22.
- **R-sc80-6: N not below 2^(16·r).** RFC 7914 §2 excludes it, and Ubuntu's crypt accepts it (N = 2^16 and 2^17 at r = 1); `passwd` and `scrypt` refuse `bad_n`. Options: (A) the RFC's bound; (B) Ubuntu's. **Recommended: A.**
- **R-sc80-7: a zero octet in a password.** crypt(3) stops at it, while `hash` hashes every octet, so such a line made here does not verify under Ubuntu's crypt. Options: (A) as built, a named difference with a row; (B) refuse a password containing a zero octet by name. **Recommended: A.** A terminal read does not produce one.
- **R-sc80-8: the cost PAX picks.** It is px106's to choose and state. The numbers above put N = 2^17 to 2^18 at r = 8 near its 0.5–1 s target on the default build, before KVM's overhead.

## Done-when

- [x] Branch `sc80` from trunk `d4a4d4c`; pins unchanged.
- [x] Prediction committed first (`800574b`), every line scored above.
- [x] `std.x.crypto.scrypt`: `salsa20_8`, `blockmix`, `romix`, `pbkdf2_hmac_sha256`, `scrypt`, `memory`; every refusal by name; every RFC 7914 vector a row, red first (`3b146b6`). Vectors 1–3 run in CI; the N = 2^20 rows run in the local record under the cap.
- [x] `std.x.crypto.passwd`: `hash`, `verify` (constant-time over the key), `gen_salt`, `parse`/`encode`, `unsupported` for every other method, and every malformed form refused by name.
- [x] Interop with Ubuntu's crypt both ways at two costs (20 random passwords and salts each), `mkpasswd`, Perl, and raw scrypt against `hashlib.scrypt`.
- [x] `std/x/README.md`: the two rows and the surface section; the module headers; `README.md` (newest surfaces, 634 tests); `CHANGELOG.md` under Unreleased.
- [x] Follow-ons filed: #136 (yescrypt), #137 (Argon2id).
- [ ] The green record, the plant record and the Mac record at the final head (paths and `END` lines in the PR body).
