# Phase B census — stdc02-os, opened at sc07

Started at the sc07 pins (wolf trunk `f0da6e6`, lupin 0.1.4). The Phase-A
census (`phase-a-census.md`) answered "what shipped, what is blocked, and
what each upstream fix would unblock" for the core campaign; this file is
its Phase-B counterpart, opened with the first os sprint so that the
sprints after it inherit numbers rather than impressions.

## 1. Headline, sc07

| measure | value |
|---|---|
| modules added | 2 (`std.fs`, `std.io`) |
| free `pub fn` added | **21** (15 in `std.fs`, 6 in `std.io`) |
| public types added | 1 (`std.fs.File`) |
| free `pub fn` in `std/` | **274** (253 + 21) |
| entry tests | **136** (127 + 9) |
| fenced doc examples, extracted and RUN | **232** (211 + 21) |
| named-and-unbuilt in the os tier (reviewed contracts) | **10** |
| findings filed this sprint | 6 (F-0043 … F-0048) |
| findings retired/part-retired this sprint | 4 (F-0015 whole; F-0018, F-0031, F-0036 compiler halves) |

## 2. What `std.fs` and `std.io` deliver

`std.fs` (15 functions and one type): `read_text`, `write_text`,
`append_text`, `exists`, `remove`, `copy_file`, `move_file` · the fd tier `open`, `create`, `read`,
`write`, `close` over `File` · the pure path helpers `join`, `parent`,
`file_name`.

`std.io` (6): `write`, `write_line`, `write_err`, `write_err_line`,
`input_line`, `prompt`.

Two names differ from the sprint contract's list, both for reasons the
contract could not have known:

- **`copy` → `copy_file`.** `copy` is a reserved KEYWORD at this pin (the
  copy operator landed in the language), so `pub fn copy` is `E0008`
  ("all 50 keywords are reserved everywhere, and wolf has no raw
  identifiers"). `copy_file` pairs with `move_file`, which is the better
  name anyway.
- **`read_line` → `input_line`.** A module item that shares an ambient
  prelude name resolves to the builtin under wolfc and to ITSELF under
  lupin — a facade in one machine, unbounded recursion in the other, with
  no diagnostic (F-0047). std does not build on an unruled resolution
  order.

## 3. The os-tier blocked inventory, by blocker

Ten reviewed contracts, every one of them written into a module header
with the finding that would unblock it. This is the table the next os
sprint should read.

### F-0044 — the fs builtin set is nine calls wide · **6 contracts**

`std.fs`: `read_dir` (nothing enumerates a directory — and the book's P1
guided project needs it), `read_bytes`/`write_bytes` (no byte-level io,
which is also why `copy_file`/`move_file` are text operations that refuse
a binary file), `rename` (an ATOMIC move; `move_file` is
copy-then-remove and says so), `create_dir`, `remove_dir`. Metadata
(`size`, `modified`, `is_file`) is named in the same header and counted
with them.

### F-0043 — a multi-tag row cannot be branched on · **1 contract, and a convention**

`std.io.input_all`: a loop over `input_line` cannot distinguish `eof` from
`io`, because a bare-identifier handler binds every tag and a payload
pattern is a wolfc rejection (E0806). The same finding holds
`tests/errors/coarsen_and_chain.lu` at `fail(E0806)` and makes
API-CONVENTIONS §13's row-expectation convention unavailable on the
compiler lanes.

### F-0046 — the io tier's gaps · **1 contract**

`std.io.flush` cannot exist (no builtin), which is why `prompt` documents
that its prompt may appear only when the line is read. The same finding
carries the two rig-facing asks: stdin injection through `conform-run`,
and `--deny-warnings` on the lane this repo runs.

### F-0004 — trait dispatch · **2 contracts**

`io.Read`/`io.Write` as traits, and the buffered reader/writer over them.
Blocked on dispatch, not on the filesystem — the same blocker that holds
16 Phase-A functions.

## 4. The evidence: what runs, and where

136 tests × 3 lanes at these pins:

| lane | run | unsupported | fail(E…) |
|---|---|---|---|
| lupin | **114** | 22 | 0 |
| wolfc `--checked` | **100** (2 of them unstable) | 31 | 5 |
| native | **23** | 108 | 5 |

The wolfc column more than doubled in one pin bump (40 → 100): 51 rows
advanced — 49 because a row raise now executes across a module boundary
(F-0015) or because the builtin `str` surface stopped costing importers
their lane (F-0018's compiler half), and 2 because two tests were fixed to
spell `else copy empty` — and the nine new fs/io rows land on top. The
five `fail` rows are held rejections, each with a finding behind it:
`errors/coarsen_and_chain.lu` (E0806, F-0043 — new this sprint),
`list/mutate_while_iterating.lu` (E1001, F-0014),
`range/is_empty.lu` (E0301, F-0030),
`x/option_flatten/flatten_propagate.lu` (E0201, F-0039) and
`x/option_expect/expect_trap.lu` (E0401, F-0040). A rejection held as a
ledger row is a finding that cannot rot.

Two rows are `unstable(run|unsupported)` on the wolfc lane: the checked
lane answers the same program two ways at random (F-0048), so the ledger
records both outcomes rather than claiming one and failing CI at random.
The vocabulary is new this sprint and should stay rare — a nondeterministic
verdict is a conformance defect, not a lane.

All nine sc07 tests are `unsupported` under lupin, by design and not by
gap: the interpreter has no filesystem, no line read and no stderr
writer. The rig records that posture once (`CONTRIBUTING.md`, the sc07
section of `tests/ledger.toml`) rather than in nine file headers.

## 5. What Phase B inherits from Phase A's biggest prize

F-0018's compiler half retiring makes 30 census-blocked functions
WRITABLE for one lane — 15 in `std.str` (`find`, `rfind`, `split`,
`split_once`, `ends_with`, `replace`, `get`, `bytes`, …), 9 in
`std.bytes` (the whole module), 2 in `std.json`, and the rest scattered.
Nothing in sc07 spends that prize: it belongs to a `std.str`/`std.bytes`
sprint, and it comes with a decision the fan-out will have to make, which
is why it is recorded here — those functions would run on the compiler's
checked lane and be `unsupported` under lupin until the interpreter's
`str` subset catches up (lupin 0.1.4 has no `get`, `find`, `rfind`,
`split` or `ends_with`). Phase A's rule was "the reference machine runs
it"; Phase B's os tier had to relax that to "at least one lane runs it,
honestly". Whether `std.str` may do the same is the question the next
sprint inherits — with the sc07 precedent (a capability the machine
genuinely lacks) as the narrow reading, and "lupin is simply behind" as
the wide one this file recommends against.

## 6. sc08 — `std.net`, and the native column's flip

Re-measured at the sc08 pins (wolf trunk `13b811f`, lupin 0.1.5).

| measure | sc07 | sc08 |
|---|---|---|
| modules in `std/` | 27 | **28** (`std.net`) |
| free `pub fn` in `std/` | 274 | **284** (+10) |
| public types | — | +2 (`net.Listener`, `net.Socket`) |
| entry tests | 136 | **144** (+8) |
| fenced doc examples, extracted and RUN | 232 | **242** (+10) |
| named-and-unbuilt in the os tier (reviewed contracts) | 10 | **17** (+7 in `std.net`) |
| findings filed | 6 (F-0043…F-0048) | **6** (F-0049…F-0054, issues #45–#50) |
| findings retired | 4 | **3 whole** (F-0018's interpreter half, F-0031, F-0048) |

### What `std.net` delivers

Ten functions and two types: `listen`, `port`, `accept`, `connect`, `read`,
`write`, `close`, `close_listener` over `Listener`/`Socket`, plus the pure
address helpers `endpoint` and `loopback`. Row vocabulary
`{refused, timeout, closed, utf8, io}`, adopted verbatim from the s39 builtin
tier. Eight tests: an echo round trip that accepts twice on one listener,
a stream-in-pieces read, three row litmuses (`refused`, `closed`, `io`), the
`take`-consumed-close rejection (`fail(E1001)`), the comptime refusal
(`fail(E0701)`), and the pure helpers.

Two names differ from what the sprint contract listed, both measured rather
than chosen:

- **`read_all` is NOT shipped.** Written, tested, withdrawn: the loop must
  stop on `closed` and re-raise the other three tags, and a handler's `match`
  matches its first arm for every tag on the executing lane (F-0052). §14's
  refusal rule applies in the same words it used for `std.io.input_all`.
- **`write_all` is not shipped either, and that one is good news**: the
  builtin write is complete, so a short-write loop would document a hazard
  this surface does not have. The name is reserved.

### The lane table at these pins

144 tests × 3 lanes:

| lane | run | unsupported | fail(E…) |
|---|---|---|---|
| lupin | **117** | 27 | 0 |
| wolfc `--checked` | **109** | 30 | 5 |
| native | **66** | 73 | 5 |

The native column is the story: 23 → 64 on the sc07 test set (+41 rows), plus
the two net rejection witnesses. #40 landed native `str`, `List` and the fs/io
builtin tier together, so `std.fs` and `std.io` have two executing lanes now
and F-0026's capability map is down to generics, `trait`/`enum`/`impl`
modules, `Map`, `const` declarations and the `net` tier. No row anywhere is
`unstable(...)` any more (F-0048 closed).

### The os-tier blocked inventory, by blocker (updated)

- **F-0049 — the net builtin tier · 5 contracts**: deadline helpers
  (`set_deadline`/`read_timeout` — the `timeout` tag is declared and
  unreachable), `shutdown`, `peer_address`/`local_address`, UDP, and name
  resolution as an operation.
- **F-0052 — no tag discrimination · 2 contracts, both loops**:
  `std.net.read_all` and `std.io.input_all`. The blocker changed shape between
  sprints (rejection → silent wrong answer) and blocks the same two functions.
- **F-0050 — no byte-level socket read · 1 contract**: `read_line` over a
  socket and any line protocol above it. The buffered-reader half of the
  problem is solved (a mutable `str` field through a `mut` parameter works on
  both executing lanes); the fill is not.
- **F-0044 (6), F-0046 (1), F-0004 (2)** — unchanged from sc07's table.

### What Phase B no longer inherits

§5 above left the next `std.str` sprint a decision: those 30 census-blocked
functions would run on the compiler's checked lane and be `unsupported` under
lupin, so was "at least one lane, honestly" allowed outside a genuine
capability gap? **The question is moot.** lupin 0.1.5 has the whole builtin
`str` set, so those functions run on every lane that has `str` at all — which
is now all three. Phase A's rule ("the reference machine runs it") stands
unrelaxed, and the narrow reading of §14's capability posture — an
implementation that genuinely lacks a capability — remains the only reading
this repo has ever needed.

## 7. sc09 — the F-0018 prize, spent

Re-measured at the sc09 pins (wolf trunk `8321aba`, lupin 0.1.6). This is
the sprint §5 was written for: the 30 census-blocked functions §3 of the
Phase-A census grouped under "the boundary primitive", claimable at last on
every lane.

| measure | sc08 | sc09 |
|---|---|---|
| modules in `std/` with code | 27 | **28** (`std.bytes` had none) |
| free `pub fn` in `std/` | 284 | **314** (+30) |
| `std.str` functions | 16 | **37** (+21) |
| `std.bytes` functions | 0 | **9** (+9) |
| entry tests | 144 | **160** (+16) |
| fenced doc examples, extracted and RUN | 242 | **272** (+30) |
| findings filed | 6 (F-0049…F-0054) | **3** (F-0055…F-0057, issues #56–#58) |
| findings retired | 3 | **1 whole** (F-0018's last two clauses minus `char`; see below) |

### The census flip, exactly

The blocked block was **30 functions**: 15 in `std.str`, 9 in `std.bytes`
(the whole module), 2 in `std.json`, 1 each in `std.strbuf`,
`std.unicode`, `std.fmt` and `std.hex`. sc09 lands **20 of the 30**:

- **`std.str`: 12 of 15.** `get`, `find`, `rfind`, `count`, `split`,
  `split_once`, `rsplit_once`, `ends_with`, `strip_suffix`, `replace`,
  `replacen`, `bytes`. Still contracts: `chars`, `to_list_chars`,
  `graphemes` — all three need a `char` type or segmentation tables, not a
  primitive (F-0018's surviving clause).
- **`std.bytes`: 8 of 9.** `len`, `is_empty`, `at`, `slice`, `find`,
  `starts_with`, `ends_with`, `from_str`. Still a contract: `to_str`,
  blocked on the missing bytes→str materialization (F-0057, new).
- **The other 10 belong to other modules' sprints**, and the sprint that
  takes each one will find the blocker gone for four of them: `json.parse`
  and `json.unescape` (a scanner over arbitrary text is writable with
  `get`+`find`), `fmt.truncate_to` (boundary-safe truncation is `get`), and
  `hex.encode(str)` (`s.bytes()`). The remaining two —
  `strbuf.push(c: char)` and `unicode.char.code` — wait on `char` with
  `std.str`'s three.

**Ten functions land that the census never named**, because the primitive
made a family obvious once it existed: `str.find_all`, `splitn`, `rsplit`,
`starts_with_any`, `ends_with_any`, `char_count`, `char_offsets`,
`code_points`, `is_ascii`, and `bytes.is_utf8`. Two of them are interim
faces of blocked contracts — `code_points` IS `to_list_chars` in
`std.unicode`'s `int` currency, and `char_offsets` is the offset half of
`chars` — so the `char` flip will be a signature change with identical
bodies, exactly as `std.unicode` planned its own.

**Padding is not here and will not be.** The sprint contract lists
"pad/center/justify" under `std.str`; those are `std.fmt`'s
(`pad_left`/`pad_right`/`center` + `_with` variants, shipped sc05), and §1
allows one home per concept. `std.str`'s header says so rather than
growing a second width family.

### The lane table at these pins

160 tests × 3 lanes:

| lane | run | unsupported | fail(E…) |
|---|---|---|---|
| lupin | **133** | 27 | 0 |
| wolfc `--checked` | **125** | 30 | 5 |
| native | **82** | 73 | 5 |

Every one of the 16 new rows is `run` on all three lanes — the first
all-three block a sprint has landed, and the reason is structural rather
than lucky: this code is monomorphic functions over `str`, `int`, `bool`
and `List[int]`/`List[str]`, and the four things that darken a column
(generics, `trait`/`enum`/`impl` modules, `Map`, `const`) are absent from
it. A capability module cannot say that; a text module can.

### What sc09 changed about the rig

`cargo xtask std-test` and `doc-examples` now **deny warnings**: a
non-empty `warnings` array in the observation record is RED, on every lane
that reports one. `conform-run` still rejects `--deny-warnings` (F-0046,
re-verified at this sha), so the rig denies them itself — and there are two
reporting lanes now, because lupin 0.1.6's lint wave populates the array
per `[proto.record.warn]` where sc08 had wolfc alone. It found three real
doc bugs on its first run (four `0.0 - 1.0` sites in `std.fmt.decimal`'s
`parse_float` example and two `0.0 - x` sites in `std.math.float`'s
`neg_inf`/`f_min` examples — W0402, the lint sc08's guide entry was written
about), all now spelled with unary minus. F-0053's open half stands: the
array covers the ENTRY file only, so a warning inside a std module body is
still invisible from here.

### The os-tier blocked inventory (unchanged, plus one)

F-0049 (5), F-0052 (2), F-0050 (1), F-0044 (6), F-0046 (1), F-0004 (2) all
stand as sc08 recorded them. **F-0057 joins them with 1**
(`std.bytes.to_str`), and it is the one to watch: `fs_read_bytes` (F-0044)
and `to_str` unblock each other's usefulness, so the sprint that lands
either should land both.

## 8. sc10 — the nursery opens, and three builtin tiers arrive at once

Re-measured at the sc10 pins (wolf trunk `e94b879` — the s40 + s70 + s69
wave — and lupin **held** at 0.1.6). This is the first sprint whose surface
comes almost entirely from ONE upstream merge: s40 landed fourteen builtins
in three families and sc10 wraps three of them.

| measure | sc09 | sc10 |
|---|---|---|
| modules in `std/` with code | 28 | **31** (`std.time`, `std.env`, `std.x.json`) |
| free `pub fn` in `std/` | 314 | **359** (+45) |
| entry tests | 160 | **175** (+15) |
| fenced doc examples, extracted and RUN | 272 | **317** (+45) |
| findings filed | 3 (F-0055…F-0057) | **4** (F-0058…F-0061) |
| findings retired | 1 | **1** (F-0052, closed upstream by s70) |
| nursery residents | 5 | **6** (`x.json`, D31's named first tenant) |

### What landed

- **`std.time`, 24 functions.** Six reach the clock (`now`, `unix_ms`,
  `sleep`, `elapsed`, `now_iso8601`, `unix_iso8601`, capability `Clock`);
  eighteen are pure — the `Instant`/`Duration` algebra and `to_iso8601`, an
  exact RFC 3339 renderer written over Hinnant's `civil_from_days` with the
  floor-division correction spelled out rather than assumed.
- **`std.env`, 8 functions.** `args`, `arg`, `get`, `has`, `vars`, `set`
  (capability `env`) plus the two pure `K=V` cutters. This is the module
  bs10 asked for: a project can stop holding its input as data.
- **`std.x.json`, 11 functions.** `is_valid`, `get`, `type_name`, `len`,
  `has`, the typed reads `str_at`/`bool_at`/`is_null_at`/`int_at`, and the
  two pure path builders. Capability-free.
- **Two of the four census rows sc09 re-owned**: `std.fmt.truncate_to` and
  `std.hex.encode_str`, both three-lane, both one small body over a
  primitive that landed at s37. `std.fmt` now has no blocked contract at
  all.

### What did NOT land, and why — the other two census rows

sc09 re-owned four rows on the strength of F-0018 retiring, and predicted
all four were writable. Two were. The other two are `std.json`'s, and the
sprint that took them found a DIFFERENT finding standing behind the one
that retired:

- **`json.parse` is blocked by F-0037, not by F-0018.** Its signature is
  `-> Value ! {syntax, deep}` — an enum returned through an error row, which
  takes the miss path on every call, re-measured unmoved at this pin. The
  scanner is writable; the SIGNATURE is not, and neither is `json.get`/`at`,
  withdrawn at sc05 for the same reason.
- **`json.unescape` is blocked by F-0057**, not by F-0018 either. The walk
  is writable and its OUTPUT is not: decoding `\u00e9` means building `"é"`
  from the number 233, and nothing in the language turns a scalar into a
  `str` — no `from_utf8`, no `char`, no `strbuf.push_byte`. An ASCII-only
  `unescape` would be a border post that refuses text, which §9 forbids.
  Its tag also changes in the contract, from the interim `boundary` to
  `parse`: the boundary condition is genuinely gone, and what is left is
  ordinary bad data.
- **`json.escape`'s one refusal is the only one of the four that is simply
  unwritten**, and deliberately: making `escape` total changes
  `stringify`'s row from `{boundary, deep}` to `{deep}`, which ripples
  through every test and doc example in `std.json`. That is a signature
  change worth making once, beside the function that turns this module into
  a reader.

The census correction worth recording, and it is the most transferable
thing in this sprint: **"the blocker retired" is not "writable"** until
every finding on the SIGNATURE has been re-measured, not just the one that
was in the way last time. sc09 predicted four writable functions from one
retirement and got two — and the two it missed are blocked by findings that
were open, filed and visible the whole time (F-0037 since sc05, F-0057 since
sc09 itself). The re-measurement is cheap; the prediction was the expensive
part.

### One function written and withdrawn

`std.x.json.float_at` (F-0061). `std.fmt.decimal.parse_float` is
`unsupported` on both compiler rungs and the checked tier is
`std.x.json`'s only executing lane, so the function would have had zero
lanes, no runnable test and no fenceable doc example. It is a contract in
the module header with a four-line body waiting for either rung. The rule it
sets is in API-CONVENTIONS §14 and in the nursery register: check a
resident's dependencies against its own lanes BEFORE writing the function.

### The lane table at these pins

175 tests × 3 lanes:

| lane | run | unsupported | fail(E…) |
|---|---|---|---|
| lupin | **140** | 35 | 0 |
| wolfc `--checked` | **140** | 30 | 5 |
| native | **93** | 77 | 5 |

Fifteen new rows, and no existing row moved — the first three-wave pin bump
that advanced nothing already recorded, which is exactly what the wave was:
new builtin families std had never wrapped (s40), a fixed shape std had
refused to write (s70), and lints std already obeyed (s69).

The new rows split three ways, and the split is the sprint's whole lane
story: **five are three-lane** (both pure test files, both census rows, and
the F-0052 witness), **six are two-lane** (every capability test — lupin has
no `time_*`/`env_*`, and that is a pin DRIFT rather than a design refusal),
**one is two-lane the other way** (`x/json/path_helpers.lu`: lupin and
checked, native dark), and **three are one-lane** (the json queries, checked
only). A pure member always buys a lane, and separating it into its own test
file is now house habit.

### The blocked inventory

Unchanged from sc09 except for the movement above: F-0049 (5), F-0050 (1),
F-0044 (6), F-0046 (1), F-0004 (2), F-0057 (1). **F-0052's two (`io.input_all`,
`net.read_all`) are no longer blocked** — the finding closed — and are
unwritten pending a sprint that owns them; their module headers still say
"blocked" until one does, which is a debt this census names so it cannot sit
quietly. **F-0058 adds 1** (`x.json.keys`, blocked on a query tier with no key
enumeration) and **F-0061 adds 1** (`x.json.float_at`).

## 9. sc11 — std.process, and the backlog four closed findings unblocked

Pins: wolf trunk `0b4e79c` (five waves: s71, s72, s51, s41, s73, plus the r01
identity release — and one commit BEHIND trunk's tip, which fails the ritual's
first gate, F-0063), lupin 0.1.8 at conformance pin `26fa98e`.

| measure | sc10 | sc11 |
|---|---|---|
| modules under `std/` | 33 | **34** (`std.process`) |
| `pub fn` in `std/`, nursery excluded | 328 | **342** (+10 process, +3 net, +1 io) |
| `pub fn` including `std/x/` | 359 | **373** |
| public types in the os tier | 3 (`File`, `Listener`, `Socket`) | **5** (`Command`, `Child`) |
| entry tests | 175 | **183** |
| findings filed | F-0056…F-0061 | **F-0062 … F-0069** (eight) |
| findings CLOSED | 1 (F-0052) | **4** (F-0014, F-0043, F-0055, F-0056) |

(The count method is `grep -c "^pub fn"` over the tracked `.lu` files, stated
because earlier census rows in this file used a different one and a total that
cannot be reproduced is not a measure. The DELTA is what matters and it is
+14.)

### What `std.process` delivers

Ten functions and two types: `command`, `from_argv`, `push_arg`, `argv` (the
pure builder) · `start`, `wait`, `kill`, `run` (the trio, plus the
spawn-and-wait pair) · `is_success`, `exit`. Rows are the builtin tier's
verbatim: `{not_found, denied, io}` starting, `{signal, io}` waiting, `{io}`
killing — `signal` being new to §12's inventory.

Two names differ from the obvious ones, both for reasons the contract could
not have known: **`start`, not `spawn`** (`spawn` is one of the 50 keywords —
it opens a task and a `spawn proc` — so `pub fn spawn` is `E0008`, F-0062), and
**`push_arg`, not `arg`** (`std.env.arg(i)` already means "the argument at i",
and a program forwarding its own argv to a child imports both modules).

### The backlog, verified then written

- **`io.input_all`** (sc07's worked refusal) and **`net.read_all`** (sc08's,
  written and withdrawn inside that sprint) — both shipped, both four lines
  and one handler. Neither became writable because std lowered its standard:
  both needed a loop that stops on ONE tag and re-raises the others, and both
  waited for s70's match tier (F-0052) and s71's payload-pattern ruling
  (F-0043). §14's refusal rule now has its first two retractions and a third
  clause: ship the function in the sprint after the finding closes.
- **`net.write_line` and `net.read_lines`** — the line protocol's two halves,
  in the shape available at this pin: send one line, or read the whole stream
  and split it. A single-line incremental read is still blocked (F-0050: the
  buffer fill is a byte-count read and a chunk can split a code point), and
  `read_lines` says on itself that it is not a substitute.
- **`bytes.to_str`** — still a contract, re-documented after four PROBES
  rather than an assumption: `str.from_utf8` does not resolve,
  `strbuf.push_byte` is not a method, `'h'` is `E0101` at the lexer, there is
  no `bytes_to_str`. wolf-lang#58 is open; F-0057 is unmoved for a third
  sprint.

### The ripple, every flip listed

Seven rows moved and three files were rewritten. Every movement is an advance:

| test | lane | was | is | why |
|---|---|---|---|---|
| `time/monotonic.lu` | lupin | unsupported | run | lupin 0.1.8 has `time_*` |
| `env/variable_round_trip.lu` | lupin | unsupported | run | lupin 0.1.8 has `env_*` |
| `errors/coarsen_and_chain.lu` | wolfc | fail(E0806) | run | s71 ruled `else \|Tag(p)\|` |
| `errors/coarsen_and_chain.lu` | native | fail(E0806) | run | same |
| `list/mutate_while_iterating_trap.lu` | lupin | run (exit 0) | run (trap) | s72's D40 |
| `list/mutate_while_iterating_trap.lu` | wolfc | fail(E1001) | fail(E1013) | same |
| `list/mutate_while_iterating_trap.lu` | native | fail(E1001) | fail(E1013) | same |

The three rewrites carry no row movement because the ANSWER changed while the
depth did not: `split_empty_separator_trap.lu` and
`replace_empty_pattern_trap.lu` lost their traps, their guards and their
`_trap` names (s71 defined the empty needle, `[mem.str.empty]`), and
`repeat_negative_trap.lu` changed kind from `bounds` to `assert`
(`[mem.str.repeat]`) — which turned red inside a minute of the bump, because
sc09 wrote the file instead of the sentence.

**s73 moved no row at all, and that is the honest report.** Native
concurrency is the wave this sprint was told to ripple, and std wraps no
concurrency surface at this pin — no task facade, no channel helpers, nothing
that names `spawn` (which is a keyword, F-0062). So the ripple is entirely
DOCUMENTARY and it is real: every sentence in this repository that budgeted a
concurrent program to the interpreter lane is now false, `std.time.sleep`'s doc
had to say that a thread-blocking sleep now stalls sibling TASKS on three
rungs rather than one, and `std.process`'s "kill it from another task" advice
became writable advice instead of a one-machine trick. Zero rows, three
paragraphs, and the alternative — leaving the claims standing — is how a
document starts lying.

Two rows did NOT move and both are worth naming. `env/args_and_vars.lu` stays
lupin-dark because ONE builtin of the five is missing — `env_vars` "does not
resolve" at 0.1.8 while `env_args`, `env_get`, `env_set`, `os_cwd` and
`os_exit` all run (F-0070, measured one call at a time rather than inferred
from the family, which is how a wrong sentence nearly reached this file).
The three `x/json/*` rows stay dark for a NEW reason: lupin 0.1.8 declines
the json surface by DESIGN ("rather than risk a second, guessed RFC 8259
reading"), so what sc10 recorded as drift is now a posture. A drift closes on
a release; a posture closes on a decision.

### The lane table at these pins

183 tests × 3 lanes:

| lane | run | unsupported | fail(E…) | (sc10) |
|---|---|---|---|---|
| lupin | **144** | 39 | 0 | 140 / 35 / 0 |
| wolfc `--checked` | **149** | 30 | 4 | 140 / 30 / 5 |
| native | **97** | 82 | 4 | 93 / 77 / 5 |

The `fail` column shrank by one on both compiler rungs because
`errors/coarsen_and_chain.lu` stopped being a held rejection — the only time
this ledger has recorded a rejection turning into a run.

Eight new rows, and the split is the sprint's lane story: **two are two-lane
on the interpreter side** (`process/builder.lu` and `process/exit_code.lu` —
the pure builder and `os_exit`, which lupin has), **one is two-lane on the
compiler side** (`io/input_all_empty.lu`: both rungs, lupin has no stdin),
**two are one-lane** (`process/not_found_row.lu`,
`process/forged_handle_io_rows.lu` and `net/read_all_and_lines.lu` — checked
only), and **two are rejection witnesses that run on both compiler rungs**
(`process/use_after_wait.lu`, `process/comptime_refuses.lu`), because a
rejection never reaches lowering.

### The blocked inventory

F-0049 (5), F-0050 (1 — now `net.read_line` alone), F-0044 (6), F-0046 (1,
re-shaped: `io.read_bytes` rather than `input_all`), F-0004 (2), F-0057 (1),
F-0058 (1), F-0061 (1), and **F-0065 adds 6** (`process.output`,
`stdin_text`, `env_for`, `current_dir`, `try_wait`, `wait_timeout`, `pid`
being the seventh named in the header). F-0066 adds no contract and one debt:
the module's central claim — a child's exit code comes back — has no portable
witness in this repository at all.

## 10. sc12 — the byte view, and a wave that moves no row

Pins: wolf trunk `f8dca42` (ten commits past sc11's `0b4e79c`: s74, s53, s75,
s78, s76, s77 and the rt test gating), lupin **0.1.10** (skipping 0.1.9), its
own conformance pin `613c3dc` being an ANCESTOR of the wolf sha — the
narrowest two-upstream drift this repo has recorded, and the first one
measurable as a commit distance rather than as a list of waves. Both ritual
gates green in a clean scratch clone, first attempt.

**This repository's own gate needed two attempts, and the reason is a
finding.** The first `cargo xtask ci` run went RED on a single row —
`tests/fmt/decimal/shortest_round_trip.lu` timed out at the rig's 60-second
per-test ceiling — after three `std-test` runs had passed it in the same
hour. The re-run was green and so was the run after it, and the flake is
load: that test takes 28-35 s under lupin 0.1.10 on a quiet machine. Chasing
it produced F-0074 (`List.push` is O(n) per push on the interpreter lane),
which is the largest thing this sprint found and was not in its contract.
The attempt count is stated here for the same reason the pin ritual states
its own: a gate that needed a re-run is information, and a sprint that only
reports the green run is reporting half of what happened.

| measure | sc11 | sc12 |
|---|---|---|
| modules under `std/` | 34 | **34** |
| `pub fn` in `std/`, nursery excluded | 342 | **342** |
| `pub fn` including `std/x/` | 373 | **373** |
| function BODIES rewritten | — | **8** (5 public, 3 private, +1 new private helper) |
| entry tests | 183 | **185** (+2) |
| doc-example blocks, extracted and RUN | 317 (sc10) | **331** |
| findings filed | F-0062…F-0070 (eight) | **F-0071 … F-0074** (four) |
| findings CLOSED | 4 | **2** (F-0037, F-0032 — both upstream, both lupin's half) |

**Zero functions added and zero signatures changed**, which is the first time
a sprint here can say that. The subject was not new surface; it was what s77
made cheap.

### What s77 gave, and what std was allowed to take

`s.bytes()` is the receiver's own `{ptr, len}` pair now — bit-identical to the
`str` and to every zero-copy subslice `trim`/`split`/`get` already returned —
read in place wherever the call is CONSUMED. `s[a..b]` and `s.get(a..b)`
stopped calling the runtime in the same wave. The compiler's consuming set is
seven positions: iteration, indexing, and `len`/`count`/`is_empty`/`get`/
`first`/`last`.

**std may use two of them** (F-0071, filed as wolf-lang#85). Measured, one
program per position:

| shape | lupin | checked | native |
|---|---|---|---|
| `for b in s.bytes()` | run | run | run |
| `s.bytes().len` | run | run | run |
| `s.bytes()[i]` | run | **unsupported** — indexing outside the modelled surface | run |
| `s.bytes().get(i)` | run | **unsupported** — List method on a temporary | run |
| `s.bytes().count()` | run | **unsupported** — same | run |
| `s.bytes().first()` | **unsupported** — no `first` in this std subset | **unsupported** — same | run |
| `let bs = s.bytes()` then `bs[i]` | run | run | run (materializes) |

std cannot spend an execution lane on a performance shape, so every rewrite
below is on the two-position subset. The last row is the one that makes this a
gap rather than a design: the checked tier indexes a byte LIST perfectly well,
and what it does not model is the temporary.

### The eight bodies, and what changed shape

| function | was | is |
|---|---|---|
| `str.char_count` | `let bs = s.bytes()` + `while` + `bs[i]` | `for b in s.bytes()` |
| `str.is_ascii` | same, with an early `return false` | `for` over the view, returning out of it |
| `str.char_offsets` | bind + indexed walk | `for` + an offset counter |
| `str.code_points` | bind + RANDOM-ACCESS walk (`bs[i+1..i+3]`, advance by width) | one-pass state machine: a lead byte sets a pending count, each continuation folds six bits |
| `hex.encode_str` | `encode(s.bytes())` — a materializing ARGUMENT | `for` over the view, `byte_digits` per byte |
| `hex.digit_of` | up to 32 `starts_with` probes against an owned alphabet | one byte, three ASCII range tests |
| `fmt.digit_of` | up to `2 * radix` probes (72 at radix 36) | one byte, three range tests, one radix bound |
| `base64.value_of` | 64 one-byte slices + `starts_with` | one byte, then a `for` over the alphabet's bytes comparing numbers |

Two of those are more than a mechanical substitution and both are worth
naming. `code_points` lost its lookahead: a view cannot be indexed on the
checked lane, so the decoder had to become a machine that carries state
forward instead of reading ahead — and the result is shorter than what it
replaced. The three digit probes lost sc03's INVERSION, which is a small
piece of this repository's history retiring: "probe the input with a literal
you own, never slice the caller's string" was the rule that made `parse_int`,
`hex.decode` and three base64 decoders total over arbitrary UTF-8 while the
language had no byte accessor. The totality survives with a shorter argument —
a byte is a byte, and a multi-byte character's lead byte (195 for `é`) is
simply not a digit — and the probe loop it required is gone.

### What did NOT get rewritten, and why

- **`std.bytes`, all nine functions.** Their first parameter is `List[int]`,
  and an argument is exactly the position s77 materializes in, so
  `bytes.is_utf8(bytes.from_str(s))` copies `s` where `str.char_count(s)`
  walks it. The difference is the parameter, not the implementation, and the
  library cannot fix it from its side: F-0072 (wolf-lang#86) is the ask —
  the `Bytes` type this repo has documented as an interim since sc05, or a
  mode that lets a callee borrow `{ptr, len}`.
- **`str.bytes`** still allocates, and its doc now says that this is a
  statement about its SIGNATURE rather than about the pin: it RETURNS the
  list, which is a materializing position and must be, because the list
  outlives the expression.
- **`fmt.truncate_to`** keeps its `get` walk. The byte at the caller's width
  would answer the boundary question in one load, but §9's sc09 rule says the
  recoverable slice is for offsets the CALLER named, and that rule is
  doctrine rather than a cost problem. Recorded so the next sprint does not
  re-derive the question.
- **`str.find_all`, `splitn`, `replacen`** keep their `find` loops: the
  builtin search is the implementation's, and slicing at the offset it
  returned stopped calling the runtime in the same wave with no source change.

### The one number this sprint can show, and where it came from

The sprint was told to measure nothing, and the rig cannot see an
allocation, so no benchmark was written. One number arrived anyway, from a
CI timeout that turned into a bisect (F-0074), and it is worth recording
because it is the only direct evidence here that the rewrites do anything:

`hex.decode` over an 8 192-character hex string, under lupin, same input,
same output (`8192 4096`), the only difference being `digit_of`:

| digit probe | wall |
|---|---|
| the sc03 alphabet probe (up to 16 iterations of slice + `starts_with` + `upper()` + `starts_with`) | **364.6 s** |
| the sc12 byte view (`for` over `s.bytes()`, three range tests) | **3.7 s** |

Roughly 100x, on the lane where the view MATERIALIZES and therefore should
have helped least. The old probe's cost was the per-iteration `.upper()`
allocation and the sixteen one-byte slices, not the search; the new one
reads a byte. Two caveats keep this honest: both versions remain
superlinear in the input for a reason that has nothing to do with either
probe (F-0074: `List.push` is O(n) per push on this lane, and `decode`
pushes a byte per pair), and this is one input on one lane on one machine.
It is an anecdote with a mechanism, not a benchmark, and it is labelled as
one.

**The one rewrite this sprint made and then withdrew** is the other half of
the lesson. `std.fmt.decimal`'s `dval` is the hottest private function in
the library (once per digit, and a subnormal has 750), so it looked like the
best candidate of all. Its only executing lane is the interpreter — the f64
body is `unsupported` on both compiler rungs (F-0026, F-0061) — and there
the view materializes, so the rewrite measured neutral-to-slower against a
probe that was already cheap (no `.upper()`, ten single-byte slices). It was
reverted inside the sprint. **Check which lane runs the module before
rewriting it onto a primitive whose win is native-only**: a one-lane module
whose lane is the interpreter gets nothing from s77, and `std.fmt.decimal`
is the whole class.

### The lane table at these pins

185 tests × 3 lanes:

| lane | run | unsupported | fail(E…) | (sc11) |
|---|---|---|---|---|
| lupin | **146** | 39 | 0 | 144 / 39 / 0 |
| wolfc `--checked` | **150** | 31 | 4 | 149 / 30 / 4 |
| native | **99** | 82 | 4 | 97 / 82 / 4 |

**Every flip, listed, and there are none.** Not one of the 183 rows this repo
carried into the bump moved in any direction — measured before a single std
edit, with the rig green against the sc11 ledger at the sc12 pin — and the
whole difference above is the two rows sc12 adds:

| test | lupin | wolfc | native | why |
|---|---|---|---|---|
| `str/byte_view_walk.lu` | run | run | run | the two portable view shapes, held so they cannot rot |
| `str/byte_view_index.lu` | run | **unsupported** | run | the held refusal behind F-0071 |

A wave that makes existing code faster moves no ledger row by construction:
this ledger measures how DEEP each implementation gets, and s77/s76/s75
changed cost rather than depth. That is worth stating plainly, because a
sprint whose entire subject is a performance primitive and whose ledger is
unchanged looks, from the ledger alone, like a sprint that did nothing. What
changed is in the diff: eight bodies, one of them an algorithm.

### The blocked inventory

F-0049 (5), F-0050 (1), F-0044 (6), F-0046 (1), F-0004 (2), F-0057 (1),
F-0058 (1), F-0061 (1), F-0065 (6) all stand as sc11 recorded them, each
re-measured. **F-0072 adds no contract and one debt** (`std.bytes` is
copy-only at every entry point). **F-0071 adds no contract either**, and
what it costs is a shape rather than a function. **F-0074 adds no contract
and the largest debt on this list**: every `List`-returning function in the
library is quadratic on the reference lane, which is a fact about the
machine and not about the algorithms, and the only thing std could do about
it is the workaround `CONTRIBUTING.md` forbids.

**One block LEAVES the inventory, and it is the biggest one here.** F-0037 —
an enum returned through an error row takes the miss path on every call — is
CLOSED at lupin 0.1.10 (wolf-interp#16), re-measured with the finding's own
reproducer. `std.json.parse`, `json.get` and `json.at` were written, tested
and withdrawn to reviewed contracts on that finding, and the interpreter is
`std.json`'s only executing lane, so the DOM half of json is writable for the
first time. sc12 does not write it: it is not in this sprint's contract, and
sc10's rule applies to itself — "the blocker retired" is not "writable" until
every finding on the SIGNATURE has been re-measured. The sprint that takes the
row owes F-0039 (nested rows) and F-0029 (cross-module enum consumption) a
fresh measurement first. API-CONVENTIONS §11's "no std accessor returns an
enum through an error row" was written as an interim with F-0037 as its exit,
and the exit has arrived.

## 11. sc13 — two closures spent, in opposite directions

Pins: wolf trunk **`4e316ad`** (three waves past sc12's `f8dca42`: s79 bench,
s80 token audit, s81 str equality — the three sc12 named as in flight and
deliberately did not chase), lupin **0.1.11**, whose own conformance pin is
`f8dca42`: the interpreter now names the compiler sha this repository held
one sprint ago, which is the narrowest drift yet and the first time it has
been exactly one sprint. Both ritual gates green in a clean scratch clone at
the sha, **first attempt, exit codes printed** (`cargo test --workspace` = 0,
`cargo run -p xtask -- ci` = 0).

| measure | sc12 | sc13 |
|---|---|---|
| modules under `std/` | 34 | **34** |
| `pub fn` in `std/`, nursery excluded | 342 | **345** (+3) |
| `pub fn` including `std/x/` | 373 | **376** (+3) |
| entry tests | 185 | **189** (+4) |
| doc-example blocks, extracted and RUN | 331 | **334** (+3) |
| reviewed contracts RETIRED into code | — | **3** (`json.get`, `json.at`, `bytes.to_str`) |
| findings filed | F-0071…F-0074 (four) | **F-0075, F-0076, F-0077** (three) |
| findings CLOSED | 2 | **2** (F-0057 upstream; F-0037 spent) |

**Three functions, and every one of them was a reviewed contract somebody
wrote and withdrew.** `std.json.get` and `std.json.at` were written, tested
and withdrawn at sc05; `std.bytes.to_str` has been a contract since sc05 and
was re-probed at four consecutive pins. Nothing here is new design. What
this sprint did was re-measure, then spend.

### The two halves have opposite lane shapes, and that is the finding

| function | lupin | wolfc | native | what decides it |
|---|---|---|---|---|
| `json.get` / `json.at` | **run** | unsupported | unsupported | F-0029 + `Map`: an imported module that produces an enum is refused on both compiler rungs |
| `bytes.to_str` | **unsupported** | run | run | F-0075: `str_from_utf8` is the compiler's prelude only |

One closure was the interpreter's (F-0037, an enum through an error row) and
lands on the lane only the interpreter has; the other was the compiler's
(F-0057, a bytes-to-str primitive) and lands on the lanes only the compilers
have. A reader who takes either half as "std gained a lane" would be wrong in
both directions, which is why the row is drawn this way.

### The re-measurement, before a line was written

sc10's rule — "the blocker retired" is not "writable" until every finding on
the SIGNATURE has been re-measured — applied to itself. sc12 named the two
findings the json sprint owed a measurement, and both were taken first:

- **F-0037** — `fn id(v: V) -> V ! {none} { v }` prints `value path wins 7`
  under lupin 0.1.11. Closed, and now SPENT.
- **F-0029** — unmoved. An enum value crosses a module boundary (and now
  crosses one through a row); an importer's `match` on it does not. That
  costs the getters nothing, because every inspection in `std.json` lives in
  the declaring module by construction.
- **F-0039** — unmoved. `int ! {none} ! {none}` runs under lupin and is
  `fail(E0201)` at parse on both compiler rungs. Neither getter needs a
  second `!`.

### What did NOT land, and it is now a DEBT rather than a wall

`std.json.parse` and `std.json.unescape` are the two contracts left in that
module, and at these pins **neither has a blocker**: `parse`'s was F-0037
(closed), `unescape`'s was F-0057 (closed by s81 — the same primitive
`bytes.to_str` spends). `escape`'s remaining refusal has been writable since
sc09 and rides with them, because making it total changes `stringify`'s row
from `{boundary, deep}` to `{deep}` and that is a signature change worth
making once.

API-CONVENTIONS §14's sc11 clause is what governs: a contract ships in the
sprint AFTER its blocker closes — one sprint of grace and no more. F-0037
closed at sc12's pin, so the getters were owed HERE and are here. F-0057
closes at sc13's pin, so `unescape` (and `parse` behind it) are owed at
sc14, and both module headers say so with the clause named. That is the
whole reason this section exists rather than a note.

### The lane table at these pins

189 tests × 3 lanes:

| lane | run | unsupported | fail(E…) | (sc12) |
|---|---|---|---|---|
| lupin | **148** | 41 | 0 | 146 / 39 / 0 |
| wolfc `--checked` | **152** | 33 | 4 | 150 / 31 / 4 |
| native | **101** | 84 | 4 | 99 / 82 / 4 |

**Every flip, listed, and there are none.** No row this repo carried into the
bump moved in any direction; the whole difference is the four rows sc13 adds:

| test | lupin | wolfc | native | why |
|---|---|---|---|---|
| `json/navigation.lu` | run | unsupported | unsupported | the getters' value side |
| `json/navigation_rows.lu` | run | unsupported | unsupported | `none` and `kind`, one ridden out of `main` |
| `bytes/to_str_border.lu` | **unsupported** | run | run | every UTF-8 rejection class through the facade, plus the 256-byte sweep |
| `bytes/to_str_row.lu` | **unsupported** | run | run | the `utf8` tag, named by the record |

Third sprint in a row with an unchanged existing ledger, and the reason is
the same one every time: this ledger measures how DEEP each implementation
gets, not what it costs, and s79/s80/s81 are a benchmark wave, a miscompile
fix and a lowering change plus one prelude name.

### The blocked inventory

F-0049 (5), F-0050 (1), F-0044 (6), F-0046 (1), F-0004 (2), F-0058 (1),
F-0061 (1), F-0065 (6) stand as sc12 recorded them.

**F-0057 LEAVES the inventory** and takes `std.bytes`' last contract with it:
the module has nine reviewed functions and nine bodies. Its neighbour does
not leave — `fs_read_bytes` (F-0044) is still unwritten, so a byte read has
no producer to hand `to_str`, and the two halves of that pair have now been
waiting for each other for six sprints.

**F-0037 leaves too**, and what it leaves behind is the debt above rather
than a contract: two functions that nothing blocks.

**Three findings arrive and none of them adds a contract.** F-0075 costs one
function one lane. F-0076 costs a SPELLING (`p == q` on two `bool`s is
refused natively) and is the second instance of F-0071's lesson — write the
form that keeps every lane, file the one that does not. F-0077 costs a
sentence in a doc: a pure builtin whose argument is a `List` cannot be
reached at comptime, so `to_str`'s comptime story is "the sandbox has no
objection and the engine cannot get there", measured rather than inferred.
