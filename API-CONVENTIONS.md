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
  **RESOLVED at the sc06 pins** — rows parse and execute in parameter
  position, all six are written, and four of them are in the facade
  (`expect` and `flatten` sit in the nursery behind F-0040 and F-0039).
  Both spellings above are retired: the tag is lowercase everywhere and
  the helpers exist. Lazy variants and the data forms still wait.

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

**§4 amendment (sc07) — a propagating statement.** A fenced example
line may end in `?`, and it is emitted verbatim as a statement. The
extractor's generated `main` is `-> !int`, so `?` widens into its sealed
row and the line documents exactly what a caller writes at that call site:
`fs.write_text("out.tmp", "x")?`. Without it an os-facing operation could
appear in an example only under an `else`, which would document handling
the caller did not ask for — and a statement that CANNOT fail is still
spelled without a `?`, so the presence of one is information.

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

**§9 amendment (sc09) — the primitive landed, and four rules came with it.**
`std.str` delegates to the whole s37 builtin set now and ships 37 functions;
`std.bytes` ships 9. Writing them settled four things §9 could only gesture at
while the boundary primitive was missing.

- **`get` is for the caller's offsets; `s[a..b]` is for yours.** The
  recoverable slice (`get -> str ! {none}`) exists so that an offset a
  program received as DATA can be refused instead of faulting. An offset std
  computed — the one `find` just returned, `prefix.len` after a successful
  `starts_with` — is a boundary by construction, so std slices it with the
  language's checked form and lets a trap there mean what it would mean: a
  bug in std. Writing those sites as `get(...) else ""` would convert std's
  own bug into a wrong answer, which is the failure mode §9's
  refusal-over-approximation rule exists to prevent.
- **Where the lanes disagree about a primitive, the std function decides —
  before it delegates.** `count("")`, `split("")` and `replace(s, "", …)`
  are `unsupported` on two lanes and defined on the third (F-0055), so six
  `std.str` functions guard the empty argument themselves: `count` answers
  0, the rest trap `assert`. The guard is not a workaround for a bug in one
  implementation (which §0's house rule would file rather than paper over) —
  it is std stating a contract the toolchain has not stated, in the one
  place a caller can rely on it, with the filing attached. A std function may
  never have three behaviours because it has three lanes.
- **A code point is an `int` until `char` exists, and the interim is
  `std.unicode`'s.** `str.code_points`, `char_offsets` and `char_count` ship
  in the same currency `std.unicode` chose for its classifiers, so the two
  modules compose today and flip together the day `char` lands. Neither
  function is named `chars`: the blocked contract keeps its name, and the
  interim faces take descriptive ones, so nothing has to be renamed when the
  real one arrives.
- **Byte width is `List[int]` and monomorphic beats generic.**
  `std.bytes`' `len`/`is_empty`/`at`/`find`/`starts_with`/`ends_with`
  duplicate `std.list` names by design: `std.list`'s are generic and
  therefore execute on the interpreter lane alone, while these are concrete
  and execute on all three. When a std function could be generic or concrete
  and the concrete type is the one programs actually hold, ship the concrete
  one and say why in the header. Two modules may share function names as
  long as no program imports both (F-0026 is a per-program linker
  collision), which is why `std.bytes` has its own producer (`from_str`) and
  its tests never import `std.str`.

One naming ruling rides with them: **a PREDICATE does not trap.**
`bytes.is_utf8` answers `false` for an element outside 0..255 where §11 has
an ENCODER trap `assert` on the same input, and the difference is the
question being asked: "are these bytes text?" has an answer for a non-byte,
and a predicate a caller must guard before calling is not a predicate. §11's
rule is unchanged for the encoders it was written for.

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

## 12. The error taxonomy (sc06) — and §2's interim, retired

This section is the binding statement about what an error row CARRIES;
§2 remains the binding statement about when a row is used at all.
`docs/error-taxonomy.md` is the audit behind it — every row shipped in
sc01–sc05, its sites, and its verdict — and `std.errors` is its worked
home, as `std.option` is §2's.

- **Marks are lowercase, payload-carrying tags are CapCase.** A mark is a
  payload-free tag naming a failure mode: `none`, `gone`, `eof`, `done`,
  `parse`, `base`, `utf8`, `overflow`, `div_zero`, `deep`, `boundary`. A
  payload-carrying tag is CapCase and names its payload TYPE:
  `Parse(ParseErr)`. The case is the reader's signal about whether there
  is anything to destructure.
- **§2's spelling interim is RETIRED and the rename is applied.** std
  wrote `None`/`Done`/`Overflow`/`DivZero` from sc01 to sc05 because no
  implementation resolved a lowercase bare tag at a raise site (F-0003,
  whose ownership flipped implementations twice before dying at the sc05
  pins). Every occurrence — 148 across 32 files — is lowercase as of
  sc06, at zero cost to the ledger: no row moved.
- **A payload is DATA, never a rendered string.** `ParseErr {offset,
  kind}` is the pattern: where it failed, and which way. A payload that
  carried a sentence would force every caller to accept this library's
  wording and would make the position unrecoverable. Rendering is
  `describe`'s job and lives beside the type.
- **One tag per failure mode the caller can ACT on** — not one per call
  site, not one per internal cause. `hex.decode` raising a single `parse`
  for "odd length" and "bad digit" is right while no caller branches on
  the difference; the day one does, the tag gains a payload rather than a
  sibling.
- **Absence is not an error.** `none` never carries a payload and never
  joins a cause vocabulary. "There is nothing here" and "this went wrong"
  are answered by different tags on purpose (§2).
- **Coarsening is a named call the caller writes.**
  `errors.coarsen(e)` turns a `ParseErr` into a `Failure`; no std
  signature widens a row silently, and there is no `From` conversion to
  make it implicit (D30). The precision given up is the ability to branch
  on the kind, and giving it up should be visible in the caller's source.
- **The chain idiom is a FIELD.** wolf has no existentials and no
  boxing, so a wrapping error carries its cause as a field of its own
  payload (`Failure.cause`), not as a hidden box. A wrapper that needs
  more context grows another field.
- **Kinds are `int` until enums cross a module boundary.** An enum's
  values cross, but nothing that inspects them does (F-0029), so a
  payload's classifier is an `int` from a table its module documents.
  This is a recorded interim with a named exit, not a house style.
- **`errdefer` is the error-path cleanup form**, and Phase A ships none:
  no core function owns a resource that outlives a failure. The
  convention is recorded here so the io tier inherits it rather than
  inventing it — `errdefer` releases what the function acquired, `defer`
  releases what it borrowed, and neither ever changes the row.
- **A tag may not share a name with anything else in scope at the raise
  site** (§11's rule, restated because it is the sharpest edge in the
  area): the tag resolves to that thing and rides out as a value with no
  diagnostic (F-0036). Grep std's module list and your own module's items
  before naming a tag.

## 13. Test-authoring conventions (sc06) — binding for every sc sprint

`std.testing`'s module header carries the same list; this is its
normative home, and `cargo xtask std-test --lint-conventions` enforces
the five rules that can be decided mechanically. The rest are judgement,
and a lint that guessed at them would train authors to work around it.

- **A trap ends the process, so the RIG is the catch mechanism.** There
  is no in-language trap catching and there will not be (D30, no
  unwinding). A trap expectation is a directive — `check:
  run(exit=trap(kind))` — never a `catch`.
- **Happy-path assertions group into one entry file per theme**; grouping
  is safe only because a satisfied assertion is silent and effect-free.
- **Every trap expectation is its own entry file, named `…_trap.lu`**
  (`…_traps.lu` for a file whose one program can trap in several ways).
  The name is a promise, and the lint checks it both ways: a trap file
  that is not named so, and a `…_trap.lu` that expects no trap, are both
  errors.
- **A trap expectation names its KIND.** Bare `exit=trap` would pass for
  a program that trapped the wrong way, which is the failure the test
  exists to catch (`[conf.trap.exit]`).
- **No `stdout=` beside a trap expectation**: a trap record carries no
  stdout, so the hash would never be compared.
- **`testing.fail`/`unreachable`/`todo` appear only in trap files.** They
  never return, so a call in a file that expects `exit=0` is either a
  mis-directed test or dead code after the call.
- **Error-row expectations assert through `else |Tag(p)|` — tag AND
  payload.** The binding in `else |e|` is the TAG (`e.offset` is "error
  Parse has no member `offset`"), so a test that wants the payload
  destructures it in the pattern. `tests/errors/coarsen_and_chain.lu` is
  the worked example.
- **Table tests are a `List` of tuples plus a loop** until closures land
  (c05); the loop body is one indexing site, per the container rules.
- **Golden output rides the directive's `stdout=` hash**, and fixtures
  are shared rather than inlined twice.
- **Every test names its anchors** with `conforms:` (§4), and the doc
  examples are tests too: fenced means executable, always.

**§13 amendment (sc09) — the rig denies warnings.** A non-empty `warnings`
array in the observation record (`[proto.record.warn]`) is a RED in
`cargo xtask std-test` and in `doc-examples`, on every lane that reports
one. `conform-run` still rejects `--deny-warnings` (F-0046, re-verified at
the sc09 pin), so the rig does it: the flag this repo asked for twice is
approximated by the one signal the protocol does give, and both executing
implementations populate it now (wolfc since s67, lupin since 0.1.6's lint
wave). It paid on its first run — three doc examples carried `0.0 - x`
sites that W0402 had been flagging into a void. Two limits, stated so nobody
reads more into a green rig than it says: the array covers the ENTRY file
only, so a warning inside a std module body is invisible from here (F-0053's
open half), and a lane with no lint tier reports nothing rather than
reporting clean.

## 14. The os tier (sc07) — capabilities, rows, paths, handles

Phase B's first two modules (`std.fs`, `std.io`) are the worked home of
this section, as `std.option` is §2's and `std.errors` is §12's. Every
rule here is a rule for the modules that follow them (net, time, process).

- **Every os-facing `pub` item names its CAPABILITY, in its own doc, by
  the I13 name** (`fs`, `io`, `net`, `env`, `exec`, `ffi`, `unsafe`,
  `comptime`), and says that comptime refuses it (D33). The module header
  states the capability once for the module; the per-function note stays
  anyway, because a reader arrives at a function and a manifest audit
  reads functions. A function in an os module that reaches NOTHING (the
  path helpers) says so explicitly — "pure, comptime-safe" — because the
  exception is the surprising case.
- **Io errors are rows, and the row vocabulary is the toolchain's, not
  std's.** `{not_found, denied, io, utf8, eof}` are adopted verbatim from
  the builtin tier: std adds no translation layer, no error type of its
  own, and no coarsening a caller did not ask for (§12). A std function's
  row is exactly the union of what its delegates can raise, minus what it
  handles itself, and the doc lists every tag with the condition that
  produces it. Nothing in the os tier traps: a host failure is never a
  contract violation, because the caller could not have checked
  (§2's rule, in the place it matters most).
- **A path is a `str`, and the separator in a wolf literal is `/`.**
  Until a `Path` type exists, std does not invent one. Every path-taking
  function documents that forward slashes travel on every tier-1 host, and
  every path-producing helper cuts at `/` only — a backslash inside a wolf
  string literal is an ESCAPE, so a windows path written literally does
  not lex. Tests and doc examples use RELATIVE paths and the rig gives
  each program its own working directory; a std test never names an
  absolute path or a host temp directory.
- **An os handle is consumed by its closer.** `close(take f)` is the
  pattern: the mode system, not a generation counter or a liveness flag,
  is what makes use-after-close impossible, and the rejection is held as a
  test (`tests/fs/use_after_close.lu`, expected `fail(E1001)`). A handle
  that a caller could forge from data answers with a row (`io`) rather
  than trapping, for the same reason a missing file does. An operation
  that advances a handle's POSITION takes `mut` even when no field of the
  value changes — the stream's state is what the value names.
- **A capability module's lanes are honestly unequal, and the rig says so
  once.** An implementation may lack a capability entirely (lupin has no
  filesystem by design); its ledger column is `unsupported` at resolve and
  that is a posture recorded in `CONTRIBUTING.md`, not a defect noted in
  every test header. For doc examples the doc-truth rule becomes: at least
  ONE lane must reach `exit(0)`, and an honest refusal on the others is
  acceptable — a stronger requirement than "the reference machine runs it"
  for exactly the modules where the reference machine cannot.
- **What a capability module may not do to get a green lane**: emulate the
  capability, degrade silently, or ship an operation whose failure mode it
  cannot distinguish. `std.io.input_all` is the worked refusal — a loop
  over the line read cannot tell `eof` from `io` at this pin (F-0043), so
  the function is a reviewed contract in the module header instead of a
  silent truncation.

**§14 amendment (sc08) — the net tier, and the refusal rule restated.**
`std.net` is the third module of this section and the first whose whole
vocabulary is a network's. Everything above applies unchanged; four additions,
each earned by writing the module.

- **The row vocabulary is the toolchain's, again and verbatim**:
  `{refused, timeout, closed, utf8, io}`. `closed` is the socket's `eof` — the
  peer finished, an OUTCOME a reader stops on rather than reports — and it
  joins §2's absence family for that reason, not the error family. `refused`
  and `timeout` are error marks. std adds no tag, no translation and no
  coarsening a caller did not ask for.
- **A tag a program cannot reach is documented as unreachable, not omitted.**
  `timeout` is declared by three builtins and no builtin can arm a deadline at
  this pin (F-0049), so every `std.net` signature carries the tag, every doc
  says it is unobservable here, and the day a deadline lands nothing about the
  surface changes. Removing it would be the lie: the row is the toolchain's.
- **One verb, one type — so the second closer wears a longer name.** wolf has
  no overloading, so a module with two handle types cannot spell `close` twice.
  The vocabulary word goes to the value programs handle most (`net.close`
  takes the `Socket`) and the other is qualified (`net.close_listener`). Both
  are `take`-consuming per §14's handle rule; both hold their staleness
  discipline as a `fail(E1001)` test.
- **The pure member of a capability module is stated twice: on the function
  and in the module header.** `net.endpoint`/`net.loopback` touch nothing and
  are comptime-safe, exactly as `std.fs`'s path helpers are — and in an os
  module the pure function is the surprising one, so the exception is spelled
  out rather than inferred from the absence of a capability note.

**§14's last rule, restated because sc08 paid it twice.** "What a capability
module may not do to get a green lane" now has two worked refusals, blocked by
the same finding from opposite directions: `std.io.input_all` (sc07) and
`std.net.read_all` (sc08, written and withdrawn inside the sprint). A loop over
a rowed read must stop on one tag and re-raise the others, and no handler can
tell them apart — at the sc08 pin the shape that should discriminate
(`else |e| match e { … }`) compiles, runs, and matches its FIRST ARM for every
tag on the executing lane, while the other two lanes get it right (F-0052).
The rule that decides both: a capability module ships no operation whose
failure mode it cannot distinguish, and the version that "works on the happy
path" is the version that loses someone's data.

## Review record

- 2026-08-16 — sc09 amendments: §9 gains the four rules writing the landed
  `str`/`bytes` surface settled (`get` versus `s[a..b]` by whose offset it
  is; a std function decides where the lanes disagree about a primitive, and
  files; the `int` code-point interim shared with `std.unicode`; byte width
  as `List[int]` with monomorphic-beats-generic and the shared-name rule),
  plus the predicate-does-not-trap ruling — and §13 gains the warning gate.
  Review rides the same pending sc00 gate.
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
- 2026-08-13 — sc06 amendments (the campaign's last): §12 The error
  taxonomy — mark-versus-payload casing, payloads as data, one tag per
  actionable failure mode, absence-is-not-an-error, coarsening as a named
  call, the chain-as-a-field idiom, int kinds as a recorded interim, the
  `errdefer` convention Phase A records without using, and §2's
  `None`-spelling interim retired with the rename applied tree-wide —
  and §13 Test-authoring conventions, five of whose rules are enforced by
  `cargo xtask std-test --lint-conventions`. Review rides the same
  pending sc00 gate. **§2's helper inventory is no longer blocked**:
  `std.option`'s six are written and executing (four in the facade, two
  in the nursery behind F-0039/F-0040).
- 2026-08-13 — sc05 amendments: §11 Formatting, text↔number, and encodings
  (the spec-is-a-spelling split with its agree-clause-for-clause
  obligation, byte width, exactness at the number boundary, encoder/decoder
  pairing, the `List[int]` byte interim, the `base`/`deep`/`boundary` tags,
  the tag-name-collision and enum-through-a-row house rules, and the
  re-home rule for conversions). Review rides the same pending sc00 gate.
- 2026-08-15 — sc08 amendments: §14 gains the net tier (the
  `{refused, timeout, closed, utf8, io}` vocabulary adopted verbatim, an
  unreachable tag documented rather than omitted, the one-verb-one-type rule
  for a module with two handle types, and the pure-member exception stated
  twice), plus §14's refusal rule restated with its second worked refusal
  (`std.net.read_all`, withdrawn inside the sprint — F-0052). Review rides the
  same pending sc00 gate.
- 2026-08-14 — sc07 amendments (Phase B opens): §14 The os tier
  (capability notes per I13, the toolchain's row vocabulary adopted
  verbatim, paths as forward-slashed `str`, `take`-consumed handles with
  the rejection held as a test, and the capability-lane posture with its
  stronger doc-truth rule), plus §4's propagating-statement amendment.
  Review rides the same pending sc00 gate.
