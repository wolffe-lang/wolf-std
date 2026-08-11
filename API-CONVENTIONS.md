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

**§2 amendment (sc03) — the `parse` tag, and one interim retired the hard
way.** The absence inventory gains one domain tag: **`parse`**, the mark a
text-to-value conversion raises when the text is not a value of that type
(`std.str.parse_int`, `std.str.parse_float`). It earns its own noun rather
than riding `none` because a caller distinguishes "no value here" from
"these bytes are not a number", and sc05's `Parse` protocol will want the
same tag. Reserved alongside it, for the sprint that ships them:
**`utf8`** (bytes that are not well-formed UTF-8 — `std.bytes.to_str`).
Both are payload-free marks, so both are lowercase by this section's rule
and both ride the `None`-spelling interim below.

The interim itself now has a different owner: at the sc03 pin **wolfc
resolves lowercase bare tags and lupin does not** — the exact reverse of
sc01's record (F-0003, F-0023). std keeps writing `None`/`Done`/`Parse`
for the same reason as before, against the other implementation. The
lesson recorded in the register: lupin resolves tags LAZILY on the taken
path, so a raise site on an untaken branch proves nothing.

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

## 9. Strings (sc03)

- **Byte offsets are the one currency, and `len` is bytes.** Every
  position-taking or position-returning function in std deals in byte
  offsets; `len` is a byte count, O(1), documented with the test that
  proves it (`"é".len == 2`). A function that wants to count code points
  says so in its name and is not called `len`. There is no `s[i]` and no
  negative index (D25) — the redirects (`s[..n]`, `chars()`, `s[^1]`) are
  documented in `std.str`'s header, not invented around.
- **The three-way split, stated in the module header** (§8's rule, applied
  to strings): the LANGUAGE owns literals and interpolation, `s.len`,
  `s[a..b]` with its `bounds` trap, and the relational family
  (byte-lexicographic, `[mem.str.order]`); the IMPLEMENTATION owns the
  builtin method set std delegates to (`contains`, `starts_with`, `trim`,
  `lower`, `upper`, `repeat`, `lines`, `words`, `is_empty`); std owns
  everything else. A delegating function says it delegates, because the
  delegate's semantics are then part of the contract — `lower`'s case
  mapping is the clearest case (F-0019).
- **`str` is immutable, so nothing in `std.str` takes `mut`.** Mutation
  lives in `std.strbuf`, whose functions take `mut b` first and whose call
  sites spell it (§3, §8).
- **Traps versus rows at a boundary.** An offset that is out of range or
  falls inside a code point traps `bounds` — the language's checked slice,
  inherited by every std function that names an offset (`strbuf.insert`,
  `remove`, `truncate`), and documented on each. The recoverable form is
  `get`, which raises rather than traps; where `get` cannot yet exist, the
  trap is documented as the only spelling available and the gap is filed
  (F-0018) rather than papered over with a partial function.
- **Delegation over reimplementation, refusal over approximation.** When a
  builtin already provides an operation, std delegates and documents; when
  std cannot implement an operation correctly for ALL inputs, it ships the
  reviewed contract and a filing instead of a version that is right on
  ASCII and traps on text. sc03's fifteen blocked `std.str` functions are
  that rule in force.
- **Case and whitespace are the implementation's sets, and std must
  agree.** A std function whose behavior must match a builtin's set (as
  `trim_start`/`trim_end` must match `trim`) carries a test that pins the
  agreement member by member, so a builtin change fails CI instead of
  drifting silently.

## 10. Numerics, and the iterator combinator's name (sc04)

- **Checked is the default, recoverable is the complement, and the docs
  say when each is right.** `a + b` traps on overflow in every profile
  (X3) and that is what ordinary code writes; `math.checked_add ->
  int ! {overflow}` exists for the case where the OPERANDS are data and
  an out-of-range result is an ordinary outcome (parsing, accumulating
  user input, bounds arithmetic). `saturating_*` is the third form and
  has no row at all. A function may ship one, two or three of these; it
  may not ship a wrapping form without the `wrapping[T]` type, because
  intended overflow is spelled at the type (X3), not at the call.
- **§2's inventory gains two error tags**: `overflow` (a result outside
  the type) and `div_zero` (a zero divisor). Both are payload-free marks,
  so both are lowercase by §2's rule and both ride the `None`-spelling
  interim — std writes `Overflow` and `DivZero` at these pins (F-0003 /
  F-0023). They are *error* tags rather than absence tags and do not join
  the `none`/`gone`/`eof`/`parse` family: `else` handles them the same
  way, but a caller distinguishes "this arithmetic does not fit" from
  "there is nothing here".
- **Trap where the CALLER broke the contract, raise where the DATA did.**
  `pow(b, -1)` and `isqrt(-1)` trap `assert` (a negative exponent has no
  integer power; a negative radicand has no real root — the caller could
  have checked). `abs(int_min())` and `sum` overflow trap, because a
  magnitude that does not fit is a bug in the data model. `checked_add`
  raises. `rand.below(0)` traps and `rand.choose(empty)` raises, and the
  difference is exactly this rule: the first names an empty range, the
  second finds one.
- **Accuracy is a documented, measured, per-function contract.** Every
  transcendental states a bound in ulp; the bound is what the pinned
  reference table measures, not an aspiration; and the harness refuses a
  table row whose budget exceeds what the doc promises. Nothing in std
  claims faithful (≤1 ulp) or correct (≤0.5 ulp) rounding without a test
  that shows it. Where a function cannot reach the family's bound —
  `powf`, whose error is `ln`'s multiplied by the exponent — it says so
  in its own doc rather than letting the module header speak for it.
- **Pure wolf over a platform library, for anything whose value is
  observable.** `std.math.float` implements its transcendentals in wolf
  source so that every implementation agrees bit for bit and the tests
  can pin exact literals; a libm binding would make `sin(1.0)` mean
  whatever the host means. When intrinsics land, the wolf source stays
  the semantic reference (F-0028).
- **Constants are constants — unless the toolchain charges two execution
  lanes for one.** At these pins a module-level `const` is `unsupported`
  when used (checked tier) and when declared (native rung), and `INT_MIN`
  has no literal spelling at all, so `std.math` and `std.math.float` ship
  their constants as total zero-argument functions (`int_max()`, `pi()`)
  with the reason on the module. This is a recorded interim (F-0025 /
  F-0026), not a house style: the constants land the day either half is
  fixed, under the contract's own names.
- **The float family lives in its own module.** wolf has no overloading,
  so `abs`, `signum` and `round` cannot mean two things in one module;
  `std.math` is the integer family and `std.math.float` the `f64` one.
  The split is also what keeps each module importable into a native
  program, since two modules that declare a same-named function cannot be
  linked together yet (F-0026).
- **A sorted-input precondition is documented, not checked.**
  `binary_search` and the bound family assume ascending order and answer
  *something* on unsorted input rather than trapping — verifying costs
  more than the search. `is_sorted` is the check, and the docs name it.

**§10 amendment (sc04) — the iterator combinator is `limit(n)`, never
`take(n)`** (wolf-lang#16, ruled and closed). In wolf, `take` is a
parameter MODE that consumes its operand (X1); a combinator spelled
`xs.take(2)` that left `xs` usable would be a false friend at the exact
centre of the ownership story, and one that did consume it would be a
combinator nobody wants. The name is free to rule now because no
combinator has shipped. Every truncating combinator in std is `limit`,
and the family that follows it (`skip`, `step_by`, `rev`, `chunks`)
inherits the same test: a combinator name may not collide with an
ownership verb. `std.range`'s header records the ruling where the
blocked `rev`/`step_by` contracts are written down.

**§10 note (sc04) — unicode tables are std-carried source.** wolf-lang#18
is ruled and closed: the category/case tables land as **committed
generated wolf source** under `std.unicode.tables`, behind the facade,
with a pinned Unicode version, a checked-in generator and a manual
regeneration step guarded by a drift check (the grammar-sync pattern) —
because both implementations then execute the same wolf code and agree by
construction, where an intrinsic path makes Unicode a per-implementation
liability. Anything in std that classifies a code point cites that module
rather than growing a table of its own; §9's rule that a std function
matching a builtin's set carries a test pinning the agreement member by
member stands, and `tests/str/trim_whitespace_set.lu` remains the
worked example until the tables land.

## 11. Formatting, text↔number, and encodings (sc05)

- **The format spec is a spelling; std owns the operations.** Every
  meaning `{x:spec}` will carry exists here as a FUNCTION —
  `fmt.pad_left`/`pad_right`/`center` (+ `_with` fill variants),
  `with_sign`, the base family, `decimal.to_str_fixed`/`to_str_exp` — and
  that is not a stopgap. A program that computes a column width, or pads a
  value it received as data, needs the operation as a function whatever
  the f-string can do; the lowering is the ergonomic surface over the same
  semantics. The obligation the split creates: **the spec text and these
  functions must agree clause for clause**, which is why §7.4's candidate
  text was filed WITH this module as its reference implementation
  (F-0031 / F-0033) rather than after it.
- **Width is a BYTE count everywhere, and the doc says so.** `len` is
  bytes (§9), so padding is bytes. Display width needs East Asian Width
  and grapheme tables (the F-0019 budget question) and no core function
  pretends to know it; when the tables land, the display-width family
  arrives under its OWN name and `pad_left` keeps meaning bytes.
- **Exact beats approximate at the number boundary, and it is testable.**
  `std.fmt.decimal` rounds the EXACT value of a double (half-even) rather
  than a float approximation of it, `to_str` emits the shortest digit
  string that reads back as the same bits, and `parse_float` is correctly
  rounded. §10's "accuracy is a documented, measured, per-function
  contract" is satisfied here by there being no ulp budget to state: the
  contract is exactness, and the round trip
  (`to_bits(parse_float(to_str(x))) == to_bits(x)`) is a test over the
  torture corpus rather than a claim. A formatter that cannot say that
  about itself has not finished.
- **One value, one text, both directions.** Every encoder ships beside its
  decoder in the same module, the round-trip property is a test, and where
  the text direction is canonical (`base64.decode`'s strict profile) the
  module says which of the two round trips it promises and enforces the
  canonicity that makes the second one true.
- **Byte sequences are `List[int]` with a documented 0..255 element
  contract — an interim, and marked as one.** `std.bytes` cannot be
  written (F-0018/F-0035), and an encoder cannot wait for the type it
  converts. Element-range violations trap `assert` (a caller contract, §2)
  rather than encoding something the caller did not mean. Every signature
  keeps its shape when `Bytes` lands.
- **§2's inventory gains three tags**: **`base`** (a radix outside 2..36 —
  the CALLER's mistake, distinct from `parse`'s bad DATA), **`deep`** (a
  nesting limit reached, which a serializer raises instead of exhausting
  the stack), and **`boundary`** (this pin cannot find the code-point
  boundary an operation needs — an interim tag that disappears with
  F-0018). `overflow` is reused from §10 unchanged, for a decimal that
  rounds outside the finite range.
- **A row tag may not share a name with anything else in scope**, and this
  is a house rule only because the implementation makes it one: a
  colliding tag resolves to the module, function or binding it collides
  with and rides out as a VALUE, silently (F-0036). Three of this sprint's
  tags were renamed for it (`hex`→`parse`, `range`→`overflow`, and
  `std.json`'s `kind` function became `type_name` so the tag could keep
  its name). Until the filing closes, every new tag gets grepped against
  the module names in std and the item names in its own module.
- **No std accessor returns an enum through an error row** — the same
  shape is a silent wrong answer (F-0037), which is why `json.get`/`at`
  ship as contracts. Return a total value, or a non-enum payload, or wait.
- **Locale is never a parameter, in core, ever.** Grouping separators are
  the caller's argument; digit-group size is fixed at three; no function
  reads an environment.

**§11 note (sc05) — the parse family is re-homed.** `parse_int` and
`parse_float` moved from `std.str` to `std.fmt` / `std.fmt.decimal`: the
module that PRINTS numbers owns the whole number↔text boundary, so the two
directions can be read — and tested — as inverses. The general rule this
sets, for the closeout to confirm: **a conversion belongs to the module
that owns the target representation, not to the module that owns the
source type.** `std.str` keeps the string operations and no longer carries
a digit table.

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
- 2026-08-12 — sc03 amendments: §2 gains the `parse` tag (and reserves
  `utf8`) with the interim's ownership flipped to lupin, and §9 Strings.
  Review rides the same pending sc00 gate.
- 2026-08-12 — sc04 amendments: §10 Numerics (checked/recoverable/
  saturating, the `overflow` and `div_zero` error tags, the trap-versus-
  raise rule, the measured accuracy contract, the pure-wolf decision,
  constants-as-functions as a recorded interim, the float module split,
  and the documented sorted-input precondition), plus the two closed
  rulings: the iterator combinator is `limit(n)` and unicode tables are
  std-carried committed source. Review rides the same pending sc00 gate.
- 2026-08-13 — sc05 amendments: §11 Formatting, text↔number, and encodings
  (the spec-is-a-spelling split with its agree-clause-for-clause
  obligation, byte width, exactness at the number boundary, encoder/decoder
  pairing, the `List[int]` byte interim, the `base`/`deep`/`boundary` tags,
  the tag-name-collision and enum-through-a-row house rules, and the
  re-home rule for conversions). Review rides the same pending sc00 gate.
