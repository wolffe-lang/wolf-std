# Changelog

## sc35 — 2026-09-03 — the rename, and the second gate

Binary and DATA pin move together **31170d1 -> 4230b00** (20 commits),
`wolf 0.2.3+dev.4230b00 (wolfgang, pin 4230b00)`, the one-sha invariant
held. lupin stays at **0.1.23**, now **71 commits** behind wolf's own
pin. Anchors **415 -> 417** (+2/-0, `os.net` and `os.net.unix` — #227's,
key sets diffed both ways with nothing dropped); corpus 499 -> 511.

**This is the first bump in this repository's history whose drift
prediction was not a zero, and the non-zero was the point.** s136
(wolf-lang#231) moves the eight byte builtins — `str.bytes()`,
`str_from_utf8`, `fs_read_bytes`/`write_bytes`/`read_chunk`/`write_chunk`,
`net_read_bytes`/`write_bytes` — from `List[int]` to `List[byte]`, and
every one of std's sixteen byte-tier functions is a thin wrapper over
one of them, so the pin REFUSES the library that was written against
the old signatures. Predicted 198 sites / 45 files from #231's own
count; measured **149 E0401 sites over 35 files** at the bump, and
**910 sites over 172 files** once the modules were repaired and the
tests could be reached. Both are honest and they answer different
questions; the first is a floor, because a refusal inside a module
aborts the program that imports it before the compiler can see the
test's own sites.

**THE RENAME. 233 signatures across 19 std modules, 166 test files.**
`std.bytes`, `std.fs` and `std.net` are pure renames — not one
executable cast entered them, because `read_bytes` is still
`fs_read_bytes(path)?`, `from_str` is still `s.bytes()` and `to_str` is
still `str_from_utf8(b)?`, exactly as F-0106 said they would be. The
430 `as byte` and 225 `as int` casts the rename spells are concentrated
in the modules that do ARITHMETIC on octets (`p256` 176, `curve25519`
72, `x.tls.client` 49), where they are `[type.byte.op]` making an
operator's result type explicit rather than any conversion of
representation; `x.tls.cert` — a DER parser, 15 signatures — took none
at all. **Five private `require_byte` ingestion guards are deleted**,
four of them the module's ONE recorded constant-time exception carrying
the words "it leaves with F-0035's real byte type" in its own header:
the digest, cipher and ladder paths now have no value-dependent branch
at all, a constant-time improvement the type paid for.

**THE HEADLINE: 16.0x -> 1.00x, on both tiers, at the io sites.**
Measured through `std.fs` rather than synthetically — write a payload,
read it back inside a fresh region, read `budget.charged(r)`:

| payload | before (checked / native) | after (checked / native) |
|---|---|---|
| 1,024 | 16,384 / 16,368 | **1,024 / 1,072** |
| 4,096 | 65,536 / 65,520 | **4,096 / 4,144** |
| 16,384 | 262,144 / 262,128 | **16,384 / 16,432** |
| **65,536** | **1,048,576 / 1,048,560** | **65,536 / 65,584** |

Better than the contract's own 2.0x prediction, and the reason is
s136's rather than this library's: a PRODUCER mints at exact capacity
through one memcpy, so it pays no growth history — the native residue
is exactly `payload + 48`, one list header, constant from 1 KiB to
64 KiB. A list the program GROWS by `push` still costs `2 x payload +
48` natively and the payload exactly under `--checked`, so
`std.mem.budget`'s 16x caveat retires to one sentence: **read a buffer
with a producer and you pay the payload; build one by pushing and you
pay it twice on the tier that ships.**

**F-0104 and F-0106 CLOSE.** Every prediction F-0106 made about the
substitution-after-the-builtins-move holds, including the ones it made
about call-site shape. **F-0107 CLOSES too**: a consumed
`for b in s.bytes()` walk over 65,536 bytes now charges **0** on all
three lanes, where the checked machine charged 1,048,576 (wolf-lang#232
paid).

**F-0108 — AND THE RENAME CANNOT MERGE YET.** `lupin 0.1.23` refuses
`as byte` with `fail(E0301)` at `resolve`, and a byte tier that never
narrows is not a byte tier. Measured over the renamed tree: **181 of
376 rows** answer `fail(E0301)` under lupin (134 ledgered `run`, 47
ledgered `unsupported`), plus **87 of the tree's 414 doc-example blocks**. Three
independent gates in this repository refuse to record that and all
three are right: the ledger has no lupin word for a static rejection
(and `divergent(…)`'s vocabulary is one-directional — a word for "the
compilers reject and the interpreter runs", none for the mirror);
`doc-examples` rules that a static rejection on the reference machine
is a doc bug and its waiver list waives an `unsupported` verdict, not a
refusal; and §9/§12's three-lane parity is what both exist to enforce.
**Nothing is bent.** The lupin ledger column is left alone with the
count, the cause and the module breakdown in capitals at the top of the
file, and the branch is gated on **lupin 0.1.25** (is36's deliverable,
not tagged when this was written). sc34 refused this change because the
producers were missing; sc35 makes it and finds the second gate.

**Ten witnesses moved with their contract**, the sc28 precedent
verbatim: seven `non_byte_trap.lu` files, `hex/encode_non_byte_trap.lu`
and the two `invalid` row witnesses pinned a RUNTIME consequence of the
0..255 element contract that the TYPE now holds, so each keeps its
program, moves its directive to `fail(E0401)` at `typecheck`, and drops
the part of its name that promised the old outcome
(`…_trap.lu` -> `…_refused.lu`). `invalid` stays DECLARED on both byte
writes — #231's own posture, adopted verbatim: the vocabulary is
stable, an FFI caller's wrong-width list still earns it, typed code can
no longer reach it.

**F-0103 RE-CHARACTERISED, and four sprints of "minimal shape" were
probe artifacts.** Two programs differing only in a `while` loop's
bound — neither passing a literal — split: the one whose row is
actually TAKEN is `unsupported — control flow in an argument` at `mem`,
the other runs. The callee consuming the row (sc34's sharpening), the
genericity and the module boundary are all irrelevant; sc34's own
control probe refuses once its argument raises. So the checked lane's
refusal is not a shape but a PATH, which makes `unsupported` a property
of an execution rather than of a program — a corpus that never takes
the row reports the lane green for code it cannot run. Posted to
wolf-lang#201.

**One upstream crash found and filed:** a compile diagnostic on a very
long source line panics the wolf driver in its human renderer
(`wolf_diag::render::render_line`, `str::repeat` capacity overflow), so
the process dies with exit 101 and emits no record. It cost this sprint
a silently-skipped row in two scans, because a panic prints no
`error[` line for a scanner to find.

Residues, re-probed at `4230b00`: the chars-pairs tuple list is refused
at its **ninth** consecutive pin; F-0096 (`s.get(0..^2)`) verbatim;
`in(r)` unmoved (with a correction to sc34's reading of which arm
answers); `reserve(n)` unmoved but now PRICED at exactly 65,536 ledger
units on a 64 KiB native buffer; a `str` still charges no named
region's ledger on any tier.

## sc34 — 2026-09-02 — the byte tier is bytes, and it cannot be yet

The wolf binary advances **51 commits** — the largest span this repo has
crossed in one bump — to a dev-stamped trunk build,
`wolf 0.2.3+dev.31170d1 (wolfgang, pin 31170d1)`, and the DATA pin comes
back to meet it at the same sha, restoring the one-sha invariant sc33
suspended. The version stays **0.2.3** (r07 moves the tag) and the
binary is a dev build rather than the v0.2.3 tag for a measured reason:
**the tag cannot compile a byte.** `v0.2.3` = `3befc3e` sits twelve
commits before s135, and its WIR lowering refuses `Prim::Byte` outright
— confirmed on the installed tag build before it was replaced. lupin
stays at **0.1.23**, whose conformance pin is now 51 commits behind
wolf's own. Drift was predicted ZERO and measured ZERO over **376x3**,
the fourth consecutive empty drift list and the first defended against a
compiler that actually moved: anchors **411 -> 415** (+4/-0, exactly the
`[type.byte]` family, key sets diffed both ways), corpus 490 -> 499.
Every `unsupported` record in the tree gained wolf-lang#219's
`x-unsupported-construct`/`x-unsupported-span` keys — 124 records changed
shape and none could move a row, because this rig's record parser reads
a closed key list.

**D72's `byte` is in the language, this library measured what it is
worth, and it did not substitute.** A 64 KiB buffer as `List[byte]`
charges **131,120 native and 65,536 checked** where the same buffer as
`List[int]` charges 1,048,560 and 1,048,576: **16.0x -> 2.0x native,
16.0x -> 1.0x checked**, linear at every size from 1 KiB, with native's
residue exactly `2 x payload + 48` (one list header) — the push-growth
history that is #203's separable second half. **F-0104 closes** with
that after-table. What does not close is the library's ability to spend
it: **s135 gave the language a byte type and no byte-typed builtin.**
`s.bytes()`, `str_from_utf8` and all six `fs`/`net` byte builtins are
still declared over `List[int]`, and every one of std's sixteen
byte-tier functions is a thin wrapper over one of them — so a
substituted signature would have to convert elementwise against a
builtin, and with a cumulative ledger the intermediate stays charged: a
substituted `fs.read_bytes` measures **17.0x checked / 18.0x native**,
worse than the 16.0x it replaces, at every size, at exactly the io sites
the ask was filed about. Nothing is worked around; the sixteen
signatures keep their form so the change stays a rename. Filed as
**wolf-lang#231** — move the eight builtin signatures and the
substitution is the rename it was designed to be.

**Two findings and two closures.** **F-0106** is the producers gap
above. **F-0107** (wolf-lang#232): the checked machine charges
**1,048,576** — 16x the payload — for a CONSUMED `s.bytes()` walk that
allocates nothing, where native and lupin both charge 0; it is why the
one substitution with a real native win (`bytes.from_str` as a walk,
131,120 natively) regresses to 1,114,112 under `--checked`, and it makes
a region cap mis-fire between tiers on the idiom `std.bytes` teaches for
byte walking. **F-0105 closes**: D71/#220 landed in this span and its
exact reproducer now reads `[83,84]` on all three lanes, the zero-width
wolfc span gone. **F-0103 is re-probed against a moving compiler for the
first time and is unmoved** — and the probe got sharper: the checked
tier's `control flow in an argument` needs the row to be CONSUMED in the
callee, not merely passed, so a callee that ignores its row-typed
parameter runs on every lane and reports a false heal.

Residues re-probed at the new pin: the chars-pairs tuple list refuses at
its **eighth** consecutive pin, F-0096 refuses verbatim, `List[int].in(r)`
is the sc33 string unchanged, and a `str` still charges no named
region's ledger on any tier.

## sc33 — 2026-09-02 — the bytes get a width

lupin advances to **v0.1.23** at conformance pin 8cda3aa (is34, THE
LETTERS IN THE MIRROR), and its pin CATCHES UP to wolf's own — sc32's
35-commit gap, the largest this repo had recorded, closes to **zero**.
The wolf binary does NOT move (r06 takes it to v0.2.3), so for the
first time the three pins come apart on purpose: the **DATA pin**
advances to wolf-lang trunk **813153e**, 19 commits ahead of both
binaries, suspending the one-sha invariant deliberately. That costs
nothing and the reason is structural rather than lucky — doctor never
reads `vendor/upstream/PIN` (it gates the binary's self-declared
version and pin against `vendor/tools.toml`) and `sync-pin` gates the
snapshot against the SUBMODULE — predicted from the gates' source
before the run and confirmed by a green, silent doctor. The 0.1.22
doctor pin retires. Drift was predicted ZERO and measured ZERO over
**376x3**, the third consecutive empty drift list, with anchors **411
unmoved** and `anchors.json` byte-identical across the span (the
re-vendor moved no bytes; F-0100's both-ways key-set diff was a
formality this time, and said so out loud).

**The drift prediction's real content was a number and a mechanism that
disagree.** wolf-interp#55 puts trap-path stdout in lupin's records, so
the contract asked which std rows move. **Eleven rows trap after
printing** — grepped over all 56 trap files, read to confirm the print
PRECEDES the trapping call, and checked against the ledger to confirm
the lane runs them (three more contain a `print(` that sits *after* the
trap and never executes). All eleven records changed shape; **zero rows
moved**, because this rig never looks at a trap's stdout in three
independent places: `classify`'s Trap arm discards the field by
PATTERN, `diff_class` compares `stdout_sha256` only under
`Verdict::Exit(_)`, and lint R3 bars `stdout=` beside a trap
expectation outright. Measured on BOTH sides of the bump: at 0.1.22
lupin reported null where both wolf lanes already carried
`5726e3cf…`; at 0.1.23 it joins with the byte-identical digest. The
asymmetry was always lupin's alone and invisible only because the
comparator does not look. #209's root-defer divergence HEALS with the
same zero effect (one executable `defer` in the tree, on no trapping
path), and #56 is diagnostic wording, outside D22's protocol.

**wolf-lang#203's ask is filed as a spec-shaped proposal**
(`#issuecomment-5509341730`), with the evidence and no std wrapper —
sc32 measured that one changes no allocation. The io readers are
measured for the first time and reproduce the synthetic 16x to the
byte, with one new and sharp result: **`fs.read_chunk(f, n)` charges
exactly what the unbounded `read_bytes` charges**, so #203's
preallocation property is not partly taken on the one surface that
already knows its bound — it is *entirely* untaken, a 2x sitting
unclaimed behind no new type at all. The proposal's spine is that
**there is no width story to extend**: the spec has no type inventory,
no `[type.int]`, no width vocabulary, no literal suffixes, and `int`
carries no defining clause anywhere. Both cheap answers fail by the
same mechanism — the std wrapper by measurement, and the spec's own
`distinct` newtype by its own clause ("same layout as the base") — so
the recommendation is `[type.byte]` modelled on `[type.char]`, the
spec's ONE existing scalar-width clause, finishing the job s121 started
when it wrote "`char` is the scalar tier, never a byte". A stale count
in F-0104 is corrected in passing: `std.bytes` has TEN public
functions, not nine, all still monomorphic over `List[int]`.

**F-0105 filed**: wolfc's zero-width parse span (DIV-2026-020, ruled as
D71) is reachable from ordinary std-side code, not just the eight
upstream `grammar/` files — turned up by accident re-probing the
`strbuf` placement residue, and confirming from a second independent
rig why nothing measured it (this runner compares codes, never spans).
Recorded on #220.

F-0103 re-measured verbatim and NOT adopted — #201 still has not ruled
— with the honest caveat that this bump's "unmoved" is cheap, because
the wolf binary did not move and the probe could not have. The checked
tier's `breach_is_a_row` flip is **DEFERRED**: s134's item 1 has not
merged (trunk unmoved at 813153e and #219 still OPEN at both
gauntlets), so the row keeps its two-lane reason, with a lead left for
s134 on the `wolf run --checked` half of its bisection. Residues
re-dated: the chars-pairs tuple list refuses at its **seventh**
consecutive pin, F-0096 verbatim, and the `str`-charges-no-region
finding was RE-PROBED rather than carried — the lupin binary moved, so
the tier that could have changed its answer is the one that got a new
build; it still reads 0 on all three lanes.

## sc32 — 2026-09-02 — the budget has a shape

Pins advance to wolf **v0.2.2** at 8cda3aa (THE LEARNERS' RELEASE) and
lupin **v0.1.22** at conformance pin 2bfbe5e, both real tags, both
`--version`-bare. The span is 35 commits — the largest this repo has
crossed in one bump — and the gap between the two pins is named: a
windows native bring-up, an LSP navigation trio and four letters, not
lowering debt. Drift was predicted ZERO and measured ZERO over 373x3,
the second consecutive empty drift list, with anchors 404 -> 411 (+7:
`mem.region.account{,.1,.2}` and `mem.region.cap{,.1,.2,.3}`) in the
first re-vendor that actually moves bytes since sc27. The 0.1.20 doctor
pin retires.

**`std.mem.budget` lands**: `charged(r)` and `live()` name the region
ledger's two queries (three lanes, including a `region` passed across a
module boundary — affine values are RETAINED by a `read` parameter, and
that took a probe to know), and `with_cap(n, f)` collapses D68's whole
containment join — spawn, monitor, `select`, `is_alloc_contract()` —
into one call whose failure is the ordinary row `exhausted`. That is
the shape lobo's per-request 503 consumes. Fifteen probes ran before a
line of the module was written and five of them changed it: a region
may be taken but not RETURNED (native refuses `-> region`), the work's
value cannot come back at all (a channel in a std signature is refused
on BOTH wolf rungs), the checked tier's C1 refusal is reached at
EXECUTION rather than statically (so one function yields two different
checked columns across three witnesses), and the trap-shaped runner is
not shipped because it is `region r(cap: n)` with a library in the way.
The row carries no payload because `[mem.region.cap.3]`'s
free-then-deliver teardown makes the dead proc's charge unobservable by
contract; a negative budget traps at the door so a caller's arithmetic
mistake is not answered with a recoverable value.

**wolf-lang#203's evidence is measured and written** (F-0104), not
built: a `List[int]` byte buffer charges exactly **16x** its payload on
both wolf tiers at every size from 1 KiB to 64 KiB — reproducing
lobo's numbers to the byte from a different program — and **32x** under
lupin, a multiplier the issue does not carry. A fourth measurement
rides along: a `str` charges NO named region's ledger on ANY tier,
where `[mem.region.account.1]` scopes that gap to the native one. The
recommendation is a language byte-width element type behind std's
already-documented `Bytes`, because every byte signature in std is
monomorphic over `List[int]` today and keeps its shape when it lands.

F-0103 re-measured verbatim and NOT adopted — wolf-lang#201 has not
ruled, and nothing in 35 commits touches `mem`'s argument handling, so
`bind, then name` stands. Residues re-dated: the chars-pairs tuple list
refuses at its **sixth** consecutive pin, F-0096 verbatim, and
`strbuf.in(r)` was RE-PROBED rather than argued for the first time —
the span moved regions for real, so the placement syntax was measured
(absent on every lane, `fail(E0201)` at parse for the struct form)
instead of reasoned from the commit list.

## sc31 — 2026-09-01 — the row gets a name

Pins advance to wolf **v0.2.1** at 75fd2d0 (a real release tag again —
the sc30 dev stamp retires, `wolf --version` answers bare) and lupin
**v0.1.20** at conformance pin b80d239, four commits behind the data
pin with the gap named: r04's four measured letters. Drift was
predicted ZERO and measured ZERO over 372x3 — the first sc bump whose
drift list came back empty — with anchors held at 404.

`std.x.tls.client` answers its first consumer's ask (wolf-std#3):
`named` coarsens the module's twenty-row vocabulary into one
payload-carrying tag whose payload is the refusing row's own NAME, so
a caller writes one handler arm instead of twenty and never forges a
dead `Client`; `row_name` is its marking face. The call-site spelling
is `bind, then name`, and the module header says why. Adopted at the
negative battery's three naming sites with byte-identical stdout on
all three lanes. F-0103 filed (wolf-lang#201): the checked tier
refuses a raising call passed straight into a row-typed parameter
where lupin and the native rung both run it — the long-unexplained
cause of three `std.option` ledger rows. The client's lane note now
states what a handshake COSTS: seconds, not milliseconds, at
unoptimized tiers.

## sc30 — 2026-08-31 — the slice comes home

Pins advance to wolf b80d239 (the s129/s130 merges; a dev-stamped
build whose `+dev.b80d239` identity carries the pin clause doctor
gates — no release tag exists past v0.2.0, and D57's honest answer is
the dev brand) and lupin v0.1.19 at conformance pin 83f83bb, one
merge behind the data pin with the gap named. Drift predicted two
movers and measured three, all lupin, all deeper: the sc29 byte-tier
rows go three-lane (F-0102 paid exactly as filed) and
`loopback_handshake`'s lupin lane RUNS the full TLS 1.3 handshake
inside the 50M step budget — sc29's "and the step budget" was an
inference the resolve refusal had shadowed, and the measurement
outvoted it. F-0101 closes the sc28 arc: `bytes.slice` re-adopts
`b[from..to]` (the retreat commit reverses) and the one row the sc28
adoption moved holds `run` on every lane with the range spelling —
found, filed, fixed, re-adopted. Struct patterns adopted where the
struct is born: 14 patterns at 8 files, probe-proven compositions
first, ledger flat throughout. The chars-pairs tuple list refused at
its fourth consecutive pin; F-0096 verbatim; anchors 404
(+`gram.pat.struct`).

## sc29 — 2026-08-31 — the client shakes the hand

std.x.tls.client lands: a TLS 1.3 client over the library's proven
halves, two-phase begin/complete for the single-threaded reality,
CertificateVerify VERIFIES (ed25519 + ecdsa-p256; sc21's
verify-nothing retires), the eleven-shape negative battery refuses by
name, and the loopback flagship shakes hands with a server half built
from the same primitives. std.net gains `read_bytes`/`write_bytes`
and the deadline pair — F-0049's `timeout` tag reachable at last.
F-0102 filed (wolf-interp#52): the byte tier dark under lupin 0.1.18.

## sc28 — 2026-08-30 — the library writes the new words

Pins advance to wolf v0.2.0 and lupin 0.1.18, both D57-bare release
builds, and doctor now gates wolf's provenance from `--version` itself
— a binary naming no pin is a red. The four `divergent(…)` ledger rows
flip to `run` exactly as 0.1.18's notes predicted. The c33/c39 surface
is adopted, not just measured: `str +` at 23 of 27 candidate sites,
tuple destructuring at 10 sites, comma-grouped binders, and list
slices in 12 loops at 9 sites — with one lend retreat filed rather
than papered (F-0101, wolf-lang#184). `gram.lex.ident` returns to the
anchor registry; the ledger stays flat throughout.

## sc27 — 2026-08-30 — the divergences re-measure

Pins converge on wolf 0a5c1af and lupin 0.1.17; drift was predicted
zero/zero and measured zero/zero. Residue row 6 heals — `str + str`
runs on all three lanes (D62) — and #50 healed with the sc22 cursor
boundary unmoved. The divergent four (wolf-interp#47/#48) stayed
unhealed at their third measurement, commented in place. Anchors +5,
clean; adoption candidates filed for the next contract, not taken.

## sc26 — 2026-08-29 — the native lane comes home

wolf at c1ca543 (the s59 apple wave plus s125–s127) un-parks the
staticlib: all three lanes light on macOS/arm64, with 364 of 365 rows
confirming the linux record silently, one s123 healing, and zero
divergences. Native x/ rows run in ≤1.4 s against the 60 s ceiling.
F-0096 re-measured unhealed; F-0100 filed (wolf-lang#170, the
spec-extract anchor loss).

## sc25 — 2026-08-28 — the waivers die

lupin 0.1.15 brings both binaries and the data pin to one sha: three
waivers dead and three lupin ledger columns flip to `run`.
std.unicode is retyped on `char` per D58 — `from_code -> char !
{none}`, the predicates compare char literals, `code`/`from_code` the
only int doors, `code_points` staying int deliberately. The four
divergent rows re-measured unhealed; the `slow` word retires on the
arm64 rig's own refusal.

## sc24 — 2026-08-28 — the twenty-eight re-measured

F-0018's 28 boundary-blocked contracts, re-tested clause by clause
across two wolf acquisitions: 24 of 28 shipped — including three new
this sprint on the D58 char surface (`str.to_list_chars`,
`strbuf.push(char)`, `unicode.code(char)`) — and a four-item residue
owned by name. `^n` healed; F-0096–F-0099 filed; the `divergent(…)`
ledger word is born for rows whose wrongness is the counterparty's.
35 rows deeper, 25 honest-downs. Pins a900b8c + lupin 0.1.14.

## The TLS rungs — sc20 → sc23 (2026-08-27)

- **sc23** — ECDSA-P256 as evidence: RFC 6979, Wycheproof
  secp256r1 (484 vectors) and CAVP 186-4 SigVer/SigGen vendored, every
  vector reproduced by a from-scratch big-int reference first; the
  emitters are native-only — a P-256 verify is two table-free ladders,
  past the checked and lupin step budgets. The data-pin bump moved
  zero ledger rows.
- **sc22** — the certificate: an X.690 DER decoder, RFC 5280 X.509
  profile, and Ed25519 chain validation over an in-repo test PKI; 25
  malformed inputs reject by name, and an unsupported algorithm is
  named, never accepted. F-0079 found fixed upstream (lupin 0.1.13)
  and closed.
- **sc21** — the handshake: message flow, transcript hash, the X25519
  key schedule, Finished, and the client state machine, RFC 8448-gated
  on all three lanes. The ws05 seam freezes.
- **sc20** — the record layer: HKDF-Expand-Label, per-record nonce,
  TLSCiphertext seal/open over ChaCha20-Poly1305, RFC 8448-gated,
  three-lane. F-0094 and F-0095 (wolf-lang#139) filed.

## The crypto ladder — sc16 → sc19 (2026-08-26/27)

- **sc19** — the ACME signature: JWS/EdDSA compact and flattened, OKP
  JWK with the RFC 7638 thumbprint, and the ACME request body
  (RFC 7515/8037/7638/8555), byte-exact against the RFC vectors; the
  ws06 seam freezes. The native/wolfc rows are the environment
  crypto-drift class; lupin green, CI-gated.
- **sc18** — the curve and the signature: X25519 and Ed25519 from
  RFC 7748/8032 over a checked-int field, 669 Wycheproof vectors with
  every flag decided; the dark lane lights — 41 rows. Typed-int limb
  discipline kills the i32-literal default.
- **sc17** — the cipher that needs no tables: ChaCha20-Poly1305 from
  RFC 8439, 353 vectors green; the tag compare marked for the
  constant-time tier.
- **sc16** — the digest ladder: SHA-2, HMAC and HKDF written from the
  documents, 1210 vectors green; the `slow` ledger word is born for a
  lane whose semantics are right and whose clock is not.

## The text and json turn — sc12 → sc15 (2026-08-13 → 08-26)

- **sc15** — the DOM half: std.json gains a checked handle over the
  declared json kernels, rows verbatim.
- **sc14** (2026-08-24) — each word, without the list: `each_word` as
  the lazy walk and `words_count` as the counting fold.
- **sc13** (2026-08-21) — the callable core, std's first higher-order
  tier: list predicates and relations as `fn` values; both F-0052
  wildcard handlers retire (the arms discriminate at lupin 0.1.13),
  the json F-0079 split retires and its witness catches the sequel
  (F-0084); F-0082–F-0086 filed from the sprint's probes.
- **sc14** (2026-08-14) — the four owed contracts, all paid:
  `json.parse`, `json.unescape`, `escape`'s totality, and
  `hex.decode_str`.
- **sc13** (2026-08-13) — the json DOM, and a way back from bytes:
  `bytes.to_str` lands on s81's `str_from_utf8` (F-0057 closed after
  four sprints of refusing an ASCII-only border post).
- **sc12** (2026-08-13) — eight functions walk the s77 byte view
  instead of copying.

(The ids sc13 and sc14 were each used twice in the track's history;
the entries above are ordered by landing date.)

## The os surface — sc07 → sc11 (2026-08-11/12)

- **sc11** — std.process: the pure builder over the process trio
  (`start` not spawn, `slot` not handle — both were keywords,
  F-0062), checked-lane honest; the signal row joins the taxonomy;
  `io.input_all` and `net.read_all` rewritten on the now-legal
  re-raising loop; seven ledger rows flip on the s71/s72 rulings.
- **sc10** — std.time (instant and duration facades, exact RFC 3339
  render, the Clock tag), std.env (argv arrives in std), and
  std.x.json — the nursery's first tenant, with its banner and
  three-outcome graduation clock.
- **sc09** — the F-0018 prize spent: std.str grows 16 → 37 functions,
  std.bytes is born at 9, twenty census rows flip, and the first
  all-three-lane test block lands.
- **sc08** — std.net: ten functions and two types over the s39
  builtins, the row vocabulary verbatim, take-consumed close;
  `read_all` written then withdrawn on the sprint's headline finding
  (F-0052, filed).
- **sc07** — std.fs (15 functions plus File with take-consumed close)
  and std.io (the write family, `input_line`, `prompt`); `copy`
  renamed `copy_file` (reserved keyword), `read_line` renamed
  `input_line` (prelude shadowing recursed under lupin); ten blocked
  contracts, filed.

## The core library — sc01 → sc06 (2026-08-10/11)

- **sc06** — std.option lands, six functions executing under lupin;
  errors at five and testing at thirteen with static-contract freight
  stated per function; the lowercase-tag rename (148 occurrences,
  zero ledger movement).
- **sc05** — fmt and encoding: exact float formatting (half-even
  fixed and exp, genuinely shortest `to_str`, correctly rounded
  `parse_float`), hex and base64 over a documented 0..255 byte
  contract, json escaping with 22 bodies; `hex.encode(str)` stated
  impossible rather than worked around.
- **sc04** — math, sort, search: 71 of 74 contracts with bodies;
  pure-wolf transcendentals at measured ulp 1 (3 for `powf`); the
  pcg32 stream pinned and cross-checked.
- **sc03** — strings and bytes: str 18 via the one safe primitive,
  strbuf 8, unicode 9 with the 25-entry case table pinned per code
  point; ascii-only find/split refused on principle; 28 contracts
  blocked on the missing boundary primitive — F-0018, the track's
  central filing.
- **sc02** — collections: std.list 19 executing, std.map 13, pool
  reserve/init; set and deque generics held as reviewed contracts
  blocked on `struct[T]`.
- **sc01** — cmp complete (Ordering with IEEE total order, Eq/Ord,
  min/max/clamp), testing five, iter prototypes; option blocked on
  row-position parsing, filed.

## sc00 — 2026-08-10 — the rig

The track opens: pins to the wolf-lang data snapshot and both
implementation binaries, an xtask `std-test` runner staging every
module through both implementations' `conform-run` with record
validation and bidirectional ledger enforcement, `doctor`,
`sync-pin`, the findings register (F-0001 on day one), and the
prelude proof module — `least`, `greatest`, `magnitude` — green under
both implementations with identical stdout hashes.
