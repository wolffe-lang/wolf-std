# Changelog

## sc39 — 2026-09-08 — the spawn takes the set, and the first child this rig ever ran told on the docs

`sched` is admitted, and the gate announced it before a human did.
Data pin `398e5f5` (v0.2.6) -> `ed8f526` (merge s139, wolf-lang#246):
`spec/07-schedule-points.md` ruled NORMATIVE and its seven anchors
published, 424 -> 431, additive, none dropped, no owner moved, verified
by diffing the key sets both ways. `REGISTERED_NS` takes `sched` as its
twelfth, in the same commit as the re-vendor, per
`[conf.anchor.ns.admit]`.

The red, captured with the new snapshot in place and the append not yet
written:

> the pinned registry publishes **7** anchors in namespace(s) `{"sched"}`
> that REGISTERED_NS does not admit — `[conf.tag.valid]` makes citing any
> one of them a CI failure here.

The data pin now leads the `wolf` binary's pin, which is legal.
`vendor/tools.toml` is unmoved, both binaries are where sc38 left them
(`wolf 0.2.6` pin `398e5f5`, `lupin 0.1.27` pin `6ade878`), so the data
pin sits three commits ahead of the compiler that reads the data. Nothing
in the `398e5f5..ed8f526` span is runtime surface: it is `spec/07`'s
disposition, the anchor publication, upstream's own `NS_OWNERS` refactor
and one corpus tag. `doctor` reports the pairing and does not gate it
(F-0064, the two-moving-upstreams rule), and `sync-pin` verified
snapshot == submodule at the new pin. The inverse of this gap happened at
sc35, with the reasoning recorded in `tools.toml`; this is the first time
it points the other way.

That is sc38's replacement pin firing on its first real event, one pin
bump after it was written to replace a mock-backed one that was green and
always would have been. It was also the only source with the state right:
wolf-std#10's filed text said "five namespaces behind" (written before
sc38 merged) and the orchestrator's correction said the work was one
entry. The issue was stale, the correction was right, and the gate
measured the count itself.

Added while the file was open, covering the direction
`[conf.anchor.ns.admit]` calls "silent on whichever side is permissive":
`every_registered_namespace_publishes_at_least_one_anchor`. A name in
this list that the registry publishes nothing for is an admission that
ran ahead of the clause, or a bump that stranded an entry. Neither
rejects anything, so neither can be found except by asking.

The inherit pair is whole at the std tier (wolf-std#9). `std.net` has
had the child half since sc37 (`adopt_listener`); `std.process` now has
the parent half:

```wolf
pub fn start_with(c: Command, inherit: List[net.Listener])
    -> Child ! {unsupported, not_found, denied, io}
```

The set is an argument to the spawn; the `Command` does not hold it.
That is the question sc37 asked, answered by the consumer that needed it
from real code (lobo, ws18) and built here as answered. The argument is
lifetime: a master binds its listeners once and keeps them for life
because a replacement worker must inherit the same socket, then spawns
worker 1, worker 2, and re-spawns worker 2 after a `kill -9`. The set is
a property of the master and each spawn is an event that borrows it; a
builder that accumulated one would have to be reset per spawn, and one
reused across spawns would hand a stale descriptor down. The measurement:
one `List[net.Listener]` binding serves two spawns, and the listener
still answers `port` and still closes afterwards.

The tier-split reasoning that made lobo decline the net half is the
reason to build this one. lobo's reason (2) for staying on the builtins
was that the inherit pair split across tiers while the parent half was
unwrapped. That was an argument about a gap, and `start_with` closes the
gap. The handshake no longer crosses tiers: parent `process.start_with`,
child `net.adopt_listener`. lobo's other two reasons are untouched and
its loop stays raw-fd, as it said; this surface is for the next consumer.

The parameter is `List[net.Listener]`, where `net.wait` went the other
way with `List[int]`: `wait`'s ready set mixes listeners and streams and
wolf has no sum type that can hold both, so it had no typed container to
take. An inherit set has one, since the only thing a child can do with a
handed-down descriptor is `adopt_listener`, so the unwrapping belongs in
std. If a pin ever grows an adopt for connected sockets the container
question reopens with the same language dependency `wait` is waiting on,
and the two should be answered together.

This sprint has the first test in this repository that runs a parent and
a child. In `tests/process/prefork_handoff.lu` the parent binds an
ephemeral loopback listener, spawns itself (`os_exe`, s90) with the port
as its one argument and the listener as its one inherit entry; the child
adopts descriptor 3, true by position, reads the port off what it
adopted, and exits `0` only if the two agree. It also witnesses a promise
about absence that nothing in either repository had witnessed: with one
entry in the set, descriptor 4 is not adoptable (`io`), which is
`[os.proc.inherit]`'s "nothing above `2 + len(inherit)` is open in the
child that the runtime put there". Two negative controls ran before it
landed and both behaved: a port one digit off makes the child exit `7`,
and an empty inherit set makes its `adopt_listener(3)` answer `io`.

F-0066's happy path is witnessed for the first time. All three legs came
off: `os_exe` (s90) retired the first at sc12, and sc37 + sc39 gave the
child something to do that the parent can verify, which is what "a
program that spawns itself must tell the child from the parent" was
asking for. What remains is bounded: the witness is native-only, for two
measured reasons.

F-0111, and the child told on the docs. `std.process`'s header has said
since sc11 that a child's stdio is "CLOSED at this pin", wired to the
host's null device, so "neither its output nor its diagnostics reach the
parent's streams". That is false on both executing lanes: a self-spawned
child's `print` lands in the parent's stdout natively, and on the checked
lane the child's own usage line lands on the parent's stderr. Both spawn
builtins, empty set and non-empty. The claim survived four sprints
because nothing here could check it (F-0066 says no test could run a
child), so it was prose over a dark lane, and the first test to light the
lane caught it while doing something else. F-0065 stays open with a
changed shape: inheriting is not capturing, so `output(c)` still has
nowhere to read from. But "nothing it writes is visible to this program"
is the sentence a caller plans around, and a library spawning a helper is
putting that helper's chatter into its caller's streams. Filed
[wolf-lang#256](https://github.com/wolffe-lang/wolf-lang/issues/256):
`[os.proc]` states no stdio posture at all, and `[os.proc.inherit]`'s
"descriptors 0, 1 and 2 are the spawn's ordinary stdio" is the only
sentence in the spec that touches it, and "ordinary" does not decide the
question. The same filing carries a second measurement: under an
interpreter `os_exe` names the interpreter and not the program, which is
why a self-spawn witness can only be native.

The retracted-sentence lint gets its second use, and ends a second rotted
clause. sc38 wrote one for `net.accept` after a clause rotted with the
call that falsified it thirty lines below. `net.adopt_listener`'s clause
said "There is no `std.process` half of this pair at this pin", true when
sc37 wrote it, false the moment `start_with` landed, and invisible to
every gauntlet because the two halves live in different modules.
`the_inherit_pair_names_itself_from_both_ends` lints both clauses: each
must name the other's function, and `start_with`'s must keep saying the
six things `[os.proc.inherit]` promises that a signature cannot show.

Also: `std.process`'s "lanes" paragraph is corrected against the ledger.
It claimed the native rung refused the whole module, which s107
(`1b149ba`) made false four sprints ago. And `run_with` joins the
reviewed-absent list, because the only reason to hand a listener down is
that the child goes on serving, and a call that blocks until the child
exits serves nobody.

## sc38 — 2026-09-07 — the waiver retires, and so does the pin that could not see it

F-0099 is retired, and the red came first. wolf-lang v0.2.6
(`a369b22`, [#239], and [#165], this repo's own filing, the same gap
filed twice ten days apart by two lanes that never found each other)
appends `diag`, `ct`, `type` and `os` to `[conf.anchor.ns]`, on #120's
precedent: additive, nothing renumbered, `[conf.anchor.stable]`
untouched. An anchor's namespace is its leading segment, so moving the
anchors would have renumbered 71 published ones and dragged 3,273
citations across nine repositories with them, against one paragraph. A
new `[conf.anchor.ns.admit]` writes down the rule the four misses shared:
a namespace is admitted in one change or not at all, and the gap is
silent on whichever side is permissive.

The red, captured at the v0.2.6 snapshot with `REGISTERED_NS` still at
seven:

> the pinned registry publishes **71** anchors in namespace(s)
> `{ct, diag, os, type}` that REGISTERED_NS does not admit —
> `[conf.tag.valid]` makes citing any one of them a CI failure here.

71 is upstream's own count at the same head, reached from the other side.

The pin sc36 armed for this did not move. It was measured first:
`f0099_the_four_unadmitted_namespaces_still_fail` was green at that same
snapshot, with the gap already closed upstream. Its failure message said
"has `[conf.anchor.ns]` admitted its namespace? Then retire F-0099", and
it could never have asked, because it asked a hardcoded two-anchor mock,
where `classify("os.net.unix")` errs whether `os` is unregistered (the
gap) or registered-with-no-such-anchor (the mock). A gap pinned against a
mock cannot see the event it pins, and the failure message reads like a
gate while being none. The replacement asks the pinned registry
(`every_published_anchor_sits_in_a_registered_namespace`) and reds at the
next bump that publishes an unadmitted namespace, which is
`[conf.anchor.ns.admit]`'s downstream half.

The bill sc36 said would come due is paid in the same commit. Seventeen
files now cite the clause they hold instead of a stand-in: the six
`[os.net.unix]` witnesses; ten `ty.byte` -> `type.byte`; sc24's char
surface citing `type.char.cast` / `.interp` / `.order` beside
`mem.str.chars`; and sc37's four os rows citing `os.net.wait`,
`os.net.listen.opts`, `os.proc.inherit`, `os.cpus`. `ty` stays reserved,
and it is reserved-and-used here where upstream calls it
reserved-and-unused: `ty.match.exhaustive` and `ty.method.receiver-mode`
name clauses no document has written. Withdrawing a reservation is the
one direction that can reject legal input, and those two tags are the
concrete reason to leave it standing.

F-0110 retires one release earlier than its own letter said, and the
letter is the lesson. lupin 0.1.26 -> 0.1.27 (is38, pin `982f857` ->
`6ade878`). F-0110's exit was written "the first lupin conforming past
`6ade878`"; 0.1.27 conforms at `6ade878`, and that is enough, because
`6ade878` is v0.2.5, the release the builtins land in. The written
condition was a proxy for the one that mattered (has the reference
machine been shown these calls) and it was off by a release.
Re-measuring beat re-reading it. Four of the five rows flip
`unsupported` -> `run` at first sight against sc37 bodies untouched by a
character (the F-0049 pattern): `net/listen_with_default`,
`net/reuse_port`, `net/wait_readiness`, `os/cpus`.
`LUPIN_TIER_WAIVERS` empties for the third time in its life.

The fifth row did not move and its word now means something else.
`net/adopt_rows.lu` stays `lupin = "unsupported"`, but the new machine
resolves the call and declines it, `x-unsupported: "listener
adoption in checked execution"`, the checked machine's own construct
string verbatim,
for the checked machine's own reason. The word is identical; the reason
moved from a calendar to a stated posture. No ledger diff can show that
motion, so it is written into the ledger's sc37 block.

The mechanism leaves a gate behind it. Nothing in `doc_examples.rs` could
have noticed a waiver going inert: `tier_waived` fires only on
`Unsupported`, so a lane that starts running makes the entry silently
dead while the list goes on asserting a refusal that no longer happens.
Both previous emptyings (sc14, sc25) were caught by a human at a bump.
This sprint adds `waiver_fired`, so a waiver that never fires is a red
that names itself, wolf-lang#177's lesson mechanized on this side, the
same gate is38 built for `differ::retired_waivers`.

wolf-std#8 is this rig's own clause bug, fixed with a gate on the prose.
`std.net.accept`'s clause said "There is no non-blocking accept and no
way to poll … waits forever". Written at sc08, true then, and false on
both halves by the time lobo's ws18 read it: sc37 landed `net.wait`
thirty lines below it in the same file, and wolf-lang#242
(`[os.net.accept]`, new at v0.2.6) bounded the park. The clause now says
what the upstream clause says: an armed deadline bounds the call,
`net.wait` is the poll answer, a readiness wake is no claim, and a lost
race re-waits against the same budget and answers `timeout`, which on N
hands is the normal outcome for N-1 of them. `set_listener_deadline`
gains the second line ws18 asked for: on a shared listener callers arm
the deadline as the ordinary mechanism.

Two witnesses keep them from drifting again, since no gauntlet reads
prose: `net_accept_clause_agrees_with_os_net_accept` lints the clause for
the two retracted phrases and the five it must keep, and reds if
`os.net.accept` ever stops being a published anchor at the pin; and
`tests/net/accept_bounded_and_pollable.lu` proves the three live
sentences on three lanes, that a listener can be polled before it is
dialled, that an armed budget bounds the park, and that a `timeout`
leaves the listener unharmed, the property the losing hand depends on. A
rig running one program cannot lose a race (that witness is upstream's
`corpus/net/accept_race.lu`); it can show that going round the loop again
costs nothing.

The pin was classified before it was measured. `6ade878` (v0.2.5) ->
`398e5f5` (v0.2.6), 5 commits under two merges. #239 MECHANICAL upstream
/ BEHAVIOURAL for this rig's CI gate; #242 BEHAVIOURAL; #243
DIAGNOSTIC-ONLY. All three were predicted to move no `.lu` behaviour and
no ledger row, and all three measured that way; every ledger row that
moved this sprint moved on the lupin bump. Anchors 422 -> 424, key sets
diffed both ways (F-0100/#177): added
`{conf.anchor.ns.admit, os.net.accept}`, dropped `{}`, owners moved
`{}`. The prediction missed one, and the miss is recorded here: +1 was
predicted from #239's own commit message and +2 measured, because #242
publishes an anchor as well as changing behaviour. A spec delta
classified as behavioural is still a registry delta, and the both-ways
diff caught it.

Read the version off the machine. The lane opened with a written claim
that wolf v0.2.6 pairs with lupin 0.1.26. It does, the pairing line says
so, and the machine was running 0.1.27, because is38 shipped after r09
cut the tag. `--version` is the source (F-0064: the pairing line is
reported, never gated).

wolf-std#6 is closed as delivered, with the residue split into its own
issue. Everything #6 asked for at the std tier shipped at sc37,
including the §12 row question it raised in passing (`listen_with`
declares `exists` for a bind a sibling holds). lobo declined to call the
surface for three reasons of its own: a raw-fd serving loop, the inherit
pair splitting across tiers, four pure delegates. Those are reasons a
consumer does not call a surface, and none of them says the surface is
missing. The one unmet piece, the `std.process` half of the inherit pair,
is wolf-std#9, carrying lobo's answer to the shape question sc37 asked:
the inherit set goes on the spawn and not on a `Command` builder, because
the listener set is a property of the master and each spawn borrows it,
and a `Command` that remembers descriptors is a `Command` that can hand
a stale one down.

[#239]: https://github.com/wolffe-lang/wolf-lang/issues/239
[#165]: https://github.com/wolffe-lang/wolf-lang/issues/165

## sc37 — 2026-09-06 — the words came true: ten stale rows, a pin bump, and the server surface

The bump lands all three wolf-side pins on one sha again. `982f857`
(v0.2.4) → `6ade878` (v0.2.5), 42 commits: the binary's own `--version`
pin, `vendor/tools.toml` and `vendor/upstream/PIN` (with the submodule at
the same commit) all read `6ade878`, and the native rung is lit at it.
Anchors 417 → 422, `+5 / −0` with both directions checked: `os.cpus`,
`os.net.listen.opts`, `os.net.wait`, `os.proc`, `os.proc.inherit`. The
drift was predicted in writing before any measurement, and the prediction
held: s137's five builtins are new surface that no std file could have
been calling, so zero existing ledger rows moved on any of the three
columns. The one commit in the span with power to move a row, `6f57ee0`,
a diagnostic underline clamp (#238), was named in the prediction as the
first place to look, and it moved nothing, because the wolfc column's
vocabulary is an error code and directives read program output instead of
compiler stderr.

wolf-std#7: the red came first. lupin 0.1.25 → 0.1.26 (wolf-interp
`v0.1.26` = `5e774a2`), installed fresh-inode (132528014 → 135873958).
The gauntlet was then run with not one word of the ledger touched, and it
was red on ten rows: eight `divergent(…)` carriers saying *the divergence
moved*, and two corpus twins saying *deeper than the ledger claims*. Only
then were the words rewritten. The is37 sprint closed wolf-interp#62,
making the byte's domain a resolve-time refusal and not only the type
name, so all ten directives (`check: fail(E0401)`, `phase: typecheck`)
are satisfied and all ten rows become `run`. `divergent(…)` returns to
zero carriers, the second time in its life and the second time it retired
on a release as designed. A ledger edited before the measurement would
have gone green and shown nothing.

Two of the ten went unpredicted, and the miss is recorded here. The
prediction said the two corpus twins would not move, reasoning that is37
had described a change of mechanism in them and not of observed word.
That was wrong: a program refused at `typecheck` never reaches the tier
that was declining it, so `unsupported` became `run`. A row's word is
what the runner observes, and the mechanism story does not set it.

The brief was wrong about the machine and the issue was right. The lane
was told "the machine already runs wolf 0.2.5 and lupin 0.1.26 —
verify, don't rebuild". `wolf` was 0.2.5 and was left alone. `lupin` was
0.1.25, and wolf-std#7's own text said so. A lane that had trusted the
brief would have measured against 0.1.25, seen green, and closed #7 as
needing no work. The verification is what paid.

Four new std surfaces sit over s137's builtins, and all four house
shapes sc36 named survived contact with the runtime unchanged:

- `net.listen_with(addr, opts) -> Listener ! {unsupported, exists,
  denied, io}` with `ListenOpts { reuse_port, backlog }` and
  `listen_opts()`. An acquisition call: it answers the ordinary
  `Listener`, so `port`, `accept`, both deadlines and `close_listener`
  come free. `listen`'s own row stays `{io}` because its lowering
  coarsens, which is the one real reason to prefer this call even with
  default options.
- `net.adopt_listener(fd) -> Listener ! {unsupported, io}`, the child's
  half of descriptor inheritance. `close_listener` closes an inherited
  fd, and an adopted `AF_UNIX` listener leaves its path in place, since
  unlinking belongs to the process that bound it.
- `net.wait(fds, deadline_ms) -> List[int] ! {io}`, readiness over a
  set. An empty list is a successful answer. It takes raw descriptors
  because a ready set mixes listeners and streams and wolf has no sum
  type that could hold both at this pin.
- `std.os.cpus() -> int ! {io}` in a new one-function module. It
  propagates `io` and never defaults to 1, so a program resolving
  "workers auto" can say it did not learn the number.

F-0110 is the first lane word this repo has owed to a release date.
lupin 0.1.26 conforms to `982f857` = v0.2.4; s137's builtins land in
v0.2.5. The reference machine has never been shown the new calls
(`unsupported: \`os_cpus\` does not resolve`). So all six new witnesses
carry `lupin = "unsupported"` for a calendar reason, and three
doc-example blocks re-arm `LUPIN_TIER_WAIVERS`, the mechanism whose own
doc predicted this case and said it would cost "a finding name, not
plumbing". It is no `divergent(…)`: the machines agree about the
program's meaning, and one has not been given the program. Retires at the
first lupin past `6ade878`, touching no test.

`net/adopt_rows.lu` is the most uneven row in the repository, and every
column has its own reason: lupin is F-0110's calendar; the checked
machine refuses adoption in its own clause, since it is the `wolf` binary
interpreting a program, so a descriptor handed to "the program's child"
would go to the compiler's child; and the native lane measures the four
`io` refusals. The adoptable case is unreachable in a rig with no parent
process, and the file says so instead of faking one.

`reuse_port` pins two guarantees and leaves a third alone. The witness
asserts that every dial is accepted by some member and that the survivor
takes every dial after the others close; which member takes a given dial
is never asserted. linux distributes by a 4-tuple hash, macOS hands every
SYN to the newest bound socket, and a witness that pinned either would be
pinning a host's scheduling as if it were the language's contract.

A near-miss, published here. Probing the inherit pair with `wolf
run --checked` answered `io` and `spawned`, which reads as a flat
contradiction of `[os.proc.inherit]` and was one step from being filed
upstream. `wolf run --checked` runs the native build and is not the
checked machine. Under the rig's own `wolf conform-run --checked` both
calls refuse with the construct named. The clause is correct in every
particular.

Residues, re-probed at a pin whose span contains a compiler (sc36's did
not): chars-pairs `List[(int, int)]()` refused at its eleventh
consecutive pin; F-0096 verbatim; `in(r)` unmoved on both wolf rungs, and
lupin's wording compared against sc36's recorded string instead of the
bare verdict, as sc36 instructed, byte-identical. A `str` still charges
no named region's ledger on any tier. F-0103 re-probed and unmoved, with
the probe asserting that it took the row (`alpha:0`), the check sc35 paid
for twice. wolf-lang#201 is OPEN.

F-0099 re-counted: four namespaces, 70 anchors (was 65), and all five of
this sprint's new anchors landed in `os`, the namespace sc36 already
could not cite. None of the six witnesses may name the clause it conforms
to; they carry `std.net`/`std.os` forward tags instead.

## sc36 — 2026-09-03 — the socket surface: a second address family, and a lane nobody predicted

No bump, and the prediction for that was written first. sc35's second
bump left all three pins on one sha and installed the interpreter that
matched it, so this sprint opened against `wolf 0.2.4 (wolfgang, pin 982f857)`,
`lupin 0.1.25 (pin 982f857)` and `vendor/upstream/PIN 982f857`, the
one-sha invariant, held through a sprint that adds a module. Both of the contract's either/ors resolved to their second arm,
checked twice each: s137 item 1 has not merged (wolf-lang trunk is
`1323c4e` at both readings; `net_listen_with`/`os_spawn_with`/
`net_adopt_listener` do not exist at this pin), so
`std.net.listen_with`/`adopt` is not written and the ask stands on
wolf-std#6; and is37 did not tag (`wolf-interp` is at `ae34115`, newest
tag `v0.1.25`), so the eight byte-domain `divergent(…)` rows citing
wolf-interp#62 stand verbatim, shown unmoved by the run, since a
divergent row that stops matching its observation is a red here.

`std.net.unix` is two functions, and that is the module.
`unix.listen(path) -> net.Listener ! {unsupported, exists, not_found,
denied, io}` and `unix.connect(path) -> net.Socket ! {unsupported,
refused, not_found, denied, io}` over `[os.net.unix]` (s136,
wolf-lang#227). They return `std.net`'s own types, so `accept`, the str
and byte read/write pairs, both deadlines and both closers serve a unix
socket call for call, measured over a socket path. It is a separate
module because wolf has no overloading and `listen` cannot mean two
things in one (§1, the `std.math.float` rule). Two `std.net` functions
behave differently on a unix value and both now say so: `port` is `io` (a
path has no port) and `close_listener` unlinks the path.

The finding of the sprint is a lane word. Every os block in this ledger
has the same shape, the wolf rungs run it and lupin declines the
capability, and the prediction written before the first probe was a
two-lane module, because the reference machine has no filesystem. lupin
0.1.25 serves the whole family. A unix socket is a host object the
machine asks the host for, never a file it has to read. Four of six
witnesses are three-lane; the other two assert about the path with
`fs.exists`/`fs.move_file`, which is what lupin declines. Its one named
limit is a path that climbs out of the working directory (`unsupported`,
wolf-interp#18 item 6), a spelling §14's relative-path rule already
forbids, and `sun_path`'s ~104 bytes on macOS is the second reason to
keep that rule.

F-0109: the rig stages one directory for three lanes, and the machine
with no filesystem is the one that left a file in it. A red found it, the
first gauntlet failing on two rows, `exit(3)` where `exit(0)` was
expected. lupin resolves bodies lazily, so `unix.listen`, a call it
serves, created a real socket file before the next line's `fs.exists` was
declined, and wolfc's bind then answered `exists`. Every previous
capability module made the two facts coincide (no fs, so no file); this
one separates them, because the call that creates the file is a `net`
call. The fix is the practice the module already documents, the owner's
idiom `if fs.exists(path) { fs.remove(path)? }` on the first line, so the
refusal now lands before the bind.

One row std does not promise, and it came from reading somebody else's
tests. A dial of a path that is not a socket at all is neither of the
clause's two dial cases: macOS answers `ENOTSOCK` → `io`, linux answers
`ECONNREFUSED` → `refused`. A CI runner surfaced that during is36, after
every developer machine had agreed on `io`, and wolf-interp's own
`net_unix.rs` now accepts either. std adopted the posture instead of
pinning macOS: when the tag is the kernel's and the kernels disagree,
promise the class and leave the member open. It is a row and never a
trap, and `rows.lu` pins the four rows the clause rules and leaves the
fifth alone. `denied` is the other edge: measured out of band on all
three lanes (a mode-000 directory, at bind and at dial), declared in both
signatures, and with no hermetic witness because there is no `chmod`
builtin.

F-0103 re-probed at `982f857` and unmoved, predicted from a span with no
`.rs` in it. f18/f19 are character-identical but for a loop bound: the
one whose row is taken is `unsupported — control flow in an argument` at
`mem` on the checked lane, and the one that does not take it runs on all
three. wolf-lang#201 stays OPEN and the sharpened warning is repeated on
its thread. Residues at their tenth pin: the chars-pairs tuple list and
F-0096, both refused verbatim; `in(r)`'s wolf-rung reading is
re-confirmed, and lupin's wording moved (a better sentence, same verdict,
recorded so the next re-probe compares against the right string); a `str`
still charges no named region's ledger on any tier.

F-0099 re-counted at four namespaces, and the bill came due. The pinned
registry publishes 417 anchors in eleven namespaces; `[conf.anchor.ns]`
admits seven. `os`, `type`, `ct` and `diag` are outside that seven, so
this sprint implements `[os.net.unix]` and none of its six witnesses can
cite the clause it conforms to. Filed upstream as wolf-lang#239 on
the #120 precedent; the rig now pins the gap as a unit test whose
failure message says to retire F-0099. One lag in the other direction is fixed
here: `test` was reserved by the clause at s39 and this rig's
`FORWARD_NS` never followed.

`cargo xtask ci`: GREEN, exit 0 over the committed tree. 382 tests
(376 -> 382), 719 forward tags, 204 conservatism entries, 0 unstable, 0
slow, 8 divergent (the same eight); doc-examples 414 blocks GREEN, +0,
a designed zero, because §4's one-module note leaves this module prose
examples and the reason is on both functions; ulp 200 rows GREEN on all
three lanes.

## sc35 — 2026-09-03 — the rename, the second gate, and the release that opened it

Second bump, same day. lupin 0.1.23 -> 0.1.25 (is36 ships `[type.byte]`
in the mirror) and wolf `0.2.3+dev.4230b00` -> the v0.2.4 tag
(`982f857`), data pin with it, five commits whose diff touches eight
files and no `.rs` at all. Drift was predicted at zero from the wolf half
over a span with no compiler in it, and measured zero: anchors 417 ->
417, +0/-0 with the vendored snapshot byte-identical (the first bump here
where only `PIN` moves), corpus 511 -> 511.

All three pins read one sha for the first time in this repository's
history: wolf's, lupin's and `vendor/upstream/PIN` are all `982f857`.
sc30 pulled them apart, sc33 suspended the invariant, sc34 restored it
between the binary and the data pin, and this sprint adds the
interpreter. The dev stamp retires with it: v0.2.4 is the first tag that
carries s135 and s136 together, so the `+dev.<commit>` mechanism r03
built for this gap lasted as long as the gap did. wolf's pairing line
still names `lupin 0.1.24 … pin 3befc3e` (r07's release-time pairing
record) against an installed 0.1.25, reported and not gated (F-0064).

F-0108 closes one day after filing, by the release it named as its own
exit. 181 of 181 dark rows returned (122 exit / 48 unsupported / 11
trap), 87 of 87 doc blocks returned, zero residual refusals, and not one
lane word in `tests/ledger.toml` had to be corrected for any of them. A
ledger left wrong on purpose, with its own block note naming the count
and the cause, needed no repair when the cause left.

What remains is eight rows with a legal word. lupin 0.1.25 has the byte
type without the byte domain, so `push(256)` into a `List[byte]` still
stores 256, and where both compilers refuse at typecheck the interpreter
runs the program. That is `divergent(…)`, F-0098's word, used outside the
take-mode pair for the first time: five `divergent(exit(0))`, two
`divergent(exit(1))`, one `divergent(trap(assert))`, plus two ordinary
`unsupported`. Filed as wolf-interp#62 (is37's). `hex`'s `byte_digits`
assert is the one to read twice: unreachable on both wolf lanes and still
firing under lupin, so a sentence that was true of the compilers and
false of the reference machine now lives in the divergence ledger instead
of in an unchecked claim.

`cargo xtask ci`: GREEN, exit 0. 376 tests, 700 forward tags, 201
conservatism entries, 0 unstable, 0 slow, 8 divergent; doc-examples
414 blocks GREEN; ulp 200 rows GREEN on all three lanes.

## sc35 — 2026-09-03 — the rename, and the second gate

Binary and data pin move together, 31170d1 -> 4230b00 (20 commits),
`wolf 0.2.3+dev.4230b00 (wolfgang, pin 4230b00)`, the one-sha invariant
held. lupin stays at 0.1.23, now 71 commits behind wolf's own pin.
Anchors 415 -> 417 (+2/-0, `os.net` and `os.net.unix`, #227's, key sets
diffed both ways with nothing dropped); corpus 499 -> 511.

This is the first bump in this repository's history whose drift
prediction was not a zero, and the non-zero was expected. s136
(wolf-lang#231) moves the eight byte builtins (`str.bytes()`,
`str_from_utf8`, `fs_read_bytes`/`write_bytes`/`read_chunk`/`write_chunk`,
`net_read_bytes`/`write_bytes`) from `List[int]` to `List[byte]`, and
every one of std's sixteen byte-tier functions is a thin wrapper over
one of them, so the pin refuses the library that was written against
the old signatures. 198 sites / 45 files were predicted from #231's own
count; 149 E0401 sites over 35 files were measured at the bump, and 910
sites over 172 files once the modules were repaired and the tests could
be reached. The two numbers answer different questions; the first is a
floor, because a refusal inside a module aborts the program that imports
it before the compiler can see the test's own sites.

The rename covers 233 signatures across 19 std modules and 166 test
files. `std.bytes`, `std.fs` and `std.net` are pure renames, with no
executable cast entering them, because `read_bytes` is still
`fs_read_bytes(path)?`, `from_str` is still `s.bytes()` and `to_str` is
still `str_from_utf8(b)?`, as F-0106 said they would be. The 430
`as byte` and 225 `as int` casts the rename spells are concentrated in
the modules that do arithmetic on octets (`p256` 176, `curve25519` 72,
`x.tls.client` 49), where they are `[type.byte.op]` making an operator's
result type explicit and converting no representation; `x.tls.cert`, a
DER parser of 15 signatures, took none at all. Five private
`require_byte` ingestion guards are deleted, four of them the module's
one recorded constant-time exception carrying the words "it leaves with
F-0035's real byte type" in its own header: the digest, cipher and ladder
paths now have no value-dependent branch, a constant-time improvement the
type paid for.

The headline: 16.0x -> 1.00x, on both tiers, at the io sites. Measured
through `std.fs` and not synthetically: write a payload, read it back
inside a fresh region, read `budget.charged(r)`:

| payload | before (checked / native) | after (checked / native) |
|---|---|---|
| 1,024 | 16,384 / 16,368 | **1,024 / 1,072** |
| 4,096 | 65,536 / 65,520 | **4,096 / 4,144** |
| 16,384 | 262,144 / 262,128 | **16,384 / 16,432** |
| **65,536** | **1,048,576 / 1,048,560** | **65,536 / 65,584** |

Better than the contract's own 2.0x prediction, and the reason is s136's
and not this library's: a producer mints at exact capacity through one
memcpy, so it pays no growth history, and the native residue is
`payload + 48`, one list header, constant from 1 KiB to 64 KiB. A list
the program grows by `push` still costs `2 x payload +
48` natively and
the payload under `--checked`, so `std.mem.budget`'s 16x caveat
retires
to one sentence: read a buffer with a producer and you pay the payload;
build one by pushing and you pay it twice on the tier that ships.

F-0104 and F-0106 close. Every prediction F-0106 made about the
substitution-after-the-builtins-move holds, including the ones about
call-site shape. F-0107 closes too: a consumed `for b in s.bytes()` walk
over 65,536 bytes now charges 0 on all three lanes, where the checked
machine charged 1,048,576 (wolf-lang#232 paid).

F-0108, and the rename cannot merge yet. `lupin 0.1.23` refuses
`as byte` with `fail(E0301)` at `resolve`, so its byte tier never
narrows. Measured over the renamed tree: 181 of 376 rows answer
`fail(E0301)` under lupin (134 ledgered `run`, 47 ledgered
`unsupported`), plus 87 of the tree's 414 doc-example blocks. Three
independent gates in this repository refuse to record that and all
three are right: the ledger has no lupin word for a static rejection,
and `divergent(…)`'s vocabulary runs one way only, a word for "the
compilers reject and the interpreter runs" with no mirror;
`doc-examples` rules that a static rejection on the reference machine
is a doc bug, and its waiver list waives an `unsupported` verdict and not
a refusal; and §9/§12's three-lane parity is what both exist to enforce.
Nothing is bent. The lupin ledger column is left alone with the
count, the cause and the module breakdown in capitals at the top of the
file, and the branch is gated on lupin 0.1.25 (is36's deliverable,
not tagged when this was written). sc34 refused this change because the
producers were missing; this sprint makes it and finds the second gate.

Ten witnesses moved with their contract, on the sc28 precedent: seven
`non_byte_trap.lu` files, `hex/encode_non_byte_trap.lu` and the two
`invalid` row witnesses pinned a runtime consequence of the 0..255
element contract that the type now holds, so each keeps its program,
moves its directive to `fail(E0401)` at `typecheck`, and drops the part
of its name that promised the old outcome (`…_trap.lu` ->
`…_refused.lu`). `invalid` stays declared on both byte writes, #231's own
posture adopted verbatim: the vocabulary is stable, an FFI caller's
wrong-width list still earns it, and typed code can no longer reach it.

F-0103 is re-characterised, and four sprints of "minimal shape" were
probe artifacts. Two programs differing only in a `while` loop's bound,
neither passing a literal, split: the one whose row is taken is
`unsupported — control flow in an argument` at `mem`, and the other runs.
The callee consuming the row (sc34's sharpening), the genericity and the
module boundary are all irrelevant; sc34's own control probe refuses once
its argument raises. The checked lane's refusal follows a path and not a
shape, which makes `unsupported` a property of an execution, so a corpus
that never takes the row reports the lane green for code it cannot run.
Posted to wolf-lang#201.

One upstream crash found and filed: a compile diagnostic on a very
long source line panics the wolf driver in its human renderer
(`wolf_diag::render::render_line`, `str::repeat` capacity overflow), so
the process dies with exit 101 and emits no record. It cost this sprint
a silently-skipped row in two scans, because a panic prints no
`error[` line for a scanner to find.

Residues, re-probed at `4230b00`: the chars-pairs tuple list is refused
at its ninth consecutive pin; F-0096 (`s.get(0..^2)`) verbatim;
`in(r)` unmoved (with a correction to sc34's reading of which arm
answers); `reserve(n)` unmoved and now priced at 65,536 ledger units on
a 64 KiB native buffer; a `str` still charges no named region's ledger
on any tier.

## sc34 — 2026-09-02 — the byte tier is bytes, and it cannot be yet

The wolf binary advances 51 commits, the largest span this repo has
crossed in one bump, to a dev-stamped trunk build,
`wolf 0.2.3+dev.31170d1 (wolfgang, pin 31170d1)`, and the data pin comes
back to meet it at the same sha, restoring the one-sha invariant sc33
suspended. The version stays 0.2.3 (r07 moves the tag), and the binary is
a dev build instead of the v0.2.3 tag because the tag cannot compile a
byte: `v0.2.3` = `3befc3e` sits twelve commits before s135, and its WIR
lowering refuses `Prim::Byte`, confirmed on the installed tag build
before it was replaced. lupin stays at 0.1.23, whose conformance pin is
now 51 commits behind wolf's own. Drift was predicted at zero and
measured zero over 376x3, the fourth consecutive empty drift list and the
first defended against a compiler that moved: anchors 411 -> 415 (+4/-0,
the `[type.byte]` family, key sets diffed both ways), corpus 490 -> 499.
Every `unsupported` record in the tree gained wolf-lang#219's
`x-unsupported-construct`/`x-unsupported-span` keys, 124 records changed
shape, and none could move a row, because this rig's record parser reads
a closed key list.

D72's `byte` is in the language, this library measured what it is worth,
and the signatures stay as they are. A 64 KiB buffer as `List[byte]`
charges 131,120 native and 65,536 checked where the same buffer as
`List[int]` charges 1,048,560 and 1,048,576: 16.0x -> 2.0x native,
16.0x -> 1.0x checked, linear at every size from 1 KiB, with native's
residue at `2 x payload + 48` (one list header), the push-growth history
that is #203's separable second half. F-0104 closes with that
after-table. What stays open is the library's ability to spend it: s135
gave the language a byte type and no byte-typed builtin. `s.bytes()`,
`str_from_utf8` and all six `fs`/`net` byte builtins are still declared
over `List[int]`, and every one of std's sixteen byte-tier functions is a
thin wrapper over one of them, so a substituted signature would have to
convert elementwise against a builtin, and with a cumulative ledger the
intermediate stays charged: a substituted `fs.read_bytes` measures 17.0x
checked / 18.0x native, worse than the 16.0x it replaces, at every size,
at the io sites the ask was filed about. Nothing is worked around; the
sixteen signatures keep their form so the change stays a rename. Filed as
wolf-lang#231: move the eight builtin signatures and the substitution is
the rename it was designed to be.

Two findings and two closures. F-0106 is the producers gap above. F-0107
(wolf-lang#232): the checked machine charges 1,048,576, 16x the payload,
for a consumed `s.bytes()` walk that allocates nothing, where native and
lupin both charge 0. That is why the one substitution with a real native
win (`bytes.from_str` as a walk, 131,120 natively) regresses to 1,114,112
under `--checked`, and it makes a region cap mis-fire between tiers on
the idiom `std.bytes` teaches for byte walking. F-0105 closes: D71/#220
landed in this span and its reproducer now reads `[83,84]` on all three
lanes, with the zero-width wolfc span gone. F-0103 is re-probed against a
moving compiler for the first time and is unmoved, and the probe got
sharper: the checked tier's `control flow in an argument` needs the row
consumed in the callee and not merely passed, so a callee that ignores
its row-typed parameter runs on every lane and reports a false heal.

Residues re-probed at the new pin: the chars-pairs tuple list refuses at
its eighth consecutive pin, F-0096 refuses verbatim, `List[int].in(r)`
is the sc33 string unchanged, and a `str` still charges no named
region's ledger on any tier.

## sc33 — 2026-09-02 — the bytes get a width

lupin advances to v0.1.23 at conformance pin 8cda3aa (is34, THE
LETTERS IN THE MIRROR), and its pin catches up to wolf's own: sc32's
35-commit gap, the largest this repo had recorded, closes to zero.
The wolf binary does not move (r06 takes it to v0.2.3), so for the
first time the three pins come apart on purpose. The data pin advances
to wolf-lang trunk 813153e, 19 commits ahead of both binaries,
suspending the one-sha invariant. That costs nothing, for a structural
reason: doctor never reads `vendor/upstream/PIN` (it gates the binary's
self-declared version and pin against `vendor/tools.toml`) and
`sync-pin` gates the snapshot against the submodule. Both were read out
of the gates' source before the run and confirmed by a green, silent
doctor. The 0.1.22 doctor pin retires. Drift was predicted at zero and
measured zero over 376x3, the third consecutive empty drift list, with
anchors 411 unmoved and `anchors.json` byte-identical across the span
(the re-vendor moved no bytes; F-0100's both-ways key-set diff was a
formality this time, and said so out loud).

The drift prediction's content was a number and a mechanism that
disagree. wolf-interp#55 puts trap-path stdout in lupin's records, so
the contract asked which std rows move. Eleven rows trap after printing,
grepped over all 56 trap files, read to confirm the print precedes the
trapping call, and checked against the ledger to confirm the lane runs
them (three more contain a `print(` that sits *after* the trap and never
executes). All eleven records changed shape and zero rows moved, because
this rig never looks at a trap's stdout in three independent places:
`classify`'s Trap arm discards the field by pattern, `diff_class`
compares `stdout_sha256` only under `Verdict::Exit(_)`, and lint R3 bars
`stdout=` beside a trap expectation outright. Measured on both sides of
the bump: at 0.1.22 lupin reported null where both wolf lanes already
carried `5726e3cf…`; at 0.1.23 it joins with the byte-identical digest.
The asymmetry was always lupin's alone, invisible because the comparator
does not look. #209's root-defer divergence heals with the same zero
effect (one executable `defer` in the tree, on no trapping path), and #56
is diagnostic wording, outside D22's protocol.

wolf-lang#203's ask is filed as a spec-shaped proposal
(`#issuecomment-5509341730`), with the evidence and no std wrapper, since
sc32 measured that one changes no allocation. The io readers are measured
for the first time and reproduce the synthetic 16x to the byte, with one
sharp new result: `fs.read_chunk(f, n)` charges what the unbounded
`read_bytes` charges, so #203's preallocation property is *entirely*
untaken on the one surface that already knows its bound, a 2x sitting
unclaimed behind no new type at all. The proposal's spine is that there
is no width story to extend: the spec has no type inventory, no
`[type.int]`, no width vocabulary, no literal suffixes, and `int` carries
no defining clause anywhere. Both cheap answers fail by the same
mechanism, the std wrapper by measurement and the spec's own `distinct`
newtype by its own clause ("same layout as the base"), so the
recommendation is `[type.byte]` modelled on `[type.char]`, the spec's one
existing scalar-width clause, finishing the job s121 started when it
wrote "`char` is the scalar tier, never a byte". A stale count in F-0104
is corrected in passing: `std.bytes` has ten public functions and not
nine, all still monomorphic over `List[int]`.

F-0105 filed: wolfc's zero-width parse span (DIV-2026-020, ruled as D71)
is reachable from ordinary std-side code, beyond the eight upstream
`grammar/` files. It turned up by accident while re-probing the `strbuf`
placement residue, and it confirms from a second independent rig why
nothing measured it (this runner compares codes, never spans). Recorded
on #220.

F-0103 re-measured verbatim and not adopted, since #201 has not ruled,
with the caveat that this bump's "unmoved" is cheap, because the wolf
binary did not move and the probe could not have. The checked tier's
`breach_is_a_row` flip is deferred: s134's item 1 has not merged (trunk
unmoved at 813153e and #219 still OPEN at both gauntlets), so the row
keeps its two-lane reason, with a lead left for s134 on the
`wolf run --checked` half of its bisection. Residues re-dated: the
chars-pairs tuple list refuses at its seventh consecutive pin, F-0096
verbatim, and the `str`-charges-no-region finding was re-probed instead
of carried, because the lupin binary moved, so the tier that could have
changed its answer is the one that got a new build; it still reads 0 on
all three lanes.

## sc32 — 2026-09-02 — the budget has a shape

Pins advance to wolf v0.2.2 at 8cda3aa (THE LEARNERS' RELEASE) and
lupin v0.1.22 at conformance pin 2bfbe5e, both real tags, both
`--version`-bare. The span is 35 commits, the largest this repo has
crossed in one bump, and the gap between the two pins is named: a
windows native bring-up, an LSP navigation trio and four letters, none
of it lowering debt. Drift was predicted at zero and measured zero over
373x3, the second consecutive empty drift list, with anchors 404 -> 411
(+7: `mem.region.account{,.1,.2}` and `mem.region.cap{,.1,.2,.3}`) in the
first re-vendor to move bytes since sc27. The 0.1.20 doctor pin retires.

`std.mem.budget` lands. `charged(r)` and `live()` are the region ledger's
two queries (three lanes, including a `region` passed across a module
boundary; a `read` parameter retains an affine value, and that took a
probe to know), and `with_cap(n, f)` collapses D68's containment join,
spawn, monitor, `select` and `is_alloc_contract()`, into one call whose
failure is the ordinary row `exhausted`. That is the shape lobo's
per-request 503 consumes. Fifteen probes ran before a line of the module
was written and five of them changed it: a region may be taken and never
returned (native refuses `-> region`); the work's value cannot come back
at all (a channel in a std signature is refused on both wolf rungs); the
checked tier's C1 refusal is reached at execution and not statically (so
one function yields two different checked columns across three
witnesses); and the trap-shaped runner is not shipped because it is
`region r(cap: n)` with a library in the way. The row carries no payload
because `[mem.region.cap.3]`'s free-then-deliver teardown makes the dead
proc's charge unobservable by contract; a negative budget traps at the
door, so a caller's arithmetic mistake is answered with a trap and not
with a recoverable value.

wolf-lang#203's evidence is measured and written up (F-0104) instead of
built: a `List[int]` byte buffer charges 16x its payload on both wolf
tiers at every size from 1 KiB to 64 KiB, reproducing lobo's numbers to
the byte from a different program, and 32x under lupin, a multiplier the
issue does not carry. A fourth measurement rides along: a `str` charges
no named region's ledger on any tier, where `[mem.region.account.1]`
scopes that gap to the native one. The recommendation is a language
byte-width element type behind std's already-documented `Bytes`, because
every byte signature in std is monomorphic over `List[int]` today and
keeps its shape when it lands.

F-0103 re-measured verbatim and not adopted: wolf-lang#201 has not ruled,
and nothing in 35 commits touches `mem`'s argument handling, so
`bind, then name` stands. Residues re-dated: the chars-pairs tuple list
refuses at its sixth consecutive pin, F-0096 verbatim, and `strbuf.in(r)`
was re-probed for the first time instead of argued, because the span
moved regions, so the placement syntax was measured (absent on every
lane, `fail(E0201)` at parse for the struct form) instead of reasoned
from the commit list.

## sc31 — 2026-09-01 — the row gets a name

Pins advance to wolf v0.2.1 at 75fd2d0 (a real release tag again, so the
sc30 dev stamp retires and `wolf --version` answers bare) and lupin
v0.1.20 at conformance pin b80d239, four commits behind the data pin
with the gap named: r04's four measured letters. Drift was predicted at
zero and measured zero over 372x3, the first sc bump whose drift list
came back empty, with anchors held at 404.

`std.x.tls.client` answers its first consumer's ask (wolf-std#3):
`named` coarsens the module's twenty-row vocabulary into one
payload-carrying tag whose payload is the refusing row's own name, so
a caller writes one handler arm instead of twenty and never forges a
dead `Client`; `row_name` is its marking face. The call-site spelling
is `bind, then name`, and the module header says why. Adopted at the
negative battery's three naming sites with byte-identical stdout on
all three lanes. F-0103 filed (wolf-lang#201): the checked tier
refuses a raising call passed straight into a row-typed parameter
where lupin and the native rung both run it, the long-unexplained
cause of three `std.option` ledger rows. The client's lane note now
states what a handshake costs at unoptimized tiers: seconds, not
milliseconds.

## sc30 — 2026-08-31 — the slice comes home

Pins advance to wolf b80d239 (the s129/s130 merges; a dev-stamped
build whose `+dev.b80d239` identity carries the pin clause doctor
gates, since no release tag exists past v0.2.0 and D57's answer is the
dev brand) and lupin v0.1.19 at conformance pin 83f83bb, one
merge behind the data pin with the gap named. Drift predicted two
movers and measured three, all lupin, all deeper: the sc29 byte-tier
rows go three-lane (F-0102 paid as filed) and
`loopback_handshake`'s lupin lane runs the full TLS 1.3 handshake
inside the 50M step budget, where sc29's "and the step budget" was an
inference the resolve refusal had shadowed, and the measurement
outvoted it. F-0101 closes the sc28 arc: `bytes.slice` re-adopts
`b[from..to]` (the retreat commit reverses) and the one row the sc28
adoption moved holds `run` on every lane with the range spelling,
found, filed, fixed and re-adopted. Struct patterns adopted where the
struct is born: 14 patterns at 8 files, probe-proven compositions
first, ledger flat throughout. The chars-pairs tuple list refused at
its fourth consecutive pin; F-0096 verbatim; anchors 404
(+`gram.pat.struct`).

## sc29 — 2026-08-31 — the client shakes the hand

std.x.tls.client lands: a TLS 1.3 client over the library's proven
halves, two-phase begin/complete for the single-threaded reality,
CertificateVerify verifies (ed25519 + ecdsa-p256; sc21's
verify-nothing retires), the eleven-shape negative battery refuses with
the row named, and the loopback flagship shakes hands with a server half
built from the same primitives. std.net gains `read_bytes`/`write_bytes`
and the deadline pair, making F-0049's `timeout` tag reachable.
F-0102 filed (wolf-interp#52): the byte tier dark under lupin 0.1.18.

## sc28 — 2026-08-30 — the library writes the new words

Pins advance to wolf v0.2.0 and lupin 0.1.18, both D57-bare release
builds, and doctor now gates wolf's provenance from `--version` itself,
so a binary naming no pin is a red. The four `divergent(…)` ledger rows
flip to `run` as 0.1.18's notes predicted. The c33/c39 surface
is adopted as well as measured: `str +` at 23 of 27 candidate sites,
tuple destructuring at 10 sites, comma-grouped binders, and list
slices in 12 loops at 9 sites, with one lend retreat filed instead of
papered over (F-0101, wolf-lang#184). `gram.lex.ident` returns to the
anchor registry; the ledger stays flat throughout.

## sc27 — 2026-08-30 — the divergences re-measure

Pins converge on wolf 0a5c1af and lupin 0.1.17; drift was predicted
zero/zero and measured zero/zero. Residue row 6 heals, with `str + str`
running on all three lanes (D62), and #50 healed with the sc22 cursor
boundary unmoved. The divergent four (wolf-interp#47/#48) stayed
unhealed at their third measurement, commented in place. Anchors +5,
clean; adoption candidates filed for the next contract and left
untaken.

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
std.unicode is retyped on `char` per D58: `from_code -> char !
{none}`, the predicates compare char literals, `code`/`from_code` are
the only int doors, and `code_points` stays int. The four
divergent rows re-measured unhealed; the `slow` word retires on the
arm64 rig's own refusal.

## sc24 — 2026-08-28 — the twenty-eight re-measured

F-0018's 28 boundary-blocked contracts, re-tested clause by clause
across two wolf acquisitions: 24 of 28 shipped, including three new
this sprint on the D58 char surface (`str.to_list_chars`,
`strbuf.push(char)`, `unicode.code(char)`), with a four-item residue
listed and owned. `^n` healed; F-0096–F-0099 filed; the `divergent(…)`
ledger word is born for rows whose wrongness is the counterparty's.
35 rows deeper, 25 honest-downs. Pins a900b8c + lupin 0.1.14.

## The TLS rungs — sc20 → sc23 (2026-08-27)

- sc23 — ECDSA-P256 as evidence: RFC 6979, Wycheproof
  secp256r1 (484 vectors) and CAVP 186-4 SigVer/SigGen vendored, every
  vector reproduced by a from-scratch big-int reference first; the
  emitters are native-only, since a P-256 verify is two table-free
  ladders, past the checked and lupin step budgets. The data-pin bump
  moved zero ledger rows.
- sc22 — the certificate: an X.690 DER decoder, RFC 5280 X.509
  profile, and Ed25519 chain validation over an in-repo test PKI; 25
  malformed inputs reject with the reason named, and an unsupported
  algorithm is named and never accepted. F-0079 found fixed upstream
  (lupin 0.1.13)
  and closed.
- sc21 — the handshake: message flow, transcript hash, the X25519
  key schedule, Finished, and the client state machine, RFC 8448-gated
  on all three lanes. The ws05 seam freezes.
- sc20 — the record layer: HKDF-Expand-Label, per-record nonce,
  TLSCiphertext seal/open over ChaCha20-Poly1305, RFC 8448-gated,
  three-lane. F-0094 and F-0095 (wolf-lang#139) filed.

## The crypto ladder — sc16 → sc19 (2026-08-26/27)

- sc19 — the ACME signature: JWS/EdDSA compact and flattened, OKP
  JWK with the RFC 7638 thumbprint, and the ACME request body
  (RFC 7515/8037/7638/8555), byte-exact against the RFC vectors; the
  ws06 seam freezes. The native/wolfc rows are the environment
  crypto-drift class; lupin green, CI-gated.
- sc18 — the curve and the signature: X25519 and Ed25519 from
  RFC 7748/8032 over a checked-int field, 669 Wycheproof vectors with
  every flag decided; the dark lane lights, 41 rows. Typed-int limb
  discipline kills the i32-literal default.
- sc17 — the cipher that needs no tables: ChaCha20-Poly1305 from
  RFC 8439, 353 vectors green; the tag compare marked for the
  constant-time tier.
- sc16 — the digest ladder: SHA-2, HMAC and HKDF written from the
  documents, 1210 vectors green; the `slow` ledger word is born for a
  lane whose semantics are right and whose clock is too slow.

## The text and json turn — sc12 → sc15 (2026-08-13 → 08-26)

- sc15 — the DOM half: std.json gains a checked handle over the
  declared json kernels, rows verbatim.
- sc14 (2026-08-24) — each word, without the list: `each_word` as
  the lazy walk and `words_count` as the counting fold.
- sc13 (2026-08-21) — the callable core, std's first higher-order
  tier: list predicates and relations as `fn` values; both F-0052
  wildcard handlers retire (the arms discriminate at lupin 0.1.13),
  the json F-0079 split retires and its witness catches the sequel
  (F-0084); F-0082–F-0086 filed from the sprint's probes.
- sc14 (2026-08-14) — the four owed contracts, all paid:
  `json.parse`, `json.unescape`, `escape`'s totality, and
  `hex.decode_str`.
- sc13 (2026-08-13) — the json DOM, and a way back from bytes:
  `bytes.to_str` lands on s81's `str_from_utf8` (F-0057 closed after
  four sprints of refusing an ASCII-only border post).
- sc12 (2026-08-13) — eight functions walk the s77 byte view
  instead of copying.

(The ids sc13 and sc14 were each used twice in the track's history;
the entries above are ordered by landing date.)

## The os surface — sc07 → sc11 (2026-08-11/12)

- sc11 — std.process: the pure builder over the process trio
  (`start` and `slot`, because spawn and handle were keywords,
  F-0062), the checked lane recorded as measured; the signal row joins
  the taxonomy;
  `io.input_all` and `net.read_all` rewritten on the now-legal
  re-raising loop; seven ledger rows flip on the s71/s72 rulings.
- sc10 — std.time (instant and duration facades, exact RFC 3339
  render, the Clock tag), std.env (argv arrives in std), and
  std.x.json, the nursery's first tenant, with its banner and
  three-outcome graduation clock.
- sc09 — the F-0018 prize spent: std.str grows 16 → 37 functions,
  std.bytes is born at 9, twenty census rows flip, and the first
  all-three-lane test block lands.
- sc08 — std.net: ten functions and two types over the s39
  builtins, the row vocabulary verbatim, take-consumed close;
  `read_all` written then withdrawn on the sprint's headline finding
  (F-0052, filed).
- sc07 — std.fs (15 functions plus File with take-consumed close)
  and std.io (the write family, `input_line`, `prompt`); `copy`
  renamed `copy_file` (reserved keyword), `read_line` renamed
  `input_line` (prelude shadowing recursed under lupin); ten blocked
  contracts, filed.

## The core library — sc01 → sc06 (2026-08-10/11)

- sc06 — std.option lands, six functions executing under lupin;
  errors at five and testing at thirteen with static-contract freight
  stated per function; the lowercase-tag rename (148 occurrences,
  zero ledger movement).
- sc05 — fmt and encoding: exact float formatting (half-even
  fixed and exp, genuinely shortest `to_str`, correctly rounded
  `parse_float`), hex and base64 over a documented 0..255 byte
  contract, json escaping with 22 bodies; `hex.encode(str)` stated
  impossible and left unbuilt.
- sc04 — math, sort, search: 71 of 74 contracts with bodies;
  pure-wolf transcendentals at measured ulp 1 (3 for `powf`); the
  pcg32 stream pinned and cross-checked.
- sc03 — strings and bytes: str 18 via the one safe primitive,
  strbuf 8, unicode 9 with the 25-entry case table pinned per code
  point; ascii-only find/split refused on principle; 28 contracts
  blocked on the missing boundary primitive (F-0018, the track's
  central filing).
- sc02 — collections: std.list 19 executing, std.map 13, pool
  reserve/init; set and deque generics held as reviewed contracts
  blocked on `struct[T]`.
- sc01 — cmp complete (Ordering with IEEE total order, Eq/Ord,
  min/max/clamp), testing five, iter prototypes; option blocked on
  row-position parsing, filed.

## sc00 — 2026-08-10 — the rig

The track opens: pins to the wolf-lang data snapshot and both
implementation binaries, an xtask `std-test` runner staging every
module through both implementations' `conform-run` with record
validation and bidirectional ledger enforcement, `doctor`,
`sync-pin`, the findings register (F-0001 on day one), and the
prelude proof module (`least`, `greatest`, `magnitude`) green under
both implementations with identical stdout hashes.
