# sc84 — wolf-std: the crypto never divides or branches on a secret

wolf-std#151. An independent review of a downstream age codec read the
objects wolf 0.2.39 generates for this library's X25519 and Bech32 code
and found a division on the secret scalar in the x86-64 release ladder,
divisions on identity bits in release Bech32 decoding on both targets,
an alphabet scan that leaves early on each secret character, and more
divisions in default builds than the dw03 audit had stated. This note
holds the census, the prediction, the decisions and the evidence for the
change that removes them.

## Forbidden

No new cryptography: the change rewrites how existing arithmetic is
spelled and never what it computes. Inputs are RFC 7748, RFC 8439,
RFC 5869, FIPS 180-4, BIP-173 and the age specification, this library's
own sources and tests, and the published wolf language specification
(`[type.int.shift]`, `[type.byte.op]`, `[gram.expr.prec]`). No other
implementation's source, no compiler or runtime source and no libc
source was read. No compiler, runtime or toolchain pin changes. Nothing
here is a timing measurement.

## Inputs verified

- Base: trunk `bb002f3ed4f4cf87f62e50da3391cb224dc1ebc1`.
- wolf 0.2.39 (`3fc86f77`) and lupin 0.1.61 (`5f1f1c94`), the pins in
  `vendor/tools.toml`. Archive SHA-256, each read twice before use:
  macOS arm64 wolf `63de9476…cfde0`, lupin `72364246…25ed5af`; Linux
  x86-64 wolf `53f75d2b…f0457`, lupin `61ce849e…ef41b` (the full digests
  are in `docs/dw03-std-for-age.txt`; they match).
- Disassemblers: GNU objdump 2.42 (x86-64 ELF) and Apple LLVM objdump /
  otool 21 (arm64 Mach-O). The release tier compiles through clang 18.1.3
  on Linux and the Apple toolchain on macOS.
- A probe built at the pin (all four machines: lupin, the checked
  machine, native default and release) confirmed what the rewrite leans
  on: `x >> 26` and `x & 67108863` on a plain `int` equal `x / 67108864`
  and `x % 67108864` for `x >= 0`, `-7 >> 1` is `-4` (floor, as
  `[type.int.shift]` says), and a `wrapping[u64]` word shifted by a
  variable count extracts the same bit as the division. The comments
  that said a plain-`int` shift or mask "costs the checked lane"
  (F-0026, sc16) predate wolf 0.2.31 and are stale at this pin.

## Prediction

Written before the first change, from the census below.

### The census at bb002f3

**Method.** `docs/sc84-witness/census/entry.lu` calls every public
function of `std.x.crypto.curve25519`, `std.x.crypto.chacha20` (with
Poly1305 and the AEAD), `std.x.crypto.sha2` (SHA-256/384/512, HMAC,
HKDF) and `std.bech32`, with every input taken from `argv`, so no call
can be folded away. `capture.sh` builds its default and release objects
with the pinned compiler (`--emit=obj --no-cache`) and disassembles each;
`scan.py` lists every integer division instruction (`div`/`idiv` in any
width on x86-64, `sdiv`/`udiv` on arm64) by function and offset, and
counts conditional branches and selects per function. The default tier
emits every function of each imported module; the release tier emits
what the entry reaches, after inlining. Object digests are in
`docs/sc84-witness/census/`.

**Release divisions** (each is a full hardware divide on data):

| target | function | offset | instruction | source | data |
|---|---|---|---|---|---|
| x86-64 | `curve25519.ladder` | `+0x23d` | `div %sil` | 846-847 `kb / pow2(t % 8)` | the clamped secret scalar byte, 255 times per scalar multiplication |
| x86-64 | `curve25519.ge_scalarmult` | `+0x2879` | `div %sil` | 1226-1227, the same bit extraction | the secret scalar and nonce when signing; public in verify |
| x86-64 | `bech32.decode_bytes` | `+0x943`, `+0x950` | `idiv %rcx`, `div %ecx` | 196-198 `acc / factor` | identity bits when decoding an age identity |
| x86-64 | `bech32.encode_bytes` | `+0x722`, `+0x735` | `idiv %rcx`, `div %ecx` | 164-166 `acc / factor` | identity bits when encoding one |
| x86-64 | `bech32.encode` | `+0x53d` | `div %rcx` | 96 `sum / power(…)` | the checksum, a function of the data |
| arm64 | `bech32.decode_bytes` | `+0x8f8` | `sdiv x28, x21, x27` | 196-198 | as above |
| arm64 | `bech32.encode_bytes` | `+0x5d0` | `sdiv x27, x24, x19` | 164-166 | as above |

The arm64 ladder, `ge_scalarmult` and `encode` use shifts. On x86-64 the
two Bech32 conversions pick a 64-bit or a 32-bit divide by a branch on
the operands' width, a second data-dependent edge. No release division
appears in chacha20, Poly1305, sha2, HMAC or HKDF on either target.

**Default divisions**, identical counts on both targets (`idiv` on x86-64,
`sdiv` on arm64), 276 in all (275 in the library):

- Secret operands: `curve25519.fe_tobytes` 62 (it serialises the shared
  secret), `fe_carry` 40, `fe_freeze` 38, `ladder` 2 of 4, `clamp` 2;
  `chacha20.poly1305` 50, `absorb` 19, `push_tag_word` 7 (the one-time
  key and the accumulator); `bech32.decode_bytes` 2, `encode_bytes` 2,
  `encode` 2; Ed25519 signing's `ge_scalarmult` 2 of 4 and `floordiv` 1
  (the signed carries of the secret scalar reduction).
- Public operands: `fe_frombytes` 18 (a peer's u-coordinate or a point
  being decompressed), `ge_decompress` 4, `ge_compress` 1 (the sign bit of
  a published point), `fe_sample` 1 (the self-test helper), `ladder` and
  `ge_scalarmult` 2 each (`t / 8`, `t % 8`), `chacha20.stream_xor` 3,
  `mac_data` 2, `push_len64_le` 2 (lengths and positions), `bech32.hrp_state`
  2 (the HRP), `sha2.hkdf384_expand`, `hkdf512_expand`,
  `counter_absorb256` 1 each (the block counter).
- Either, by caller: `to_hex` 2 in each of curve25519, chacha20 and sha2
  (documented as not constant-time).

**Branches and early exits.** Every conditional in the four modules was
read at source; each one whose direction can depend on secret data is
listed, with its machine form at this pin. The rest decide on lengths,
positions, loop counters, the HRP or public inputs, or are verdicts whose
direction is the same for every valid input (a malformed-input refusal,
a mixed-case refusal, a checksum or padding verdict, the X25519 zero
verdict, the AEAD tag verdict), so they disclose only the published
outcome.

| source | what it decides | default | release x86-64 | release arm64 |
|---|---|---|---|---|
| `bech32.value` 51-58 | the alphabet scan returns at the secret character's index | loop, exits early | an indirect jump through a table indexed by the character (`decode +0x38f`, `jmp *%rcx`) | a compare loop that exits early |
| `bech32.lower` 26-29 and `decode`'s case flags 121-122 | letter or digit, per data character | branches | `cmov` | `csel` |
| `bech32.step` 35-39 | five generator XORs on bits of the checksum state | five branches per character | `cmove` | `csel` |
| `bech32.encode` 91, 96 | `chars[v]`: a load indexed by the secret word (a memory index, not a branch) | load | load | load |
| `curve25519.ge_scalarmult` 1228 | the add is gated on each scalar bit (Ed25519 signing; documented in the module as signing's obligation) | branch | branch | branch |
| `curve25519.floordiv` 1033 | the sign correction of a signed carry from the secret scalar limbs (signing) | branch | `cmovns` | select |
| `to_hex` (three modules) | a slice indexed by each nibble | load | load | load |

The compilers add branches of their own: checked-arithmetic overflow
guards on every `+`, `-` and `*`, bounds checks, and in default builds
the divide-by-zero and overflow guards of each `/` and `%`. Their operands
can be secret; within the bound proofs their direction is the same for
every valid input, and a taken edge is a trap. They were counted per
function (`scan.py` tables), not read one by one.

**What the review listed, and what it missed.** It listed the x86-64
release ladder division, release `decode_bytes` on both targets, the
early-exit alphabet scan, and default divisions in `fe_carry`,
`fe_freeze`, `fe_tobytes`, `clamp`, `ladder`, `absorb`, `poly1305` and
`push_tag_word`. It did not list: the release divisions in
`encode_bytes` (both targets) and `encode` (x86-64), which divide an
identity when one is generated; the x86-64 release division in
`ge_scalarmult` (it did not read Ed25519); the x86-64 jump table that
the alphabet scan becomes; the secret letter-or-digit branches and the
checksum-step branches of default builds; the default divisions in
`bech32` and in Ed25519 signing's `floordiv`; the width branch in front
of the x86-64 Bech32 divides; and the secret-indexed alphabet load in
`encode`. Its counts for the eight functions it listed match mine
exactly.

### What each change should show

1. **The gate, red first.** A new `cargo xtask codegen` step builds the
   entry's default and release objects on the host target, disassembles
   them and fails by name on any integer division, naming the tier, the
   function and the offset. Committed before the fix, it is red at
   `bb002f3` on x86-64 Linux with the seven release lines above and 275
   default lines, and on arm64 macOS with two release lines and 275
   default lines (the gate's own entry divides nowhere). A self-check builds a probe that must divide; a scan
   that cannot see that division fails as blind rather than passing.
2. **The fix.** The ladder and `ge_scalarmult` take each scalar bit as
   `(byte >> position) & 1` on a `wrapping[u64]` word, with the byte and
   position as `t >> 3` and `t & 7`. Every other power-of-two `/` and `%`
   in the four modules becomes a literal shift or a mask on a value the
   bound proofs keep non-negative, where both spellings give the same
   answer (`floordiv`'s calls become `>> 8` and `>> 4`, which are floor
   division by `[type.int.shift]`, so its sign-correction branch goes
   too). The one non-power-of-two modulus, `fe_sample`'s `% 251` on a
   value below 502, becomes a single subtraction. Bech32 decoding reads
   the alphabet in a constant 32-step loop with an arithmetic match, folds
   case and the checksum generator arithmetically, and converts 5-bit
   words to bytes by shifts; encoding selects each character the same way
   instead of indexing the alphabet. Expected: the gate is green on both
   targets in both tiers; every existing vector and row keeps its answer
   on every lane; the ledger does not move.
3. **Rows.** A new Bech32 row decodes every alphabet character in both
   cases at a data position, refuses every other printable character,
   and round-trips every 5-bit word and every byte length from 0 to 51
   (all three lanes). The gate also builds and runs, in the release tier,
   the vector rows the change touches, the RFC 7748 1000-iteration chain
   among them, since no ledger lane is a release build.
4. **The claim.** The dw03 note's sentence about release counterparts is
   corrected in place, and each module header states what it now
   promises at source level and in generated code, and what it does not.
5. **The plant.** One hunk on the final head puts a division back in the
   ladder's bit extraction; the gate goes red naming
   `std.x.crypto.curve25519.ladder`.

## Decisions

Each was weighed and the recommended option implemented.

1. **Where the gate lives.** (a) A `cargo xtask codegen` step in the local
   gate and in `ci.yml`'s `gates` job, so the drift check that pairs the
   two lists still holds; (b) a `cargo test` that shells out to the
   compiler; (c) a lane-only witness script. Recommended (a): it is the
   repository's gate, it runs where the toolchain is already acquired,
   and it is red in the same place a reader looks. It touches CI, so the
   change is cross-cutting.
2. **Which tiers.** Release only (the minimum asked) or release and
   default. Recommended both: once the source no longer divides, default
   builds are division-free too, and the gate keeps them so.
3. **Which functions.** Every function in the objects built from the four
   modules, with no allowlist, or only the secret paths. Recommended
   every function: the public divisions are all by powers of two (one
   test helper aside), rewriting them costs nothing, and a gate without
   an allowlist cannot be widened quietly.
4. **Bech32's decoder.** A separate secret-safe entry beside a public one
   that keeps its early exit, or the one decoder changed in place.
   Recommended in place: no caller needs the early exit, strings are at
   most 90 characters, and with one decoder no caller can pick the wrong
   one.
5. **Bech32's encoder.** Generating an age identity encodes the secret,
   so `encode` and `encode_bytes` get the same treatment: shifts for the
   conversion and the checksum characters (the gate needs that), and a
   constant-trip select instead of the secret-indexed alphabet load.
6. **The ladder's bit.** A shift by the public bit position on a
   `wrapping[u64]` word, or an eight-way select of literal shifts.
   Recommended the shift: one instruction with no count check and no
   branch, run on all four machines at this pin (the probe; `std.rand`
   already shifts a wrapping word by a variable count).
7. **Ed25519 signing.** `ge_scalarmult`'s bit extraction is shared with
   the ladder and changes with it; `floordiv`'s calls become floor
   shifts. The gated add stays: making signing's scalar multiplication
   fixed-sequence changes its algorithm, not its spelling, and is a
   follow-on.
8. **Windows.** The gate reports a loud SKIP on Windows hosts: their
   objects are COFF and no disassembler for them is acquired or tested
   here. The Linux and macOS legs carry the gate.
9. **The disassembler.** The host's `objdump` (GNU on Linux, LLVM on
   macOS), not pinned; the gate prints its version line, and its
   self-check fails if the probe's division is not seen.

## Results against the prediction

- **Red first, as predicted.** The gate (`41a02f1`) and its Linux rig
  (`da27890`) were committed on the base library. At `da27890` the gate
  was red on x86-64 Linux with exactly the seven release divisions of the
  census and 275 default ones. The first arm64 run, at the same commit,
  was red with the two predicted release divisions and 275 default ones,
  and it showed a fault in the gate: LLVM's objdump labels the start of a
  Mach-O section with an assembler temporary (`ltmp0`) and hides the
  function that begins there, so a division in that function would have
  been named `ltmp0`. `2a7ca45` takes names from the object's symbol
  table and reports each offset from its function's start. Both targets
  were taken red again at `2a7ca45`: x86-64 `ladder +0x23d div %sil`,
  `ge_scalarmult +0x539 div %sil`, `decode_bytes +0x143 idiv %rcx` and
  `+0x150 div %ecx`, `encode_bytes +0x122 idiv %rcx` and `+0x135 div
  %ecx`, `encode +0x53d div %rcx`; arm64 `decode_bytes +0x138 sdiv`,
  `encode_bytes +0x118 sdiv`; 275 default lines on each.
- **Green after the fix, both targets, both tiers.** From `4c3f9d9` on,
  the gate finds no integer division in any function: on x86-64, 68
  release functions (21,083 instructions) and 156 default functions; on
  arm64, 465 release labels (cold splits and outlined code counted
  separately; 21,095 instructions) and 156 default functions. The fifteen
  release rows, the RFC 7748 1000-iteration chain among them, pass on
  both. The self-check saw the probe's division every time.
- **The branches went with them.** Read in the fixed objects: the x86-64
  release `decode` has no jump table and no data-dependent branch; its
  23 conditional jumps are loop counters, list-capacity checks, overflow
  guards and the four verdicts. The alphabet pass is 32 compares and
  multiplies with no exit, and the checksum step's generator terms are
  shift-and-mask selections. arm64 release `decode` has the same shape.
- **Answers unchanged.** Every row of `bech32`, `x/crypto/curve25519`,
  `x/crypto/chacha20`, `x/crypto/sha2` and `bytes` was run on lupin, the
  checked machine and native at the fixed code: 333 runs, each verdict the
  ledger's (12 more are compile-time rows the whole gate covers; one
  `slow` lupin posture answered as before). The ledger moves only by the
  new row, `run` on all three lanes.
- **Not predicted:** `wolf fmt` drops parentheses the precedence table
  makes redundant, so several rewritten lines read as `x + y & m` and
  `next ^ 0 - (top & 1) & k`; they are the canonical form and mean what
  the parenthesised spelling meant.

## Claim and limits

What the four modules now claim, at wolf 0.2.39:

- The default and release objects generated for
  `std.x.crypto.curve25519`, `std.x.crypto.chacha20`,
  `std.x.crypto.sha2` and `std.bech32` contain no integer division
  instruction on x86-64 Linux or arm64 macOS, and `cargo xtask codegen`
  fails if one appears on the host target it runs on.
- At source level, X25519's ladder reads the secret scalar without a
  division or a branch, and Bech32 reads and writes data characters
  without a division, a branch, an early exit or an index on their
  value; the remaining branches are on lengths, the HRP, the separator's
  position and the published verdicts.

What they do not claim:

- No timing was measured, and nothing is said about the latency of the
  instructions that remain (multiplies, shifts) on any CPU.
- The compiler is not audited. The gate re-reads the objects of the
  pinned compiler on each run; another compiler version is covered only
  when the gate runs with it.
- Only the host target is scanned on a run: the Linux and macOS legs
  between them cover x86-64 and arm64; Windows objects (COFF) are not
  scanned, and no other target is built.
- The gate does not gate branches; the census read them at this pin.
  Checked-arithmetic overflow guards and bounds checks remain, with
  secret operands and a direction that is the same for every valid input.
- Ed25519 signing still adds on each secret scalar bit behind a branch,
  and every `to_hex` indexes by value; both are documented in place.
- The interpreters (lupin and the checked machine) execute no generated
  code and are not covered.
- Release objects hold what the gate's entry reaches; the entry calls
  every public function of the four modules, and the default tier, which
  emits every function, is scanned too.

## Evidence index

Records stay with the lane; each is cited by its SHA-256 and quoted where
it matters.

- Census at `bb002f3`: `docs/sc84-witness/census/` (the entry, the
  capture and scan scripts, the division tables of both targets).
- Gate red, x86-64, at `da27890`: record SHA-256
  `e8cb77456a84c06bd69347cc37d0baf4ceaa314c16474fb5efe947c23c74fd88`,
  `END codegen exit=1`, 282 `FAIL` lines (7 release, 275 default).
- Gate red, x86-64, at `2a7ca45`: record SHA-256
  `bcf9e9e2c471b5e4bf5eac329e8f5cb2a0de6cbea958a4d7d19328dbb7e72fdd`,
  `END codegen exit=1`, 282 `FAIL` lines.
- Gate red, arm64, at `da27890` and `2a7ca45` (gate logs, the recorder's
  Mac rig did not exist yet): SHA-256
  `3303ba9cd1577029ac5219ec0b813e9b0d03420a455c312991d3d722a2dfeec3` and
  `916c4ab45eded3a948c6230dfe5295a39961d562af2729b895ea07db1c4fe47c`,
  `codegen exit=1`, 277 `FAIL` lines each (2 release, 275 default).
- Gate green, arm64, at `4c3f9d9`: log SHA-256
  `16863173a7286d726f769354a9795824d64243c3420e8a998cdead7875b5f205`.
- The final records (the whole Linux take, the Mac take and the plant)
  are taken at the final head and cited in the pull request, since a
  record cannot name the commit that would carry it.
- The Bech32 known answers: `docs/sc84-witness/bech32_oracle.py`, written
  from BIP-173's text and checked against its published vector
  `abcdef1qpzry9x8gf2tvdw0s3jn54khce6mua7lmqqqxw`.

## Done-when

The PR is open at a head whose local gauntlet record is green and whose
plant record is red by name at the gate; the census, the decisions and
this note's claim match the code; README and CONTRIBUTING name the new
step. Fixes wolf-std#151.
