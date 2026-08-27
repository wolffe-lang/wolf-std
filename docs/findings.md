# Findings register

The track's house rule (sc00): a gap in what the language can express is
a **finding filed to wolf-lang**, never a workaround invented here. Every
finding gets a row; the filing link is the proof it left the building.

| id | filed | title | routed to | status |
|---|---|---|---|---|
| F-0001 | 2026-08-10 | No std search path: `use std.*` resolves against a builtin stub (wolfc) / builtin ambient (lupin), never the package; lupin resolves no nested package directory | wolf-lang (s37+ prelude/build plumbing), interp counterpart noted in the filing | [filed: wolf-lang#1](https://github.com/wolffe-lang/wolf-lang/issues/1) |
| F-0002 | 2026-08-10 | (contract F2) Absence/result over rows: rows are second-class — `!`-rows parse only in return position, so the six `std.option` helpers cannot be written; `try`⇄`Result` blocked twice over (s16 generic data + reification); plus the pin-era execution floor (no trait/method/enum dynamics) | wolf-lang s15-revisit/s37 owners | [filed: wolf-lang#3](https://github.com/wolffe-lang/wolf-lang/issues/3) |
| F-0003 | 2026-08-10 | Lowercase bare row tags (`none`, `done`, …) resolve in neither implementation at the raise site; API-CONVENTIONS §2 mandates lowercase pure marks — std spells `None`/`Done` interim | wolf-lang s15/resolve owners, interp counterpart | [filed: wolf-lang#4](https://github.com/wolffe-lang/wolf-lang/issues/4) |
| F-0004 | 2026-08-10 | (contract F3) Operator↔trait bridge: `<=>` builtin-only, yields int (lupin); enum `==` refused by wolfc at resolve; no supertrait clause spells `Ord requires Eq`; bare literals default `i32` and miss `impl … for int` bounds (E0502) | wolf-lang s14/s17 owners | [filed: wolf-lang#5](https://github.com/wolffe-lang/wolf-lang/issues/5) |
| F-0005 | 2026-08-10 | wolfc's pinned List typing: `push` typed as read receiver (X1-correct `(mut xs).push` is E0804; bare push then E1001); `.len`-then-`[i]` reads count as moves (E1001; fix-it names a `copy` operator outside the pinned grammar) — blocks `min_of`/`max_of` and every List-building wolfc lane | wolf-lang s21/s22 mem-tier owners | [filed: wolf-lang#6](https://github.com/wolffe-lang/wolf-lang/issues/6) |
| F-0006 | 2026-08-10 | str ordering divergence: lupin runs `<`/`>` on str, wolfc rejects E0409; no byte accessor exists to define it in-library — blocks `impl Ord for str` | wolf-lang typecheck/spec owners | [filed: wolf-lang#7](https://github.com/wolffe-lang/wolf-lang/issues/7) |
| F-0007 | 2026-08-10 | lupin pattern semantics: bare-ident patterns BIND instead of resolving in-scope variants/tags — first arm always matches (silently wrong), non-exhaustive matches run, qualified path patterns accepted though outside the pinned grammar | wolf-interp | [filed: wolf-interp#5](https://github.com/wolffe-lang/wolf-interp/issues/5) |
| F-0008 | 2026-08-10 | (contract F1) The iterator protocol: `Iter[T]` with `next(mut self) -> T ! {done}`, the `for` desugar, range-type ownership, builtin adoption — prototyped executing in `std.iter` | wolf-lang spec/01 + s37 owners | [filed: wolf-lang#8](https://github.com/wolffe-lang/wolf-lang/issues/8) |
| F-0009 | 2026-08-10 | (contract F4) `assert`, defined: an intrinsic, not a prelude fn (a module fn named `assert` shadows it module-wide, E0402 both implementations); the two-argument form needs arity overloading or intrinsic growth; comptime/runtime duality | wolf-lang spec comptime/forward-std owners | [filed: wolf-lang#9](https://github.com/wolffe-lang/wolf-lang/issues/9) |

| F-0010 | 2026-08-11 | lupin has no std root: `use std.X` resolves nothing against a package `std/` tree (it binds the path's LAST segment and looks for a top-level package directory) — the interpreter half of F-0001, still open after wolfc closed its half with `--std-root`/`WOLF_STD`; the rig stages a flat mirror beside the real tree | wolf-interp | [filed: wolf-interp#6](https://github.com/wolffe-lang/wolf-interp/issues/6) |
| F-0011 | 2026-08-11 | (contract Target-1) **What is builtin and what is std**: recommendation + four gaps — `List`/`Pool` language-blessed core types with std owning their API mass; `Map`/`Set`/`Deque` std-defined once s16 lands generic data types (`struct X[T]` is E0201 in BOTH implementations today); the `Hash`/`Eq` key protocol is unspecified; `Pool` exposes no `len`/`capacity`/liveness probe/iteration; `Map` cannot erase a key; the wordcount `tally[w] += 1` idiom runs nowhere (lupin: index is not a place; absent key reads `()`, not a zero) | wolf-lang s16/s21/s37 owners | [filed: wolf-lang#11](https://github.com/wolffe-lang/wolf-lang/issues/11) |
| F-0012 | 2026-08-11 | wolfc checked tier: an imported module whose items include a `trait`/`enum`/`impl` makes every importer `unsupported — module items in checked execution` — so `use std.cmp` costs a module every wolfc run row, which is why std.list's `Eq` family sits in `std/x/list_eq` | wolf-lang s23/s31 (checked execution) | [filed: wolf-lang#12](https://github.com/wolffe-lang/wolf-lang/issues/12) |
| F-0013 | 2026-08-11 | lupin false `ub(mem.ub)` on std-shaped code, two shapes: (a) a `mut` argument inside an f-string interpolation leaves a stale borrow tag — a later read is "read through tag … which is Disabled"; (b) a `mut`-mode call followed by a read-mode call whose body ALLOCATES is "foreign write … while tag … is PROTECTED". Both are false positives in the provenance model; both are avoidable only by writing around them | wolf-interp (Tier-3 provenance) | [filed: wolf-interp#7](https://github.com/wolffe-lang/wolf-interp/issues/7) |
| F-0014 | 2026-08-11 | Mutate-while-iterating does not agree: wolfc rejects statically as `fail(E1001)` (reads-as-moves, F-0005's lens — not an exclusivity rule), lupin RUNS it silently with no trap at all, where `[conf.trap.map]` predicts `exclusivity`. The sprint contract's designated triage case, recorded in `tests/list/mutate_while_iterating_trap.lu` | wolf-lang s22 + wolf-interp | [filed: wolf-lang#15](https://github.com/wolffe-lang/wolf-lang/issues/15) + [wolf-interp#9](https://github.com/wolffe-lang/wolf-interp/issues/9) — **CLOSED at sc11** by s72's D40 (`[mem.iter.excl]`): `E1013` on both rungs, `trap(exclusivity)` under lupin, the two machines agreeing at last |
| F-0015 | 2026-08-11 | wolfc checked tier: the row RAISE path inside an imported module's function is `unsupported — module items in checked execution` — the same call runs when it yields a value, and the same body inlined into the entry file runs both ways. Every `! {None}` miss test in std.list/x.deque_int is `unsupported` for this reason alone | wolf-lang s23/s31 | [filed: wolf-lang#13](https://github.com/wolffe-lang/wolf-lang/issues/13) |
| F-0016 | 2026-08-11 | `wolf fmt` splits a dotted call at the dot when a `//` comment line precedes it (`xs.push(1)` becomes `xs.` + newline + `push(1)`), idempotently — fmt is law (D34), so its output is the style, and this one is a defect | wolf-lang fmt owner | [filed: wolf-lang#14](https://github.com/wolffe-lang/wolf-lang/issues/14) |
| F-0017 | 2026-08-11 | lupin still accepts `let` reassignment after wolf-lang#2 closed compiler-side (wolfc now says E0410 with a `var` fix-it): the interpreter half of the divergence is open | wolf-interp | [filed: wolf-interp#8](https://github.com/wolffe-lang/wolf-interp/issues/8) |
| F-0018 | 2026-08-12 | (sc03 Targets 1–4) **The boundary primitive is missing**: no recoverable slice (`str.get`), no byte accessor, no `chars()`/`char` type — so no scan can advance past a code point of unknown length, and 28 of sc03's 64 functions are unwritable rather than merely unimplemented; plus `^n` resolving nowhere, `str` methods `unsupported` in wolfc, `str + str` diverging | wolf-lang s37 (core types) owners | [filed: wolf-lang#17](https://github.com/wolffe-lang/wolf-lang/issues/17) |
| F-0019 | 2026-08-12 | (sc03 Target 4) **Decision request — the unicode tables budget**: where category/case/normalization/segmentation data lives (std recommends penumbra or `std/x`, never core), with the evidence that lupin's `lower`/`upper` ALREADY do simple Unicode case mapping and `trim`/`words` already use Unicode `White_Space` — so std had to carry a 25-entry table to agree with the builtin | wolf-lang spec/std owners | [filed: wolf-lang#18](https://github.com/wolffe-lang/wolf-lang/issues/18) |
| F-0020 | 2026-08-12 | `assert(cond, msg)` traps `assert` even when `cond` HOLDS (wolfc 12ae8c2) — the two-argument intrinsic wolf-lang#9 just landed ignores its condition; the one-argument form is correct; contradicts `[conf.trap.assert]`'s "silent and effect-free when the condition holds" | wolf-lang (the #9 implementation) | [filed: wolf-lang#19](https://github.com/wolffe-lang/wolf-lang/issues/19) |
| F-0021 | 2026-08-12 | lupin: a method call on a SLICE EXPRESSION over a binding (`d[0..1].upper()`) is `unsupported` at resolve ("does not denote a place at run time") while the same shape over a literal runs — every std.str body binds the slice first | wolf-interp | [filed: wolf-interp#10](https://github.com/wolffe-lang/wolf-interp/issues/10) |
| F-0022 | 2026-08-12 | lupin: `n as f64` does not convert — the value stays an int, compares equal to ints and unequal to floats, and no diagnostic appears (wolfc correctly refuses the mixed comparison E0401); `std.str.parse_float` ships a ten-branch digit→f64 table to avoid it | wolf-interp | [filed: wolf-interp#11](https://github.com/wolffe-lang/wolf-interp/issues/11) |
| F-0023 | 2026-08-12 | lupin, the interpreter half of wolf-lang#3 and #4 (both closed compiler-side): postfix rows in param/`let` positions are E0201, lowercase bare tags do not resolve at raise sites — **and tag resolution is LAZY**, so a `return none` on an untaken branch certifies falsely, which is how sc02's F-0003 update came to be wrong | wolf-interp | [filed: wolf-interp#12](https://github.com/wolffe-lang/wolf-interp/issues/12) |
| F-0024 | 2026-08-12 | `cargo test --workspace` is deterministically RED at trunk `12ae8c2` (wolf_parse `blast_radius`: 4 added diagnostics, max 3, on `corpus/comptime/norm_witness.lu`) while that sha's trunk CI run reports success — so "the last green trunk run" is not by itself a sufficient pin criterion | wolf-lang CI/parser owners | [filed: wolf-lang#20](https://github.com/wolffe-lang/wolf-lang/issues/20) |
| F-0025 | 2026-08-12 | Integer literals ignore their context in lupin: `INT_MIN` has NO spelling (`-9223372036854775808`, `-MAX - 1`, `0 - MAX - 1` all trap `overflow` at `i32` whatever the annotation says), `var k = 0` infers `i32`, and a cross-module `-> int` call does not type its own operator (`math.int_max() - 1` traps) — the only working shape puts a typed binding on the left | wolf-interp | [filed: wolf-interp#14](https://github.com/wolffe-lang/wolf-interp/issues/14) |
| F-0026 | 2026-08-12 | Capability map of the compiler's two rungs, with std's cost per refusal: the checked tier refuses every f64 literal, every `const` USE and `&`/`^`/`>>`/`\|`; the native rung refuses `const` DECLARATIONS, `List`, generics, `print` and `str` — and refuses two modules that declare a function with the same name (a real mangling bug: `std.list.len` and `std.str.len` already collide) | wolf-lang backend/checked-tier owners | [filed: wolf-lang#26](https://github.com/wolffe-lang/wolf-lang/issues/26) |
| F-0027 | 2026-08-12 | **Silent wrong answer**: the native rung lowers `!=` on `f64` to an ORDERED comparison, so `nan != nan` is FALSE natively and true in the interpreter (`==`, `<`, `<=`, `>`, `>=` are all correct). `x != x` is the portable NaN test and was `std.cmp.total_cmp`'s; every float inequality in std now spells `!(x == y)` | wolf-lang s28 codegen | [filed: wolf-lang#22](https://github.com/wolffe-lang/wolf-lang/issues/22) |
| F-0028 | 2026-08-12 | (sc04 Target 2) The pure-wolf transcendentals and the intrinsics request: 29 functions, measured ≤ 1 ulp except `cbrt` (2) and `powf` (3), bit-identical on BOTH executing lanes over 200 pinned values — plus the two asks, that the wolf source stay the semantic reference and that `sqrt` be re-derived from the hardware instruction rather than the other way round | wolf-lang s37 intrinsics / s41 llvm | [filed: wolf-lang#25](https://github.com/wolffe-lang/wolf-lang/issues/25) |
| F-0029 | 2026-08-12 | Cross-module enum consumption: an enum's VALUES cross a module boundary but nothing that inspects them does — variant patterns do not resolve against an imported enum (lupin), methods do not dispatch, and an enum-returning call is `unsupported` in the checked tier and has "no recorded type" natively. Blocks `sort_by`, `is_sorted_by`, `binary_search_by` — and the sorting STABILITY WITNESS, which is only observable through a comparator that ignores part of the value | wolf-lang #12's family + wolf-interp | [filed: wolf-lang#23](https://github.com/wolffe-lang/wolf-lang/issues/23) |
| F-0030 | 2026-08-12 | (sc04 Target 5) A range value has no bounds accessor under any spelling (`start`/`end`/`len`/`lo`/`hi`/`first`/`last` all "no member") and `range` does not resolve as a TYPE in wolfc (E0301, both rungs) — so `contains`, `len` and `clamp_to` are unwritable (only by iterating, which hangs on `0..2^63`) and `std.range` ships one function | wolf-lang s37 core types | [filed: wolf-lang#24](https://github.com/wolffe-lang/wolf-lang/issues/24) |
| F-0031 | 2026-08-13 | (sc05 Target 1) **The format spec means two different things**: `{x:>8}` pads under lupin and is IGNORED by wolfc, which parses the spec and prints the unpadded value — a stdout divergence with no diagnostic on either side; lupin additionally implements only `[[fill]align][width]`, refusing sign/zero-pad/precision/type as `unsupported`, and silently reads `{n:08}` as width 8 with a space fill | wolf-lang s38 (f-string lowering) + spec §7.4 owners | [filed: wolf-lang#10](https://github.com/wolffe-lang/wolf-lang/issues/10) |
| F-0032 | 2026-08-13 | lupin's `as` accepts an UNKNOWN target type silently: `s as int` and `s as nonsense` both pass the value through unchanged with no diagnostic, while `n as f64` converts correctly — a typo in a cast is invisible, and so is a cast the machine has no rule for | wolf-interp | [filed: wolf-interp#17](https://github.com/wolffe-lang/wolf-interp/issues/17) |
| F-0033 | 2026-08-13 | (sc05 Target 1) **spec §7.4 does not exist**: `FORMAT_SPEC` is in the grammar with its semantics an explicit IOU, so every question a formatter must answer is unanswered — what each spec means per builtin type, what a malformed or type-mismatched spec does, and how a USER type formats. Candidate section text filed with a running reference implementation (`std.fmt`, whose functions are the spec's worked examples) plus the **`Show` proposal** (`fmt(self, spec) -> str`) and the s38 dispatch hook it needs | wolf-lang spec owners + s38 | [filed: wolf-lang#28](https://github.com/wolffe-lang/wolf-lang/issues/28) |
| F-0034 | 2026-08-13 | The module namespace is FLAT at the last path segment: `std.fmt.float` cannot import `std.math.float` (lupin: "this import completes a cycle: `float` → `float`", E0303) and cannot be imported beside it (wolfc: E0306 — while lupin silently binds one of the two and drops the other). Two facades may never grow a same-named leaf, so §10's "the float family lives in its own module" pattern is unrepeatable; sc05's module is `std.fmt.decimal` because of it | wolf-lang resolve owners + wolf-interp | [filed: wolf-lang#29](https://github.com/wolffe-lang/wolf-lang/issues/29) |
| F-0035 | 2026-08-13 | (sc05 Targets 3-4) **The encoders have no byte type**: `std.bytes` is still 0/9 (F-0018 re-tested, unchanged), so `std.hex` and `std.base64` ship over `List[int]` with a documented 0..255 element contract — and `hex.encode(str)`, the commonest use of a hex encoder anywhere, cannot exist because nothing reads a `str`'s bytes. The same root blocks `json.parse`, `json.unescape`, `fmt.truncate_to`, and forces `json.escape`'s one refusal | wolf-lang s37 core types | [filed: wolf-lang#17](https://github.com/wolffe-lang/wolf-lang/issues/17) (sc05 evidence on F-0018's issue) |
| F-0036 | 2026-08-13 | **Silent wrong answer**: a row tag that shares a name with anything in the value namespace at the raise site resolves to that THING instead of raising — `-> int ! {tagmod}` inside module `tagmod` hands the caller the module value, `else` never fires and no diagnostic appears. Found three ways in one sprint (`std.hex` raising `hex`, `std.json`'s `kind` function versus its `kind` tag, and `std.fmt.decimal` nearly raising `range` beside `std.range`) | wolf-lang resolve + wolf-interp | [filed: wolf-lang#30](https://github.com/wolffe-lang/wolf-lang/issues/30) |
| F-0037 | 2026-08-13 | **Silent wrong answer**: a function whose return type is an ENUM and whose signature carries an error row takes the MISS path on every call — `fn id(v: Value) -> Value ! {none} { v }` raises instead of returning `v`, in one line, with no diagnostic. Blocks `json.get` and `json.at`, which were written, tested and withdrawn to reviewed contracts; until it closes, no std accessor may return an enum through a row | wolf-interp (row/enum value representation) | [filed: wolf-interp#16](https://github.com/wolffe-lang/wolf-interp/issues/16) |
| F-0038 | 2026-08-13 | **Absence has no literal**: a row VALUE cannot be written as an expression — `none` resolves at a raise site and nowhere else, so `f(none)` is `unsupported` — which means no doc example can call a row-taking function and `std.option`'s six ship with prose examples instead of fenced ones | wolf-lang s15/resolve owners + wolf-interp | [filed: wolf-lang#38](https://github.com/wolffe-lang/wolf-lang/issues/38) |
| F-0039 | 2026-08-13 | **Nested rows diverge**: `T ! {none} ! {none}` parses and EXECUTES in lupin and is `fail(E0201)` at parse in wolfc, in both return and parameter position — the grammar's `type` production admits one `!`. `std.option.flatten` is the only helper whose type needs two, so it ships from the nursery to keep the module's other four out of a rejection | wolf-lang s03 grammar + s37 | [filed: wolf-lang#34](https://github.com/wolffe-lang/wolf-lang/issues/34) |
| F-0040 | 2026-08-13 | **No bottom type**: a diverging `else` handler cannot typecheck generically — wolfc types an `else \|_\| { … }` block by its last expression, `assert(false)` is `()`, and the block must produce `T` (E0401). A monomorphic helper writes an unreachable dummy; a generic one has no `T` to conjure, so `std.option.expect` ships from the nursery. lupin accepts and runs it | wolf-lang s14 typecheck + spec (D30's no-unwinding makes divergence ordinary) | [filed: wolf-lang#35](https://github.com/wolffe-lang/wolf-lang/issues/35) |
| F-0041 | 2026-08-13 | (sc06 Target 2) **The error-set alias surface** — s15's parked amendment, filed with the measured cost: 49 `pub` signatures carry a row across 16 modules, 11 distinct shapes, 45 of them one tag and 4 of them two, and NOTHING exceeds two. Core does not need aliases yet and the filing says so; what it argues is the semantics (`error Set Name = {…}` as a transparent name, never nominal — from D30) and the io taxonomy that makes it urgent in stdc02, plus the `try`⇄Result bridge's standing dependency on s16 | wolf-lang s03 grammar + s37 (std error taxonomy owners) | [filed: wolf-lang#36](https://github.com/wolffe-lang/wolf-lang/issues/36) |
| F-0042 | 2026-08-13 | (sc06 Target 6) **`wolf test` must subsume this rig without rewrites** — the s39 alignment requirements, with the rig as the working reference: directive-header compatibility (`check:`/`phase:`/`conforms:` verbatim), one trap expectation per entry file as the catch mechanism, subtest naming (Go 12166), a `--json` record stream (Go 2981), the three-lane ledger as a first-class concept, and the D36 bench-format reservation so `std.bench` can land in stdc02+ without a format war | wolf-lang s39 (+ D36 owners) | [filed: wolf-lang#34](https://github.com/wolffe-lang/wolf-lang/issues/34) |
| F-0043 | 2026-08-14 | (sc07 Targets 1-3) **A multi-tag error row cannot be branched on at all**, which is the io tier's whole vocabulary: a bare identifier in an `else` handler BINDS instead of matching (`else \|eof\|` fires for `io` too — measured on a `-> str ! {eof, io}` raising each tag), and a payload pattern is now refused by wolfc as refutable (`E0806: this pattern can fail to match, but a binding cannot`) even for a ONE-tag row, while lupin executes it. So API-CONVENTIONS §13's own row-expectation convention (`else \|Tag(p)\|`) is a compiler rejection, `std.io.input_all` cannot tell end-of-input from a read error, and the `NotFound{path}` payload retrofit the sprint contract predicted would not help until this closes | wolf-lang s14/s15 (else-handler patterns) + wolf-interp | [filed: wolf-lang#43](https://github.com/wolffe-lang/wolf-lang/issues/43) — **CLOSED at sc11**: s71 ruled the handler pattern (#43/#59), so a covering pattern binds its payload on all three lanes (`E0809` names the tags left out) and the `E0806` half is gone. With F-0052 closed at s70, both blockers of `io.input_all`/`net.read_all` are retired and both functions ship in sc11 |
| F-0044 | 2026-08-14 | **CLOSED at the sc12 (02-os) pin** (s90, wolf-lang#51/#52: fifteen new `fs_*` builtins, natively lowered in the same wave). Five contracts became code and one — an ATOMIC `rename` — was WITHDRAWN rather than shipped, because the language deliberately does not promise atomicity on a tier-1 target windows cannot keep it on. The filing as it stood: (sc07 Target 1) **The fs builtin set is nine calls wide and five std functions cannot be written honestly over it**: no `read_dir` (so no directory listing at all), no byte-level read or write (so `copy_file`/`move_file` are TEXT operations that refuse a non-UTF-8 file, and a fixed-size `read` can split a code point and raise `utf8`), no atomic `rename`, no `create_dir`/`remove_dir`, no metadata (`size`, `modified`, `is_file`). Also: `denied` and `utf8` have no portable litmus, so std ships two documented tags it cannot test | wolf-lang s38 owners (the fs builtin tier) | [filed: wolf-lang#51](https://github.com/wolffe-lang/wolf-lang/issues/51) (filed late, at sc08 — re-verified unmoved) |
| F-0045 | 2026-08-14 | **CLOSED at the sc12 (02-os) pin** (s90, wolf-lang#52: `fs_open_mode(path, mode)` with five modes, mode 2 a real `O_APPEND`/`FILE_APPEND_DATA` handle). `std.fs.append_text` is open-write-close, reads zero bytes of the file (measured: three syscalls touching a 1 MiB log, none of them a read, against eight and a whole-file read for the old body) and has lost its `utf8` row. The filing as it stood: (sc07 Target 1) **`fs_open` has no mode**: it opens read-only and `fs_create` truncates, so there is no append-mode open and no read-write handle. `std.fs.append_text` is therefore read-concat-write — linear in the FILE's size rather than the text's, non-atomic, and carrying a `utf8` row for the existing contents it must decode. The ask is an open-mode argument (or `fs_open_append`), and a positioned write | wolf-lang s38 owners | [filed: wolf-lang#52](https://github.com/wolffe-lang/wolf-lang/issues/52) (filed late, at sc08 — re-verified unmoved) |
| F-0046 | 2026-08-14 | (sc07 Target 2) **The io tier's three gaps, from writing the facade**: `conform-run` cannot inject stdin (the checked machine's `run_checked_with_input` exists but no flag reaches it), so std can witness only `eof` and `read_all`/`prompt`'s hit paths are untestable in this rig; there is no `read_all` builtin, and a line read strips the terminator, so the whole-input operation cannot be composed from `read_line` even with a working handler; and writes are infallible with no flush, so `prompt` cannot guarantee its prompt appears before the read | wolf-lang s38 + s39 (`wolf test` stdin) | its deny-warnings half is [filed: wolf-lang#49](https://github.com/wolffe-lang/wolf-lang/issues/49) with F-0053; the stdin and flush asks are re-verified unmoved at sc11 (`--deny-warnings` is still `unknown flag`) and still unfiled on their own. **RE-SHAPED at sc11**: `std.io.input_all` SHIPS — the blocker was never this finding but the handler that could not discriminate (F-0043/F-0052, both closed) — so the surviving read ask is byte-exactness, not composition: a line read strips terminators, so `"a\nb"` and `"a\nb\n"` are indistinguishable through std and `io.read_bytes` is the contract that replaces the old `read_all` one |
| F-0047 | 2026-08-14 | (sc07 Target 2) **Silent wrong answer, and a name std cannot have**: a module item whose name matches an AMBIENT PRELUDE name resolves differently in the two implementations. `pub fn read_line() { read_line()? }` in `std.io` — the obvious facade — delegates to the builtin under wolfc and recurses forever under lupin (`unsupported — call depth exceeded 512 frames`), with no diagnostic on either side; the reverse case (`assert`) shadows the intrinsic module-wide in both. std renamed its reader `input_line` rather than depend on a resolution order. The ask: retire the ambient host names now that the real std surface exists (the prelude's own comment promises exactly that), and rule the shadowing question for the names that stay | wolf-lang resolve owners + wolf-interp | [filed: wolf-lang#44](https://github.com/wolffe-lang/wolf-lang/issues/44) |
| F-0048 | 2026-08-14 | **The checked lane's verdict is not deterministic**: the same program, same binary, same inputs answers `exit(0)` or `unsupported — place projection outside the modelled surface` at random — measured 5 of 12 runs one way and 7 the other on `tests/str/byte_length_honesty.lu`, and reproduced on `tests/str/interpolation_interplay.lu` (2 of 136 tests, both `str`-heavy). Verdict stability is a conformance property, not a nicety: spec/06's differential protocol compares records across implementations, and this rig's ledger gate cannot express "sometimes" without the `unstable(…)` vocabulary this sprint had to add | wolf-lang s23/s31 (checked execution) | [filed: wolf-lang#42](https://github.com/wolffe-lang/wolf-lang/issues/42) — **CLOSED and RETIRED at the sc08 pin** (14-for-14 deterministic on both files) |
| F-0049 | 2026-08-15 | (sc08 Target 2) **The net builtin tier is seven calls wide, and the `timeout` tag it declares is unreachable**: `wolf_rt`'s net table implements a per-socket deadline and unit-tests it, no builtin exposes it, and the executing lane's sockets are plain blocking `std::net` sockets that never consult it — so no wolf program can arm a deadline and `accept`/`read`/`connect` block forever by construction. Also absent: `shutdown` (an orderly half-close), any address accessor beyond the LOCAL port as an `int`, byte-level read/write, UDP, and any reactor/`select` composition (X6's posture is s35's and does not reach std yet). `std.net` ships the vocabulary with the tag documented as unobservable rather than a helper that cannot work | wolf-lang s39 + s35 owners | [filed: wolf-lang#45](https://github.com/wolffe-lang/wolf-lang/issues/45) |
| F-0050 | 2026-08-15 | (sc08 Target 3) **A line protocol over a socket needs a byte-level read, not a buffer**: the buffered half WORKS — a `str` field mutated through a `mut` parameter across a module boundary runs on both executing lanes (measured) — but filling the buffer is a byte-count `net_read`, and a chunk that ends inside a code point raises `utf8` with the bytes already consumed. So a `read_line`-over-socket is broken the moment a stream carries a non-ASCII character, and `std.net` ships the contract instead (§9's refusal-over-approximation). The ask: a read that returns bytes, or one that leaves an undecodable tail in the socket | wolf-lang s39/s37 owners | [filed: wolf-lang#46](https://github.com/wolffe-lang/wolf-lang/issues/46) |
| F-0051 | 2026-08-15 | **A `comptime fn` cannot call across a module boundary**, so a capability module's D33 refusal cannot be witnessed through its own facade: `comptime fn probe() { net.connect(…) }` is `unsupported` at resolve rather than `E0701`, and so is a comptime call to a PURE std helper where no capability is involved — the engine has no cross-module call at this pin, and the checked record carries no reason string to say so. std's witness therefore names the builtin one level down (`tests/net/comptime_refuses.lu`) | wolf-lang s16 (CTFE engine) owners | [filed: wolf-lang#47](https://github.com/wolffe-lang/wolf-lang/issues/47) |
| F-0052 | 2026-08-15 | **Silent wrong answer, and a three-lane divergence: a `match` inside an `else` handler matches its FIRST ARM for every tag** on wolfc's checked lane. `miss_io() else \|e\| match e { eof => "said-eof", io => "said-io", _ => … }` answers `said-eof`; swap the arms and it answers `said-io` — measured both ways, no diagnostic. lupin 0.1.5 and the native rung both discriminate CORRECTLY, so one program has two meanings across three lanes. It is F-0043's successor rather than its fix: sc07's `E0201` on the shape is gone and what replaced it is worse, because a rejection cannot ship and this can (the `E0806` on payload patterns is unmoved). It costs `std.net.read_all` and keeps `std.io.input_all` a contract: a loop that cannot tell `closed`/`eof` from `io` truncates data silently | wolf-lang s14/s15 + s23 (checked execution) | [filed: wolf-lang#48](https://github.com/wolffe-lang/wolf-lang/issues/48) |
| F-0053 | 2026-08-15 | **The warning signal covers the ENTRY file only**, so the `--deny-warnings` gate F-0046 asked for would not see std at all: over 144 staged programs the record's `warnings` array reports only the entry's own spans, while `std/math/float/float.lu` carries 40-plus `0.0 - x` sites that W0402 diagnoses and no importing test surfaces one. `conform-run` still rejects `--deny-warnings` (re-verified at this pin; the flag exists on `wolf build`/`wolf test`). The warning system's first real catch is recorded with it: W0402 found 29 sites in two std TEST files, one of which claimed to assert the two signed zeros and asserted `+0.0` against `+0.0` | wolf-lang s67/s69 (warnings) + s39 (`wolf test` surface) | [filed: wolf-lang#49](https://github.com/wolffe-lang/wolf-lang/issues/49) |
| F-0054 | 2026-08-15 | **The pin's own ritual gates are load-flaky**: `cargo test --workspace` and `cargo run -p xtask -- ci` each failed once at trunk `13b811f` in a clean scratch clone and each passed on re-run, both times in `wolf_rt::task::proc`'s scheduler-seam tests (`seam_observes_proc_events` missed a `ProcExit` event; `killed_proc_skips_defers_and_frees_regions` counted 0 where it wanted 1). Run alone the crate is 14-for-14 green, so the failures are timing under full-workspace parallelism — which makes a green pin a probabilistic claim and F-0024's two-gate ritual a coin flip rather than a check | wolf-lang s32/s34 (task runtime) + CI owners | [filed: wolf-lang#50](https://github.com/wolffe-lang/wolf-lang/issues/50) |
| F-0055 | 2026-08-16 | **CLOSED at sc11 (ruled, `[mem.str.empty]`).** The empty needle is three different things: `count("")`, `split("")` and `replace(s, "", …)` are refused as `unsupported` by lupin 0.1.6 AND the checked tier, and DEFINED by the native runtime (0, one whole piece, identity) — a three-lane split on a shape every caller-supplied separator can reach. `wolf_rt` calls its answers "the documented deterministic placeholder" and `wolf_mem` refuses the same three, so both sides know; neither is ruled. `std.str` guards all six affected functions before delegating (`count` answers 0, the rest trap `assert`) so no caller sees it | wolf-lang s37 (core types) + spec owners | [filed: wolf-lang#56](https://github.com/wolffe-lang/wolf-lang/issues/56) |
| F-0056 | 2026-08-16 | **CLOSED at sc11 (ruled `assert`, `[mem.str.repeat]`).** `repeat(-1)` traps `bounds` on every lane and no clause says so — and it silently CHANGED: sc03 measured `""` under the interpreter and `std.str.repeat`'s doc claimed that answer for five sprints with no test holding it. `bounds` is also arguably the wrong kind for a caller contract violation (`[conf.trap.map]` spells that `assert`), and `wolf_rt`'s own `__wolf_rt_str_repeat` clamps with `count.max(0)` — so the three lanes agree by construction, not by rule | wolf-lang spec (`[conf.trap.map]` / `[mem.str.*]`) | [filed: wolf-lang#57](https://github.com/wolffe-lang/wolf-lang/issues/57) |
| F-0057 | 2026-08-16 | **s37 gave the language a byte VIEW and no byte SOURCE**: `s.bytes()` exists on every lane and nothing turns bytes back into a `str`, so `std.bytes.to_str -> str ! {utf8}` — the D24 border post, the last unwritten member of the census's byte block — has no spelling. Needs one of `str.from_utf8`, a `char` type with scalar-to-`str` (F-0018's half), or `strbuf.push_byte`. std ships the predicate half instead (`bytes.is_utf8`, full validation in wolf source, 31 rows on three lanes) so the gap is visible rather than silent | wolf-lang s37 core types | [filed: wolf-lang#58](https://github.com/wolffe-lang/wolf-lang/issues/58) |
| F-0058 | 2026-08-17 | **The import half CLOSED at the sc15 pin** (s108, wolf-lang#29 probe-closed: module identity is the FULL path; the leaf clash is only a binding clash and `use … as` names the second binding). Re-measured with a co-importing probe at 1b149ba: the pair RESOLVES on all three lanes, and what refuses now is each module's lanes (`std.json`'s generic bodies on the compiler rungs, `json_*` under lupin) — tests stay single-module for lane reasons, not name reasons. **The key-enumeration half is not closed and moves to F-0087**, where sc15's DOM half re-files it with the ask spelled. The filing as it stood: module identity is the last path segment (F-0034), so `std.x.json` and `std.json` are both `json` to an importer — `use std.json` beside `use std.x.json` is `fail(E0306)` on BOTH compiler rungs and `unsupported` under lupin, measured at those pins. D31's whole promise is that graduation is a MOVE (`std.x.foo` becomes `std.foo`, and the path is the release note), and this makes the two paths mutually exclusive for the campaign in which a resident and its successor coexist. Also filed with it: the query tier has no key ENUMERATION (`json_len` counts an object's members and nothing names them), so an object can be counted and not walked | wolf-lang resolve owners (the F-0034 issue) + s40 owners (the json tier) | [filed: wolf-lang#29](https://github.com/wolffe-lang/wolf-lang/issues/29) (module identity) + the json half re-verified against s40 |
| F-0059 | 2026-08-17 | **The clock ABI is milliseconds and the deadline hole is now everywhere**: `time_now_ms`/`time_unix_ms`/`time_sleep_ms` are the whole time tier, so `std.time` can offer no sub-millisecond resolution (a `_ns` face over a `_ms` source would report a thousand-fold lie) and no `Deadline` type at all — nothing in the toolchain arms a deadline anywhere (F-0049 for sockets, the same hole for everything else), and there is no `select` to race a timer against work. Also: no monotonic-clock IDENTITY, so two `Instant`s from different processes are silently incomparable with no way to detect it; and the s36 clock-hook seam does not yet reach clock READS, so `--schedules`/`--replay` cannot virtualize time and a timing test has no deterministic mode | wolf-lang s40 + s35/s36 owners | filed with sc10's evidence on the s40 tier |
| F-0060 | 2026-08-17 | **A pure builtin family is refused at comptime with no diagnostic**: `json_valid`/`json_get`/`json_type`/`json_len` carry no I13 capability and no sandbox category by design (the metadata for a package using only them stays capability-free — correct), and the comptime engine still refuses them, as `unsupported` at resolve with no code and no reason string. Every capability family answers `E0701` naming what it reaches and why (`reaches the clock, which comptime code can never touch`); the pure family answers nothing, so a package cannot learn WHY its `comptime fn` will not evaluate, and this repo cannot hold the refusal as a test the way `tests/{net,time,env}/comptime_refuses.lu` hold theirs. The ask is a diagnostic for "no evaluator at v0", distinct from the capability refusal | wolf-lang s16 (CTFE engine) + s40 owners | filed with sc10's evidence |
| F-0061 | 2026-08-17 | **`std.fmt.decimal.parse_float` is `unsupported` on BOTH compiler rungs** ("arithmetic outside integers" at the mem tier, one body deep — F-0026's f64 ceiling, still open at sc05's row), and that now costs a function rather than a lane: `std.x.json.float_at` was written and WITHDRAWN inside sc10 because the checked tier is `std.x.json`'s only executing lane, so the function would have had zero lanes, no runnable test and no fenceable doc example. A std function nobody can run is a claim, not code | wolf-lang s23/s31 (checked execution) + s28 (native) | re-verified unmoved at this pin on [wolf-lang#26](https://github.com/wolffe-lang/wolf-lang/issues/26) |
| F-0062 | 2026-08-18 | **Two keywords ate two std names, and one of them reported it badly**: `spawn` (a task AND `spawn proc`) and `handle` (the pool tier) are reserved, so `std.process` is `start` over a `Child { slot: int }`. `pub fn spawn` is a clean `E0008` with a fix-it; a keyword as a FIELD name is not — lupin says `E0008: \`handle\` is a reserved keyword` and wolfc says `E0201: expected a field initializer` pointing at the field, which reads as a typo in the struct literal below and is where this sprint looked first. The ask: one code, one sentence, both positions | wolf-lang parser owners + wolf-interp (already correct) | sc11 evidence; unfiled at report time, routed with the closeout |
| F-0063 | 2026-08-18 | **Trunk's tip failed the pin ritual's first gate**: `17ea078` banks an unfixed fmt fuzz class as `tests/regressions/unfixed/idem_class_six.lu.pending`, and `wolf_fmt`'s `fuzz_regressions` sweep `read_dir`s that corpus and `fs::read`s every entry — so the new SUBDIRECTORY panics `IsADirectory` and `cargo test --workspace` exits 101, with every banked regression unchecked and the "corpus never shrinks" assertion unreachable. The ask: filter to files (or to `.lu`) before reading. sc11 pins `0b4e79c`, its parent, green on both gates first try | wolf-lang s63 (fmt) owners | sc11 evidence; unfiled at report time, routed with the closeout |
| F-0064 | 2026-08-18 | **A pairing line is not a pin**: `wolf 0.1.0` answers `--version` on two lines and the second names lupin's sha in the wolf-interp repo ("paired with lupin 0.1.8 …, pin 7886559"). This rig's doctor read the whole output as one line, compared two repositories' histories and failed the bump. Not an upstream bug — the pairing is the right thing to report — but the first time a tool's `--version` grew a line, so any other consumer has the same latent bug. Fixed here (identity from line one; the pairing is printed, never gated) with the two-line shape held as a unit test | wolf-std rig (fixed in `xtask/src/bins.rs`); recorded for other consumers | fixed in this repo at sc11 |
| F-0065 | 2026-08-18 | **The process trio is four operations short of a Command facade**: no piped stdio (v0 wires all three of the child's streams to the null device, so `output`/`stdin_text` have nothing to read and nowhere to write), no child environment or working directory (`os_spawn(argv)` takes one argument), no non-blocking wait and no deadline (so `try_wait`/`wait_timeout` have no honest implementation), and no real pid (the `int` is the machine's child-table index). Six reviewed contracts in `std.process`'s header. What the trio gets RIGHT and std would have fought otherwise: argv-array only, no shell-string spawn anywhere, so injection is structurally impossible | wolf-lang s40 + s35 (reactor) owners | sc11 evidence; unfiled at report time, routed with the closeout |
| F-0066 | 2026-08-18 | **The happy path of a spawn is unwitnessable from a portable test**: no program exists on every tier-1 host (`/bin/sh` is not on windows, `cmd.exe` is not on linux), a wolf program cannot learn its own path (`env_args` drops argv[0], no `os_exe`), and the directive schema has no per-platform gate — so no `.lu` test in this repository can start a real child, and `std.process`'s central claim (an exit code comes back; a killed child answers `signal`) rests on the toolchain's own unix-gated tests. Every ROW is witnessed portably (empty argv, a name no host has, a forged handle). Cheapest fix by far: an `os_exe` builtin, which lets a test spawn ITSELF | wolf-lang s40 + conformance-directive owners | sc11 evidence; unfiled at report time, routed with the closeout |
| F-0067 | 2026-08-18 | **`os_cwd` has no home in std, because `chdir` does not exist**: the s40 os family is split between `std.env` (the env four) and `std.process` (the trio plus `os_exit` as `exit`), and `os_cwd` is left over — `env`-tagged so not `std.process`'s by capability, a query about this program like `args`/`vars` so `std.env`'s by shape, but there is no `os_chdir`, so std would ship a directory READ with no WRITE beside it and no way to give a child a different one either (F-0065's item 2). The ask: `os_chdir`, or a stated decision that a process-wide chdir is deliberately absent — either way `cwd` lands with a reason rather than by accident | wolf-lang s40 owners | sc11 evidence; unfiled at report time, routed with the closeout |
| F-0068 | 2026-08-18 | **`conform-run <bare-name.lu>` says the wrong thing**: a path with no directory component has an empty parent, so the package root is searched in nowhere and the message is "the package root has no wolf source files" about a directory holding exactly one `.lu` file. `./main.lu` works. No cost to this rig (it passes absolute paths) and every cost to a person at a prompt. The ask: normalize `Path::parent` of a bare name to `.`, and name the root that was searched | wolf-lang driver owners | sc11 evidence; unfiled at report time, routed with the closeout |
| F-0069 | 2026-08-18 | **`?` inside a `comptime fn` is `unsupported`, and it MASKS the capability refusal**: a row RETURN, a raise and an `else` all evaluate at comptime; `let v = inner(x)?` answers `unsupported` at resolve with an empty `diagnostics` array. The interaction is the finding — `tests/process/comptime_refuses.lu` written the obvious way (`os_kill(slot)?`) answers `unsupported` instead of `fail(E0701)`, so a D33 rejection test proves NOTHING while looking healthy, and the bare-call form trips `W0601` (a discarded `() ! {io}`) which this rig denies. Two asks: support `?` or refuse it by name (F-0060's shape again), and make the permanent check (the sandbox) win over the temporary one (the subset) | wolf-lang s16 (CTFE engine) owners | sc11 evidence; unfiled at report time, routed with the closeout |
| F-0070 | 2026-08-18 | **lupin 0.1.8 has four fifths of the os/env builtin family**: `env_args`, `env_get`, `env_set`, `os_cwd`, `os_exit` and every `time_*` call run at its own conformance pin; `env_vars` "does not resolve" — the generic unknown-name refusal, not the reasoned decline this machine gives `fs_*`/`net_*`/`json_*`/the process trio, so it reads as an oversight. Costs one ledger row (`tests/env/args_and_vars.lu`) and nearly cost three documents a wrong sentence: **a builtin FAMILY is not a unit of evidence; a builtin is** | wolf-interp | sc11 evidence; unfiled at report time, routed with the closeout |
| F-0071 | 2026-08-19 | **CLOSED at the sc12 (02-os) pin** (s88, wolf-lang#85: "a temporary can be read from"). All five refused shapes — `s.bytes()[i]`, `.get(i)`, `.first()`, `.last()`, `.count()` — execute on the checked tier now, re-measured one at a time; `tests/str/byte_view_index.lu` advanced from `unsupported` to `run` and grew the two shapes it did not cover. std did NOT undo the rewrites: the one-pass state machine is shorter than the indexed walk it replaced. **One half survives on the other machine and is its own row now**: lupin has no `first`/`last` on a `List` at all (`tests/str/byte_view_first_last.lu`). The filing as it stood: **wolfc's checked tier models two of s77's seven byte-view positions**: `for b in s.bytes()` and `s.bytes().len` run on all three lanes, while `s.bytes()[i]` is `unsupported — indexing outside the modelled surface` and `.get(i)`/`.first()`/`.count()` are `unsupported — List method on a temporary`, both at `mem` — the materialized shape (`let bs = s.bytes()` then `bs[i]`) still runs everywhere, so the gap is the TEMPORARY and not the operation. std cannot spend a lane on a performance shape, so seven bodies were rewritten onto `for`-and-counter forms (including a UTF-8 decoder turned into a one-pass state machine) rather than onto the fast indexed ones | wolf-lang s77/s23 (checked execution) | [filed: wolf-lang#85](https://github.com/wolffe-lang/wolf-lang/issues/85), held as `tests/str/byte_view_index.lu` |
| F-0072 | 2026-08-19 | **CLOSED at the sc12 (02-os) pin** (s89, wolf-lang#86: the argument position is a LEND). Eight of the nine take a view without copying, `to_str` materializes because a builtin consumer does, and a lend that would outlive its call is `E1015` rather than a silent copy. The ask was answered in the shape it was asked in — a mode-like borrow rather than a new type — and the `Bytes` type this repo wants is still open on its own merits. The filing as it stood: **A byte view cannot cross a function boundary, so `std.bytes`' nine functions are copy-only**: s77 materializes in every non-consuming position — a `let`, an argument, a return — which is the right conservative default and means `bytes.is_utf8(bytes.from_str(s))` copies `s` where `str.char_count(s)` walks it. The difference is the PARAMETER, not the implementation, and the library cannot fix it from its side. The ask is the `Bytes` type this repo has documented as an interim since sc05, or a mode that lets a callee borrow `{ptr, len}` (which a `str` parameter already is), plus a spec rule about which positions materialize — today that is discoverable only from a comment in `wolf_wir::lower` | wolf-lang s37/s77 core types | [filed: wolf-lang#86](https://github.com/wolffe-lang/wolf-lang/issues/86) |
| F-0073 | 2026-08-19 | **The `--version` pairing line is a hardcoded constant and it rots**: trunk `f8dca42` says "paired with lupin 0.1.8" one day after lupin 0.1.10 shipped, and 0.1.10's own conformance pin is ten commits BEHIND that sha — so the two binaries really are meant to be used together and the line says otherwise. The line is genuinely useful (F-0064 taught this rig to read it), which is exactly why a claim inside a shipped binary needs a mechanism keeping it true: a reader trusts it more than a note | wolf-lang release owners | [filed: wolf-lang#87](https://github.com/wolffe-lang/wolf-lang/issues/87) |

| F-0074 | 2026-08-19 | **CLOSED at lupin 0.1.12** (wolf-interp#24: the cost was the METHOD CALL copying the receiver four times, not `push`; the fix lends it — 32k pushes 30.33s → 0.191s upstream's numbers, and this repo's slowest test 41.5s → 33.5s on one loaded host). **The index read is NOT fixed and is F-0078.** The filing as it stood: **`List.push` is O(n) per push under lupin, so every `List`-returning std function is quadratic on the reference lane**: 4k/8k/16k/32k pushes take 0.46/1.91/7.83/37.53s (doubling N quadruples the time) where both compiler rungs finish 32k in 0.14s of whole-process time. Suffix slicing and `starts_with` are both linear, measured, so it is the list representation and not the scanner shape. It is the ceiling behind this rig's slowest test (`fmt/decimal/shortest_round_trip.lu`, 28-35s against a 60s per-test limit, timed out once under load this sprint) and std cannot write around it: pushing into a fresh list IS the portable spelling (sc04's rule, because index assignment runs on one lane). Also measured: 0.1.10 is ~15% slower than 0.1.8 on that test | wolf-interp | [filed: wolf-interp#24](https://github.com/wolffe-lang/wolf-interp/issues/24) |

| F-0075 | 2026-08-20 | **CLOSED at lupin 0.1.12** (one release after filing, re-measured with the finding's own reproducer at sc14: `str_from_utf8([195, 169])` is `é`, `[255]` is the `utf8` row). Two ledger rows advance and the doc rig's tier-waiver list is empty again. The filing as it stood: **lupin 0.1.11 does not have s81's `str_from_utf8`**: the language's first bytes-to-str primitive is in the compiler's prelude (both rungs execute it) and the interpreter answers ``unsupported: `str_from_utf8` does not resolve`` — the generic unknown-name refusal, not the reasoned decline that machine gives `fs_*`, `net_*`, `json_*` and the process trio, so it reads as drift rather than posture (F-0070's shape, second occurrence). It costs `std.bytes.to_str` its interpreter lane the day the function lands: two ledger rows are `unsupported / run / run` where the module's other eight are three-lane, and `bytes.is_utf8` stays hand-written rather than delegating precisely so the predicate keeps the third lane | wolf-interp | [filed: wolf-interp#26](https://github.com/wolffe-lang/wolf-interp/issues/26) |
| F-0076 | 2026-08-20 | **CLOSED at the sc12 (02-os) pin** (s88, wolf-lang#100: "native: two bools can be compared"). `tests/fmt/parse_bool.lu`'s native column is lit after six sprints dark. std keeps the `!p`-and-a-branch spellings the finding prescribed — they cost nothing and reverting them would be churn. The filing as it stood: **`bool` comparisons are `unsupported` on the native rung**: `a == b`, `a != b`, `a == true` and even `true == false` are all `unsupported — comparison outside integers/floats (str/enum compares, c06/std)` at `mem`, where `int` and `f64` have always lowered and `str` does since s81. The refusal names str and enums and does not name `bool`, which is why it has gone six sprints undiagnosed: `tests/fmt/parse_bool.lu` has carried a dark native column since sc05 for exactly this and the ledger attributed it to nothing in particular. It is cheap to write around (`!p` and a branch instead of `p == q`), and that is the argument for fixing it rather than against: nothing about a `bool` compare is hard, so a library pays a lane for a spelling | wolf-lang s28/native lowering | [filed: wolf-lang#100](https://github.com/wolffe-lang/wolf-lang/issues/100) |
| F-0077 | 2026-08-20 | **`List[int]()` inside a `comptime fn` is `unsupported` at resolve**, with no code and no reason string (F-0051's silence again), on both compiler rungs — `var k = 0`, a `str` literal, an `else` and a row return all evaluate there. The consequence is a rule rather than an inconvenience: **a pure builtin whose argument is a `List` cannot be reached at comptime at all**, because a `List` is the only way to spell the argument. `str_from_utf8` is the first such builtin and `std.bytes.to_str` is the first std function whose comptime story is "the sandbox has no objection and the engine cannot get there" | wolf-lang s16 (CTFE engine) | [filed: wolf-lang#101](https://github.com/wolffe-lang/wolf-lang/issues/101) |
| F-0078 | 2026-08-21 | **The `List` INDEX READ is O(n) under lupin, and a read-mode `List` ARGUMENT copies the whole list per call** — the half of F-0074 that 0.1.12's lend did not reach. Measured at 2k/4k/8k/16k: `xs[i]` in a loop is 0.072/0.234/0.895/3.442s (four times per doubling) where `for v in xs` is 0.016/0.027/0.053/0.107s, and 20k calls of `fn value_at(bs: List[int], at: int)` over a 20k list is 58.1s against 5.3s for the same index read inline. It is why this repo's slowest test got 20% faster rather than 100× at the bump (base-10^9 limbs, indexed), and it changed a design decision inside the sprint: `std.json.parse`'s scanner walks `text.get(i..i + 1)` instead of materializing `s.bytes()` and indexing it | wolf-interp | [filed: wolf-interp#28](https://github.com/wolffe-lang/wolf-interp/issues/28) |
| F-0079 | 2026-08-21 | **Silent wrong answer**: a multi-arm handler (`expr else \|e\| match e { … }`) takes its FIRST ARM for every tag when the row is raised by a function in an IMPORTED MODULE — measured in both arm orders (`fwd: 10 10 10`, `rev: 30 30 30` where both should be `10 20 30`), exit 0, no diagnostic. The same shape over an entry-file raise discriminates correctly on all three lanes, which is what `tests/errors/handler_discriminates.lu` holds. It is the mirror of F-0052 (the compiler's checked lane, closed at s70) and it costs the same thing: a loop that stops on one tag and re-raises the others is unwritable across a module boundary on the reference lane | wolf-interp | [filed: wolf-interp#29](https://github.com/wolffe-lang/wolf-interp/issues/29) |
| F-0080 | 2026-08-22 | **The net tier has no byte-level read or write, where the fs tier now does.** `net_read` returns a `str` and raises `utf8`, so a TCP stream carrying non-ASCII text splits a code point across two reads and the second one is a miss — a caller cannot join the pieces, because the pieces it would join do not exist. This used to be the same gap as `std.fs`'s and s90 closed the fs half (`fs_read_chunk`/`fs_write_chunk` carry `List[int]`, no `utf8` row anywhere), so the shape of the fix is settled, measured and shipped one tier over: `net_read_bytes`/`net_write_bytes` with the same `List[int]` currency and the same `invalid` row for a non-byte element. Until then `std.net.read_all` is safe only over ASCII and says so, and the `utf8` row on `net.read` is a limitation of the surface rather than a property of the data | wolf-lang s39/s90 owners | sc12 (02-os) evidence; unfiled at report time, routed with the closeout |
| F-0081 | 2026-08-22 | **lupin refuses the s90 fs names with the generic unknown-name message, where it declines the s38 ones with a sentence** — ``unsupported: `fs_read_dir` does not resolve`` beside "`fs_write_text` is the s38 io/fs surface; this machine has no filesystem (or injectable stdin) by design, so the fs tier is declined rather than mocked". Costs NO ledger row and never will: that machine has no filesystem under either sentence. What it costs is a reader's ability to tell a POSTURE from DRIFT, which is the exact distinction sc11 spent a sprint learning to record and which F-0070 and F-0075 both turned on — one of those was drift and closed on a release, the other was a decision. The ask is one line: put the s90 names in the same declined-by-design table as the s38 ones | wolf-interp | sc12 (02-os) evidence; unfiled at report time, routed with the closeout |
| F-0082 | 2026-08-21 | **impls on primitives execute nowhere** — `impl Ord2 for int` answers "methods on non-nominal self types" on both wolf lanes and "`i64` has no method" under lupin 0.1.13, measured with a fresh probe at the dd6d13c/0.1.13 pins. This is the single blocker under `sort[T: Ord]`, `cmp.min/max/clamp`, and `sort/generic_contract.lu`'s freight status: the c22 dispatch campaign moved every STRUCT-receiver dispatch and left primitive receivers exactly where F-0002 found them. The ask is a lowering (and interp) story for primitive `Self` | wolf-lang | sc13 probe; supersedes F-0002's evidence at four pins older |
| F-0083 | 2026-08-21 | **a single-line bodiless trait declaration parses under lupin and E0201s under wolfc** — `trait T { fn f(self) -> str }` on one line is accepted by lupin 0.1.13 and refused "expected a function body" by wolfc at dd6d13c (both lanes; multi-line form fine everywhere). A parse-level implementation divergence: one grammar admits a Term-free bodiless decl before `}`, the other does not. Whichever is [gram]-correct, the other is wrong | wolf-lang + wolf-interp | sc13 probe |
| F-0084 | 2026-08-21 | **a tag widened from a sub-row selects its handler arm by sub-row INDEX under lupin 0.1.13** — `overflow` raised in a `!{overflow}` helper, `?`-widened into `!{syntax, deep, overflow}`, lands the FIRST arm (`1 2 1` where wolfc's two lanes print `1 2 3`); in the wild, `json.parse("1e400")` lands the `deep` arm of a three-arm handler (index against the sorted row). Ride-out identity is intact — arm selection only, F-0079's sequel one layer deeper. `parse_misses.lu` asserts the syntax/deep arms and deliberately leaves overflow's unasserted with wolf-interp#33 linked | wolf-interp#33 | sc13, the un-split's own measurement |
| F-0085 | 2026-08-21 | **a cross-module qualified fn VALUE refuses on the compiled lanes** — `let f = str.is_ascii` is "a member access without a recorded type" on both wolf lanes at dd6d13c while lupin 0.1.13 runs it; s95's fn-value reads cover same-module names only. Half of `call.ind`'s value for std waits on this: no doc example can pass a std fn as a predicate | wolf-lang#116 | sc13 probe |
| F-0086 | 2026-08-21 | **nested fns parse and never resolve, all three lanes agreeing** — a doc example (wrapped in `fn main`) therefore cannot define a predicate, and with F-0085 open cannot import one: the callable tier's examples are prose citing test files, the sort_by precedent. A limit, not a divergence; filed for the resolve story or an honest diagnostic | wolf-lang#116 | sc13 probe |
| F-0087 | 2026-08-26 | **nothing enumerates a json object's keys, and s107 crossed the family without adding it** — `json_len` counts members and no builtin names them, so an object can be counted and never walked: `keys(doc, path)` has been a reviewed contract in `std.x.json` since sc10, and sc15's DOM half (`Node` + navigation over the kernels) inherits the hole whole (`keys_of` is the same contract over a handle). This is sc15's NAMED STOP under the no-invented-surface rule. The ask is one kernel in the family's own shape: `json_keys(doc, path) -> List[str] ! {parse, missing, kind}`, member names in document order — s90's `fs_read_dir` proved the `List`-returning builtin shape one tier over, and the parity pattern (`wolf_mem` reference + `wolf_rt` hand mirror) is already this family's | wolf-lang | [filed: wolf-lang#123](https://github.com/wolffe-lang/wolf-lang/issues/123); sc15's named stop, see the long entry below |
| F-0088 | 2026-08-26 | **the json kernels read a duplicate key FIRST-wins where `std.json.parse` keeps the LAST** — measured at the sc15 pins (see the long entry): `json_get` on `{"a": 1, "a": 2}` at `"a"` answers `1` on both compiler rungs, and `json_len` of the same object counts 2 members, where `std.json.parse` of the same text keeps `2` at `"a"` and counts 1 (a `Map` assignment; also the wider ecosystem's reading — last-wins is what JS, serde and Python do). Both are RFC 8259-legal, neither corpus witness pins it, and the two std read surfaces now answer the same document two ways. Filed for a ruling rather than absorbed; `std.x.json`'s docs state the measured behaviour with this finding cited | wolf-lang | [filed: wolf-lang#124](https://github.com/wolffe-lang/wolf-lang/issues/124); sc15 differential seam, see the long entry below |
| F-0089 | 2026-08-26 | **CLOSED at the sc18 pin (s111, wolf-lang#131)** — the narrowing cast lowers natively at 21b129e, re-probed four lines at the sha; the masked-low-half spelling it forced stays in sha2/chacha20 as a recorded follow-on (a storage respell is a deliberate change, not a bump side-effect). The original finding: **the native rung refuses `int as wrapping[u32]`** ("narrowing numeric casts (range-check semantics, s27)") while the widening `as wrapping[u64]` lowers — so SHA-256's 32-bit words cannot be assembled in the u32 domain on the lane that needs them, and `std.x.crypto.sha2` carries them in the LOW HALF of a `wrapping[u64]` with a `& 0xffffffff` mask after every add and rotation (~20 sites, each a hand-maintained invariant the vectors have to police). For a WRAPPING target the range-check question does not arise: bit-truncation is the only defensible meaning, and it is what lupin implements today | wolf-lang | [filed: wolf-lang#131](https://github.com/wolffe-lang/wolf-lang/issues/131); sc16, the digest ladder |
| F-0090 | 2026-08-26 | **CLOSED at the sc18 pin (s111, wolf-lang#132)** — `List[wrapping[u64]]` builds, pushes and reads natively at 21b129e, re-probed at the sha; the paired hi/lo tables stay as the same recorded follow-on. The original finding: **the native rung refuses `List[wrapping[u64]]` at resolve** ("this prelude container instantiation (generic data)") — and a 64-bit word cannot ride `List[int]` either, because `as int` is CHECKED everywhere and a value with the top bit set traps `overflow`. So SHA-512's eighty-word schedule and its K table are each a PAIR of `List[int]` halves, split from the verbatim FIPS literals and recombined at every use (`khi[t] as wrapping[u64] << 32 \| klo[t] as wrapping[u64]`, four recombinations per round). The ask is monomorphic: `List` of a fixed 8-byte scalar, no generic-data machinery | wolf-lang | [filed: wolf-lang#132](https://github.com/wolffe-lang/wolf-lang/issues/132); sc16, the digest ladder |
| F-0091 | 2026-08-26 | **CLOSED at the sc18 pin (s111, wolf-lang#130)** — shifts, bitwise ops and 2^63+ literals all execute at mem at 21b129e; 41 wolfc rows flipped `unsupported` -> `run` in the bump commit (4 rand + 18 sha2 + 19 chacha20), and the nine still-dark sha2 rows record the tier's step/shadow-memory BUDGETS, a different mechanism, named in the ledger. The original finding: **the checked tier refuses every shift and bitwise operator on `wrapping[T]`** ("this operator in checked execution"; `+`/`*` run) and refuses a `wrapping[u64]` literal above 2^63 - 1 earlier still ("this literal shape") — std.rand's four-sprint-old lane posture, now load-bearing: there is no spelling of SHA-2 without rotations, so the whole digest ladder (module + 1210 vendored vector assertions) is lupin+native with the checked column dark. D53 wants c28's ct tier exercised over exactly these kernels, and a lane that cannot run a rotation cannot ever check one. The filing quotes the round function | wolf-lang | [filed: wolf-lang#130](https://github.com/wolffe-lang/wolf-lang/issues/130); sc16, the digest ladder's named lane cost |
| F-0092 | 2026-08-26 | **CLOSED at the sc18 pin (s111, wolf-lang#133)** — the shape lowers natively at 21b129e, re-probed at the sha, and `update384` delegates `mut st.st` directly again (the copy dance retired in the bump commit; the diff was one line, the mechanical bar the contract set). The original finding: **the native rung refuses `f(mut param.field)` — a `mut` field path whose base is itself a `mut` parameter** ("`mut` places beyond local by-value bindings"), while the same path off a LOCAL binding lowers at 87405ac (both shapes probed; the local half healed across the sc16 bump, the parameter half did not). Costs `Sha384` its one-line delegation to the `Sha512` core: `update384` copies the core out, advances it, writes it back — semantics X1 already licenses directly, ~330 elements deep-copied per call, measured ~1.9x on the interpreter against SHA-512 on the same core | wolf-lang | [filed: wolf-lang#133](https://github.com/wolffe-lang/wolf-lang/issues/133); sc16 |
| F-0093 | 2026-08-26 | **under lupin, per-iteration cost grows with program AGE**: the same 129 CAVP SHA-384 short vectors cost ~27s as four programs and ~132s as one (16 alone: 0.69s; first 64: 10.6s) — total quadratic in vector count, not message-length scaling, and `region { }` scoping per vector measures identical. Distinct from F-0074/F-0078's per-operation costs: no per-op fix moves a curve keyed to how much the program has already run. Sized this sprint's suites: the CAVP short sets are chunked 2/4/4 to keep the differential lane inside the ceiling, and the long/Monte suites are `slow`-skipped there (the sc16 ledger word). **sc17 re-measure (the AEAD rung, 0.1.13)**: the curve holds at new constants — 32 ChaCha20-Poly1305 seal+open pairs 98s in one program, 30 opens 29s, a 10-case smoke 2.1s — so the Wycheproof AEAD parts are chunked 32/30 with `slow` and smoke subsets from the start, and each part flips back per-part at the fixing bump (evidence commented on the issue) | wolf-interp | [filed: wolf-interp#41](https://github.com/wolffe-lang/wolf-interp/issues/41); sc16, the honest-slow-skip's mechanism; sc17 evidence appended |


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

## F-0014 — mutate-while-iterating (the contract's triage case) — **CLOSED at the sc11 pin**

**Closed by s72's D40** (`[mem.iter.excl]`, wolf-lang#15). Iteration now takes
a read claim over the loop's whole extent, so `for` no longer MOVES its
iterable and a write during the loop is an exclusivity violation on its own
terms: `E1013` on both compiler rungs, `trap(exclusivity)` under lupin — which
is exactly what `[conf.trap.map]` predicted for nine sprints while one machine
over-explained it (`E1001`, through the reads-as-moves lens) and the other ran
it silently. The file that held the divergence is
`tests/list/mutate_while_iterating_trap.lu` now, and it holds the agreement.
The record below is the original.


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

## F-0043 — a multi-tag row cannot be branched on (sc07 Targets 1-3) — **CLOSED at the sc11 pin**

**The surviving half closed at s71** (wolf-lang#43, #59): a handler pattern
must COVER the row — `E0809` when it does not — and a covering pattern BINDS
its payload, on all three lanes, measured. `else |Parse(p)| { p.offset }` runs
where it was `E0806` ("this pattern can fail to match, but a binding cannot")
from sc07 to sc10, so §13's own row-expectation convention is legal at last
and `tests/errors/coarsen_and_chain.lu` stops being a held rejection. With
F-0052 (the discriminating `match`) closed at s70, the two findings that made
`std.io.input_all` and `std.net.read_all` unwritable are both gone and both
functions ship in sc11. The record below is the original.


The io tier's vocabulary is `{not_found, denied, io, utf8}` and
`{eof, io, utf8}`, and at these pins a caller cannot ask WHICH of them
fired. Two independent halves, both measured:

**A bare identifier in a handler binds, it does not match.** Given
`fn miss_io() -> str ! {eof, io} { return io }` and
`fn miss_eof() -> str ! {eof, io} { return eof }`, both of
`… else |eof| { "handler-fired" }` fire. The handler binding is a
binding — the same rule that makes `let` patterns irrefutable — so
`|eof|` catches `io` as well, silently, exactly as bare-ident MATCH
patterns bind in lupin (F-0007). There is no diagnostic; the program
reads as if it discriminated and does not.

**A payload pattern is refused outright by wolfc.** `else |Parse(p)| { … }`
— API-CONVENTIONS §13's own row-expectation convention, written in sc06
against this pin's predecessor — is now
`error[E0806]: this pattern can fail to match, but a binding cannot`,
with the note "test and destructure with `match` instead", in a position
where `match` is E0201. It fires for a ONE-tag row too
(`-> int ! {Parse(P)}`), so it is not a refutability computation this
sprint can satisfy by narrowing the row. lupin runs the same source and
destructures correctly (`v=-1 seen=3`), so this is also a divergence.

What it costs this sprint, concretely:

- `std.io.read_all`/`input_all` is NOT WRITTEN. A loop over `input_line`
  must treat `io` as the end of input, which is a silent truncation of
  someone's data. §9's "refusal over approximation" decides it: the
  function ships as a reviewed contract in the module header.
- Every row litmus in `tests/fs` and `tests/io` witnesses the tag by
  PROPAGATING it out of `main`, where the process outcome prints
  `error: <tag>` and exits 1. That is the only place in this rig where a
  tag appears as itself, and it is why `not_found_row.lu` and
  `forged_handle_io_row.lu` are written the way they are.
- `tests/errors/coarsen_and_chain.lu` is held at `fail(E0806)` on both
  compiler lanes rather than rewritten: the convention it demonstrates is
  correct and the rejection is the finding.
- The `NotFound{path}` payload retrofit the sprint contract offered as
  the fix for bare tags would not help until this closes — a payload you
  cannot destructure at the handler is worse than a mark.

The ask, in the order std would spend it: (1) `else |Tag(p)|` matches the
tag and propagates the rest — the natural D30 reading, since a handler
that covers part of a row leaves a narrower row behind; (2) failing that,
a `match` over the miss (which needs a row VALUE, F-0038's territory);
(3) at minimum, a DIAGNOSTIC when a bare-identifier handler shadows a tag
name in the operand's row, because that shape is silently wrong today in
both implementations.

## F-0044 — the fs builtin set, and the five functions above it

**CLOSED at the sc12 (02-os) pin** (s90, wolf-lang#51/#52). Fifteen new
`fs_*` builtins land and the native rung lowers all of them in the same
wave, so nothing std wrote over them has an unequal lane. Contract by
contract: `read_dir` ships (names, sorted); `read_bytes`/`write_bytes` and
`read_chunk`/`write_chunk` ship, so `copy_file` and `move_file` are byte
operations and a file holding a lone `0x80` survives both;
`create_dir`/`create_dir_all`/`remove_dir`/`remove_dir_all` ship;
`size`/`modified_ms`/`is_file`/`is_dir` ship. **One contract was WITHDRAWN
rather than filled**, and that is the finding's most interesting outcome:
`rename` was asked for as an ATOMIC move, and upstream deliberately does
not promise atomicity — POSIX replaces a destination atomically and windows
`MoveFileEx` is documented to replace but not to replace atomically, so
there is no `fs_rename_atomic` and there will not be. `fs_rename` claims
the EFFECT, `std.fs.move_file` wraps it with copy-then-remove behind the
`cross_device` row, and the atomically-promisable primitive the language
does offer everywhere is `fs_open_mode`'s exclusive create-new. std adopted
that reading instead of re-promising one level up. Two rows this repository
could never observe are still unobserved and still documented: `denied`
needs a permission call, `cross_device` needs two filesystems. `utf8` left
that list — `fs.write_bytes` can make a file that is not text, so
`tests/fs/utf8_row.lu` witnesses the tag the toolchain's way.

The filing as it stood:

Nine builtins (`fs_read_text`, `fs_write_text`, `fs_open`, `fs_create`,
`fs_read`, `fs_write`, `fs_close`, `fs_remove`, `fs_exists`) carried the
whole of `std.fs`, and the shape of what is missing is one theme: bytes,
directories, and atomicity.

- **No `read_dir`.** Nothing enumerates a directory, so `std.fs` ships no
  listing at all — and the guided project the book plans (P1 counts words
  in the files of a directory) cannot be written yet.
- **No byte-level read or write.** `copy_file` and `move_file` are
  therefore TEXT operations composed from `read_text` + `write_text`:
  they carry a `utf8` row, they refuse a binary file, and they read the
  whole file into memory. The signatures are the ones the byte versions
  will keep; the bodies are what the filing is about. `fs_read`'s
  fixed-size chunk has the same root: a `max` that lands inside a code
  point raises `utf8`, so a chunked reader is safe only over ASCII.
- **No atomic `rename`.** `move_file` is copy-then-remove and says so;
  `rename` is a contract in the module header. A crash between the two
  leaves both files, which is a durability property a library cannot
  emulate.
- **No `create_dir`/`remove_dir`, no metadata** (`size`, `modified`,
  `is_file`) — so `exists` cannot say WHAT exists, and a writer cannot
  create the directory it is about to write into.
- **Two tags std documents and cannot test**: `denied` needs a
  permission change (a unixism, and the rig is platform-agnostic by
  rule) and `utf8` needs a file containing bytes that are not UTF-8,
  which nothing in the language can write. They are documented per
  function and no test claims to have observed them.

## F-0045 — `fs_open` has no mode, so `append_text` is a rewrite

**CLOSED at the sc12 (02-os) pin** (s90, wolf-lang#52). `fs_open_mode(path,
mode)` has five modes — 0 read, 1 write+truncate, 2 append (create), 3
read-write (create, no truncate), 4 create-new (exclusive) — with a mode
outside the set decided as `invalid` before the filesystem is touched.
`std.fs.append_text` is now open-write-close over mode 2 and `open_append`
exposes the handle.

**Measured, because the finding's whole complaint was a cost** (syscalls,
not a stopwatch; `strace` on the native rung, one append of 14 bytes to a
1 MiB log, counting only calls that touch the file):

| | syscalls on the file | bytes read | bytes written |
|---|---|---|---|
| the old body, transcribed verbatim | 8 (`statx`, `openat` RDONLY, 2×`read`, `close`, `openat` WRONLY\|TRUNC, `write`, `close`) | 1 048 576 | 1 048 590 |
| `fs.append_text` at this pin | 3 (`openat` WRONLY\|CREAT\|APPEND, `write`, `close`) | 0 | 14 |

The `statx` in the first row is `fs_exists`, which is the check-then-use
race the old doc had to warn about; it is not in the second row because the
question is not asked. The `utf8` row went with the read that no longer
happens.

The filing as it stood:

`fs_open` opens read-only; `fs_create` truncates. There is no
append-mode open, no read-write handle, and no positioned write. So
`std.fs.append_text` reads the file, concatenates in memory and writes
the whole thing back: linear in the FILE's size rather than the appended
text's, non-atomic (two appenders lose a write), and carrying a `utf8`
row for contents it must decode in order to keep. The one honest
alternative — leaving the function out — is worse, because appending to a
log is the commonest file operation there is.

The ask: an open mode (`fs_open(path, mode)` or `fs_open_append`), and a
positioned or appending write. With either, `append_text` becomes three
calls that touch no existing bytes and its `utf8` row disappears from the
signature.

## F-0046 — the io tier's three gaps, from writing the facade

1. **No stdin injection through the front door.** The checked machine
   takes a stdin buffer (`run_checked_with_input`) and `conform-run`
   exposes no flag for it, so this rig can witness `read_line`'s `eof`
   and nothing else: `prompt`'s hit path and every reader loop are
   untestable here, and `tests/io/input_line_eof.lu` says so in its
   header rather than pretending. The ask is a `--stdin=<file>` (or
   `--stdin-text=`) on `conform-run`, and the same on `wolf test` when
   s39 lands — F-0042's requirement list gains an eighth item.
2. **No `read_all` builtin, and the line read drops the terminator.**
   Even with a working tag-selective handler (F-0043), a whole-input
   operation composed from `read_line` cannot know whether the input
   ended with a newline, because that byte is consumed and dropped. A
   `read_all` builtin returning the bytes as they are is the fix; the
   std contract is written and waiting.
3. **Writes are infallible and unflushable.** `print`/`eprint` return
   nothing and raise nothing, so a closed stdout is unreportable, and
   there is no flush — which means `prompt` cannot guarantee its prompt
   reaches the terminal before the blocking read. Both are recorded
   rather than emulated: a row that always misses would be a lie, and a
   flush std cannot perform is not a function.

Recorded with it, because it is the same interface: **`conform-run`
rejects `--deny-warnings`** (`unknown flag`), so the warning gate that
`wolf build` implements cannot be exercised on the lane this repo runs.
The record's `warnings` array is the workaround, and it is enough for a
report — over all 136 staged programs at this pin the array is EMPTY
every time, so the std tree is warning-clean going into s69 — but a gate
needs the flag.

## F-0047 — a module item that shares an ambient prelude name

**Silent wrong answer, and the implementations disagree about which way.**
`std.io` wanted to be written the obvious way:

```
pub fn read_line() -> str ! {eof, io, utf8} {
    read_line()?
}
```

Under wolfc the inner call resolves to the PRELUDE builtin and the
facade works (measured: a module `read_line` returning a plain `str`
still typechecked its body's `?`, which only the builtin's row can
license, and a qualified `io.read_line()` from an importer reached the
module item). Under lupin the inner call resolves to the module's own
item: unbounded recursion, reported as
`unsupported — call depth exceeded 512 frames`. One source, a facade in
one machine and a stack overflow in the other, no diagnostic on either
side.

The reverse case is already known and is the other half of the same
question: a module item named `assert` shadows the intrinsic MODULE-WIDE
in both implementations (F-0009 / sc01), severing the module from the
trap it needs. So the resolution order between "ambient prelude name" and
"module's own item" is unspecified, implemented two ways, and
consequential in both directions.

std's answer for now is to give the reader a name of its own —
`std.io.input_line` — rather than depend on an order neither spec nor
implementation has ruled on. The asks: retire the ambient host stubs
(`read_text`, `read_line`, `net_fetch`, `env_var`, `clock_ms`,
`random_seed`) now that the real std surface exists, which the prelude
table's own comment already promises ("retire the moment the real std
surface (s05) replaces them"); and until then, rule the order and
diagnose the collision, because a name that means two things in two
machines is not a name.

## Retirements and movements at the sc07 pins

The pin bump is wolf `29a9d9c` → trunk **`f0da6e6`** (both ritual gates
green in a clean scratch clone: `cargo test --workspace` and
`cargo run -p xtask -- ci`, "all steps green" — F-0024's lesson applied
for the fourth time) and lupin stays at **0.1.4** (0.1.5 is in flight and
deliberately not chased). Five fan-out lanes merged upstream between the
two shas — s32/s33 (tasks, channels), s37 (the `str` core types), s38
(fmt, io, fs), s67 (the warning system) — and three of them reach this
repo. Every claim below was re-measured at the new pins.

- **F-0015 RETIRED.** A row RAISE inside an imported module's function
  executes on the checked lane now (`ca41dc3`, "row raises execute in the
  miri-lite cross-module"). This is the single biggest ledger movement
  this repo has recorded: 51 rows advance from `unsupported` to `run` in
  the wolfc column, and every "the miss path is lupin-only" note this
  ledger carried since sc02 is gone. `std.fs` exists because of it — a
  facade over a builtin tier is nothing BUT propagation.
- **F-0018's compiler half RETIRED, and it is the biggest prize in the
  Phase-A census.** The boundary primitive landed as
  `s.get(a..b) -> str ! {none}` (`1833c4a`, anchor `mem.str.get`) inside
  an 18-method builtin `str` set that also includes `find`, `rfind`,
  `split`, `count`, `ends_with`, `strip_prefix`/`strip_suffix`,
  `replace`, `trim_start`/`trim_end` and `bytes`. The RESOLVE-level half
  is gone with it: a `str` method inside an imported module no longer
  costs its importers the wolfc lane (measured — `std.fs`'s path helpers
  use `rfind` and `get`, and every importer runs), which is what advanced
  13 `str`/`strbuf` tests and both `x/testing_text` rows. What remains
  open is the INTERPRETER half: lupin 0.1.4's `str` subset still has no
  `get`, `find`, `rfind`, `split` or `ends_with` (measured:
  "`str` has no method `ends_with` in this machine's std subset"), so
  the 30 blocked functions of the census are writable for one lane. They
  are a `std.str` sprint's prize, not this sprint's, and the census's
  Phase-B section records the size of it.
- **F-0031 RETIRED.** The format spec is no longer ignored by wolfc:
  `tests/fmt/pad_and_align.lu`, `sign_and_group.lu` and
  `decimal/fixed_and_exp.lu` all run on the checked lane and the rig's
  pairwise record comparison finds no stdout divergence against lupin —
  which is exactly the divergence F-0031 filed. (`1071388`: one comptime
  spec parser, `E0412`/`E0413` with spans inside the literal;
  `1833c4a`: native `print` refuses a spec it cannot honour instead of
  silently dropping it.)
- **F-0036's compiler half RETIRED; the interpreter half is now WORSE
  than filed.** A raise-position bare identifier resolves against the
  declared row first under wolfc (`1833c4a`, "closes #30"): a module with
  both a `parse` function and a `parse` tag raises the tag and `else`
  fires. lupin 0.1.4 still hands the caller the FUNCTION — measured as
  `miss=tagmod::parse` on stdout, a module-qualified function value
  printed where an `int` was declared, with no diagnostic. The house rule
  (grep every new tag against the module's items) stays until the
  interpreter half closes.
- **Retested and still open**: F-0004 (no trait dispatch anywhere),
  F-0011 (generic data types), F-0012 (narrowed, unmoved: the five
  `std.cmp` tests are still `unsupported` at the module boundary),
  F-0014, F-0016 (`wolf fmt` still splits a dotted call under a `//`
  comment — it bit `tests/fs/copy_and_move.lu` during this sprint and the
  note moved into the file header, as the guide says), F-0025's last
  third (`var k = 0` is `i32`), F-0026, F-0027, F-0029, F-0037,
  F-0038, F-0039, F-0040.
- **A test-side movement worth recording, because it is not a
  workaround.** `tests/hex/round_trip.lu` and
  `tests/base64/round_trip.lu` became `fail(E1001)` at this pin: an
  `else` FALLBACK that names a container binding MOVES it, so a
  `let empty = List[int]()` reused as the fallback of two different
  `decode` calls is a use-after-move. The fix is the language's own
  operator, which both implementations run — `else copy empty` — and both
  tests now run on the checked lane too. The reads-as-moves lens is
  right; the tests were written before `copy` existed.
- **A drift note for the anchor registry.** The s67 lane registered ten
  `diag.*` anchors in `spec/anchors.json` without amending
  `[conf.anchor.ns]`'s namespace list in spec/05, so a `conforms: diag.…`
  tag would be a CI failure in this rig ("a tag outside all registered
  and reserved namespaces"). Nothing in std wants one yet; the rig's
  `REGISTERED_NS` stays a copy of the spec clause, and this is recorded
  so the next sprint that reaches for a diagnostics anchor knows why it
  cannot have one.

## F-0048 — the checked lane answers at random

The same file, the same binary, the same `--std-root`, twelve runs:
five `exit(0)`, seven
`unsupported — place projection outside the modelled surface @3272..3279`.
No seed, no concurrency, no filesystem state involved
(`tests/str/byte_length_honesty.lu` asserts byte lengths of literals).
`tests/str/interpolation_interplay.lu` does the same at a lower rate; the
other 134 tests were stable over repeated runs, and the two that are not
are the two most `str`-literal-heavy files in the tree — which points at
whatever the refusal's span is being computed from (the span does not
correspond to any place expression in the staged sources; offsets 3272..3279
land inside a COMMENT in `std/str/str.lu`, so the record's span and its
source have parted company somewhere too).

Why this is more than a flake:

- **Verdict stability is a conformance property.** spec/06's differential
  protocol exists to compare one implementation's record against
  another's; a machine that answers two ways cannot be differenced, and
  the corpus gate upstream is exposed to exactly the same coin flip.
- **A refusal is supposed to be a capability statement.** "This construct
  is outside the modelled surface" is a fact about the construct, so it
  cannot depend on the run.
- **It forced a rig change.** `tests/ledger.toml` grew an
  `unstable(run|unsupported)` value (documented in `xtask/src/ledger.rs`
  and `CONTRIBUTING.md`): the honest record of a nondeterministic pin,
  loud in every `std-test` summary, and narrowing back to one value the
  day this closes. Nothing else in this repo has ever needed it, and
  nothing else should.

The ask: make the refusal deterministic (an iteration order over a
`HashMap` is the usual culprit for this exact shape), and give the
`unsupported` record's span a file so the construct can be named.

## Retirements and movements at the sc08 pins

The pin bump is wolf `f0da6e6` → trunk **`13b811f`** ("merge the #40 sprint:
native str, List, and fs — the three-consumer gate opens (wave-four lane
four, wave closed)") and lupin **0.1.4 → 0.1.5** (tag `v0.1.5`, which names
`f0da6e6` as its own conformance pin — the lawful two-upstream drift). Both
ritual gates were run in a clean scratch clone at the sha and both are green
— each on its SECOND attempt, which is F-0054.

This is the largest ledger movement this repo has recorded and it is almost
all in one column: **44 rows moved, 41 of them native.**

- **The native column: 23 → 64 of 136 rows** (`unsupported` → `run`, every
  one of them). By module: `str` 8/8 of the previously dark rows, `strbuf`
  4/4, `fs` 6/7 (the seventh is the E1001 rejection witness, already `run`),
  `io` 2/2, `base64` 3, `fmt` 6, `hex` 3, `iter` 2, `x/deque_int` 4,
  `errors` 1, `list` 1, `unicode` 1. #40 landed native `str`, `List` and the
  fs/io builtin tier at once, so every refusal this repo mapped in F-0026 as
  "the native rung refuses `str`/`List`/`print`" is gone with it, and the two
  os modules gained a second executing lane. What still refuses natively:
  generics and rows-over-generics (`option`, `list`'s generic family),
  `trait`/`enum`/`impl` modules (`cmp`, `sort`, `json`), `Map`, module-level
  `const`s, and the whole `net` tier by name.
- **The lupin column: 114 → 116.** `fs/path_helpers.lu` runs because lupin
  0.1.5 has the full 18-method builtin `str` set, and `io/writers_and_streams.lu`
  runs because 0.1.5 has `eprint`/`eprint_raw`. lupin still has no `fs_*`, no
  `read_line` and no `net_*` — by design, and it says so in the refusal.
- **The wolfc column: 100 → 101, and the `unstable` vocabulary is retired.**
  `str/to_strbuf_round_trip.lu` advanced, and the two `unstable(run|unsupported)`
  rows narrowed to plain `run`: 14 consecutive runs of each file plus the rig's
  own run, all `exit(0)`, against sc07's measured 5-of-12. **F-0048 is
  RETIRED** (wolf-lang#42, closed upstream) and nothing in the tree carries
  the vocabulary now — as intended, it was a truthful record of one pin and
  not a lane.

Retirements and re-verifications, each re-measured rather than assumed:

- **F-0018's INTERPRETER half is RETIRED.** lupin 0.1.5 answers `find`,
  `rfind`, `split`, `get`, `ends_with`, `strip_prefix`, `strip_suffix`,
  `replace`, `trim_start`, `trim_end`, `count` and `bytes` — the whole s37
  set the compiler got at the sc07 pin. Nothing in sc08 spends it, and it is
  the biggest prize in the Phase-A census: the 30 census-blocked functions
  that were "writable for one lane" at sc07 are now writable for BOTH, which
  retires the question `phase-b-census.md` §5 left for the next `std.str`
  sprint (it no longer has to choose between a narrow and a wide reading of
  the doc-truth rule).
- **F-0043 is HALF superseded and half unmoved — see F-0052.** The `E0201`
  on a `match` inside an `else` handler is gone, and what replaced it is a
  silent wrong answer on the executing lane. The `E0806` on a payload pattern
  is exactly where sc07 left it: `tests/errors/coarsen_and_chain.lu` is still
  `fail(E0806)` at typecheck on both compiler lanes (re-measured), so §13's
  own row-expectation convention is still a compiler rejection and the row is
  still held as one.
- **F-0046 is UNMOVED on all three asks.** `conform-run` still rejects
  `--deny-warnings` and still has no stdin flag (`--stdin=` is an unknown
  flag), and there is still no `read_all` builtin or flush. The deny-warnings
  signal is therefore reported the same way sc07 reported it, from the
  record's `warnings` array — with a new caveat that is now its own finding
  (F-0053): the array covers the entry file only.
- **The warning system's first catch, and it was a real bug.** Over the 144
  staged programs at this pin the `warnings` array was non-empty for two
  files, 29 occurrences, all `W0402` ("`0.0 - x` is not negation" — the s67/s69
  wave implementing sc04's own lesson). `tests/testing/near_and_ulps.lu`
  claimed in its header to assert "the two signed zeros" and asserted `+0.0`
  against `+0.0`, because `0.0 - 0.0` is `+0.0` under round-to-nearest; it now
  writes `-0.0` and means it. `tests/fmt/decimal/parse_float_shapes.lu`'s 26
  sites were a sentinel `-1.0` spelled the old way. After the fixes all 144
  programs report an empty array.
- **`wolf test` EXISTS at this pin** (s39's third target), which moves F-0042
  from "requirements filed against an unbuilt tool" to "requirements to check
  against a built one". Nothing in this rig switches to it: the coupling
  doctrine is `conform-run` plus the record protocol, and a rig migration is a
  sprint of its own.
- **Three sc07 findings were never FILED and are now** (the house rule is that
  a finding leaves the building): F-0044 → wolf-lang#51, F-0045 →
  wolf-lang#52, both re-verified unmoved at this pin, and F-0046's
  deny-warnings half rides F-0053's issue. F-0046's stdin and
  `read_all`/flush asks are still unfiled on their own and are the next
  sprint's to place.
- **Retested and still open**: F-0004 (no trait dispatch anywhere), F-0011,
  F-0012 (the checked tier's `trait`/`enum`/`impl` module ceiling — five
  `std.cmp` rows unmoved), F-0014, F-0016 (`wolf fmt` still splits a dotted
  call under a `//` comment — fourth sprint running), F-0025's last third,
  F-0026 (narrowed hard by the native flip, not closed), F-0027, F-0029,
  F-0030, F-0034, F-0035, F-0036's interpreter half, F-0037, F-0038, F-0039,
  F-0040, F-0044, F-0045, F-0047.

## F-0049 — the net builtin tier, and the deadline that exists but cannot be armed

Seven builtins (`net_listen`, `net_port`, `net_accept`, `net_connect`,
`net_read`, `net_write`, `net_close`) carry the whole of `std.net`, over the
row vocabulary `{refused, timeout, closed, utf8, io}`. They are enough for a
TCP echo — the module's eight tests and ten doc examples prove it on the
checked lane — and the shape of what is missing is one theme: **nothing can
wait with a bound, and nothing can say less than "closed" or more than "the
local port".**

- **`timeout` is declared and unreachable.** `wolf_rt`'s net table implements
  a per-socket deadline (`set_deadline`, with its own unit tests, including
  one named `timeout_row_reachable`), and no builtin exposes it. The
  executing lane does not even go through that table: the checked machine
  opens plain blocking `std::net` sockets directly, so a deadline armed by
  some future builtin would have to be plumbed there too. Consequence for a
  program: `accept` with no peer coming, `connect` to a black hole and `read`
  from a silent peer all block forever, and there is no `select` at this tier
  to race them against a timer (X6's composition story is s35's). Every
  `std.net` signature already carries the tag, so the fix is additive — but
  every std TEST has to be written so that the peer is the program itself,
  which is a discipline the rig cannot enforce.
- **No `shutdown`.** Closing the socket is the only way to tell a peer "I am
  done sending", so a request/response protocol where the client signals
  end-of-request by half-closing cannot be written, and `read`-to-close on
  the other end is the only termination signal there is.
- **No address surface beyond the local port.** `net_port` answers an `int`
  for the local end; there is no peer address, no local address string, no
  family. A server cannot log who connected.
- **No byte-level read or write**, which is why `read` carries a `utf8` row
  at all (a chunk boundary inside a code point is a miss and the bytes are
  gone) and why the line-protocol helper is F-0050.
- **No UDP, no name resolution as an operation.** `net_connect` hands its
  string to the host, so a hostname WOULD resolve through the host resolver —
  which std neither uses nor tests, because the no-external-network law is
  absolute here.
- **Two tags std documents and cannot test**: `timeout` (above) and `utf8`
  (it needs a peer that writes bytes that are not UTF-8, and nothing in wolf
  can write them — the compiler's own s39 test spawns a Rust thread to do it).
  Both are documented per function and no test claims to have observed them.

## F-0050 — a line protocol needs a byte read, not a buffer

The obvious blocker for `read_line`-over-a-socket is state, and state turns
out to be free: a public struct with a `str` field, mutated through a `mut`
parameter across a module boundary, WORKS on both executing lanes
(measured — `stash(mut r, text)` then two `take_line(mut r)` calls returning
`one`/`two` and then missing, identically under lupin 0.1.5 and the checked
tier). So a buffered reader is writable in principle.

What is not writable is FILLING the buffer. Every read is a byte count over
a `str`, and a chunk that ends inside a code point raises `utf8` with the
bytes already consumed — so a line reader is correct only while the stream
is ASCII, and silently loses data the moment it is not. §9's rule decides
it: a function that is right on ASCII and corrupts on text is not a partial
function, it is a broken one. `std.net` ships the contract in its header.

The ask, in the order std would spend it: (1) a read that returns bytes
(`net_read_bytes(fd, max) -> List[int]`, or the `Bytes` type of F-0035);
(2) failing that, a read that leaves an undecodable tail IN the socket rather
than consuming it, which makes a re-read with a larger `max` a correct
recovery; (3) a `net_read_line` builtin, which is the least general and would
still be worth having.

## F-0051 — a `comptime fn` cannot call across a module boundary

A capability module's D33 refusal is supposed to be witnessable: §14 requires
every os-facing function to say that comptime refuses it, and a claim in a doc
that nothing tests is a claim. It cannot be witnessed through the facade at
this pin.

```
comptime fn probe() -> int ! {refused, timeout, io} {
    net.connect("127.0.0.1:1")?      // unsupported at resolve, NOT E0701
}
```

The same shape one level down is exactly right — a `comptime fn` calling
`net_connect` directly is `fail(E0701)` at typecheck on both compiler rungs,
with the catalog's "reaches the network, which comptime code can never touch"
— so the sandbox table is doing its job and the CALL is what fails first.
Measured with no capability involved at all: a `comptime fn` returning
`net.endpoint("127.0.0.1", 0)`, a pure string function, is also `unsupported`
at resolve. So this is the engine's cross-module call, not the sandbox.

Two asks: make the comptime engine resolve and evaluate a std module's
functions (pure ones at least — a comptime `fs.join` or `net.endpoint` is
ordinary metaprogramming), and give the checked record's `unsupported`
verdict a reason string, because at this pin the record says nothing about
WHY and the human-readable output says nothing either.

`tests/net/comptime_refuses.lu` therefore witnesses the builtin, and its
header says so rather than implying the facade was tested.

## F-0052 — a `match` in a handler matches its first arm, on the lane that executes

**This is F-0043's successor, and it is worse than its predecessor, because a
rejection cannot ship and this can.** sc07 recorded two rejections: a `match`
inside an `else` handler was `E0201`, and a payload pattern was `E0806`. The
first is gone at this pin and what replaced it is a silent wrong answer; the
second is unmoved (re-measured on `tests/errors/coarsen_and_chain.lu`).

```
fn miss_io()  -> str ! {eof, io} { return io }
fn miss_eof() -> str ! {eof, io} { return eof }

miss_io()  else |e| match e { eof => "said-eof", io => "said-io", _ => "?" }
miss_eof() else |e| match e { io => "said-io", eof => "said-eof", _ => "?" }
```

The checked lane prints `said-eof` for the first and `said-io` for the second:
**the first arm matches whatever the tag is.** Swap the arms and the answers
swap with them, which is the measurement that rules out any reading other than
"a bare-identifier arm binds". No diagnostic, on either line.

lupin 0.1.5 and the native rung both answer correctly (`said-io`,
`said-eof`). So this is also a three-lane DIVERGENCE on a program every lane
runs — the shape spec/06's differential protocol exists to catch, and the
shape this rig's own divergence gate would fail on if the program were a test.
It is not one: a test that passed by agreeing with the wrong answer would
fossilize the bug, so `tests/net/closed_row.lu` deliberately contains no
discriminating handler and says why in its header.

What it costs, concretely:

- **`std.net.read_all` is NOT WRITTEN.** It was written, tested and withdrawn
  inside this sprint: the loop must stop on `closed` and re-raise
  `timeout`/`utf8`/`io` unchanged, and on the executing lane it would take the
  `closed` arm for every one of them — a silent truncation of a caller's
  stream, which §14 forbids in the same words it used for `std.io.input_all`.
- **`std.io.input_all` stays a contract** for the same reason, one sprint
  after being blocked on the opposite problem.
- **A tag's identity is still witnessable only by propagating it out of
  `main`**, where the process outcome prints `error: <tag>`. Six of this
  repo's row litmuses are written that way and none of them can be written
  any other way.
- **API-CONVENTIONS §13's row-expectation convention is still a rejection.**
  `else |Tag(p)|` is still `E0806` ("this pattern can fail to match, but a
  binding cannot") on both compiler lanes, so the convention sc06 wrote and
  sc07 could not use is unusable for a third sprint — and this finding says
  that even if it compiled, the arm-matching semantics beneath it would need
  re-measuring before std leaned on them.

The ask, in order: (1) resolve a bare identifier in a handler's `match` arm
against the operand row's TAGS before treating it as a binding, which is the
same rule wolfc already applies at a RAISE site (wolf-lang#30, closed); (2)
failing that, diagnose the shadowing, because a pattern that silently catches
everything is the one failure mode a reader cannot see; (3) and either way,
make the three lanes agree — one program, one meaning.

## F-0053 — the warning signal covers the entry file only

The deny-warnings gate F-0046 asked for is still not available on the lane
this repo runs (`conform-run: unknown flag --deny-warnings`, re-verified;
the flag exists on `wolf build` and now on `wolf test`), so the signal is
still read from the record's `warnings` array. At this pin that array turns
out to be narrower than the claim it was used to support.

Measured over all 144 staged programs (each staged exactly as the rig stages
it: entry file plus the whole `std/` tree, `--std-root` at the staged root):
the array reports diagnostics from the ENTRY file only. `std/math/float/float.lu`
carries more than forty `0.0 - x` sites — deliberately, per sc04's own
reasoning, and one of them is the literal `0.0 - 1.0` that W0402 flags 26
times when it appears in a test — and not one importing test surfaces a
single warning. So "the std tree is warning-clean" is not a claim this signal
can make; "no staged ENTRY file warns" is.

Recorded with it, because it is the same wave: **W0402 earned its keep on its
first run.** `tests/testing/near_and_ulps.lu` said in its header that it
asserted "the two signed zeros (one step apart, not zero — they are distinct
bit patterns)" and asserted `0.0 - 0.0` against `0.0`, which is `+0.0` against
`+0.0` — the exact wrong result sc04 wrote a guide entry about and this repo
then committed anyway. The lint found it; the test now writes `-0.0`.

Two asks: `--deny-warnings` on `conform-run` (F-0046's item, restated because
it is now the only thing standing between this repo and a warning GATE), and
warnings collected for every module in the staged package, not only the entry
— a library's own source is exactly where a lint like W0402 pays.

## F-0054 — the pin's ritual gates are load-flaky

F-0024 gave this repo a two-gate pin ritual: `cargo test --workspace` AND
`cargo run -p xtask -- ci`, both in a clean scratch clone at the sha. At
`13b811f` each gate failed once and passed on re-run, and both failures were
in the same place:

- `wolf_rt::task::proc::tests::seam_observes_proc_events` — the observed
  scheduler-event set was missing `ProcExit { kind: 2 }` (release profile, full
  workspace run), with a `Box<dyn Any>` panic logged from `task/pool.rs:150`
  alongside it.
- `wolf_rt::task::proc::tests::killed_proc_skips_defers_and_frees_regions` —
  `assert_eq!(left: 0, right: 1)` (debug profile, `xtask ci`'s test step).

Run alone the crate is green 14 times out of 14 (8 single-test runs plus 6
whole-crate runs), and the whole workspace is green on its second attempt, so
the failures are timing under the parallelism of a full `cargo test`. That
makes a "green trunk" claim probabilistic exactly where F-0024's lesson tried
to make it deterministic: a pin verified by two runs that each fail ~half the
time on a busy machine is not verified.

The ask: make the proc seam tests wait on the event they assert rather than on
elapsed time (or serialize them), so that a red gate means a red pin. This
repo's own posture in the meantime is to state the re-run in the pin's
`vendor/tools.toml` note rather than quietly re-running until green.

## F-0055 — the empty needle is three different things — **CLOSED at the sc11 pin**

**Ruled by s71** (`[mem.str.empty]`, wolf-lang#56): an empty needle counts 0,
splits to one whole piece, and replaces nothing. Measured on all three lanes
at the sc11 pin, where two of them used to refuse the call as `unsupported`
and the third answered. The six guards `std.str` carried are deleted —
`count`, `split` and `replace` delegate, and the three functions std writes
itself (`splitn`, `rsplit`, `replacen`) state the ruled answer explicitly
because they walk `find`, which answers 0 for an empty needle forever. Nothing
in `std.str` traps on an empty argument any more, and the two `…_trap.lu`
files that held the guards are `split_empty_separator.lu` and
`replace_empty_pattern.lu`, holding the ruling instead. The record below is
the original.


The sprint that spends F-0018's prize found the prize has one hole in it,
and it is the hole every caller-supplied separator falls through.

```
"abc".count("")            // lupin: unsupported · checked: unsupported · native: 0
"abc".split("")            // lupin: unsupported · checked: unsupported · native: ["abc"]
"abc".replace("", "-")     // lupin: unsupported · checked: unsupported · native: "abc"
```

Both sides of the toolchain know they are doing this. `wolf_rt`'s
`__wolf_rt_str_count` documents "Empty needle: 0 (the documented
deterministic placeholder — the checked lane refuses, see the design
note)", and `wolf_mem`'s ubcheck answers `refuse("count of an empty
needle")` for the same call. So this is not a bug in one lane: it is an
unruled semantics with two implementations of "we decided not to decide"
and one implementation of a decision.

**Why `unsupported` is the worst of the three answers.** A trap would be a
program outcome a test can name; a defined answer would be a contract a
library can delegate to. `unsupported` is neither — it says "this
implementation cannot do this", which is plainly false (all three can), and
it takes a whole ledger row with it: a std function that reached this shape
would be `unsupported` on two lanes for one branch of one input.

**What std did.** Every `std.str` function that takes a separator or a
pattern guards before it delegates:

- `count(s, "")` returns 0 — the reviewed contract's answer ("a count of
  nothing is nothing"), and now std's own rather than the builtin's;
- `split`, `splitn`, `rsplit`, `replace`, `replacen` trap `assert` on an
  empty separator — also the reviewed contract's, and held as
  `tests/str/split_empty_separator_trap.lu` and
  `tests/str/replace_empty_pattern_trap.lu` on all three lanes;
- `split_once`/`rsplit_once`/`find_all` are defined over `find`/`rfind`
  instead, which ARE defined for the empty needle (0 and `len`
  respectively, on every lane), and each says so in its doc.

The cost is six guards and a branch per call. The benefit is that
`std.str`'s answer to an empty separator is the same sentence on every
lane, which is the only thing a library can honestly promise.

Asks, in order: rule the semantics (native's answers are the obvious
ruling) and make the other two lanes obey; failing that, make it a `trap`
everywhere rather than an `unsupported` on two lanes; either way, do not
leave a shape where two rungs refuse and one answers.

## F-0056 — `repeat(-1)` traps, and the doc that said otherwise rotted — **CLOSED at the sc11 pin**

**Ruled by s71** (`[mem.str.repeat]`, wolf-lang#57): a negative repeat count
is a caller contract violation, so the kind is `assert` and not `bounds`.
Measured on all three lanes. The claim has now changed twice — `""` under the
sc03 interpreter, `trap(bounds)` everywhere at sc09, `trap(assert)` everywhere
at sc11 — and the second change was caught by the rig within a minute of the
pin bump because sc09 wrote `tests/str/repeat_negative_trap.lu` instead of a
sentence. That is the whole argument for §13's rule, paid back with interest.
The record below is the original.


At this pin `"ab".repeat(0 - 1)` is `trap(bounds)` with clause
`[mem.ub.defined]` on all three lanes ("a repeat count cannot be
negative"). sc03 measured it as `""` under the interpreter and wrote that
into `std.str.repeat`'s doc — "the implementation's answer, recorded
because it is observable (`"ab".repeat(0 - 1)` is `""`, not a trap)" —
where it sat for five sprints and four pin bumps.

Three things are wrong here and only one of them is upstream's.

1. **Ours**: a doc claimed an implementation's observable behaviour and no
   test held it, so the claim rotted silently. Fixed by
   `tests/str/repeat_negative_trap.lu`, which names the kind. The general
   rule, now in the guide: a sentence about what an implementation ANSWERS
   is a test, or it is a rumour.
2. **The spec's**: nothing says a negative repeat count is a fault, or
   which kind it raises. `[conf.trap.set]` lists the kinds and
   `[mem.ub.defined]` is what the implementations cite, but no clause
   connects the two for this operation.
3. **Arguably the kind's**: a negative count is a caller contract violation
   — `[conf.trap.map]`'s `assert` — not an out-of-range access. And the
   agreement is fragile: `wolf_rt::__wolf_rt_str_repeat` clamps with
   `count.max(0)` and would have returned `""`, so the native lane traps
   above the runtime rather than in it. Three lanes agreeing by
   construction is the state that drifts.

std does NOT guard this one, deliberately: the trap is the right answer for
a contract violation the caller could have checked (§2), and a guard would
hide the one place the implementations agree.

## F-0057 — a byte view with no way back

`s.bytes()` executes on all three lanes as of #40, so `std.bytes` finally
has a producer and eight of its nine reviewed functions are code. The
ninth is the important one and it has no spelling at all:

```text
to_str(b: Bytes) -> str ! {utf8}
```

Nothing in the language builds a `str` from a number. There is no `char`,
no `from_utf8` builtin, no `strbuf.push_byte` — so the VALIDATION half is
writable in wolf source (and is written: `bytes.is_utf8`, 31 rows on three
lanes, rejecting stray continuations, truncations, overlong forms,
surrogates and scalars above U+10FFFF) while the MATERIALIZATION half has
nowhere to happen.

An ASCII-only `to_str` would be a border post that refuses text, which §9
forbids std from shipping, so the function stays a contract in the module
header with this finding beside it.

What would unblock it, cheapest first: `str.from_utf8(b) -> str ! {utf8}`
as a builtin (the validity check belongs where the invariant lives);
`strbuf.push_byte` plus a validating `finish`; or the `char` type F-0018
already asks for, with a scalar-to-`str` constructor.

Two notes for whoever takes it. The decoder in `bytes.is_utf8` is written
in divisions and range comparisons rather than masks because `&`, `|`, `^`
and `>>` on a plain `int` are still `unsupported` on the checked lane
(F-0026) — a `to_str` in library code would pay the same tax, which is one
more argument for the builtin. And the io tier will want this the moment
`fs_read_bytes` lands (F-0044): a byte read with no way back to text is a
byte read nobody can use.

**sc10 addendum — this finding blocks a second function, and nobody
expected it to.** sc09 re-owned `std.json.unescape` to a json sprint on the
grounds that F-0018 had retired and a scanner over arbitrary text was
writable. It is; the scanner is not the problem. `unescape` has to DECODE
`\uXXXX`, which means building `"é"` from the number 233 — the same
materialization half F-0057 is about, arrived at from text rather than from
bytes. An ASCII-only `unescape` would be a border post that refuses text
(§9 forbids it), so the contract stays, and its tag moves from the interim
`boundary` to `parse` because the boundary condition really is gone and
what is left is ordinary bad data.

Two consequences worth carrying. First, the ask list above gains a
motivation: `str.from_utf8` or a scalar-to-`str` constructor unblocks JSON
unescaping, not just byte IO. Second, the general lesson, which is now in
the guide: a finding that blocks "bytes to text" blocks every ESCAPE
FORMAT too — json, url-encoding, `\x` escapes, HTML entities — because
every one of them decodes a number into a character. That is a family, not
a function, and it is worth knowing before the next census predicts one of
them writable.

## Retirements and movements at the sc10 pins

The pin bump is wolf `8321aba` → trunk **`e94b879`** ("merge s69: the idiom
arbiter — c16-warnings closes"), three merged waves in one step (s40, s70,
s69), with lupin **HELD at 0.1.6**. Both ritual gates were run in a clean
scratch clone at the sha and both are green on their FIRST attempt — the
second clean pair in a row, which is why F-0054 stays open rather than
retiring: two clean pairs are not proof a timing dependence is gone, and the
posture this repo keeps is to state the attempt count.

v0.1.7 of lupin exists and was deliberately not chased. The consequence is
the widest two-upstream drift this repo has recorded — the interpreter's
conformance pin (`13b811f`) is three waves behind the compiler's — and it is
the whole explanation for sc10's lupin column: lupin has no `time_*`, no
`env_*` and no `json_*` builtins because they did not exist at its pin. That
is a DRIFT, not a design refusal like `fs`/`net`, and it closes on a release
rather than on a decision. The ledger says so where it would otherwise read
like a capability posture.

### F-0052 is CLOSED, and it is the biggest thing in this bump

For two sprints the guide called `v else |e| match e { … }` the most
dangerous shape in the language: it compiled, it ran, and on wolfc's checked
lane it matched its FIRST ARM whatever the tag was, silently, against two
lanes that got it right. s70's match tier fixed it upstream (wolf-lang#48 —
"handler matches resolve bare idents against the scrutinee's tags before
binding"), and it is re-measured here on all three lanes with the arms
written in BOTH orders, which is the experiment that found the bug in sc08
run in reverse.

It is held as `tests/errors/handler_discriminates.lu` rather than only
believed, per sc09's rule that a claim about what an implementation answers
is a test or it is a rumour. Three lanes, `run`.

**What it unblocks and what sc10 deliberately did not do with it.** Two
functions were written and withdrawn on this finding — `std.io.input_all`
(sc07) and `std.net.read_all` (sc08) — and both are now writable: a loop
over a rowed read can stop on `eof`/`closed` and re-raise the others, which
is the only thing that ever blocked them. Neither is in this sprint's
contract and neither is written; both module headers still say "blocked",
and the sprint that reconsiders them owns the change. `std.x.json` uses the
shape nowhere either: its one place for it (`float_at`'s two-tag delegate)
turned out to have a simpler answer, because the kernel's guarantee makes
one of the two tags unreachable and a wildcard says exactly that without
claiming to tell tags apart.

### Re-verified UNMOVED, each re-measured rather than assumed

- **F-0037** — an enum returned through an error row still takes the miss
  path on every call (`fn id(v: V) -> V ! {none} { v }`, one line, no
  diagnostic). This is now the sole blocker on `std.json.parse`: sc09
  retired its F-0018 half and re-owned the census row to a json sprint, and
  the sprint that took the row found the OTHER finding still standing. That
  is why the DOM half of json is still write-only and why the query half
  lives in the nursery over a different tier entirely.
- **F-0046 / F-0053** — `conform-run` still rejects `--deny-warnings`
  (`unknown flag`, re-measured at this sha), and the record's `warnings`
  array still covers the entry file only. The rig denies warnings itself, on
  every lane that reports one, and it stayed green across 175 tests and 317
  doc-example blocks at this pin. s69 landed eleven NEW lints
  (W0310–W0316, W0602–W0604, W1002, W1003), several policing exactly what
  API-CONVENTIONS §1 already requires — no `get_` prefix, `is_` answers
  `bool`, `as_` borrows rather than consumes, bare `get` carries a row, a
  `pub` item is documented. Nothing in `std/` trips one that this rig can
  see, and F-0053 is the reason that is not the same sentence as "nothing in
  `std/` trips one".
- **F-0034** — module identity is still the last path segment, and it now
  costs a module PAIR rather than a name: see F-0058.
- **F-0026** — the checked tier's f64 ceiling is unmoved in the one body
  that matters here, and it cost a function this sprint: see F-0061.
- **F-0016** — `wolf fmt` still splits a dotted call under a `//` comment.
  Fifth sprint running; it bit three of sc10's test files within a minute of
  their existing, and all three now carry the note in the file header, which
  is the documented dodge and is getting old.
- **F-0043's `E0806` half**, F-0004, F-0011, F-0012, F-0014, F-0025's last
  third, F-0027, F-0029, F-0030, F-0035 (the byte-type half), F-0038,
  F-0039, F-0040, F-0044, F-0045, F-0047, F-0049, F-0050, F-0051, F-0054,
  F-0055, F-0056, F-0057: all retested, all open.

### The ledger movement

Fifteen new rows and no existing row moved — the first pin bump in this
repo's history where a three-wave jump advanced nothing already recorded.
That is not a disappointment, it is what the wave was: s40 added builtin
families std had never wrapped, s70 fixed a shape std had refused to write,
and s69 added lints std already obeyed. Nothing in the existing 160 tests
touches any of the three.

## F-0058 — the nursery's first tenant cannot be imported beside its facade

D31's graduation mechanism is a MOVE: `std.x.foo` becomes `std.foo`, and
because the path is the API, the move is the whole release note. F-0034 says
module identity is the LAST path segment. Put those together and a resident
and its facade successor are the same name:

```
use std.json
use std.x.json          // fail(E0306) on both compiler rungs
                        // unsupported under lupin (silently binds one)
```

Measured at these pins with two one-function probe modules, so the result is
about the paths and not about either module's contents. Importing EITHER on
its own resolves and runs, which is what lets `std.x.json` keep the name
upstream's prelude comment gives it.

**Why it matters beyond a naming annoyance.** The nursery exists so that a
module can be USED while it is still moving (D31: the residents are not
experiments, they are complete bodies kept out of the facade for a measured
reason). A program that wants both halves of json today — the DOM to build a
document, the query kernel to read one — cannot have them, and neither
module can grow toward the other by depending on it. `std.json` and
`std.x.json` therefore divide the work strictly by direction, each says so
in its header, and every test and doc example in the repository imports
exactly one.

**The second half of this filing is the query tier's own hole**, and it is
the one that makes `std.x.json` a query surface rather than a reader: there
is no key ENUMERATION. `json_len` counts an object's members and nothing
names them, so an object can be counted and not walked. `keys(doc, path)` is
a reviewed contract in the module header, blocked rather than unwritten, and
until it lands a program can only ask about keys it already knew. (An array
CAN be walked — `json_len` plus `child_index` plus `get` — and `items` is
left out for a different reason, stated in the header: it would re-parse the
document once per element.)

Asks, in order: rule how a resident and its facade successor coexist during
a graduation campaign (a `use … as` rename would do it, and so would module
identity being the full path); and grow the query tier a key enumerator.

## F-0059 — the clock ABI is milliseconds, and the deadline hole is everywhere

`time_now_ms`, `time_unix_ms` and `time_sleep_ms` are the whole time tier.
They are enough for `std.time`'s twenty-four functions and the shape of what
is missing is three themes.

- **Milliseconds are the floor of the resolution.** A `_ns` face over a
  `_ms` source would report a thousand-fold lie, so `std.time` ships no
  nanosecond anything. Benchmarking, latency histograms and anything that
  wants to see a fast function are out of reach — and D36's bench format is
  a stdc02+ item that will want exactly this.
- **Nothing arms a deadline, anywhere in the toolchain.** F-0049 filed this
  for sockets, where `wolf_rt` has a per-socket deadline no builtin
  exposes; s40 makes it general. `std.time` cannot ship a `Deadline` type or
  a `with_timeout` combinator, because there is nothing to arm and no
  `select` to race a timer against work (X6's composition story is s35's and
  has not reached std). Every operation in std that could block — `accept`,
  `read`, `connect`, `sleep` — blocks unboundedly.
- **A monotonic reading has no identity.** `time_now_ms` counts from an
  arbitrary process-local anchor, which is correct; nothing marks WHICH
  anchor, so two `Instant`s from different processes subtract to a number
  that means nothing and no code can detect it. `std.time` documents the
  hazard, which is all a library can do — the fix is a clock id in the ABI,
  or a `SystemTime`/`Instant` distinction the tier itself understands.

One more, and it is the one that makes timing tests unprincipled here: the
s36 clock-hook seam does not reach clock READS yet, so `--schedules` and
`--replay` cannot virtualize time. A test over a sleep is therefore a
predicate over a real host clock (`elapsed >= 3`) with no deterministic
mode, which is why `tests/time/monotonic.lu` asserts only inequalities and
why every exact assertion in the module is against the PURE renderer
instead.

## F-0060 — a pure builtin family is refused at comptime with no diagnostic

The json four are the toolchain's one pure builtin family: no I13
capability, no sandbox category, nothing reached, so a package that uses
only them declares no capability at all. That is the right design and this
finding does not argue with it.

What it argues with is the refusal. The comptime engine still refuses them —
correctly, there is no json evaluator in the D33 allowlist at v0 — and the
refusal is `unsupported` at resolve, with no code, no reason string and
nothing in the record's `diagnostics` array:

```
comptime fn probe(doc: str) -> bool { json_valid(doc) }
// checked: unsupported (phase_reached=resolve), diagnostics: []
```

Every capability family answers `E0701` with a sentence naming what the call
reaches and why comptime cannot: "`time_now_ms` reaches the clock, which
comptime code can never touch", "`env_get` reaches environment variables …".
A package author who hits the json refusal learns nothing at all, and cannot
tell "no evaluator yet" from "you wrote something wrong".

The cost here is concrete and small: this repository holds the D33 refusal
as a test for `net`, `time` and `env` (`fail(E0701)`, one file each, the
kind named in the directive), and it cannot hold one for json, because
`unsupported` is a ledger row rather than a directive.

The ask: a diagnostic for "this builtin has no comptime evaluator at v0",
distinct from the capability refusal — because the two are different
sentences and one of them is temporary.

## F-0061 — `parse_float` is unsupported on both compiler rungs, and it cost a function

`std.fmt.decimal.parse_float` answers `unsupported — arithmetic outside
integers` at the mem tier on the checked lane and on the native rung
(F-0026's f64 ceiling, one body deep; sc05 recorded the ledger row and it is
unmoved at this pin). For five sprints that was one `unsupported` row among
several and nothing depended on it.

sc10 is where it stopped being free. `std.x.json.float_at` — the number
reader every JSON caller reaches for second — was written, tested and
withdrawn inside the sprint, and the arithmetic is simple: the checked tier
is `std.x.json`'s ONLY executing lane (the native rung refuses the json
builtins by name, lupin is behind the pin), and `parse_float` is refused on
exactly that lane. So the function would have had zero lanes: no test could
run it, no doc example could be fenced, and §14's doc-truth floor — at least
one lane reaches `exit(0)` — would have been missed on every example it had.

It is a reviewed contract in the module header with this filing beside it,
and its body is four lines waiting for either compiler rung to execute
`parse_float`. Nothing about the signature changes when it lands.

The general shape, worth stating because it will recur: a refusal that costs
one lane is a ledger row, and a refusal that costs a module's ONLY lane is a
withdrawn function. The nursery is full of modules with one lane by
construction, so every dependency a resident takes should be checked against
that lane before it is written, not after.

## F-0062 — two keywords ate two std names, and one of them reported it badly

wolf has 50 reserved keywords, no raw identifiers, and two of them are words
the process tier wants: **`spawn`** (it opens a task, `s.spawn(fn() { … })`,
and a supervised proc, `spawn proc`) and **`handle`** (the pool tier's
two-phase reserve/init value).

Both refusals are correct and neither is the finding. `pub fn spawn` is a
clean `E0008` with a fix-it — "`spawn_` is the usual dodge, or a more
specific word" — exactly as `copy`/`copy_file` was in sc07, and
`std.process.start` is the better name anyway: a child process and a task are
different things with different failure models, and borrowing the keyword's
word for the other one would have invited the confusion the type system then
has to unpick. Recorded so the next author checks the list first.

**The finding is the OTHER diagnostic.** A keyword used as a STRUCT FIELD
name splits the implementations, and the compiler's half is much the worse:

```
pub struct Child { handle: int }
// lupin 0.1.8:  E0008 — `handle` is a reserved keyword           (at parse)
// wolfc 0.1.0:  E0201 — expected a field initializer             (at parse)
```

`E0201: expected a field initializer` points at `handle` and says nothing
about keywords, so the reader's first hypothesis is a typo in the struct
literal three lines below — which is where the second error appears. lupin
names the cause in five words; wolfc names a symptom. The same asymmetry
appears at the USE site: `var forged = process.Child { handle: 4242 }` is
lupin's keyword error and wolfc's "expected a field initializer".

The ask: the keyword check that produces `E0008` for an item name should
produce it for a field name too — one code, one sentence, both positions.
The generalization for wolf-std: check every new field name against the
keyword list, not just every new function name.

## F-0063 — trunk's tip failed the pin ritual's first gate, and the fix is a `read_dir`

`17ea078` (wolf-lang trunk HEAD at the sc11 bump) fails
`cargo test --workspace` with exit 101:

```
---- fuzz_regressions stdout ----
panicked at crates/wolf_fmt/tests/properties.rs:187:
read regression: Os { code: 21, kind: IsADirectory, message: "Is a directory" }
```

The commit adds one file — `tests/regressions/unfixed/idem_class_six.lu.pending`,
an unfixed fmt fuzz class banked deliberately — and its message says "the
.pending suffix keeps it out of the regression sweep". The suffix does; the
enclosing DIRECTORY does not. `fuzz_regressions` walks the corpus with
`read_dir` and calls `fs::read` on every entry, so the new subdirectory is
read as a file and the sweep panics before it checks anything.

Consequences worth naming: the sweep's own assertion ("the regression corpus
never shrinks") cannot fail because the loop cannot finish, and every
regression in the corpus goes unchecked on that run.

The ask: filter to files, or to the `.lu` extension, before reading — and
consider asserting on the count of files SKIPPED, since the corpus now has a
deliberate skip category.

This repository's response is the pin ritual working as designed: the sha is
held one commit back at `0b4e79c`, which is green on both gates on the first
attempt and contains every wave sc11 needed. Recorded here because a pin that
was chosen for a reason should say what the reason was, and because "trunk
HEAD is green" is exactly the kind of assumption F-0024 taught this repo to
stop making.

## F-0064 — a pairing line is not a pin, and it broke doctor

`wolf 0.1.0` (the r01 identity release) answers `--version` on two lines:

```
wolf 0.1.0 (wolfgang)
paired with lupin 0.1.8 (reference interpreter), pin 7886559
```

This repository's `cargo xtask doctor` verifies a binary's self-reported pin
against `vendor/tools.toml`, and its parser read the whole output as one
line: name `wolf`, version `0.1.0`, and — from the second line — pin
`7886559`. That sha is lupin's own commit in the **wolf-interp** repository,
so doctor compared two repositories' histories and failed the bump with
"`--version` names pin 7886559, recorded pin is 0b4e79c".

Not an upstream bug: the pairing is exactly the right thing for a release
binary to report, and the rig was reading it wrong. Fixed here — identity
comes from the first line, the remainder is captured as `pairing` and PRINTED
by doctor without gating — with the two-line shape held as a unit test so the
next release cannot break it silently.

Recorded as a finding anyway, for two reasons. It is the first time a
tool's `--version` has grown a line, so any other consumer of that output has
the same latent bug; and the pairing is genuinely useful information this
repo now surfaces (`doctor: pairing: paired with lupin 0.1.8 …`), which is
the answer to "are these two binaries meant to be used together" that
`vendor/tools.toml`'s drift note has been answering in prose for ten sprints.

## F-0065 — the process trio is four operations short of a Command facade

`os_spawn`/`os_wait`/`os_kill` (s40, checked lane only) are enough to write
`std.process` and its five verbs, and this finding is the list of what a
caller then cannot do. Every item is a reviewed contract in the module
header; none is a workaround std can write.

1. **No stdio.** `os_spawn` wires all three of the child's streams to the
   host's null device, so `output(c) -> str` — the single most-reached-for
   operation in any process API — has nothing to read, and `stdin_text` has
   nowhere to write. The book's pargrep chapter will feel this first. The ask
   is a piped-stdio spawn plus a read on the pipe; the s38 io traits are the
   natural shape, and `wolf_rt` already has the reactor (s35) that would
   serve it.
2. **No child environment or working directory.** `os_spawn(argv)` takes one
   argument and the child inherits both from the parent. A caller who must
   change them can only change its OWN with `env_set` first, which is a
   different operation with a different meaning (it is not scoped to the
   child, and it races every task in this process).
3. **No non-blocking wait and no deadline.** `os_wait` blocks, so
   `try_wait` — "has it finished?" — has no honest implementation, and
   `wait_timeout` is the same missing deadline as the net tier's (F-0049).
   With no way to poll, the only way to stop a runaway child is a kill from
   another task.
4. **No real process id.** The `int` `os_spawn` hands back is the machine's
   own child-table index, so a `pid` accessor would name something this tier
   does not have. A program that must be reported to an operator, or written
   into a pid file, cannot be.

One thing the trio gets exactly right, recorded so it is not lost in a list
of gaps: **argv-array only, with no shell-string spawn anywhere.** That is a
design decision std would have had to fight if it had gone the other way,
and it makes command injection structurally impossible rather than a
quoting-discipline problem. `std.process` has no `shell` function and its
header says it never will.

## F-0066 — the happy path of a spawn is unwitnessable from a portable test

`std.process` ships six functions and this repository cannot write a single
test that starts a real program, because three things are missing at once:

- **No program exists on every tier-1 host.** `/bin/sh` is not on windows,
  `cmd.exe` is not on linux or macOS. The toolchain's own process tests are
  `#[cfg(unix)]`-gated and use `/bin/sh`, which is the right answer for a
  Rust test suite and unavailable to a `.lu` file.
- **A wolf program cannot learn its own path.** `env_args` drops argv[0] by
  design ("the program name is not the program's input"), and there is no
  `os_exe`/`current_exe`. So the one program guaranteed to exist and be
  executable — the test itself — cannot be named.
- **The directive schema has no per-platform gate.** A test is `run` or it is
  not; there is no `platform: unix` to make a unix-only witness legal, and
  the ledger's three columns are implementations rather than hosts.

The rows are all witnessable and they are what `tests/process/` holds: an
empty argv and a name no host has are both `not_found`, a forged handle is
`io`, and none of them starts anything. What cannot be held here is the
central claim of the module — that a child's exit code comes back, and that
a killed child answers `signal` — so those two sentences rest on the
toolchain's unix-gated tests, which is a weaker place for them than this
repo's usual standard (§13: a claim about what an implementation answers is a
test or it is a rumour).

The ask, cheapest first: (1) an `os_exe`-style builtin, which would let a
test spawn ITSELF with an argument that makes it exit with a chosen code —
this single addition closes the whole finding, portably, with no fixture;
(2) failing that, a `platform:` directive key so a unix-gated witness can
live in the corpus honestly; (3) failing both, a documented fixture program
the toolchain guarantees on every tier-1 host.

## F-0067 — `os_cwd` has no home in std, because `chdir` does not exist

The s40 os family is now split across two std modules: the `env_*` four are
`std.env`'s and the process trio plus `os_exit` are `std.process`'s (as
`exit`). One builtin is left over — `os_cwd` — and sc11 declines to place it
rather than placing it badly.

Why it is awkward. It carries the `env` capability, not `exec`, so it is not
`std.process`'s by the capability table. It is a query about this program's
own situation exactly as `args` and `vars` are, so `std.env.cwd()` is the
obvious home. But the operation a caller reaches for next does not exist:
there is no `os_chdir` at the builtin tier, so std would ship a directory
READ with no directory WRITE beside it — and a program that wants to resolve
a relative path against the working directory can do it, while a program that
wants to run in another directory cannot, and neither can it ask a child to
(F-0065's item 2).

The ask: `os_chdir(path) -> () ! {not_found, denied, io}`, at which point
`std.env.cwd`/`chdir` land together as a pair and this finding closes. If the
answer is that a process-wide chdir is deliberately absent — a defensible
position, since it races every task in the process and is the reason
per-child working directories exist — then say so, and `cwd` lands alone with
that reason quoted, which is a decision rather than an oversight.

## F-0068 — `conform-run <file>` with no directory component says the wrong thing

```
$ wolf conform-run --checked main.lu
wolf conform-run: the package root has no wolf source files
$ wolf conform-run --checked ./main.lu
{"verdict":"exit(0)", …}
```

The file exists, is in the working directory, and is the only `.lu` file
there. The package root is evidently derived from the argument's parent
directory, and a bare file name's parent is the empty string rather than
`.`, so the loader looks in nowhere and reports the one thing that is
certainly not true.

No cost to this rig — it passes absolute paths (`stage::Staged::entry`) and
always has — and no cost to `wolf test`, which takes a directory. The cost is
to a person at a prompt, which is where every first impression of a
toolchain is formed, and the message is actively misleading: it describes a
package problem for what is a path-normalization bug.

The ask: normalize the argument (`Path::parent` of a bare name is `Some("")`
— treat it as `.`), and consider naming the root that was searched in the
message, since a diagnostic that says where it looked cannot be this wrong.

## F-0069 — `?` inside a `comptime fn` is `unsupported`, and it MASKS the capability refusal

Measured at the sc11 pin, three programs that differ only in how one row is
handled:

```
comptime fn probe(x: int) -> int ! {io} { if x < 0 { return io }  x }
// checked: exit(0) — a row RETURN and a raise are fine at comptime

comptime fn probe(x: int) -> int { let v = inner(x) else 0 - 1  v }
// checked: exit(0) — a same-module call whose row is HANDLED is fine

comptime fn probe(x: int) -> int ! {io} { let v = inner(x)?  v }
// checked: unsupported (phase_reached=resolve), diagnostics: []
```

So `?` — the language's ordinary propagation, and the first thing anyone
writes — is outside the comptime engine's subset, while the raise and the
`else` are inside it. That alone would be an ordinary NotYet. What makes it a
finding is the interaction with D33.

`tests/process/comptime_refuses.lu` holds the `Exec` refusal, and the natural
spelling of its witness is `os_kill(slot)?` inside a `comptime fn`. Written
that way the file answers **`unsupported`, not `fail(E0701)`** — the engine
refuses the propagation at resolve, so the capability check never runs, and a
rejection test that was supposed to prove "compiling a package can never kill
a process" proves nothing at all while looking healthy in a ledger. The
bare-call version (`os_kill(slot)` with the row discarded) reaches E0701 and
then trips `W0601`, which this rig denies. Only the third spelling —
`os_kill(slot) else |_| { }` — both reaches the refusal and warns about
nothing.

Two asks, in order:

1. **Support `?` in the comptime engine**, or refuse it with a diagnostic that
   names it (`E`-coded, "`?` has no comptime evaluator yet"). A bare
   `unsupported` with an empty `diagnostics` array is the shape F-0060 already
   asked about for the json family: the author cannot tell "not implemented"
   from "you wrote something wrong".
2. **Decide the ORDER between the sandbox check and the subset check.** A
   capability refusal is permanent and a subset gap is temporary, so the
   permanent one should win: `os_kill` in a `comptime fn` should be `E0701`
   whatever surrounds it. Today the temporary answer hides the permanent one,
   which is the wrong way round for anything a security posture rests on.

The general lesson for this repository, recorded in the guide: when a
rejection test's verdict changes because you changed something UNRELATED to
the rejection, the test has stopped witnessing what its header says.

## F-0070 — lupin 0.1.8 has four fifths of the os/env builtin family

At its own conformance pin (`26fa98e`, two waves past the 0.1.6 this repo held
at sc10) the interpreter runs `env_args`, `env_get`, `env_set`, `os_cwd` and
`os_exit`, and every `time_*` call. One builtin of the s40 os/env family is
absent:

```
$ lupin conform-run env_vars.lu
unsupported: `env_vars` does not resolve
```

It is not a declined surface — the other four env calls run, and the refusals
this machine DOES make by design (`fs_*`, `net_*`, `json_*`, the process trio)
all carry a sentence explaining themselves. `env_vars` just is not there, with
the generic "does not resolve" that any unknown name gets, so it reads as an
oversight rather than a posture.

The cost here is one ledger row: `tests/env/args_and_vars.lu` asserts the
`K=V` listing round trip and stays `unsupported` on the interpreter lane while
its four siblings advanced at this bump. Small, and worth filing for the
sentence it produced: this sprint nearly wrote "lupin has the env family now"
into three documents on the strength of `env_get` working, and the correct
record needed five separate one-call probes. **A builtin FAMILY is not a unit
of evidence; a builtin is.**

The ask: implement `env_vars` (sorted `K=V` lines, non-UTF-8 entries skipped —
the semantics `wolf_mem`'s `os_builtin` already pins and `std.env.vars`
documents), or decline it with a reason the way the other refusals do.

## Retirements and movements at the sc12 pins

The pin bump is wolf `0b4e79c` → trunk **`f8dca42`** (ten commits: s74, s53,
s75, s78, s76, s77 and the rt test gating), with lupin **0.1.8 → 0.1.10**,
skipping 0.1.9. Both ritual gates were run in a clean scratch clone at the sha
and both are green on their FIRST attempt — the fourth clean pair in a row, so
F-0054 stays open on the reasoning it has always stayed open on: a clean run is
not proof that a timing dependence is gone, and the posture is to state the
attempt count.

The pin is trunk's tip, which is a change from sc11 (where the tip was red,
F-0063). Three compiler sprints — s79 bench, s80 token audit, s81 str
equality — were in flight during this sprint and none of them is in this pin,
deliberately: the pin is taken once and held.

### F-0037 is CLOSED, and it is the biggest thing in the interpreter's half

For five sprints a function whose return type was an ENUM and whose signature
carried an error row took the MISS path on every call — `fn id(v: W) -> W !
{none} { v }` raised instead of returning `v`, in one line, with no
diagnostic. wolf-interp#16 fixed it at 0.1.10 ("an enum variant is a value,
not a raise": `ErrorValue` records where its name resolved, and `is_error` —
the only question `?` and `else` ask — reads it). Re-measured here with the
finding's own reproducer, on the lane that matters:

```
$ lupin ./main.lu
value path wins
```

**What it unblocks, and what this sprint deliberately did not do with it.**
`std.json.parse`, `json.get` and `json.at` were written, tested and withdrawn
to reviewed contracts on this finding — `parse`'s signature is `-> Value !
{syntax, deep}`, an enum through a row — and the interpreter is `std.json`'s
only executing lane, so the module's DOM half becomes writable for the first
time. None of it is in this sprint's contract and none of it is written; the
module header still says blocked, and the sprint that owns the json row owns
the change. sc10's rule applies to itself here: "the blocker retired" is not
"writable" until every finding on the SIGNATURE has been re-measured, and the
sprint that takes it should re-measure F-0039 (nested rows) and F-0029
(cross-module enum consumption) before writing a line.

API-CONVENTIONS §11's rule — "No std accessor returns an enum through an error
row" — was written as an interim with this finding as its exit, and the exit
has arrived.

### F-0032 is closed too

`s as nonsense` was a silent no-op under lupin (the value passed through
unchanged, no diagnostic). At 0.1.10 it is `E0301` at resolve, spanning the
type name, matching the counterparty span for span (wolf-interp#17). Nothing
in `std/` depended on the bug, so the closure costs no row and is recorded
because a silent wrong answer that closes deserves the same paragraph one that
opens gets.

### Re-verified UNMOVED, each re-measured rather than assumed

- **F-0057** — the four probes sc11 used, run again at this pin:
  `str.from_utf8(b)` "does not resolve", `(mut b).push_byte(104)` is
  "`StrBuf` has no method `push_byte` in this machine's std subset", `'h'` is
  `E0101` at the LEXER, `bytes_to_str` "does not resolve". `std.bytes.to_str`
  is a reviewed contract for the fourth sprint running. What is NEW is
  agreement about the shape of the fix: s77's lowering says the byte view
  "cannot become a `str` … a `List[int] -> str` conversion would have to
  VALIDATE (wolf-std's `bytes.to_str`, still blocked, and this is why: it
  wants a checked primitive, not a cast)". The view is now bit-identical to a
  `str`, which makes an unchecked conversion look one instruction away, and
  the compiler names that as the forging hole rather than taking it.
- **F-0070** — `env_vars` still "does not resolve" under lupin 0.1.10, so
  `tests/env/args_and_vars.lu` keeps its dark interpreter column while its
  four siblings run. Two releases later, unmoved.
- **F-0046 / F-0053** — `conform-run` still rejects `--deny-warnings`
  (`unknown flag`, re-measured at this sha) and the record's `warnings` array
  still covers the entry file only. The rig denies warnings itself and stayed
  green across 185 tests and 317 doc-example blocks.
- **F-0016** — `wolf fmt` still splits a dotted call under a `//` comment.
  Sixth sprint running; this sprint's rewrites carry their notes in `///` docs
  instead, which is the documented dodge and is now simply how this repository
  writes.
- **F-0026's checked-tier ceiling** is what F-0071 is a new face of: the tier
  models a subset of the machine, and the subset is where std has to live.
- F-0004, F-0011, F-0012, F-0025's last third, F-0027, F-0029, F-0030,
  F-0035 (the byte-type half), F-0038, F-0039, F-0040, F-0044, F-0045,
  F-0047, F-0049, F-0050, F-0051, F-0054, F-0058, F-0061, F-0065, F-0066,
  F-0067, F-0068, F-0069: all retested, all open.

### The ledger movement

**Zero rows moved and two rows were added.** The 183 tests this repo carried
into the bump answer exactly what they answered at the sc11 pin — 144 / 149 /
97 `run` on lupin / checked / native, with 39 / 30 / 82 `unsupported` and
0 / 4 / 4 held rejections — and the two new sc12 rows are the byte-view pair
(`str/byte_view_walk.lu` three lanes, `str/byte_view_index.lu` two).

That is the honest report and it is the second time this repo has recorded it
(sc10 was the first). The reason is structural rather than lucky: s77 changed
the LOWERING of `bytes()` and not its surface, s76 and s75 changed where and
how containers allocate and not what they answer, and the seven std bodies
rewritten this sprint were rewritten to keep the lanes they had. A wave that
makes existing code faster moves no ledger row by construction — the ledger
measures depth, not cost — which is worth stating plainly, because a sprint
whose whole subject is a performance primitive and whose ledger is unchanged
looks like a sprint that did nothing.

## F-0071 — the checked tier models two of the byte view's seven positions

**CLOSED at the sc12 (02-os) pin** (s88, wolf-lang#85 — "a temporary can be
read from"). Re-measured one shape at a time on the checked tier at
`02c1e88`: `"wolf".bytes()[1]` is 111, `.get(1)` is 111, `.first()` is 119,
`.last()` is 102, `.count()` is 4 — five verdicts where every one of them
used to be `unsupported` at `mem`. `tests/str/byte_view_index.lu` advanced
`unsupported` -> `run` on that lane and now asserts indexing, `get` and
`count` together.

Two things worth recording beside the closure. **std did not undo the
rewrites**: `str.code_points`' one-pass state machine is shorter than the
random-access walk it replaced, and a constraint that forced a better
algorithm is not one to reverse the day it lifts — what the closure buys is
that a FUTURE body may index a view without paying a lane. And **half of
the finding survives on the other machine**: lupin has no `first` and no
`last` on a `List` at all ("`List` has no method `last` in this machine's
std subset"), which is a gap in its container surface rather than anything
about views, and it is held as its own ledger row
(`tests/str/byte_view_first_last.lu`) rather than left inside a closed
finding where nothing would measure it.

The filing as it stood:

s77 (#80) makes `s.bytes()` the receiver's own `{ptr, len}` pair and reads it
in place wherever the call is CONSUMED. The lowering names seven such
positions: iteration, indexing, and the `len`/`count`/`is_empty`/`get`/`first`/
`last` queries. `wolf conform-run --checked` models two.

```text
s.bytes()[0]        unsupported — indexing outside the modelled surface  @mem
s.bytes().get(i)    unsupported — List method on a temporary             @mem
s.bytes().first()   unsupported — List method on a temporary             @mem
s.bytes().count()   unsupported — List method on a temporary             @mem
for b in s.bytes()  run, all three lanes
s.bytes().len       run, all three lanes
let bs = s.bytes(); bs[i]   run, all three lanes (the materialized shape)
```

The last line is what makes this a gap rather than a design: the checked tier
indexes a byte LIST perfectly well. What it does not model is the temporary.

**The cost, measured in bodies rather than in nanoseconds.** wolf-std cannot
spend an execution lane on a performance shape — that is §14's honesty applied
to an optimization instead of to a capability — so this sprint's seven
rewrites are all on the two-position subset:

- `str.char_count`, `str.is_ascii` — `for` over the view, one of them
  returning out of the loop;
- `str.char_offsets` — `for` plus a counter, where the natural body indexes;
- `str.code_points` — the UTF-8 decoder rewritten from a random-access walk
  (`bs[i + 1]`, `bs[i + 2]`, `bs[i + 3]`) into a one-pass state machine with a
  pending-continuation count, because lookahead is exactly what a view cannot
  do on this lane;
- `fmt.digit_of`, `hex.digit_of`, `base64.value_of` — each takes its first
  byte by iterating and returning out of the first iteration.

Every one of those is a fine body. The rule they add up to is not fine: an
algorithm that needs genuine random access over bytes must keep materializing
or drop a lane, and the next one might not have a one-pass form.

Two asks, either of which closes it: model the view's indexing and query
family in the checked tier, or — if the temporary is deliberate there — say so
in the refusal. `unsupported — List method on a temporary` is a place-model
sentence that never mentions views, and learning that `for` was the shape to
keep took a four-program bisect.

Held as `tests/str/byte_view_index.lu` (`lupin=run`, `wolfc=unsupported`,
`native=run`) so the day the tier models it, the row advances and says so.

## F-0072 — a byte view cannot cross a function boundary

**CLOSED at the sc12 (02-os) pin** (s89, wolf-lang#86 — "a byte view that
can cross a call"). The answer is the one the filing asked for, in the shape
it asked for it: not a new type, but the region checker's argument one scale
down. A `List[int]` parameter whose every use is one of s77's read positions
(or a re-lend into another such parameter) is **Lendable**, and the caller
hands over the string's own `{ptr, len}`; anything unproven is **Opaque** and
materializes exactly as before, which is never wrong and only slower; a use
that provably outlives the call is **E1015** with the one-word fix in the
diagnostic (bind first — a `let` materializes). Eight of `std.bytes`' nine
functions are lendable; `to_str` is not, because it hands its parameter to
`str_from_utf8` and a builtin consumer materializes.

What this repository can and cannot say about it: the no-copy half is an IR
property, pinned upstream in `wolf_wir`'s `lower_shapes` suite and
`wolf_mem`'s `byteview` tests, and no lane here observes an allocation. So
`tests/bytes/lend_across_calls.lu` asserts the half that IS observable — the
same nine calls through a view and through an owned list must agree, on all
three lanes — and its header cites the other half rather than restating it
(§9's sc12 rule). The `Bytes` type this repository has wanted since sc05 is
still open on its own merits; the lend is what stopped the interim currency
costing a copy per call.

The filing as it stood:

s77's own comment states the boundary: every position that is not consuming —
a `let` binding, an argument, a return — materializes through
`__wolf_rt_str_bytes`. That is the right conservative default, and this
finding is about what it costs a library, so the cost is on the record when
the `Bytes` question is decided.

`std.bytes` is nine functions whose first parameter is `List[int]`, so every
one of them receives its bytes as an ARGUMENT and every call from a string
copies the string first:

```wolf
bytes.is_utf8(bytes.from_str(s))   // copies s, then validates
str.char_count(s)                  // walks s in place (the sc12 rewrite)
```

Same kind of work; one of them is free. The difference is the parameter, not
the implementation, and std cannot fix it from its side: there is no signature
that accepts a view, and inlining the byte tier into `std.str` to get the win
would duplicate the module and destroy the split `std.bytes` exists for
(bytes a program HOLDS, versus bytes it walks through once).

The ask is a design one: the `Bytes` type this repo has documented as an
interim since sc05, or a parameter mode that lets a callee borrow `{ptr, len}`
without materializing — which a `str` parameter already is, so the machinery
exists and only the name is missing — plus a spec rule about which positions
materialize, since today that is discoverable only from a comment in
`wolf_wir::lower`.

The module header now says this in the place a reader meets it, because the
alternative is a library that looks slower than it is for a reason nobody can
see from the signature.

## F-0073 — the pairing line is a constant, and it rots

```
$ wolf --version
wolf 0.1.0 (wolfgang)
paired with lupin 0.1.8 (reference interpreter), pin 7886559
```

Measured at trunk `f8dca42` (2026-08-13), one day after lupin 0.1.10 shipped
(2026-08-12) — and 0.1.10's own conformance pin is `613c3dc`, a wolf-lang
commit ten commits behind this sha. So the two binaries this repo pins really
are meant to be used together, the drift really is the narrowest it has ever
been, and the line still names the release before last.

r01 introduced the line and F-0064 taught this repo's doctor to read it:
identity from the first line, the pairing reported and never gated. It is
useful — it answers "are these two meant to be used together?", which
`vendor/tools.toml` had been answering in prose for ten sprints. That is
exactly why it needs a mechanism: a claim that comes out of a binary is
trusted more than a note, and this one has nothing keeping it true.

The ask: make the pairing a value the release process writes, or state it with
the DATE of the differential run that established it, so that a stale line
reads as history rather than as a claim about now.

## F-0074 — the reference lane builds every list quadratically

Found by accident, which is the only reason it is in this sprint at all:
`tests/fmt/decimal/shortest_round_trip.lu` timed out at the rig's 60-second
per-test ceiling during a `cargo xtask ci` run, having passed three
`std-test` runs the same hour. Chasing the timeout produced a much larger
answer than the flake it started as.

The canonical list-building loop — the one every `std` function returning a
`List` writes, because pushing into a fresh list is the portable spelling
(sc04's rule: index assignment executes on one lane only) — is quadratic
under lupin:

| N pushes | lupin 0.1.10 | wolf `--checked` | wolf `--native` |
|---|---|---|---|
| 4 000 | 0.46 s | — | — |
| 8 000 | 1.91 s | 0.08 s whole process | 0.22 s whole process |
| 16 000 | 7.83 s | — | — |
| 32 000 | 37.53 s | 0.14 s whole process | 0.13 s whole process |

Doubling N quadruples the time (4.1x, 4.1x, 4.8x), which is a copy per push
rather than an amortized doubling. Both compiler rungs are flat at the same
sizes, so this is the interpreter's list representation and not a cost the
language imposes.

**Two suspects cleared before filing**, both measured rather than reasoned
about, and both worth recording because a std scanner does them constantly:

- suffix slicing (`rest = rest[1..]` until empty) is LINEAR: 2 000 / 4 000 /
  8 000 / 16 000 bytes → 0.008 / 0.013 / 0.025 / 0.055 s;
- `starts_with` is O(1) in the receiver: 200 000 probes cost 0.73 s against a
  16-byte string and 0.80 s against an 8 192-byte one.

So a scanner that walks a string is fine here; a scanner that BUILDS
something is not.

**What it costs this repository.** `hex.decode`, `base64.decode`,
`str.split`/`find_all`/`char_offsets`/`code_points`, `list.map`/`filter`/
`concat`, `sort`'s merge passes and `fmt.decimal`'s big-integer limb
arithmetic are all quadratic in their output size on the reference machine.
The visible symptom is the slowest test sitting at 28-35 s against a 60 s
ceiling — which is why it fell over once under load — and the invisible one
is that this repository has been describing those functions' costs in terms
of the algorithm rather than the machine.

Also measured while isolating it: lupin 0.1.10 is consistently ~15% slower
than 0.1.8 on that test (28.3 / 35.3 s versus 24.0 / 31.0 s, alternating
runs, one staged std tree). Reported with the filing, not separately: a
release that adds analyses may legitimately cost time, and 15% is not the
story next to a quadratic.

The ask is amortized growth, or — if the representation is deliberately
persistent — a sentence saying so, because the shape wolf-std has
standardized on is that representation's worst case.

**What this sprint did NOT do about it.** Nothing in `std/` changed for it.
The portable spelling is still the portable spelling, the alternative
(index assignment) still costs a lane, and rewriting the library around one
implementation's data structure is precisely the workaround
`CONTRIBUTING.md` forbids. It is filed, it is measured, and the two numbers
that matter — the ceiling and the doubling — are here so the next sprint
that sees a timeout knows within a minute whether it has found a new
problem or this one.

## Retirements and movements at the sc13 pins

The pin bump is wolf `f8dca42` → trunk **`4e316ad`** ("snapshots: s81's two
str shapes carry s80's role immediate"), three merged waves in one step
(s79 bench, s80 token audit, s81 str equality) — the three sc12 named as in
flight and deliberately did not chase. lupin goes **0.1.10 → 0.1.11**.

Both ritual gates were run in a clean scratch clone at the sha and both are
green on their FIRST attempt (`cargo test --workspace` = 0,
`cargo run -p xtask -- ci` = 0, exit codes printed rather than read off a
summary line — F-0063's lesson). Fifth clean pair in a row, so F-0054 stays
open on the same reasoning it has stayed open on since sc09: five clean
pairs are not proof a timing dependence is gone, and the posture is to state
the attempt count.

### F-0057 is CLOSED, and the border post is open

For four sprints `std.bytes.to_str` was a reviewed contract because nothing
in the language built a `str` from a number: no `char`, no `from_utf8`, no
`strbuf.push_byte`, probed four ways at four consecutive pins. s81
(wolf-lang#58) landed `str_from_utf8(b: List[int]) -> str ! {utf8}` in the
prelude, and it is the function the contract described rather than the cast
the representation invited:

```text
lone continuation (80)          {utf8}
truncated (E2 82)               {utf8}
overlong (C0 AF, E0 80 80)      {utf8}
surrogate (ED A0 80, ED BF BF)  {utf8}
past U+10FFFF (F4 90 80 80)     {utf8}
never-bytes (C0, C1, F5..FF)    {utf8}
not a byte (300, -1)            {utf8}
interior NUL (77 00 66)         accepted — a str carries its length
"wolf é".bytes() round trip     accepted — the view and the source agree
```

`std.bytes.to_str` is four lines over it, its row is the primitive's own
(§14's verbatim-adoption rule, applied to a pure tier), and the sc05
contract needed no amendment to land — which is the thing worth recording.
std refused to ship an ASCII-only border post for four sprints and the
toolchain refused to ship an unchecked cast; both refusals were right and
the function they produced is the one both descriptions asked for.

What it does NOT close: `str_from_utf8` is the compiler's prelude only
(F-0075 below), so the function has two lanes. And its own family stays
open — `fs_read_bytes` (F-0044) is still unwritten, so a byte read still has
no producer to hand `to_str`.

### F-0037's closure is SPENT: the json DOM has its navigation

sc12 measured F-0037 closed and deliberately did not write the functions,
per sc10's rule that every finding on the SIGNATURE gets re-measured first.
This sprint did that re-measurement before writing a line:

- **F-0037** — `fn id(v: V) -> V ! {none} { v }` under lupin 0.1.11 prints
  `value path wins 7`. Closed, re-measured at the new release rather than
  inherited from sc12's note.
- **F-0029** — UNMOVED, and it is the finding the design already lives
  with. An enum VALUE crosses a module boundary (and now crosses one
  through an error row); a `match` in the importer is still
  `unsupported: no `match` arm applied; exhaustiveness is the type
  checker's`. `std.json` keeps every inspection inside the declaring module
  (`type_name`, `as_*`, `is_null`), so `get` and `at` hand an importer a
  value it can only read through this module — which is exactly what the
  four sc05 accessors already did.
- **F-0039** — UNMOVED. `int ! {none} ! {none}` runs under lupin 0.1.11 and
  is `fail(E0201)` at PARSE on both compiler rungs, re-measured with a
  fresh reproducer. It touches neither getter: one `!` per type is the
  portable budget and both signatures spend exactly one.

`std.json.get` and `std.json.at` ship with the signatures the sc05 contract
wrote, unchanged. The module's lanes are unchanged too (F-0029 + `Map`), so
the two new rows are `run / unsupported / unsupported` beside the four the
module already had.

**What is left, and it is now a debt rather than a wall.** `parse` and
`unescape` have both had every finding on them re-measured and both are
UNBLOCKED at these pins — `parse` by F-0037's closure, `unescape` by
F-0057's. §14's rule says a contract ships in the sprint AFTER its blocker
closes, which is one sprint of grace and no more, so both are owed next
sprint and both module headers say so with the clause. `escape`'s totality
rides with them, because it changes `stringify`'s row.

### Re-verified UNMOVED, each re-measured rather than assumed

- **F-0073** — the `--version` pairing line still says "paired with lupin
  0.1.8" at `4e316ad`, three releases after 0.1.8. Filed as wolf-lang#87,
  unmoved, and doctor still reports it without gating it.
- **F-0046 / F-0053** — `conform-run` still rejects `--deny-warnings`, and
  the record's `warnings` array still covers the entry file only. The rig
  denies warnings itself and stayed green.
- **F-0074** — measured again, because this sprint's rig went RED on it
  once. `fmt/decimal/shortest_round_trip.lu` under lupin 0.1.11 takes
  24.8 / 25.3 / 26.8 / 27.7 s on an idle machine and **116.6 s on a loaded
  one** — the same program, the same staged tree, five runs. The 60 s
  per-test ceiling is therefore not a margin the test has, it is a margin
  the MACHINE has, and sc12's "a CI timeout is a bisect, not a flake" reads
  one notch stronger from here: the bisect was already done, the mechanism
  is known, and what a red run means now is "this host was busy", which is
  a scheduling fact and not new evidence. Nothing in `std/` changed for it,
  for sc12's reason.
- F-0004, F-0011, F-0012, F-0016, F-0025's last third, F-0026, F-0027,
  F-0029, F-0030, F-0035 (the byte-type half), F-0038, F-0039, F-0040,
  F-0044, F-0045, F-0047, F-0049, F-0050, F-0051, F-0054, F-0058, F-0060,
  F-0061, F-0065, F-0066, F-0067, F-0068, F-0069, F-0070, F-0071, F-0072:
  all retested or unaffected, all open.

### The ledger movement

**Four rows added, zero rows moved.** 185 → 189. The two json rows are
one-lane (`run / unsupported / unsupported`) and the two bytes rows are
two-lane the other way (`unsupported / run / run`), which is the sprint in
one sentence: both halves are a closure being spent, and they have opposite
lane shapes because one is blocked by an interpreter-only enum property and
the other by a compiler-only prelude name.

No existing row changed verdict across the bump. s81 changed the LOWERING
of `str` equality and ADDED a prelude function; s80 fixed a miscompile no
lane here observes; s79 is a benchmark wave. That is the third time this
repository has recorded "the ledger is unchanged and that is the expected
result" (sc10, sc12, sc13), and the reason is the same one every time: the
ledger measures how deep each implementation gets, not what it costs.

## F-0075 — the interpreter has no `str_from_utf8`

s81 put the language's first bytes-to-str primitive in the compiler's
prelude. lupin 0.1.11, whose own conformance pin is `f8dca42` — the commit
before s81 merged — does not have it:

```text
$ lupin conform-run ./main.lu
unsupported: `str_from_utf8` does not resolve
```

That is the GENERIC unknown-name refusal. It is not the reasoned decline
this machine gives the four tiers it has decided about (`fs_*`: no
filesystem by design; `net_*`: no sockets; `json_*`: "declines the surface
rather than risk a second, guessed RFC 8259 reading"; the process trio:
"runs no child processes by design"). A pure, total, table-free function
that turns a `List[int]` into a `str` is nothing like those four — there is
no capability to decline and no second reading to risk, since RFC 3629 is
one page and this repository has already implemented it twice (once in
`bytes.is_utf8`, once in `str.code_points`). So this reads as drift, which
is F-0070's shape a second time, and F-0070's lesson holds: a builtin
FAMILY is not a unit of evidence, a builtin is.

**The cost, in this repository:** `std.bytes.to_str` lands with two lanes
instead of three, and `tests/bytes/to_str_border.lu` and
`to_str_row.lu` carry dark interpreter columns while the module's eight
other rows stay three-lane (lupin resolves module bodies lazily, so nothing
else in `std.bytes` pays). It also decided a design question:
`bytes.is_utf8` did NOT become a one-line call to the primitive, because
that would have traded the predicate's third lane for a tautology. The two
decoders stay independent and a test asserts they agree, which is the
better arrangement anyway and would not have been chosen without this
finding.

The ask is the builtin, at the release that re-pins past `4e316ad`.

## F-0076 — the native rung cannot compare two `bool`s

**CLOSED at the sc12 (02-os) pin** (s88, wolf-lang#100 — "native: two bools
can be compared"). `tests/fmt/parse_bool.lu`'s native column advanced
`unsupported` -> `run`, six sprints after it went dark and two after the
mechanism was named. std's bodies are unchanged: `!p` instead of
`p == false`, and a branch instead of `p == q`, cost nothing and reverting
them would be churn with no reader-visible gain.

The filing as it stood:

Measured with four one-line programs after `tests/bytes/to_str_border.lu`
lost its native lane to `is_utf8(b) == accepts(b)`:

```text
a == b          unsupported — comparison outside integers/floats  @mem
a == true       unsupported — comparison outside integers/floats  @mem
t() == true     unsupported — comparison outside integers/floats  @mem
true == false   unsupported — comparison outside integers/floats  @mem
t() != false    unsupported — comparison outside integers/floats  @mem
s1() == "wolf"  run  (str equality lowers since s81)
n == 3          run
x == 1.5        run
```

The refusal's parenthetical is "(str/enum compares, c06/std)", which names
the two cases it was written for and does not name `bool` — and that is why
this has gone six sprints undiagnosed. `tests/fmt/parse_bool.lu` is ten
`== true`/`== false` assertions and has carried `native = "unsupported"`
since sc05 with no explanation beside it; that row is this finding, and it
is where the claim is now held.

Writing around it is trivial: `!p` instead of `p == false`, and a branch
instead of `p == q`. That is the argument FOR fixing it rather than
against. Nothing about comparing two `i1`s is hard, the checked tier and
the interpreter both do it, and what a library pays today is a lane for a
spelling — which is the same shape as F-0071 and gets the same response
here: write the form that keeps every lane, and file the one that does not.

## F-0077 — a `comptime fn` cannot build a `List`

```text
comptime fn probe() -> int { var k = 0  k = k + 3  k }        run
comptime fn probe() -> int { let s = "hi"  s.len }            run
comptime fn probe() -> int { miss(3) else 0 }                 run
comptime fn probe() -> int { let b = List[int]()  b.len }     unsupported @resolve
comptime fn probe() -> int { str_from_utf8(List[int]()) … }   unsupported @resolve
```

Both compiler rungs, no code and no reason string in the record — F-0051's
silence, which is why this needed a bisect rather than a reading. The
refusal is the `List`, not the builtin and not the row.

The consequence is a rule and not an inconvenience: **a pure builtin whose
argument is a `List` is unreachable at comptime, whatever the sandbox
thinks.** `str_from_utf8` is the first such builtin and `std.bytes.to_str`
is the first std function whose comptime story has to be written as "the
D33 sandbox has no objection — it carries no capability and no sandbox
category — and the engine cannot get there anyway". That sentence is in the
function's doc, measured rather than inferred, because the alternative was
to write "pure and comptime-safe" and be wrong in a way no test here would
have caught (§13, sc09's rule: a doc sentence about what an implementation
answers is a test or it is a rumour — and when it cannot be a test, it is a
measurement with its date on it).

The ask: `List` construction and indexing inside the comptime engine (s16's
own scope), or a named refusal so a package author learns why — F-0060 and
F-0069 have asked for the second half twice, and this is the third caller.

## F-0078 — the reference lane reads a list as slowly as it used to build one

wolf-interp#24 (F-0074) is FIXED at 0.1.12, and the fix is worth stating
because it explains what is left: the cost was never in `push`, it was in
`eval_method` copying the receiver out of its slot, copying it again to
compare against, comparing two whole values to decide whether the method
had written, and copying the result back. Four traversals per append. The
fix lends the receiver instead. Upstream measures 32k pushes at 30.33s
before and 0.191s after.

**The index read gets none of that**, measured here at the new pin, release
build, same program at four sizes:

| N | `xs[i]` in a loop | `for v in xs` |
|---|---|---|
| 2 000 | 0.072 s | 0.016 s |
| 4 000 | 0.234 s | 0.027 s |
| 8 000 | 0.895 s | 0.053 s |
| 16 000 | 3.442 s | 0.107 s |

Four times the work per doubling against twice. And a read-mode `List`
ARGUMENT copies as well, so the two costs compound: 20 000 calls of
`fn value_at(bs: List[int], at: int) -> int { bs[at] }` over a
20 000-element list is **58.1 s**, where the same index read written inline
in the loop is 5.3 s and a `str` read-mode argument at the same size is
0.33 s. It is the container, not argument passing in general.

**What it cost this sprint, in the design rather than in the clock.**
`std.json.parse` was going to materialize `s.bytes()` once (the shape
F-0071 says runs everywhere) and index it. That is quadratic in the
document on the module's ONLY executing lane, so the scanner reads through
`text.get(i..i + 1)` and takes the first byte with a one-element `for`
instead — measured at 0.876s for 40 000 steps against the index walk's
curve above. The shape is fine and the body is no worse for it, but it was
chosen for one implementation's cost, which is the thing this repository's
house rule tries not to do. Saying so here is the alternative to pretending
it was a language-level preference.

**What it explains.** `fmt/decimal/shortest_round_trip.lu` is this repo's
slowest test (26–43s against a 60s ceiling on a host running three other
sprints' cargo jobs) and it is base-10⁹ limbs in a `List[int]` indexed in
the inner loop. The push fix made it about 20% faster — 41.5s → 33.5s and
33.1s → 25.9s, two pairs of runs, same host, same load — and not 100×,
which is exactly what this finding predicts and is the reason to state the
mechanism rather than to celebrate the release and move on.

## F-0079 — a handler cannot tell an imported module's tags apart

**Silent wrong answer**, and the mirror of F-0052 (which was the compiler's
checked lane doing the same thing until s70).

```wolf
// std/tagmod/tagmod.lu
pub fn miss(k: int) -> int ! {alpha, beta, gamma} { … }

// entry
tagmod.miss(k) else |e| match e { alpha => 10, beta => 20, gamma => 30 }
```

| lane | arms forward | arms reversed | expected |
|---|---|---|---|
| **lupin 0.1.12** | `10 10 10` | `30 30 30` | `10 20 30` |
| wolf `--checked` | `10 20 30` | `10 20 30` | ✓ |
| wolf `--native` | `10 20 30` | `10 20 30` | ✓ |

Exit 0, no diagnostic, both arm orders measured — which is the experiment
that tells "first arm always" from "one tag happened to be right", and it
is sc08's experiment run again on the other machine. The tag is NOT lost on
the way out: the same call propagated out of `main` prints `error: beta`
correctly, and a wildcard `else |_| { … }` behaves. It is the arm
resolution against an IMPORTED callee's row.

The same shape over a row raised in the ENTRY FILE discriminates correctly
here — `tests/errors/handler_discriminates.lu` is three-lane green at this
pin, re-run — so the two files together say exactly where the line is.

**How it was found**, which is the part worth keeping: a 66-level document
made `std.json.parse` raise `deep`, and the test's handler printed
`syntax`. The reproducer that mattered was not the handler but the
alternative witness — the same document with the tag ridden out of `main`
printed `error: deep`, and the disagreement between those two readings is
what turned "my parser has a bug" into "the handler is lying". When two
ways of observing the same value disagree, bisect the OBSERVERS before the
code (sc12's rule, arriving from a new direction).

**The cost here**: `std.json.parse`'s number branch uses a wildcard
`else |_| { return overflow }` rather than the two-arm handler it wants
(honest, because the grammar has already been checked and only one tag is
reachable — but chosen for this and not only for that), and `deep` and
`overflow` each get their own test file so the record names the tag instead
of a handler asserting it. §14's "a wildcard that claims nothing is the
right handler when the module cannot act on the difference" now has a
second clause: it is also the right handler when the module cannot SEE the
difference.


## F-0080 — the net tier has no byte-level read or write

`net_read(fd, max) -> str ! {closed, timeout, utf8, io}`. TCP delivers
whatever has arrived, so a stream carrying non-ASCII text splits a code
point across two reads and the second sequence is a `utf8` miss — and there
is nothing a caller can do about it, because the two halves it would join
never become values. `std.net.read`'s doc has said so since sc08 and
`std.net.read_all` is honest only over ASCII.

**This used to be the same gap as `std.fs`'s, and it is not any more.** s90
gave the fs tier `fs_read_bytes`/`fs_write_bytes` (whole file) and
`fs_read_chunk`/`fs_write_chunk` (handle), all carrying `List[int]`, with no
`utf8` row anywhere and an `invalid` row for an element that is not a byte —
so `std.fs.copy_file` is a byte copy and a file holding a lone `0x80`
survives it. The shape of the fix is therefore settled, implemented,
lowered on both compiler rungs and shipped one tier over.

**The ask**: `net_read_bytes(fd, max) -> List[int] ! {closed, timeout, io}`
and `net_write_bytes(fd, b) -> () ! {closed, invalid, io}`, the same
currency and the same rows the fs chunk pair uses. Then a reader joins
chunks and validates once with `str_from_utf8`, where the validation belongs
— over the whole message, rather than once per arbitrary boundary the
network chose.

Why it matters more than a `utf8` row usually would: the fs tier's version
of this bug was silent corruption of a caller's data (`copy_file` refusing a
binary file), and the net tier's version is worse, because the boundary is
not the caller's chunk size but the network's packetization — the same
program is correct on a fast loopback and wrong across a link that
fragments. `std.net`'s header records the gap and cites this finding where
it used to cite F-0044.

## F-0081 — the interpreter declines the s38 fs names with a sentence and the s90 names with a shrug

Two refusals from lupin 0.1.12, same tier, same machine, same run:

```text
unsupported: `fs_write_text` is the s38 io/fs surface; this machine has no
filesystem (or injectable stdin) by design, so the fs tier is declined
rather than mocked

unsupported: `fs_read_dir` does not resolve
```

The first is a POSTURE — a decision, recorded, permanent, and exactly what
`CONTRIBUTING.md` records once for the whole tier. The second is the generic
unknown-name refusal, which is what a name that simply has not been wired
looks like: DRIFT, the kind that closes on a release.

**It costs no ledger row and it never will**: lupin has no filesystem under
either sentence, so every `tests/fs` row is `unsupported` either way. What it
costs is the reader's ability to tell those two things apart, and this
repository has now twice had that distinction turn out to be the whole
question. F-0070 (`env_vars` "does not resolve" while its four siblings ran)
was drift and closed on a release. F-0075 (`str_from_utf8`) was drift and
closed one release after filing. `json_*` and the process trio are postures
and will not close at all. The rule sc11 wrote — *a drift closes on a
release, a posture closes on a decision* — is unusable when the refusal text
does not say which one it is.

**The ask is one line**: put the fifteen s90 names in the same
declined-by-design table as the s38 nine, so the whole fs tier answers with
the sentence it has earned. Nothing else about the machine needs to change.

## F-0087 — an object can be counted and never walked: the json tier has no key enumeration

Filed: [wolf-lang#123](https://github.com/wolffe-lang/wolf-lang/issues/123).

The shape, unchanged since sc10 and now inherited twice: `json_len` on an
object answers how many members it has, `json_get`/`json_type` answer about
a member you can NAME, and nothing in the family names one. A program
holding a document with unknown keys — a config with optional sections, a
map keyed by user data, every "iterate the object" loop ever written — can
learn the count and nothing else. `std.x.json.keys(doc, path)` has been a
reviewed contract in the module header since the module existed, and sc15's
DOM half inherits the hole whole: `keys_of(n)` over a `Node` is the same
contract one handle deeper, so the nursery now carries the same missing
function at two tiers of its own surface.

**Why sc15 files it rather than working around it**: there is no
workaround. Every other gap in this module's history had a spelling one
tier down (a loop, a guard, a helper); this one has none — the information
is simply not exposed. The no-invented-surface rule makes that a NAMED
STOP: the ask is written, the contract stays in the header, and the sprint
ships without it.

**The ask**: `json_keys(doc, path) -> List[str] ! {parse, missing, kind}` —
member names, in document order, `kind` for a non-object exactly as
`json_len` spells it. Both precedents are already paid for upstream: s90's
`fs_read_dir` proved the `List`-returning builtin shape (names, sorted,
over the eu ABI on every lane), and s107 built this family's two-copy
parity discipline (`wolf_mem::json` reference, `wolf_rt::json` hand
mirror, the driver's json_parity test pinning them together). One kernel,
in the family's own shape, and both std read surfaces complete on the day
it lands.

## F-0088 — the json kernels read a duplicate key first-wins where std.json reads it last-wins

Filed: [wolf-lang#124](https://github.com/wolffe-lang/wolf-lang/issues/124).

Measured at the sc15 pins (wolf at the 1b149ba bump, checked and native
lanes agreeing), with the probe's own output:

```text
let doc = "{{\"a\": 1, \"a\": 2}}"
json_get(doc, "a")   ->  "1"        (the FIRST occurrence)
json_len(doc, "")    ->  2          (both occurrences COUNT)
```

`std.json.parse` of the same text keeps the LAST occurrence and counts ONE
member — a `Map` assignment rather than a policy, held as a test in
`tests/json/` since sc14 — and last-wins is also the wider ecosystem's
reading (JavaScript's `JSON.parse`, serde, Python's `json`). RFC 8259 §4
permits both: names "SHOULD be unique", and when they are not, behaviour
is explicitly implementation-defined. So neither surface is wrong. What is
wrong is that ONE toolchain now answers the same legal document two ways
depending on which std read surface a program reached for, and no corpus
witness pins either reading (`corpus/json/rows.lu` and `query.lu` never
present a duplicate key).

**Where it bites**: the walk in `wolf_mem::json` resolves an object
segment with a first-match scan and `len_of` counts the raw member list,
so the divergence is structural, identical in the `wolf_rt` mirror (the
parity test covers the two copies against each other, not against the
DOM), and invisible to any test that never writes a duplicate key.

**What std does meanwhile**: states the measured behaviour in
`std.x.json`'s header with this finding cited, and HOLDS it
(`tests/x/json/dom_typed_reads.lu` pins first-wins-and-both-count through
the DOM half), so an upstream ruling flips a test row here instead of
rotting a doc sentence — the sc09 guard discipline, applied to a
divergence between two surfaces of one toolchain.

**The ask**: a ruling, in the spec or the kernel's module doc, on which
reading the json tier promises — and if the answer is last-wins, the
kernel walk takes it in both copies; if first-wins, `std.json.parse`'s
duplicate rule is restated where its header currently says "a Map
assignment, not a policy". Either answer retires this finding; the corpus
gains the one-line witness either way.
