# The error taxonomy — the sc06 audit

Every error row shipped in sc01–sc05, audited against the rules
API-CONVENTIONS §12 now states, with the verdict per tag and the changes
this sprint made. The measurements are mechanical (a script over
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
| `gone` | 0 in std (the language's own `weak.upgrade`, `[mem.shared.rc.3]`) | none | conforming, reserved |
| `eof`, `utf8` | 0 — reserved for the io tier and `bytes.to_str` | none | reserved |

## What this sprint changed

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
