# API-CONVENTIONS: the one-author document

Status: **sc00 deliverable, pending recorded human review** (the sc00
acceptance gate). Once reviewed, this document is binding: every scNN
review cites it, every `pub` item conforms to it, and a deviation is
either a bug or a reviewed amendment to this file. A local style is
neither. Decisions in force: D30 (rows, no unwinding), D31 (facade +
nursery), D32 (directory = module), X1 (receiver modes), X3 (checked
arithmetic), D10 (regions are the allocators). sc01 extends §2 (absence
tags) and sc06 extends §2 (tag naming and payloads). Both extend it by
amendment here, not by folklore there.

## 1. Naming

- Functions and module names: `snake_case`. Types, traits, and row tags:
  `CapCase`. Modules are nouns (`std.str`), never verbs.
- Predicates spell `is_` or `has_` and return `bool`. Nothing else does:
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
  `StrBuf` unless the semantics genuinely differ, and then the
  difference is documented on both.

## 2. Error-row discipline (`!T`)

- Every `pub` signature states its error row explicitly or names a
  reviewed alias. sc01 and sc06 own the alias inventory. The sealing rule
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

**§2 amendment (sc01): the absence inventory.** The default absence tag
is `none` and the consuming forms are the language's: `else` for
defaults and handling, `?` for propagation. Domain tags in force:
`gone` (dead weak upgrade, `[mem.shared.rc.3]`), and `eof`, reserved for
the io tier. A new absence-flavored tag cites this list or amends
it here. Two recorded interims, each filed upstream and flipped with
the pin bump that fixes it (`docs/findings.md`):
- **Spelling:** std code writes the tag `None` today, because neither
  pinned implementation resolves a lowercase bare tag in raise position
  (F-0003).
- **The helper inventory is blocked:** rows exist only in return
  position (a `!`-row in a parameter or `let` type is E0201 in the
  pinned grammar and both implementations), so `or`/`expect`/`flatten`/
  `to_list`/`exists`/`is_none` cannot be written as functions; the
  language forms are the idiom and `std.option`'s module doc + tests
  are their worked home (F-0002). Lazy variants additionally wait on
  closures (c05); `Option[T]`/`Result[T, E]` as data wait on s16.
  **RESOLVED at the sc06 pins.** Rows parse and execute in parameter
  position, all six are written, and four of them are in the facade
  (`expect` and `flatten` sit in the nursery behind F-0040 and F-0039).
  Both spellings above are retired: the tag is lowercase everywhere and
  the helpers exist. Lazy variants and the data forms still wait.

**§2 amendment (sc03): the `parse` tag, and one interim retired the hard
way.** The absence inventory gains one domain tag, **`parse`**, the mark a
text-to-value conversion raises when the text is not a value of that type
(`std.str.parse_int`, `std.str.parse_float`). It earns its own noun
instead of riding `none` because a caller distinguishes "no value here"
from "these bytes are not a number", and sc05's `Parse` protocol wants
the same tag. Reserved alongside it, for the sprint that ships them:
**`utf8`**, for bytes that are not well-formed UTF-8
(`std.bytes.to_str`). Both are payload-free marks, so both are lowercase
by this section's rule and both ride the `None`-spelling interim below.

The interim itself now has a different owner. At the sc03 pin **wolfc
resolves lowercase bare tags and lupin does not**, the exact reverse of
sc01's record (F-0003, F-0023). std keeps writing `None`/`Done`/`Parse`
for the same reason as before, against the other implementation. The
lesson recorded in the register: lupin resolves tags LAZILY on the taken
path, so a raise site on an untaken branch proves nothing.

## 3. Parameter modes (X1)

- The `read` default is sacred: the absence of a keyword *is* the mode,
  and std never adds ceremony to read-only use.
- `mut self` / `mut x` only where mutation is the operation's point
  (`push`, `sort_in_place`), never for incidental scratch space.
- `take` only for true consumption, where the value is gone afterwards
  (`freeze`, builders' `finish`). If the caller could reasonably keep
  using the value, the signature is wrong.
- Every doc example writes honest call-site modes (`(mut b).push(1)`),
  exactly as the call site must. Examples that hide the mode are doc
  bugs and fail review.

## 4. Doc format

- Module header: the file opens with `//!`, carrying one sentence of
  contract plus the stability tier: `core`, `penumbra`, or `x` (D31). In
  a member file the header follows the `member: true` directive line
  (`[conf.directive.member]`).
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
  wraps the lines into a staged entry file. Each line `EXPR` becomes
  `if EXPR { } else { return 1 }` in a generated `fn main() -> !int`
  ending in `0`, imported against the staged module. The rig then runs
  it like any test, ledgered like any test. Examples that need state
  write let-bindings on their own lines. Anything unrunnable today is
  written as prose, never as a fenced example: fenced means executable,
  always.
- Docs never show output they did not produce (the doc-truth rule the
  interpreter track enforces mechanically; the extractor is this repo's
  mechanism).

**§4 amendment (sc02): statements in examples.** A fenced example's
lines are classified mechanically, and every line stays real wolf:

- a line containing a relational operator (`==`, `!=`, `<`, `<=`, `>`,
  `>=`) is an **assertion**, wrapped as `if LINE { } else { return N }`;
- every other line is a **statement**, emitted verbatim: a `let`/`var`
  binding, an assignment (`m[k] = v`), a unit-returning call
  (`list.push(mut xs, 1)`), a one-line `for`;
- a line that is neither an assertion nor a plausible statement is a
  hard extractor error, so a typo cannot pass through unchecked.

The rule exists because mutation is half of a container API: without it
`push`, `set`, `clear` and `extend` could carry no runnable example at
all. Its cost is one explicit spelling: a predicate is asserted as
`list.is_empty(xs) == true`, never as a bare call, because a bare call
is a statement.

**§4 amendment (sc07): a propagating statement.** A fenced example
line may end in `?`, and it is emitted verbatim as a statement. The
extractor's generated `main` is `-> !int`, so `?` widens into its sealed
row and the line documents exactly what a caller writes at that call site:
`fs.write_text("out.tmp", "x")?`. Without it an os-facing operation could
appear in an example only under an `else`, which would document handling
the caller did not ask for. A statement that CANNOT fail is still
spelled without a `?`, so the presence of one is information.

**§4 note (sc02): one module per example.** The extractor imports
exactly the documented module (`use std.<path>`), so an example may only
name that module plus the language itself. Examples that would need a
sibling module are restructured or written as prose.

## 5. Regions

- Allocate in the ambient region by default; std code never demands a
  region argument for ordinary use (D10: zero annotations in application
  code).
- Types that own a buffer offer `.in(r)` variants (the s37 pattern:
  `List.in(r)`, `StrBuf.in(r)`), and that is the *only* region-explicit
  surface. Algorithms stay region-silent and inherit their operands'
  homes.
- Nothing in Phase A assumes runtime services beyond the language's own
  allocation story (D15: rt-thin). A module that needs more waits for
  stdc02. It does not shim.

## 6. Ordering (sc01)

- `std.cmp.Ordering` (`Less | Equal | Greater`) is the one comparison
  verdict in std; no module invents an int-coded or bool-pair order.
- `Eq` is equivalence (reflexive, symmetric, transitive); `Ord` is a
  total order (trichotomy, antisymmetry, transitivity) that must agree
  with `Eq` where both exist. `Ord` requires `Eq` by doctrine; the
  pinned grammar has no supertrait clause, so the requirement is
  documentary until the trait-composition question lands (F-0004).
- **Float, decided:** `f64` has no `Eq`, so IEEE partial comparison
  stays on the builtin operators, and its `Ord` IS `total_cmp` (IEEE
  totalOrder; NaN's one home is above `+inf`). Sorting floats is legal.
  The two pin-era approximations (NaN sign/payload unobserved) are
  documented on `total_cmp` itself.
- Ties: `min` and `max` both return their FIRST argument on a tie. That
  is stability a caller can rely on.
- `clamp(v, lo, hi)` traps `assert` when `lo > hi`: a caller contract
  violation, not a row (per §2's trap rule).
- The operator bridge (`==`/`<=>` desugaring to `Eq.eq`/`Ord.cmp` for
  user types) is filed, not assumed (F-0004): today `<=>` is
  builtin-only and yields an int in lupin, and enum `==` is refused by
  wolfc. std code writes trait calls and never leans on the bridge.

## 7. Assert (sc01)

- `assert(cond)` is an intrinsic, not a prelude function: one name in
  both tiers (comptime witness, s16; runtime trap `assert`,
  `[conf.trap.map]`), silent and effect-free when satisfied.
- **No std module defines a function named `assert`.** A module item
  shadows the intrinsic module-wide (observed on both implementations),
  severing the module from the trap it needs. The two-argument
  `assert(cond, msg)` is therefore filed (F4 / wolf-lang) and not
  shipped. Its interim spelling is `if !cond { testing.fail(msg) }`.
- `testing.fail`/`unreachable`/`todo` all trap `assert` and never
  return normally. Helpers print at most one fixed line before
  trapping and render no values until sc05's fmt trait (`assert_eq`/
  `assert_ne` say only which side of the relation broke).

## 8. Containers (sc02)

- **The language's, or std's.** Every container module's header states
  the split, and no module assumes it: `List`/`Pool` construction
  (`List[T]()`), indexing (`xs[i]`, `p[h]`), index assignment, `for`
  iteration and `Pool`'s two-phase `reserve`/`init` are the LANGUAGE's;
  every named operation over them is std's. The split itself is a filed
  recommendation, not folklore (F-0011).
- **Container first, mode honest.** A container operation takes its
  container as the first parameter with the X1 mode the operation
  deserves: `mut` for mutators (`push`, `set`, `clear`), the bare read
  default for queries (`len`, `get`, `contains`), `take` only for true
  consumption. Call sites spell it: `list.push(mut xs, 1)`.
- **Absence over sentinels, traps for contract violations** (§2 applied):
  `pop`, `get`, `first`, `last`, `pop_front` raise `none`; `remove(i)`,
  `set(i, v)` and the language's `xs[i]` trap `bounds`, because a caller
  that could have checked `len` and did not has violated a contract. A
  container never returns `-1`, `0`, or an empty value to mean "absent".
- **Order is promised only where it is meant.** `Map`'s iteration order
  is unspecified, and `keys`/`values`/`pairs` promise only to agree with
  each other for an unmodified map. `Pool`'s slot order is unspecified.
  `Deque.to_list` promises front-to-back, and says so.
- **Naming across containers is one vocabulary**: `len`, `is_empty`,
  `clear`, `extend`, `to_list`, `get`, `remove`, `has`/`contains`
  (`contains` for sequences, `has` for keyed lookup; the difference is
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
  negative index (D25). The redirects (`s[..n]`, `chars()`, `s[^1]`) are
  documented in `std.str`'s header, not invented around.
- **The three-way split, stated in the module header** (§8's rule, applied
  to strings): the LANGUAGE owns literals and interpolation, `s.len`,
  `s[a..b]` with its `bounds` trap, and the relational family
  (byte-lexicographic, `[mem.str.order]`); the IMPLEMENTATION owns the
  builtin method set std delegates to (`contains`, `starts_with`, `trim`,
  `lower`, `upper`, `repeat`, `lines`, `words`, `is_empty`); std owns
  everything else. A delegating function says it delegates, because the
  delegate's semantics are then part of the contract. `lower`'s case
  mapping is the clearest case (F-0019).
- **`str` is immutable, so nothing in `std.str` takes `mut`.** Mutation
  lives in `std.strbuf`, whose functions take `mut b` first and whose call
  sites spell it (§3, §8).
- **Traps versus rows at a boundary.** An offset that is out of range or
  falls inside a code point traps `bounds`. That is the language's
  checked slice, inherited by every std function that names an offset
  (`strbuf.insert`, `remove`, `truncate`), and documented on each. The
  recoverable form is `get`, which raises where the slice traps. Where
  `get` cannot yet exist, the trap is documented as the only spelling
  available and the gap is filed (F-0018), never papered over with a
  partial function.
- **Delegation over reimplementation, refusal over approximation.** When a
  builtin already provides an operation, std delegates and documents. When
  std cannot implement an operation correctly for ALL inputs, it ships the
  reviewed contract and a filing instead of a version that is right on
  ASCII and traps on text. sc03's fifteen blocked `std.str` functions are
  that rule in force.
- **Case and whitespace are the implementation's sets, and std must
  agree.** A std function whose behavior must match a builtin's set (as
  `trim_start`/`trim_end` must match `trim`) carries a test that pins the
  agreement member by member, so a builtin change fails CI instead of
  drifting silently.

**§9 amendment (sc09): the primitive landed, and four rules came with it.**
`std.str` delegates to the whole s37 builtin set now and ships 37 functions;
`std.bytes` ships 9. Writing them settled four things §9 could only gesture at
while the boundary primitive was missing.

- **`get` is for the caller's offsets; `s[a..b]` is for yours.** The
  recoverable slice (`get -> str ! {none}`) exists so that an offset a
  program received as DATA can be refused instead of faulting. An offset std
  computed (the one `find` just returned, `prefix.len` after a successful
  `starts_with`) is a boundary by construction, so std slices it with the
  language's checked form and lets a trap there mean what it would mean: a
  bug in std. Writing those sites as `get(...) else ""` would convert std's
  own bug into a wrong answer, which is the failure mode §9's
  refusal-over-approximation rule exists to prevent.
- **Where the lanes disagree about a primitive, the std function decides
  before it delegates, AND THE DECISION IS TEMPORARY.** `count("")`,
  `split("")` and `replace(s, "", …)` were `unsupported` on two lanes and
  defined on the third (F-0055), so six `std.str` functions guarded the empty
  argument themselves: `count` answered 0, the rest trapped `assert`. That
  was std stating a contract the toolchain had not stated, in the one place a
  caller can rely on it, with the filing attached. It was never a workaround
  for a bug in one implementation; the house rule in `CONTRIBUTING.md` files
  such a bug instead of papering over it. A std function may never have three
  behaviours because it has three lanes.

  **sc11 completes the rule with its other half.** s71 RULED the empty needle
  (`[mem.str.empty]`: count 0, one whole piece, identity) and every lane
  answers it, so the guards are deleted and the delegation is the contract
  again. That holds where the ruling DISAGREED with std's reviewed contract,
  which it did twice: `split("")` and `replace(s, "", …)` trapped `assert` in
  std and are defined answers in the language. std adopts the ruling without
  argument, because a std function that second-guesses a spec clause is a
  worse thing than the ambiguity the guard was covering. Two of the six
  guards survive in a different role and say so on themselves: `splitn` and
  `replacen` walk `find` instead of delegating, and `find("")` is 0 forever,
  so they state the ruled answer explicitly to reach it at all. **The full
  rule, then: guard where the lanes disagree, file it, and delete the guard
  the day the toolchain rules. The filing is what makes the second half
  happen.**
- **A code point is a `char` (sc25; D58 `[type.char]`), and the doors
  are two.** `char` landed at the sc24 data pin, executed wolf-side at
  sc24 and everywhere at sc25 (lupin 0.1.15/is26), and `std.unicode`
  flipped whole at that release: `from_code(n: int) -> char ! {none}`
  and `code(c: char) -> int` are the ONLY places in std where a number
  and a code point trade places; the classifiers take `char` and
  COMPARE against `char` literals — never compute — and `utf8_len` is
  total over `char`. No arithmetic on the scalar anywhere is the review
  gate. What deliberately did NOT flip: `str.code_points`,
  `char_offsets` and `char_count` keep the `int`/byte-offset currency —
  `code_points` is std's own UTF-8 reading, held against the builtin
  `chars()` by `tests/str/chars_builtin_agree.lu` (two independent
  decoders that must agree; a retype onto `chars()`'s currency would
  make that a tautology), and the offset trio's currency is byte
  offsets, which were never code points. Their descriptive names (the
  sc03 rule: the blocked contract keeps its name) stay correct: none of
  them is `chars`, and `chars` — the `(offset, char)` pair — still
  waits on tuple lists.
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
an ENCODER trap `assert` on the same input. The difference is the question
being asked. "Are these bytes text?" has an answer for a non-byte, and a
predicate a caller must guard before calling is not a predicate. §11's
rule is unchanged for the encoders it was written for.

**§9 amendment (sc12): the byte view, and the rule a performance primitive
needs.** s77 made `s.bytes()` the receiver's own pointer and length, read in
place wherever the call is consumed. Writing eight bodies against it settled
three things.

- **A shape that costs a lane is not a shape std writes.** §14 says a
  capability module may not emulate, degrade or ship an operation whose
  failure mode it cannot distinguish, all in the service of a green lane. The
  same discipline governs an OPTIMIZATION, and it needed saying because the
  temptation is opposite in direction and identical in kind: the fast shape is
  right there, it runs on two of three lanes, and taking it would trade a
  documented execution lane for a cost nothing in this repository can measure.
  The compiler consumes the view in seven positions; wolfc's checked tier
  models two (`for b in s.bytes()` and `s.bytes().len` — F-0071), so std uses
  two. Where the natural body wants the other five, the ALGORITHM changes:
  `str.code_points` is a one-pass state machine because a view cannot be read
  ahead of on that lane.
- **Do not assert a property this rig cannot observe.** The no-call,
  no-allocation claim belongs to the counterparty, where it is a pinned IR
  snapshot; here it would be a rumour in the §13 sense. std's docs say what
  the body DOES (walks the view, carries a counter, allocates the output) and
  cite the wave that made it cheap, and the test that ships with them
  (`tests/str/byte_view_walk.lu`) pins the shapes RUNNING and answering, which
  is the part this rig can see. A repository that cannot measure a property
  does not assert it, and saying so in the test header is how the next reader
  learns which half was measured.
- **A doc that explains a body's shape must name the constraint, not just the
  wave.** "Written as a `for` because s77 made iteration free" is half the
  sentence; the other half is "and because indexing the view costs the checked
  lane". A reader who has only the first half will helpfully rewrite the loop.

- **Checked is the default, recoverable is the complement, and the docs
  say when each is right.** `a + b` traps on overflow in every profile
  (X3) and that is what ordinary code writes. `math.checked_add ->
  int ! {overflow}` exists for the case where the OPERANDS are data and
  an out-of-range result is an ordinary outcome (parsing, accumulating
  user input, bounds arithmetic). `saturating_*` is the third form and
  has no row at all. A function may ship one, two or three of these. It
  may not ship a wrapping form without the `wrapping[T]` type, because
  intended overflow is spelled at the type (X3), not at the call.
- **§2's inventory gains two error tags**: `overflow` (a result outside
  the type) and `div_zero` (a zero divisor). Both are payload-free marks,
  so both are lowercase by §2's rule. Both rode the `None`-spelling
  interim: std wrote `Overflow` and `DivZero` from sc04 until the sc06
  rename (F-0003 / F-0023), which §12 records as retired. They are
  *error* tags and do not join the `none`/`gone`/`eof`/`parse` family.
  `else` handles them the same way, and a caller still distinguishes
  "this arithmetic does not fit" from "there is nothing here".
- **Trap where the CALLER broke the contract, raise where the DATA did.**
  `pow(b, -1)` and `isqrt(-1)` trap `assert`: a negative exponent has no
  integer power, a negative radicand has no real root, and the caller
  could have checked. `abs(int_min())` and `sum` overflow trap, because a
  magnitude that does not fit is a bug in the data model. `checked_add`
  raises. `rand.below(0)` traps and `rand.choose(empty)` raises, and the
  difference is exactly this rule: the first names an empty range, the
  second finds one.
- **Accuracy is a documented, measured, per-function contract.** Every
  transcendental states a bound in ulp. The bound is what the pinned
  reference table measures, and the harness refuses a table row whose
  budget exceeds what the doc promises. Nothing in std claims faithful
  (≤1 ulp) or correct (≤0.5 ulp) rounding without a test that shows it.
  Where a function cannot reach the family's bound, it says so in its own
  doc and does not let the module header speak for it. `powf` is the
  case: its error is `ln`'s multiplied by the exponent.
- **Pure wolf over a platform library, for anything whose value is
  observable.** `std.math.float` implements its transcendentals in wolf
  source so that every implementation agrees bit for bit and the tests
  can pin exact literals. A libm binding would make `sin(1.0)` mean
  whatever the host means. When intrinsics land, the wolf source stays
  the semantic reference (F-0028).
- **Constants are constants, unless the toolchain charges two execution
  lanes for one.** At these pins a module-level `const` is `unsupported`
  when used (checked tier) and when declared (native rung), and `INT_MIN`
  has no literal spelling at all, so `std.math` and `std.math.float` ship
  their constants as total zero-argument functions (`int_max()`, `pi()`)
  with the reason on the module. This is a recorded interim (F-0025 /
  F-0026), not a house style: the constants land the day either half is
  fixed, under the contract's own names.
- **The float family lives in its own module.** wolf has no overloading,
  so `abs`, `signum` and `round` cannot mean two things in one module.
  `std.math` is the integer family and `std.math.float` the `f64` one.
  The split is also what keeps each module importable into a native
  program, since two modules that declare a same-named function cannot be
  linked together yet (F-0026).
- **A sorted-input precondition is documented, not checked.**
  `binary_search` and the bound family assume ascending order and answer
  *something* on unsorted input instead of trapping, because verifying
  costs more than the search. `is_sorted` is the check, and the docs name
  it.

**§10 amendment (sc04): the iterator combinator is `limit(n)`, never
`take(n)`** (wolf-lang#16, ruled and closed). In wolf, `take` is a
parameter MODE that consumes its operand (X1). A combinator spelled
`xs.take(2)` that left `xs` usable would be a false friend at the exact
centre of the ownership story, and one that did consume it would be a
combinator nobody wants. The name is free to rule now because no
combinator has shipped. Every truncating combinator in std is `limit`,
and the family that follows it (`skip`, `step_by`, `rev`, `chunks`)
inherits the same test: a combinator name may not collide with an
ownership verb. `std.range`'s header records the ruling where the
blocked `rev`/`step_by` contracts are written down.

**§10 note (sc04): unicode tables are std-carried source.** wolf-lang#18
is ruled and closed. The category/case tables land as **committed
generated wolf source** under `std.unicode.tables`, behind the facade,
with a pinned Unicode version, a checked-in generator and a manual
regeneration step guarded by a drift check (the grammar-sync pattern).
Both implementations then execute the same wolf code and agree by
construction, where an intrinsic path makes Unicode a per-implementation
liability. Anything in std that classifies a code point cites that module
instead of growing a table of its own. §9's rule that a std function
matching a builtin's set carries a test pinning the agreement member by
member stands, and `tests/str/trim_whitespace_set.lu` remains the
worked example until the tables land.

**§9 amendment (sc28): `+` joins, interpolation converts, strbuf
builds.** D62 (`[type.str.concat]`) made `a + b` and `a += b` legal on
two strs at the sc27 pin, and `[type.str.concat.cost]` prices them as
the SAME interpolation-append lowering as `"{a}{b}"` — so the choice is
a reading, not a cost, and std spells it by role:

- **A pure two-str join is `+`** (`pad + s`, `sign + out`,
  `base + name`); **a self-append accumulation is `+=`**
  (`out += piece`). The operator names the operation; a two-hole
  interpolation at such a site was the pre-D62 spelling and reads as
  formatting where none happens.
- **A hole that CONVERTS keeps interpolation**: `"{b.s}{c}"` on a
  `char` (`[type.char.interp]` — the appended bytes are the scalar's
  UTF-8 encoding, which IS the contract at `strbuf.push`),
  `"{digits}{d}"` on an int. The mixes are refused by name
  (`str + char` / `str + int` are E0409 by ruling,
  `[type.str.concat.mix]`), so interpolation at these sites is
  load-bearing, not legacy.
- **Three or more pieces keep interpolation**
  (`"{sign}{head}.{tail}"`, `"{out}{sep}{c}"`): one hole-string reads
  better than an operator chain, and the lowering is identical anyway.
- **`+=` in a loop is still quadratic by the anchor's own words** —
  `std.strbuf` remains the builder. The adopted `out += piece` loops
  are the pre-existing interpolation-append loops made VISIBLE, not
  endorsed: each was already carrying that cost, and the operator now
  says so.

The sc28 measure over std/: 27 two-hole interpolation sites; 23
adopted (pure two-str joins and self-appends), 4 kept as conversion
holes (`strbuf.push`'s char, `std.fmt.decimal`'s int digits), every
3+-piece site kept. Zero verdict motion — the rig proved the whole
family is style (`[type.str.concat.cost]`, measured).

## 11. Formatting, text↔number, and encodings (sc05)

- **The format spec is a spelling; std owns the operations.** Every
  meaning `{x:spec}` can carry exists here as a FUNCTION:
  `fmt.pad_left`/`pad_right`/`center` (+ `_with` fill variants),
  `with_sign`, the base family, `decimal.to_str_fixed`/`to_str_exp`. None
  of it is a stopgap. A program that computes a column width, or pads a
  value it received as data, needs the operation as a function whatever
  the f-string can do, and the lowering is the ergonomic surface over the
  same semantics. The obligation the split creates: **the spec text and
  these functions must agree clause for clause**, which is why §7.4's
  candidate text was filed WITH this module as its reference
  implementation (F-0031 / F-0033) instead of after it.
- **Width is a BYTE count everywhere, and the doc says so.** `len` is
  bytes (§9), so padding is bytes. Display width needs East Asian Width
  and grapheme tables (the F-0019 budget question) and no core function
  pretends to know it. When the tables land, the display-width family
  arrives under its OWN name and `pad_left` keeps meaning bytes.
- **Exact beats approximate at the number boundary, and it is testable.**
  `std.fmt.decimal` rounds the EXACT value of a double (half-even), never
  a float approximation of it. `to_str` emits the shortest digit
  string that reads back as the same bits, and `parse_float` is correctly
  rounded. §10's "accuracy is a documented, measured, per-function
  contract" is satisfied here by there being no ulp budget to state: the
  contract is exactness, and the round trip
  (`to_bits(parse_float(to_str(x))) == to_bits(x)`) is a test over the
  torture corpus. A formatter that cannot say that about itself has not
  finished.
- **One value, one text, both directions.** Every encoder ships beside its
  decoder in the same module, the round-trip property is a test, and where
  the text direction is canonical (`base64.decode`'s strict profile) the
  module says which of the two round trips it promises and enforces the
  canonicity that makes the second one true.
- **Byte sequences are `List[int]` with a documented 0..255 element
  contract. That is an interim, and it is marked as one.** `std.bytes`
  cannot be written (F-0018/F-0035), and an encoder cannot wait for the
  type it converts. Element-range violations trap `assert` (a caller
  contract, §2) instead of encoding something the caller did not mean.
  Every signature keeps its shape when `Bytes` lands.
- **§2's inventory gains three tags**: **`base`** (a radix outside 2..36,
  the CALLER's mistake, distinct from `parse`'s bad DATA), **`deep`** (a
  nesting limit reached, which a serializer raises instead of exhausting
  the stack), and **`boundary`** (this pin cannot find the code-point
  boundary an operation needs; an interim tag that disappears with
  F-0018). **`boundary` is GONE as of sc14** — the first tag std has ever
  removed — and it went the way an interim should: `json.escape` became
  total, its three sites went to zero in one commit, and `stringify`'s row
  narrowed with it. An interim tag that is documented as temporary and then
  outlives its condition is worse than no tag at all, because callers write
  handlers for it. `overflow` is reused from §10 unchanged, for a decimal that
  rounds outside the finite range.
- **A row tag may not share a name with anything else in scope**, and this
  is a house rule only because the implementation makes it one: a
  colliding tag resolves to the module, function or binding it collides
  with and rides out as a VALUE, silently (F-0036). Three of this sprint's
  tags were renamed for it (`hex`→`parse`, `range`→`overflow`, and
  `std.json`'s `kind` function became `type_name` so the tag could keep
  its name). Until the filing closes, every new tag gets grepped against
  the module names in std and the item names in its own module.
- **No std accessor returns an enum through an error row.** The same
  shape was a silent wrong answer (F-0037), which is why `json.get`/`at`
  shipped as contracts. Return a total value, or a non-enum payload, or wait.
  **RETIRED at the sc12 pin**: the rule was written as an interim with F-0037
  as its exit, wolf-interp#16 closed at lupin 0.1.10 ("an enum variant is a
  value, not a raise"), and the one-line reproducer takes the value path on
  the lane that matters. The shape is legal now and the withdrawn functions
  are writable; the sprint that writes them owes F-0039 and F-0029 a fresh
  measurement first, because "the blocker retired" is not "writable" until
  every finding on the SIGNATURE has been re-measured (§14's sc10 rule,
  applied to the finding that taught it).
- **Locale is never a parameter, in core, ever.** Grouping separators are
  the caller's argument; digit-group size is fixed at three; no function
  reads an environment.

**§11 note (sc05): the parse family is re-homed.** `parse_int` and
`parse_float` moved from `std.str` to `std.fmt` / `std.fmt.decimal`. The
module that PRINTS numbers owns the whole number↔text boundary, so the two
directions can be read as inverses, and tested as inverses. The general
rule this sets, for the closeout to confirm: **a conversion belongs to the
module that owns the target representation, not to the module that owns
the source type.** `std.str` keeps the string operations and no longer
carries a digit table.

## 12. The error taxonomy (sc06), and §2's interim retired

This section is the binding statement about what an error row CARRIES.
§2 remains the binding statement about when a row is used at all.
`docs/error-taxonomy.md` is the audit behind it, holding every row
shipped in sc01–sc05, its sites, and its verdict. `std.errors` is this
section's worked home, as `std.option` is §2's.

- **Marks are lowercase, payload-carrying tags are CapCase.** A mark is a
  payload-free tag naming a failure mode: `none`, `gone`, `eof`, `done`,
  `parse`, `base`, `utf8`, `overflow`, `div_zero`, `deep`, ~~`boundary`~~
  (RETIRED at sc14 — see below), `syntax` (sc14, `std.json`'s malformed
  document), and, from the os tier, `not_found`, `denied`, `io`, `missing`,
  `invalid`, `refused`, `timeout`, `closed`, `signal`, and — from sc12
  (02-os)'s widened fs tier — `exists` and `cross_device`. A
  payload-carrying tag is CapCase and names its payload TYPE:
  `Parse(ParseErr)`. The case is the reader's signal about whether there
  is anything to destructure.
- **§2's spelling interim is RETIRED and the rename is applied.** std
  wrote `None`/`Done`/`Overflow`/`DivZero` from sc01 to sc05 because no
  implementation resolved a lowercase bare tag at a raise site (F-0003,
  whose ownership flipped implementations twice before dying at the sc05
  pins). All 148 occurrences across 32 files are lowercase as of
  sc06, at zero cost to the ledger: no row moved.
- **A payload is DATA, never a rendered string.** `ParseErr {offset,
  kind}` is the pattern: where it failed, and which way. A payload that
  carried a sentence would force every caller to accept this library's
  wording and would make the position unrecoverable. Rendering is
  `describe`'s job and lives beside the type.
- **One tag per failure mode the caller can ACT on**, never one per call
  site and never one per internal cause. `hex.decode` raising a single
  `parse` for "odd length" and "bad digit" is right while no caller
  branches on the difference. The day one does, the tag gains a payload,
  not a sibling.
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
  convention is recorded here so the io tier inherits it instead of
  inventing it. `errdefer` releases what the function acquired, `defer`
  releases what it borrowed, and neither ever changes the row.
- **A tag may not share a name with anything else in scope at the raise
  site** (§11's rule, restated because it is the sharpest edge in the
  area): the tag resolves to that thing and rides out as a value with no
  diagnostic (F-0036). Grep std's module list and your own module's items
  before naming a tag.

## 13. Test-authoring conventions (sc06), binding for every sc sprint

`std.testing`'s module header carries the same list. This is its
normative home, and `cargo xtask std-test --lint-conventions` enforces
the five rules that can be decided mechanically. The rest are judgement,
and a lint that guessed at them would train authors to work around it.

- **A trap ends the process, so the RIG is the catch mechanism.** There
  is no in-language trap catching, and D30 (no unwinding) rules it out.
  A trap expectation is a directive, `check: run(exit=trap(kind))`, never
  a `catch`.
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
- **Error-row expectations assert through `else |Tag(p)|`, tag AND
  payload.** The binding in `else |e|` is the TAG (`e.offset` is "error
  Parse has no member `offset`"), so a test that wants the payload
  destructures it in the pattern. `tests/errors/coarsen_and_chain.lu` is
  the worked example.
- **Table tests are a `List` of tuples plus a loop** until closures land
  (c05). The loop body is one indexing site, per the container rules.
- **Golden output rides the directive's `stdout=` hash**, and fixtures
  are shared instead of inlined twice.
- **Every test names its anchors** with `conforms:` (§4), and the doc
  examples are tests too: fenced means executable, always.

**§13 amendment (sc09): the rig denies warnings.** A non-empty `warnings`
array in the observation record (`[proto.record.warn]`) is a RED in
`cargo xtask std-test` and in `doc-examples`, on every lane that reports
one. `conform-run` still rejects `--deny-warnings` (F-0046, re-verified at
the sc09 pin), so the rig does it: the flag this repo asked for twice is
approximated by the one signal the protocol does give, and both executing
implementations populate it now (wolfc since s67, lupin since 0.1.6's lint
wave). It paid on its first run. Three doc examples carried `0.0 - x`
sites that W0402 had been flagging into a void. Two limits, stated so nobody
reads more into a green rig than it says: the array covers the ENTRY file
only, so a warning inside a std module body is invisible from here (F-0053's
open half), and a lane with no lint tier reports nothing at all, which is
not the same as reporting clean.

## 14. The os tier (sc07): capabilities, rows, paths, handles

Phase B's first two modules (`std.fs`, `std.io`) are the worked home of
this section, as `std.option` is §2's and `std.errors` is §12's. Every
rule here is a rule for the modules that follow them (net, time, process).

- **Every os-facing `pub` item names its CAPABILITY, in its own doc, by
  the I13 name** (`fs`, `io`, `net`, `env`, `exec`, `ffi`, `unsafe`,
  `comptime`), and says that comptime refuses it (D33). The module header
  states the capability once for the module. The per-function note stays
  anyway, because a reader arrives at a function and a manifest audit
  reads functions. A function in an os module that reaches NOTHING (the
  path helpers) says so explicitly, in the words "pure, comptime-safe",
  because the exception is the surprising case.
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
  every path-producing helper cuts at `/` only. A backslash inside a wolf
  string literal is an ESCAPE, so a windows path written literally does
  not lex. Tests and doc examples use RELATIVE paths and the rig gives
  each program its own working directory. A std test never names an
  absolute path or a host temp directory.
- **An os handle is consumed by its closer.** `close(take f)` is the
  pattern: the mode system, not a generation counter or a liveness flag,
  is what makes use-after-close impossible, and the rejection is held as a
  test (`tests/fs/use_after_close.lu`, expected `fail(E1001)`). A handle
  that a caller could forge from data answers with a row (`io`) instead
  of trapping, for the same reason a missing file does. An operation
  that advances a handle's POSITION takes `mut` even when no field of the
  value changes, because the stream's state is what the value names.
- **A capability module's lanes are honestly unequal, and the rig says so
  once.** An implementation may lack a capability entirely (lupin has no
  filesystem by design). Its ledger column is `unsupported` at resolve,
  and that is a posture recorded in `CONTRIBUTING.md`, not a defect noted
  in every test header. For doc examples the doc-truth rule becomes: at
  least ONE lane must reach `exit(0)`, and an honest refusal on the others
  is acceptable. That is a stronger requirement than "the reference
  machine runs it" for exactly the modules where the reference machine
  cannot.
- **What a capability module may not do to get a green lane**: emulate the
  capability, degrade silently, or ship an operation whose failure mode it
  cannot distinguish. `std.io.input_all` was the worked refusal. A loop
  over the line read could not tell `eof` from `io` at the sc07 pin
  (F-0043), so the function stayed a reviewed contract in the module
  header instead of a silent truncation. The sc11 amendment below records
  that it now ships.

**§14 amendment (sc08): the net tier, and the refusal rule restated.**
`std.net` is the third module of this section and the first whose whole
vocabulary is a network's. Everything above applies unchanged. Four additions,
each earned by writing the module.

- **The row vocabulary is the toolchain's, again and verbatim**:
  `{refused, timeout, closed, utf8, io}`. `closed` is the socket's `eof`. The
  peer finished, which is an OUTCOME a reader stops on and does not report,
  so `closed` joins §2's absence family and not the error family. `refused`
  and `timeout` are error marks. std adds no tag, no translation and no
  coarsening a caller did not ask for.
- **A tag a program cannot reach is documented as unreachable, not omitted.**
  `timeout` is declared by three builtins and no builtin can arm a deadline at
  this pin (F-0049), so every `std.net` signature carries the tag, every doc
  says it is unobservable here, and the day a deadline lands nothing about the
  surface changes. Removing it would be the lie: the row is the toolchain's.
- **One verb, one type, so the second closer wears a longer name.** wolf has
  no overloading, so a module with two handle types cannot spell `close` twice.
  The vocabulary word goes to the value programs handle most (`net.close`
  takes the `Socket`) and the other is qualified (`net.close_listener`). Both
  are `take`-consuming per §14's handle rule, and both hold their staleness
  discipline as a `fail(E1001)` test.
- **The pure member of a capability module is stated twice: on the function
  and in the module header.** `net.endpoint`/`net.loopback` touch nothing and
  are comptime-safe, exactly as `std.fs`'s path helpers are. In an os
  module the pure function is the surprising one, so the exception is spelled
  out and never left to be inferred from a missing capability note.

**§14's last rule, restated because sc08 paid it twice.** "What a capability
module may not do to get a green lane" now has two worked refusals, blocked by
the same finding from opposite directions: `std.io.input_all` (sc07) and
`std.net.read_all` (sc08, written and withdrawn inside the sprint). A loop over
a rowed read must stop on one tag and re-raise the others, and no handler can
tell them apart. At the sc08 pin the shape that should discriminate
(`else |e| match e { … }`) compiles, runs, and matches its FIRST ARM for every
tag on the executing lane, while the other two lanes get it right (F-0052).
The rule that decides both: a capability module ships no operation whose
failure mode it cannot distinguish, and the version that "works on the happy
path" is the version that loses someone's data.

**§14 amendment (sc10): time, env, and the tier that is not a capability.**
`std.time` and `std.env` are the fourth and fifth modules of this section and
the first whose builtin tiers BOTH compiler rungs execute. Everything above
applies unchanged. Four additions.

- **The capability name is the sandbox table's, and `Clock` is one of the
  eight.** I13's list as the toolchain spells it is `Io`, `Fs`, `Net`, `Env`,
  `Clock`, `Random`, `Ffi`, `Exec`. The two new modules take `Clock` and
  `Env`. A per-function capability note quotes the table's own
  reason as well as its name, because the reason is what a reader needs:
  `Clock` is refused at comptime for determinism alone (*two identical builds
  must not observe different times*) where `Env` is refused for determinism
  AND confinement (*contents differ per machine and may hold secrets*). One
  category, one sentence, and std does not paraphrase it.
- **A facade over a scalar ABI exists to make unit confusion a compile
  error.** The time builtins speak in bare milliseconds. `std.time` ships
  `Instant` and `Duration` as one-field structs, and that IS the module.
  `sleep(millis(2))` and `sleep(seconds(2))` cannot be mistyped for each
  other, an `Instant` can never be rendered as a date, and an elapsed span can
  never be added to a count of bytes. The rule this sets for the modules that
  follow: when a builtin tier hands std a bare `int` whose UNIT is the whole
  contract, std names the unit in a type, never in a parameter name. It
  costs nothing at run time (structs cross module boundaries completely) and
  it buys no privacy at this pin (a caller can forge one and read its field),
  so a forged value must produce arithmetic and never a trap.
- **Rendering is a documented posture, not an apology per function.** A
  formatter in an os module states once, in the header, exactly which profile
  it writes and what it deliberately has not got. `std.time`'s is: RFC 3339
  in UTC with three fractional digits, and NO parsing, NO timezones, NO
  calendar arithmetic and NO leap seconds, each with its reason (a zone is a
  database and §10's unicode-tables ruling applies verbatim; "one month later"
  is a policy, not a function). The one deviation, years outside `0..9999`, is
  stated and then held as a TEST, because a sentence about what an
  implementation answers is a test or it is a rumour (§13, sc09's rule).
- **A pure builtin family is not a capability, and it still does not get a
  free lane.** The `json_*` four carry no I13 tag and no sandbox category by
  design, so a package using only them declares nothing, and they still
  execute on ONE rung at s40. The doc-truth accommodation §14 wrote for
  capability modules ("at least one lane must reach `exit(0)`, and an honest
  refusal on the others is acceptable") is therefore about UNEQUAL LANES and
  not about capabilities, and the rig's list is named that way now. The
  distinction that survives: a capability lane may be dark forever by design
  (lupin has no sockets), where a tier lane is dark until someone writes the
  mirror.

**§14 amendment (sc10): D31's nursery, its banner and its clock, stated
normatively.** `std/x/README.md` is the register and `std.x.json` is the
worked home, as `std.option` is §2's and `std.errors` is §12's.

- **Every resident's module header carries the banner**: that `std.x.*` is
  not `std.*`, that its path, names, rows and behaviour may change or vanish
  in one sprint, that no other std module may depend on it (a resident MAY
  depend on the facade), and that the register is where its reason and its
  trigger are written down. One paragraph, in every resident, at the top.
- **The graduation clock ticks at campaign closeouts** and has exactly three
  outcomes, recorded in the closeout: GRADUATE (the module moves under
  `std.`, and because the path is the API the move is the release note),
  DELETE (the reviewed contract in its header is what survives), or EXTEND by
  one campaign with a written reason. Nothing sits in `x` unexamined. "Still
  useful" is not a reason; a named trigger is.
- **A resident is not an experiment.** Every one so far is a complete, tested
  body kept out of the facade because one refused or rejected body costs
  every importer of its module a lane. The reason is measured and named in the
  register, never "we are not sure about the API yet".
- **Check a resident's dependencies against its own lanes before writing a
  function.** A resident often has ONE executing lane, and a delegate that is
  refused on exactly that lane leaves a function with no test and no fenceable
  example, which is a claim nothing backs. `std.x.json.float_at` was
  written and withdrawn inside sc10 for precisely that (F-0061).
- **A resident and its facade successor cannot be imported by one program**
  (F-0058): module identity is the last path segment, so `std.x.json` and
  `std.json` are both `json`. Until that is ruled, a resident that shadows a
  facade name divides the work with it strictly and says so in both headers.

**§14 amendment (sc11): the process tier, and the three rules writing it
settled.** `std.process` is the sixth module of this section and the first
whose capability can end another program. Everything above applies unchanged.
Four additions.

- **The row vocabulary is the toolchain's, for the fourth time, and it gains
  one mark**: `{not_found, denied, io}` for starting a child (the fs tier's
  three, exactly), `{signal, io}` for waiting on one, `{io}` for killing one.
  **`signal`** is new to §12's inventory: the child died WITHOUT an exit code.
  It is a payload-free mark by §12's casing rule, and it is deliberately not a
  number. Every fake code a library could invent collides with a code some
  program really returns, and "it did not exit" is a different sentence from
  "it exited with 137". The day the builtin reports WHICH signal, the tag
  gains a payload, not a sibling (§12's rule 4).
- **When the language owns the verb, std takes the next word, and says
  why.** `spawn` is one of the 50 keywords, so `std.process.start` is the
  spelling (F-0062), exactly as `copy` made `std.fs.copy_file`. The rule this
  sets: a name collision with a keyword is a paragraph on the function, never
  a doc-free rename, because the reader arriving from another language types
  the keyword first. The same paragraph is what keeps a std verb from
  competing with a language concept. A child process and a task are different
  failure models and should not share a word.
- **A builder is PURE, and in a capability module that is worth stating three
  times**: on the module, on each builder function, and in the ledger's shape.
  `command`/`from_argv`/`push_arg`/`argv`/`is_success` reach nothing, so they
  are comptime-safe and they are what gives the module its second lane under
  an implementation that declines the capability entirely (lupin resolves
  module bodies lazily). §14's sc08 rule said the pure MEMBER of a capability
  module is stated twice. The process tier adds that a pure builder is a
  DESIGN choice with a lane consequence: a facade whose construction touched
  the host would have had one lane and no fenceable example for its own
  constructor.
- **A capability whose happy path cannot be witnessed portably says so, on
  the module and in a filing.** No program exists on every tier-1 host, a wolf
  program cannot learn its own path, and the directive schema has no
  per-platform gate, so no `.lu` test in this repository can start a real
  program (F-0066). What `tests/process/` witnesses instead is every ROW: an
  empty argv, a name no host has, a forged handle. The module header
  states plainly that the exit-code path rests on the toolchain's own
  unix-gated tests. This is a weaker standard than §13's "a claim about what
  an implementation answers is a test or it is a rumour", and the honest
  response is to name the gap in three places, never to let a green rig
  imply coverage it does not have.

**§14 amendment (sc11): the refusal rule, and its first two RETRACTIONS.**
§14's "what a capability module may not do to get a green lane" had two worked
refusals, `std.io.input_all` (sc07) and `std.net.read_all` (sc08, written and
withdrawn inside the sprint). Both are shipped in sc11, and the rule is
unchanged by that. That is the point worth recording.

Neither function became writable because std lowered its standard. Both needed
a loop that stops on ONE tag and re-raises the others, both were blocked by
handlers that could not discriminate (F-0043's binding catches everything;
F-0052's `match` took its first arm on the executing lane), and both waited
for the toolchain: s70's match tier and s71's payload-pattern ruling. **The
refusal was the mechanism that got them written.** A version shipped in sc07
would have treated a read error as the end of the stream, would have passed
its tests, and would still be quietly truncating data at this pin. Nothing
would have been filed, so nothing would have been fixed.

The rule the pair now states, in full: **a capability module ships no
operation whose failure mode it cannot distinguish; it writes the contract
into the module header with the finding that would unblock it; and it ships
the function in the sprint after the finding closes.** The third clause is
new and it is a debt, not a courtesy. sc10's census had to name both
functions as "no longer blocked, still unwritten" to keep them from sitting
quietly, which is one sprint of grace and no more.

**§14 amendment (sc13): the debt clause, paid once and re-armed once.**
sc11 added the third clause of the refusal rule — *a capability or tier module
writes its contract into the module header with the finding that would unblock
it, and ships the function in the sprint after that finding closes.* sc13 is
the first sprint to be on the receiving end of it twice, and both halves are
worth recording because they are the clause working rather than the clause
being invoked.

- **Paid.** F-0037 (an enum returned through an error row is always a miss)
  closed at sc12's pin. `std.json.get` and `std.json.at` — written, tested and
  withdrawn at sc05 — ship here, in the next sprint, with the signatures the
  contract wrote and no amendment. The rule's value is visible in that last
  clause: a contract written while the function was unwritable turned out to
  be right, because it was written as a specification rather than as an
  apology.
- **Re-armed.** F-0057 (nothing builds a `str` from a number) closes at THIS
  sprint's pin, and `std.bytes.to_str` — the one function the sprint brief
  named — ships with it. Everything ELSE that finding was blocking is now owed
  at sc14 and says so in its own header: `std.json.unescape`, `std.json.parse`
  behind it, `std.json.escape`'s totality (a signature change, because it
  removes `boundary` from `stringify`'s row), and `std.hex.decode_str` (§11's
  encoder/decoder pairing, refused for four sprints). Naming them in four
  headers rather than in one closeout is the point: the debt lives where the
  function is missing.

Two additions the pair earned, both about EVIDENCE rather than about surface.

- **A pure tier gets its row vocabulary adopted verbatim too.** §14 wrote the
  rule for capability tiers (fs, io, net, process) and sc10 extended the
  lane-honesty half to pure ones (`json_*`). `to_str`'s `{utf8}` is the third
  case and the cleanest: the row is `str_from_utf8`'s, unrenamed and
  uncoarsened, and the test that proves it is not a handler asserting the tag
  but the RECORD naming it (`error: utf8`, exit 1). A wrapper that renamed or
  widened a row would look identical from inside a handler, which is exactly
  why the propagation test is the one that matters.
- **When a std function wraps a validating primitive, its test walks the
  primitive's whole refusal surface through the FACADE.** Not a sample — every
  named class, plus the accepted rules that are easy to lose (an empty input,
  an interior NUL, the boundary value adjacent to the first invalid one). And
  where std already had an independent implementation of the same predicate
  (`bytes.is_utf8`), it KEEPS it and a test holds the two to the same answers.
  Delegating the predicate to the primitive would have made the agreement a
  tautology and cost the predicate a lane; two implementations that must agree
  is a stronger arrangement than one, and this is the case that showed it.

**§14 amendment (sc14): the debt clause, PAID IN FULL — and what a contract
is worth when it comes due.** sc13 re-armed the clause on four functions and
named them in four headers. sc14 ships all four (`json.parse`,
`json.unescape`, `json.escape`'s totality, `hex.decode_str`) with no
extension and no renegotiation, which makes this the clause's first complete
cycle: armed, named where the function was missing, and paid on the date.
Four rules the payment settled.

- **A contract written as a specification survives its wait; a contract
  written as an apology does not.** Every signature sc05 and sc10 wrote for
  these four was still right, except where a MEASUREMENT had moved under it,
  and both movements are worth the ink. `unescape`'s tag changed from `parse`
  to `syntax` because the module gained a `pub fn parse` and F-0036 makes a
  tag that shares a name with anything in scope resolve to that thing — so
  **a new `pub fn` must be checked against its module's TAGS, not only a new
  tag against the module's functions**; the collision is symmetric and only
  one direction has ever been written down. And `parse`'s row gained
  `overflow`, because the number tier refuses a magnitude with no `f64` and
  §12 gives a distinct actionable failure its own tag: "this document is
  malformed" and "this number has no `f64`" are different sentences and a
  parser that conflates them lies about the caller's data.
- **Re-ask what the function NEEDS, not whether its named blocker closed.**
  `json.escape`'s refusal was documented for five sprints as waiting on
  `str.code_points` — a code-point walk, to step over a character it must not
  escape. It needed no primitive: the characters JSON escapes are all ASCII,
  no byte of a multi-byte code point is ASCII, so a BYTE walk never has to
  know it is inside one. It had been writable since s37. sc10's rule ("the
  blocker retired" is not "writable") has a blind side, and this is it — a
  contract can name the wrong blocker and then be believed for years.
- **A signature change is a paragraph, not a policy.** Making `escape` total
  removed `boundary` from `stringify`'s and `stringify_pretty`'s rows. The
  cost was eight annotations, one test file and one doc example; callers that
  wrote `else "?"` still compile, and a caller that named the tag in a handler
  does not. std states such a change in the module header, in the census and
  in the sprint report, and makes it in the sprint that makes it possible. A
  library at this stage does not carry a tag to avoid a change.
- **A blocked lane is not a blocked function until you check WHOSE lane it
  is.** §14's sc10 rule (check a resident's dependencies against its own
  lanes) generalizes: `std.json` executes on the interpreter alone, so
  `unescape` — which builds a `str` from a `\uXXXX` scalar — needed
  `str_from_utf8` ON THAT LANE, not merely in the language. At sc13's pin it
  would have had zero lanes and would have been withdrawn like
  `x.json.float_at`. It ships because lupin 0.1.12 closed F-0075 one release
  after it was filed. The rule for the register: **file the mechanism, not
  the symptom** — "this machine cannot build a `str` from bytes" is what the
  next sprint reads, where "one ledger column is dark" is not.

**§14 amendment (sc14): a wildcard handler is also right when the module
cannot SEE the difference.** sc08's rule was that a wildcard claiming nothing
is the right handler when a module cannot ACT on the difference between tags.
F-0079 adds the harder half **[F-0079 CLOSED at lupin 0.1.13 — wolf-interp#29's arm-selection pass; the register's sc22 closure note has the timeline. The amendment's two honest shapes remain good doctrine on their own merits]**: under lupin (0.1.12, the pin this was measured at) a multi-arm `else |e| match e` over
a row raised ACROSS A MODULE BOUNDARY takes its first arm, silently, in both
arm orders — the compiler rungs are correct, the interpreter is not, and it
is F-0052 exactly, on the other machine, two sprints after that one closed.
So a std body that must discriminate an imported module's tags cannot, on
that lane, and the shapes that stay honest are: a wildcard where only one tag
is reachable (with the reachability argued in the doc), and one TEST FILE PER
TAG riding it out of `main`, where `error: <tag>` is the toolchain's word and
not this repository's. That second habit is what caught the finding — a
handler said `syntax` and the record said `deep` about the same document — so
the rule it earns is general: **when two ways of observing one value
disagree, bisect the observers before the code.**


**§14 amendment (sc12, 02-os): what happens when the tier a module was
writing around finally lands.** `std.fs` is the first module of this section
to have its OWN contracts filled rather than a new module's written, and the
five rules that came out of doing it are all about the second half of a
capability's life.

- **A row is deleted the day its delegate stops raising it.** `append_text`,
  `copy_file` and `move_file` all carried `utf8` because all three DECODED —
  the append read the file back, the copy and the move were `read_text` +
  `write_text`. None of them decodes now, so `utf8` came off all three
  signatures in the same commit as the bodies. This is §14's union rule read
  in the shrinking direction, and it needs saying because the growing
  direction is the one that feels like work: **a tag that outlives the
  delegate that raised it is a lie a caller writes a handler for**, and it
  costs a caller more than an added tag ever does.
- **A row is NOT added for a tag the function's own arguments cannot reach,
  and the coarsening goes in a private helper.** `fs_open_mode` declares
  five tags across its five modes; with the mode fixed at 2 by std rather
  than by a caller, `invalid` (a mode outside the set) and `exists` (mode 4
  losing its exclusive create) cannot happen, so a private `append_fd`
  handles both — answering `io`, which is truthful if a future pin ever does
  reach them — and `append_text`/`open_append` carry three tags instead of
  five. The same judgement leaves `invalid` off `copy_file` (its bytes came
  out of `fs_read_bytes`) and ON `write_bytes` (the list is the caller's).
  **Whose data it is decides whether a tag is reachable**, and two functions
  that differ only in that should sit near each other so the difference is
  readable.
- **A contract can be answered by a DECISION, and then it is withdrawn
  rather than left open.** `std.fs.rename` was a reviewed contract for an
  ATOMIC move. The language deliberately does not promise atomicity — POSIX
  replaces a destination atomically, windows `MoveFileEx` is documented to
  replace but not to replace atomically, and upstream's platform rule says a
  promise that cannot be kept on a tier-1 target does not get a `#[cfg]`
  keeping it on two out of three — so there is no `fs_rename_atomic` and
  there will not be. std adopts that reading instead of re-promising it one
  level up: `move_file` is the wrapper, there is no second name for the same
  call, and the module header records the withdrawal with the reason. A
  contract left open forever because its exact words were never met is worse
  than a withdrawal that says what happened.
- **When the toolchain hands std a tag whose TRIGGER has no portable litmus,
  the tag is still documented and the gap is stated in three places.**
  `cross_device` appears in no `std.fs` signature because `move_file`
  HANDLES it, and its fallback therefore cannot be reached by any test in
  this repository (it needs two filesystems). §14's sc11 posture applies
  unchanged: say so on the function, in the module header and in the report,
  and never let a green rig imply coverage. What DID become witnessable is
  worth the same honesty in reverse — `utf8` had been declared on
  `fs.read_text` since sc07 and observed nowhere, and the moment
  `write_bytes` existed it got the same standard of evidence as
  `not_found`: a tag ridden out of `main` where the record names it.
- **An optimization that lands does not license undoing the algorithm it
  forced.** §9's sc12 amendment ruled that a shape costing a lane is not a
  shape std writes, and the lane cost is gone (F-0071 closed at this pin:
  the checked tier models all seven byte-view positions). std's walks did
  not go back to indexing, because the one-pass forms are SHORTER than what
  they replaced. The rule the pair leaves: a constraint that produced a
  better body has paid for itself, and the closure buys the NEXT body an
  option rather than obliging this one to change. Where the code stays, the
  doc must still move — a comment claiming a live refusal that has closed is
  the sc13 failure mode, so each of the six sites now reads "was refused
  when this was written, closed at the sc12 pin, kept because".

## Review record

- 2026-08-30, sc29 amendments (the net byte tier + the deadline, and the
  first PROTOCOL CLIENT): §14's os-tier vocabulary reaches sockets' binary
  half — `net.read_bytes`/`write_bytes` carry `List[int]` with the `invalid`
  tag for a non-byte element, verbatim from `fs`'s byte tier (no `utf8` row:
  a lone `0x80` is data), and the `timeout` tag `std.net` declared from sc08
  becomes REACHABLE via `set_deadline`/`set_listener_deadline` over s106's
  `net_deadline` — a declared-early row paid as a PURE ADDITION (no signature
  changed, F-0049's deadline half closed). The lane split is recorded, not
  designed: the str tier and the deadline are three-lane, the byte pair is
  compiler-lanes-only (lupin 0.1.18 does not resolve it, F-0102), and a
  facade keeps its other lanes because lupin resolves module bodies lazily.
  A protocol CLIENT (`std.x.tls.client`) adds the reviewed pattern for a
  blocking, no-reactor tier: a two-PHASE handshake (`begin` writes without
  reading, `complete` reads the flight) so an in-process client+server
  witness alternates with kernel-buffered flights instead of deadlocking a
  single thread — the shape stated in the module header as an API decision;
  every adversarial input a NAMED row (the negative battery, pure and
  three-lane); the CertificateVerify verify a DISPATCH that answers
  `unsupported_alg` for a scheme it cannot check, never a silent accept; and
  a descriptor held as an `int` (rebuilding the `net.Socket` wrapper per
  call) where a `mut self` method would otherwise trip F-0092. Review rides
  the same pending sc00 gate.

- 2026-08-22, sc12 (02-os) amendments: §14 gains the second-half-of-a-
  capability rules (a row deleted when its delegate stops raising it; a row
  not added for a tag the arguments cannot reach, with the coarsening in a
  private helper and "whose data it is" as the test; a contract answered by a
  DECISION and therefore withdrawn, with `rename`'s atomicity as the worked
  case; a handled tag documented anyway when its trigger has no portable
  litmus, and its mirror — a declared tag gaining a witness the day the
  surface allows one; and the rule that an optimization's closure does not
  license undoing the algorithm it forced). §12's mark inventory gains
  `exists`, `invalid` (reused) and `cross_device`. Review rides the same
  pending sc00 gate.

- 2026-08-21, sc14 amendments: §14 gains the debt clause's first COMPLETE
  cycle (the four contracts paid on the date, the symmetric tag/function
  naming rule, `parse`'s extra `overflow` tag, the re-ask-what-it-needs rule
  that cost `json.escape` five sprints, the signature-change posture, and the
  whose-lane-is-it rule with F-0075's closure as its worked case), plus the
  wildcard-handler rule's second half (F-0079: a multi-arm handler cannot
  discriminate an imported module's tags under lupin) and the
  bisect-the-observers rule it earned. Review rides the same pending sc00
  gate.

- 2026-08-20, sc13 amendments: §14 gains the debt clause's first payment and
  its re-arming (the four functions F-0057's closure now owes, each named in
  its own header), the verbatim-row rule extended to a pure tier's `{utf8}`,
  and the evidence rule for wrapping a validating primitive. Review rides the
  same pending sc00 gate.

- 2026-08-19, sc12 amendments: §9 gains the byte-view rules (a shape that
  costs a lane is not a shape std writes, even when the cost is only speed;
  do not assert a property this rig cannot observe; a doc explaining a body's
  shape names the constraint and not only the wave), and §11's
  no-enum-through-a-row rule is RETIRED with F-0037's closure at lupin
  0.1.10. Review rides the same pending sc00 gate.
- 2026-08-18, sc11 amendments: §14 gains the process tier (the row
  vocabulary's fourth verbatim adoption with the new `signal` mark, the
  keyword-collision naming rule, the pure-builder rule and its lane
  consequence, and the unwitnessable-happy-path posture), §14's refusal rule
  gains its first two retractions with the three-clause rule they settle, §12's
  mark inventory gains the os tier's nine, and §9's sc09 amendment gains its
  other half: a guard written where the lanes disagree is DELETED the day the
  toolchain rules, even when the ruling contradicts std's reviewed contract.
  Review rides the same pending sc00 gate.
- 2026-08-17, sc10 amendments: §14 gains the time/env tier (the sandbox
  table's capability names with their own reasons quoted, the
  facade-over-a-scalar-ABI rule that makes unit confusion a compile error, the
  render-only posture stated once and held as a test, and the ruling that the
  unequal-lanes accommodation is about lanes and not about capabilities), and
  §14 gains D31's nursery normatively: the banner every resident carries, the
  three-outcome graduation clock, the measured-reason rule, the
  check-your-lane rule that cost `float_at`, and F-0058's
  resident-versus-facade import collision. Review rides the same pending sc00
  gate.

- 2026-08-16, sc09 amendments: §9 gains the four rules writing the landed
  `str`/`bytes` surface settled (`get` versus `s[a..b]` by whose offset it
  is; a std function decides where the lanes disagree about a primitive, and
  files; the `int` code-point interim shared with `std.unicode`; byte width
  as `List[int]` with monomorphic-beats-generic and the shared-name rule),
  plus the predicate-does-not-trap ruling. §13 gains the warning gate.
  Review rides the same pending sc00 gate.
- 2026-08-10, drafted (sc00). Human review: **pending**; record the
  reviewer and date here when it lands, then flip Status above to
  binding.
- 2026-08-10, sc01 amendments: §2 absence inventory (with the two
  filed interims), §6 ordering, §7 assert. Review rides the same
  pending sc00 gate.
- 2026-08-11, sc02 amendments: §4 statement-vs-assertion rule for
  fenced examples plus the one-module note, and §8 Containers. Review
  rides the same pending sc00 gate.
- 2026-08-12, sc03 amendments: §2 gains the `parse` tag (and reserves
  `utf8`) with the interim's ownership flipped to lupin, and §9 Strings.
  Review rides the same pending sc00 gate.
- 2026-08-12, sc04 amendments: §10 Numerics (checked/recoverable/
  saturating, the `overflow` and `div_zero` error tags, the trap-versus-
  raise rule, the measured accuracy contract, the pure-wolf decision,
  constants-as-functions as a recorded interim, the float module split,
  and the documented sorted-input precondition), plus the two closed
  rulings: the iterator combinator is `limit(n)` and unicode tables are
  std-carried committed source. Review rides the same pending sc00 gate.
- 2026-08-13, sc06 amendments (the campaign's last): §12 The error
  taxonomy (mark-versus-payload casing, payloads as data, one tag per
  actionable failure mode, absence-is-not-an-error, coarsening as a named
  call, the chain-as-a-field idiom, int kinds as a recorded interim, the
  `errdefer` convention Phase A records without using, and §2's
  `None`-spelling interim retired with the rename applied tree-wide),
  and §13 Test-authoring conventions, five of whose rules are enforced by
  `cargo xtask std-test --lint-conventions`. Review rides the same
  pending sc00 gate. **§2's helper inventory is no longer blocked**:
  `std.option`'s six are written and executing (four in the facade, two
  in the nursery behind F-0039/F-0040).
- 2026-08-13, sc05 amendments: §11 Formatting, text↔number, and encodings
  (the spec-is-a-spelling split with its agree-clause-for-clause
  obligation, byte width, exactness at the number boundary, encoder/decoder
  pairing, the `List[int]` byte interim, the `base`/`deep`/`boundary` tags,
  the tag-name-collision and enum-through-a-row house rules, and the
  re-home rule for conversions). Review rides the same pending sc00 gate.
- 2026-08-15, sc08 amendments: §14 gains the net tier (the
  `{refused, timeout, closed, utf8, io}` vocabulary adopted verbatim, an
  unreachable tag documented and not omitted, the one-verb-one-type rule
  for a module with two handle types, and the pure-member exception stated
  twice), plus §14's refusal rule restated with its second worked refusal
  (`std.net.read_all`, withdrawn inside the sprint, F-0052). Review rides the
  same pending sc00 gate.
- 2026-08-14, sc07 amendments (Phase B opens): §14 The os tier
  (capability notes per I13, the toolchain's row vocabulary adopted
  verbatim, paths as forward-slashed `str`, `take`-consumed handles with
  the rejection held as a test, and the capability-lane posture with its
  stronger doc-truth rule), plus §4's propagating-statement amendment.
  Review rides the same pending sc00 gate.
