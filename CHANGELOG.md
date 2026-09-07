# Changelog

## sc38 — 2026-09-07 — the waiver retires, and so does the pin that could not see it

**F-0099 IS RETIRED, AND THE RED CAME FIRST.** wolf-lang **v0.2.6**
(`a369b22`, [#239] — and **[#165], this repo's own filing**, the same gap
filed twice ten days apart by two lanes that never found each other)
appends `diag`, `ct`, `type` and `os` to `[conf.anchor.ns]`, on #120's
precedent exactly: additive, nothing renumbered, `[conf.anchor.stable]`
untouched — because an anchor's namespace IS its leading segment, so
MOVING the anchors would have renumbered 71 published ones and dragged
**3,273 citations across nine repositories** with them, against one
paragraph. A new `[conf.anchor.ns.admit]` writes down the rule the four
misses shared: a namespace is admitted **in one change or not at all**,
and the gap is **silent on whichever side is permissive**.

The red, captured at the v0.2.6 snapshot with `REGISTERED_NS` still at
seven:

> the pinned registry publishes **71** anchors in namespace(s)
> `{ct, diag, os, type}` that REGISTERED_NS does not admit —
> `[conf.tag.valid]` makes citing any one of them a CI failure here.

71 is upstream's own count at the same head, reached from the other side.

**AND THE PIN sc36 ARMED FOR EXACTLY THIS DID NOT MOVE.** Measured
first, deliberately: `f0099_the_four_unadmitted_namespaces_still_fail`
was **GREEN** at that same snapshot, with the gap already closed
upstream. Its failure message said "has `[conf.anchor.ns]` admitted its
namespace? Then retire F-0099" — and it could never have asked, because
it asked a hardcoded **two-anchor mock**, where `classify("os.net.unix")`
errs whether `os` is unregistered (the gap) or registered-with-no-such-
anchor (the mock). **A gap pinned against a mock cannot see the event it
pins**, and the failure message reads like a gate while being none. The
replacement asks the PINNED REGISTRY
(`every_published_anchor_sits_in_a_registered_namespace`) and reds at the
next bump that publishes an unadmitted namespace, which is
`[conf.anchor.ns.admit]`'s downstream half.

**The bill sc36 said would come due is paid, in the same commit.**
Seventeen files now cite the clause they hold instead of a stand-in: the
six `[os.net.unix]` witnesses; ten `ty.byte` -> **`type.byte`**; sc24's
char surface citing `type.char.cast` / `.interp` / `.order` beside
`mem.str.chars`; and sc37's four os rows citing `os.net.wait`,
`os.net.listen.opts`, `os.proc.inherit`, `os.cpus`. **`ty` stays
RESERVED**, and is reserved-and-USED here where upstream calls it
reserved-and-unused: `ty.match.exhaustive` and `ty.method.receiver-mode`
name clauses no document has written. Withdrawing a reservation is the
one direction that can reject legal input, and those two tags are the
concrete reason not to.

**F-0110 RETIRES ONE RELEASE EARLIER THAN ITS OWN LETTER SAID, AND THE
LETTER IS THE LESSON.** lupin **0.1.26 -> 0.1.27** (is38, pin `982f857`
-> `6ade878`). F-0110's exit was written "the first lupin conforming
**past** `6ade878`"; 0.1.27 conforms **at** `6ade878` — and that is
enough, because `6ade878` IS v0.2.5, the release the builtins land in.
The written condition was a PROXY for the one that mattered (has the
reference machine been SHOWN these calls) and it was off by a release.
**Re-measuring beat re-reading it.** Four of the five rows flip
`unsupported` -> `run` at FIRST SIGHT against sc37 bodies untouched by a
character (the F-0049 pattern): `net/listen_with_default`,
`net/reuse_port`, `net/wait_readiness`, `os/cpus`.
`LUPIN_TIER_WAIVERS` empties for the **third** time in its life.

**The fifth row did not move and its word now means something else.**
`net/adopt_rows.lu` stays `lupin = "unsupported"`, but is38 no longer
fails to RESOLVE the call — it DECLINES it, `x-unsupported: "listener
adoption in checked execution"`, the checked machine's own construct
string verbatim, for the checked machine's own reason. The word is
identical; the reason moved from a CALENDAR to a STATED POSTURE. That is
the one motion a lane word can make that **no ledger diff can show**, so
it is written into the ledger's sc37 block rather than left to inference.

**And the mechanism leaves a gate behind it.** Nothing in
`doc_examples.rs` could ever have noticed a waiver going inert —
`tier_waived` fires only on `Unsupported`, so a lane that starts RUNNING
makes the entry silently dead while the list goes on asserting a refusal
that no longer happens. Both previous emptyings (sc14, sc25) were caught
by a human at a bump. sc38 adds `waiver_fired`: **a waiver that never
fires is a RED naming itself** — wolf-lang#177's lesson mechanized on
this side, the same gate is38 built for `differ::retired_waivers`.

**wolf-std#8 — this rig's own clause bug, fixed with a gate on the
prose.** `std.net.accept`'s clause said "There is no non-blocking accept
and no way to poll … waits forever". Written at sc08, true then, and
false on both halves by the time lobo's ws18 read it: sc37 landed
`net.wait` **thirty lines below it in the same file**, and wolf-lang#242
(`[os.net.accept]`, new at v0.2.6) bounded the park. The clause now says
what the upstream clause says — an armed deadline **BOUNDS** the call,
`net.wait` is the poll answer by name, a readiness wake **is not a
claim**, and a lost race re-waits against the **same budget** and answers
`timeout`, which on N hands is the NORMAL outcome for N-1 of them.
`set_listener_deadline` gains the second line ws18 asked for: on a shared
listener the deadline is the mechanism, not the net for a mistake.

Two witnesses keep them from drifting again, because **prose is what no
gauntlet reads**: `net_accept_clause_agrees_with_os_net_accept` lints the
clause for the two retracted phrases and the five it must keep, and reds
if `os.net.accept` ever stops being a published anchor at the pin; and
`tests/net/accept_bounded_and_pollable.lu` proves the three live
sentences on three lanes — a listener CAN be polled before it is dialled,
an armed budget bounds the park, and **a `timeout` leaves the listener
unharmed**, which is the property the losing hand depends on. A rig
running one program cannot LOSE a race (that witness is upstream's
`corpus/net/accept_race.lu`); it can prove that going round the loop
again costs nothing.

**THE PIN, classified before it was measured.** `6ade878` (v0.2.5) ->
`398e5f5` (v0.2.6), 5 commits under two merges. #239 MECHANICAL upstream
/ BEHAVIOURAL for this rig's CI gate; #242 BEHAVIOURAL; #243
DIAGNOSTIC-ONLY. Predicted zero `.lu` behaviour motion and zero ledger
motion from all three, and measured zero of all three — every ledger row
that moved this sprint moved on the **lupin** bump, not the wolf one.
Anchors **422 -> 424**, key sets diffed BOTH ways (F-0100/#177): added
`{conf.anchor.ns.admit, os.net.accept}`, dropped `{}`, owners moved
`{}`. **The prediction missed one and the miss is recorded rather than
tidied**: +1 was predicted from #239's own commit message, +2 measured,
because **#242 publishes an anchor as well as changing behaviour**. A
spec delta classified as behavioural is still a registry delta, and the
both-ways diff is what caught it.

**Verify the machine, never the brief.** The lane opened with a written
claim that wolf v0.2.6 pairs with lupin 0.1.26. It does — the pairing
line says so — and the machine was running **0.1.27**, because is38
shipped after r09 cut the tag. Read from `--version`, not from the
paragraph (F-0064: the pairing line is reported, never gated).

**wolf-std#6 closed as DELIVERED, with the residue split out rather than
orphaned.** Everything #6 asked for at the std tier shipped at sc37,
including the §12 row question it raised in passing (`listen_with`
declares **`exists`** for a bind a sibling holds). lobo declined to CALL
the surface for three reasons of its own — a raw-fd serving loop, the
inherit pair splitting across tiers, four pure delegates — and those are
reasons a consumer does not call a surface, not reasons the surface is
missing. The one genuinely unmet piece, the `std.process` half of the
inherit pair, is **wolf-std#9**, carrying lobo's answer to the shape
question sc37 asked by name: **the inherit set goes on the SPAWN, not on
a `Command` builder**, because the listener set is a property of the
master and each spawn merely borrows it — a `Command` that remembers
descriptors is a `Command` that can hand a stale one down.

[#239]: https://github.com/wolffe-lang/wolf-lang/issues/239
[#165]: https://github.com/wolffe-lang/wolf-lang/issues/165

## sc37 — 2026-09-06 — the words came true: ten stale rows, a pin bump, and the server surface

**THE BUMP, AND ALL THREE WOLF-SIDE PINS LAND ON ONE SHA AGAIN.**
`982f857` (v0.2.4) → `6ade878` (v0.2.5), 42 commits: the binary's own
`--version` pin, `vendor/tools.toml` and `vendor/upstream/PIN` (with the
submodule at the same commit) all read `6ade878`, and the native rung is
lit at it. Anchors **417 → 422**, `+5 / −0` with both directions checked
— `os.cpus`, `os.net.listen.opts`, `os.net.wait`, `os.proc`,
`os.proc.inherit`. **The drift was predicted in writing before any
measurement and the prediction held exactly**: s137's five builtins are
NEW surface that no std file could have been calling, so zero existing
ledger rows moved on any of the three columns. The one commit in the
span with power to move a row — `6f57ee0`, a diagnostic underline clamp
(#238) — was named in the prediction as the first place to look, and it
moved nothing, because the wolfc column's vocabulary is an error CODE
and directives read program output rather than compiler stderr.

**wolf-std#7 — THE RED CAME FIRST, AND THAT IS THE PROOF.** lupin
0.1.25 → **0.1.26** (wolf-interp `v0.1.26` = `5e774a2`), installed
fresh-inode (132528014 → 135873958). The gauntlet was then run with
**not one word of the ledger touched**, and it was RED on **ten** rows —
eight `divergent(…)` carriers saying *the divergence moved*, and two
corpus twins saying *deeper than the ledger claims*. Only then were the
words rewritten. is37 closed wolf-interp#62: the byte's DOMAIN is a
resolve-time refusal now and not only the type name, so all ten
directives (`check: fail(E0401)`, `phase: typecheck`) are satisfied and
all ten rows become **`run`**. **`divergent(…)` returns to ZERO
carriers** — the second time in its life, and the second time it retired
on a release exactly as designed. A ledger edited before the measurement
would have gone green and proved nothing.

**Two of the ten were NOT predicted, and the miss is recorded rather
than tidied.** sc37 predicted the two corpus twins would not move,
reasoning that is37 had described a change of MECHANISM in them and not
of observed word. Wrong: a program refused at `typecheck` never reaches
the tier that was declining it, so `unsupported` became `run`. A row's
word is what the runner OBSERVES, never what the mechanism story
predicts.

**THE BRIEF WAS WRONG ABOUT THE MACHINE AND THE ISSUE WAS RIGHT.** The
lane was told "the machine already runs wolf 0.2.5 and lupin 0.1.26 —
verify, don't rebuild". `wolf` was 0.2.5 and was left alone. `lupin` was
**0.1.25**, and wolf-std#7's own text said so. A lane that had trusted
the brief would have measured against 0.1.25, seen green, and closed #7
as needing no work. Verification is the instruction that paid.

**FOUR NEW STD SURFACES over s137's builtins**, and all four house
shapes sc36 named survived contact with the runtime unchanged:

- **`net.listen_with(addr, opts) -> Listener ! {unsupported, exists,
  denied, io}`** with `ListenOpts { reuse_port, backlog }` and
  `listen_opts()`. An ACQUISITION call and nothing else: it answers the
  ordinary `Listener`, so `port`, `accept`, both deadlines and
  `close_listener` come free. `listen`'s own row stays `{io}` because
  its lowering coarsens, which is the one real reason to prefer this
  call even with default options.
- **`net.adopt_listener(fd) -> Listener ! {unsupported, io}`** — the
  CHILD's half of descriptor inheritance. `close_listener` **does**
  close an inherited fd, and an adopted `AF_UNIX` listener **does not**
  unlink its path: that belongs to the process that BOUND it.
- **`net.wait(fds, deadline_ms) -> List[int] ! {io}`** — readiness over
  a set. **An empty answer is an ANSWER, not a failure.** Takes raw
  descriptors because a ready set mixes listeners and streams and wolf
  has no sum type that could hold both at this pin.
- **`std.os.cpus() -> int ! {io}`** in a NEW one-function module.
  Propagates `io` and **never defaults to 1** — a program resolving
  "workers auto" must be able to say it did not learn the number.

**F-0110 — the first lane word this repo has owed to a release DATE.**
lupin 0.1.26 conforms to `982f857` = v0.2.4; s137's builtins land in
v0.2.5. The reference machine does not decline the new calls, it has
never been shown them (`unsupported: \`os_cpus\` does not resolve`). So
all six new witnesses carry `lupin = "unsupported"` for a reason that is
a calendar and not a semantics, and three doc-example blocks re-arm
`LUPIN_TIER_WAIVERS` — the mechanism whose own doc predicted this case
and said it would cost "a finding name, not plumbing". It is **not** a
`divergent(…)`: the machines do not disagree about a program's meaning,
one has not been given the program. Retires at the first lupin past
`6ade878`, touching no test.

**`net/adopt_rows.lu` is the most uneven row in the repository**, and
every column has its own reason: lupin is F-0110's calendar, the CHECKED
machine refuses adoption BY NAME (its own clause — it is the `wolf`
binary interpreting a program, so a descriptor handed to "the program's
child" would go to the compiler's child), and the NATIVE lane measures
the four `io` refusals. The adoptable case is unreachable in a rig with
no parent process, and the file says so rather than faking one.

**`reuse_port` pins two guarantees and deliberately not a third.** Every
dial is accepted by SOME member, and the survivor takes every dial after
the others close — asserted; WHICH member — never. linux distributes by
a 4-tuple hash, macOS hands every SYN to the newest bound socket, and a
witness that pinned either would be pinning a host's scheduling as if it
were the language's contract.

**A near-miss worth publishing.** Probing the inherit pair with `wolf
run --checked` answered `io` and `spawned`, which reads as a flat
contradiction of `[os.proc.inherit]` and was one step from being filed
upstream. **`wolf run --checked` is not the checked machine** — it runs
the native build. Under the rig's own `wolf conform-run --checked` both
calls refuse by name with the construct named. The clause is correct in
every particular.

**Residues, re-probed at a pin whose span actually contains a
compiler** (unlike sc36's): chars-pairs `List[(int, int)]()` refused at
its **eleventh** consecutive pin; F-0096 verbatim; `in(r)` unmoved on
both wolf rungs AND lupin's wording compared against sc36's recorded
STRING rather than the bare verdict, as sc36 instructed — byte-identical.
A `str` still charges no named region's ledger on any tier. F-0103
re-probed and unmoved, with the probe asserting that it TOOK the row
(`alpha:0`) — the check sc35 paid for twice. wolf-lang#201 is OPEN.

**F-0099 re-counted: four namespaces, 70 anchors** (was 65), and all
five of this sprint's new anchors landed in `os` — precisely the
namespace sc36 already could not cite. None of sc37's six witnesses may
name the clause it conforms to; they carry `std.net`/`std.os` forward
tags instead.

## sc36 — 2026-09-03 — the socket surface: a second address family, and a lane nobody predicted

**NO BUMP, and the prediction for that was written first.** sc35's second
bump left all three pins on one sha and installed the interpreter that
matched it, so sc36 opened against `wolf 0.2.4 (wolfgang, pin 982f857)`,
`lupin 0.1.25 (pin 982f857)` and `vendor/upstream/PIN 982f857` — the
one-sha invariant, held through a sprint that adds a module. Both of the
contract's either/ors resolved to their SECOND arm, checked twice each:
**s137 item 1 has not merged** (wolf-lang trunk is `1323c4e` at both
readings; `net_listen_with`/`os_spawn_with`/`net_adopt_listener` do not
exist at this pin), so `std.net.listen_with`/`adopt` is not written and
the ask **stands on wolf-std#6**; and **is37 did not tag** (`wolf-interp`
is at `ae34115`, newest tag `v0.1.25`), so the eight byte-domain
`divergent(…)` rows citing wolf-interp#62 stand verbatim — proven
unmoved by the run rather than assumed, since a divergent row that stops
matching its observation is a RED here.

**`std.net.unix` — two functions, and that is the whole module.**
`unix.listen(path) -> net.Listener ! {unsupported, exists, not_found,
denied, io}` and `unix.connect(path) -> net.Socket ! {unsupported,
refused, not_found, denied, io}` over `[os.net.unix]` (s136,
wolf-lang#227). They return `std.net`'s OWN types, so `accept`, the str
and byte read/write pairs, both deadlines and both closers serve a unix
socket call for call — measured over a socket path, not asserted. A
separate module because wolf has no overloading and `listen` cannot mean
two things in one (§1, the `std.math.float` rule). Two `std.net`
functions behave differently on a unix value and both now say so: `port`
is `io` (a path has no port) and `close_listener` UNLINKS the path.

**The finding of the sprint is a lane word.** Every os block in this
ledger has the same shape — the wolf rungs run it, lupin declines the
capability — and the prediction written before the first probe was a
two-lane module, because the reference machine has no filesystem.
**lupin 0.1.25 serves the whole family.** A unix socket is not a file the
machine has to READ; it is a host object it asks the host for. Four of
six witnesses are THREE-LANE; the two that are not assert about the path
with `fs.exists`/`fs.move_file`, which is what lupin actually declines.
Its one named limit is a path that climbs out of the working directory
(`unsupported` by name, wolf-interp#18 item 6) — a spelling §14's
relative-path rule already forbids, and `sun_path`'s ~104 bytes on macOS
is the second reason to keep that rule.

**F-0109 — the rig stages ONE directory for THREE lanes, and the machine
with no filesystem is the one that left a file in it.** Found by a RED:
the first gauntlet failed on exactly two rows, `exit(3)` where `exit(0)`
was expected. lupin resolves bodies LAZILY, so `unix.listen` — a call it
SERVES — created a real socket file before the next line's `fs.exists`
was declined, and wolfc's bind then answered `exists`. Every previous
capability module made the two facts coincide (no fs, so no file); this
one separates them, because the call that creates the file is a `net`
call. The fix is the practice the module already documents — the owner's
idiom, `if fs.exists(path) { fs.remove(path)? }`, on the first line — so
the refusal now lands before the bind.

**One row std deliberately does NOT promise, and it came from reading
somebody else's tests.** A dial of a path that is not a socket at all is
neither of the clause's two dial cases: macOS answers `ENOTSOCK` → `io`,
linux answers `ECONNREFUSED` → `refused`. is36 found that on a CI runner
after every developer machine had agreed on `io`, and wolf-interp's own
`net_unix.rs` now accepts either. std adopted the posture rather than
pinning macOS: **when the tag is the kernel's and the kernels disagree,
promise the CLASS and not the member** — it is a row, never a trap — and
`rows.lu` pins the four rows the clause rules and deliberately not the
fifth. `denied` is the other honest edge: measured out of band on all
three lanes (a mode-000 directory, at bind AND at dial), declared in both
signatures, and with no hermetic witness because there is no `chmod`
builtin.

**F-0103 re-probed at `982f857` — unmoved, predicted from a span with no
`.rs` in it.** f18/f19, character-identical but for a loop bound: the one
whose row is TAKEN is `unsupported — control flow in an argument` at
`mem` on the checked lane, the one that does not take it runs on all
three. wolf-lang#201 stays OPEN and the sharpened warning is repeated on
its thread. Residues at their tenth pin: the chars-pairs tuple list, and
F-0096 — both refused verbatim; `in(r)`'s wolf-rung reading is
re-confirmed and LUPIN'S WORDING moved (a better sentence, same verdict,
recorded so the next re-probe compares against the right string); a `str`
still charges no named region's ledger on any tier.

**F-0099 re-counted at FOUR namespaces and the bill came due.** The
pinned registry publishes 417 anchors in eleven namespaces;
`[conf.anchor.ns]` admits seven. `os`, `type`, `ct` and `diag` are not
among them — so **sc36 implements `[os.net.unix]` and none of its six
witnesses can cite the clause it conforms to**. Filed upstream as
wolf-lang#239 on the #120 precedent; the rig now pins the gap as a unit
test whose failure message says to retire F-0099. One lag in the other
direction fixed here: `test` was reserved by the clause at s39 and this
rig's `FORWARD_NS` never followed.

**`cargo xtask ci`: GREEN, exit 0** over the committed tree — 382 tests
(376 -> 382), 719 forward tags, 204 conservatism entries, 0 unstable, 0
slow, **8 divergent** (the same eight); doc-examples **414 blocks GREEN,
+0** — a designed zero, because §4's one-module note leaves this module
prose examples and the reason is on both functions; ulp 200 rows GREEN on
all three lanes.

## sc35 — 2026-09-03 — the rename, the second gate, and the release that opened it

**SECOND BUMP, same day. lupin 0.1.23 -> 0.1.25 (is36 ships `[type.byte]`
in the mirror) and wolf `0.2.3+dev.4230b00` -> the **v0.2.4 TAG**
(`982f857`), data pin with it — five commits whose whole diff touches
eight files and NOT ONE `.rs`. Drift predicted zero from the wolf half
over a span with no compiler in it, and measured zero: anchors **417 ->
417, +0/-0** with the vendored snapshot BYTE-IDENTICAL (the first bump
here where only `PIN` moves), corpus 511 -> 511.

**All three pins read one sha for the first time in this repository's
history** — wolf's, lupin's and `vendor/upstream/PIN` are all `982f857`.
sc30 pulled them apart, sc33 suspended the invariant, sc34 restored it
between the binary and the data pin; this adds the interpreter. **And the
dev stamp retires**: v0.2.4 is the first tag that carries s135 and s136
together, so the `+dev.<commit>` mechanism r03 built for exactly this gap
was used for precisely as long as the gap lasted. wolf's pairing line
still names `lupin 0.1.24 … pin 3befc3e` (r07's release-time PAIRING
record) against an installed 0.1.25 — reported, not gated (F-0064).

**F-0108 CLOSES, one day after filing, by the release it named as its own
exit.** 181 of 181 dark rows returned (122 exit / 48 unsupported / 11
trap), 87 of 87 doc blocks returned, **zero residual refusals** — and not
one lane word in `tests/ledger.toml` had to be corrected for any of them.
A ledger left honestly wrong, with its own block note naming the count
and the cause, needed no repair when the cause left.

What remains is **eight rows with a legal word**. lupin 0.1.25 has the
byte TYPE but not the byte DOMAIN — `push(256)` into a `List[byte]` still
stores 256 — so where both compilers refuse at typecheck the interpreter
runs the program. That is `divergent(…)`, F-0098's word, used outside the
take-mode pair for the first time: five `divergent(exit(0))`, two
`divergent(exit(1))`, one `divergent(trap(assert))`, plus two ordinary
`unsupported`. Filed as **wolf-interp#62** (is37's). `hex`'s
`byte_digits` assert is the one to read twice — unreachable on both wolf
lanes and still firing under lupin, so a sentence that was true of the
compilers and false of the reference machine now lives in the divergence
ledger instead of in an unchecked claim.

**`cargo xtask ci`: GREEN, exit 0** — 376 tests, 700 forward tags, 201
conservatism entries, 0 unstable, 0 slow, **8 divergent**; doc-examples
414 blocks GREEN; ulp 200 rows GREEN on all three lanes.

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
