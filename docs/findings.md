# Findings register

The track's house rule (sc00): a gap in what the language can express is
a **finding filed to wolf-lang**, never a workaround invented here. Every
finding gets a row; the filing link is the proof it left the building.

| id | filed | title | routed to | status |
|---|---|---|---|---|
| F-0001 | 2026-08-10 | No std search path: `use std.*` resolves against a builtin stub (wolfc) / builtin ambient (lupin), never the package; lupin resolves no nested package directory | wolf-lang (s37+ prelude/build plumbing), interp counterpart noted in the filing | [filed: wolf-lang#1](https://github.com/tenseleyFlow/wolf-lang/issues/1) |
| F-0002 | 2026-08-10 | (contract F2) Absence/result over rows: rows are second-class — `!`-rows parse only in return position, so the six `std.option` helpers cannot be written; `try`⇄`Result` blocked twice over (s16 generic data + reification); plus the pin-era execution floor (no trait/method/enum dynamics) | wolf-lang s15-revisit/s37 owners | [filed: wolf-lang#3](https://github.com/tenseleyFlow/wolf-lang/issues/3) |
| F-0003 | 2026-08-10 | Lowercase bare row tags (`none`, `done`, …) resolve in neither implementation at the raise site; API-CONVENTIONS §2 mandates lowercase pure marks — std spells `None`/`Done` interim | wolf-lang s15/resolve owners, interp counterpart | [filed: wolf-lang#4](https://github.com/tenseleyFlow/wolf-lang/issues/4) |
| F-0004 | 2026-08-10 | (contract F3) Operator↔trait bridge: `<=>` builtin-only, yields int (lupin); enum `==` refused by wolfc at resolve; no supertrait clause spells `Ord requires Eq`; bare literals default `i32` and miss `impl … for int` bounds (E0502) | wolf-lang s14/s17 owners | [filed: wolf-lang#5](https://github.com/tenseleyFlow/wolf-lang/issues/5) |
| F-0005 | 2026-08-10 | wolfc's pinned List typing: `push` typed as read receiver (X1-correct `(mut xs).push` is E0804; bare push then E1001); `.len`-then-`[i]` reads count as moves (E1001; fix-it names a `copy` operator outside the pinned grammar) — blocks `min_of`/`max_of` and every List-building wolfc lane | wolf-lang s21/s22 mem-tier owners | [filed: wolf-lang#6](https://github.com/tenseleyFlow/wolf-lang/issues/6) |
| F-0006 | 2026-08-10 | str ordering divergence: lupin runs `<`/`>` on str, wolfc rejects E0409; no byte accessor exists to define it in-library — blocks `impl Ord for str` | wolf-lang typecheck/spec owners | [filed: wolf-lang#7](https://github.com/tenseleyFlow/wolf-lang/issues/7) |
| F-0007 | 2026-08-10 | lupin pattern semantics: bare-ident patterns BIND instead of resolving in-scope variants/tags — first arm always matches (silently wrong), non-exhaustive matches run, qualified path patterns accepted though outside the pinned grammar | wolf-interp | [filed: wolf-interp#5](https://github.com/tenseleyFlow/wolf-interp/issues/5) |
| F-0008 | 2026-08-10 | (contract F1) The iterator protocol: `Iter[T]` with `next(mut self) -> T ! {done}`, the `for` desugar, range-type ownership, builtin adoption — prototyped executing in `std.iter` | wolf-lang spec/01 + s37 owners | [filed: wolf-lang#8](https://github.com/tenseleyFlow/wolf-lang/issues/8) |
| F-0009 | 2026-08-10 | (contract F4) `assert`, defined: an intrinsic, not a prelude fn (a module fn named `assert` shadows it module-wide, E0402 both implementations); the two-argument form needs arity overloading or intrinsic growth; comptime/runtime duality | wolf-lang spec comptime/forward-std owners | [filed: wolf-lang#9](https://github.com/tenseleyFlow/wolf-lang/issues/9) |

| F-0010 | 2026-08-11 | lupin has no std root: `use std.X` resolves nothing against a package `std/` tree (it binds the path's LAST segment and looks for a top-level package directory) — the interpreter half of F-0001, still open after wolfc closed its half with `--std-root`/`WOLF_STD`; the rig stages a flat mirror beside the real tree | wolf-interp | [filed: wolf-interp#6](https://github.com/tenseleyFlow/wolf-interp/issues/6) |
| F-0011 | 2026-08-11 | (contract Target-1) **What is builtin and what is std**: recommendation + four gaps — `List`/`Pool` language-blessed core types with std owning their API mass; `Map`/`Set`/`Deque` std-defined once s16 lands generic data types (`struct X[T]` is E0201 in BOTH implementations today); the `Hash`/`Eq` key protocol is unspecified; `Pool` exposes no `len`/`capacity`/liveness probe/iteration; `Map` cannot erase a key; the wordcount `tally[w] += 1` idiom runs nowhere (lupin: index is not a place; absent key reads `()`, not a zero) | wolf-lang s16/s21/s37 owners | [filed: wolf-lang#11](https://github.com/tenseleyFlow/wolf-lang/issues/11) |
| F-0012 | 2026-08-11 | wolfc checked tier: an imported module whose items include a `trait`/`enum`/`impl` makes every importer `unsupported — module items in checked execution` — so `use std.cmp` costs a module every wolfc run row, which is why std.list's `Eq` family sits in `std/x/list_eq` | wolf-lang s23/s31 (checked execution) | [filed: wolf-lang#12](https://github.com/tenseleyFlow/wolf-lang/issues/12) |
| F-0013 | 2026-08-11 | lupin false `ub(mem.ub)` on std-shaped code, two shapes: (a) a `mut` argument inside an f-string interpolation leaves a stale borrow tag — a later read is "read through tag … which is Disabled"; (b) a `mut`-mode call followed by a read-mode call whose body ALLOCATES is "foreign write … while tag … is PROTECTED". Both are false positives in the provenance model; both are avoidable only by writing around them | wolf-interp (Tier-3 provenance) | [filed: wolf-interp#7](https://github.com/tenseleyFlow/wolf-interp/issues/7) |
| F-0014 | 2026-08-11 | Mutate-while-iterating does not agree: wolfc rejects statically as `fail(E1001)` (reads-as-moves, F-0005's lens — not an exclusivity rule), lupin RUNS it silently with no trap at all, where `[conf.trap.map]` predicts `exclusivity`. The sprint contract's designated triage case, recorded in `tests/list/mutate_while_iterating.lu` | wolf-lang s22 + wolf-interp | [filed: wolf-lang#15](https://github.com/tenseleyFlow/wolf-lang/issues/15) + [wolf-interp#9](https://github.com/tenseleyFlow/wolf-interp/issues/9) |
| F-0015 | 2026-08-11 | wolfc checked tier: the row RAISE path inside an imported module's function is `unsupported — module items in checked execution` — the same call runs when it yields a value, and the same body inlined into the entry file runs both ways. Every `! {None}` miss test in std.list/x.deque_int is `unsupported` for this reason alone | wolf-lang s23/s31 | [filed: wolf-lang#13](https://github.com/tenseleyFlow/wolf-lang/issues/13) |
| F-0016 | 2026-08-11 | `wolf fmt` splits a dotted call at the dot when a `//` comment line precedes it (`xs.push(1)` becomes `xs.` + newline + `push(1)`), idempotently — fmt is law (D34), so its output is the style, and this one is a defect | wolf-lang fmt owner | [filed: wolf-lang#14](https://github.com/tenseleyFlow/wolf-lang/issues/14) |
| F-0017 | 2026-08-11 | lupin still accepts `let` reassignment after wolf-lang#2 closed compiler-side (wolfc now says E0410 with a `var` fix-it): the interpreter half of the divergence is open | wolf-interp | [filed: wolf-interp#8](https://github.com/tenseleyFlow/wolf-interp/issues/8) |
| F-0018 | 2026-08-12 | (sc03 Targets 1–4) **The boundary primitive is missing**: no recoverable slice (`str.get`), no byte accessor, no `chars()`/`char` type — so no scan can advance past a code point of unknown length, and 28 of sc03's 64 functions are unwritable rather than merely unimplemented; plus `^n` resolving nowhere, `str` methods `unsupported` in wolfc, `str + str` diverging | wolf-lang s37 (core types) owners | [filed: wolf-lang#17](https://github.com/tenseleyFlow/wolf-lang/issues/17) |
| F-0019 | 2026-08-12 | (sc03 Target 4) **Decision request — the unicode tables budget**: where category/case/normalization/segmentation data lives (std recommends penumbra or `std/x`, never core), with the evidence that lupin's `lower`/`upper` ALREADY do simple Unicode case mapping and `trim`/`words` already use Unicode `White_Space` — so std had to carry a 25-entry table to agree with the builtin | wolf-lang spec/std owners | [filed: wolf-lang#18](https://github.com/tenseleyFlow/wolf-lang/issues/18) |
| F-0020 | 2026-08-12 | `assert(cond, msg)` traps `assert` even when `cond` HOLDS (wolfc 12ae8c2) — the two-argument intrinsic wolf-lang#9 just landed ignores its condition; the one-argument form is correct; contradicts `[conf.trap.assert]`'s "silent and effect-free when the condition holds" | wolf-lang (the #9 implementation) | [filed: wolf-lang#19](https://github.com/tenseleyFlow/wolf-lang/issues/19) |
| F-0021 | 2026-08-12 | lupin: a method call on a SLICE EXPRESSION over a binding (`d[0..1].upper()`) is `unsupported` at resolve ("does not denote a place at run time") while the same shape over a literal runs — every std.str body binds the slice first | wolf-interp | [filed: wolf-interp#10](https://github.com/tenseleyFlow/wolf-interp/issues/10) |
| F-0022 | 2026-08-12 | lupin: `n as f64` does not convert — the value stays an int, compares equal to ints and unequal to floats, and no diagnostic appears (wolfc correctly refuses the mixed comparison E0401); `std.str.parse_float` ships a ten-branch digit→f64 table to avoid it | wolf-interp | [filed: wolf-interp#11](https://github.com/tenseleyFlow/wolf-interp/issues/11) |
| F-0023 | 2026-08-12 | lupin, the interpreter half of wolf-lang#3 and #4 (both closed compiler-side): postfix rows in param/`let` positions are E0201, lowercase bare tags do not resolve at raise sites — **and tag resolution is LAZY**, so a `return none` on an untaken branch certifies falsely, which is how sc02's F-0003 update came to be wrong | wolf-interp | [filed: wolf-interp#12](https://github.com/tenseleyFlow/wolf-interp/issues/12) |
| F-0024 | 2026-08-12 | `cargo test --workspace` is deterministically RED at trunk `12ae8c2` (wolf_parse `blast_radius`: 4 added diagnostics, max 3, on `corpus/comptime/norm_witness.lu`) while that sha's trunk CI run reports success — so "the last green trunk run" is not by itself a sufficient pin criterion | wolf-lang CI/parser owners | [filed: wolf-lang#20](https://github.com/tenseleyFlow/wolf-lang/issues/20) |

## F-0001 — the std search path

Evidence, both implementations at the sc00 pins (wolfc f1f9d80, lupin
0.1.0/cbde620), staged package root containing `std/prelude/prelude.lu`
with a `pub fn`, entry `use std.prelude`:

- **wolfc** routes any `use` whose head segment is `std` to
  `resolve_std_use` -> the builtin stub table (`prelude.rs`
  `STD_MODULES`: only `std.fs.read_text`); the package's `std/` directory
  is never consulted -> `fail(E0301)`. Nested non-std directories DO
  resolve (`use outer.inner` binds `inner` and runs to `exit(0)`).
- **lupin** treats `std.fs` as builtin-ambient (wordcount.lu reaches
  run) but resolves no nested package directory at all: `use outer.inner`
  -> `unsupported` at resolve.

Interim (documented in `xtask/src/stage.rs` and the tests themselves):
the rig stages `std/<mod>/` as `<mod>/` in the scratch package root and
tests import bare module names. The repository tree remains the
namespace (D32); only the staged spelling is interim. When the search
path lands upstream, tests flip to `use std.<mod>` in the same commit as
the pin bump.

RETIRED, compiler half (sc02, pin bump to wolf-lang trunk `a0c4564`).
s26 landed real std resolution — `--std-root <dir>` with a `WOLF_STD`
fallback through the normal loader, the `STD_MODULES` stub retired,
nested paths (`std.x.deque_int`) resolving, diagnostics displaying the
location-free `std://` scheme — and wolf-lang#1 is closed. This repo's
half of the retirement, in this sprint:

- `xtask/src/stage.rs` stages the tree AS `std/` and the runner passes
  `--std-root <scratch>/std` on the wolf lane;
- every test and every std-internal import now writes the real
  `use std.<mod>` (the doc-example extractor generates it too);
- what remains is the LUPIN half, filed as F-0010: lupin has no std
  root, so staging additionally mirrors each module directory flat
  under its last segment. That mirror is the only interim left, it is
  documented in `stage.rs`, and it dies with the pin bump that closes
  wolf-interp#6.

## F-0002 — absence/result over rows (contract F2)

UPDATE (sc03 pin `12ae8c2`): the GRAMMAR half is closed — wolf-lang#3
landed postfix rows in every type position (`type '!' error_row`), and
wolfc now carries `fn or(v: int ! {none}, d: int)` cleanly through mem.
lupin does not (E0201 at parse, F-0023 / wolf-interp#12), and lupin is the
only implementation that EXECUTES these shapes, so `std.option`'s six
helpers stay unwritten and this finding stays open on the interpreter's
half. Retested at the new pin, cheaply, exactly as the sprint asked.

Evidence, both implementations at the sc01 pins: a `!`-row in a
parameter or `let` type is a parse error — `fn or(v: int ! {None},
d: int)` → E0201 `expected ')', found '!'` (wolfc and lupin agree; the
pinned grammar's `param`/`let_item` productions admit no `!`). Absence
values therefore cannot be passed, stored, or named: every one of the
contract's six helpers (`or`, `expect`, `flatten`, `to_list`, `exists`,
`is_none`) is unwritable, and nested rows (`flatten`'s domain) doubly
so. `std.option` ships the convention and the six idioms as run-rung
tests; the module doc carries the reviewed signatures that land when
either (a) rows become reifiable value types, or (b) s16's generic-data
thaw gives `Option[T]`/`Result[T, E]` plus the s15 `try` bridge.
Amendment candidate: decide (a) vs (b) — std needs exactly one.
Also recorded here (the execution floor the prototypes met): lupin
executes no user methods, no trait dispatch, no type-path expressions
("no dynamic semantics here"); wolfc's checked tier refuses rows,
enums, and trait dispatch at the run rung after a clean static ladder.

## F-0003 — lowercase bare tags

UPDATE (sc03 pin `12ae8c2`): **the halves have SWAPPED, and sc02's
update above was wrong.** wolf-lang#4 is closed — wolfc now resolves
`return none` under `-> int ! {none}` (clean through mem) — and lupin
does NOT: `unsupported: `none` does not resolve`. std therefore keeps the
`None`/`Done`/`Parse` interim for the same reason as before, against the
other implementation.

sc02's claim that lupin 0.1.1 resolved lowercase tags was a
false positive with a mechanism worth remembering: **lupin resolves row
tags lazily, at run time, on the taken path only.** A `return none` on a
branch the program never enters produces no diagnostic, so a probe that
exercises only the hit path certifies a raise site that does not work.
Recorded in F-0023 and filed (wolf-interp#12); the guide's learnings
carry the rule.

RETIRED (sc02 pins, superseded above): lupin 0.1.1 was reported to
resolve a lowercase bare tag at the raise site. It does not.

API-CONVENTIONS §2: payload-free marks are lowercase (`none`, `eof`,
`gone`); `[mem.shared.rc.3]` already writes `T ! {gone}`. At the pins,
`return none` under `-> int ! {none}` is E0301 (wolfc, "nothing named
`none` in scope") and `unsupported: `none` does not resolve` (lupin) —
CapCase `None` resolves in lupin and parses everywhere. The raise-site
resolution of row tags needs to consult the declared row before the
value namespace, or the convention needs amending. Interim: std spells
`None`/`Done`, flipped repo-wide on the fixing pin bump.

## F-0004 — the operator↔trait bridge (contract F3)

Does `<=>` desugar to `Ord.cmp` and `==` to `Eq.eq` for user types?
Today: `1 <=> 2` evaluates to `-1` (an int) in lupin and is
`unsupported` in wolfc's checked tier; `==` on a user enum runs in
lupin but is refused by wolfc at RESOLVE (one occurrence anywhere in a
module poisons every importer — std.cmp writes match-based bodies to
stay wolfc-clean). Three adjacent gaps ride along: (1) no supertrait
clause exists, so `Ord requires Eq` is documentary; (2) bare integer
literals default `i32` and fail bounds against `impl … for int`
(E0502) — call sites need `let a: int = 3` bindings; (3) std types
want the bridge badly — `total_cmp` results can only be *compared*
today via enum `==`, which only lupin runs. Proposed clause: the
bridge desugars operators to the trait calls for types with in-scope
impls, `<=>` yields `std.cmp.Ordering`, and enum structural `==` is
defined language-side.

## F-0005 — wolfc's pinned List typing

**RETIRED (sc03 pin bump to trunk `12ae8c2`, wolf-lang#6 closed.)** The
fix inverted which spelling is legal: `(mut xs).push(v)` — the X1-correct
one — is now the ONLY accepted form in wolfc (`xs.push(v)` is E0804 with
a fix-it naming `(mut …)`), lupin accepts it too, and `.len`-then-`[i]`
is a copy-read rather than a move. This repo's half of the retirement, in
sc03: every std body and every test writes the mode-spelled receiver (26
call sites across `std.list`, `std.map`, `std.pool`, `std.x.deque_int`,
`std.x.list_eq` and 13 test files), `std.list`'s `at` helper and the
single-read rule it existed for are no longer load-bearing, three ledger
rows moved from `fail(E…)` to `unsupported` (nothing is REFUSED any
more), and the doc-example waiver list is empty for the first time.
The original evidence follows.


Three observations, one owner (s21 sema-builtin typing + s22 mem
tier): (1) `List.push` is typed with a READ receiver — the X1-correct
`(mut xs).push(v)` is rejected E0804 ("`push` reads its receiver");
(2) the bare spelling `xs.push(v)` is accepted and then CONSUMES `xs`
(next read E1001) — mutually exclusive with (1) leaving no legal
spelling; (3) reading `xs.len` then `xs[i]` counts as overlapping
moves even at `List[int]` (E1001, "`xs[_]` is part of `xs.len`"), and
the fix-it suggests a `copy` operator that is not in the pinned
grammar. Blocks: `cmp.min_of`/`cmp.max_of` (bodies held in the sc01
workbench), the wolfc lane of every List-building test, and the
`std.iter` list-cursor prototype's wolfc lane. lupin accepts both
spellings (its own under-enforcement of X1 is noted in the filing).

## F-0006 — str ordering

**RETIRED (sc03 pin bump, wolf-lang#7 closed).** The spec answered with
`[mem.str.order]`: the relational family on `str` × `str` is
byte-lexicographic over the UTF-8 bytes, unsigned byte compare, shorter
first on a shared prefix, total on all `str` values, with `==`/`!=` byte
equality and `<=>` yielding the same ordering value as on integers.
`[mem.str.impl]` draws the consequence this repo asked for: `impl Ord for
str` is shippable in-library with no bytes accessor.

Observed at the new pin: `"a" < "b"` no longer REJECTS in wolfc — it
passes the static ladder and stops at the checked tier's `unsupported`
(the ordinary `str`-method ceiling, F-0018) — and runs in lupin.
`tests/str/affix_and_case.lu` pins the ruling's observable content on the
lupin lane: `"wolf" < "wolves"` (shared prefix), `"wolf" < "wolf!"`,
`"z" < "é"` and `"é" < "🐺"` (every multi-byte code point sorts above
every ASCII one), and `"" < "a"`.

The `impl Ord for str` block itself belongs to `std.cmp` — sc01's
inventory — and lands in that module's own commit rather than here: it
would be trait-dispatch freight (`unsupported` on both lanes, F-0002 /
F-0012) and sc03's contract does not own std.cmp's surface. Recorded as
unblocked-and-available for the stdc01 closeout.

Original evidence: `"a" < "b"` ran in lupin (byte order) and was rejected
E0409 by wolfc; no str byte accessor existed to define ordering
in-library, so `impl Ord for str` was unshippable without poisoning every
wolfc importer of std.cmp.

## F-0007 — lupin pattern semantics (filed to wolf-interp)

Bare-ident match patterns bind instead of resolving in-scope
variants/tags: `match Ordering.Greater { Less => 1, Equal => 2,
Greater => 3 }` yields 1 — the first arm always matches, silently
wrong (the corpus never catches it: match_exhaustive.lu passes by
arithmetic coincidence). Consequences: non-exhaustive matches run to
completion (no E0801 analogue), and payload-free enum/tag dispatch is
unusable under lupin — std.cmp's exhaustive-match test flattens its
arm values to stay honest, and no negative E0801 test is shippable
(lupin's exit-verdict hard-mismatches a fail directive). lupin also
accepts qualified path patterns (`Ordering.Less =>`), which are
outside the pinned grammar — dispatching correctly there is the bug's
mirror image. Payload-carrying patterns (`Tag(x) =>`) dispatch
correctly.

## F-0008 — the iterator protocol (contract F1)

UPDATE (sc03 pin `12ae8c2`): **CLOSED upstream** — wolf-lang#8 adopted the
protocol with range-for, and `[mem.iter.*]` now specifies it (`List[T]`
and `Pool[T]` adopt `Iter` builtin-side; user types implement by name, no
structural conformance). Observed at the new pin: `for i in 0..3` runs in
BOTH implementations. `std.iter`'s prototypes still ship as written — they
are the record of what was proposed — and their retirement into the
builtin protocol belongs to the stdc01 closeout, not to sc03.

Amendment candidate: nominal trait `Iter[T]` with
`next(mut self) -> T ! {done}`; `for pat in e` desugars to the
explicit drive loop over `Iter`; range expressions get an owned range
type implementing it; `List`/`Pool` adopt it builtin-side. The
row-signal design is prototyped EXECUTING in `std.iter` (lupin lane):
`range_iter`/`range_next` and `list_cursor`/`list_next`, driven by
`while` + `else |_|` — exhaustion-stays-exhausted proven. Prototype
evidence for the trait: without dispatch, the two `next`s collide and
ship as `range_next`/`list_next` (one name, dispatched, is the fix);
the end tag is `Done` pending F-0003; methods await execution support
(F-0002). Weighed alternative recorded: an absence-tag (`none`) end
signal was rejected — iteration's end is its own noun, and `done`
keeps absence and exhaustion separable in rows that carry both.

## F-0009 — assert, defined (contract F4)

UPDATE (sc03 pin `12ae8c2`): **CLOSED upstream** — wolf-lang#9 is closed
and `[conf.trap.assert]` now specifies the intrinsic, including the
two-argument form with `msg` evaluated only on the failing path. The
implementation of the second arity is broken, though: it traps even when
the condition holds (F-0020 / wolf-lang#19), so std.testing keeps the
`if !cond { testing.fail(msg) }` interim for one more pin.

Definition filed: `assert(cond)` is an INTRINSIC, one name in both
tiers — comptime witness (s16) and runtime user trap
(`[conf.trap.map]`), silent and effect-free when satisfied, trapping
at its own span when not. Why intrinsic: a prelude/std function named
`assert` shadows the builtin module-wide (observed: E0402 arity
errors in both implementations), severing any module that defines it
from the trap it must raise — the name cannot be both library surface
and primitive. The two-argument `assert(cond, msg)` therefore needs
either arity overloading (the language has none, one-name-one-
signature) or the intrinsic itself growing the parameter — the filing
proposes the latter, with `msg` evaluated only on the failing path.
Trap payload/rendering: one line to stdout before the trap is the
sc01 floor (`testing.fail`); rendering values waits on sc05's fmt.
Interim spelling shipped: `if !cond { testing.fail(msg) }`.


## F-0010 — lupin has no std root (the interpreter half of F-0001)

Evidence at the sc02 pins: one package root holding `main.lu` with
`use std.prelude` and the tree at `std/prelude/prelude.lu`. wolfc with
`--std-root <root>/std` (or `WOLF_STD`) reaches `run` / `exit(0)`; lupin
answers `resolve` / `unsupported`, `x-unsupported: "prelude.least does
not resolve"`. Mechanically (`src/sema.rs`), lupin binds a `use`'s LAST
segment and looks for a package directory of that name — `use std.cmp`
tries `<root>/cmp` and `<root>/std`, never `<root>/std/cmp`.

That last-segment rule is also what makes the interim cheap: staging
mirrors `std/<mod>/` to `<mod>/` beside the real tree, so ONE source
text — the real `use std.<mod>` — runs on both lanes. The rig refuses a
mirror collision (two modules with the same last segment) loudly rather
than overwriting, and the whole mirror disappears when wolf-interp#6
lands.

## F-0011 — what is builtin and what is std (contract Target-1)

The recommendation, filed with its four gaps (wolf-lang#11):

- `List[T]` and `Pool[T]` are **language-blessed core types** — spec-owned
  semantics, native in both implementations, with std owning the
  documented API surface and extension mass (this sprint's `std.list`,
  `std.pool`).
- `Map`, `Set`, `Deque` are **std-defined** types once s16 delivers
  generic data types; the prelude's ambient `Map` is a builtin interim
  that must not outlive s16.

The four gaps, each blocking inventory this sprint had to leave unbuilt:
(1) `struct X[T]` does not parse in EITHER implementation (E0201), so
`Set`/`Deque` ship as reviewed contracts plus the concrete running
prototype `std/x/deque_int`; (2) the `Map` key protocol is unspecified —
std spells the `Eq` half as a bound, the `Hash` half is the open
question — and no builtin ERASES a key, so `remove` rebuilds; (3) `Pool`
exposes no length, capacity, liveness probe or iteration, which is
exactly four of `std.pool`'s six functions; (4) the wordcount idiom
`tally[w] += 1` runs nowhere (lupin: an index is not a place; an absent
key reads `()`, not a zero) — std recommends owning it as `map.tally`,
because a defaulting read hides the miss the absence row makes visible.
The s37 receiver-mode debt rides along: `(mut xs).push` is still E0804
(F-0005), so every std.list mutator takes `mut xs` and calls the bare
builtin inside.

## F-0012 — trait/enum/impl items make importers unsupported (wolfc)

`wolf conform-run --checked` answers `unsupported — module items in
checked execution` for any entry importing a module whose items include
a `trait`, `enum` or `impl` — the span names the item, and the entry
need not touch it. Consequence for std: `use std.cmp` costs a module
every wolfc `run` row, which is why std.list's six element-comparing
functions live in `std/x/list_eq` instead of `std.list` (they would have
flipped twenty other functions' rows from `run` to `unsupported`).
Reproducer in the filing: a two-item module (`pub enum Tag`, `pub fn ok`)
is enough; deleting the enum makes the same entry run.

## F-0013 — false ub(mem.ub) on ordinary std code (lupin)

Two shapes, both reproduced from sc02's own modules, both false
positives in lupin's Tier-3 provenance model, both worked around in the
test suite with the workaround documented in each test header:

- **(a)** a `mut` argument inside an f-string interpolation
  (`print("{m.pop(mut xs) else 0}")`) leaves its borrow tag Disabled but
  live; a later read resolves through that tag —
  `read through tag#2 …, which is Disabled`. Binding the call in a `let`
  first runs clean.
- **(b)** a `mut`-mode call followed by a read-mode call whose body
  ALLOCATES (`map.set` then `map.keys`) reports
  `foreign write … while tag#3 … is PROTECTED for a call's extent`,
  though the only write is to the callee's own fresh container; the
  allocation is reported as "1 byte(s)", which suggests the model cannot
  tell the two containers apart. Populating through the language's
  `m[k] = v` runs clean.

Highest-severity verdict class (`[proto.record.ub]`), so it matters that
it be right before triage is built on it.

## F-0014 — mutate-while-iterating (the contract's triage case)

wolfc rejects statically, `fail(E1001)`, naming `for x in xs` as the
move and suggesting a `copy` operator outside the pinned grammar —
a static answer, but reached through reads-as-moves (F-0005) rather than
any container-exclusivity rule. lupin RUNS the same program to
`exit(0)`, no trap, where `[conf.trap.map]` predicts `exclusivity`.
Recorded as `tests/list/mutate_while_iterating.lu` (directive matches
lupin; the ledger records wolfc's `fail(E1001)`), so CI holds the
divergence until the spec answers: static error, dynamic trap, or both.

## F-0015 — the row RAISE path inside a module (wolfc checked tier)

`list.get(xs, 0)` on a present index runs (`exit(0)`); `list.get(xs, 5)`
— the same call, same module, taking the `return None` path — is
`unsupported — module items in checked execution`, the span pointing at
the raise. Inlining the identical body into the entry file runs both
paths. Consequence: every absence MISS test is `unsupported` under wolfc
while its hit twin is `run` — six such rows in this sprint's ledger
(`list/get_hit_miss`, `list/first_last_hit_miss`, `list/pop_drains`,
`x/deque_int/{fifo_order,both_ends,pop_back_and_clear}`), all of which
lupin runs.

## F-0016 — wolf fmt splits a dotted call after a comment

`// comment` on its own line immediately above `xs.push(1)` makes
`wolf fmt` emit `xs.` + newline + `    push(1)`, idempotently, for both
method and module-qualified calls. D34 makes fmt's output the style, so
wolf-std ships whatever fmt produces; sc02's workaround was deleting the
comments that sat above dotted calls (the information moved into the
file headers).

## F-0017 — let reassignment still executes (lupin)

wolf-lang#2 is closed compiler-side (wolfc: `E0410` with a `var`
fix-it); lupin still runs `let a = 1` / `a = 2` to `exit(0)` printing
`2`. The guide's "never rely on it" stands, now against one
implementation instead of two.

## F-0018 — the boundary primitive (sc03's central filing)

`wolf-lang#17`. A byte-offset string library cannot be written in wolf at
this pin, and the reason is one missing primitive rather than a long list
of missing conveniences.

D25 gives `s[a..b]` with the checked fault, and the fault is right:
`"wolf"[0..9]` traps `bounds` ("byte range 0..9 is outside a 4-byte
string"), `"é"[0..1]` traps `bounds` ("byte range 0..1 splits a UTF-8 code
point"), both deterministic, both pinned as tests here. What it leaves a
library author is the problem: to scan for a needle, `find` must advance
past code points of unknown length; it can learn a length only by slicing;
a wrong guess is not `false`, it is process death, and there is no
unwinding to catch it (D30). A `find` written over slicing would trap on
`find("héllo", "l")` — legal input. That is a broken function, not a
documented partial, so std shipped none of them.

What IS writable is the **boundary-safe probe**: after `s.starts_with(p)`
answers true, `p.len` is a code-point boundary of `s`. That one fact
carried five real functions — `strip_prefix`, `trim_start`, `trim_end`,
`parse_int`, `parse_float` — all total over arbitrary UTF-8, all
executing. `parse_int` never slices the input except by one byte after
matching an ASCII digit through `starts_with` against an alphabet literal
std owns, which is why `parse_int("é", 10)` is a `Parse` miss and not a
trap.

Any ONE of three s37 primitives unblocks the family: the recoverable
slice `str.get(a..b) -> str ! {none}` (cheapest, and it cannot be written
in library code because writing it needs itself), a byte accessor
(`bytes()` with `b[i] -> int`, which D25 explicitly licenses), or
`chars()` yielding `(offset, char)`.

Blocked inventory: `std.str` 15 of 33 (`find`, `rfind`, `count`, `split`,
`split_once`, `rsplit_once`, `ends_with`, `strip_suffix`, `replace`,
`replacen`, `get`, `bytes`, `chars`, `graphemes`, `to_list_chars` — note
`ends_with` blocked while `starts_with` is trivial: the asymmetry is the
bug's signature); `std.strbuf` 3 of 11 (`push(c)`, `strbuf.in(r)`,
`reserve(n)`); `std.bytes` 9 of 9; `std.unicode` 1 of 10 (`char.code`).
Every one is a reviewed contract in its module header, so the landing
sprint implements a signature instead of redoing a design.

Adjacent, filed in the same issue: there is **no `char` type at all**
(`'a'` is E0101 in lupin / E0107 in wolfc; `for c in "abc"` is
`unsupported` in both), `^n` end-relative indexing resolves in NEITHER
implementation (`"hello"[..^1]`), wolfc answers `unsupported` at RESOLVE
for every builtin-receiver `str` method and for `s[a..b]` (the s37 prelude
plumbing sc03's contract predicted — which is why all 13 std.str/strbuf
tests are lupin-lane and the wolfc column flips as a ledger commit), and
`str + str` runs in lupin while wolfc says E0409 (std writes `"{a}{b}"`,
which runs in both).

## F-0019 — the unicode tables budget (a decision request)

`wolf-lang#18`. sc03's acceptance requires this filing; it asks four
questions and offers a recommendation, and it carries two pieces of
evidence that change the question.

The posture shipped: v1 promises UTF-8 validity as a type invariant and
boundary-refusing slicing (both the language's) plus ASCII-true
classification named as such (`std.unicode`, nine executing functions).
Not shipped: category tables, case mapping/folding, normalization,
grapheme segmentation. Recommendation: they land in **penumbra or
`std/x`, never core** — core is welded to the compiler (D31) and the two
things core needs (validity, boundaries) need no tables.

Evidence 1: **the pinned interpreter already does simple Unicode case
mapping.** `"ÉA".lower()` is `"éa"`. The contract planned ASCII-only
`lower`/`upper` this sprint; the implementation is already past that, so
the tables exist somewhere and the open question is where they live and
which implementation is authoritative. std delegates (an in-library
ASCII-only mapping is unwritable without a byte accessor) and documents
these two as "the implementation's simple case mapping"; `eq_ignore_case`
inherits it and the contract's "ASCII, named honestly" qualifier is
dropped as no longer true.

Evidence 2: **the whitespace set is Unicode too.** `trim` and `words` use
`White_Space`, not ASCII — U+0085, U+00A0, U+1680, U+2000..U+200A,
U+2028, U+2029, U+202F, U+205F and U+3000 all behave as whitespace
(observed, all 25). std's `trim_start`/`trim_end` therefore carry a
25-entry table of their own purely to AGREE with the builtin, and
`tests/str/trim_whitespace_set.lu` pins the agreement code point by code
point so a builtin that changes its set fails CI here instead of silently
disagreeing with std. That table is the first Unicode data in wolf-std and
it is the argument for deciding table ownership before every module grows
its own.

## F-0020 — `assert(cond, msg)` traps when the condition holds

`wolf-lang#19`. At trunk `12ae8c2`, `assert(1 == 1, "one is one")` and
`assert(2 > 1, msg)` both `trap(assert)` under wolfc while
`assert(1 == 1)` exits 0 and lupin runs all three correctly. The
two-argument intrinsic wolf-lang#9 just landed appears to ignore its
condition. It contradicts `[conf.trap.assert]` ("silent and effect-free
when the condition holds"), and the failure mode is the worst available
for a test primitive: a PASSING assertion aborts the program. `std.testing`
therefore keeps sc01's interim `if !cond { testing.fail(msg) }` at this
pin — holding, not working around.

## F-0021 — method call on a slice expression (lupin)

`wolf-interp#10`. `d[0..1].upper()` where `d` is a binding is
`unsupported` at resolve: "`d["0..1"]` does not denote a place at run
time". The same shape over a literal (`"abc"[0..1].upper()`) runs, and
`"abcd".lower()[0..1]` runs, so only `binding[range].method()` falls into
the refusing path — and the diagnostic stringifies the range, suggesting
the index is reduced to a string key before the place check. Sharp because
it is exactly the shape a string library reaches for, and it fails at
resolve, taking out the whole file. Every `std.str` body binds the slice
first.

## F-0022 — `n as f64` does not convert (lupin)

`wolf-interp#11`. Silently wrong arithmetic: `(3 as f64) == 3.0` is
false, `(3 as f64) == 3` is true — the cast is accepted and produces an
int. wolfc refuses the mixed comparison outright (E0401), which is what
makes the bug invisible on the lane that runs. `std.str.parse_float`
therefore ships a ten-branch `digit_f(d: int) -> f64` literal table so
that no `as` appears in its accumulation; the reason is documented on the
helper, and 28 executing assertions depend on it.

## F-0023 — the interpreter half of rows and tags (lupin)

`wolf-interp#12`. wolf-lang#3 and #4 are closed compiler-side and neither
landed in lupin: `fn or(v: int ! {none}, d: int)` is `fail(E0201)` at
parse (wolfc reaches mem), `let a: int ! {none} = …` likewise, and
`return none` is `unsupported: `none` does not resolve` (wolfc reaches
mem). So `std.option`'s six helpers are STILL unwritable — the blocker
moved from both implementations to the one that executes them, which is
progress that changes nothing about what ships.

The third part is the one to remember: **lupin resolves row tags lazily,
at run time, on the taken path.** A `return none` on a branch the program
never enters produces no diagnostic and the program exits 0 — which is
how sc02's F-0003 update came to claim, wrongly, that lupin had learned
lowercase tags. Any probe of raise-site behavior must exercise the raise.

## F-0024 — trunk red locally, green in CI

`wolf-lang#20`. `cargo test --workspace` at `12ae8c2` in a clean scratch
clone fails `wolf_parse::blast_radius::single_token_mutations_have_bounded_blast_radius`
deterministically (4 added diagnostics against a max of 3, on
`corpus/comptime/norm_witness.lu`), on the rustc version
`rust-toolchain.toml` pins, while that sha's trunk CI run — whose
`cargo xtask ci` includes `cargo test --workspace` — reports success.
sc03 pinned the sha anyway (the driver builds and behaves correctly across
63 staged programs) with the asterisk in `vendor/tools.toml`. The
downstream lesson: "the last green trunk run" is not by itself a
sufficient pin criterion, and this repo's pin ritual should grow a second
gate.
