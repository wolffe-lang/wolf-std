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
| F-0018 | 2026-08-12 | (sc03 Targets 1–4) **The boundary primitive is missing**: no recoverable slice (`str.get`), no byte accessor, no `chars()`/`char` type — so no scan can advance past a code point of unknown length, and 28 of sc03's 64 functions are unwritable rather than merely unimplemented; plus `^n` resolving nowhere, `str` methods `unsupported` in wolfc, `str + str` diverging | wolf-lang s37 (core types) owners | [filed: wolf-lang#17](https://github.com/wolffe-lang/wolf-lang/issues/17) — **#17 CLOSED at s120/s121 and sc24 RE-MEASURED every clause, twice — the sprint spanned TWO wolf acquisitions**: 21 of the 28 were already shipped (sc09–sc13); `^n` HEALED on all three lanes (its one residue is F-0096); and after the mid-sprint s121-carrying wolf (@ a900b8c) landed, sc24 SHIPPED three more — `str.to_list_chars`, `strbuf.push(c: char)`, `unicode.code(c: char)`, wolf-lane rows (lupin's char tier is is26, ledgered by lane) — for **24 of 28 shipped**. The 4-item residue: `chars` (the pair — needs tuple lists, `List[(int, int)]` still `unsupported` at resolve on the wolf rungs), `strbuf.in(r)` (region placement), `reserve` (capacity/SSO), `graphemes` (segmentation tier). See "the sc24 re-measure" below. **sc27: residue row 6 — the `str + str` adjacent clause — HEALS at wolf @ 0a5c1af (D62, [type.str.concat]): `+`/`+=`/chains three-lane, mixes E0409 by ruling; the 4-item residue stands (`List[(int, int)]` re-probed `unsupported` at resolve on both rungs — s128 destructures tuples, it does not instantiate tuple lists). Adoption filed for the next contract, not taken** |
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
| F-0035 | 2026-08-13 | (sc05 Targets 3-4) **The encoders have no byte type**: `std.bytes` is still 0/9 (F-0018 re-tested, unchanged), so `std.hex` and `std.base64` ship over `List[int]` with a documented 0..255 element contract — and `hex.encode(str)`, the commonest use of a hex encoder anywhere, cannot exist because nothing reads a `str`'s bytes. The same root blocks `json.parse`, `json.unescape`, `fmt.truncate_to`, and forces `json.escape`'s one refusal | wolf-lang s37 core types | [filed: wolf-lang#17](https://github.com/wolffe-lang/wolf-lang/issues/17) (sc05 evidence on F-0018's issue) — **sc24 re-measure: stale in the reader's favour and now corrected**: `std.bytes` has been 9/9 since sc09 (+`to_str` sc13, +`is_utf8` beyond the contract), `hex.encode_str` and `fmt.truncate_to` shipped sc10, `json.parse`/`unescape`/`escape`-totality and `hex.decode_str` shipped sc14 — every knock-on this finding names was cashed before sc24, and #17 is closed. CLOSED with F-0018 |
| F-0036 | 2026-08-13 | **Silent wrong answer**: a row tag that shares a name with anything in the value namespace at the raise site resolves to that THING instead of raising — `-> int ! {tagmod}` inside module `tagmod` hands the caller the module value, `else` never fires and no diagnostic appears. Found three ways in one sprint (`std.hex` raising `hex`, `std.json`'s `kind` function versus its `kind` tag, and `std.fmt.decimal` nearly raising `range` beside `std.range`) | wolf-lang resolve + wolf-interp | [filed: wolf-lang#30](https://github.com/wolffe-lang/wolf-lang/issues/30) — wolfc half closed at the sc07 pin; **the lupin half HEALED, measured at 0.1.14 (sc24)**: both collision shapes (tag-vs-module and tag-vs-function, the exact reproducers above) raise correctly and `else` fires. CLOSED; the both-direction naming grep stands down after eleven sprints |
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
| F-0093 | 2026-08-26 | **under lupin, per-iteration cost grows with program AGE**: the same 129 CAVP SHA-384 short vectors cost ~27s as four programs and ~132s as one (16 alone: 0.69s; first 64: 10.6s) — total quadratic in vector count, not message-length scaling, and `region { }` scoping per vector measures identical. Distinct from F-0074/F-0078's per-operation costs: no per-op fix moves a curve keyed to how much the program has already run. Sized this sprint's suites: the CAVP short sets are chunked 2/4/4 to keep the differential lane inside the ceiling, and the long/Monte suites are `slow`-skipped there (the sc16 ledger word). **sc17 re-measure (the AEAD rung, 0.1.13)**: the curve holds at new constants — 32 ChaCha20-Poly1305 seal+open pairs 98s in one program, 30 opens 29s, a 10-case smoke 2.1s — so the Wycheproof AEAD parts are chunked 32/30 with `slow` and smoke subsets from the start, and each part flips back per-part at the fixing bump (evidence commented on the issue). **sc18 re-measure (the curve rung, 0.1.13, is20 still unreleased)**: the curve is heavier per program than the digests — one X25519 Montgomery ladder is ~2.5-6.5s, one Ed25519 verify ~7-8s, so the differential lupin column shrank to a 5-vector X25519 smoke (~13s) and a 1-valid+1-invalid Ed smoke (~18s); the 1000-iteration chain, the 40-vector Wycheproof parts and the RFC 8032 §7.1 set are native + `slow`, and the checked tier additionally exhausts its STEP budget past a few ladders (a capacity refusal, the sha2-long mechanism, not F-0091's operator gap — that one closed at this pin). is20's fix (merged wolf-interp ec4b9c4, unreleased) is the exit for every curve `slow` row | wolf-interp | [filed: wolf-interp#41](https://github.com/wolffe-lang/wolf-interp/issues/41); sc16, the honest-slow-skip's mechanism; sc17 evidence appended; **sc25**: the 4 rows still carrying `slow` after the sc24 re-measure produced the 50M-step refusal at 19.3-20.4s idle on nomad-1/arm64 (the sc24 rig's wall clock, not depth — no perf commit in v0.1.14..v0.1.15) and moved to `unsupported`; the `slow` word retires at this pin with zero carriers, evidence appended to the closed #41 |
| F-0094 | 2026-08-27 | **the AES-GCM/ChaCha reconciliation for RFC 8448 (a std design decision, not an upstream defect)**: RFC 8448 §3 is the TLS 1.3 record layer's vector spine, but its records are AES-128-GCM (cipher suite TLS_AES_128_GCM_SHA256), while the MTI AEAD this ladder ships is ChaCha20-Poly1305 (§9.1; AES-GCM is a later rung's D-question, out of the sc20 contract). Resolution: the key schedule, the per-record nonce, the record header and the additional-data are all AEAD-INDEPENDENT and are asserted against RFC 8448 byte-for-byte (16-byte AES keys included — `hkdf_label`/`expand_label`/`derive_secret`/`traffic_key`/`traffic_iv` reproduce every §3 secret, `info` dump and traffic key); only the sealed ciphertext+tag needs the actual cipher, and that byte-match uses ChaCha20-Poly1305 fixtures derived from RFC 8448's REAL server-handshake-traffic secret (a 32-byte `"key"` Expand-Label where the trace takes 16 — same derivation, different length), generated and pinned by the independent reference (the sc17 discipline). The trace stays the spine; the cipher stays the MTI. Not filed upstream — this is a vendoring/scope note, recorded so the split is not mistaken for missing coverage | wolf-std (sc20) | docs/findings.md + `vendor/vectors/README.md` (RFC 8448 take/omit); the AES-GCM record rung retires it by adding the second cipher |
| F-0095 | 2026-08-27 | **the checked mem tier refuses `for x in <List bound through a DIVERGING `else`>` on the reject path** — "unsupported — iteration outside ranges and List". `let inner = chacha20.open(...) else \|_\| { return tag }` then `for b in inner { ... }` runs on the checked tier when the open SUCCEEDS (the valid-record path is three-lane, `chacha_records.lu`), but a program whose executed path makes the open FAIL — the tamper-reject witness `reject_tampered_row.lu` — is `unsupported` at `mem`: after a diverging `else` the tier's abstract value for `inner` is not a modelled List, so the downstream iteration is refused. Per-executed-path, not module-wide (the sc17 dynamic-refusal shape): the module's `open` body is identical in both tests, only the taken path differs. Costs the tamper witness its checked column (`wolfc = "unsupported"`, native runs it); the valid open carries the lane. A decrypt-then-walk reject witness over any AEAD inherits it | wolf-lang | [filed: wolf-lang#139](https://github.com/wolffe-lang/wolf-lang/issues/139); the checked tier should model a List bound through a diverging `else` as the callee's success type; sc20 — **CLOSED at the sc24 BINARY bump**: #139's fix reached the pinned wolf (@ e7abf03) and `reject_tampered_row`'s wolfc column flipped `unsupported` -> `run`, exactly the flip the sc22/sc23 carried lead predicted |
| F-0096 | 2026-08-28 | **`get` refuses an end-relative endpoint at resolve on both compiler rungs** — `s.get(0..^1)` is `unsupported` at resolve under wolfc AND native while `s[..^1]` slices on the same binaries and `[mem.str.get]`'s own sentence says `^n` resolves exactly as in `s[a..b]` before the domain question; lupin 0.1.14 runs it (hit bit-identical, `^n`-into-a-code-point the `none` miss). The one incomplete corner of `^n`'s sc24 healing; held as `tests/str/end_relative_get.lu` (`run/unsupported/unsupported`) | wolf-lang | [filed: wolf-lang#164](https://github.com/wolffe-lang/wolf-lang/issues/164); sc24 |
| F-0097 | 2026-08-28 | **Silent wrong answer: a multi-arm `else`-match over a BUILTIN-raised row takes its FIRST ARM under lupin 0.1.14** — measured in both arm orders over `net_connect`'s `refused` (`-1` with `refused` first, `-9` with `io` first, same call); the propagated tag out of `main` is RIGHT (`error: refused`), which is what caught it. F-0052/F-0079's mechanism at a THIRD address (entry-file raises healed s70; module raises healed 0.1.13/#29; a builtin's row is in neither place the checker consults, so every arm binds). Newly observable: 0.1.14 is the first lupin with net/process tiers. Costs `net/closed_row` + `net/refused_row` their lupin stdout; ledgered `divergent(stdout)` (the sc24 word) so the fix reads as a red | wolf-interp | [filed: wolf-interp#47](https://github.com/wolffe-lang/wolf-interp/issues/47); sc24 |
| F-0098 | 2026-08-28 | **take-mode reuse is a STATIC `fail(E1001)` on both compiler rungs and EXECUTES under lupin 0.1.14** — `net/use_after_close` runs to `trap(use-after-move)` (`[mem.tier0.move.2]`, the trap map's own dynamic answer) and `process/use_after_wait` never reaches the reuse (`start` honestly raises `not_found` first, `error: not_found` exit 1). Whether take-reuse joins lupin's static set (the E1007 precedent) or the dynamic trap is the reference is the counterparty's ruling to make; the two rows carry `divergent(trap(use-after-move))` / `divergent(exit(1))` until it lands | wolf-interp | [filed: wolf-interp#48](https://github.com/wolffe-lang/wolf-interp/issues/48); sc24 |
| F-0099 | 2026-08-28 | **`[conf.anchor.ns]` does not admit `type` (nor `ct`/`diag`/`os`) while `spec-extract` publishes their anchors** — the s115/#120 class again, four documents later: `[type.char]` is in the registry the clause calls authoritative and a conforming test may not cite it by the clause's own letter. Surfaced by sc24's char-surface tests (`conforms: type.char` refused by this rig, which mirrors the letter); the same investigation caught this rig's own registry lagging `pkg` by nine sprints (fixed here). The reserved-forward `ty` now points at a name nothing will use — named in the filing for the same decision | wolf-lang spec | [filed: wolf-lang#165](https://github.com/wolffe-lang/wolf-lang/issues/165); sc24 |
| F-0100 | 2026-08-28 | **`spec-extract`'s anchor scanner swallows a still-normative anchor: `gram.lex.ident` fell out of anchors.json at the s126 regen** — the scanner pairs each `[` with the NEXT `]` and resumes past it, and s126's amended `[gram.lex.shebang]` prose introduces bare literal `[` runs (`` `[` ``, `` `#[` ``, `` `#![` ``): the `[` of `` `#[` `` pairs with the `]` of the `### 1.3 Identifiers [gram.lex.ident]` heading below it, the span swallows the anchor's only occurrence, and the registry the spec calls authoritative ([conf.anchor.index]) silently disagrees with the spec's own text. `link_check` cannot catch it (it tests `contains`, still true). Measured at the sc26 bump: registry 393 -> 397 is +5/-1 where the clean delta is +5/0. No wolf-std test cites the anchor, so nothing reds here; the snapshot is re-vendored byte-faithful with the defect named in the bump record | wolf-lang spec/xtask | [filed: wolf-lang#170](https://github.com/wolffe-lang/wolf-lang/issues/170); sc26 |
| F-0102 | 2026-08-30 | **lupin 0.1.18 does not resolve the net BYTE tier (`net_read_bytes`/`net_write_bytes`), where it resolves the str tier and `net_deadline`** — s106 shipped all three builtins and lupin's is18 socket crossing (0.1.14) took the str calls + the deadline (both measured three-lane at the sc29 pin: a 150ms `set_deadline` budget resolves a silent-peer `net_read` as `timeout` under lupin, and a peer-less `net_accept` too), but the byte pair is `unsupported: `net_write_bytes` does not resolve` at resolve — the one-release-behind half of the same tier. Costs the sc29 byte-tier facade rows (`net/bytes_round_trip`, `net/write_bytes_invalid_row`) their lupin lane and every `std.x.tls.client` socket witness its lupin lane (`loopback_handshake` is native-only for this AND the step budget); the pure client parse/verify half stays three-lane by lupin's lazy body resolution. The str/deadline tier proves the surface exists on that lane, so this is a completeness gap in the byte mirror, not a design decline — the shape of the fix is the str calls' own | wolf-interp | [filed: wolf-interp#52](https://github.com/wolffe-lang/wolf-interp/issues/52); sc29 — **CLOSED at sc30** by lupin 0.1.19 (is30, #52 paid exactly as filed: the byte pair lands as the str calls' own shape, `List[int]` marshalling, no utf8 row, whole-or-raise writes). Healing measured at the sc30 bump: `net/bytes_round_trip` and `net/write_bytes_invalid_row` flip lupin `unsupported` -> `run` at FIRST SIGHT against sc29 bodies untouched (declared against the fix, the F-0049 pattern), the sc29 byte-tier block goes three-lane, and `loopback_handshake`'s lupin lane flips all the way to `run` — the "AND the step budget" half of its sc29 native-only attribution was an inference the resolve refusal had shadowed, and the measurement outvoted it (18.9s idle, inside the 50M budget). #52 commented same-day with the downstream healing |
| F-0103 | 2026-09-01 | **the CHECKED tier refuses an unhandled raising call passed straight into a ROW-TYPED parameter — `unsupported — control flow in an argument` at `mem` — where lupin AND wolf's own native rung both run it.** The shape is `std.option`'s (`fn or[T](v: T ! {none}, default: T)`, sc06): a helper whose parameter is a row union, called as `row_name(narrow(1))`. Four probes characterize it, all three lanes green on the last three: a non-raising call in an argument; `take_int(narrow(9) else 0)`; `take_int(narrow(9)?)`; and `let r = narrow(1)` then `row_name(r)` — so the refusal is specific to the UNHANDLED union riding into the parameter, and the BOUND form of the same expression compiles on the same lane, which reads as a `mem`-phase argument-lowering gap rather than a typing rule. This is the cause behind `option/or_else_default.lu`, `option/exists_marking.lu` and `option/is_none_marking.lu` carrying `wolfc = unsupported` since sc06 with lupin and native both running them — the rows were ledgered, the cause had never been isolated. Costs `std.x.tls.client`'s sc31 naming pair its one-line call site: the surface documents "bind, then name" so every naming site stays three-lane (module header + `tests/x/tls/client/row_naming.lu`), which is a workaround in the CALLER's source for a limit only one of the two wolf tiers has | wolf-lang | [filed: wolf-lang#201](https://github.com/wolffe-lang/wolf-lang/issues/201); sc31 |
| F-0101 | 2026-08-30 | **the native rung refuses a slice of a LENT byte view at `mem`, and the refusal names the wrong conservatism** — `b[from..to]` in a callee is `unsupported` ("range VALUES outside `for` headers (owned `Iter[int]` ranges, c06/std)") when the caller lends `s.bytes()` across the call (s89), while the SAME callee over an owned list and the inline slice of an owned local both `exit(0)` at v0.2.0/c88ab64 — the message cites a limit the owned probes show lifted; the operative edge is the mem-phase lowering of slice-of-lent-view. `--checked` and lupin 0.1.18 run all three shapes. Caught by the sc28 slices adoption: `bytes.slice` took the range spelling and the gauntlet moved exactly one row (`tests/bytes/lend_across_calls.lu [native]`, `run` -> `unsupported`, the corpus's only lend-into-slice witness — `bytes.slice`'s doc examples lend too); the site retreated to the index loop the same day with the shape named in a comment, and the row holds `run` again. Blocks the s128 slice spelling in any std function whose parameter is lend-reachable; three-probe isolation at `probes/sc28_p6_slice_of_view` in the filing | wolf-lang | [filed: wolf-lang#184](https://github.com/wolffe-lang/wolf-lang/issues/184); sc28 — **CLOSED at sc30** by s129 (the #184 fix, in the sc30 pin b80d239): the diagnosis upstream was a real mem-phase gap wearing the retired range-value conservatism's refusal string, and the fix gave the lent-view path its slice arm. Healing measured at the re-adopt: `bytes.slice` took back `b[from..to]` and `lend_across_calls.lu` — the ONE row the sc28 adoption moved — holds `run` on ALL THREE lanes with the range spelling (native re-measured singly, exit(0), the exact shape that refused at `mem` at c88ab64); ledger flat everywhere else. The retreat commit bc01f8c reverses at the sc30 re-adopt with the arc named at the site (found -> filed -> fixed -> re-adopted); upstream's own `byte_view_slice_lent` corpus twin joined the two-machine agreement class at lupin's 83f83bb pin bump (lupin ran the lent slice all along). #184 commented same-day with the downstream healing |
| F-0104 | 2026-09-02 | **the byte-buffer cost, measured from the library's side** — a byte buffer held as `List[int]` charges 16.0x its payload on both wolf tiers and 32x under lupin, linear from 1 KiB to 64 KiB, reproducing wolf-lang#203's own numbers to the byte from a different program and again through std's readers (`fs.read_bytes` and `fs.read_chunk` charge identically, so #203's preallocation half is ENTIRELY unrealized). Recommended `[type.byte]` modelled on `[type.char]` as a spec-shaped proposal; both cheap answers (a std wrapper, the spec's `distinct` newtype) fail by the same layout-preserving mechanism. **CLOSED at sc34 with the after-table: 16.0x -> 2.0x native, 16.0x -> 1.0x checked** | wolf-lang | [filed: wolf-lang#203](https://github.com/wolffe-lang/wolf-lang/issues/203); D72 ruled, s135 landed; sc32-sc34 |
| F-0106 | 2026-09-02 | **the byte TYPE landed and the byte PRODUCERS did not: std's byte tier cannot take `List[byte]` without a conversion that costs more than the type saves.** `[type.byte]` is in the language at 31170d1 and there is NO byte-typed builtin — `str`'s `.bytes()`, `str_from_utf8`, `fs_read_bytes`/`write_bytes`/`read_chunk`/`write_chunk` and `net_read_bytes`/`write_bytes` are all still declared over `List[int]` in `wolf_sema`'s signature table. Every one of std's sixteen byte-tier functions is a thin wrapper over one of those eight, so a substituted signature must convert elementwise against a builtin, and the cumulative ledger keeps the intermediate charged: a substituted `fs.read_bytes` measures **17.0x checked / 18.0x native** against today's 16.0x, at every size — WORSE, at exactly the io sites #203 was filed about. The fix is upstream and small: move the eight builtin signatures, and the std change becomes the rename it was designed to be | wolf-lang | [filed: wolf-lang#231](https://github.com/wolffe-lang/wolf-lang/issues/231); sc34 |
| F-0107 | 2026-09-02 | **the checked machine charges 16x for a CONSUMED `s.bytes()` view where native and lupin charge nothing.** `for b in s.bytes()` over a 64 KiB `str` inside a fresh region reads `region_bytes` = 0 natively, 0 under lupin and **1,048,576** under `--checked`, for a walk that allocates nothing on the tier that ships — s77's borrow is not modelled in the checked ledger. Invisible to every gauntlet (no row may print a ledger count), it is the reason F-0106's one winning shape wins natively and regresses checked, and it makes `region r(cap: n)` mis-fire by an order of magnitude between tiers on the very idiom `std.bytes`' header teaches for byte walking | wolf-lang | [filed: wolf-lang#232](https://github.com/wolffe-lang/wolf-lang/issues/232); sc34 |


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

**CLOSED at lupin 0.1.13 (wolf-interp#29, the is13 arm-selection pass,
released 2026-08-15 against pin 02c1e88 — the RECORDED pin).** The row
now travels with the raised value, so arm resolution asks the value
which row it came from instead of the entry file's own declarations.
`tests/json/parse_misses.lu` witnessed the fix where the bug lived, and
sc22 re-confirmed it with a NINE-arm handler over
`std.x.tls.cert.validate_chain`'s row — every tag answered correctly
across the module boundary. What lagged was THIS REGISTER: the closure
was upstream in the very release wolf-std pinned, the counterparty's
changelog names the F-number, and two shipped files already relied on
the fix while this entry still read as open (the sc12 changelog rule,
missed for eight sprints and caught by the sc22 re-measure). The
prose that survives it: the ride-the-tag-out-of-main witness stays the
STRONGEST row evidence and every sc22 row test still uses it.

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

## Retirements and movements at the sc19 pin

**The pin is HELD at 21b129e; the re-measure moved ZERO rows.** The sc19
contract's first act — "the pin bump to wolf-lang 21b129e or later
(whatever trunk is at launch), re-vendor, flip any sc16/17/18 dark-lane
rows" — resolves to a HOLD, stated as a mechanism rather than a
non-event:

- **The DATA pin is already 21b129e** (sc18 set it, `vendor/upstream/PIN`
  and the `[wolf]` tool pin), which satisfies the contract's floor
  ("21b129e or later"). Trunk has since advanced to **da8582d** (s112,
  "the compiler keeps the secret" — the CONSTANT-TIME tier / c28: the
  `#[consttime]` attribute, the WIR taint verifier, E1601-E1607, and two
  objdump asm witnesses). The span's spec addition is the `ct.*` anchor
  namespace (346 -> 360 anchors, +14, zero removed — a clean superset).

- **No available binary conforms to da8582d in a way that keeps the
  ledger green.** The s112 ct tier is verified by c28's OWN witnesses in
  wolf-lang (the two flagship kernels, the seven E16xx refusals), NOT by
  any std test lane — so it lights no sc16/17/18 dark row. lupin stays
  0.1.13 (is20's F-0093 program-age fix remains MERGED-but-UNRELEASED, so
  every `slow` row keeps its 0.1.13 measurement, curve and JOSE alike).
  The 9 big sha2 vector rows stay `unsupported` on the checked tier's
  step/shadow-memory budgets (mechanism unchanged). **Flipped rows: 0.**

**The JOSE rung (sc19) filed NO new upstream finding, and that is the
rung working as designed.** `std.x.jose` is pure PLUMBING over three
existing primitives — sc18's `curve25519.sign`/`verify`, sc16's
`sha2.sum256`, and sc05's `base64.url_encode`/`url_decode` (§5, no pad,
reused not re-landed). It compiles and runs; the RFC 8037 A.4/A.5 (JWS),
RFC 8037 A.3 + RFC 7638 §3.1 (thumbprint) and the base64url negatives are
byte-exact on the reference lane. The one wolf-language friction — a JSON
literal in wolf source doubles its braces (`{{`/`}}`) because `{` opens
f-string interpolation in every literal — is DOCUMENTED grammar (the
Candidate-A `{x}`-in-every-literal decision), not a defect; `std.x.jose`
builds every JSON string with it, and the `acme_body`/thumbprint outputs
prove it round-trips byte-exact.

**One observation held for the NEXT pin bump to confirm-or-file** (not
filed now, because it cannot yet be separated from a local build
artifact): a `wolf` built from a working checkout at da8582d refused
`std.x.crypto.curve25519` on the NATIVE rung with `unsupported — `mut`
places beyond local by-value bindings`, where sc18's ledger runs that
module natively at 21b129e. If a clean release build at the next pinned
sha reproduces it, it is a native-lowering regression against the sc18
measurement and gets an F-number and a filing then; recorded here so the
next bump reads a lead rather than a surprise.

## Retirements and movements at the sc20 pin

**The DATA pin BUMPS 21b129e -> 77466a3; the re-measure moved ZERO
ledger rows.** sc20's first act — "pin bump to current wolf-lang trunk
(77466a3+), re-vendor, flip any dark-lane rows s113's D54 lights" —
resolves to a bump-with-no-flip, stated as a mechanism:

- **The bump is 21b129e -> 77466a3** (`vendor/upstream/PIN` +
  `vendor/upstream/anchors.json` re-vendored from `spec/anchors.json` at
  trunk 77466a3; `sync-pin` green). The span 21b129e..77466a3 is s112 +
  s113. The anchor delta is a clean superset (+24, zero removed): the
  `ct.*` namespace (s112, the constant-time tier / c28 — `ct.attr.*`,
  `ct.taint.*`) and the `type.numlit.*` namespace (s113, **D54** — int
  literals adopt the float type in a float context, casts trap).

- **No binary moved, so no row flips.** lupin stays **0.1.13** (its own
  conformance sha reports `da8582d`; is20's F-0093 program-age fix is
  still MERGED-but-UNRELEASED, so every `slow` row keeps its 0.1.13
  measurement). `wolf` stays **0.1.0**. D54 (s113) is a spec/compiler
  change the pinned 0.1.0 binary does not carry, and the crypto ladder
  touches no float context regardless (it is checked-`int` and
  `wrapping[u64]` throughout), so D54 lights no sc16-20 dark row. The
  s112 ct tier is verified by c28's OWN witnesses in wolf-lang, not by
  any std lane. **Flipped rows: 0.** (The sc18/sc19 dark-row set is
  unchanged: the 9 big sha2 vector rows on the checked step/shadow-memory
  budget, the curve/JOSE `slow` rows on F-0093.)

- **The sc19 lead did NOT reproduce as a clean regression here.** sc19
  held one observation for the next bump: a `wolf` from a working
  checkout at da8582d refused `std.x.crypto.curve25519` natively with
  "`mut` places beyond local by-value bindings". At the sc20 pin, in THIS
  sandbox, the `wolf 0.1.0` binary runs `std.x.tls.record` on ALL THREE
  lanes (lupin, checked, native) including the ChaCha20-Poly1305 record
  seal/open — so the native rung is NOT globally refusing crypto here.
  The curve-specific native refusal is not re-probed this sprint (sc20
  ships no new curve code); it stays a lead for the curve module's next
  re-measure, not yet an F-number.

**PROVISIONING NOTE (sandbox, not a finding).** `cargo xtask doctor` is
RED in this sandbox on ONE inherited line: the lupin binary reports pin
`da8582d` while `vendor/tools.toml` records `02c1e88` for lupin 0.1.13 —
a binary-acquisition drift, not a product of sc20. Per the sprint's own
guidance (lupin lane green, CI clean-clone is the real gate for the
compiled lanes) it is stated and NOT chased: `sync-pin`, `ledger-check`,
`lint-conventions`, `gen-vectors --check` are all green, and every sc20
test runs on all three lanes here. The clean-clone CI (3 OSes) is the
gate that decides the compiled lanes.

**The record layer (sc20) filed ONE upstream lead (F-0095) and one std
scope note (F-0094).** F-0095 (the checked tier refusing iteration over a
List bound through a diverging `else`, on the reject path) is a genuine
compiler modelling gap, costing the tamper-reject witness its checked
column. F-0094 (the AES-GCM/ChaCha reconciliation) is a vendoring/scope
decision, not a defect. Everything else is PLUMBING working as designed:
the module is HKDF (sc16) + AEAD_CHACHA20_POLY1305 (sc17) + public
framing, three-lane by construction, and the whole RFC 8448 §3 key
schedule reproduces byte-for-byte.

## Retirements and movements at the sc22 pin

**The DATA pin BUMPS d99d81a -> 64a38f3; the bump itself moves ZERO
ledger rows.** sc22's first act — "pin bump to current wolf-lang trunk
(64a38f3+), re-vendor, flip dark rows" — resolves to a
bump-with-no-flip plus TWO recorded leads and one loud provisioning
observation:

- **The bump is d99d81a -> 64a38f3** (`vendor/upstream/PIN` +
  `vendor/upstream/anchors.json` re-vendored at trunk 64a38f3;
  `sync-pin` green, snapshot-only per CI's normal state). The span is
  s115: net_read_bytes/net_write_bytes (#137 — the byte path the record
  layer's socket future needs), D56's `[type.numlit.cast.wrap]` clause
  (#135 — wrapping-as-int traps), and the diverging-else fix (#139).
  Anchor delta +1 (`type.numlit.cast.wrap`, 379 -> 380), zero removed.

- **LEAD: #139 is F-0095's fix, and it is in the DATA pin but not in
  any pinned binary.** The `wolf 0.1.0` binary conforms to c9da6d9,
  151 commits BEFORE 64a38f3, so `x/tls/record/reject_tampered_row`'s
  wolfc column stays `unsupported` until a binary bump. The next wolf
  binary bump should re-measure that row first and close F-0095's
  ledger half.

- **CLOSED: F-0079 — and the closure was eight sprints old.** A
  nine-arm `else |e| match e` over `std.x.tls.cert.validate_chain`'s
  row discriminated every tag correctly under this lupin, which sent
  sc22 to the counterparty's changelog: wolf-interp#29 was FIXED in
  lupin 0.1.13 itself (the is13 arm-selection pass, released
  2026-08-15 against 02c1e88 — the recorded pin), and
  `tests/json/parse_misses.lu` had already witnessed the fix while the
  register entry stayed open. Closed in the entry above with the
  timeline; the Guide's sc14 entry and API-CONVENTIONS' sc14
  amendment carry dated closure notes. sc22's tests never depended on
  it: every row witness rides its tag out of `main`, which stays the
  strongest shape regardless.

- **PROVISIONING OBSERVATION (loud, not an F-number, the sc20
  precedent's bigger sibling): this sandbox's binaries run 22 ledger
  rows DEEPER than the recorded pins.** lupin runs the 10 `x/json`
  rows (its own json reading — the is18 posture apparently closed on
  a release), 4 `net` rows (sockets, the "no sockets by design"
  posture), 2 `process` rows and 3 `cmp` rows; the native rung runs
  the 3 `fmt/decimal` f64 rows. `lupin --version` still SAYS 0.1.13 /
  pin da8582d while behaving beyond both, and doctor already flags the
  recorded-pin mismatch (da8582d vs 02c1e88). The ledger KEEPS the
  recorded-pin measurements — advancing 22 rows on an unidentifiable
  binary would break the clean-clone 3-OS CI that provisions at the
  recorded pins, and a posture flip (json, sockets, process on the
  interpreter) is exactly the kind of claim that needs a release to
  cite. The next DELIBERATE lupin/wolf binary bump inherits this list
  as its expected flip set: 22 rows + the F-0095 row.

**The certificate rung (sc22) files NO new upstream finding.** The
language carried a 1500-line DER/X.509 module on the first try's
shapes: the known MVS rule (read a field's `len` BEFORE moving the
list into a struct literal — the sc16/sc18 E1001 class, three sites)
and the known i32-literal rule (typed-int bindings around the
14-digit timestamp arithmetic, sc04/sc18's) were the only language
contact points, both already filed and both already in the Guide.
The multi-arm-handler observation above is a re-measure lead on an
EXISTING finding, not a new one.

## Retirements and movements at the sc23 pin

**The DATA pin BUMPS 64a38f3 -> 53f6191; the bump moves ZERO ledger
rows.** sc23's first act — "pin bump to current wolf-lang trunk
(53f6191+, s118 os_random + s119 codegen), re-vendor, flip dark rows" —
is a bump-with-no-flip (the binaries are unchanged: lupin 0.1.13, wolf
0.1.0), plus one carried lead and the persisting provisioning drift:

- **The bump is 64a38f3 -> 53f6191** (`vendor/upstream/PIN` +
  `anchors.json` re-vendored at trunk 53f6191; `sync-pin` green,
  snapshot-only). The span is s116-s119: s118's `os_random`
  (getrandom/getentropy/BCryptGenRandom, no fallback — the sanctioned
  entropy source sc23's RFC-6979 choice deliberately does NOT need) and
  s119's codegen fixes (the versioning pass's stale CFG, token-linearity,
  element-stride alignment). Anchor delta +6 (the `os.random`* family:
  os.random / .checked / .fill / .platform / .sole / .trap), 380 -> 386,
  zero removed.

- **LEAD, CARRIED: F-0095 is unmoved.** #139's fix is in the DATA pin
  (has been since 64a38f3) but not in the pinned `wolf 0.1.0` binary, so
  `x/tls/record/reject_tampered_row`'s wolfc column stays `unsupported`
  until a BINARY bump. Unchanged by a data-only pin move.

- **PROVISIONING DRIFT persists, unchanged (the sc22 set).** This
  sandbox's binaries still run the same ~22-26 rows deeper than the
  recorded pins (json/net/process/cmp on lupin; the f64 fmt/decimal rows
  on native), and `lupin --version` still says 0.1.13/da8582d while
  doctor flags da8582d vs the recorded 02c1e88. A data-only pin bump
  touches none of it. The ledger KEEPS the recorded-pin measurements;
  clean-clone 3-OS CI at the recorded pins is the authoritative gate for
  the compiled lanes (the sc19-22 precedent — stated, not chased). The
  next DELIBERATE binary bump inherits this list as its flip set.

**The Weierstrass rung (sc23) files NO new upstream language finding.**
The 2560-line P-256/ECDSA module rode entirely on shapes already filed
and already in the Guide: the sc04 i32-literal rule (every field/scalar
constant is a TYPED-`int` binding so a 16-bit limb literal cannot default
to i32 and trap two multiplies later — sc18's exact lesson, applied to
sixteen-limb constants), and the sc18 split-read-then-write rule for
in-place list mutation (`let v = x[i] + c` then `x[i] = v`, in the
Montgomery CIOS inner loops). The checked-`int`, no-`wrapping`,
no-bitwise-on-`int` discipline (F-0026's spelling) carried the whole
field. Three sc23 OBSERVATIONS, none a new upstream bug:

- **A table-free P-256 scalar multiplication exceeds BOTH non-native
  step budgets — the sc23 lane headline, and the real distinction from
  curve25519.** One `scalar_mul` is 256 iterations of a COMPLETE-formula
  double AND a complete-formula add AND an arithmetic select (the design
  ruling's table-free ladder) — roughly 256 x 30 field multiplies, each
  a 16x16 schoolbook. That is ~2x the interpreter cost of curve25519's
  Montgomery-ladder STEP and it runs a full double-and-add every bit, so
  a single P-256 scalar mult (a) returns lupin's `unsupported: did not
  terminate within 50000000 evaluation steps` and (b) exhausts the
  checked tier's step budget at `mem`. curve25519's single ladder fits
  lupin at ~2.5-6.5s (F-0093's curve, `slow`); P-256's does NOT fit at
  all — so every sign/verify/vector file here is NATIVE-ONLY
  (`unsupported`, a step-budget verdict, NOT `slow` — `slow` is a
  wall-clock timeout of a program that WOULD finish; lupin actively
  refuses this one). The three-lane column is the FIELD / DER codec /
  early-reject / trap files, whose executed paths never reach a ladder
  (the sc17 per-executed-path rule). This is inherent cost, not a bug:
  the ruling chose provably-constant-time over fast, and the honest
  price is a native-only differential. Recorded in the ledger's sc23
  section and every vector file header.

- **`let byte` shadows the builtin type name `byte` (W0304) — a latent
  rename debt, invisible to the rig today.** wolfc warns `W0304: this
  `let` binding shadows the built-in type name `byte`` on `let byte =
  k[i / 8]` — the bit-selector spelling `std.x.crypto.curve25519` also
  uses (sc18, unflagged) and `std.x.crypto.p256` inherited. The rig sees
  ENTRY-file warnings only (F-0053), and a std MODULE body's warnings are
  invisible from here, so neither module's row moves and CI is green. The
  day F-0053's open half closes (the rig reads module-body warnings),
  both curve modules owe a rename (`let byte` -> `let kb`/`let eb`). Not
  filed as a new number: it is F-0053's known blind spot, and the fix is
  a mechanical rename in two files when the rig can see it.

- **`wolf fmt` STILL splits a dotted call under a `//` comment (F-0016,
  the crypto campaign's recurring tax).** A `// class` comment directly
  above `if p256.verify_der(...)` made fmt emit `if p256.` newline
  `verify_der(...)`. The dodge is now reflex: the DER/early-reject test
  batteries hold their bad-input classes in a TABLE (a `List` walked in a
  loop) with the class list in the file header, so no comment sits above
  a dotted call. F-0016 unmoved, six-plus sprints running.

**A methodology note worth keeping (not a finding).** Half a debugging
hour was lost to a DEGENERATE lupin parse error — `E0201: expected }`,
found identifier `z` at 15..16` — that appears IDENTICALLY for a bare
`fn main() -> int { 0 }` and for `continue`/`loop`/`break`. It is not a
language gap (all three keywords RUN correctly under lupin in a clean
package root, measured): it was the PACKAGE ROOT. `lupin conform-run`
eagerly parses every `.lu` sibling of the entry file (the package =
directory rule, D32), and a scratch directory full of broken probe
files makes lupin report the FIRST sibling's failure against the entry's
name with a bogus span. The rig never sees this because it stages one
clean directory per test. Lesson, old: probe in a clean directory, one
`.lu` at a time — never in a shared scratch dir.

## Retirements and movements at the sc24 pin — the twenty-eight, re-measured

**The DATA pin BUMPS 53f6191 -> a900b8c and this is the DELIBERATE
BINARY BUMP sc22 predicted — measured across TWO wolf acquisitions,
because the provisioner swapped the binary mid-sprint**: lupin
0.1.13/02c1e88 -> **0.1.14**/90c90df (r02, the is14–is25 catch-up
release) and wolf 0.1.0 @ 21b129e -> first **@ e7abf03** (s120 + s122,
PRE-s121: `chars()` in the v0 `List[int]` currency, `'a'` E0107 at lex,
`char` E0301 at resolve) -> then **@ a900b8c**, the data pin itself,
carrying the whole of s121 (verified behaviorally: `'é'` lexes, `c as
int` total, `55296 as char` traps `overflow`, `{c}` prints the
character, `chars()` yields `List[char]` across module boundaries). The
swap was CAUGHT by the sprint's own claim-holding: the agreement test
written in the first binary's int currency went red within hours, which
is the designed signal arriving at the shortest interval it ever has.
Anchor delta +7
(`mem.str.chars`, `gram.lex.char`, the five-clause `type.char` family),
386 -> 393, zero removed. std-test measured the whole 359-row ledger
before a single edit: **23 rows deeper, 0 shallower** — the sc22
provisioning-drift list arriving release-backed, exactly as that note
predicted (3 `cmp` + 4 `net` + 2 `process` + 10 `x/json` lupin rows on
the is14–is18 waves; 3 `fmt/decimal` native rows; and
`x/tls/record/reject_tampered_row` wolfc, which is **F-0095 CLOSED** —
#139's fix finally in a pinned binary). Four net/process rows moved to
the new `divergent(…)` word (F-0097/F-0098 below). Four P-256 rows
(wycheproof_p256_p2/p3, the two tls/cert P-256 rows) timed out in that
FIRST run and re-measured to their old `unsupported` on a quiet
machine: 0.1.14's 50M-step refusal still fires, now at 48-56s idle —
knife-edge against the 60s ceiling, so a timeout there on a loaded
machine is load, not depth (F-0093's margin rule, sharpened).

**THE RE-MEASURED INVENTORY — the sprint contract's first-act number.**
F-0018's 28 unwritable functions, probed from wolf-std's side at THESE
binaries (not the data pin's text, not s120's compiler-side estimate of
23-then-25):

- **21 of 28 are SHIPPED, GREEN, and were before sc24 opened** —
  `std.str`'s twelve scanners (`find`, `rfind`, `count`, `split`,
  `split_once`, `rsplit_once`, `ends_with`, `strip_suffix`, `replace`,
  `replacen`, `get`, `bytes`; sc09, all three lanes) and all nine of
  `std.bytes` (sc09; `to_str` sc13; plus `is_utf8` beyond the
  contract). The findings' text ("std.bytes 0/9", "hex.encode(str)
  impossible") was ELEVEN SPRINTS stale when sc24's contract quoted it,
  which is exactly the class of error the contract's own first act
  exists to catch — in both directions.
- **At the FIRST acquisition, 0 of the remaining 7 were writable** —
  against the compiler-side estimate that the char tier's four are: D58
  ruled the type, the data pin carried it, and no pinned binary
  executed it (zero executing lanes is zero evidence, and a char-typed
  body would have cost `std.unicode`/`std.strbuf` their wolfc columns
  besides). **At the SECOND acquisition three of the four became
  writable and sc24 SHIPPED them the same day**: `str.to_list_chars ->
  List[char]` (a delegation to `s.chars()`), `strbuf.push(c: char)`
  (the `{c}`-interpolation body — the appended bytes are the scalar's
  UTF-8 encoding by `[type.char.interp]`), and `unicode.code(c: char)
  -> int` (`c as int`, total by `[type.char.cast]`). Wolf-lane rows,
  lupin honestly `unsupported` until is26 (every char in the tests and
  examples is spelled through `.chars()` so that lane's refusal stays
  the clean method refusal — `'` does not lex there). No int-shaped
  char helper anywhere: `code`/`from_code` are the only places a number
  and a code point trade places, and D58's not-an-integer ruling is
  enforced by construction. **The honest final number: 24 of 28
  shipped / 4 blocked** — against s120+s121's compiler-side 25.
- The 4, each with its owner: `chars` (the `(offset, char)` pair) —
  tuple lists (`List[(int, int)]` re-measured `unsupported` at resolve
  on both wolf rungs, at BOTH acquisitions); `strbuf.in(r)` — region
  placement over a std type (wolf-lang, the s37 `.in(r)` pattern);
  `strbuf.reserve(n)` — the capacity/SSO builder decision (wolf-lang
  D24; a no-op hint would be a lie); `graphemes` — segmentation
  tables, a later tier by contract and by the #18 ruling (committed
  generated wolf source when it comes), and sc24 deliberately does not
  start it. The FULL `std.unicode` retype (`from_code -> char`,
  char-typed predicates) is deliberately NOT residue on the char type:
  it waits for is26 so the flip costs no lane, and the module header
  records the reviewed final signatures (§14's debt clause, armed on
  the is26 release).

**F-0035's nine and every knock-on it names were already cashed** —
`std.bytes` 9/9 (above), `hex.encode_str` + `fmt.truncate_to` (sc10),
`json.parse` + `json.unescape` + `escape`'s totality + `hex.decode_str`
(sc14). Each retirement is recorded in its own sprint's section; what
sc24 adds is the correction of BOTH findings' register rows, because a
finding that is stale in the reader's favour is still stale.

**What sc24 could and did write against the s120 primitive**:

- **`^n` end-relative indexing HEALED on all three lanes** — F-0018's
  last living clause, dead: every from-end form (`s[..^1]`, `s[^2..]`,
  `s[^5..^1]`, `s[..^0]`, astral fixtures) slices byte-counted from the
  end, and a `^n` into a code point traps `bounds` identically
  everywhere. Held by `tests/str/end_relative_slice.lu` +
  `end_relative_split_codepoint_trap.lu` (both 3-lane green). The one
  residue is **F-0096**: `get(0..^1)` is `unsupported` at resolve on
  both wolf rungs against `[mem.str.get]`'s own sentence, while lupin
  runs it — filed (wolf-lang#164), held by
  `tests/str/end_relative_get.lu`.
- **The scalar tier held to std's own decoder**:
  `tests/str/chars_builtin_agree.lu` compares `str.to_list_chars` (the
  landed delegation) element-by-element against `str.code_points`
  (sc09's one-pass state machine) through the total `c as int` bridge,
  plus the `[mem.str.chars]` width-walk identity — two independent
  UTF-8 readings that must agree, the `bytes.is_utf8` pattern one tier
  up. The file's own history IS the sprint's: written first in the v0
  int currency with a header promising to go red at the s121 binary, it
  went red MID-SPRINT when that binary arrived, and the rewrite is the
  claim-holding mechanism paying out at the shortest interval on
  record. `tests/strbuf/push_char.lu` and `tests/unicode/char_code.lu`
  carry the other two landings (byte-honest round-trips; the
  `from_code`/`code` inverse loop).

**F-0036 is CLOSED, and the closure was measured before it was
believed.** Both reproducer shapes (a tag sharing a module's name, a
tag sharing a function's name) raise correctly under lupin 0.1.14 with
`else` firing — the wolfc half closed at sc07 (#30), and the r02
catch-up carried the interpreter half without naming it. Eleven sprints
of both-direction naming greps stand down.

**Two NEW divergences, both surfaced by lupin's first net/process
release, both filed the day they were measured, both ledgered under the
sc24 `divergent(…)` word** (the rig's third lupin-only word; the
F-0048/sc16 precedent of extending the grammar when reality demands a
new sentence):

- **F-0097** (wolf-interp#47): a multi-arm handler over a
  BUILTIN-raised row takes its first arm — F-0052/F-0079's mechanism at
  a third address, measured in both arm orders; the tag ridden out of
  `main` is correct, which is what caught it (the sc14 rule paying out
  again: when two observers disagree, bisect the observers).
- **F-0098** (wolf-interp#48): take-mode reuse is static `fail(E1001)`
  on the compiler rungs and EXECUTES under lupin — to
  `trap(use-after-move)` where the reuse is reached, to the honest row
  path where execution diverts first.

**The `slow` block, re-measured per is20's release promise** (the
ledger's own debt: "an interpreter perf wave is its exit"), and the
answer is MIXED — all 40 rows measured one at a time, idle:

- **12 rows COMPLETE and flip `slow` -> `run`** — the ten chacha20
  Wycheproof part files (0.6-6.3s each, against 29-98s at 0.1.13:
  is20's prune fix arriving, F-0093's exit for exactly the family sc17
  priced), plus `x25519_shared_p13` (15.8s) and
  `x/jose/jws_eddsa_vectors` (16.5s). Twelve lupin rows gained in one
  release wave.
- **25 rows return lupin's own `unsupported: the program did not
  terminate within 50000000 evaluation steps`** and flip `slow` ->
  `unsupported`: the nine sha2 long/monte/hkdf-full sets and fifteen
  curve25519 ladder files plus the rfc7748 chain — the sc23 P-256
  verdict arriving for the block. A step-budget refusal is the
  machine's word where `slow` was this rig's word for a wall-clock
  skip; depth goes DOWN on paper and honesty goes up. The next lupin
  release that raises or retires the evaluation-step budget inherits
  these 25 as its expected flip set.
- **3 rows stay `slow`** (`x25519_shared_p3/p4/p5`): neither an answer
  nor the refusal arrives inside 70s idle — the one word that keeps the
  lane un-invoked, with the caveat in the row comment.

## Retirements and movements at the sc25 pin — the waivers die

**The BINARY pin bumps lupin 0.1.14/90c90df -> 0.1.15 @ a900b8c — THE
SCALAR RELEASE (is26, one sprint behind s121/D58) — and nothing else
moves: the wolf binary stays the sc24 second acquisition @ a900b8c
(the release notes force nothing), the data pin is unchanged, and for
the first time all three pins name ONE wolf-lang sha.** Acquired on
nomad-1 (macOS arm64) by building the release TAG v0.1.15 (= d907302)
per D57 — the binary answers the bare `lupin 0.1.15 … pin a900b8c`,
which is D57 doing exactly what it was ruled to do after the r02 gap.
The native lane is DARK on this host (the staticlib is parked until
s59-apple), so every native column keeps its recorded linux-lane value
and the two-lane posture is stated wherever it matters.

**The untouched-ledger drift, measured before a single sc25 edit: 3
rows deeper, 0 shallower — EXACTLY the flip set the sc24 records
predicted, zero deltas in the std-test run.**
`str/chars_builtin_agree` (the two independent UTF-8 readings — std's
`code_points` against the builtin's `chars()` — compare cross-machine
at last, which is what the file was written for),
`strbuf/push_char`, `unicode/char_code`: lupin `unsupported` -> `run`.
The 3 `LUPIN_TIER_WAIVERS` died in the same motion, their count-pin
test moved 3 -> 0 in the same commit, and the mechanism itself stays
armed (third arming will come; sc13's plumbing, sc14's first emptying,
sc24's re-arming, sc25's second emptying — each on schedule).

**The full std.unicode retype landed, and the review gate held.** The
reviewed final signatures sc24 wrote into the module header are the
module now: `from_code(n: int) -> char ! {none}` (the validated
`n as char` — every trap row of the raw cast is a `none` here),
char-typed predicates that COMPARE against `char` literals and never
compute, `utf8_len` total over `char` (its `{none}` row existed only
because an `int` can hold a non-scalar). Grepped before and after:
`as char` appears at exactly ONE executable site in std/ (`from_code`),
`c as int` at one (`code`) plus the deliberate bridge in
`chars_builtin_agree`, no arithmetic on the scalar anywhere in std/ or
tests/, and no caller of the int-interim surface survives.
API-CONVENTIONS §9's interim clause is rewritten to the ruled state,
with the deliberate non-flip stated: `str.code_points` KEEPS the `int`
currency as std's independent second reading (retyping it onto
`chars()` would make the agreement test a tautology), and the offset
trio's currency was always byte offsets. Re-measured on both executing
lanes at the pins; the native column is carried on sc24's
both-rungs verification of the whole s121 surface, owed its first
macOS re-measure at s59-apple.

**The 4 `divergent(…)` rows are UNHEALED at 0.1.15 — re-measured one
at a time, byte-identical to the sc24 filings, and both upstream
issues commented the day of the measurement** (wolf-interp#47:
`refused_row` still answers the io arm — `handled: -9` then the
correct propagated `error: refused` — and `closed_row` likewise;
wolf-interp#48: `use_after_close` still executes to
`trap(use-after-move)` at `[mem.tier0.move.2]`, `use_after_wait` still
diverts at `start()` to `error: not_found` exit 1). The rows stand
with dated comments; the runner still reads the eventual heal as a
red, which is the design.

**The step-budget blocks, re-measured release-notes-first, and the one
DELTA of the sprint is a rig fact rather than a release fact.** The
v0.1.14..v0.1.15 span contains no perf or budget commit, so the
predicted interpreter motion was zero — and the interpreter's motion
IS zero: all 25 50M-step `unsupported` rows answer the same refusal
sentence at the same budget. But the 4 `slow` rows moved anyway,
because the WALL CLOCK was never the interpreter's: on nomad-1 (arm64)
the same refusal arrives at 19.3-20.4s idle where the sc24 rig got
neither an answer nor the refusal inside 70s (its knife-edge P-256
refusals sat at 48-56s against the 60s ceiling; this rig's sit near
20s). All four flip `slow` -> `unsupported` on the machine's own word
— depth on paper goes down, honesty goes up, the sc24 pattern at a new
address — and the evidence is appended to wolf-interp#41 (closed; the
append is the sc17 practice). F-0093's margin rule survives re-scoped:
knife-edge is a PER-RIG fact, and a loaded run's timeout is load on
any rig.

**No new finding was filed at this pin.** Nothing 0.1.15 does
contradicts a clause this rig observes — the release's one spec
tension (`[gram.lex.char]`'s prose-only `\u` digit cap) is the
counterparty's own CHOICES-register entry, filed from its side. A
sprint that consumes no F-number is itself worth recording: the char
debt economy closed with the waivers, on the release its records
predicted, with the measured list matching the predicted list to the
row.

## Retirements and movements at the sc26 pin — the native lane comes home

**The wolf binary and the data pin move TOGETHER to trunk `c1ca543`
(s59-apple + s122-s127); lupin stays 0.1.15 @ a900b8c — the
one-moving-part rule (0.1.16/is27 exists and is a later sprint's
bump).** Built in ../wolf-lang at the sha (`cargo build --release -p
wolf_driver -p wolf_rt`); `--version` still names no pin (pre-s66), so
the sha is recorded from acquisition and VERIFIED BEHAVIORALLY by the
s125 witness: `wolf run` on an out-of-bounds index prints the TWO-LINE
trap report — `wolf-trap: bounds` then `  at file:line:col` — exit 134,
where a pre-s125 binary prints line one only.

**The deliberate un-parking.** libwolf_rt.a moves from ~/.local/lib
(parked 2026-08-28 at the first nomad-1 gauntlet, when a lit native
lane on a pre-s59 binary redded every native `run` row with an honest
host refusal) to ~/.local/bin beside the binary — un-parked the SAME
DAY, after s59-apple landed in the pin. bootstrap.sh's non-linux park
case is retired in the planning repo (`4e7bf6e`), with the comment
dating the park's one-day life. Doctor: native rung LIT on this host
for the first time.

**The untouched-ledger drift, measured over all 365 rows with THREE
lanes lit on nomad-1 before a single sc26 edit: 1 row deeper, 0
shallower, 0 cross-lane divergences, 0 directive mismatches, 0
timeouts.** The headline is the 364 rows that did NOT move: every
native and wolfc column — linux-lane measurements until today — is
silently confirmed on macOS/arm64, the whole crypto tier included. The
sc19 env-crypto-drift class produced nothing; no platform divergence
exists to file. The one mover is `testing/near_and_ulps` wolfc
`unsupported` -> `run`, and its mechanism is measured against BOTH
binaries (the a900b8c driver rebuilt for exactly this): the old
checked tier refused `unsupported — this literal shape in checked
execution` spanning `order_key`'s `-9223372036854775808` INT_MIN
literal in std/testing; the s123 numeric-literal wave (E0415, width
defaulting) owns the span, and at c1ca543 the row runs to its
directive. Its native column stays `unsupported` on an EVOLVED
sentence (`match over this scrutinee type`, was c08's
interpolation-in-fail) — verdict unchanged, wording noted on the row.

**F-0096 re-probed at c1ca543: UNHEALED.** `s.get(0..^1)` still
answers `unsupported — open-ended or end-relative ranges (slicing)` at
resolve on BOTH rungs — now measured on the macOS rungs' own machinery
rather than carried from linux — while `s[..^1]` runs three-lane in
the same gauntlet. wolf-lang#164 commented with the re-measure the day
it was taken; `str/end_relative_get` stands `run/unsupported/
unsupported` and the register row stays open from our side.

**The 4 `divergent(…)` rows observed exactly as filed** (F-0097
first-arm-over-builtin-rows, F-0098 take-mode reuse) — lupin did not
move this sprint, so they could not, and the runner's designed red at
the eventual heal is undisturbed.

**Trap rows and the s125 second line: tolerance is structural, and it
was verified anyway.** The rig reads conform-run VERDICTS, never raw
stderr, so the site line cannot move a row — and zero mismatches in
the gauntlet is the proof at scale. One ledgered trap row verified
raw: `wolf run` on `str/slice_oob_traps.lu` prints
`wolf-trap: bounds` / `  at <staged path>:13:5`, exit 134.

**s126 is a no-op over this corpus, verified by the letter:** zero
`#![` markers and zero `#!`-initial lines in std/ and tests/ (both
grepped at the bump), so D61's zero-cost-when-absent claim is not
merely believed here — no row can observe the origin marker or the
narrowed shebang.

**The anchors regen measured 393 -> 397 (+5/-1), NOT the predicted
"+2", and the -1 is an upstream defect (F-0100, wolf-lang#170, filed
the day it was measured).** Added: [conf.trap.report] and
[conf.trap.render] (s125), [gram.attr.index] and
[gram.expr.index.origin] (s126), [conf.directive.standalone] (D59, in
the span). Removed: [gram.lex.ident] — NOT a retirement: the heading
is still in 01-grammar.md, but s126's shebang prose added bare literal
`[` runs and spec-extract's bracket scanner pairs `[` with the next
`]`, so the span from `` `#[` `` swallows the anchor's only
occurrence. [gram.lex.shebang]'s amendment is prose-only and invisible
in the id->file registry. No rig test cites the lost anchor, so the
snapshot is re-vendored byte-faithful with the defect named.

**Knife-edge, re-scoped per-rig: the native lane's wall clocks are
measured on this rig for the first time, and NOTHING is near a
ceiling.** All 160 native-`run` x/ rows timed one at a time, idle,
full `conform-run --native` (compile + link + execute per row): the
slowest is 1.40s (`p256/wycheproof_p256_p7`) against the 60s per-test
ceiling — 2.3% of it, a 43x margin — and the whole 160-row pass totals
79s. Zero margin notes are owed on any native row on this rig; the
lupin knife-edge story is unchanged from sc25 (that lane did not
move). The wolfc checked tier's budget-refusal grinds remain the slow
part of a full gauntlet (~50 min for the 365-row three-lane pass on
this rig, idle) — a rig fact recorded for the next operator, not a
ledger word.

**One F-number consumed (F-0100), one issue commented (#164), no row
regressed.** The sprint's measurement thesis held: lighting a third
lane on a new platform moved NOTHING the records did not predict,
except one row whose mechanism was hunted down and named, and one
registry defect the delta measurement itself surfaced.

## Retirements and movements at the sc27 pin — the divergences re-measure

**The contract's either/or resolved at launch: s128 HAS merged, so BOTH
binaries and the data pin move to ONE wolf-lang sha — trunk `0a5c1af`
(s128's merge + the ir-volume ratchet) — and lupin moves to the v0.1.17
TAG in the same bump, carrying TWO releases (0.1.16/is27 + 0.1.17/is28;
0.1.16 was deliberately not chased at sc26).** wolf built in ../wolf-lang
at the sha (`cargo build --release -p wolf_driver -p wolf_rt`), wolf +
libwolf_rt.a installed via fresh inode (the sc26 SIGKILL rule); lupin
built at the tag (= 450ed42), answering the bare
`lupin 0.1.17 (wolf-interp, reference interpreter at pin addcd7f)`, the
wolf-interp tree returned to trunk after the build. Doctor green at the
trio; sync-pin green at the re-vendor.

**The pairing nuance, reported-never-gated (F-0064; the wsm03
precedent):** wolf @ 0a5c1af's `--version` pairs "lupin 0.1.16 …, pin
e561c6f" — ONE release behind the 0.1.17 staged beside it, because the
pairing stamp is s128's file, cut before the 0.1.17 release existed.
Recorded at the bump; the rig reads identity from the first line only.

**The acquisition verified behaviorally by the span's own witness:**
`let c = a + b` on two strs compiles and prints the concatenation on
BOTH rungs (`wolfgang`, exit 0; lupin agreeing) where every pre-s128
binary answers E0409 — cheap, decisive, un-fakeable by a stale binary.

**The two-upstream drift is nonzero again and honest:** data pin
0a5c1af, lupin conformance pin addcd7f — behind by exactly the s128
span. is28 shipped concurrently with s128, and D62/D63 were already the
interpreter's behavior ("nothing to build — this machine's behavior IS
the ruling", its own changelog).

**Anchors regen: 397 -> 402, +5/-0 CLEAN** — +[type.str],
+[type.str.concat], +[type.str.concat.cost], +[type.str.concat.mix]
(D62), +[mem.list.slice] (s128's slice ruling). [gram.lex.ident] is
STILL absent at 0a5c1af: F-0100 / wolf-lang#170 unhealed (the heading
survives in 01-grammar.md; the hole is carried, not new — and
wolf-interp independently hit and filed the same defect as
wolf-lang#177 at its own addcd7f bump, a second witness for the fix).

**The untouched-ledger drift, measured over all 365 rows, three lanes,
idle, before a single sc27 edit: ZERO deeper, ZERO shallower — against
a prediction of ZERO/ZERO written down BEFORE the run** (release notes
and span logs read first, per release; then the grep letter: zero
`str + str` expressions, zero comma binders, zero tuple destructuring,
zero list slices, zero `#![` markers, zero `#!` lines in std/ or
tests/ — the corpus predates every s128/is28 surface). std-test GREEN,
exit 0; 0 unstable; 0 slow skips (`slow` keeps zero carriers since
sc25); the 3 sc24-era char rows (`str/chars_builtin_agree`,
`strbuf/push_char`, `unicode/char_code`) hold `run`; the 25 lupin
50M-step rows answer the same refusal sentence at the same budget — no
perf or budget commit anywhere in v0.1.15..v0.1.17 or s128, so the
step-budget/knife-edge story is one line: zero motion. Every clause of
the prediction is reconciled; no delta exists to explain or file.

**The four `divergent(…)` rows' THIRD measurement — re-measured one at
a time, all UNHEALED, all byte-identical to the sc24 filings**
(predicted: no net/process commit exists in the span):
`net/refused_row` exit(1) `handled: -9` + `error: refused` (the io arm,
listed first, still answers where `refused` should — F-0097);
`net/closed_row` exit(1) `peer-gone: not-firedio-arm` + `error: closed`
(same mechanism — F-0097); `net/use_after_close`
trap(use-after-move) at [mem.tier0.move.2] (F-0098);
`process/use_after_wait` exit(1) `error: not_found` (diverts at
`start()` — F-0098). wolf-interp#47 and #48 commented the day of the
measurement; the rows stand and the runner's designed red at the
eventual heal is undisturbed.

**The #50 char-copy heal is REAL, measured with the issue's own
reproducer, and its boundary is measured too.** Under 0.1.17,
`let c = "x".chars()[0]; var d = 0 as char; d = c; print("{c}{d}")`
prints `xx` exit 0 on ALL THREE lanes — 0.1.15/0.1.16 trapped
use-after-move [mem.tier0.move.2] exit 3 on the lupin lane. A `char`
is a copy value in the tier-0 move discipline now, exactly as `int`
is. The heal is the char scalar ONLY: the sc22 cursor-struct shape
(`Rd { b: bs, pos: 0, end: bs.len }` over a `List`) re-probed UNMOVED
— E1001 on both compiler rungs, trap(use-after-move) [mem.tier0.move.2]
under 0.1.17 — so the Guide's sc22 entry stands as written and the
E1001-class ledger rows (`x/crypto/sha2/use_after_final_trap`, the
net/process pair above) keep their values; no guidance history is
rewritten. wolf-interp#50 commented with the heal measurement
(closable from this side). Incidental but worth a line: lupin's E0302
now TEACHES the D59 escape ("two separate programs sharing a directory
each mark themselves `//! member: false`") — is28's voice observed
live while staging the probe.

**F-0018 residue row 6 (str + str) HEALS at this pin — the residue
economy shrinks by one.** From sc03's filing: "`str + str` runs in
lupin while wolfc says E0409" — the last living adjacent clause. At
0a5c1af, `+` and `+=` and a chain all run three-lane (D62,
[type.str.concat]: interpolation-append, a builtin operator on the
builtin type, NOT an Add bridge); the mixes (`str + int` probed) stay
E0409 by ruling ([type.str.concat.mix]), refused by name under lupin.
NOT adopted into std/ this sprint (the contract's one-moving-part
rule): std keeps `"{a}{b}"`, which is the SAME lowering by
[type.str.concat.cost] — adoption is cosmetic, filed below. The other
residues, re-probed cheaply at the trio:

- **`chars` (the pair): UNMOVED, and the probe is sharper now.**
  `List[(int, int)]` is still `unsupported` at resolve on BOTH wolf
  rungs — "prelude container instantiation (generic data)" — while
  lupin runs the same program (push a tuple, index it, destructure it:
  `12`). s128's tuple work is DESTRUCTURING and tuple PATTERNS; it
  does not instantiate a tuple LIST, so the
  `chars(s) -> List[(int, char)]` fusion stays blocked on exactly the
  sc24 refusal, now dated at a third pin.
- **`strbuf.in(r)`: unmoved** — no placement syntax exists in the spec
  at 0a5c1af (grepped; `[mem.region.promote]` is about unobservable
  placement, not a surface).
- **`reserve(n)`: unmoved** — no capacity/SSO decision anywhere in the
  span; a documented no-op would still be a lie.
- **`graphemes`: unmoved** — segmentation is a later tier by the #18
  ruling; nothing in s128/is28 brings it closer.
- F-0096 (`get(a..^n)`) stands via the gauntlet: `str/end_relative_get`
  observed at its recorded `run/unsupported/unsupported` in the
  untouched-ledger run — [mem.list.slice] rules `cs[a..^b]` for LISTS
  and does not touch `str.get`'s refusal.

**The adoption-candidate list for the next sc contract (measured this
sprint, adopted never this sprint — one moving part):**

- `str +`/`+=` where std spells `"{a}{b}"` today (24 two-hole
  interpolation sites in std/) — cost-equal by [type.str.concat.cost]
  (the same strbuf lowering), so purely a readability adoption;
  `std.strbuf` remains the loop builder and `+=`-in-a-loop stays
  quadratic by the anchor's own words.
- Tuple destructuring where std/tests spell `.0`/`.1` today:
  `decompose`'s six `let parts = decompose(x)` sites in
  std/fmt/decimal, `equal_range`'s four call sites in
  tests/search/bounds_and_equal_range.lu.
- Comma-grouped binders (D63) in test files — fixture setup lines.
- List slices ([mem.list.slice]) in std bodies that hand-roll index
  windows (sort/search) — priced against the anchor's fresh-List copy
  cost before any adoption.
- `chars(s) -> List[(int, char)]` fusion — GATED, not a candidate yet:
  waits on tuple-list instantiation on the wolf rungs (the residue
  above).

**No F-number consumed, no new issue filed, three issues commented
(#47, #48, #50 — the first two never closed by this repo, the third
now closable from upstream's side).** The sprint's thesis held at a
new address: a bump whose release notes predict zero motion, measured
to zero motion over 365 rows with the prediction written first, is the
cheapest gauntlet a rig can run — and the four divergences' third
measurement is now upstream where the heal will read it.

## Retirements and movements at the sc28 pin — the library writes the new words

**The drift prediction, written 2026-08-30 14:13 EDT, BEFORE any
gauntlet at the new pins** (release notes read first, per release: the
wolf v0.2.0 CHANGELOG, the five-commit span 0a5c1af..c88ab64 read
commit by commit, and the lupin v0.1.18 CHANGELOG/is29):

- **Wolf 0a5c1af -> v0.2.0 (= c88ab64, the TAG; data pin follows): ZERO
  row motion.** The span is five commits and none is semantic: ad6f83f
  (the F-0100 extractor fix), 42f632f (pairing stamp -> lupin 0.1.17),
  371dfd6 (D57 +dev stamp; line 1 gains the doctor-parseable pin
  clause), dd94f8c (CHANGELOG), c88ab64 (release). Predicted anchors:
  402 -> 403, +1/-0, the +1 EXACTLY `gram.lex.ident` — the FILED anchor
  RETURNS (F-0100 / wolf-lang#170 / wolf-interp's #177 twin healed by
  ad6f83f) and the sc26-era carried-hole waiver prose dies.
- **lupin 0.1.17 -> v0.1.18 (= 66dc06c, the TAG; conformance pin
  addcd7f unchanged): EXACTLY FOUR rows move, all deeper, zero
  shallower — the four `divergent(…)` rows flip to `run`** (is29 pays
  F-0097/#47 and F-0098/#48; the runner REDS on each because a
  divergent row observing anything but its filed observation is the
  designed signal). Predicted healed observations, per row:
  `net/refused_row` stdout `handled: -1` + `error: refused` exit 1
  (0.1.17 answered `-9`, the io arm); `net/closed_row` `peer-gone:
  handled` + `error: closed` exit 1 (was `not-fired`+io-arm);
  `net/use_after_close` fail(E1001) at the reuse (was
  trap(use-after-move) [mem.tier0.move.2]); `process/use_after_wait`
  fail(E1001) (was `error: not_found` exit 1 — the lane diverted at
  `start()` before the reuse). Each satisfies its directive, so each
  flips to `run` and the `divergent` word retires to ZERO carriers —
  the MECHANISM stays armed per the sc25 waiver precedent (keep the
  plumbing; price any future entry at a named finding).
- **Zero else.** W0317 (is29's D61 lint) needs a `#![index(1)]` scope
  and the corpus carries zero `#![` markers (re-grepped at this pin);
  no perf/budget commit anywhere in v0.1.17..v0.1.18, so the 25
  50M-step rows keep the same refusal at the same budget; the 3 char
  rows hold `run`; `slow` keeps zero carriers; no commit in either
  span touches net/process beyond the two heals themselves.

**The measurement (same day, untouched ledger, three lanes, idle):
SIX movers, all lupin-lane, all paid by is29's two heals — the
prediction named FOUR and under-enumerated one mechanism's footprint
by two rows.** The four predicted flips observed EXACTLY as
pre-verified, each re-measured one at a time from its staged dir
(`lupin conform-run --std-root std <entry> --json`, the sc27 method):
`net/refused_row` exit(1), stdout `handled: -1\nerror: refused` (three
releases said `-9`); `net/closed_row` exit(1), `peer-gone:
handled\nerror: closed`; `net/use_after_close` fail(E1001) at resolve,
span [1137,1140]; `process/use_after_wait` fail(E1001), span
[1178,1180]. Each satisfies its directive; each flipped to `run` on
the runner's designed red; the `divergent(…)` word holds ZERO carriers
for the first time since sc24 and the mechanism stays armed (the sc25
waiver-plumbing precedent — keep the plumbing, price any future entry
at a named finding).

**The two unpredicted movers are the SAME #48 take-mode rung at the
corpus's two unfiled E1001-class addresses — the prediction read the
release notes' four downstream verifications and missed that a static
rung sweeps a corpus, not a list.** Both re-measured one at a time,
both with spans BYTE-IDENTICAL to the compiler's own E1001 on the same
staged entry (is29's span-parity promise, observed here):
- `fs/use_after_close`: lupin `unsupported` -> `run`, DEEPER — the
  static fail(E1001) (span [929,930] = wolfgang's) answers at resolve,
  BEFORE the machine's by-design fs decline ever would, so the lane
  that refused this file for its capability now holds it for its
  discipline. The directive was already `fail(E1001)`.
- `x/crypto/sha2/use_after_final_trap`: lupin rejected fail(E1001)
  span [837,839] (= wolfgang's) where 0.1.14–0.1.17 executed to
  [mem.tier0.move.2]'s trap — and no lupin ledger word can spell a
  static rejection under a trap directive (`fail(…)` is a wolfc word
  by design), so the DIRECTIVE modernized: `run(exit=trap(use-after-
  move))` -> `fail(E1001)`, all three lanes `run` against it, the
  two-rung history kept in the file's header — and the file RENAMED to
  `use_after_final.lu`, because §13's lint-conventions gate enforced
  the name-promise the old directive made (`…_trap.lu` must expect a
  trap) the moment the directive stopped making it: the rig catching
  its own paperwork, working as built. The compilers' answers did not
  move; the directive met them where they stood.

The rest of the prediction reconciled at face value: anchors 402 ->
403, +1/-0, the +1 exactly `gram.lex.ident` (measured by set-diff at
the re-vendor before the run); zero motion anywhere else over the
remaining 359 rows; the 25 50M-step rows answer the same refusal at
the same budget; the 3 char rows hold `run`; `slow` keeps zero
carriers; W0317 sees nothing (zero `#![` markers, re-grepped). The
lesson the miss teaches is filed in the Guide entry: when a release
lands a STATIC rung, grep the corpus for the rung's SHAPE (every
`take`-then-reuse witness), not just the issues it closes — the four
filed rows were the mechanism's carriers, not its extent.

**Both acquisitions, recorded:** wolf at the v0.2.0 TAG (= c88ab64),
provenance verified FROM THE BINARY for the first time — r03's D57
clause prints the bare `wolf 0.2.0 (wolfgang, pin c88ab64)` with no
`+dev`, so the sc24-era behavioral-witness ritual and doctor's
"trusted from acquisition" WARN both retire (doctor now gates the pin
clause; its own commit). lupin at the v0.1.18 TAG (= 66dc06c, built
there, bare `lupin 0.1.18 (wolf-interp, reference interpreter at pin
addcd7f)`, fresh-inode install, tree returned to trunk). Data pin
follows wolf to c88ab64; lupin's conformance pin addcd7f is unchanged
by its own release (wolf-lang had no v0.2.0 tag when is29 shipped —
the pin question re-opens at is30), so the two-upstream drift is the
addcd7f..c88ab64 release span, nothing semantic in it.

## Retirements and movements at the sc30 pin — the slice comes home

**The drift prediction, written 2026-08-31 16:35 EDT, BEFORE any
gauntlet at the new pins** (release notes read first, per release:
wolf-lang's CHANGELOG through the s129 merge `83f83bb` and the s130
merge `b80d239` — both merge commits read, s130 landed first so the
bump takes `b80d239`; lupin's v0.1.19 CHANGELOG/is30 read at the tag
`23dcf62`; the corpus grepped for every new surface before the run):

- **wolf c88ab64 (v0.2.0) -> b80d239 (the s130 merge; data pin
  follows): ZERO wolf-lane row motion.** The span is s129 (struct
  patterns whole-pipe per `[gram.pat.struct]`; the #184 lend-slice
  mem fix — F-0101's counterparty) + s130 (match arms take the
  product domain, c06 retired; checked guard/dotted-tag fixes; a
  release-tier bool-fold ICE fix). Grepped first, the sc28 lesson
  (grep for the RUNG'S SHAPE, not the issues it closes): ZERO struct
  patterns in std/ or tests/ (they predate s129), ZERO match-arm
  guards, ZERO `#![` markers, and the #184 fix finds ZERO carriers
  because `bytes.slice` still spells the sc28 retreat loop — the
  healing is only observable at the RE-ADOPT, which is deliberate,
  in its own commit, after the bump. Match-arm tuple PRODUCTS exist
  at exactly ONE module (std/cmp: three `match (self, other)` sites)
  and every carrying row's wolfc/native verdict is `unsupported` at
  the EARLIER trait/enum/impl tier (the F-0012/F-0026 class), so
  s130's product domain has no reachable carrier here; the
  bool-fold ICE fix is release-tier and no lane here builds release.
- **lupin 0.1.18 -> v0.1.19 (= the TAG 23dcf62; conformance pin
  addcd7f -> 83f83bb, the two-bump is30 story): EXACTLY TWO rows
  move, both lupin, both deeper — `net/bytes_round_trip` and
  `net/write_bytes_invalid_row` flip `unsupported` -> `run`**
  (F-0102's fix: `net_read_bytes`/`net_write_bytes` land as the str
  calls' own shape, whole-or-raise writes, no utf8 row — the fs
  vocabulary this facade adopted verbatim at sc29, so the rows
  should observe at first sight). The ledger's byte-tier block flips
  compiler-lanes-only -> three-lane. In lupin's OWN census the same
  fix moves its two byte-tier corpus rows (`net/byte_roundtrip`,
  `net/line_reader_bytes`) out-of-scope -> match — read from the
  release's tracked verdict table at the tag, cited here because it
  is the same mechanism observed from the other repo's side.
  is30's element-move destructure discipline finds zero carriers:
  sc28's tuple destructures never read a moved-from element (they
  were adopted behavior-neutral under 0.1.18's whole-tuple move, so
  a LOOSER move rule cannot change their answers).
- **`x/tls/client/loopback_handshake` stays native-only with the
  lupin REASON STRING moving**: the byte tier resolving takes away
  the F-0102 can't-resolve refusal, and the step budget is what
  remains — four X25519 ladders + Ed25519 sign/verify + the AEAD
  flight blow lupin's 50M evaluation-step budget exactly as they
  blow wolfc's step budget one rung down. Verdict word unchanged
  (`unsupported`), comment re-attributed; the reason string is to be
  verified from the runner's record and noted.
- **Anchors 403 -> 404, +1/-0, the +1 EXACTLY `gram.pat.struct`**
  (s129's clause; s130 added no clause — its witnesses live under
  anchors that already exist). Set-diffed BOTH WAYS at the
  re-vendor per the F-0100 lesson. (The wave letter said 405; its
  own parenthetical adds one anchor to 403, and the release notes
  and the pre-vendor set-diff both say 404 — the letter's arithmetic,
  not a third clause.)
- **Zero else.** No perf/budget commit in either span: the 25
  50M-step rows keep the same refusal at the same budget; the 3 char
  rows hold `run`; `slow` and `divergent` keep zero carriers; the
  doctor trio is wolf = data pin = b80d239 with lupin's conformance
  pin 83f83bb exactly ONE merge behind (the s130 span, whose
  semantic content is the product match domain — named, not
  papered).

Predicted total: TWO verdict movers (both lupin, both deeper), one
reason-string move with no verdict motion, 404 anchors.

**The measurement (same day, untouched ledger, three lanes, idle):
THREE movers, all lupin, all DEEPER — the prediction named two
exactly and called the third's verdict word wrong in the right
direction.** The gauntlet's designed red named exactly three rows
(`ledger says unsupported, observed run`), each then re-measured one
at a time from its staged dir (`lupin conform-run --std-root std
<entry> --json`, the sc27 method):

- `net/bytes_round_trip` — `run`, exit(0), stdout
  `got: 0 255 128 7 10\nback: 3`: as predicted (F-0102's fix).
- `net/write_bytes_invalid_row` — `run`, exit(1), stdout
  `handled: invalid\nerror: invalid`: as predicted. Both byte-tier
  rows observed at FIRST SIGHT against sc29 bodies untouched — the
  facade was declared against the fix (the F-0049 lesson), so the
  heal is a pure flip. The sc29 byte-tier block is three-lane.
- `x/tls/client/loopback_handshake` [lupin] — **`run`**, exit(0),
  the exact directive stdout, 18.9s idle. UNPREDICTED AS A VERDICT:
  the prediction repeated sc29's "native-only for F-0102 AND the
  step budget", and that second attribution was an INFERENCE that
  had never been measured — at 0.1.18 the byte tier refused at
  RESOLVE, before a single evaluation step ran, so no observation
  of the budget ever existed for this program. 0.1.19 removes the
  resolve refusal and the measurement outvotes the inference: the
  whole handshake — four X25519 ladders, Ed25519 sign/verify, the
  AEAD flight — fits inside lupin's 50M evaluation-step budget. The
  corpus's own rungs had already sorted the classes: the
  single-ladder x25519 rows (`x25519_rfc7748_s52`, `_s61_dh`) have
  run under lupin since sc18, and only the VECTOR SWEEPS (the
  wycheproof parts, the chain) blow 50M — a handshake is a handful
  of ladders, not a sweep, and it lands in the class its siblings
  predicted even though the sprint prose did not. The flagship is
  TWO-LANE now (lupin + native); wolfc's checked tier keeps its
  MEASURED step-budget refusal (`@1924..11690`) and is the one dark
  lane. The lesson is sc28's rung-shape lesson wearing lane
  colors: a refusal that fires EARLY in one release hides every
  refusal behind it, so "and the budget" claims about a
  resolve-refused program are predictions, not measurements —
  write them as such.

The rest reconciled at face value: anchors 403 -> 404, +1/-0, the
+1 exactly `gram.pat.struct`, set-diffed both ways at the re-vendor
(nothing dropped — the F-0100 regen lesson held); zero motion over
the remaining 369 rows; the 25 50M-step rows answer the same refusal
at the same budget; the 3 char rows hold `run`; `slow` and
`divergent` keep zero carriers; doctor green at the trio with the
agreement state stated: wolf's own commit = data pin = b80d239 (the
first bump where the BINARY's pin clause and the data pin agree by
construction on a dev-stamped build), lupin's conformance pin
83f83bb one merge behind — the s130 span, the product match domain,
named. F-0102 CLOSES on the two byte rows' measured heal; lupin's
own census moved its twin rows (`net/byte_roundtrip`,
`net/line_reader_bytes`) out-of-scope -> match at the same fix,
read from the release's tracked verdict table at the tag.

**The residues, re-probed at b80d239, one line each:** the
chars-pairs tuple list (`List[(int, int)]`) is refused at resolve on
both wolf rungs at its FOURTH consecutive pin — same "prelude
container instantiation (generic data)" word, lupin runs it; s129/
s130 are struct patterns and product match arms, neither
instantiates a tuple list (dated in the str header). F-0096
(`s.get(0..^1)`) refuses verbatim — "open-ended or end-relative
ranges (slicing)" at resolve on both rungs against `[mem.str.get]`'s
own sentence, lupin runs; dated in the str header beside the row
that flips at its closure. `graphemes` owes no probe: a segmentation
TABLES tier, and nothing in either span brings it closer.
`strbuf.in(r)` (region placement) and `reserve` (capacity/SSO) are
unmoved: no commit in c88ab64..b80d239 or 0.1.18..0.1.19 touches
regions or capacity. The four `divergent(…)`-era addresses stay
healed (re-observed green in every sc30 gauntlet).

## Retirements and movements at the sc31 pin — the letters, and the arms

**The drift prediction, written 2026-09-01 12:14 EDT, BEFORE any
gauntlet at the new pins** (release notes read first, per release:
wolf-lang's CHANGELOG 0.2.1 entry at the tag `75fd2d0` with the whole
`b80d239..75fd2d0` span read commit by commit; lupin's v0.1.20
CHANGELOG/is31 at the tag `e3736c1`; both upstream trees — `corpus/`
and `spec/` — diffed at the two data-pin shas before the run, and the
repo grepped for every new surface's SHAPE):

- **wolf b80d239 -> 75fd2d0 (the v0.2.1 TAG; data pin follows): ZERO
  wolf-lane row motion.** The span is r04 — four measured letters and
  a release — and not one of them adds a surface. Each grepped for its
  shape, the sc28 lesson:
  - **D66, `defer` runs at SCOPE exit, not as the frames return**
    (#193; `[mem.model.order]` amended, `defer_loop_turn` pins the
    loop-body interleaving upstream). std/ and tests/ hold exactly ONE
    executable `defer` — `std/fs/fs.lu:262`, `defer fs_close(fd) else
    |_| {}` in `append_text` — and ZERO `errdefer`. It is a function-
    body defer with no loop around it, where scope exit and frame
    return are the same instant, so the clarified letter cannot change
    its answer. Zero carriers.
  - **#189, `\u{…}` takes one to six hex digits** (`CHAR_ESC`'s
    `HEX_DIGIT+` amends to `UNI_ESC`; seven is E0101, and leading
    zeros count toward the bound). Every `\u{…}` in the tree is four
    digits or fewer except `'\u{10FFFF}'`/`"\u{10ffff}"` at exactly
    six — the bound's own maximum, no leading zeros anywhere. Nothing
    gains a refusal; the lexer already behaved this way and only the
    prose moved. Zero carriers.
  - **#192, two region diagnostics stop lying** (W1001 now also
    requires no CALL in the region's extent; E1010 reads a region
    block's tail THROUGH the error row). std/ and tests/ contain ZERO
    `region { … }` BLOCKS: the two region-bearing rows
    (`list/freeze_then_read.lu`, `strbuf/region_build_and_freeze.lu`)
    take a region VALUE (`let r = region()`) and `freeze` it, so
    neither diagnostic has a block to judge. Both fixes only REMOVE a
    judgement — a narrowed warning, a lifted refusal — and both rows
    already read `run` on all three lanes, so there is nothing left
    for them to move.
  - **The release-tier bool-fold ICE fix** (a type-blind peephole
    folded `bxor x, x` to an integer constant): no lane in this repo
    builds `--release`. Zero.
  - **The Windows staticlib/dist fix** (#183 — the reason the tag
    exists): release machinery, not a language surface; nomad-1 is
    macOS arm64 and the archive lane is not this repo's.
- **The upstream trees, counted before the run.** `upstream/corpus`
  471 -> 475 files, the +4 EXACTLY r04's own witnesses and nothing
  dropped: `grammar/defer_loop_turn` (D66), `grammar/char_uni_seven_digits`
  (#189), `lints/region_call_allocates` + `memory/region_unit_tail_call`
  (#192). `upstream/spec` is FILE-IDENTICAL between the two shas —
  both letters amend an existing clause's text rather than register a
  clause — so **anchors HOLD at 404**, and `vendor/upstream/anchors.json`
  is already BYTE-IDENTICAL to `upstream/spec/anchors.json` at
  75fd2d0, set-diffed BOTH WAYS (+0/-0) per the F-0100 lesson. The
  re-vendor is a no-op on the snapshot; only `PIN` moves.
- **lupin 0.1.19 -> v0.1.20 (= the TAG `e3736c1`; conformance pin
  83f83bb -> b80d239): ZERO row motion.** is31 is one surface —
  struct patterns in match ARMS (`[gram.pat.struct]` in arm position,
  s130/wolf-lang#179) — plus E0802's reachability walk widening
  column-wise over product arms. Grepped for the SHAPE, not the issue
  it closes: **ZERO struct patterns in arm position** anywhere in
  std/ or tests/. sc30's own 14-pattern adoption is BINDERS only
  (`let RawRecord { rtype, wire } = …`, `let CertVerifyView { … } =
  …`), which is exactly why the shape that landed this release finds
  nothing here. The only product arms in the tree are std/cmp's three
  `match (self, other)` sites (12 tuple arms): the tuple twin already
  agreed at 0.1.19 — lupin has run products all along — every cmp row
  carrying them already reads `run` under lupin and `unsupported` on
  both wolf lanes at the EARLIER trait/impl tier (the F-0012/F-0026
  class), and no cmp arm is covered column-by-column by an earlier one
  (`(Less, _)` follows `(Less, Less)`, `(Greater, _)` follows
  `(Greater, Greater)`; nothing splits a bool column and constrains
  nothing else), so the widened E0802 warns nowhere. is31's
  arm-boundary restatement — an arm takes the WHOLE scrutinee, no
  clause extends partial moves to arms — is the status quo both
  machines already had for the scalar and enum arms this repo writes.
  Zero carriers.
- **is31's 8 arm witnesses enter LUPIN's census, not this one**: 455
  -> 463 files / 430 entries / 33 members, and every one of the 455
  files carried over from 0.1.19 verdict-IDENTICAL, class for class;
  the seven-witness differential table is six agreements plus one
  honest conservatism (`match_arm_product_nonexhaustive`, beside
  `match_missing` — exhaustiveness is the type checker's and E0801 has
  no dynamic half). Read from the release's tracked verdict table at
  the tag and cited because it is the same mechanism seen from the
  other repo's side; this repo has no carrier for it.
- **The two float-cast SOUNDNESS rows are unchanged** (wolf-lang#168's
  family: `corpus/faults/cast_float_nan_trap.lu` and
  `corpus/faults/cast_float_overflow_trap.lu` — the checked lane
  exits 0 where the fault twins demand `trap(overflow)`). wolf-lang's
  PAIRING ritual re-ran against the v0.1.20 release build and records
  **checked 248 agreements / 2 soundness, native 268 / 0** — the same
  two, at a strictly better agreement count than 0.1.19's 242/2 and
  263/0. #168 stays OPEN upstream; nothing in either span touches cast
  semantics; and this repo has no carrier at all — every `as int` in
  std/ and tests/ is from an integer word type (`sha2`'s u32/u64
  packing, `chacha20`'s word32, `handshake`'s `wrapping[u64]` ct
  compare, `unicode`'s `c as int`), and there is not one float->int
  cast in the tree.
- **Zero else.** No perf/budget commit in either span: the
  step-budget refusals keep the same wall at the same budget, the char
  rows hold `run`, `slow` keeps zero carriers, and the four
  `divergent(…)`-era addresses stay healed. The doctor trio reads
  wolf's own pin = data pin = 75fd2d0, with lupin's conformance pin
  b80d239 exactly FOUR COMMITS behind — r04's four letters, named
  above, none of them a lowering change. That is a new shape for this
  gap: every previous bump's lupin lag was lowering work waiting on a
  release, and this one is prose the interpreter did not need.

**Predicted total: ZERO verdict movers over 372x3.** The baseline the
gauntlet must reproduce exactly, counted from the untouched ledger:
lupin 300 `run` / 72 `unsupported`; wolfc 296 `run` / 74
`unsupported` / 2 `fail(E…)`; native 321 `run` / 49 `unsupported` /
2 `fail(E…)`. Anchors 404, +0/-0. Corpus 471 -> 475 upstream.

**The measurement (same day, untouched ledger, three lanes, idle):
ZERO movers — the prediction is exact, and this is the first sc bump
whose drift list came back empty.** The gauntlet ran 12:26:34 ->
12:52:21 EDT over files last touched at 12:15 (the prediction, the
two tool records and PIN — mtimes checked against the run window,
the sc30 rule), `cargo xtask ci` exit code 0, `ci: GREEN`, no piping:

- `std-test: 372 test(s); forward tags: 693; conservatism ledger: 199
  entries; unstable rows: 0; slow skips: 0; divergent rows: 0.` Every
  row observed exactly the verdict the untouched ledger claims —
  a deeper answer is a designed RED here, so a green run over an
  unedited ledger IS the zero-motion measurement. The baseline holds
  to the number: lupin 300 `run` / 72 `unsupported`, wolfc 296 / 74 /
  2 `fail(E…)`, native 321 / 49 / 2. `x/tls/client/loopback_handshake`
  observed TWO lanes again (lupin + native, wolfc's measured step
  budget the one dark lane) — sc30's unpredicted flip is stable at the
  new pins, not a one-run accident.
- Anchors 404, +0/-0, `vendor/upstream/anchors.json` byte-identical to
  `upstream/spec/anchors.json` at 75fd2d0 — set-diffed both ways
  BEFORE the re-vendor and confirmed by `sync-pin`'s own comparison in
  the green run. Nothing was regenerated, so the F-0100 hole had
  nothing to fall through; the check was run anyway, because the
  lesson is about the check, not the regeneration.
- The upstream census reconciled at face value: corpus 471 -> 475,
  the +4 exactly r04's four witnesses; spec tree file-identical;
  lupin's own census 455 -> 463 with the 455 carried-over files
  verdict-identical; wolf-lang's PAIRING at the v0.1.20 build reads
  checked 248/2-soundness and native 268/0, so #168's two float-cast
  rows are where they were.
- Doctor green at the trio, each edge stated:
  `lupin 0.1.20 … pin b80d239` matches `vendor/tools.toml`;
  `wolf 0.2.1 (wolfgang, pin 75fd2d0)` matches, BARE — the release
  shape, the sc30 dev stamp retired by a real tag — with the pairing
  line reported and not gated; the native rung's `libwolf_rt.a`
  present, lane lit. wolf's own pin = the data pin = 75fd2d0; lupin's
  conformance pin b80d239 is four commits behind, and those four
  commits are r04's letters.

**What the empty drift list is worth saying about.** Three of the four
letters in this span are the kind of change that CANNOT move a
downstream row by construction — a clarified `defer` timing that both
machines already implemented, a lexer bound the lexer already
enforced, and two diagnostics that only ever REMOVE a judgement. The
fourth (is31's arms) is a real new surface with a real new capability,
and it moved nothing here for the reason the sc30 adoption chose: this
repo destructures in BINDERS, and the release that landed the same
pattern in ARM position finds no carrier. A zero is only worth
recording when it was predicted for stated reasons and each reason was
checked separately; that is what the four bullets above are.

**The residues, re-probed at 75fd2d0, one line each:** the chars-pairs
tuple list (`List[(int, int)]`) is refused at resolve on both wolf
rungs at its **FIFTH** consecutive pin — same "this prelude container
instantiation (generic data)" word, `@455..473`, lupin runs it; r04 is
four measured letters and container instantiation is none of them
(dated in the str header). F-0096 (`s.get(0..^2)`) refuses verbatim —
"open-ended or end-relative ranges (slicing)" at resolve on both
rungs, `@493..498`, against `[mem.str.get]`'s own sentence, lupin runs;
dated in the str header beside the row that flips at its closure.
`graphemes` owes no probe: a segmentation TABLES tier, and nothing in
either span brings it closer. `strbuf.in(r)` is unmoved BY NAME — r04
touches regions only through #192's two DIAGNOSTICS, which judge
region blocks and change no placement plumbing — and `reserve` is
unmoved for the same reason plus the absence of any capacity or
SSO commit in `b80d239..75fd2d0` or `0.1.19..0.1.20`. The four
`divergent(…)`-era addresses stay healed (re-observed green in the
sc31 gauntlets). **And one deliberate non-consumption, noted so the
next lane does not re-discover the question:** the region ACCOUNTING
surface wolf-lang#187 asks for (a charged-bytes query plus a
creation-time cap that faults as a catchable row — lobo's
`memory_budget` customer) is s131's, landing MID-WAVE and therefore
after this bump's pin. It is not consumed at sc31 and is not a residue
this sprint failed to move; it is the NEXT sc bump's first
measurement, and `strbuf`'s header says so at the `reserve` line where
a reader will look for it.

## F-0103 — the checked tier and a row in argument position (sc31)

Isolated 2026-09-01 while designing `std.x.tls.client`'s naming pair
for wolf-std#3, filed the same day as
[wolf-lang#201](https://github.com/wolffe-lang/wolf-lang/issues/201).
Four probes, each its own directory (D32: two probe files in one
scratch dir are ONE module — the sc30 lesson), all at
`wolf 0.2.1 (pin 75fd2d0)` / `lupin 0.1.20 (pin b80d239)`:

| probe | shape | lupin | wolfc `--checked` | wolf `--native` |
|---|---|---|---|---|
| p1 | `row_name(narrow(1))` — a generic helper with a WIDE row union parameter, argument's row a narrower SUBSET | `exit(0)` | **`unsupported` @`mem`** | `exit(0)` |
| p2 | the same via a payload re-raise, `named[T](v: T ! {…}) -> T ! {Row(str)}` | `exit(0)` | **`unsupported` @`mem`** | `exit(0)` |
| p3 | `let r = narrow(1)` then `row_name(r)`; and `named(r)` with `else \|Row(name)\|` | `exit(0)` | `exit(0)` | `exit(0)` |
| p4 | `take_int(plain(1))`, `take_int(narrow(9) else 0)`, `take_int(narrow(9)?)` | `exit(0)` | `exit(0)` | `exit(0)` |

The refusal string is `control flow in an argument`, at phase `mem`.
Three things the table settles that prose would have guessed at:

1. **Width subtyping in ARGUMENT position works** — p1's helper takes
   a twenty-one-tag union and p1/p3 pass values whose declared rows are
   four-tag and two-tag subsets, on every lane that gets past `mem`.
   That is what makes one naming helper serve a whole module's
   vocabulary (and `cert`'s nine rows, which this module re-exports)
   instead of one helper per raising function.
2. **A payload-carrying row round-trips across the boundary** — p2/p3
   raise `Row(str)` from a GENERIC function and destructure it in
   `else |Row(name)|`, which `tests/errors/coarsen_and_chain.lu` had
   only ever proven for a monomorphic raiser with an imported payload
   struct.
3. **The refusal is the UNHANDLED union riding in, and nothing else.**
   Handled (`else`), propagated (`?`) and plain arguments all compile
   on the checked tier, and so does the BOUND form of the very
   expression p1 refuses. A limit that a `let` dissolves is a lowering
   gap, not a rule — which is why the filing points at `mem`'s
   argument handling and cites the native rung as the existence proof.

The downstream cost is one line of call-site shape, recorded rather
than hidden: `std.x.tls.client`'s header states **bind, then name**,
the new witness and the adopted battery sites all bind first, and
every naming row stays three-lane. The finding's real value is
retroactive — `option/or_else_default.lu`, `option/exists_marking.lu`
and `option/is_none_marking.lu` have carried `wolfc = "unsupported"`
since sc06 against lupin and native `run`, and until today nothing in
this repo said WHY. Three ledger rows just stopped being unexplained.

## The naming surface at sc31 (wolf-std#3) — the row gets a name

The first consumer's report (wolf-std#3, lobo's wsm04 doors) named two
costs and asked for one of three shapes. The landed answer takes the
issue's middle option and gives it API-CONVENTIONS §12's own form:
**coarsening is a named call the caller writes**, so
`std.x.tls.client` grows `named` (the value rides through; a refusal
comes back as the single payload tag `Row(str)` carrying the row's own
NAME) and `row_name` (the marking face — the word, or `"ok"`), both
thin over ONE match, exactly as `std.option`'s six are thin over
`else`. Both costs the consumer measured die at once: the twenty-arm
match becomes one arm, and the DEAD `Client` those arms had to forge
is never needed, because the success path returns the real value and
the failure path returns a row.

What the issue's other two options cost, stated because they were
weighed and not taken. Splitting `complete`'s union into four routing
classes with the fine row in a `str` detail (option a) is a BREAKING
change to a surface with a live consumer, and it decides for every
future caller that four classes are the right routing granularity —
a coarsening baked into the signature is exactly what §12 forbids
doing silently. Row-tag stringification (option c) is a wolf-lang
D-question, correctly identified as such in the issue, and it would
make this helper unnecessary rather than wrong; when it lands, `named`
becomes a one-line body and its call sites do not move.

The §12 reading is recorded in the API-CONVENTIONS review record
rather than assumed: "a payload is DATA, never a rendered string"
forbids a SENTENCE — this library's wording, an unrecoverable
position — and a row tag's own name is neither. The alternative, an
`int` kind from a documented table plus a `describe`-shaped renderer
(`std.errors`' exact pattern), was declined for two reasons: it would
invent a numbering the protocol does not have, and it would route
every consumer through a second call to get the word it already
wanted. The names are declared stable in the doc comment, so a rename
is a breaking change spelled as one.

**Behavior-neutral, proven by the hash.** The three in-repo
match-to-name sites in `tests/x/tls/client/negative_battery.lu`
(`negotiate_name`'s four-arm negotiation match, `chain_name`'s NINE
chain arms, `bad_cv_name`'s two) adopt the pair, and the battery's
stdout is **byte-identical on all three lanes before and after**:
`f97c6d53b784c51c1a0764548d9022ef65492d57802d2b0a6b9f9dd804be9a97`,
`exit(0)`, lupin + wolfc + native. Fifteen hand-written arms became
five bound calls and the observable behaviour did not move. The fourth
match in that file — `tampered_record_name`'s — deliberately STAYS,
with a comment saying why: `record.open` raises the RECORD layer's own
vocabulary (`tag`, `short`, `unexpected`) and the arms there are a
TRANSLATION into the client's names, not a transcription of them.
`client.row_name` would answer the wrong words, so that match is the
work rather than boilerplate, and an adoption that swallowed it would
have been wrong in a way the ledger could not see.

## Retirements and movements at the sc32 pin — the budget arrives

**The drift prediction, written 2026-09-02 03:27 EDT, BEFORE any
gauntlet at the new pins** (release notes read first, per release:
wolf-lang's CHANGELOG 0.2.2 entry at the tag `8cda3aa` with the whole
`75fd2d0..8cda3aa` span read commit by commit — 35 commits, 9
first-parent; lupin's v0.1.21 and v0.1.22 CHANGELOG entries and the
is32/is33 records at the tag `753d686`; both upstream trees — `corpus/`
and `spec/` — counted and diffed at the two data-pin shas before the
run, and the repo grepped for every new surface's SHAPE):

- **wolf 75fd2d0 -> 8cda3aa (the v0.2.2 TAG; data pin follows): ZERO
  wolf-lane row motion over the 373 rows that exist today.** The span
  is s60a + s133 + s131/s132's merges + four letters + the release.
  Each grepped for its shape, the sc28 lesson:
  - **s131 + s132, the region ledger and the cap** (`region_bytes(r)`,
    `live_region_bytes()`, `region r(cap: n)` / `region(cap: n)`,
    `trap(alloc-contract)` at the allocating site, `fault(kind)` at the
    proc join with `is_fault()`/`is_alloc_contract()`). This is a real
    new capability with real new surface — and it has ZERO carriers in
    the tree as it stands: `grep` finds no `region_bytes`, no
    `live_region_bytes`, no `cap:`, and no `spawn`/`proc`/`select`
    anywhere in std/ or tests/ (the tree's only "process" is
    `std.process`, the child-process trio, an unrelated word). The two
    region-bearing rows (`list/freeze_then_read.lu`,
    `strbuf/region_build_and_freeze.lu`) take a region VALUE and
    `freeze` it; neither reads a ledger nor sets a budget. **This
    sprint ADDS carriers deliberately — new rows are the sprint's
    work, not the bump's drift**, and they are counted separately
    below.
  - **D69, the comma insists everywhere** (struct LITERAL fields
    including the newline-separated spelling, closure parameters
    `fn(a b)`, inline-C capture lists `unsafe c [a b]` — all E0201,
    machine-applicable). Static rejections: a carrier would red the
    parse phase on every wolfc lane. Swept by SHAPE over all 420 `.lu`
    files in std/ + tests/: zero `unsafe c` capture lists anywhere,
    zero `fn(` closure headers with an unseparated parameter pair, and
    a line-pair scan for `name: expr` struct-literal fields with no
    trailing separator followed by another field flags **zero** files.
    Upstream's own blast-radius measurement counted this repo at 887
    files with the sole flagged file in the world being wolf-lang's
    fuzz-minimized formatter fixture. Zero carriers.
  - **#206, a bare entry name means `.`** — `conform-run hello.lu`
    used to answer "the package root has no wolf source files" because
    `Path::parent()` on a bare relative name is the EMPTY path, and
    the anchoring now lives in the loader. Two reasons this cannot
    move a row here, both checked in the rig's source rather than
    assumed: the runner never passes a bare name — `stage::stage_test`
    copies the entry to `scratch.join(file_name)` and
    `runner` passes that ABSOLUTE path (`cmd.arg(staged.entry)`), so
    the empty-parent case was never reachable; and the record's
    `file` field, whose spelling the fix changes, is one the rig
    checks for PRESENCE only (`record::parse`'s required-field list)
    and never compares between implementations — the divergence
    report's `file` is the rig's own `format!("tests/{test}")`
    string. Zero carriers.
  - **#198, `STR_PART` derives escapes** — `STR_ESC`/`UNI_ESC` land in
    the productions so the one-to-six-digit `\u{…}` bound is read off
    the grammar for `"…"` as well as `'…'`. The BOUND did not move
    (v0.2.1 already had it); only the derivation did. Every `\u{…}` in
    std/ + tests/ is six hex digits or fewer with no leading zeros —
    the maximum in the tree is `'\u{10FFFF}'` / `"\u{10ffff}"` at
    exactly six, the bound's own top. Zero carriers.
  - **#209, a trap runs no defers, at the ROOT too** — the letter that
    carries a MEASURED DIVERGENCE at this very pin: every wolfc lane
    abandons a pending root defer, **lupin 0.1.22 runs it**
    (`faults/trap_skips_root_defers.lu`; is34's flip). Carrier check
    done by shape, because this is the one letter in the span that
    could bite: the tree holds exactly **ONE** executable `defer` —
    `std/fs/fs.lu:262`, `defer fs_close(fd) else |_| {}` in
    `append_text` — and **ZERO** `errdefer`, unchanged since sc31. The
    four rows that reach that defer (`fs/append_is_an_append.lu`,
    `fs/text_round_trip.lu` and their siblings) propagate with `?` and
    never trap; and not one of the tree's ~24 trap rows (cmp, hex,
    bytes, strbuf, fmt, math, search, testing) has a `defer` anywhere
    in the frame that traps. So there is **no std row that traps at
    root with a pending defer**, and the divergence has nothing to
    ride. Recorded anyway, in writing, because it is live at this pin
    and the next lane to write a `defer` beside a trap will meet it.
    (Second-order note from the release's own text: on a trapping
    program the interpreter's record carries no stdout, so even a
    carrier would compare verdict-identical — the differ could not
    see this divergence, which is why it took a spec letter to find.)
  - **#205, two nondeterministic verdicts retired** — wolf-lang's own
    `cargo test` net refusal probes dialing an ephemeral port. Test
    infrastructure in the upstream repo; not a language surface, not
    consumed here. Zero.
  - **s60a, the Windows native bring-up** — nomad-1 is macOS/arm64.
    The 21 by-name refusals (task layer, `os.signal`, `net` deadlines)
    are windows-only rows on a windows-only floor line
    (259/255/0/274/0); macOS's floors moved by measurement, not by
    refusal. This repo's three lanes are unaffected. Zero.
  - **s133, the LSP navigation trio** — `wolf lsp`'s
    definition/references/rename over the new binding table. This repo
    runs `conform-run` only and links no `wolf_query`. Zero.
  - **The release tier** — no lane in this repo builds `--release`.
    Zero.
- **lupin 0.1.20 (`e3736c1`) -> v0.1.22 (= the TAG `753d686`);
  conformance pin b80d239 -> 2bfbe5e: ZERO row motion.** Two releases
  carried in one bump (0.1.21/is32 was deliberately not chased
  mid-wave), read separately:
  - **is32 / 0.1.21 — the ledger in the mirror.** `region_bytes` /
    `live_region_bytes` land as two pure prelude builtins over state
    this machine has held since is02, with lupin's own geometry
    written down (a 16-byte grain, a 32-byte allocation header, 16
    bytes a value slot, a `str`'s UTF-8 length, container capacity in
    powers of two from four, and a growth realloc charging the WHOLE
    new buffer while the abandoned one stays charged). Additive; zero
    carriers, same grep as the wolf half. #53's `--explore --json`
    doors add `seed`/`schedule` as `x-` EXTENSION keys emitted only
    under `--seed=`/`--schedule=`, which this rig never passes, and
    `record::parse` validates a required-field SET rather than
    rejecting extras. Zero. The one REAL behaviour change in is32 is
    the lexer: `\u{…}`'s one-to-six-digit bound now binds in STRING
    literals at **E0101**, where lupin previously had no bound at all
    and quietly decoded `"\u{0000041}"` to `A` — and where this
    machine used to file E0110. Grepped both halves: no `\u{…}` in the
    tree exceeds six digits (above), and no std row or ledger entry
    pins **E0110** anywhere. Zero carriers.
  - **is33 / 0.1.22 — the mirror holds.** The cap twin at pin
    `2bfbe5e`: `[mem.region.cap.1-3]` and `[conc.proc.exit]`'s
    `fault(kind)` implemented, so s132's three witnesses flip
    resolve-refusal -> run in LUPIN's census, not in this one. Plus
    the dist lane (`lupin.exe`) and a CI lex-rung fix. Additive; zero
    carriers today, and the sprint's new rows measure it rather than
    inherit it.
- **The upstream trees, counted before the run.** `upstream/corpus`
  475 -> 490 files, the **+15 exactly the span's own witnesses and
  nothing dropped**: s131's two ledger relations
  (`memory/region_bytes_query`, `memory/region_bytes_value`), s132's
  three cap witnesses (`faults/region_cap_breach`,
  `memory/region_cap_boundary`, `conc/proc_cap_fault_join`), #196's
  two or-pattern divergence pins (`grammar/match_arm_or_over_product`,
  `grammar/match_arm_or_inside_product`), D67/D69's five comma
  refusals (`grammar/struct_pattern_no_separator`,
  `grammar/struct_pattern_rest_bare`,
  `grammar/tuple_pattern_no_separator`,
  `grammar/struct_literal_no_separator`,
  `grammar/closure_params_no_separator`), #198's two string-escape
  witnesses (`grammar/str_uni_seven_digits`,
  `strings/str_uni_leading_zeros`), and #209's
  `faults/trap_skips_root_defers`. `upstream/spec` **moves this time**:
  anchors **404 -> 411**, `+7 / -0`, and the seven are exactly
  `mem.region.account`, `.account.1`, `.account.2`, `mem.region.cap`,
  `.cap.1`, `.cap.2`, `.cap.3` — set-diffed BOTH WAYS per the F-0100
  lesson before the re-vendor. The re-vendor is a REAL snapshot move,
  the first since sc27, so the check that was a formality at sc31 is
  load-bearing here.
- **The pin gap, named.** wolf's own pin = the data pin = `8cda3aa`;
  lupin's conformance pin `2bfbe5e` is **35 commits behind** it — the
  largest gap this repo has recorded — and every one of those commits
  is named above: s60a's windows bring-up (a platform this machine is
  not), s133's LSP navigation (a binary this repo does not run), and
  the four letters (#206, #198, #209, #205). `2bfbe5e` is the s132
  merge, which is to say **lupin is pinned at exactly the commit that
  landed the surface this sprint consumes** — the gap is a windows
  port and an editor server, not lowering debt. State which kind of
  gap you have (the sc31 rule); this one is "35 commits, none of them
  a language lowering either machine owes the other".

**Predicted total: ZERO verdict movers over 373x3.** The baseline the
gauntlet must reproduce exactly, counted from the untouched ledger at
trunk `a62a5d4`: lupin 301 `run` / 72 `unsupported`; wolfc 297 `run` /
74 `unsupported` / 2 `fail(E…)` (E1013, E0301); native 322 `run` / 49
`unsupported` / 2 `fail(E…)`. Anchors 404 -> **411**, +7/-0. Corpus
475 -> 490 upstream. (sc31's register recorded 372x3 — that gauntlet
ran before `x/tls/client/row_naming.lu` landed three-lane `run` at the
end of the same sprint; the +1 to each `run` column is that file and
nothing else.)

**The measurement (same day, untouched ledger, three lanes, idle):
ZERO movers — the prediction is exact, and this is the second
consecutive sc bump whose drift list came back empty, over the largest
upstream span this repo has ever crossed in one go.** The gauntlet ran
03:30:02 -> 03:50:49 EDT over files last touched at 03:28:34
(the prediction) and 03:29:36 (the two tool records, PIN and the
anchors re-vendor) — mtimes checked against the run window, the sc30
rule — with `tests/ledger.toml` untouched at 03:13:51 (the worktree's
own checkout). `cargo xtask ci` exit code **0**, `ci: GREEN`, no
piping:

- `std-test: 373 test(s); forward tags: 694; conservatism ledger: 199
  entries; unstable rows: 0; slow skips: 0; divergent rows: 0.` Every
  row observed exactly the verdict the untouched ledger claims — a
  deeper answer is a designed RED here, so a green run over an unedited
  ledger IS the zero-motion measurement. The baseline holds to the
  number: lupin 301 `run` / 72 `unsupported`; wolfc 297 `run` / 74
  `unsupported` / 2 `fail(E…)` (E1013, E0301); native 322 `run` / 49
  `unsupported` / 2. `ledger-check: 373 test(s), all ledgered` and
  `lint-conventions: 373 test(s), all conforming (5 rules)`;
  `doc-examples: 412 block(s), GREEN`; `ulp: 200 reference row(s),
  GREEN` with the standing 16-value libm note unchanged.
- Anchors **411**, +7/-0, `vendor/upstream/anchors.json` byte-identical
  to `upstream/spec/anchors.json` at 8cda3aa — set-diffed both ways
  BEFORE the re-vendor and confirmed by `sync-pin`'s own comparison in
  the green run. This is the first re-vendor since sc27 that actually
  moves bytes, which is exactly the case F-0100 was filed about: the
  key sets were compared in BOTH directions rather than the count, and
  the seven added keys are the seven the spec's own diff shows.
- The upstream census reconciled at face value: corpus 475 -> 490, the
  +15 exactly the span's own witnesses, nothing dropped; the spec tree
  moves only in `02-memory-model.md` (the account and cap clauses),
  `03-concurrency.md` (`[conc.proc.exit]`'s `fault(kind)`),
  `01-grammar.md`/`grammar.ebnf` (the `region_cap` production and D69's
  three separator tightenings) and `05-conformance.md`.
- Doctor green at the trio, each edge stated:
  `lupin 0.1.22 (wolf-interp, reference interpreter at pin 2bfbe5e)`
  matches `vendor/tools.toml`, BARE (a real tag build; the release's
  own D57 assertion refuses a `+dev.` binary before it can ship);
  `wolf 0.2.2 (wolfgang, pin 8cda3aa)` matches, BARE — the second
  consecutive release-tag acquisition — with the pairing line
  (`paired with lupin 0.1.22 (reference interpreter), pin 2bfbe5e`)
  reported and not gated, and for once naming EXACTLY the interpreter
  installed beside it (r05's pairing commit was cut after the 0.1.22
  release existed, so the sc27 one-release-behind nuance does not
  recur). The native rung's `libwolf_rt.a` is present, lane lit.
  wolf's own pin = the data pin = 8cda3aa; lupin's conformance pin
  2bfbe5e is 35 commits behind, and those 35 commits are a windows
  port, an editor server and four letters.

**What the empty drift list is worth saying about, and it is a
different thing from sc31's.** sc31's zero came from a span with no new
capability in it: three letters that could not move a row by
construction and a fourth whose surface this repo does not write. This
span is the opposite — it carries the biggest new memory surface since
regions themselves (a byte ledger, a creation-time cap, a contained
fault at the proc boundary) — and the zero is real for a reason that
only sounds like the same sentence: **a capability with no carrier
moves nothing, and this repo had no carrier because it had never asked
the question.** That is not a stable state; it is a sprint's worth of
work sitting in front of the lane, which is what sc32 spends. The
useful rule is the one the two bumps share from opposite directions: a
predicted zero is only a measurement when each surface's reason is
checked separately, and "additive, and nobody here calls it yet" is a
reason you must confirm with a grep rather than assume from the word
"additive".

## The budget surface at sc32 — `std.mem.budget`, and what the probes refused

wolf-lang s131 + s132 landed the whole region-budget pipe at this
sprint's pin (the query `region_bytes(r)` / `live_region_bytes()`, the
creation-time cap `region r(cap: n)`, `trap(alloc-contract)` at the
allocating site, and D68's containment — `fault(kind)` at a proc's
join). sc31's register ended by naming this as "the NEXT sc bump's
first measurement". This is that measurement, and it is written before
the module because the module is what fell out of it.

**Fifteen probes, each in its own directory** (D32: two probe files in
one scratch dir are ONE module — the sc30 lesson), all at
`wolf 0.2.2 (wolfgang, pin 8cda3aa)` / `lupin 0.1.22 (pin 2bfbe5e)`:

| probe | shape | lupin | wolfc `--checked` | wolf `--native` |
|---|---|---|---|---|
| p1 | `charged(r: region) -> int` across a module boundary, called TWICE on one region | `exit(0)` | `exit(0)` | `exit(0)` |
| p2 | the same over the bare builtin, no module (control) | `exit(0)` | `exit(0)` | `exit(0)` |
| p3 | `region name(cap: n) { … }` SUGAR inside a std body, `n` an int parameter | `exit(0)` | `exit(0)` | `exit(0)` |
| p4 | `capped(n) -> region` — a std function RETURNING a region value | `exit(0)` | `exit(0)` | **`unsupported` — first-class region values beyond local bindings (c05)** |
| p5 | the corpus join shape at root (`spawn proc` + `monitor` + `select` + `is_alloc_contract`) | `exit(0)` | **`unsupported` — structured concurrency in checked execution (C1 deferred)** | `exit(0)` |
| p7 | a `fn` VALUE parameter called inside a capped region, NO proc | `exit(0)` | `exit(0)` | `exit(0)` |
| p8 | `spawn proc` over a std-module function, predicates read in a std body | `exit(0)` | **`unsupported` — C1** | `exit(0)` |
| p6 | the containment as a std helper, breach back as a ROW | `exit(0)` | **`unsupported` — C1** | `exit(0)` |
| p9 | the same, returning the work's value through a `channel[int]` | `exit(0)` | **`unsupported` — methods on generic std data (the std surface)** | **same refusal** |
| p10 | a `channel[int]` created and consumed entirely INSIDE a std body | `exit(0)` | **`unsupported` — methods without resolvable bodies** | `exit(0)` |
| p11 | the channel as the CALLER's, passed into a std signature | `exit(0)` | **`unsupported` — methods on generic std data** | **same refusal** |
| p12 | the containment with NO value crossing back | `exit(0)` | **`unsupported` — C1** | `exit(0)` |
| p13 | the landed shape: `with_cap(n, f: fn())` + `charged` + `live` | `exit(0)` | **`unsupported` — C1** | `exit(0)` |
| p14 | the QUERY half alone (`charged` + `live`, no proc) | `exit(0)` | `exit(0)` | `exit(0)` |
| p15 | a `with_cap` call behind a RUNTIME-FALSE branch | `exit(0)` | `exit(0)` | `exit(0)` |

Five things the table settles that prose would have guessed at, and
each of them changed the shipped surface:

1. **A `region` in ARGUMENT position is read, not consumed** (p1/p14).
   Region values are affine — `[mem.region.create.2]` says they move
   and are never copied — so the reasonable expectation is that
   `charged(r)` eats its argument and the query is useless as a std
   function. It does not, on any lane: two calls in a row answer the
   same number and the region is still usable after both. The licence
   is `[mem.tier0.mode.read]`, whose sentence is exactly this ("the
   callee reads a value that is immutable for the whole call; the
   caller retains it") and which nobody had ever tested against a
   region. That one probe is the difference between a `charged(r)` std
   can ship and a `charged(r)` it cannot.
2. **A region may be TAKEN but not RETURNED** (p4). The native rung
   refuses `-> region` by name — "first-class region values beyond
   local bindings (c05)" — where lupin and the checked rung run it. So
   there is no `capped(n) -> region` constructor in the module even
   though it is two lines: the value form `region(cap: n)` stays the
   language's, at the caller's own site, where it is three lanes.
3. **THE VALUE CANNOT COME BACK, and it takes three measurements to
   close the door** (p9/p10/p11). The obvious signature is
   `with_cap[T](n, f) -> T ! {exhausted}`. A proc is a failure domain
   with no shared address space (`[conc.proc.1]`), so the only licensed
   way out is a channel; a `channel[int]` in a std SIGNATURE is refused
   on **both** wolf rungs (the F-0026 monomorphization family,
   reported at the caller's instantiation site); and a channel kept
   inside a std BODY is refused on the checked rung. The helper
   therefore answers a question rather than producing a value — which
   costs less than it reads, because `[conc.proc.kill]` bulk-frees the
   proc's regions before the reason delivers, so nothing the work
   ALLOCATED could have survived the call under any signature. Only a
   scalar was ever crossing, and lobo's request path already carries
   its region results out as scalars by hand for the same reason.
4. **The C1 refusal is reached at EXECUTION, not statically** (p15,
   against p6/p8/p12/p13). A `with_cap` call behind a runtime-false
   branch compiles and runs to `exit(0)` on the checked rung. That is
   why this module ships three witnesses with two different checked
   columns from one function: `negative_cap_trap.lu` is `run` on all
   three lanes because its guard traps before the `spawn proc` is
   reached, and `breach_is_a_row.lu` is `unsupported` because it
   reaches it. A caller may link this module on the checked tier and
   pay only for the paths that actually spawn — worth knowing, and not
   a thing anyone would have predicted from "the checked tier defers
   the task layer".
5. **The trap-shaped runner is not std's to ship** (p3/p7). Running a
   fn value inside a capped region, with the breach left as the
   process-ending trap, is three lanes — and it is also
   `region r(cap: n) { … }` with a library in the way. What a caller
   cannot write in one line is the CONTAINED form, so that is the only
   runner in the module.

**What landed.** `std.mem.budget`, three functions: `charged(r)` and
`live()` naming the two queries (three lanes), and
`with_cap(n, f: fn()) -> () ! {exhausted}` collapsing the spawn /
monitor / `select` / `is_alloc_contract()` join into one call whose
failure is an ordinary row (lupin + native). The row is the payload-free
mark `exhausted` per API-CONVENTIONS §12, and the reason it carries no
number is not modesty — `[mem.region.cap.3]`'s free-then-deliver
teardown makes the dead proc's charge **unobservable by contract**, so
a payload would have to be invented. A negative budget traps `assert`
at the door rather than riding out as a row, because
`[mem.region.cap.2]`'s own `trap(alloc-contract)` would fire inside the
proc, be contained, and answer a caller's arithmetic mistake with a
recoverable value — the exact failure §2's trap rule exists to prevent.

**The witnesses, and the one test-design decision that mattered.** The
ledger's numbers are per-tier measured facts and NOT comparison surface
(`[mem.region.account.1]`), and this repo's rig compares a `stdout=`
hash across lanes — so a witness that printed a count would be a
per-lane file. Measured, to make the point concrete: the same
100-element `List[int]` reads **4064** under lupin, **2032** native and
**1600** on the checked machine. Every row therefore prints RELATIONS —
zero at creation, monotone, stable between allocations, the live total
rising and returning wholesale, birth-region attribution, the breach
contained, the memory already back — and one hash covers every lane the
row runs on (`ledger_relations.lu`
`2a60aeaecaed6c6bbc2b5d957fe06236be0e95748179ba8629acd3dc8b5de1ff`
three lanes; `breach_is_a_row.lu`
`05c2dc7adf52121ef88612a33e8ecd5c7cc8adc45c3774e50633309507419e37`
lupin + native).

One shape cost a red before it was written down, and it is worth the
sentence: growing a root-born `List` INSIDE a `region scratch { … }`
block is **E1010** on both wolf rungs ("`root_born` still holds a value
allocated in region `scratch` when the region is freed"), where lupin's
birth-region attribution runs it happily. The attribution half of the
witness is therefore written the way wolf-lang's own
`memory/region_bytes_value.lu` writes it — the push sits AFTER the
`in r { … }` block, not inside it. Copying a corpus witness's shape is
not laziness when the shape is the part that was measured.

## F-0103 re-measured at the sc32 pin — #201 has NOT ruled, and the residue is re-dated

Re-probed 2026-09-02 at `wolf 0.2.2 (wolfgang, pin 8cda3aa)` /
`lupin 0.1.22 (pin 2bfbe5e)`, both halves, each in its own directory:

| probe | shape | lupin | wolfc `--checked` | wolf `--native` |
|---|---|---|---|---|
| f1 | `row_name(narrow(1))` — a raising call STRAIGHT into a row-typed parameter | `exit(0)` `alpha` | **`unsupported` — `control flow in an argument` @`mem`, `@575..584`** | `exit(0)` `alpha` |
| f2 | `let r = narrow(1)` then `row_name(r)` — the BOUND form | `exit(0)` `alpha` | `exit(0)` `alpha` | `exit(0)` `alpha` |

**Verbatim. Nothing moved, and wolf-lang#201 is still OPEN — no
ruling, so there is nothing to adopt.** The residue is re-dated rather
than retired, and the reason is checked per surface the way the drift
prediction's are: the v0.2.1..v0.2.2 span is a windows native bring-up
(a different backend on a host this machine is not), an LSP navigation
trio (a `wolf_query` surface, not a lowering), the region ledger and
its cap (new `mem` capability, no change to `mem`'s ARGUMENT handling),
D69's separator tightening (a parser rule) and four letters. Not one
commit in 35 touches how the checked tier lowers an unhandled raising
call in argument position.

So `std.x.tls.client`'s header keeps its **bind, then name** sentence
unamended, and the three `std.option` rows this finding explains
(`or_else_default.lu`, `exists_marking.lu`, `is_none_marking.lu`) keep
their `wolfc = "unsupported"` with the cause still named. The new
module inherits the lesson without paying it: `std.mem.budget` has no
row-typed parameter, so nothing in it needed the workaround — which is
the second time in two sprints that reading F-0103 first changed a
signature before it cost a ledger row.

## The residues, re-probed at 8cda3aa, one line each

- **The chars-pairs tuple list is refused at its SIXTH consecutive
  pin.** `List[(int, int)]()` is `unsupported — this prelude container
  instantiation (generic data)` at resolve on both wolf rungs,
  `@72..90`, lupin runs it. Six pins is long enough to state the shape
  rather than re-argue it each time: this refusal has never moved as a
  SIDE EFFECT of anything, and it will move the sprint someone lowers
  generic container instantiation on purpose. Dated in the str header.
- **F-0096 refuses verbatim.** `s.get(0..^2)` is `unsupported —
  open-ended or end-relative ranges (slicing)` at resolve on both
  rungs, `@103..108`, against `[mem.str.get]`'s own sentence, lupin
  runs it. Dated in the str header beside the row that flips at its
  closure (`tests/str/end_relative_get.lu`).
- **`strbuf.in(r)` was RE-PROBED this time rather than argued from the
  span, and it is the one residue whose method had to change.** Every
  previous bump could say "regions did not move" from the commit list;
  this span moved regions more than any since regions landed. So the
  placement syntax itself was measured, both shapes: `List[int].in(r)`
  is `unsupported — a std/prelude stub without a signature` at resolve
  on both wolf rungs and refused by lupin too, and `Buf.in(r) { … }`
  over a plain struct is `fail(E0201)` at PARSE on all three — the form
  is not in the grammar. s131/s132 gave regions an ACCOUNTING surface
  and a BUDGET and gave them no placement plumbing; the two are
  different work, and a reader of the strbuf header can now see that
  the distinction was measured and not assumed.
- **`reserve(n)` is unmoved**, and its forward-looking sentence is
  ANSWERED rather than carried: no capacity or string-backing commit
  exists anywhere in `75fd2d0..8cda3aa` or `0.1.20..0.1.22`, and the
  region accounting surface sc31's header pointed the next lane at
  landed in this span, was consumed at sc32, and lives in
  `std.mem.budget`. It bought `strbuf` nothing — a ledger says what a
  buffer COST and a cap says when to stop, and neither is a capacity
  you can reserve.
- **`graphemes` owes no probe**: a segmentation TABLES tier, and
  nothing in either span brings it closer.
- **The four `divergent(…)`-era addresses stay healed** (re-observed
  green in both sc32 gauntlets).

## F-0104 — the byte-buffer cost, measured from the library's side (wolf-lang#203's evidence)

The sprint contract asked for the #203 ask's EVIDENCE rather than its
fix: measure what a `bytes`-tier io buffer would cost against
`List[int]` in std's own readers, recommend, and build nothing. Filed
here as a finding because it is a measurement this repo owns and will
be re-run at every bump until the ask lands.

**Method.** `region_bytes` over a fresh region per size, a `List[int]`
filled by `push` to N elements — the exact shape every byte-producing
surface in std hands back (`fs.read_bytes`, `fs.read_chunk`,
`net.read_bytes`, `bytes.from_str`, `str.bytes()` materialized). Three
lanes, 2026-09-02, at wolf 0.2.2/8cda3aa and lupin 0.1.22/pin 2bfbe5e,
macOS arm64.

| payload bytes | lupin ledger | checked ledger | native ledger | native ÷ payload |
|---|---|---|---|---|
| 1,024 | 32,736 | 16,384 | 16,368 | 16.0x |
| 2,048 | 65,504 | 32,768 | 32,752 | 16.0x |
| 4,096 | 131,040 | 65,536 | 65,520 | 16.0x |
| 8,192 | 262,112 | 131,072 | 131,056 | 16.0x |
| 16,384 | 524,256 | 262,144 | 262,128 | 16.0x |
| 32,768 | 1,048,544 | 524,288 | 524,272 | 16.0x |
| **65,536** | **2,097,120** | **1,048,576** | **1,048,560** | **16.0x** |

**Three things this table says that #203 could not.**

1. **It reproduces lobo's numbers to the byte, from a different
   program, at a later pin.** The issue reports 1,048,560 native and
   1,048,576 checked for a 64 KiB chunk; this repo measures exactly
   those two numbers with no lobo code involved. The 16x is a property
   of the representation, not of one consumer's loop.
2. **It is linear and clean at every scale**, from 1 KiB to 64 KiB —
   `ledger = 16 x payload` on both wolf tiers, with the native tier
   sixteen bytes under (one allocation header). So a fix is worth
   exactly its multiplier: a byte-width element type takes every row
   in this table down 8x, and preallocation from a known length takes
   the remainder down 2x. Neither is a rounding error at any size a
   server sees.
3. **The reference interpreter is 32x, not 16x — a multiplier #203
   does not carry.** lupin's own documented geometry is a 16-byte value
   slot (against the wolf tiers' 8), so the element-width half of the
   cost is 16x there and the growth history doubles it again. Every
   number in the lupin column is exactly 2x its checked twin. That
   matters to the ask: a portable program deriving a budget from a
   measured `region_bytes` reading is fine (the clause's own advice),
   but a program that hard-codes a per-tier constant is out by 2x
   between the two machines before it is out by 16x against its
   payload.

**And a fourth measurement, which is a finding in its own right: a
`str` charges NO named region's ledger on ANY tier at this pin.**
`[mem.region.account.1]` names this gap and scopes it to the NATIVE
tier ("the native tier realizes `str` materialization's ambient region
as the process root — wolf-lang#191, the c09 seam — so string bytes
appear in no named region's ledger THERE today"). Measured: 200 fresh
interpolated strings built inside `in r { … }` leave `region_bytes(r)`
at **0** under lupin, on the checked machine and natively alike. The
clause's own warning — "programs must not read this clause as `str`
never charges" — is currently true of every tier, not one, and the
sentence should either widen or the two non-native tiers should charge.
This is the half of lobo's memory story that no ledger can see, and
it means an operator reading `region_bytes` as "what this request
cost" is missing string bytes entirely on every machine.

**The recommendation, in the ask's own terms.** #203 asks for one of
two properties and says either alone helps. Both are worth having and
they are separable work:

- **One byte per byte** is the bigger win (8x, and it is the half that
  makes the type honest about what it holds — a byte buffer that cannot
  hold invalid UTF-8 is not a byte buffer). std has documented `Bytes`
  as an interim since sc05, and `std.bytes`' header already states the
  landing shape: **every signature keeps its form — `List[int]` becomes
  `Bytes` and nothing else moves.** That is a real, checked property of
  the existing surface, not a hope: all nine functions in `std.bytes`,
  plus `fs.read_bytes`/`read_chunk`/`write_bytes`/`write_chunk` and
  `net.read_bytes`/`write_bytes`, are monomorphic over `List[int]`
  today precisely so they run on all three lanes (F-0026), and a
  byte-width nominal type keeps that property where a generic would
  lose it.
- **Preallocation from a known length** is the smaller win (2x) and the
  cheaper one: `fs_read_chunk(fd, n)` and `net_read_bytes(s, max)` both
  KNOW their bound at the call. A buffer sized from that argument
  rather than grown by doubling reports the buffer instead of the
  buffer's history, and it needs no new type at all.

**Recommendation: a std `Bytes` over a language byte-width element
type, not a std-only wrapper.** std cannot fix this from its own side
and should not pretend to: a `struct Bytes { xs: List[int] }` would
change no allocation, hide the cost behind a nicer name, and cost a
lane (a wrapper's methods are methods on a std type, and this sprint's
own probes just measured what the checked rung does with those). The
element-width half is a language change by construction. What std can
do, and did at this sprint, is make the cost VISIBLE and keep every
signature in the landing shape so the fix is a pure substitution when
it comes — the F-0049 lesson (declare against the fix; the fix is then
an addition) applied to a type rather than to a row.

**No build. Nothing in this repo is worked around** — the byte
surfaces keep their shapes, the ledger did not move, and
`std.mem.budget`'s header carries the caveat so a caller sizing a
budget from payload arithmetic is warned at the place they would make
the mistake.

## Retirements and movements at the sc33 pin — the bytes get a width

**The drift prediction, written 2026-09-02 07:26 EDT, BEFORE the
binary was replaced and BEFORE any gauntlet at the new pins** (read
first, per the ritual: lupin's 0.1.23 CHANGELOG entry at the tag
`127b6fa`; the whole `8cda3aa..813153e` data-pin span commit by commit
— 19 commits, 4 first-parent; both upstream trees counted and diffed
at the two data-pin shas before the run; the repo grepped for every
new surface's SHAPE).

**Two of the three pins move, and they move in different directions.**
This bump is the first in this repo's history where the three pins
have to be told apart out loud:

- **The wolf BINARY does not move.** It stays at the v0.2.2 tag
  (`8cda3aa`) — r06 moves it later. So the checked and native lanes
  meet no new compiler at all, and their predicted zero is a zero **by
  construction rather than by grep**. That is a third kind of zero
  after sc31's (a span with no capability) and sc32's (a capability
  with no carrier), and it is the cheapest of the three to defend:
  nothing was measured about wolfc this bump because nothing about
  wolfc changed.
- **The lupin BINARY moves** 0.1.22 (`753d686`, conformance pin
  `2bfbe5e`) **-> 0.1.23** (`127b6fa`, conformance pin `8cda3aa`). One
  release, is34, three letters. Note what happens to the gap sc32
  recorded as the largest this repo had ever seen: lupin's conformance
  pin **catches up to wolf's own, 35 commits behind -> ZERO**. It
  closes because is34 chased v0.2.2 the tag, not because either
  machine owed the other a lowering.
- **The DATA pin moves `8cda3aa` -> `813153e`** — trunk, no tag, the
  sc30 dev-stamp precedent — and therefore ends up **19 commits AHEAD
  of both binary pins**. The pin clause is written out below.

### The lupin lane, letter by letter

- **#209 — a trap runs no defers, at the ROOT too. ZERO movers, and
  this is the prediction sc32 banked coming due.** sc32 recorded this
  as a MEASURED DIVERGENCE live at that pin (every wolfc lane
  abandoned a pending root defer; lupin 0.1.22 ran it) and said in
  writing that it had nothing to ride here. 0.1.23 abandons it too, so
  the divergence **heals** — and the carrier check is re-run rather
  than inherited: the tree still holds exactly **ONE** executable
  `defer` (`std/fs/fs.lu:262`, `defer fs_close(fd) else |_| {}` in
  `append_text`) and **ZERO** `errdefer`, unchanged since sc31; the
  rows that reach that defer propagate with `?` and never trap; and no
  trap row in the tree has a `defer` anywhere in the trapping frame.
  A heal with no carrier moves nothing, exactly as a divergence with
  no carrier moved nothing.

- **#55 — the record carries the trap's output. This is the letter the
  contract asked for a count from, and the honest answer is a number
  and a mechanism that disagree.**

  **Eleven std rows trap AFTER printing.** Grepped by shape over all
  56 `*_trap.lu`/`*_traps.lu` files, then read to confirm the print
  precedes the trapping call, then checked against the ledger to
  confirm the lane actually runs them (a row `unsupported` on lupin
  emits no record to change):

  | row | first print |
  |---|---|
  | `x/crypto/chacha20/counter_wrap_trap.lu` | `"reached the call"` |
  | `x/crypto/chacha20/key_len_trap.lu` | `"reached the call"` |
  | `x/crypto/chacha20/non_byte_trap.lu` | `"reached the call"` |
  | `x/crypto/chacha20/nonce_len_trap.lu` | `"reached the call"` |
  | `x/crypto/curve25519/non_byte_trap.lu` | `"reached the call"` |
  | `x/crypto/curve25519/scalar_len_trap.lu` | `"reached the call"` |
  | `x/crypto/sha2/hkdf_expand_cap_trap.lu` | `"extracted {prk.len}"` |
  | `x/crypto/sha2/non_byte_trap.lu` | `"reached the call"` |
  | `x/tls/record/iv_len_trap.lu` | `"reached the call"` |
  | `x/tls/record/negative_seq_trap.lu` | `"reached the call"` |
  | `x/tls/record/non_byte_trap.lu` | `"reached the call"` |

  Three more trap rows contain a `print(` and are **not** in the list,
  which is why the grep had to be read rather than counted:
  `x/crypto/p256/non_byte_trap.lu`, `x/crypto/p256/private_key_trap.lu`
  and `x/tls/cert/non_byte_trap.lu` each hold exactly one print, and it
  sits AFTER the trapping call as the `"unreachable: …"` guard. Those
  programs write nothing before they die, so their records carry null
  at 0.1.23 exactly as at 0.1.22. The other 42 trap rows print nothing
  at all.

  **Predicted verdict movers from those eleven: ZERO.** Their lupin
  RECORDS change — `stdout_sha256` and `stdout_inline` go from `null`
  to the real digest and text — and no row in the ledger moves,
  because this rig never looks at a trap's stdout. Three independent
  reasons, each read out of the rig's own source rather than assumed:

  1. `classify` (`xtask/src/runner.rs`) — the Trap arm destructures
     `Check::Run { exit: ExitExpect::Trap(want), .. }`. The stdout
     field is discarded **by the pattern**; a trap row is satisfied by
     KIND alone.
  2. `diff_class` (`xtask/src/runner.rs`) — compares `stdout_sha256`
     only under `matches!(a.verdict, Verdict::Exit(_))`. A pair of trap
     records never reaches the stdout arm at all.
  3. Lint **R3** (`xtask/src/runner.rs`) — rejects `stdout=` beside a
     trap expectation outright ("a trap record carries no stdout, so
     the hash would never be compared"), so not one trap row in the
     tree even CARRIES an expectation the change could contradict.

  So this repo's rig already implements `[proto.cmp.phase]`'s "for
  `trap`, compare kind only" — the same posture lupin's own differ
  keeps, and the one wolf-lang#216 is filed to widen. **The letter
  changes what the record SAYS, not what any lane CONCLUDES.**

  **The before-picture, measured at 0.1.22 before the binary was
  replaced**, because a claim about what a bump changes is worth more
  with both sides of it observed. A three-lane witness that prints and
  then divides by zero:

  | lane (pre-bump) | verdict | `stdout_sha256` | `stdout_inline` |
  |---|---|---|---|
  | lupin 0.1.22 | `trap(div-zero)` | **null** | **null** |
  | wolf 0.2.2 `--checked` | `trap(div-zero)` | `5726e3cf…` | `"reached the call\n"` |
  | wolf 0.2.2 `--native` | `trap(div-zero)` | `5726e3cf…` | `"reached the call\n"` |

  **The asymmetry was always on lupin's side alone** — the two wolf
  lanes have carried trap stdout all along — and it stayed invisible
  for exactly the reason it is harmless here: the comparator does not
  look. Predicted at 0.1.23: lupin joins with the **same digest**, the
  three lanes agree field for field, and the rig notices nothing.

- **#56 — the comma refusals teach the comma. ZERO.** Diagnostic
  WORDING plus an additive `help` line, and D22 puts wording outside
  the differential protocol; lupin's own release measured that no
  record moved and asserted span parity structurally. This repo
  compares verdicts and diagnostic CODES, never message text, and no
  ledger row pins a message. Zero by shape as well as by protocol:
  sc32 swept all 420 `.lu` files for the D67/D69 separator forms and
  found none, and the tree has not gained one since.

- **is34's remaining items are lupin-internal or corpus-only. ZERO.**
  #198's string half lands two upstream corpus witnesses (re-grepped
  here: the tree's maximum `\u{…}` is still six digits, and no row
  pins E0110 or E0101); wolf-interp#57 is a declared conservatism with
  a standing test on lupin's side; DIV-2026-020 (wolf-lang#220) is a
  span-WIDTH class over eight upstream `grammar/` files, and this
  repo pins no spans.

### The data pin, `8cda3aa` -> `813153e`

19 commits, 4 first-parent: s60b (the windows task layer), the
2026-09-02 ledger ritual, and an xtask dist-smoke repair.

- **`spec/` moves in exactly ONE file and adds NO anchor.**
  `git diff 8cda3aa..813153e -- spec corpus` is a single hunk in
  `spec/11-os.md`, +15/-6, all of it prose INSIDE the already-existing
  `[os.signal.platform]` clause, recording that windows console-control
  delivery landed and that `os_signal_raise` there is an in-process
  delivery.
- **Anchors 411, and `spec/anchors.json` is byte-identical between the
  two shas** — `git diff` reports nothing for that file. So the
  re-vendor moves no bytes and the F-0100 both-ways key-set diff is a
  formality this time. Said out loud rather than skipped: sc32's was
  load-bearing (404 -> 411, the first byte-moving re-vendor since
  sc27), this one is not, and knowing which you have is the point of
  doing it either way.
- **`corpus/` is unchanged, 490 -> 490**, counted at both shas with
  `git ls-tree -r --name-only`.
- **Zero carriers for the `[os.signal.platform]` prose**:
  `grep -rn "os\.signal\|os_signal" std tests` finds NOTHING, and
  nomad-1 is macOS/arm64 in any case — every new sentence in the
  clause is windows semantics.
- **s60b is a windows RUNTIME bring-up** — Win32 workers on kernel
  guard stacks, a VEH reporter, the WSAPoll reactor rung, the clif
  by-name refusal table emptying — plus its CI. This repo runs
  `conform-run` on macOS/arm64 against a compiler binary that does not
  move this bump. Zero.
- **`4d9683d`, the dist smoke degrading by name on unserved hosts** —
  wolf-lang's own xtask, an archive-gap repair for the next tag. Not a
  language surface, not consumed here. Zero.

### The pin clause: a DATA pin ahead of BOTH binary pins

Stated before the run because doctor has an opinion and the sprint
should be on record predicting it. After this bump:

| pin | sha | relation |
|---|---|---|
| wolf binary | `8cda3aa` (v0.2.2 tag) | the machine's compiler |
| lupin binary | `8cda3aa` (0.1.23's conformance pin) | caught up, gap 35 -> 0 |
| **data** | **`813153e`** | **19 commits AHEAD of both** |

sc30 set the precedent for a data pin that runs ahead of a binary pin
and doctor's expected posture is REPORT, NOT GATE — the gate is on the
binary's self-declared pin matching `vendor/tools.toml`, which is a
different field from the vendored snapshot's. The prediction is that
doctor is GREEN and says nothing about the 19 commits, because it has
no field that compares them. If it does say something, that is the
finding.

**Predicted total: ZERO verdict movers over 376x3.** The baseline the
gauntlet must reproduce exactly, counted from the untouched ledger at
trunk `6d6b025`: lupin **304** `run` / **72** `unsupported`; wolfc
**299** `run` / **75** `unsupported` / **2** `fail(E…)` (E1013, E0301);
native **325** `run` / **49** `unsupported` / **2** `fail(E…)`. Anchors
**411**, +0/-0. Corpus **490**, unchanged. (sc32's register recorded
373x3; the +3 is `std.mem.budget`'s own rows, landed at the end of that
sprint after its gauntlet.)

**And the sub-verdict prediction, which is this bump's actual content:
eleven lupin records change shape and not one row moves.**

**The measurement (same day, untouched ledger, three lanes, idle rig):
ZERO movers — the prediction is exact, and the eleven records changed
shape exactly as predicted while not one row moved.** The gauntlet ran
**07:38:43 -> 08:04:50 EDT** over files last touched at 07:25:15 (the
prediction), 07:26:29 (the PIN) and 07:28:00 (the tools.toml record) —
mtimes checked against the run window, the sc30 rule — with
`tests/ledger.toml` untouched at 07:15:19 (the worktree's own checkout)
and `git diff` empty on it. `cargo xtask ci` exit code **0**,
`ci: GREEN`, the exit code read directly and **not** piped (the
gauntlet's own standing lesson):

- `std-test: 376 test(s); forward tags: 697; conservatism ledger: 200
  entries; unstable rows: 0; slow skips: 0; divergent rows: 0` —
  `std-test: GREEN`. Every row observed exactly the verdict the
  untouched ledger claims; a deeper answer is a designed RED here, so a
  green run over an unedited ledger IS the zero-motion measurement.
  `ledger-check: 376 test(s), all ledgered`; `lint-conventions: 376
  test(s), all conforming (5 rules)`; `doc-examples: 414 block(s),
  GREEN`; `ulp: 200 reference row(s), GREEN` with the standing
  16-value libm note unchanged and all three lanes reproducing the 200
  recorded values exactly.
- `sync-pin: PIN 813153ec1bdfbb85ab1b41737dafb369d052a581` /
  `sync-pin: snapshot == submodule at pin — OK`. Anchors **411**,
  `vendor/upstream/anchors.json` untouched at 07:15:19 because it is
  byte-identical to `upstream/spec/anchors.json` at 813153e — the
  re-vendor moved no bytes, as predicted.
- **Doctor green, exit 0, and the data-pin sentence is the one to
  quote**, because sc33 is the first bump where the three pins come
  apart:

      doctor: lupin — /Users/…/.local/bin/lupin (source: PATH)
              version: lupin 0.1.23
              pin: 8cda3aa matches vendor/tools.toml — OK
      doctor: wolf — /Users/…/.local/bin/wolf (source: PATH)
              version: wolf 0.2.2
              pairing: paired with lupin 0.1.22 (reference interpreter), pin 2bfbe5e
              pin: 8cda3aa matches vendor/tools.toml — OK
      doctor: native rung — libwolf_rt.a at /Users/…/libwolf_rt.a (lane lit)

  **Doctor says NOTHING about the data pin standing 19 commits ahead of
  both binary pins, and that is the correct answer rather than a
  missed one.** Predicted from the gates' source before the run and
  confirmed by it: doctor's two gates are the binary's self-declared
  version and its self-declared pin against `vendor/tools.toml`; it
  never opens `vendor/upstream/PIN` at all, and `sync-pin` gates that
  snapshot against the SUBMODULE rather than against either binary. So
  the "one-sha invariant" the sc32 entry named is a CONVENTION recorded
  in a comment, not a rule enforced by a gate — and suspending it costs
  exactly nothing, by design. sc30's dev-stamp precedent holds and is
  now understood rather than merely followed.
  The pairing line goes one release stale again (`paired with lupin
  0.1.22 … pin 2bfbe5e` beside an installed 0.1.23) because r05 cut
  v0.2.2's pairing commit before is34 released; reported, never gated
  (F-0064), and the sc27 nuance that sc32 could report as not
  recurring does recur here.

**The eleven records, confirmed changed and confirmed harmless.** The
same three-lane witness measured before the fresh-inode install was
re-run after it:

| lane | verdict | `stdout_sha256` | `stdout_inline` |
|---|---|---|---|
| lupin **0.1.22** (before) | `trap(div-zero)` | **null** | **null** |
| lupin **0.1.23** (after) | `trap(div-zero)` | **`5726e3cf…`** | **`"reached the call\n"`** |
| wolf 0.2.2 `--checked` | `trap(div-zero)` | `5726e3cf…` | `"reached the call\n"` |
| wolf 0.2.2 `--native` | `trap(div-zero)` | `5726e3cf…` | `"reached the call\n"` |

lupin joins with the **byte-identical digest** and the three lanes now
agree field for field. The eleven std rows that trap after printing
carry the same change; `divergent rows: 0` and a green ledger say that
none of them moved a verdict. **A record can change shape without any
lane changing its mind, and the two questions have to be predicted
separately** — the release note says what an implementation EMITS, the
rig says what you CONCLUDE.

## F-0104's io half — the readers measured, and the preallocation win is *entirely* unrealized

sc32's table was a synthetic `List[int]` filled by `push`. The sprint
contract asked for the io readers' own cost, which is what wolf-lang
#203 is actually about, so std's readers were measured directly: one
fresh region per call, `region_bytes` read inside it, payload written
and read back from a real file.

| payload | `fs.read_bytes` charged | `fs.read_chunk(f, n)` charged | ÷ payload |
|---|---|---|---|
| 1,024 | 16,384 / **16,368** | 16,384 / **16,368** | 16.0x |
| 4,096 | 65,536 / **65,520** | 65,536 / **65,520** | 16.0x |
| 16,384 | 262,144 / **262,128** | 262,144 / **262,128** | 16.0x |
| **65,536** | 1,048,576 / **1,048,560** | 1,048,576 / **1,048,560** | 16.0x |

(checked / **native**. The lupin lane cannot run this probe —
`fs_write_bytes` does not resolve there, the long-standing F-0081 gap:
12 of the tree's 14 `fs` rows are `unsupported` on that lane — so its
column stays the synthetic one.)

**Two findings, one of them new and sharp.**

1. **The readers cost exactly what the synthetic list costs**, to the
   byte, at every size. Native's 1,048,560 for a 64 KiB chunk is
   lobo's own reported number, now reproduced a third time — first by
   lobo, then by sc32's synthetic list, now through std's actual
   reader. The 16x is the representation, not any consumer's loop and
   not any particular allocation path.
2. **`read_bytes` and `read_chunk` charge IDENTICALLY, and that is the
   new result.** `fs.read_chunk(f, n)` is handed its bound `n` AT THE
   CALL — it is the single surface in std best positioned to take
   #203's second property (preallocate from a known length) — and it
   charges precisely what the unbounded whole-file read charges,
   because the buffer is still grown by doubling rather than sized
   from the argument. So the preallocation win is not partly taken
   today; it is **entirely** untaken, on the one call that already
   knows the answer. That is a 2x sitting unclaimed behind no new type
   at all, and it is the half of #203 that could land independently.

## The #203 proposal, as filed

Filed as the recommended shape on wolf-lang#203
(`#issuecomment-5509341730`), spec-shaped, with the evidence tables
above. The compiler lane decides; std builds nothing. Its spine, and
the reason it is shaped the way it is:

**The spec has no width story to extend — it has no type inventory at
all.** Read at 813153e before the ask was written: the `[type.*]`
family is twenty anchors in exactly three groups (`numlit`, `char`,
`str`); there is no `[type.scalar]`, no `[type.prim]` and **no
`[type.int]`**; `int` carries no defining clause anywhere, so its
width, signedness and representation are all unstated (the chapter
closes by naming the omission deliberate); there are no fixed-width
integer types, no width vocabulary and no literal suffixes, so `1u8`
is unspellable by the grammar; and scalar type names are not even
keywords. So the ask is not an amendment. **It is the first clause of
an inventory that does not exist**, which is worth knowing before
anyone scopes it and is the honest reason this has sat in three issues
(#80, #86, #137) without a home.

**Both cheap answers fail by the same mechanism, one measured and one
read.** sc32 MEASURED that a std `struct Bytes { xs: List[int] }`
changes no allocation. sc33 READ that the spec's own newtype is no
better: `[gram.item.trait]` defines `distinct` as "same layout as the
base, free bidirectional `as` casts", because layout preservation is
the entire point of the construct. The library-side wrapper and the
language-side newtype are the same non-answer from two directions —
which is what makes the pair worth more than either alone, and what
establishes that this genuinely cannot be absorbed by std.

**The recommended shape is `[type.byte]` modelled on `[type.char]`,
which already exists.** The spec contains exactly ONE clause giving a
scalar a stated width, and it carries every part a byte type needs: a
domain, "Layout: 4 bytes, alignment 4", a `List[char]` stride, a C-seam
rule, a no-arithmetic posture and an explicit cast bridge. And
`[mem.str.chars]` says in as many words why the byte tier was left
behind — "The byte tier is unchanged: `bytes()` stays `List[int]` of
bytes — `char` is the scalar tier, **never a byte**." s121 gave the
scalar tier a width-bearing type and consciously declined to give the
byte tier one; the proposal is to finish that job, as `[type.byte]` /
`[type.byte.cast]` (with `[type.byte.lit]` separable and optional, and
a `[mem.list.repr]`-shaped clause if element storage is to become a
stated fact rather than a measured one). Copying `char`'s "no
numeric-literal adoption" also sidesteps the spec's own recorded
`i32`-vs-`int` defaulting contradiction (F-0004 gap 3, still open after
10-types.md landed) rather than inheriting it.

**One correction to F-0104 as sc32 wrote it**, caught by re-measuring a
number rather than quoting it: `std.bytes` has **TEN** public
functions, not nine (`len`, `is_empty`, `at`, `slice`, `find`,
`starts_with`, `ends_with`, `from_str`, `to_str`, `is_utf8`). The claim
the count serves is unchanged and re-verified — every one is
monomorphic over `List[int]`, as are `fs.read_bytes`/`read_chunk`/
`write_bytes`/`write_chunk` and `net.read_bytes`/`write_bytes` — so the
landing shape still holds: `List[int]` becomes the byte type and no
signature changes form. But the wrong number was two keystrokes from
being restated in a public upstream issue as this repo's own evidence.

## The checked tier and `breach_is_a_row` — DEFERRED, with the reason and a note for s134

The sprint contract makes this conditional: if s134's item 1 (wolf-lang
#219) has merged by the second gauntlet, `mem/budget/breach_is_a_row.lu`
flips from `wolfc = "unsupported"` to three-lane and takes the trunk
sha as the data pin. **It has not merged. The flip is deferred and the
row keeps its sc32 reason unamended.**

Checked at both gauntlets: `git -C ../wolf-lang log origin/trunk` is
`813153e` at each, unmoved, with `56ed5a5` (s60b) and `f8725e7` beneath
it — the same three commits, no s134 merge; and `gh issue view 219`
reports **OPEN** with one comment, the ws13 bisection, and no fix
landed. So the data pin stays at `813153e` (already the sprint's pin,
chosen before this condition was evaluated) and the row stays two-lane.

**A note for s134, because this repo has a bearing on half of #219's
bisection.** The comment observes that `wolf conform-run --checked`
refuses every proc spawn at `mem` with an empty `diagnostics` array
while "`wolf run --checked` runs the same files to their expected
stdout", and reads the pair as two `--checked` drivers disagreeing.
This repo's standing operational note is that **`wolf run --checked`
executes the native build** — the ubcheck machine answers only through
`conform-run --json --checked` — which, if it still holds at this pin,
would mean the second half of that pair is not evidence about the
checked tier at all, and the two drivers are not disagreeing so much as
one of them is not the checked tier. Not re-verified here (it is s134's
issue and its surface, and this sprint had no reason to spend a probe
on it), so it is offered as a lead rather than a finding. The
`diagnostics`-should-carry-the-name half of the report stands
regardless, and this repo would consume it: a rig cannot triage an
empty array.

## F-0103 re-measured at the sc33 pin — unmoved, and the "unmoved" is worth one sentence of trust

Re-probed 2026-09-02 at `wolf 0.2.2 (wolfgang, pin 8cda3aa)` /
`lupin 0.1.23 (pin 8cda3aa)`, both halves, each in its own directory:

| probe | shape | lupin | wolfc `--checked` | wolf `--native` |
|---|---|---|---|---|
| f1 | `row_name(narrow(1))` — a raising call STRAIGHT into a row-typed parameter | `exit(0)` `alpha` | **`unsupported` — `control flow in an argument`, phase `mem`** | `exit(0)` `alpha` |
| f2 | `let r = narrow(1)` then `row_name(r)` — the BOUND form | `exit(0)` `alpha` | `exit(0)` `alpha` | `exit(0)` `alpha` |

The refusal string is byte-identical to sc31's and sc32's.
**wolf-lang#201 is still OPEN — no ruling, so there is nothing to
adopt**, and the residue is re-dated rather than retired.

**But the reason this bump's "unmoved" is cheap, said plainly rather
than dressed up as evidence: the wolf binary did not move.** sc33 took
a lupin release and a data-pin advance while holding the compiler at
the v0.2.2 tag, so this probe ran against the SAME binary that produced
sc32's table. It could not have moved. Previous sprints could write
"nothing in 35 commits touches `mem`'s argument handling" and mean
something by it; this one cannot, and reporting the result as evidence
of stability would be overclaiming. It is re-run because the rule is to
measure rather than assume — and the honest content of the measurement
this time is only "the workaround is still in std's source". Recorded
upstream in the same terms (#201 `#issuecomment-5509353963`).

So `std.x.tls.client`'s header keeps its **bind, then name** sentence
unamended, and the three `std.option` rows this finding explains
(`or_else_default.lu`, `exists_marking.lu`, `is_none_marking.lu`) keep
their `wolfc = "unsupported"` with the cause still named.

## F-0105 — wolfc's zero-width parse span is reachable from ordinary std-side code, not just the grammar corpus

is34's sixteenth corpus differential named **DIV-2026-020**
(wolf-lang#220, ruled as **D71**) as its dominant class: eight upstream
`grammar/` files where both machines agree on the code and the starting
byte and disagree about the span's WIDTH — lupin spans the offending
token, wolfc emits a zero-width span at its start. This repo hit the
same class by accident while re-probing the `strbuf` placement residue,
which makes it a second and independent witness:

```wolf
struct Buf { n: int, }
fn main() -> int {
    region r {
        let b = Buf.in(r) { n: 1 }
        print("{b.n}")
    }
    0
}
```

| lane | verdict | phase | span |
|---|---|---|---|
| lupin 0.1.23 | `fail(E0201)` | parse | **`[88, 89]`** |
| wolf 0.2.2 `--checked` | `fail(E0201)` | parse | **`[88, 88]`** |
| wolf 0.2.2 `--native` | `fail(E0201)` | parse | **`[88, 88]`** |

Byte 88 is the `{`; both machines report 7:27 and E0201. **The class is
not local to the eight corpus files** — this is an ordinary
struct-and-region program written for an unrelated purpose and it
carries the divergence anyway, so any std-side E0201 will.

**And it confirms why nothing measured it, from a second rig.** This
repo's runner compares verdicts and diagnostic CODES and never spans
(`record::parse` reads `diagnostics[].code`; `diff_class` compares
verdict, then `stdout_sha256` for `Exit` only), so the pair is
verdict-identical, code-identical, and passes GREEN — exactly as is34
reported of its own differ's walk, which "compares codes". Two
independent rigs look past it for the same structural reason, which is
a fair argument that the fix should not wait on either rig changing.
Nothing here is worked around: no row in this tree pins a span, and the
placement residue's own answer (the form is not in the grammar) is
unaffected. Filed upstream on #220 (`#issuecomment-5509349787`).

## The residues, re-probed at 813153e / 0.1.23, one line each

- **The chars-pairs tuple list is refused at its SEVENTH consecutive
  pin.** `List[(int, int)]()` is `unsupported — this prelude container
  instantiation (generic data)` at resolve on both wolf rungs,
  `@32..50`, and lupin runs it (`exit(0)`). Seven pins is long past the
  point of re-arguing it: this refusal has never moved as a side effect
  of anything, and it will move the sprint someone lowers generic
  container instantiation on purpose. Dated in the str header.
- **F-0096 refuses verbatim.** `s.get(0..^2)` is `unsupported —
  open-ended or end-relative ranges (slicing)` at resolve on both
  rungs, `@57..62`, against `[mem.str.get]`'s own sentence; lupin runs
  it and prints `hel`. Dated in the str header beside the row that
  flips at its closure (`tests/str/end_relative_get.lu`).
- **`strbuf.in(r)` re-probed in both shapes, and it earned its keep
  this time by turning up F-0105.** `List[int].in(r)` is `unsupported —
  a std/prelude stub without a signature` at resolve on both wolf rungs
  (`@51..55`) and refused by lupin too, whose reason now reads
  ``builtin List` has no method `in` in this machine's std subset` —
  a wording move, not a verdict move. `Buf.in(r) { … }` over a plain
  struct is `fail(E0201)` at PARSE on all three: the form is not in the
  grammar. Unmoved; the span disagreement inside that agreement is
  F-0105 above.
- **`reserve(n)` is unmoved and owes no new probe.** sc32 answered its
  forward-looking sentence with the region accounting surface and
  consumed it in `std.mem.budget`; nothing in `8cda3aa..813153e` or
  `0.1.22..0.1.23` is a capacity or string-backing commit — the span is
  a windows runtime, a ledger ritual and three letters about traps,
  defers and commas.
- **`graphemes` owes no probe**: a segmentation TABLES tier, and
  nothing in either span brings it closer.
- **A `str` still charges NO named region's ledger, on ANY tier, and
  this one WAS re-probed rather than carried** — the lupin binary moved
  this bump, so the tier that could have changed its answer is exactly
  the one that got a new build. 200 fresh interpolated strings built
  inside `region r { … }` leave `region_bytes(r)` at **0** before and
  after, on all three lanes (`before 0 after 0 sink 9890`, byte-identical
  stdout across lupin, checked and native). `[mem.region.account.1]`
  still scopes this gap to the NATIVE tier alone; it remains true of
  every tier, and the clause should either widen or the two non-native
  tiers should charge.
- **The four `divergent(…)`-era addresses stay healed** (re-observed
  green in both sc33 gauntlets; `divergent rows: 0`).

## The second gauntlet, and what it does and does not cover

`cargo xtask ci` exit code **0**, `ci: GREEN`, **08:18:42 -> 08:41:33
EDT**, numbers identical to the first: `std-test: 376 test(s); forward
tags: 697; conservatism ledger: 200 entries; unstable rows: 0; slow
skips: 0; divergent rows: 0`; `ledger-check: 376`;
`lint-conventions: 376 (5 rules)`; `doc-examples: 414 block(s), GREEN`;
`ulp: 200 reference row(s), GREEN`; `sync-pin: snapshot == submodule at
pin — OK`. Two consecutive greens over an untouched ledger at the new
pins.

**And the mtime audit, because two identical greens are exactly the
shape that once hid a pre-edit run.** Every ci-relevant byte predates
the second run: `tests/ledger.toml` and `vendor/upstream/anchors.json`
at 07:15:19 (the worktree's own checkout), `vendor/upstream/PIN` at
07:26:29, and **zero** `.lu` files under `tests/` or `std/` touched
after 07:16 — the whole tree is still the checkout, which is what makes
a green run the zero-motion measurement. Two files were written after
the run began and both are outside everything ci reads for a verdict:
`docs/findings.md` (this register) and one comment word in
`vendor/tools.toml` (a first-parent count corrected from 3 to 4 —
`git diff` shows that single line and no `version`/`pin` change). So
the second gauntlet covers the functional tree completely, and the
edits it does not cover could not have changed a verdict. Stated rather
than assumed, because "the gauntlet must cover the edits" is a lesson
this repo paid for.

**The s134 checkpoint, re-taken at the second gauntlet as the contract
requires**: `git -C ../wolf-lang log origin/trunk` is still `813153e`
(fetched fresh at 08:19:28, the same three commits beneath it) and
`gh issue view 219` still reports **OPEN**, with no s134 branch on the
remote. Item 1 has not merged at either gauntlet, so
`breach_is_a_row`'s three-lane flip is **deferred** and the data pin
stays `813153e`.

## The sc34 pin bump — the byte arrives, and the three pins come back together

**The drift prediction, written 2026-09-02 21:50 EDT, BEFORE the binary
was installed and BEFORE any gauntlet at the new pins** (read first, per
the ritual: the whole `8cda3aa..31170d1` binary span and the
`813153e..31170d1` data span commit by commit; both upstream trees
counted and diffed at the two data-pin shas before the run; the repo
grepped for every new surface's SHAPE; and — the sc33 lesson —
the BEFORE side measured on the binary that was about to be replaced).

**All three pins move, and the one-sha invariant sc33 suspended is
RESTORED.** sc33 was the first bump where the three came apart; sc34 is
the bump that puts them back:

- **The wolf BINARY moves 8cda3aa (the v0.2.2 tag) -> 31170d1**, a
  dev-stamped build at trunk: `wolf 0.2.3+dev.31170d1 (wolfgang, pin
  31170d1)`. **51 commits — the largest binary span this repo has ever
  crossed in one bump** (sc32's 35 was the previous record), because
  sc33 held the compiler at its tag while the data pin ran ahead, so
  this bump pays for two sprints of compiler at once: s60b (the windows
  task layer), the 2026-09-02 ledger ritual, s134 (the LSP annotates,
  #219, D71/#220), r06 (v0.2.3, #212/#214/#215) and s135 (the byte,
  #222, #224).
- **The DATA pin moves 813153e -> 31170d1**, 32 commits, and lands on
  the SAME sha as the binary. The invariant is a convention, not a
  gate (the sc33 lesson); it is honoured here because nothing this
  sprint wants it suspended for.
- **The lupin BINARY does NOT move**: 0.1.23 (`127b6fa`), conformance
  pin `8cda3aa`, unchanged in `vendor/tools.toml`. Note what that does
  to the gap sc33 recorded as CLOSED: lupin's conformance pin was zero
  commits behind wolf's own at sc33 and is **51 behind** here, the
  largest this repo has recorded, and every one of those 51 is a
  compiler the interpreter has not chased yet. is36 is where it closes.

### Why the binary is a DEV STAMP and not the v0.2.3 tag — measured, not assumed

The contract says "the machine wolf stays at v0.2.3 until r07", and it
does: a dev build off trunk self-brands `0.2.3+dev.<commit>`, claiming
no release. What forces the dev build rather than the tag is that
**the v0.2.3 TAG CANNOT COMPILE THIS SPRINT'S SUBJECT.** `v0.2.3` =
`3befc3e` sits twelve commits BEFORE s135, and at that tag
`crates/wolf_wir/src/lower.rs:1571` reads
`Prim::Byte => Err(refuse("byte lowering (runtime byte views, c08)", span))`
— the type name has resolved in sema for many sprints and the lowering
has always refused it. Measured on the installed tag build before it
was replaced, in its own directory, on both wolf rungs:

    $ wolf run main.lu          # let b = 65 as byte
    wolf run: cannot compile this yet — byte lowering
    (runtime byte views, c08) @24..34
    $ wolf run --checked main.lu
    (byte-identical refusal)

and `git grep "byte lowering" 31170d1` finds nothing — s135 deleted the
refusal. So there is no tagged wolf in existence that can build a
`List[byte]`, and the sc30 precedent is the honest answer: build at the
sha with the stamp applied
(`WOLF_COMMIT=31170d1 cargo build --release -p wolf_driver -p wolf_rt`),
install `wolf` + `libwolf_rt.a` to `~/.local/bin` through **fresh
inodes** (the sc26 SIGKILL rule — never overwrite in place). The
`+dev.<commit>` string is exactly what r03 designed for this case: it
claims no release and still names its own pin on line 1, so doctor's
provenance gate reads the pin clause the same way it reads a tag
build's, and `vendor/tools.toml`'s `version` key carries the full dev
identity (a lying binary is worse than none; an honest dev binary is
neither).

**And the same measurement is the before-picture the sc33 lesson says
to take**: lupin 0.1.23 refuses `as byte` with `fail(E0301)` at phase
**resolve** — `nothing with this name is in scope, so this cast names
no target type … [mod.scope]` — which is the interpreter's answer both
before and after this bump, since lupin does not move.

### The drift prediction, surface by surface

**Predicted total at the bump (before any substitution): ZERO verdict
movers over 376x3.** The baseline the gauntlet must reproduce exactly,
counted from the untouched ledger at trunk `35f69ef`: lupin **304**
`run` / **72** `unsupported`; wolfc **299** `run` / **75**
`unsupported` / **2** `fail(E…)` (E1013, E0301); native **325** `run` /
**49** `unsupported` / **2** `fail(E…)`. `std-test` should print 376
tests, 697 forward tags, 200 conservatism entries, 0 unstable, 0 slow,
0 divergent; `doc-examples` 414 blocks; `ulp` 200 rows. Anchors
**411 -> 415** (+4 / -0). Corpus **490 -> 499** files.

This zero is a **fourth** kind, and it is the most expensive of the four
to defend, so it gets the most words. sc31's was a span with no new
capability; sc32's was a capability with no carrier; sc33's was a
binary that did not move at all. sc34's is **a binary that moved
further than any before it, across a surface every one of whose
carriers has to be grepped for by name** — the cheap answer is not
available and the work has to be done:

- **`byte` itself (s135) is NEW SURFACE with no carrier at the bump.**
  `grep -rn "byte" std/ tests/ --include=*.lu` finds the word only in
  prose and in identifiers (`read_bytes`, `bytes.len`, `non_byte_trap`);
  there is not one `as byte` and not one `List[byte]` in the tree when
  the gauntlet runs. The substitution is this sprint's SECOND half and
  is deliberately not in the bump's measurement — that is what makes
  the bump a clean control.
- **#219's record-shape change moves nothing, for sc33's exact
  reason, re-read from source rather than inherited.** `conform-run`
  now adds `x-unsupported-construct` / `x-unsupported-span` to every
  `unsupported` record (`[proto.record.ext]`). This rig's
  `record::parse` reads a closed list — `protocol`, `impl`,
  `phase_reached`, `verdict`, `stdout_sha256`, `stdout_inline`,
  `warnings` — and **never looks at an extension key**, and `classify`
  maps `Verdict::Unsupported` to `Achieved::Unsupported` BY PATTERN
  without reading another field. So 75 wolfc rows and 49 native rows
  will emit strictly richer records and not one of them can move. This
  is the same shape as sc33's #55 and the same answer for the same
  structural reason: predict at both levels, and the level that counts
  is what the comparator reads.
- **D71/#220 (a parse refusal spans its offending token) moves
  nothing.** 35 upstream snapshots moved; this rig pins **no span
  anywhere**. `classify` compares a `fail(E…)` row by CODE
  (`code == want`) and `diff_class` compares verdicts and, for
  `Exit` only, `stdout_sha256`. The tree's two `fail` rows are E1013
  and E0301, and neither names a locus. (This was already established
  as F-0105's aside at sc33 and is re-read, not carried, because the
  binary that produced the spans is new here.)
- **#222 (one path spelling across 23 pkg sites) has no carrier: this
  rig never runs a package verb.** `runner.rs` invokes exactly
  `wolf conform-run [--checked|--native] --std-root <dir>` and
  `lupin conform-run`; `add`/`rm`/`init`/`vendor`/`publish` are not in
  the rig's vocabulary at all.
- **#224 (net_deadline arms on a reset socket) has no carrier, and the
  carrier check is specific rather than categorical.** The change is
  25 lines in `wolf_mem/src/ubcheck.rs` — the CHECKED machine only —
  and it fires on exactly one condition: `setsockopt` answering
  `InvalidInput` while `local_addr()` still succeeds, i.e. a socket
  the peer RESET by closing over unread data. The tree has exactly one
  `set_deadline` call site in a test (`tests/net/read_deadline_row.lu`)
  and it arms the budget on a socket whose peer (`conn`) is accepted
  and never closed, never written to and never reset — the whole point
  of that test is a SILENT peer, not a departed one. Native's timer
  wheel never called `setsockopt` in the first place. Both wolf lanes
  predicted unmoved.
- **s60b (the windows task layer) has no carrier: the host is macOS
  arm64.** `stack_win.rs` is a new file behind a target gate, and the
  reactor/signal churn beside it is the same lane's. The one thing in
  that merge that is NOT windows-only is a poller wake fix; the tree's
  net rows are loopback and synchronous and were green across it at
  the sc33 data pin already.
- **s134's LSP trio (`signatureHelp`/`semanticTokens`/`inlayHint`) and
  the binding table under it have no carrier**: `wolf_query` is an
  editor surface, this repo runs `conform-run`.
- **#219's LLVM half (func.addr across partitions) has no carrier: no
  lane in this repo builds the release tier.**
- **r06 is a version bump plus dist/release-notes plumbing**, and
  #215's nine grammar productions are SPEC text — upstream states
  anchors held at 411 across them, and the +4 this bump sees is
  s135's `[type.byte]` family alone.
- **The two `fail(E…)` rows and the three F-0103 rows are predicted
  verbatim.** F-0103's asymmetry lives in `mem`-phase argument
  lowering; nothing in 51 commits touches argument lowering, and
  `wolf_wir/src/lower.rs`'s 76 changed lines are the byte cast/op
  bridges. Re-probed as its own item below rather than assumed —
  and unlike sc33, this time the binary DID move, so the probe is
  evidence rather than a formality.

**The lupin lane is predicted UNCHANGED AT THE BUMP, and that is a
zero of sc33's cheapest kind — the binary did not move.** No std row
uses `byte` when the gauntlet runs, so there is nothing for lupin's
E0301 to reach even if it could.

**The doctor prediction.** Doctor gates the binary's self-declared
version and pin against `vendor/tools.toml` and never reads
`vendor/upstream/PIN`; `sync-pin` gates the vendored snapshot against
the SUBMODULE. With `version = "0.2.3+dev.31170d1"` and
`pin = 31170d1…` recorded, doctor is predicted GREEN on both binaries
with wolf's line 1 read as the dev identity and the pairing line
(`lupin 0.1.23 … pin 8cda3aa`) reported and not gated (F-0064). If
doctor gates the `+dev.` suffix in a way sc30's note did not record,
that is the finding.

### The measurement at the bump — ZERO movers, the prediction exact

`cargo xtask ci` exit code **0**, `ci: GREEN`, **21:52:28 -> 22:20:25
EDT**, the exit code read directly from the process and **not** through
a pipe (the gauntlet's standing lesson). Over the untouched ledger:

- `std-test: 376 test(s); forward tags: 697; conservatism ledger: 200
  entries; unstable rows: 0; slow skips: 0; divergent rows: 0` —
  `std-test: GREEN`. A row that answers DEEPER than its ledger claims
  is a designed RED here, so a green run over an unedited
  `tests/ledger.toml` IS the zero-motion measurement. The 200-entry
  conservatism ledger is the per-lane counts summed and therefore
  re-proves them arithmetically: 72 (lupin `unsupported`) + 75 (wolfc)
  + 49 (native) + 2 + 2 (the `fail(E…)` pairs) = 200, the sc33 baseline
  to the entry.
- `ledger-check: 376 test(s), all ledgered`; `lint-conventions: 376
  test(s), all conforming (5 rules)`; `doc-examples: 414 block(s),
  GREEN`; `ulp: 200 reference row(s), GREEN` on all three lanes with
  the standing 16-value libm note unchanged.
- `sync-pin: PIN 31170d119379086f6242cd88d4f4e5386f6aef23` /
  `sync-pin: snapshot == submodule at pin — OK`. Anchors **415**.
- **Doctor green, exit 0, and it reads the dev stamp exactly as sc30's
  note predicted it would:**

      doctor: lupin — /Users/…/.local/bin/lupin (source: PATH)
              version: lupin 0.1.23
              pin: 8cda3aa matches vendor/tools.toml — OK
      doctor: wolf — /Users/…/.local/bin/wolf (source: PATH)
              version: wolf 0.2.3+dev.31170d1
              pairing: paired with lupin 0.1.23 (reference interpreter), pin 8cda3aa
              pin: 31170d1 matches vendor/tools.toml — OK
      doctor: native rung — libwolf_rt.a … (lane lit)

  Both gates pass on a binary that claims no release. The pairing line
  names the interpreter actually installed beside it, reported and not
  gated (F-0064).

**And #219's extension keys are live and confirmed unread**, which is
the one prediction worth showing rather than asserting. A wolf
`unsupported` record now reads:

    {"commit":"31170d1", … ,"verdict":"unsupported","warnings":[],
     "x-unsupported-construct":"this prelude container instantiation (generic data)",
     "x-unsupported-span":[25,43]}

Every `unsupported` record in the tree gained those two keys this bump
— 124 of them on the wolf lanes — and `record::parse`'s key list does
not contain either name, so all 124 changed shape and none could move.
That is sc33's #55 lesson arriving a second time from a different
implementation: **predict at both levels, and the level that decides is
what your comparator reads.**

## F-0104 CLOSES — the after-table, and the substitution that cannot be made yet

**Status: CLOSED as a measurement.** wolf-lang#203's ask landed as D72
/ s135's `[type.byte]`, the multiplier F-0104 filed is retired IN THE
LANGUAGE, and this repo has now measured the after-picture from its own
side. What does NOT close with it is the library's ability to spend the
win, and the reason is a fact this sprint discovered rather than
inherited — it is F-0106 below.

### The after-table (native / checked), measured 2026-09-02 at `wolf 0.2.3+dev.31170d1` and `lupin 0.1.23`, macOS arm64

Same method as sc32's before-table, run in one program so the two
columns cannot drift: a fresh `region` per row, a list filled by `push`
to N elements, `region_bytes` read inside the block.

| payload | `List[int]` (before) | `List[byte]` (after) | after ÷ payload | `List[int]` then converted to `List[byte]` |
|---|---|---|---|---|
| 1,024 | 16,384 / **16,368** | 1,024 / **2,096** | 1.00x / **2.05x** | 17,408 / **18,464** |
| 2,048 | 32,768 / **32,752** | 2,048 / **4,144** | 1.00x / **2.02x** | 34,816 / **36,896** |
| 4,096 | 65,536 / **65,520** | 4,096 / **8,240** | 1.00x / **2.01x** | 69,632 / **73,760** |
| 8,192 | 131,072 / **131,056** | 8,192 / **16,432** | 1.00x / **2.01x** | 139,264 / **147,488** |
| 16,384 | 262,144 / **262,128** | 16,384 / **32,816** | 1.00x / **2.00x** | 278,528 / **294,944** |
| 32,768 | 524,288 / **524,272** | 32,768 / **65,584** | 1.00x / **2.00x** | 557,056 / **589,856** |
| **65,536** | 1,048,576 / **1,048,560** | **65,536** / **131,120** | **1.00x** / **2.00x** | 1,114,112 / **1,179,680** |

(checked / **native**. The lupin column is gone from this table on
purpose and is the last section below.)

**Four things the table says.**

1. **16.0x -> 2.0x native, 16.0x -> 1.0x checked, at every size.** The
   8x element-width multiplier is retired on both wolf tiers. Natively
   the residue is exactly `2 x payload + 48` — the 48 is one list
   header, constant from 1 KiB to 64 KiB — so what remains is the
   push-growth history `[mem.region.account.1]` keeps charged, which is
   #203's separable second property and no type change can touch it.
   **The checked machine has no growth history at all and charges the
   payload EXACTLY**: 65,536 for 65,536 bytes, 1.00x, the first time
   any number in this family has been 1.
2. **The before column reproduces sc32's table to the byte at a
   compiler 51 commits newer**, including lobo's own 1,048,560. The 16x
   was the representation, and it still is wherever the representation
   is still `List[int]`.
3. **The io readers are the same numbers, measured through std rather
   than synthetically** — `fs.write_bytes` a payload, read it back in a
   fresh region: 16,368 / 65,520 / 262,128 / 1,048,560 native at
   1 KiB / 4 KiB / 16 KiB / 64 KiB, and 16,384 / 65,536 / 262,144 /
   1,048,576 checked. Identical to F-0104's io half at sc33, and to the
   synthetic column above. Nothing about std's readers changed, which
   is the point of the next column.
4. **The fourth column is the finding.** `List[int]` **then converted
   to** `List[byte]` — one list read from a producer, one list built by
   `push`ing `x as byte` — charges **18.0x native and 17.0x checked**,
   both **WORSE than the 16.0x it replaces**, at every size, because
   the ledger is cumulative and the intermediate list is never
   subtracted. That column is not a curiosity. **It is exactly what
   every substituted signature in std would charge today**, and it is
   why this sprint did not substitute. See F-0106.

### The lupin lane, and the count the flip set needs

lupin 0.1.23 does not move this bump, and it **refuses `byte` by name**:
`fail(E0301)` at phase **resolve** — `nothing with this name is in
scope, so this cast names no target type … [mod.scope]` — measured on
each probe above.

**But the refusal is narrower than "every substituted row", and the
narrowness is worth a sentence, because it changes what is36 has to
flip and what a lane audit should count.** lupin is dynamically typed
and **does not check a type NAME in annotation position at all**: a
program declaring `fn blen(b: List[byte]) -> int` and calling it RUNS
under 0.1.23 and prints its answer. What refuses is the **cast target**
`as byte` — the one construct that names the type in a position lupin
resolves. So:

- a signature-only substitution would move **zero** lupin rows;
- and every row whose reachable source contains one `as byte` moves to
  `fail(E0301)@resolve` — which is every row of a real substitution,
  because with no byte-typed builtin in the language a `List[byte]` can
  only be BUILT by casting (see F-0106), so the cast is unavoidable
  wherever the tier is real.

**The is36 flip set, counted rather than estimated: 57 files and 262
call sites carry the byte tier in this tree** (95 `.bytes()` builtin
calls, 132 `std.bytes` calls, 16 `std.fs` byte calls, 15 `std.net` byte
calls, 4 direct `str_from_utf8`), spread over `std.bytes`, `std.fs`,
`std.net`, `std.hex`, `std.base64`, `std.json`, `std.x.jose`,
`std.x.crypto.{sha2,chacha20,curve25519,p256}` and
`std.x.tls.{cert,handshake,record,client}`. Of the tree's 376 ledger
rows, the ones a substitution would take dark on lupin are the rows
reaching those files. Counted against the ledger: **40 of the 376
rows sit on a test file that names the byte tier directly, and 32 of
those 40 are `run` on lupin today** (the other 8 are already
`unsupported` for other reasons). So a substitution's floor is **32
lupin rows moving `run` -> `fail(E0301)@resolve`**, and the ceiling is
higher, because a cast inside a std module refuses at resolve for every
test that imports it, not only for the tests that name the tier.

is36 needs `byte` in type position, the two casts, operator
widening, `Value::Byte` with 1-byte list slots and `{b}` printing the
number; until it ships, ANY substitution here trades three lanes for
two on every row it touches.

## F-0106 — the byte TYPE landed; the byte PRODUCERS did not, so std's tier cannot take `List[byte]` without paying more than the type saves

**This sprint's headline, and the reason target 2 was not executed as
written.** The contract and wolf-lang#203's closing comment both call
the std-side change "a pure substitution: `List[int]` becomes
`List[byte]` and nothing else moves". That was true of the SHAPE of
std's signatures — sc32 and sc33 verified the monomorphism twice and it
still holds — and it is false of the PIPE at this pin, for a reason
neither sprint could have known before the landing: **s135 gave the
language a byte type and gave it no byte-typed builtin.**

### The measurement that decides it

Read out of `crates/wolf_sema/src/check.rs` at `31170d1`, every builtin
that produces or consumes a byte sequence, with the type it is declared
at:

| builtin | signature at 31170d1 |
|---|---|
| `str`'s `.bytes()` method | `-> List[int]` |
| `str_from_utf8` | `(List[int]) -> str ! {utf8}` |
| `fs_read_bytes` | `(str) -> List[int] ! {…}` |
| `fs_write_bytes` | `(str, List[int]) -> () ! {…}` |
| `fs_read_chunk` | `(int, int) -> List[int] ! {…}` |
| `fs_write_chunk` | `(int, List[int]) -> () ! {…}` |
| `net_read_bytes` | `(int, int) -> List[int] ! {…}` |
| `net_write_bytes` | `(int, List[int]) -> () ! {…}` |

`grep -n "Prim::Byte" crates/wolf_sema/src/check.rs` returns eleven
hits and **not one of them is in the builtin signature table** — they
are the cast kinds, the operator widenings, the diagnostic arms and one
column-type mapping. There is **no byte-typed builtin in the language.**

Every one of std's sixteen byte-tier functions is a thin wrapper over
one of those eight builtins — `read_bytes` is `fs_read_bytes(path)?`,
`from_str` is `s.bytes()`, `to_str` is `str_from_utf8(b)?` — so a
signature moved to `List[byte]` cannot be a rename. It has to CONVERT,
elementwise, against a builtin on the other side. And the ledger is
cumulative, so the list it converted FROM stays charged for the
region's life (`[mem.region.account.1]`: "nothing is ever subtracted
while the region lives"). Measured, at every size, in F-0104's fourth
column:

| shape (65,536 bytes) | checked | native |
|---|---|---|
| `fs.read_bytes` today (`List[int]`) | 1,048,576 (16.0x) | 1,048,560 (16.0x) |
| **the same reader substituted to `List[byte]`** | **1,114,112 (17.0x)** | **1,179,680 (18.0x)** |

**A substituted std reader charges MORE than the reader it replaces, on
both tiers, at every size, at exactly the io sites #203 was filed
about.** That is not a tuning question. It is the arithmetic of a
cumulative ledger plus a producer of the wrong type.

### The one shape that could have won, and why it does not

`std.bytes.from_str` is the single place std could plausibly have
avoided the intermediate, because `s.bytes()` in a CONSUMED position is
s77's borrow rather than a materialization — so
`for b in s.bytes() { out.push(b as byte) }` should allocate exactly
one list, the byte one. Measured over a 65,536-byte `str`:

| shape | checked | native |
|---|---|---|
| `from_str` today (`s.bytes()` bound and returned) | 1,048,576 | 1,048,560 |
| `from_str` as a walk building `List[byte]` | **1,114,112** | **131,120** |
| `to_str`'s round trip (walk to `List[byte]`, widen back for `str_from_utf8`) | 2,162,688 | 1,179,680 |

**Natively it works exactly as hoped — 131,120, a clean 8x win. On the
checked machine it REGRESSES**, and the reason is F-0107 below. So the
one substitution with a real win is a win on one tier and a loss on the
other, which makes it a per-tier bet rather than an improvement, and
this library does not ship those.

### What std did instead, and what would change the answer

**Nothing is worked around and nothing is built** — the sc32/sc33
posture for the third sprint running, and for the same house rule
(sc00: a gap in what the language can express is a finding, never a
workaround invented here). Inserting a conversion loop at every byte
boundary in std IS a workaround, it is measurable, and the measurement
says it costs more than it saves. The sixteen signatures keep their
form, so the substitution stays a rename for whoever gets to make it.

**The one change that makes target 2 a pure substitution is upstream
and small**: move those eight builtin signatures from `List[int]` to
`List[byte]`. Then `fs.read_bytes` is a rename, `from_str` is
`s.bytes()` unchanged, `to_str` is `str_from_utf8(b)?` unchanged, every
call site keeps its shape, and the 16x becomes 2.0x/1.0x with no
intermediate anywhere. Until then the honest reading of the landing is:
**the type is right, it is measured, and the library cannot reach it.**

Filed as **wolf-lang#231**, with the std-side numbers posted to #203's
thread (`#issuecomment-5519440224`); sc35 or the sprint after the
builtins move takes the substitution as the rename it was always
designed to be.

## F-0107 — the checked machine charges 16x for a CONSUMED `s.bytes()` view where native and lupin charge nothing

Found while measuring F-0106's one hopeful shape, and it is a finding
on its own because it is invisible to every gauntlet: no row in this
tree prints a ledger count (`[mem.region.account]` forbids it — the
units are per-tier), so this divergence cannot move a verdict and will
not be caught by the differ.

The probe is four lines: walk a 65,536-byte `str`'s bytes inside a
fresh region, summing them, allocating nothing.

```wolf
region r {
    for b in s.bytes() {
        sink = sink + b
    }
    out = region_bytes(r)
}
```

| lane | `region_bytes(r)` |
|---|---|
| native | **0** |
| lupin 0.1.23 | **0** |
| wolf `--checked` | **1,048,576** — 16.0x the payload |

s77's rule is that `s.bytes()` CONSUMED on the spot is the receiver's
own `{ptr, len}` and materializes only where the list must outlive the
expression; `for b in s.bytes()` is the canonical consumed position and
the one `std.bytes`' header teaches ("a caller who only wants to walk
should write `for b in s.bytes()`"). **The native tier and the
reference interpreter both honour it in the ledger. The checked
machine's shadow model does not** — it charges a full `List[int]`,
`16 x payload` exactly, for a walk that allocates nothing on the tier
that ships.

Three reasons this matters beyond the curiosity:

1. **It is the reason F-0106's best case fails.** The one substitution
   with a real native win (`from_str` as a walk) reads 131,120 natively
   and 1,114,112 checked, and the difference is precisely this
   1,048,576.
2. **It makes `region r(cap: n)` mis-fire between tiers on the ONE
   idiom std recommends for byte walking.** A cap derived from a
   checked-tier `charged` reading is 16x too generous natively; a cap
   derived natively breaches instantly under `--checked`. D68's own
   advice — derive budgets from measured readings — is sound, and this
   is a case where the two measurements disagree by more than an order
   of magnitude for a program that allocates nothing.
3. **It is the mirror of the `str` gap already in the clause.**
   `[mem.region.account.1]` scopes ONE known ledger blind spot to the
   native tier (str materialization's ambient region is the process
   root). This is a blind spot in the other direction on a different
   tier: the checked machine charging for storage the program never
   takes. Re-measured this sprint against a NEW compiler, the `str`
   half is unmoved — 200 fresh interpolated strings built inside
   `region r { … }` leave `region_bytes(r)` at **0 on all three lanes**,
   `before 0 after 0` byte-identical — so the clause's sentence should
   still either widen or the non-native tiers should charge.

Filed as **wolf-lang#232**.
