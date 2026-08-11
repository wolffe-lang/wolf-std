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
| F-0025 | 2026-08-12 | Integer literals ignore their context in lupin: `INT_MIN` has NO spelling (`-9223372036854775808`, `-MAX - 1`, `0 - MAX - 1` all trap `overflow` at `i32` whatever the annotation says), `var k = 0` infers `i32`, and a cross-module `-> int` call does not type its own operator (`math.int_max() - 1` traps) — the only working shape puts a typed binding on the left | wolf-interp | [filed: wolf-interp#14](https://github.com/tenseleyFlow/wolf-interp/issues/14) |
| F-0026 | 2026-08-12 | Capability map of the compiler's two rungs, with std's cost per refusal: the checked tier refuses every f64 literal, every `const` USE and `&`/`^`/`>>`/`\|`; the native rung refuses `const` DECLARATIONS, `List`, generics, `print` and `str` — and refuses two modules that declare a function with the same name (a real mangling bug: `std.list.len` and `std.str.len` already collide) | wolf-lang backend/checked-tier owners | [filed: wolf-lang#26](https://github.com/tenseleyFlow/wolf-lang/issues/26) |
| F-0027 | 2026-08-12 | **Silent wrong answer**: the native rung lowers `!=` on `f64` to an ORDERED comparison, so `nan != nan` is FALSE natively and true in the interpreter (`==`, `<`, `<=`, `>`, `>=` are all correct). `x != x` is the portable NaN test and was `std.cmp.total_cmp`'s; every float inequality in std now spells `!(x == y)` | wolf-lang s28 codegen | [filed: wolf-lang#22](https://github.com/tenseleyFlow/wolf-lang/issues/22) |
| F-0028 | 2026-08-12 | (sc04 Target 2) The pure-wolf transcendentals and the intrinsics request: 29 functions, measured ≤ 1 ulp except `cbrt` (2) and `powf` (3), bit-identical on BOTH executing lanes over 200 pinned values — plus the two asks, that the wolf source stay the semantic reference and that `sqrt` be re-derived from the hardware instruction rather than the other way round | wolf-lang s37 intrinsics / s41 llvm | [filed: wolf-lang#25](https://github.com/tenseleyFlow/wolf-lang/issues/25) |
| F-0029 | 2026-08-12 | Cross-module enum consumption: an enum's VALUES cross a module boundary but nothing that inspects them does — variant patterns do not resolve against an imported enum (lupin), methods do not dispatch, and an enum-returning call is `unsupported` in the checked tier and has "no recorded type" natively. Blocks `sort_by`, `is_sorted_by`, `binary_search_by` — and the sorting STABILITY WITNESS, which is only observable through a comparator that ignores part of the value | wolf-lang #12's family + wolf-interp | [filed: wolf-lang#23](https://github.com/tenseleyFlow/wolf-lang/issues/23) |
| F-0030 | 2026-08-12 | (sc04 Target 5) A range value has no bounds accessor under any spelling (`start`/`end`/`len`/`lo`/`hi`/`first`/`last` all "no member") and `range` does not resolve as a TYPE in wolfc (E0301, both rungs) — so `contains`, `len` and `clamp_to` are unwritable (only by iterating, which hangs on `0..2^63`) and `std.range` ships one function | wolf-lang s37 core types | [filed: wolf-lang#24](https://github.com/tenseleyFlow/wolf-lang/issues/24) |
| F-0031 | 2026-08-13 | (sc05 Target 1) **The format spec means two different things**: `{x:>8}` pads under lupin and is IGNORED by wolfc, which parses the spec and prints the unpadded value — a stdout divergence with no diagnostic on either side; lupin additionally implements only `[[fill]align][width]`, refusing sign/zero-pad/precision/type as `unsupported`, and silently reads `{n:08}` as width 8 with a space fill | wolf-lang s38 (f-string lowering) + spec §7.4 owners | [filed: wolf-lang#10](https://github.com/tenseleyFlow/wolf-lang/issues/10) |
| F-0032 | 2026-08-13 | lupin's `as` accepts an UNKNOWN target type silently: `s as int` and `s as nonsense` both pass the value through unchanged with no diagnostic, while `n as f64` converts correctly — a typo in a cast is invisible, and so is a cast the machine has no rule for | wolf-interp | [filed: wolf-interp#17](https://github.com/tenseleyFlow/wolf-interp/issues/17) |
| F-0033 | 2026-08-13 | (sc05 Target 1) **spec §7.4 does not exist**: `FORMAT_SPEC` is in the grammar with its semantics an explicit IOU, so every question a formatter must answer is unanswered — what each spec means per builtin type, what a malformed or type-mismatched spec does, and how a USER type formats. Candidate section text filed with a running reference implementation (`std.fmt`, whose functions are the spec's worked examples) plus the **`Show` proposal** (`fmt(self, spec) -> str`) and the s38 dispatch hook it needs | wolf-lang spec owners + s38 | [filed: wolf-lang#28](https://github.com/tenseleyFlow/wolf-lang/issues/28) |
| F-0034 | 2026-08-13 | The module namespace is FLAT at the last path segment: `std.fmt.float` cannot import `std.math.float` (lupin: "this import completes a cycle: `float` → `float`", E0303) and cannot be imported beside it (wolfc: E0306 — while lupin silently binds one of the two and drops the other). Two facades may never grow a same-named leaf, so §10's "the float family lives in its own module" pattern is unrepeatable; sc05's module is `std.fmt.decimal` because of it | wolf-lang resolve owners + wolf-interp | [filed: wolf-lang#29](https://github.com/tenseleyFlow/wolf-lang/issues/29) |
| F-0035 | 2026-08-13 | (sc05 Targets 3-4) **The encoders have no byte type**: `std.bytes` is still 0/9 (F-0018 re-tested, unchanged), so `std.hex` and `std.base64` ship over `List[int]` with a documented 0..255 element contract — and `hex.encode(str)`, the commonest use of a hex encoder anywhere, cannot exist because nothing reads a `str`'s bytes. The same root blocks `json.parse`, `json.unescape`, `fmt.truncate_to`, and forces `json.escape`'s one refusal | wolf-lang s37 core types | [filed: wolf-lang#17](https://github.com/tenseleyFlow/wolf-lang/issues/17) (sc05 evidence on F-0018's issue) |
| F-0036 | 2026-08-13 | **Silent wrong answer**: a row tag that shares a name with anything in the value namespace at the raise site resolves to that THING instead of raising — `-> int ! {tagmod}` inside module `tagmod` hands the caller the module value, `else` never fires and no diagnostic appears. Found three ways in one sprint (`std.hex` raising `hex`, `std.json`'s `kind` function versus its `kind` tag, and `std.fmt.decimal` nearly raising `range` beside `std.range`) | wolf-lang resolve + wolf-interp | [filed: wolf-lang#30](https://github.com/tenseleyFlow/wolf-lang/issues/30) |
| F-0037 | 2026-08-13 | **Silent wrong answer**: a function whose return type is an ENUM and whose signature carries an error row takes the MISS path on every call — `fn id(v: Value) -> Value ! {none} { v }` raises instead of returning `v`, in one line, with no diagnostic. Blocks `json.get` and `json.at`, which were written, tested and withdrawn to reviewed contracts; until it closes, no std accessor may return an enum through a row | wolf-interp (row/enum value representation) | [filed: wolf-interp#16](https://github.com/tenseleyFlow/wolf-interp/issues/16) |
| F-0038 | 2026-08-13 | **Absence has no literal**: a row VALUE cannot be written as an expression — `none` resolves at a raise site and nowhere else, so `f(none)` is `unsupported` — which means no doc example can call a row-taking function and `std.option`'s six ship with prose examples instead of fenced ones | wolf-lang s15/resolve owners + wolf-interp | [filed: wolf-lang#38](https://github.com/tenseleyFlow/wolf-lang/issues/38) |
| F-0039 | 2026-08-13 | **Nested rows diverge**: `T ! {none} ! {none}` parses and EXECUTES in lupin and is `fail(E0201)` at parse in wolfc, in both return and parameter position — the grammar's `type` production admits one `!`. `std.option.flatten` is the only helper whose type needs two, so it ships from the nursery to keep the module's other four out of a rejection | wolf-lang s03 grammar + s37 | [filed: wolf-lang#34](https://github.com/tenseleyFlow/wolf-lang/issues/34) |
| F-0040 | 2026-08-13 | **No bottom type**: a diverging `else` handler cannot typecheck generically — wolfc types an `else \|_\| { … }` block by its last expression, `assert(false)` is `()`, and the block must produce `T` (E0401). A monomorphic helper writes an unreachable dummy; a generic one has no `T` to conjure, so `std.option.expect` ships from the nursery. lupin accepts and runs it | wolf-lang s14 typecheck + spec (D30's no-unwinding makes divergence ordinary) | [filed: wolf-lang#35](https://github.com/tenseleyFlow/wolf-lang/issues/35) |
| F-0041 | 2026-08-13 | (sc06 Target 2) **The error-set alias surface** — s15's parked amendment, filed with the measured cost: 49 `pub` signatures carry a row across 16 modules, 11 distinct shapes, 45 of them one tag and 4 of them two, and NOTHING exceeds two. Core does not need aliases yet and the filing says so; what it argues is the semantics (`error Set Name = {…}` as a transparent name, never nominal — from D30) and the io taxonomy that makes it urgent in stdc02, plus the `try`⇄Result bridge's standing dependency on s16 | wolf-lang s03 grammar + s37 (std error taxonomy owners) | [filed: wolf-lang#36](https://github.com/tenseleyFlow/wolf-lang/issues/36) |
| F-0042 | 2026-08-13 | (sc06 Target 6) **`wolf test` must subsume this rig without rewrites** — the s39 alignment requirements, with the rig as the working reference: directive-header compatibility (`check:`/`phase:`/`conforms:` verbatim), one trap expectation per entry file as the catch mechanism, subtest naming (Go 12166), a `--json` record stream (Go 2981), the three-lane ledger as a first-class concept, and the D36 bench-format reservation so `std.bench` can land in stdc02+ without a format war | wolf-lang s39 (+ D36 owners) | [filed: wolf-lang#34](https://github.com/tenseleyFlow/wolf-lang/issues/34) |

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

## F-0025 — integer literals ignore their context (lupin)

`wolf-interp#14`. Three shapes, one cause, and one of them makes a value
that exists in the type unwritable.

**`INT_MIN` has no spelling.** Every form of -2^63 traps `overflow`
"outside `i32`", whatever the declared type says: `const A: int =
-9223372036854775808`, `-9223372036854775807 - 1`, `0 - 9223372036854775807
- 1`, `let d: int = -9223372036854775807`, `let e: i64 = …`. The one shape
that works puts a TYPED BINDING on the left of the operator, so the
operation is typed by the binding and not by the literal's `i32` default:
`let zero: int = 0` then `zero - 9223372036854775807 - 1`. That line, with
that comment, is `std.math.int_min`'s body.

**An unannotated `var` is `i32`.** `var k = 0` then `k * 4503599627370496`
traps. Every accumulator in `to_bits`/`from_bits`/`mantissa_bits` carries
an explicit `: int`.

**A cross-module `-> int` call does not type its own operator.**
`math.int_max() - 1` traps; the same expression over a same-file function
does not. This is why `std.math`'s doc examples say
`math.int_max() > 0` rather than `math.int_max() - 1 < math.int_max()`.

Consequence for the sprint: the contract's `INT_MIN`/`INT_MAX` constants
ship as the zero-argument functions `int_min()`/`int_max()`, and every
big literal anywhere in std or its tests goes through a typed binding.
wolfc is correct on all three shapes at trunk `d147a54`.

## F-0026 — the two rungs' capability map

`wolf-lang#26`. Not a defect report: a map of what each of the compiler's
execution rungs refuses, with std's cost. It is the direct cause of this
sprint's rig change (a third ledger column), because the two rungs refuse
DIFFERENT things and neither alone can carry a library's evidence.

Checked tier refuses: every `f64` literal ("this literal shape in checked
execution" — so all thirty float functions get zero evidence there);
every USE of a module-level `const`; `&`, `^`, `>>`, `|` on integers
("this operator in checked execution"); `str` ordering.

Native rung refuses: every DECLARATION of a module-level `const`
("item-initializer lowering (globals, c06)"); `List`/`Pool`
construction; generic functions (monomorphization); `print`; `str`; and
**two modules that declare a function with the same name** — a real
mangling bug, since `std.list.len` and `std.str.len` already collide.

Two of these shaped the API directly. Constants became functions
(F-0025's other half): a `const` costs the checked lane when used and the
native lane when declared, so `pi()` costs one pair of parentheses and
keeps both. `midpoint` is written with halves and remainders rather than
`(a & b) + ((a ^ b) >> 1)` because the bit-twiddling form loses the
checked lane for nothing.

What the native rung gives BACK is worth as much: it executes f64
arithmetic, rows and their raise paths (wolf-lang#13's refusal does not
apply there), structs, `wrapping[u64]`, recursion and traps by kind. It
is the only compiler-side evidence `std.math.float` has.

## F-0027 — `!=` on f64 is ordered in the native backend

`wolf-lang#22`. The sprint's most severe filing, because it is a silent
wrong answer rather than a refusal. `nan != nan` is FALSE under
`conform-run --native` and true everywhere else; `==`, `<`, `<=`, `>` and
`>=` are all correct, which is the signature of an ordered `fcmp ne`
where the unordered one was meant.

`x != x` is *the* portable NaN test and it is what sc01's
`std.cmp.total_cmp` was written with, so this sprint rewrote those two
probes as `!(x == x)` in the same commit as the finding — along with
every inequality in `std.math.float`, whose header now carries the rule
so the next author does not reintroduce it. All of it is revertible when
the issue closes. `tests/math/float/predicates.lu` is the pin.

## F-0028 — the pure-wolf transcendentals, and the intrinsics request

`wolf-lang#25`. sc04's acceptance requires this filing. The claim it
carries is the one the sprint set out to make good on: **29 functions in
pure wolf, and both executing lanes agree bit for bit.**

`cargo xtask ulp` reads `tests/ulp/reference.txt` (200 rows: the call,
the value std produces, the correctly rounded value, the ulp budget the
doc comment promises), checks each distance against its budget, refuses a
row whose budget exceeds what the function documents, and then generates
a program asserting all 200 values EXACTLY and runs it on every lit lane.
The interpreter and the native backend each reproduce all 200, which is
the pure-wolf determinism decision demonstrated rather than asserted.

Measured worst case over that set: ≤ 1 ulp for `sqrt`, `exp`, `exp2`,
`ln`, `log2`, `log10`, `sin`, `cos`, `tan`, `asin`, `acos`, `atan`,
`atan2` and `hypot`; 2 for `cbrt`; 3 for `powf` (documented 4). Three
constants earned their places the hard way and are worth recording: the
two-piece `ln2` split in `exp` needs a HIGH piece with 33 significant
bits or `exp(700.0)` is 220 ulp out; the three-piece `pi/2` split in the
trig reduction needs the same or `cos(1000.0)` is 304 ulp out; and
`log10`'s multiplier was one ulp wrong (`0.30102999566398114` for
`0.3010299956639812`), which cost 2 ulp on every input until the harness
caught it.

The asks: the wolf source stays the semantic reference when intrinsics
land, and `sqrt` is the exception to take first — IEEE requires it
correctly rounded and the hardware gives that for free, so it is the one
function whose intrinsic will be MORE accurate than this source and where
the source should be re-derived from the instruction rather than the
other way round.

## F-0029 — an enum's values cross a module boundary; inspecting them does not

`wolf-lang#23`. `oo.is_lt(oo.cmp_int(1, 2))` RUNS in lupin — an enum
value is produced by one module and consumed by another — but every way
of looking at that value from outside the declaring module fails: a
`match` with variant patterns is `unsupported: no match arm applied`
(lupin), a method call is "has no method … in this machine's std subset",
and the whole shape is `unsupported — module items in checked execution`
in the checked tier and "a member access without a recorded type"
natively.

The lupin half is NEW information produced by a FIX: wolf-interp#5 landed,
so a bare-ident pattern is a variant pattern now instead of a binding that
always matched the first arm — and the honest refusal replaced a wrong
answer. `tests/cmp/ordering_exhaustive.lu` moved `run` → `unsupported` in
the ledger for exactly that reason and the comment there says so.

Blocked: `std.sort.sort_by`, `std.sort.is_sorted_by`,
`std.search.binary_search_by` — all three ship as bodies that pass the
static ladder (`tests/sort/generic_contract.lu`, `check: pass`, phase
mem) and execute nowhere. And, less obviously, **the sorting stability
witness**: stability is only observable through a comparator that ignores
part of the value, so the three executing sorts — which order by the
whole value — cannot demonstrate it. sc04 ships a stable merge sort whose
stability has no executable test at these pins, and says so on the
function rather than claiming a property no test covers.

## F-0030 — a range has no bounds accessor

`wolf-lang#24`. `2..7` can be built, passed as `range[int]` and iterated,
and that is the entire surface: `start`, `end`, `len`, `lo`, `hi`,
`first`, `last` all answer "a range has no member", and wolfc does not
resolve the TYPE `range` at all (E0301, both rungs), which makes a range
an interpreter-only parameter type.

`contains` and `len` are expressible only by iterating — O(len) for an
O(1) question, and a hang rather than an answer on
`0..9223372036854775807`. sc03's refusal-over-approximation rule (§9)
says a function that is right on small inputs and fatal on legal ones
does not ship, so it did not. `clamp_to` has no spelling at all.
`is_empty` survives because emptiness is decidable from iteration in O(1)
(`for _ in r { return false }`) and ships as `std.range`'s single
function — the demonstration that the blocker is the accessor and not the
iteration.

## F-0031 / F-0033 — the format spec, and the section that defines it

`wolf-lang#10` (re-measured) and `wolf-lang#28` (the amendment). sc05's
acceptance requires the second; the first is what makes it urgent.

The grammar has the hook — `INTERP ::= '{' expr FORMAT_SPEC? '}'` — and
spec §7.4, named as the home of its semantics, does not exist. Each
implementation therefore decided for itself, and they decided differently:

| program | lupin 0.1.3 | wolfc 6bfff9a `--checked` |
|---|---|---|
| `"[{s:8}]"`, `s = "hi"` | `[hi      ]` | `[hi]` |
| `"[{n:>6}]"`, `n = 42` | `[    42]` | `[42]` |
| `"[{n:*>8}]"` | `[******42]` | `[42]` |
| `"[{n:08}]"` | `[      42]` | `[42]` |
| `"[{n:+}]"` | `unsupported` | `[42]` |
| `"[{f:.2}]"` | `unsupported` | `[3.14159]` |
| `"[{n:x}]"` | `unsupported` | `[42]` |

Three separable facts. **wolfc parses the spec and silently ignores it** —
two implementations printing different bytes for one program, which is the
class the differential rig exists to catch, and neither is *wrong* because
there is nothing to be wrong against. **lupin refuses honestly** outside
`[[fill]align][width]`, which is the right posture for an unimplemented
spec. And **`{n:08}` is wrong even under lupin's own semantics**: the
zero-pad flag is absorbed into the width, so a reader who asked for
`00000042` gets spaces.

The amendment filed on #28 carries: the grammar
(`[[fill] align] [sign] ['0'] [width] ['.' precision] [type]`), the meaning
of each field per builtin type (width in BYTES per D25, default alignment
left for `str`/`bool` and right for numbers, zero-pad after the sign,
precision as a code-point-boundary-respecting maximum on `str`, `b`/`o`/
`x`/`X`/`e`/`E`/`f` as the type set, and shortest-round-trip as the
default float rendering), the posture that a malformed or type-mismatched
spec is a COMPILE-TIME error at the interpolation's span (D22 quality —
possible precisely because the grammar admits no computed spec), and the
`Show` proposal: `fn fmt(self, spec: str) -> str`, with `"{v:spec}"`
lowering to `Show.fmt(v, "spec")`, no fallback rendering for a type
without an impl, and the dispatch hook named as s38's.

The reference implementation is not a sketch: every meaning in the
candidate text is a function in `std.fmt`/`std.fmt.decimal` with tests, so
the spec's worked-example table and this repo's test table can be the same
table. The one entry std cannot fill is `{s:.8}` — precision on a string —
because truncating at a byte width needs F-0018's primitive.

## F-0032 — `as` accepts an unknown target type (lupin)

`wolf-interp#17`. The cast matrix landed and converts numerics correctly;
what it does not do is resolve the target type. `s as int`, `s as bytes`
and `s as nonsense` are all accepted and all no-ops — the last one names a
type that exists nowhere, which rules out any "permissive conversion"
reading. A typo in a cast is invisible. wolfc rejects an unknown type in
cast position at resolve, so this is an interpreter-side divergence.

## F-0034 — module identity is the last path segment

`wolf-lang#29`. `std.fmt.float` — the obvious name for the float half of
the formatting facade, following §10's `std.math`/`std.math.float` pattern
— cannot exist beside `std.math.float`:

- from inside it, `use std.math.float` is `E0303: this import completes a
  cycle: float → float` (lupin);
- from an entry, `use std.fmt.float` + `use std.math.float` is
  `fail(E0306)` (wolfc) and, worse, RUNS under lupin with one of the two
  silently dropped — the call resolved to whichever binding survived.

D32 makes the tree the namespace; if the leaf is the identity, every
facade competes for a flat pool of short nouns and `std.io.error` /
`std.net.error` are already impossible. sc05 shipped `std.fmt.decimal`,
which is a better name anyway — the module owns the decimal boundary — so
nothing was lost this time. The filing asks for full-path identity and, if
two same-leaf modules should be nameable in one file, a `use … as …` form,
which the pinned grammar does not have.

## F-0035 — the encoders have no byte type

`wolf-lang#17` (sc05's evidence on F-0018's issue). Re-tested at these
pins: `str.get`, `bytes()`, `chars()`, byte indexing and `for c in s` are
all still absent, so `std.bytes` remains 0/9 and this sprint paid for it
three more times.

`std.json.parse` and `json.unescape` are contracts rather than code: a
JSON reader must walk past code points of unknown length, and a `parse`
that guesses offsets traps `bounds` on `{"name": "café"}`. `std.hex` and
`std.base64` ship over `List[int]` with a documented 0..255 element
contract, and `hex.encode(str)` — hex-encoding TEXT, the commonest use of
the module anywhere — cannot be written at all. `std.fmt.truncate_to` is a
contract for the same reason, and it is the one format-spec meaning std
could not offer as a function.

What the sprint proved on the other side is worth as much: sc03's
inversion (probe the input with `starts_with` against an alphabet the
library owns, advance only by a matched literal's length) carried
`parse_int_base` over 36 radices, a correctly-rounded `parse_float`,
`hex.decode`, all three base64 decoders and `json.escape`'s ASCII walk —
every one TOTAL over arbitrary UTF-8. **Decoding a known alphabet is
already safe; copying or skipping unknown text is what is impossible.**
`json.escape` marks the line exactly: it returns `s` unchanged when nothing
needs escaping (so `"café 🐺"` works) and walks ASCII-by-ASCII when
something does, but it cannot step over a non-ASCII code point — so a
string with a non-ASCII character followed by something needing an escape
raises `boundary`. One primitive retires all of it.

## F-0036 — a row tag that collides with a name in scope

`wolf-lang#30`. Silent wrong answer, and the sprint found it three ways.

```wolf
pub fn miss(k: int) -> int ! {tagmod} {
    if k < 0 {
        return tagmod
    }
    k
}
```

In module `tagmod`, `miss(-1) else -7` yields **the module value**, printed
`tagmod`, bound to an `int`. The raise resolved its tag in the value
namespace first, found the module (a module's own name is in scope inside
it), and returned it. No diagnostic, and `else` never fires.

The three collisions, each of which changed a shipped API: `std.hex.decode
-> List[int] ! {hex}` (the tag the sprint contract specifies) became
`! {parse}`; `std.json`'s `pub fn kind` versus its `kind` accessor tag —
`as_int(json.null())` returned the FUNCTION as its `int` — so the function
became `type_name`; and `std.fmt.decimal.parse_float`'s `range` tag, which
would have collided with the `std.range` MODULE at any call site importing
both, became `overflow`. The ask: resolve a bare identifier in raise
position against the declared row first (the fix #4 made when nothing else
was in scope), or diagnose the collision.

## F-0037 — an enum through an error row is always a miss

`wolf-interp#16`. Silent wrong answer, one line of body:

```wolf
pub fn id(v: W) -> W ! {none} {
    v
}
```

`id(mknum(3)) else mkobj()` takes the `else`. An enum returned with NO row
is fine; a `List[W]` or `Map[str, W]` through a row is fine; an `int`
through the same row is fine. It is specifically an enum value riding a
row-typed return, and the likely mechanism is that a raise and an enum
variant share a tagged representation.

What it cost: `std.json.get` and `std.json.at` — written, tested, and
WITHDRAWN to reviewed contracts in the module header, because every call
missed including the hits. `std.json` navigates with `has` +
`as_obj`/`as_arr` and the language's own indexing instead, which works and
is uglier. The rule this repo now writes into its guide — "no std accessor
returns an enum through a row" — is a rule about an interpreter bug and
should not outlive the issue.

## Retirements at the sc04 pins

Four findings this repo filed died with lupin 0.1.2, and one with wolfc
`d147a54`. Each was retested before being written off.

- **F-0007 (wolf-interp#5) RETIRED** — bare-ident match patterns
  dispatch. The first-arm-always bug is gone; a variant pattern over a
  SAME-FILE enum now selects correctly (`tests/cmp/ordering_exhaustive.lu`
  proves the other half, F-0029).
- **F-0010 (wolf-interp#6) RETIRED** — lupin has `--std-root`/`LUPIN_STD`.
  This repo's half of the retirement: the flat mirror is deleted from
  `xtask/src/stage.rs`, all three lanes are pointed at one staged tree,
  the last-segment collision rule is gone with it (a self-test now proves
  two modules may share a last segment), and `std/testing/testing.lu`'s
  `use cmp` — a pre-std-root leftover the mirror had been hiding — became
  the real `use std.cmp`. That last one is the mirror's parting gift: an
  interim that works is an interim you cannot see.
- **F-0013 (wolf-interp#7) RETIRED** — both false `ub(mem.ub)` shapes
  are gone with the `drop_frame` key repair. The workarounds in
  `std.map`'s tests are no longer load-bearing; nothing in this sprint
  needed either shape.
- **F-0017 (wolf-interp#8) RETIRED** — `let` reassignment is E0410 at
  resolve in lupin, byte-identical to wolfc.
- **F-0020 (wolf-lang#19) RETIRED** — `assert(cond, msg)` no longer traps
  when the condition holds (trunk `425a3dc`: ubcheck stopped treating the
  message as a second condition). `std.testing`'s
  `if !cond { testing.fail(msg) }` interim can be retired by whichever
  sprint owns that module next; sc04 did not touch it, since changing a
  test primitive is not a numerics sprint's business.
- **F-0024's lesson APPLIED** — "the last green trunk CI run" is not a
  sufficient pin criterion, so the pin ritual grew a second gate:
  `cargo test --workspace` in a clean scratch clone at the sha. It is
  green at `d147a54` (96 test binaries) where it was red at `12ae8c2`,
  so the gate earned its keep on first use. Recorded in
  `vendor/tools.toml`.

Retested and still open: **F-0022** (`n as f64` does not convert) —
wolf-interp#11 is marked closed and the behaviour is unchanged at 0.1.2;
reopened with fresh evidence. It is why `std.math.float.from_bits`,
`to_bits` and `std.rand.next_float` accumulate mantissas bit by bit
instead of dividing. **F-0012** is narrowed rather than closed: an
imported module containing an `enum` no longer makes every importer
`unsupported` — only calls that PRODUCE the enum are refused (F-0029).
**F-0015** (the row raise inside a module) is unchanged in the checked
tier and does not apply to the native rung, which is why every miss test
in this sprint has a `run` in its native column.

## Retirements at the sc05 pins

Four findings died with lupin 0.1.3; each was retested with its original
reproducer before being written off, and each has a comment on the
upstream issue recording the downstream half.

- **F-0021 (wolf-interp#10) RETIRED** — a method call on a slice-of-binding
  receiver runs: `d[0..4].upper()` is `WOLF` where it was `unsupported` at
  resolve. `std.str`'s bind-the-slice-first bodies are now merely verbose;
  they stay until a sprint owns that module again.
- **F-0022 (wolf-interp#11) RETIRED** — `n as f64` converts. sc04 had
  reopened it with fresh evidence; the cast matrix in 0.1.3 fixes it.
  Downstream: `parse_float`'s ten-branch `digit_f` table is deleted (the
  function moved to `std.fmt.decimal` in the same sprint and now writes
  `digit as f64`), and `std.fmt.decimal.estimate` uses the cast on a
  17-digit significand where exactness matters. The cast's remaining hole
  is F-0032.
- **F-0023 (wolf-interp#12) RETIRED**, all three parts: a `!`-row parses in
  parameter and `let` positions and RUNS (`fn or(v: int ! {none}, d: int)`
  answers 42 and 7), lowercase bare tags resolve at a raise site, and raise
  resolution is eager rather than lazy — the last being the part that made
  sc02 certify a fix that had not happened.
- **F-0003 RETIRED with it** — the `None`/`Parse`/`Overflow`/`DivZero`
  interim spelling is no longer a language constraint on either
  implementation. sc05's five new modules write the convention's lowercase
  tags throughout; renaming sc01–sc04's CapCase tags is a mechanical
  repo-wide commit that this sprint deliberately did not fold into its own
  diff, and `std.str`'s header says so where the interim was documented.
- **F-0002's remaining half FALLS with F-0023**: `std.option`'s six helpers
  (`or`, `expect`, `flatten`, `to_list`, `exists`, `is_none`) are writable
  for the first time, on the implementation that executes them. sc05 does
  not own `std.option`; the module's reviewed signatures are already in its
  header, so this is a landing job for whichever sprint takes it next, and
  it is the last thing wolf-lang#3 was waiting on.

Retested and still open: **F-0018** (the boundary primitive — see F-0035
for the three new casualties), **F-0012** and **F-0015** (the checked
tier's module-boundary ceilings, unchanged), **F-0026** (the two rungs
refuse the same things at 6bfff9a as at d147a54 — the pin bump is
pin hygiene, not capability), and **F-0029** (cross-module enum
consumption, which is why `std.json`'s constructors are functions).

## F-0038 — absence has no literal

`wolf-lang#38`. A parameter can have a row type at these pins; no
expression can produce a value of one. `or(none, 9)` is
`unsupported: none does not resolve` in lupin, because `none` resolves at
a raise site and nowhere else, while `or(7, 9)` works — the hit side has
a literal and the miss side does not.

The cost is not aesthetic. §4's doc-example format is "every line is a
boolean expression or a statement over the documented module, extracted
into an entry file and RUN", and the one-module rule (sc02) forbids
importing a sibling. A function that takes a row therefore cannot have a
fenced example at all: there is no expression to pass it and no way to
bring a producer. All six of `std.option`'s helpers — the module whose
whole subject is absence — carry prose examples, with the runnable
evidence in `tests/option/` and `tests/x/option_*/`: one entry file per
helper, each with a local `find_positive`-style producer and both a hit
and a miss.

The ask offers two closes: a literal for a bare mark in value position
where the expected type admits it (the same declared-row-first rule
wolf-lang#4 applied one position wider), or a spec ruling that rows are
producer-only, so downstream doc tooling stops trying.

## F-0039 — nested rows diverge

`wolf-lang#34`. `T ! {none} ! {none}` parses and executes in lupin 0.1.4
and is `fail(E0201)` at PARSE in wolfc `29a9d9c`, in return position
("expected `{` or line end after the function header") and in parameter
position ("expected `,` or `)` in the parameter list") alike. A single
row in parameter position is fine on both sides at these pins, so this is
specifically nesting: the `type` production admits one `!`.

One consequence worth naming, because D34 makes it a process problem:
**`wolf fmt` cannot format a file with a nested row.** It warns
`W0301: this file has syntax errors, so it was only partially formatted`
and leaves the region byte-for-byte untouched — correct behaviour, and it
means the two files carrying this shape are outside the formatter until
the grammar catches up.

`std.option.flatten` is the only one of the six helpers whose type needs
two, and it ships from `std/x/option_flatten` for the reason the nursery
exists: one rejected body poisons a module for every importer, and
keeping it in the facade moved `or`, `to_list`, `exists` and `is_none`
from `unsupported` (an honest refusal after a clean ladder) to
`fail(E0201)`. `tests/x/option_flatten/flatten_propagate.lu` holds the
rejection as a ledger row — `run` / `fail(E0201)` / `fail(E0201)` — so
the divergence cannot rot quietly.

## F-0040 — no bottom type

`wolf-lang#35`. A diverging handler cannot typecheck in a generic
function. wolfc types an `else |_| { … }` block by its last expression;
`assert(false)` is `()`; the block must produce `T`:
`error[E0401]: this is (), but the else fallback must produce T`. lupin
accepts it and runs it.

The rule is right for an ordinary block. What is missing is a type that
says "this does not come back". A monomorphic helper works around it with
an unreachable dummy after the trap; a generic one has no `T` to conjure,
which is precisely what a bottom type is for — and D30 makes divergence
ORDINARY in wolf (no unwinding, traps end the process,
`fail`/`unreachable`/`todo` are the standard vocabulary for a branch that
is a bug), so this shape is one library code reaches for constantly.

`std.option.expect` ships from `std/x/option_expect` because of it, with
`tests/x/option_expect/expect_trap.lu` ledgered `run` / `fail(E0401)` /
`fail(E0401)`. The filing asks for a `never` type first (which would also
give `testing.fail` an honest signature) and a block-level divergence
rule as the cheaper alternative.

## F-0041 — the error-set alias surface (sc06 Target 2)

`wolf-lang#36`. s15 parked the amendment pending "s37 with the std error
taxonomy"; the taxonomy exists now (§12 + `docs/error-taxonomy.md`), so
the amendment is filed with the measurement it was waiting for — and the
measurement argues against urgency, which is why the filing says so.

49 `pub` signatures carry a row across 16 modules; 11 distinct shapes; 45
carry one tag, 4 carry two, and **nothing carries three**. Repeating
`{none}` 23 times costs nothing an alias would save: a one-tag row is its
own name. What the numbers show is a family beginning — `parse` already
pairs with `overflow` and with `base` — and what makes the feature urgent
is the io tier one campaign away, where `{not_found, permission, io,
utf8}` repeats across a module and then composes with `{parse}` at every
caller.

The semantics asked for: `error Set Name = {…}` at module scope,
**transparent and never nominal** (argued from D30 — rows are structural,
`?` propagates by width subtyping, and a nominal set would create two
ways to be the same type and a conversion between them), composable by
union, expanded in diagnostics so the missing-tag message still names
TAGS, and forbidden from introducing tags that do not exist
independently. Recorded with it: the `try`⇄`Result` bridge still waits on
s16, and std's position is unchanged — exactly one of reified rows or
`Result[T, E]` as data, and with rows now executing in parameter position
the row half is the one carrying weight.

## F-0042 — the s39 alignment requirements (sc06 Target 6)

`wolf-lang#37`. `wolf test` must subsume this rig without a rewrite, and
the rig is offered as its working reference: 127 tests, 211 doc examples,
three lanes. Seven requirements, each with the reason it is not
negotiable rather than a preference:

1. **Directive headers verbatim** (`check:`/`phase:`/`conforms:`) — the
   same files are conform-run inputs; extra keys must be additive.
2. **One trap expectation per entry file**, because D30 makes a trap end
   the process — this is a scheduling constraint on `wolf test`, not a
   style rule.
3. **Kinds, never exit numbers** (`[conf.trap.exit]`).
4. **Subtest naming** (Go 12166), before closures make table tests
   idiomatic and unnamed rows become the norm.
5. **A `--json` record stream** (Go 2981) reusing `conform-run`'s record
   protocol rather than inventing a second format.
6. **Per-implementation expectations as a first-class concept** — the
   ledger, where a test that passes DEEPER than its row is a CI failure.
   It has no analogue in Go or Rust and it is the most valuable thing
   this rig does.
7. **The bench format reserved** (D36 / Go 14313) so `std.bench` can land
   in stdc02+ without a format war.

What the rig does when s39 lands: delete the runner, keep the ledger, the
doc-example extractor and the convention lint.

## Retirements and movements at the sc06 pins

The pin bump is wolfc `6bfff9a` → trunk `29a9d9c` (both gates green in a
clean scratch clone: `cargo test --workspace` and `cargo run -p xtask --
ci`, "all steps green") and lupin 0.1.3 → 0.1.4 (tag `v0.1.4`, its own
conformance pin `ad6cef7` — the lawful two-upstream drift). Every claim
below was re-measured at the new pins.

- **F-0002 RETIRED, and the retirement is a landing.** The six
  `std.option` helpers are written, tested and executing: `or`,
  `to_list`, `exists`, `is_none` in the facade, `expect` and `flatten` in
  the nursery behind F-0040 and F-0039. This is the finding this repo
  filed first and it took five sprints and three pin bumps to die.
- **F-0003's rename EXECUTED.** The finding retired at the sc05 pins; the
  tree still said `None`/`Done`/`Overflow`/`DivZero`. All 148 occurrences
  across 32 files are lowercase now, and the rig is byte-identical either
  side of the change: 127 tests, 211 doc examples, no row moved. The
  audit is `docs/error-taxonomy.md`.
- **F-0020's INTERIM retired** (the finding itself died at sc04).
  `assert(cond, msg)` is silent when the condition holds on both
  implementations at these pins, so `if !cond { testing.fail(msg) }` is
  no longer required. `tests/testing/assert_msg_holds.lu` and
  `assert_msg_trap.lu` are what keep the interim from returning by habit;
  existing tests were not churned to change their spelling.
- **F-0025 two-thirds RETIRED** (lupin #14). `INT_MIN` is writable in
  every annotated spelling (`let a: int = -9223372036854775808` and
  `0 - 9223372036854775807 - 1` both give -2^63) and a cross-module
  `-> int` call types its own operator (`math.int_max() - 1` is
  9223372036854775806 where it used to trap). What remains is
  deliberate: `var k = 0` is still `i32` — the rule wolfc implements,
  now documented rather than a bug — so std keeps annotating every
  accumulator. `std.math`'s constants-as-functions stay: their other
  half is F-0026, which is unmoved.
- **E1007 is STATIC in lupin now** (#15): a call that omits `mut` where
  the signature demands it was a silent wrong answer and is a resolve
  rejection at this pin, matching wolfc's code and span. Nothing in std
  moved — the X1-correct spelling has been the only one written here
  since sc03 — but the class of bug it prevents is the one this repo
  spent sc02 working around.
- **`copy` EXISTS, in both implementations.** The fix-it that sc01 and
  sc02 recorded as "a `copy` operator that is not in the pinned grammar"
  is real: `for x in copy xs` then `xs.len` runs to `exit(0)` under lupin
  and wolfc's checked tier. std's bodies are not rewritten around it in
  this sprint (they read lengths before iterating, which needs no
  syntax), but the workaround note in `std.list`'s header is now history
  rather than law.
- **The native lane advanced by five rows.** `print` lowers to
  per-segment runtime writes at this pin, so `prelude/prelude_smoke.lu`
  runs natively where it was `unsupported`; the four old `option/*`
  idiom tests ran natively too, and were then rewritten to call the
  landed helpers, which returns them to `unsupported` (generic + row).
  The lane's refusals otherwise stand exactly as F-0026 maps them.
- **Retested and still open**: F-0004 (no trait dispatch anywhere — the
  reason seven of `std.testing`'s thirteen are freight), F-0012 and
  F-0015 (the checked tier's module-boundary ceilings), F-0018 (the
  boundary primitive, whose resolve-level half is what put
  `assert_starts_with` in the nursery), F-0026, F-0029, F-0036, F-0037.
