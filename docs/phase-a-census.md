# Phase A census — stdc01-core, sc01 through sc06

The campaign's closing count, written at the sc06 pins (wolfc trunk
`29a9d9c`, lupin 0.1.4). Three questions, answered with numbers that can
be re-derived from the tree: what shipped, what is blocked and on what,
and what each upstream fix would unblock. The last one is the point — the
fan-out sprints should be able to read this file and know the size of
their own prize.

## 1. Headline

| measure | value |
|---|---|
| free `pub fn` in `std/` | **253** (233 in the facade, 20 in the `std/x` nursery) |
| public methods (`impl Ordering`) | 8 |
| public callables | **261** |
| modules | 31 module files: 26 in the facade (`std.fmt.decimal` and `std.math.float` are nested leaves) + 5 nursery residents |
| modules shipping zero code | 3 (`std.bytes`, `std.deque`, `std.set`) — down from 4 |
| named-and-unbuilt functions (reviewed contracts) | **98** |
| entry tests | **127**, all ledgered, three lanes each |
| fenced doc examples, extracted and RUN | **211** |
| findings filed | **42** — 15 retired/closed, 1 narrowed, 2 part-retired, 24 open |

The planned inventory across the six sprint contracts was **303**
(30 + 68 + 64 + 74 + 48 + 19). Delivered-with-a-body is 253 free
functions; the gap is not slippage but the blocked inventory below, every
item of which is a reviewed contract in a module header with a finding
behind it.

## 2. Per sprint

| sprint | modules | planned | delivered | contracts (blocked) | notes |
|---|---|---|---|---|---|
| sc01 | `option`, `cmp`, `iter`, `testing` | 30 | 14 items at the time (4 fns + enum + 8 methods + 2 traits), `iter` 4, `testing` 5 | `option`'s 6, `cmp.min_of`/`max_of`, `assert(cond,msg)` | `option`'s six LANDED in sc06; the 2-arg assert intrinsic landed upstream |
| sc02 | `list`, `map`, `set`, `deque`, `pool` | 68 | 19 + 13 + 0 + 0 + 2 = **34** (+11 in `x.deque_int`, +6 in `x.list_eq`) | 28 on generic data, 11 on closures, 6 on dispatch, `list.reserve` | two whole modules are contracts; the nursery holds the running proof |
| sc03 | `str`, `strbuf`, `bytes`, `unicode` | 64 | 16 + 8 + 0 + 9 = **33** | 15 + 3 + 9 + 1 = **28**, all on one primitive | the campaign's biggest single blocker, filed as wolf-lang#17 |
| sc04 | `math`, `math.float`, `sort`, `search`, `range`, `rand` | 74 | 18 + 39 + 7 + 11 + 1 + 7 = **83** | 3 on range accessors, 2 range combinators, 10 on closures; 3 delivered-but-non-executing | over plan because constants ship as zero-argument functions (F-0025/F-0026) |
| sc05 | `fmt`, `fmt.decimal`, `json`, `hex`, `base64` | 48 | 17 + 4 + 22 + 3 + 5 = **51** | `fmt.truncate_to`, `json.parse`/`unescape`/`get`/`at`, `hex.encode(str)` | over plan because `json`'s constructors multiplied under F-0029 |
| sc06 | `errors`, `testing`, (`option` landed) | 19 | `errors` 5 + `testing` 12 + `x.testing_text` 1 = **18**, plus the `assert` intrinsic documented = **19** | none of its own; 7 of the 13 assertions are freight | additionally landed sc01's six `option` helpers (4 facade + 2 nursery) |

## 3. The blocked inventory, by blocker

98 named functions have a reviewed contract and no body. Grouped by what
would unblock them — this is the table a fan-out sprint should read.

### wolf-lang#17 — the boundary primitive (F-0018 / F-0035) · **30 functions**

The largest prize in the campaign, and one primitive buys all of it:
`str.get(a..b) -> str ! {none}`, or a byte accessor, or `chars()`.

- `std.str` (15): `find`, `rfind`, `count`, `split`, `split_once`,
  `rsplit_once`, `ends_with`, `strip_suffix`, `replace`, `replacen`,
  `get`, `bytes`, `chars`, `graphemes`, `to_list_chars`
- `std.bytes` (9, the whole module): `len`, `is_empty`, `at`, `slice`,
  `find`, `starts_with`, `ends_with`, `to_str`, `from_str`
- `std.json` (2): `parse`, `unescape`
- `std.strbuf` (1): `push(c: char)` · `std.unicode` (1): `char.code`
- `std.fmt` (1): `truncate_to` · `std.hex` (1): `encode(str)`

It also has a **resolve-level half** that costs lanes rather than
functions: a builtin `str` method anywhere in an imported module makes
every importer `unsupported` at resolve under wolfc. That is why all 13
`std.str`/`strbuf` tests are lupin-only and why sc06's
`assert_starts_with` sits in the nursery.

### wolf-lang#11 — generic data types, s16 (F-0011) · **28 functions**

`struct X[T]` is E0201 in both implementations, so a generic container
cannot be declared at all.

- `std.set` (13, the whole module): `add`, `remove`, `has`, `len`,
  `is_empty`, `clear`, `to_list`, `union`, `intersect`, `difference`,
  `symmetric_difference`, `is_subset`, `is_disjoint`
- `std.deque` (10, the whole module): `push_back`, `push_front`,
  `pop_back`, `pop_front`, `front`, `back`, `len`, `is_empty`, `clear`,
  `to_list` — with `std/x/deque_int` as the running monomorphic proof
- `std.pool` (4): `has`, `len`, `capacity`, `clear` (the builtin exposes
  no liveness probe, length, capacity or iteration)
- `std.list` (1): `reserve`

### c05 — closures · **23 functions**

Not a finding; a campaign dependency. `std.list` 11 (`map`, `filter`,
`fold`, `any`, `all`, `each`, `retain`, `count_if`, `min_by`, `max_by`,
`position`), `std.map` 5 (`each`, `map_values`, `retain`, `merge`,
`entry`), `std.search` 4 (`min_by`, `max_by`, `sum_by`,
`partition_point`), `std.sort` 3 (`sort_by_key`, `sort_unstable`,
`sort_dedup`).

### wolf-lang#5 / #12 — trait dispatch (F-0002 / F-0004) · **8 functions blocked, 16 more non-executing**

Blocked outright: `std.cmp.min_of`, `max_of` (reclassified this sprint —
their original blocker F-0005 retired at the sc03 pin; what stops them
now is `Ord.cmp` dispatch), and `std.list`'s six element-comparing
functions, whose bodies live in `std/x/list_eq` and execute nowhere.

Written but executing NOWHERE for the same reason: `std.sort.sort`,
`sort_by`, `is_sorted_by`, `std.search.binary_search_by`, and seven of
`std.testing`'s thirteen assertions (`assert_eq`, `assert_ne`,
`assert_lt`, `assert_le`, `assert_gt`, `assert_ge`, `assert_contains`).
This is the second-largest prize and the least visible one: 16 shipped
functions become testable the day dispatch runs.

### wolf-lang#24 — range accessors (F-0030) · **3 + 2 functions**

`std.range.contains`, `len`, `clamp_to`; plus `rev()` and `step_by()`
waiting on the iterator combinator surface (wolf-lang#8's successor, with
the `limit`-not-`take` naming already ruled at wolf-lang#16).

### wolf-interp#16 — enum through a row (F-0037) · **2 functions**

`std.json.get` and `std.json.at`, written, tested and WITHDRAWN because
every call missed, including the hits.

### wolf-lang#34, #35 — sc06's own two (F-0039, F-0040) · **2 functions, relocated not blocked**

`std.option.flatten` (nested rows are a wolfc parse error) and
`std.option.expect` (a diverging handler needs a bottom type). Both are
written and executing in the nursery; each graduates on the pin that
stops its diagnostic firing.

### Others · **2 functions**

`std.strbuf.in(r)` (s37 region-placement plumbing) and
`std.strbuf.reserve` (API-CONVENTIONS §8 — no capacity API without a
capacity, and the builtin exposes none). A third,
`std.testing.assert_starts_with`, is relocated rather than blocked: it is
written and executing in `std/x/testing_text`, counted with the nursery.

## 4. The evidence: what actually runs

The ledger records, per test, what each of the three lanes is expected to
achieve. 127 tests × 3 lanes:

| lane | run | unsupported | fail(E…) |
|---|---|---|---|
| lupin | **114** | 13 | 0 |
| wolfc `--checked` | **40** | 83 | 4 |
| native | **22** | 101 | 4 |

The four `fail` rows are the campaign's held divergences, one per
finding: `list/mutate_while_iterating.lu` (E1001, F-0014),
`range/is_empty.lu` (E0301, F-0030),
`x/option_flatten/flatten_propagate.lu` (E0201, F-0039),
`x/option_expect/expect_trap.lu` (E0401, F-0040). A rejection held as a
ledger row is a finding that cannot rot.

Most common lane triple: `run` / `unsupported` / `unsupported`, 65 tests
— the shape of a library written against one executing implementation
and two honest refusals. 13 tests run on all three.

Beyond the ledger: 211 doc examples extracted from module docs and run
(lupin must reach `exit(0)`; a compiler rung may refuse honestly, and a
static rejection is a doc bug), and 200 pinned ulp reference values that
both executing lanes reproduce bit-for-bit.

## 5. The findings ledger

42 filed. **15 retired or closed upstream**: F-0001, F-0002, F-0003,
F-0005, F-0006, F-0007, F-0008, F-0009, F-0010, F-0013, F-0017, F-0020,
F-0021, F-0022, F-0023. **1 narrowed**: F-0012. **2 part-retired**:
F-0024 (its lesson applied — the pin ritual's second gate, used three
times now) and F-0025 (two of its three shapes fixed at lupin 0.1.4).
**24 open**: F-0004, F-0011, F-0014, F-0015, F-0016, F-0018, F-0019,
F-0026, F-0027, F-0028, F-0029, F-0030, F-0031, F-0032, F-0033, F-0034,
F-0035, F-0036, F-0037, and sc06's five new ones F-0038 through F-0042.

Four of them are **silent wrong answers**, the class this track exists to
catch, and all four were found by writing library code rather than by
testing the implementation: F-0027 (`!=` on f64 is ordered natively),
F-0036 (a colliding row tag rides out as a value), F-0037 (an enum
through a row always misses), and lupin's own #15 (a mode-wrong call ran
with no writeback), which the interpreter fixed at this sprint's pin.

Six are **decision requests or amendments** rather than defects — the
track's other output: F-0008 (the iterator protocol, adopted), F-0011
(builtin versus std), F-0019 (the unicode tables budget, ruled), F-0028
(the transcendentals and the intrinsics ask), F-0033 (spec §7.4 with a
running reference implementation), F-0041 (the error-set alias surface).

## 6. The nursery clock

D31 gives `std/x` a graduation clock: at each campaign closeout every
resident graduates or is deleted, and nothing lives across two closeouts.
This is the first closeout, and the ruling is recorded in the campaign
closeout: all five residents get their one campaign, because each is
blocked on a filed upstream issue rather than on a design question, and
each has a named graduation trigger.

| resident | functions | graduates when |
|---|---|---|
| `x.list_eq` | 6 | trait dispatch executes (wolf-lang#5 / #12) |
| `x.deque_int` | 11 | `struct X[T]` parses (wolf-lang#11, s16) |
| `x.option_flatten` | 1 | wolfc's `type` production nests rows (wolf-lang#34) |
| `x.option_expect` | 1 | a bottom type, or a divergence rule (wolf-lang#35) |
| `x.testing_text` | 1 | `str` methods resolve in wolfc (wolf-lang#17) |

If a resident is still here at the stdc02 closeout, D31 says delete it —
and the reviewed contract in its header is what survives.
