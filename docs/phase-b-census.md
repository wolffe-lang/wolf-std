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
