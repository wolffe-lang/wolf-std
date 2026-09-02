# The error taxonomy — the sc06 audit, extended at sc07

Every error row shipped in sc01–sc05, audited against the rules
API-CONVENTIONS §12 now states, with the verdict per tag and the changes
sc06 made; the table and the last section carry sc07's io tier (`std.fs`,
`std.io`) as well, which is the first sprint to spend the `eof`/`utf8`
reservations and the first to add tags std cannot test.

The measurements are mechanical (a script over
`pub fn` signatures in `std/`, re-runnable) so the numbers in the alias
filing (F-0041) can be checked rather than believed.

## The shape of std's error surface, measured

| measure | value |
|---|---|
| `pub fn` signatures carrying a row | 49, across 16 modules |
| distinct row SHAPES | 11 |
| signatures with one tag | 45 |
| signatures with two tags | 4 |
| signatures with three or more tags | **0** |
| distinct tags in use | 9 |
| payload-carrying tags before sc06 | **0** |
| payload-carrying tags after sc06 | 1 (`Parse(ParseErr)`, `std.errors`) |

The four two-tag rows are `std.json.stringify` and `stringify_pretty`
(`{boundary, deep}`), `std.fmt.decimal.parse_float` (`{parse, overflow}`)
and `std.fmt.parse_int_base` (`{parse, base}`). Nothing in core needs a
third tag yet, which is the honest answer to the sprint contract's
question "which signatures exceed three tags": none do, and the alias
surface is filed on the io taxonomy's behalf rather than on core's
(F-0041 carries the argument and these numbers).

## Every tag, its sites, and its verdict

Verdict key: **conforming** — lowercase mark, one failure mode a caller
can act on, no payload needed; **renamed** — nonconforming at sprint
start, fixed in this sprint's rename; **watch** — conforming but with a
recorded question for a later sprint.

| tag | sites | payload | verdict |
|---|---|---|---|
| `none` | 23 (`list.get/first/last/pop`, `map.get/remove`, `search`'s six, `rand.choose`, `str.strip_prefix`, `unicode.from_code/utf8_len`, `x.deque_int`'s four, `x.list_eq`'s two, `x.option_flatten.flatten`) | never — §12 rule 4 | conforming; **renamed** from `None` |
| `parse` | 8 (`fmt.parse_bool/parse_int/parse_int_base`, `fmt.decimal.parse_float`, `hex.decode`, `base64.decode/decode_lenient/url_decode`) | none today; `Parse(ParseErr)` is the landing shape | conforming; **watch** — the payload is written (`std.errors`) and the retrofit is costed below |
| `kind` | 8 (`json.as_int/as_float/as_bool/as_str/as_arr/as_obj/keys/len`) | none | conforming — "this JSON value is not of that kind" is one actionable mode; the tag already forced `json.kind` the function to be renamed `type_name` (F-0036) |
| `overflow` | 4 (`math.checked_add/sub/mul`, `fmt.decimal.parse_float`) | none | conforming; **renamed** from `Overflow` |
| `boundary` | 3 (`json.escape/stringify/stringify_pretty`) | none | conforming, and **temporary by design**: it means "this pin cannot find the code-point boundary I need" and disappears with F-0018 |
| `base` | 2 (`fmt.parse_int_base`, `fmt.to_str_base`) | none | conforming — a radix outside 2..36 is the CALLER's mistake and distinct from bad data |
| `done` | 2 (`iter.range_next`, `iter.list_next`) | none | conforming; **renamed** from `Done`. Exhaustion is its own noun, deliberately not `none` |
| `deep` | 2 (`json.stringify/stringify_pretty`) | none | conforming |
| `div_zero` | 1 (`math.checked_div`) | none | conforming; **renamed** from `DivZero` |
| `io` | 13 (`fs.read_text/write_text/append_text/remove/copy_file/move_file/open/create/read/write/close`, `io.input_line/prompt`) | none — and it is already a COARSENING: every host failure that is not `not_found` or `denied` arrives as `io` (§12 rule 3, and the builtin tier decides it, not std) | conforming; **watch** — the day a caller can act on "disk full" versus "broken pipe" it needs a payload, and F-0043 must close first or the caller cannot read one |
| `not_found` | 7 (`fs.read_text/write_text/append_text/remove/copy_file/move_file/open`) | none today; `NotFound{path}` is the landing shape the sprint contract named | conforming; **watch** — the payload is what a caller wants (WHICH path was missing, in a chain of them) and it is blocked on F-0043, not on this taxonomy |
| `utf8` | 5 in `std.fs`/`std.io` (`fs.read_text/read`, `fs.read_dir`, `io.input_line/prompt`) plus `bytes.to_str` and `std.net.read` | none | conforming, and **witnessed at last**: `fs.write_bytes` can make a file that is not text, so `tests/fs/utf8_row.lu` rides the tag out of `main`. It also LEFT three signatures at the sc12 (02-os) pin — `append_text`, `copy_file`, `move_file` stopped decoding anything (F-0045/F-0044 closed), and a row a function can no longer raise does not stay in it |
| `denied` | 17 (the whole `std.fs` host-reaching surface, `fs.create` included — whose row is `{denied, io}`, since a directory it cannot write is `io` there, not `not_found`) | none | conforming, and **still untestable**: making a file unreadable needs a permission call the language does not have. It is the last fs tag with no witness now that `utf8` has one, and the module header says so in those words. Documented per function, observed nowhere |
| `eof` | 3 (`fs.read`, `io.input_line`, `io.prompt`) | none | conforming — an END is an outcome, not a failure, which is why it is its own noun rather than `none` (the same ruling as `done`) |
| `gone` | 0 in std (the language's own `weak.upgrade`, `[mem.shared.rc.3]`) | none | conforming, reserved |

## What sc06 changed

**The rename, executed tree-wide.** `None` → `none`, `Done` → `done`,
`Overflow` → `overflow`, `DivZero` → `div_zero` — 148 occurrences across
32 files (std bodies, tests, and the doc comments that name the tags).
The interim existed because neither implementation resolved a lowercase
bare tag at a raise site (F-0003, whose ownership flipped between
implementations twice); it was retired at the sc05 pins and the rename
was deliberately left as a separate mechanical commit, which is this one.
**Cost: zero.** The whole rig — 127 tests, 211 doc examples, three lanes
— is byte-identical either side of the rename.

**One payload type exists now.** `std.errors.ParseErr {offset, kind}` is
the exemplar §12 rule 2 describes, with `Failure {offset, cause}` as the
coarse target of `coarsen`. They are the first CapCase tags in std
(`Parse(ParseErr)`), and the audit's recommendation is that they stay the
only ones until a caller can act on the difference.

**The `parse` retrofit is costed, not done.** Turning
`hex.decode -> List[int] ! {parse}` into `! {Parse(ParseErr)}` would let
a caller say WHERE the input stopped being hex, which is a real
improvement. It is not applied in sc06 for a measured reason: the tag's
payload type has to come from `std.errors`, and every module that gains
the import gains its lane story too — plus the change is API-breaking in
the direction that matters least (a caller who only wants "it did not
parse" must now destructure). The recommendation recorded for stdc02:
retrofit `parse` when the io tier gives a second caller who branches on
position, and do it in one commit across `fmt`, `hex` and `base64` so
that the family stays one shape.

## The rules the audit produced

They are stated normatively in API-CONVENTIONS §12; the short form:

1. Marks are lowercase and payload-free; payload-carrying tags are
   CapCase and name a payload TYPE.
2. Payloads carry data (position, limit, the offending kind), never
   pre-rendered strings. `describe` is the only place words are made.
3. One tag per failure mode a caller can act on — not one per call site,
   and not one per internal cause.
4. Absence (`none`) is not an error and never carries a payload.
5. Coarsening is a named call the caller writes (`errors.coarsen`), never
   an implicit conversion.
6. A tag may not share a name with anything else in scope at the raise
   site — the implementation resolves the collision into a value,
   silently (F-0036). Grep before naming.
7. `errdefer` is the cleanup form on the error path only; in Phase A no
   std function owns a resource that needs it, so the convention is
   recorded and unused rather than invented — the first `errdefer` in std
   will be in the io tier, releasing a handle it opened.

## The io tier's five tags (sc07, Phase B opens)

`std.fs` and `std.io` add five marks and adopt them VERBATIM from the s38
builtin tier — `not_found`, `denied`, `io`, `utf8`, `eof` — which is a
decision worth recording as one: std could have defined its own error type
over the builtins and did not. Three reasons. The tags are already the
right granularity (one per response a caller can choose); a translation
layer would have to invent a mapping and then defend it; and a row that
matches the builtin's exactly means `?` propagates from the builtin to the
caller through std without widening, so `std.fs.read_text` costs nothing
that `fs_read_text` does not.

What the tier reveals about §12's rules:

- **Rule 3 (one tag per actionable mode) is doing real work here.** `io`
  covers every host failure that is not "missing" or "refused", and no
  caller can act on the difference between a full disk and a broken pipe
  in wolf today — there is no retry policy, no free-space query, nothing.
  The day one can, the tag gains a payload; it does not gain siblings.
- **Rule 2 (a payload is DATA) has a blocked landing.** `NotFound{path}`
  is the obvious next step and the sprint contract named it in advance.
  It cannot land while a payload pattern in an `else` handler is a wolfc
  rejection (F-0043): a payload nobody can destructure is worse than a
  mark, because it looks like progress.
- **Two tags ship documented and unobserved.** `denied` needs a
  permission change and `utf8` needs a file of invalid bytes; neither is
  writable from wolf at this pin (F-0044). §10's "accuracy is a measured
  contract" has an error-tier counterpart, and this is it: a tag std
  cannot witness says so on the function and in the test header, and no
  test claims otherwise. **(One of the two came off this list at the sc12
  (02-os) pin: `fs.write_bytes` makes a file that is not text, so `utf8`
  is witnessed the same way `not_found` is. `denied` is still there, and
  `cross_device` joined it — a tag `move_file` HANDLES rather than raises,
  whose trigger needs two filesystems.)**

## The os tier's later tags (sc08, sc10, sc11)

Each module after `std.fs`/`std.io` adopted its builtin tier's vocabulary
verbatim, for the three reasons above, and the inventory is now:

| tag | added | meaning | family |
|---|---|---|---|
| `not_found` | sc07 | no such path — and, from sc11, no such PROGRAM (an empty argv names none either) | error |
| `denied` | sc07 | the host refused | error |
| `io` | sc07 | every other host failure, one tag by rule 3 | error |
| `utf8` | sc07 | bytes that are not text | error |
| `eof` | sc07 | the input ended | absence |
| `refused` | sc08 | nobody is listening | error |
| `timeout` | sc08 | a deadline expired — declared, unreachable (F-0049) | error |
| `closed` | sc08 | the peer finished: the socket's `eof` | absence |
| `missing` | sc10 | no such environment variable | absence |
| `invalid` | sc10 | a variable name the platform cannot hold | error |
| `signal` | sc11 | the child died with no exit code | error |

Three observations the later modules added, each earned by writing one:

- **An absence tag is what ENDS a loop, and an error tag is what a loop
  re-raises.** `eof` and `closed` are the same shape in two vocabularies, and
  sc11 is where it paid: `io.input_all` and `net.read_all` are one loop each
  that stops on the absence tag and re-raises everything else UNCHANGED, so
  neither tag appears in either function's row. The rows are the caller's
  business and the absence is the function's — which is only writable at all
  because a handler can finally tell them apart (F-0043/F-0052, both closed).
- **`signal` is the taxonomy's first "no value at all" ERROR.** Every other
  error mark here means an operation did not happen; `signal` means it
  happened and produced no number. It is not an absence (nothing is missing —
  a child ran), and it must not be a code (every invented code collides with
  a real one). Rule 4 covers its future: the day the builtin says WHICH
  signal, the tag gains a payload, not a sibling.
- **A tag with no witness is still documented, and sc11 has two of them**:
  `denied` for a program the host will not execute, and `signal` itself,
  because no `.lu` test in this repository can start a real child to kill
  (F-0066). §10's "accuracy is a measured contract" has its error-tier
  counterpart here — a tag std cannot witness says so on the function, in the
  test header and in the census, and no test claims otherwise.

## sc14: the first tag std has RETIRED, and the reader's two new ones

Re-measured over `std/` at the sc14 pins (20 distinct tags, 15 two-tag
signatures, 21 with three or more — the os and query tiers are where the
wide rows live, and `std.json.parse` is the first three-tag row in the pure
core).

- **`boundary` is retired: 3 sites → 0.** It was `json.escape`,
  `json.stringify` and `json.stringify_pretty`, and it meant "this pin
  cannot find the code-point boundary I need". §11 declared it temporary
  when it was added at sc05 and named F-0018 as its exit. It leaves for a
  different reason than the one predicted: `escape` did not need the
  code-point primitive at all, it needed a byte walk, and the four
  characters JSON escapes are ASCII. **This is the first tag this library
  has ever removed**, and the cost was eight row annotations plus one test
  file. An interim tag that outlives its condition is worse than no tag,
  because by then callers have written handlers for it.
- **`syntax` arrives: 2 sites** (`json.parse`, `json.unescape`). "This text
  is not a JSON document" — one actionable failure mode, payload-free by
  §12's casing rule, and it will GAIN a byte offset as a payload rather
  than a sibling tag when §12's payload conventions reach this module.
  Spelled `syntax` and not the `parse` the sc05 contract named, because
  `std.json` now declares `pub fn parse` and a tag that shares a name with
  anything in scope resolves to that thing, silently (F-0036, re-measured
  live under lupin at this pin). **The rule that generalizes: check a new
  `pub fn` name against your module's TAGS, not only a new tag against the
  module's functions.**
- **`overflow` gains a site** (`json.parse`, 5 → 6 including the nursery's
  `x.json.int_at`): a JSON number whose magnitude has no `f64`. It is
  deliberately not `syntax` — the document is well-formed and saying
  otherwise would be a lie about the caller's data — and deliberately not
  silent infinity, which `stringify` would render back as `null`.
- **`parse` and `utf8` each gain a site** (`hex.decode_str`, whose row is
  `{parse, utf8}`): "that was not hexadecimal" and "those bytes are not
  text" are different answers about the same input, and coarsening them
  would tell a caller its hex was bad when it was not. The `utf8` half is
  `str_from_utf8`'s row arriving verbatim through `std.bytes.to_str`
  (§14's rule for a pure tier), which is why the test that matters rides
  it out of `main` rather than asserting it in a handler.



## sc12 (02-os): three marks from the fs surface, and a row that left

The s90 fs wave adds three marks to §12's inventory, all payload-free by
that section's casing rule, all adopted from the toolchain verbatim (§14's
rule for the fifth time — fs, io, net, process, and now the widened fs):

| tag | sites | payload | verdict |
|---|---|---|---|
| `exists` | 1 (`fs.create_dir`) | none | conforming — "it was already here" is a different outcome from "I made it", and a caller that wanted idempotence has `create_dir_all` beside it. It is the first std tag whose NAME collides with a `pub fn` in the module that raises it (`fs.exists`), which F-0036 says can make a tag ride out as the function with no `else` firing; measured not to here, on both compiler rungs, and held as `tests/fs/exists_row.lu` |
| `invalid` | 2 (`fs.write_bytes`, `fs.write_chunk`) | none | conforming — a `List[int]` element outside 0..255, which the interim byte currency (§11) makes spellable. Reused from `std.env`'s inventory rather than given a synonym |
| `cross_device` | 0 raised, 1 handled (`fs.move_file`) | none | conforming, and the interesting shape: it is declared by `fs_rename` precisely so a std wrapper can FALL BACK on it, so it appears in no public signature here. A tag std handles is not a tag std hides — the doc names it, and the module header says its trigger has no portable litmus |

**A row LEFT three signatures, which is the second time std has removed one**
(after `boundary` at sc14). `append_text`, `copy_file` and `move_file` all
carried `utf8` because all three DECODED: the append read the file back, and
the copy and the move were `read_text` + `write_text`. None of them decodes
now, so none of them can raise it, and §14's rule cuts both ways — a
function's row is the union of what its delegates raise minus what it
handles, and a tag that survived the delegate that raised it is a lie a
caller writes a handler for.

**One tag is DELIBERATELY not in a signature it could have been in.**
`fs_write_bytes` declares `invalid`; `copy_file` calls it and does not
declare it, because the bytes came out of `fs_read_bytes` one line earlier
and are bytes by construction. The handler answers `io` on that unreachable
arm rather than widening the signature. The same tag IS on `write_bytes` and
`write_chunk`, where the list is the caller's. Whose data it is decides
whether a tag is reachable, and the two functions sit four lines apart to
make the difference readable.

## sc17 (06-crypto): one mark from the AEAD surface

`std.x.crypto.chacha20` ships exactly one row, on exactly one function,
and the discipline around it is the module's whole error story:

| tag | sites | payload | verdict |
|---|---|---|---|
| `tag` | 1 (`chacha20.open`) | none | conforming — a forged, modified or truncated box is the DATA's failure (§2: raise where the data broke, trap where the caller did), and it is ONE mark for every authentication failure on purpose: a caller cannot act differently on "too short" versus "wrong tag", and answering them apart would hand a forger an oracle (§12's one-tag-per-actionable-failure rule read in the direction where merging is the security property). No payload, ever: anything a payload could carry about WHY the tag failed is information the construction exists to withhold. The name was checked both ways per sc14's symmetric rule — no module, function, binding or prelude name collides. Witnessed by `flipped_tag_row.lu` and `truncated_box_row.lu` riding `error: tag` out of `main`, and 60 Wycheproof `ModifiedTag` vectors asserting the reject path |

Everything else in the module is TOTAL or traps `assert` on a caller
contract violation (key/nonce/counter shape, non-byte elements, the
§2.4 counter-span guard) — the sc16 misuse spelling, held by four
`_trap.lu` files that run on ALL THREE lanes because every guard fires
before the first keystream operation. `seal` deliberately has NO row:
sealing cannot fail on data, only on the caller's own inputs.

## sc32 (15-the-budget): one mark for a failure std does not detect

`std.mem.budget` ships exactly one row, and it is the first row in this
library whose failure the LIBRARY never observes: the region-budget
breach is a `trap(alloc-contract)` fired by the runtime at the
allocating site, contained at a proc boundary, and read at the join as
a value (`[mem.region.cap.1/.3]`, `[conc.proc.exit]`, wolf-lang D68).
std's contribution is to turn that join into an `else` arm.

| tag | sites | payload | verdict |
|---|---|---|---|
| `exhausted` | 1 (`budget.with_cap`) | never — and the reason is a contract, not a style | conforming. One mark for one actionable mode: the work did not fit its budget, and the only thing a caller does about it is stop admitting work (lobo's per-request 503 is the motivating consumer). There is no second tag to want — every other outcome of the join is either normal or a caller's bug that traps before the spawn. The name was checked both ways per sc14's symmetric rule: no module, function, binding or prelude name in std collides with `exhausted` |

**Why there is no payload, stated because a reader will reach for
one.** "How much did it want?" is the obvious field, and it is
**unobservable by contract**: `[mem.region.cap.3]` makes teardown
free-then-deliver, so the breaching proc's regions are reclaimed
wholesale *before* the exit reason reaches the join, and no postmortem
query can see the dead proc's charge. `live()` at the join is already
back to its pre-call reading — that is the half of the contract that
makes the row safe to act on, and it is also exactly why there is no
number left to carry. A payload here would have to be invented, which
§12's "a payload is DATA, never a rendered string" forbids in the
direction that matters. The budget the caller passed is the caller's
own value and needs no return trip.

**And one failure deliberately kept OUT of the row vocabulary.** A
negative budget is `trap(assert)` at the door, not `exhausted`.
`[mem.region.cap.2]` already rules it a `trap(alloc-contract)` at the
creating site — but `with_cap` creates its region inside the proc, so
that trap would be CONTAINED and would reach the caller as a
recoverable row, answering a caller's arithmetic mistake with a value
they can handle. §2's trap rule wins: the check runs before anything
spawns, and `tests/mem/budget/negative_cap_trap.lu` holds it. `cap: 0`
is not in this family — the clause makes it a legal budget every charge
breaches, so it answers the row, and `breach_is_a_row.lu` pins that
half beside it.
