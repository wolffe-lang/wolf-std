# sc80 prediction: the password hash (scrypt, in crypt's `$7$` form)

Written before any change under `std/` or `tests/`. Base: trunk `d4a4d4c`. Toolchain unchanged: wolf 0.2.36, lupin 0.1.58 (the pins in `vendor/tools.toml`).

The work: `std.x.crypto.scrypt` (Salsa20/8, scryptBlockMix, scryptROMix and scrypt from RFC 7914; PBKDF2-HMAC-SHA-256 from RFC 8018 §5.2 over `std.x.crypto.sha2`'s HMAC) and `std.x.crypto.passwd`, the crypt(5) `$7$` string layer: `hash`, `verify`, `gen_salt` and a parser.

The sources are RFC 7914, RFC 8018, RFC 2104, FIPS 180-4 and the crypt(5) man page as Ubuntu 24.04 ships it. The man page gives the `$7$` string's regular expression and nothing of its inner encoding. That encoding comes from Ubuntu's crypt run as a black box, before this file was written: `mkpasswd -m scrypt`'s output, decoded under three hypotheses and checked against Python's `hashlib.scrypt`. One hypothesis held exactly. After `$7$`, one character carries log2 N. Five characters carry r, then five carry p, each a 30-bit number in crypt's `./0-9A-Za-z` alphabet with the least significant six bits first. Then come the salt characters, which are themselves scrypt's salt octets (they are not decoded first), then `$`, then 43 characters: the 32-octet derived key in the same alphabet, three octets to four characters, least significant first. `mkpasswd -R c` writes N = 2^(c+7), r = 32, p = 1 for c from 6 to 11, refuses 5 and 12, and draws a 22-character salt (16 random octets).

## P1. The RFC 7914 vectors

Every vector RFC 7914 prints is a row, each committed red before the module exists. Before the modules exist, they read `fail(E0301)` on the compiler lanes.

| row | RFC 7914 | parameters | predicted | lanes |
|---|---|---|---|---|
| salsa20/8 core | §8 | one 64-octet block | passes | all three |
| scryptBlockMix | §9 | r = 1 | passes | all three |
| scryptROMix | §10 | r = 1, N = 16 | passes | all three |
| PBKDF2-HMAC-SHA-256 | §11, first | "passwd", "salt", c = 1, 64 octets | passes | all three |
| PBKDF2-HMAC-SHA-256 | §11, second | "Password", "NaCl", c = 80000, 64 octets | passes | native only (step budgets) |
| scrypt vector 1 | §12 | "", "", N = 16, r = 1, p = 1 | passes | all three |
| scrypt vector 2 | §12 | "password", "NaCl", N = 1024, r = 8, p = 16 | passes | native only |
| scrypt vector 3 | §12 | "pleaseletmein", "SodiumChloride", N = 16384, r = 8, p = 1 | passes | native only |
| scrypt vector 4 | §12 | as vector 3 with N = 1048576 | passes | local record only, under a hard memory cap |
| the PKCS#8 derived key | §13 | "Rabbit", "Mouse", N = 1048576, r = 8, p = 1, 32 octets | passes | local record only, under the cap |

Vectors 1 to 3 and §8 to §11 run in CI's rig. The two N = 2^20 rows run only in the local record, under `prlimit --as` at 3 GiB.

## P2. The cost

The cost is measured on the native release build (`wolf run --release`) and on the default build the rig uses. Each figure is the minimum of five interleaved runs. Peak memory is the child's `ru_maxrss`, measured from a small parent, with the floor of an empty program recorded beside it.

The arithmetic: ROMix runs 2N BlockMix calls, and each call runs 2r Salsa20/8 cores. At N = 2^14 and r = 8 that is 524,288 cores of about 400 32-bit add, rotate and xor operations each, plus the copies. That comes to about 250 million word operations. V holds 128·r·N octets: 16 MiB at N = 2^14 and 32 MiB at N = 2^15. The words are stored as 32-bit `wrapping[u32]` elements, predicted at four octets each.

| host | N, r, p | release, predicted | default build, predicted | peak memory, predicted |
|---|---|---|---|---|
| Linux x86-64 (i7-12700KF, one six-CPU block) | 2^14, 8, 1 | 0.3–1.0 s | 0.7–2.5 s | 16 MiB + 2–10 MiB |
| Linux x86-64 | 2^15, 8, 1 | 0.6–2.0 s | 1.4–5 s | 32 MiB + 2–10 MiB |
| the Mac (Apple M5 Pro, arm64) | 2^14, 8, 1 | 0.2–0.8 s | 0.5–2 s | 16 MiB + 2–10 MiB |
| the Mac | 2^15, 8, 1 | 0.4–1.6 s | 1–4 s | 32 MiB + 2–10 MiB |

Time is linear in N, so N = 2^15 costs twice N = 2^14. Memory is 128·r·N plus a floor that does not grow with N. The inner loops allocate nothing. If they did, the ambient region would hold every BlockMix's garbage until scrypt returned, and the peak would grow with N·r beyond V. The N = 2^20 vector is predicted at 20–90 s release on Linux, with a peak of 1 GiB plus the same floor.

## P3. What Ubuntu's crypt accepts from this module

Ubuntu's crypt (libxcrypt 4.4.36 in a pinned Ubuntu 24.04 container, through Python 3.12's `crypt` module and Perl's `crypt`) accepts every string `passwd.hash` makes and verifies it, with the same password, as `crypt(password, stored) == stored`. This holds at both costs the rows use, N = 2^14 and 2^15 with r = 8 and p = 1, over at least 20 random passwords and 20 random salts per cost, each from `gen_salt`'s 16 octets. In the other direction, `passwd.verify` accepts every `$7$` string Ubuntu's crypt makes at those costs from those salts, and every string `mkpasswd -m scrypt` makes at its default cost (N = 2^14, r = 32). A wrong password is false on both sides.

One difference is predicted and will be named. crypt reads the password as a C string, so a password containing a zero octet is cut there, while `passwd.hash` hashes every octet. No row's password contains one. A row will show the difference on purpose.

## P4. Refusals by name

- `scrypt.scrypt` refuses: `bad_n` (log2 N below 1, or N not below 2^(16·r), RFC 7914 §2), `bad_rp` (r or p below 1, or r·p at least 2^30), `over_cap` (the working memory over the caller's cap), `bad_dk_len` (below 1, or above (2^32 − 1)·32). `scrypt.romix` refuses an N that is not a power of two above 1 as `bad_n`.
- `passwd.parse` and `passwd.verify` refuse: `empty` (an empty field), `locked` (a field starting `!` or `*`, shadow(5)'s marks), `unsupported` (another crypt(5) method: `$y$`, `$gy$`, `$2a$`, `$2b$`, `$2x$`, `$2y$`, `$6$`, `$5$`, `$sha1$`, `$md5`, `$1$`, `$3$`, `_` and the 13-character DES form), `malformed` (any other shape, including a `$7$` string with a missing or extra `$`), `bad_n`, `bad_rp`, `bad_salt` (a character outside the alphabet, or more than 86), `bad_hash` (not 43 characters of the alphabet, or a last character carrying bits above the 256th) and `over_cap`.
- `passwd.gen_salt` refuses `bad_random`: fewer than 16 octets, more than 64, or all zero (an unfilled buffer).
- Every refusal is a row, and every row with a password makes that password at run time.

## P5. The plant

One commit, one file, one hunk on the final head: scryptBlockMix writes the odd-indexed Salsa outputs to the first half of B' and the even-indexed ones to the second. The §9 BlockMix row goes red by name, and so do the §10 ROMix row and every §12 row. The §8 Salsa row and the §11 PBKDF2 rows stay green, because they never reach BlockMix.
