# API-CONVENTIONS — the one-author document

Status: **sc00 deliverable, pending recorded human review** (the sc00
acceptance gate). Once reviewed, this document is binding: every scNN
review cites it, every `pub` item conforms to it, and a deviation is
either a bug or a reviewed amendment to this file — never a local style.
Decisions in force: D30 (rows, no unwinding), D31 (facade + nursery),
D32 (directory = module), X1 (receiver modes), X3 (checked arithmetic),
D10 (regions are the allocators). sc01 extends §2 (absence tags) and
sc06 extends §2 (tag naming and payloads) — by amendment here, not by
folklore there.

## 1. Naming

- Functions and module names: `snake_case`. Types, traits, and row tags:
  `CapCase`. Modules are nouns (`std.str`), never verbs.
- Predicates spell `is_` or `has_` and return `bool` — nothing else does:
  `is_empty`, `has_key`.
- Conversions: `to_x` allocates a new value (`to_list`); `as_x` and bare
  nouns are views that borrow (`as_bytes`, `bytes`). A `to_`/`as_` pair on
  one type must agree about which allocates.
- **No `get_` prefix anywhere.** Bare `get` is the one checked-access
  spelling, and it returns an absence row (`-> T ! {none}`), never a
  sentinel. Everything else names what it fetches (`len`, `first`,
  `keys`).
- Abbreviations only where the domain already owns them (`len`, `cmp`,
  `fmt`, `min`); otherwise whole words. One concept, one name, across all
  of std: the operation spelled `push` on `List` is not `append` on
  `StrBuf` unless the semantics genuinely differ — and then the
  difference is documented on both.

## 2. Error-row discipline (`!T`)

- Every `pub` signature states its error row explicitly or names a
  reviewed alias — sc01/sc06 own the alias inventory. The sealing rule
  (s15, house law): a `pub` item never exports an unnamed inferred row.
- **Absence is a row tag, not a sentinel.** The default absence tag is
  `none` (`fn find(…) -> int ! {none}`), consumed by `else` and `?`.
  Domain-specific tags (`eof`, `gone`) are legal where the noun earns it;
  sc01 records the inventory.
- Coarsening (collapsing a precise row into a broader one) is a named
  `impl`/function the caller opts into, never something a std signature
  does silently.
- Row tags are `CapCase` when they carry payloads (`Tag(Payload)`),
  lowercase bare words when they are pure marks (`none`, `eof`, `gone`).
  Payload conventions land with sc06; until then a new payload-bearing
  tag needs a review comment citing this section.
- No trap where a row will do: traps are for contract violations the
  caller could not check (`[conf.trap.set]` kinds); recoverable outcomes
  ride the row.

**§2 amendment (sc01) — the absence inventory.** The default absence tag
is `none` and the consuming forms are the language's: `else` for
defaults and handling, `?` for propagation. Domain tags in force:
`gone` (dead weak upgrade, `[mem.shared.rc.3]`) and — reserved for the
io tier — `eof`. A new absence-flavored tag cites this list or amends
it here. Two recorded interims, each filed upstream and flipped with
the pin bump that fixes it (`docs/findings.md`):
- **Spelling:** std code writes the tag `None` today — neither pinned
  implementation resolves a lowercase bare tag in raise position
  (F-0003).
- **The helper inventory is blocked:** rows exist only in return
  position (a `!`-row in a parameter or `let` type is E0201 in the
  pinned grammar and both implementations), so `or`/`expect`/`flatten`/
  `to_list`/`exists`/`is_none` cannot be written as functions; the
  language forms are the idiom and `std.option`'s module doc + tests
  are their worked home (F-0002). Lazy variants additionally wait on
  closures (c05); `Option[T]`/`Result[T, E]` as data wait on s16.

## 3. Parameter modes (X1)

- The `read` default is sacred: the absence of a keyword *is* the mode,
  and std never adds ceremony to read-only use.
- `mut self` / `mut x` only where mutation is the operation's point
  (`push`, `sort_in_place`) — never for incidental scratch space.
- `take` only for true consumption, where the value is gone afterwards
  (`freeze`, builders' `finish`). If the caller could reasonably keep
  using the value, the signature is wrong.
- Every doc example writes honest call-site modes — `(mut b).push(1)` —
  exactly as the call site must; examples that hide the mode are doc
  bugs and fail review.

## 4. Doc format

- Module header: the file opens with `//!` — for member files, after the
  `member: true` directive line (`[conf.directive.member]`) — carrying
  one sentence of contract plus the stability tier: `core`, `penumbra`,
  or `x` (D31).
- Every `pub` item carries `///` with, in order:
  1. the contract sentence (what it computes, totality);
  2. the meaning of each row tag it can return;
  3. trap conditions, naming kinds from `[conf.trap.set]` and the
     licensing clause (e.g. checked arithmetic, X3);
  4. at least one runnable example where the doctrine says (core-tier
     functions: always; see extraction format below).
- **Doc-example extraction format** (defined here, consumed by the rig
  from sc01 on): a fenced block tagged `wolf-doc-example` whose every
  line is a boolean expression over the documented API. The extractor
  wraps the lines into a staged entry file — each line `EXPR` becomes
  `if EXPR { } else { return 1 }` in a generated `fn main() -> !int`
  ending in `0`, imported against the staged module — and runs it under
  the rig like any test, ledgered like any test. Examples that need
  state write let-bindings on their own lines; anything unrunnable
  today is written as prose, not as a fenced example — fenced means
  executable, always.
- Docs never show output they did not produce (the doc-truth rule the
  interpreter track enforces mechanically; the extractor is this repo's
  mechanism).

**§4 amendment (sc02) — statements in examples.** A fenced example's
lines are classified mechanically, and every line stays real wolf:

- a line containing a relational operator (`==`, `!=`, `<`, `<=`, `>`,
  `>=`) is an **assertion**, wrapped as `if LINE { } else { return N }`;
- every other line is a **statement**, emitted verbatim — a `let`/`var`
  binding, an assignment (`m[k] = v`), a unit-returning call
  (`list.push(mut xs, 1)`), a one-line `for`;
- a line that is neither an assertion nor a plausible statement is a
  hard extractor error, so a typo cannot pass through unchecked.

The rule exists because mutation is half of a container API: without it
`push`, `set`, `clear` and `extend` could carry no runnable example at
all. Its cost is one explicit spelling: a predicate is asserted as
`list.is_empty(xs) == true`, never as a bare call, because a bare call
is a statement.

**§4 note (sc02) — one module per example.** The extractor imports
exactly the documented module (`use std.<path>`), so an example may only
name that module plus the language itself. Examples that would need a
sibling module are restructured or written as prose.

## 5. Regions

- Allocate in the ambient region by default; std code never demands a
  region argument for ordinary use (D10: zero annotations in application
  code).
- Types that own a buffer offer `.in(r)` variants (the s37 pattern) —
  `List.in(r)`, `StrBuf.in(r)` — as the *only* region-explicit surface;
  algorithms stay region-silent and inherit their operands' homes.
- Nothing in Phase A assumes runtime services beyond the language's own
  allocation story (D15: rt-thin); a module that needs more waits for
  stdc02, it does not shim.

## 6. Ordering (sc01)

- `std.cmp.Ordering` (`Less | Equal | Greater`) is the one comparison
  verdict in std; no module invents an int-coded or bool-pair order.
- `Eq` is equivalence (reflexive, symmetric, transitive); `Ord` is a
  total order (trichotomy, antisymmetry, transitivity) that must agree
  with `Eq` where both exist. `Ord` requires `Eq` by doctrine; the
  pinned grammar has no supertrait clause, so the requirement is
  documentary until the trait-composition question lands (F-0004).
- **Float, decided:** `f64` has no `Eq` — IEEE partial comparison stays
  on the builtin operators — and its `Ord` IS `total_cmp` (IEEE
  totalOrder; NaN's one home is above `+inf`). Sorting floats is legal;
  the two pin-era approximations (NaN sign/payload unobserved) are
  documented on `total_cmp` itself.
- Ties: `min` and `max` both return their FIRST argument on a tie —
  stability callers can rely on.
- `clamp(v, lo, hi)` traps `assert` when `lo > hi`: a caller contract
  violation, not a row (per §2's trap rule).
- The operator bridge (`==`/`<=>` desugaring to `Eq.eq`/`Ord.cmp` for
  user types) is filed, not assumed (F-0004): today `<=>` is
  builtin-only and yields an int in lupin, and enum `==` is refused by
  wolfc — std code writes trait calls, never leans on the bridge.

## 7. Assert (sc01)

- `assert(cond)` is an intrinsic, not a prelude function: one name in
  both tiers (comptime witness, s16; runtime trap `assert`,
  `[conf.trap.map]`), silent and effect-free when satisfied.
- **No std module defines a function named `assert`** — a module item
  shadows the intrinsic module-wide (observed both implementations),
  severing the module from the trap it needs. The two-argument
  `assert(cond, msg)` is therefore filed (F4 / wolf-lang), not shipped;
  its interim spelling is `if !cond { testing.fail(msg) }`.
- `testing.fail`/`unreachable`/`todo` all trap `assert` and never
  return normally; helpers print at most one fixed line before
  trapping and render no values until sc05's fmt trait (`assert_eq`/
  `assert_ne` say only which side of the relation broke).

## 8. Containers (sc02)

- **The language's, or std's** — stated in every container module's
  header, never assumed: `List`/`Pool` construction (`List[T]()`),
  indexing (`xs[i]`, `p[h]`), index assignment, `for` iteration and
  `Pool`'s two-phase `reserve`/`init` are the LANGUAGE's; every named
  operation over them is std's. The split itself is a filed
  recommendation, not folklore (F-0011).
- **Container first, mode honest.** A container operation takes its
  container as the first parameter with the X1 mode the operation
  deserves: `mut` for mutators (`push`, `set`, `clear`), the bare read
  default for queries (`len`, `get`, `contains`), `take` only for true
  consumption. Call sites spell it: `list.push(mut xs, 1)`.
- **Absence over sentinels, traps for contract violations** (§2 applied):
  `pop`, `get`, `first`, `last`, `pop_front` raise `None`; `remove(i)`,
  `set(i, v)` and the language's `xs[i]` trap `bounds`, because a caller
  that could have checked `len` and did not has violated a contract. A
  container never returns `-1`, `0`, or an empty value to mean "absent".
- **Order is promised only where it is meant.** `Map`'s iteration order
  is unspecified — `keys`/`values`/`pairs` promise only to agree with each
  other for an unmodified map; `Pool`'s slot order is unspecified;
  `Deque.to_list` promises front-to-back, and says so.
- **Naming across containers is one vocabulary**: `len`, `is_empty`,
  `clear`, `extend`, `to_list`, `get`, `remove`, `has`/`contains`
  (`contains` for sequences, `has` for keyed lookup — the difference is
  documented on both). A new container adopts these names or argues in
  review.
- **No capacity API without a capacity.** `reserve` is not shipped while
  the builtin exposes no capacity at all: a no-op documented as a hint
  would be a lie (F-0011).

## Review record

- 2026-08-10 — drafted (sc00). Human review: **pending**; record the
  reviewer and date here when it lands, then flip Status above to
  binding.
- 2026-08-10 — sc01 amendments: §2 absence inventory (with the two
  filed interims), §6 ordering, §7 assert. Review rides the same
  pending sc00 gate.
- 2026-08-11 — sc02 amendments: §4 statement-vs-assertion rule for
  fenced examples plus the one-module note, and §8 Containers. Review
  rides the same pending sc00 gate.
