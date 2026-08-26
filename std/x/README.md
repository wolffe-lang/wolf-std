# std.x — the nursery (D31)

**The banner, which every resident's module header also carries.** A module
under `x` is a RESIDENT, not a member: `std.x.*` is not `std.*`. Its path,
its names, its rows and its behaviour may change or disappear in a single
sprint. No other std module may depend on one; a resident MAY depend on the
facade. This file is the register — every resident is listed below with the
measured reason it is not in the facade and the named trigger that graduates
it.

**The graduation clock.** At every campaign closeout each resident is
reviewed and exactly one of three things happens, recorded in that closeout:

- it **GRADUATES** — it moves under `std.`, and because the path *is* the
  API, the move is the whole release note;
- it is **DELETED** — the reviewed contract in its header is what survives;
- it is **EXTENDED** by one campaign with a written reason.

Nothing lives in `x` unexamined, and "still useful" is not a reason. A named
trigger is. The normative statement of all of this is API-CONVENTIONS §14's
sc10 amendment; this file is where it is applied.

**A resident is not an experiment.** Every one of them is a complete, tested
body kept out of the facade because **one refused or rejected body costs
every importer of its module a lane**. Budgeting a module by its worst body
is house doctrine (sc01), and this directory is where the bill goes instead.

**Two rules writing sc10's resident added.**

1. **Check a resident's dependencies against its own LANES before writing a
   function.** A resident often has one executing lane. A delegate that is
   refused on exactly that lane leaves a function with no runnable test and
   no fenceable doc example — a claim rather than code.
   `std.x.json.float_at` was written and withdrawn inside one sprint for
   that (F-0061). A refusal that costs one lane is a ledger row; a refusal
   that costs a module's only lane is a withdrawn function.
2. **A resident and its facade successor CAN be imported by one program,
   as of the sc15 pin.** For five sprints module identity was the last
   path segment (F-0034) and the pair was `E0306` on both compiler rungs
   (F-0058); s108 probe-closed wolf-lang#29 — identity is the FULL path,
   the leaf clash is only a binding clash, and `use std.x.json as xj`
   names the second binding. Re-measured at 1b149ba: the pair resolves on
   all three lanes, and what still separates `std.x.json` from `std.json`
   is their LANES (the DOM's generic bodies refuse on the compiler rungs,
   the kernels refuse under lupin), so their tests remain single-module
   for lane reasons and both headers say which. The constraint on D31's
   graduation story is retired; the division-by-direction rule survives
   as documentation hygiene while the two surfaces coexist.

## Residents

| module | fns | why it is here | graduates when |
|---|---|---|---|
| `list_eq` | 6 | `std.list`'s element-comparing family needs `cmp.Eq`; importing `std.cmp` used to flip every std.list wolfc row to `unsupported` (F-0012), and trait dispatch executes nowhere (F-0002 / F-0004) | dispatch runs — wolf-lang#5 / #12 |
| `deque_int` | 11 | the monomorphic proof that `std.deque`'s contract is implementable; `struct X[T]` does not parse (F-0011) | wolf-lang#11 / s16 |
| `option_flatten` | 1 | `flatten`'s nested row (`T ! {none} ! {none}`) is `fail(E0201)` at parse in wolfc (F-0039) | wolf-lang#34 |
| `option_expect` | 1 | `expect`'s diverging handler needs a bottom type; wolfc says `E0401: this is (), but the else fallback must produce T` (F-0040) | wolf-lang#35 |
| `testing_text` | 1 | a builtin `str` method used to make every importer `unsupported` at resolve, which would take `std.testing`'s whole floor off the compiler lane (F-0018) | wolf-lang#17 — **re-measure at the next closeout: the `str` ceiling is gone, so this one may simply graduate** |
| `crypto/sha2` | 25 | **the D53 crypto ladder's first rung** (campaign stdc-crypto, entered at sc16): SHA-256/384/512 from FIPS 180-4, HMAC (FIPS 198-1) and HKDF (RFC 5869), gated on the vendored public corpora (NIST CAVP byte-oriented, RFC 4231, RFC 5869, Wycheproof v1 — the per-family counts are in the cavp_*/rfc*/wycheproof_* test headers and the ledger section). In the nursery because D53's ladder is OPEN (`std.crypto`'s final shape — module split, generic-over-digest HMAC, the c28 ct tier — is unruled) and because its lane shape is uneven by mechanism: the checked tier refuses `wrapping[T]` shifts/bitwise at mem (F-0091), so every core is lupin+native. HMAC and HKDF live inside the module because a resident may not depend on a resident; the graduation move splits them. | c28's ct tier exists to verify the recorded `ct:` obligations, or the AEAD rung forces `std.crypto`'s module shape — whichever ruling arrives first |
| `json` | 22 | **the nursery's first tenant, and the only one D31 named in advance.** Two measured reasons, both in its header, ONE of which moved at sc15: (1) unequal lanes — the `json_*` kernels were the checked tier's alone from s40 until the sc15 pin, where s107's native crossing lit the second compiler rung; lupin still declines by design while is18 lands its own reading; (2) a dotted path is not the API json ends up with — no key enumeration (the sc15 NAMED STOP, F-0087), keys containing `.` unreachable, every read a fresh parse. sc15 added the DOM half over the same kernels: `Node` plus `root`/`node`/`member`/`element` and seven reads — the compiled lanes' navigable json, beside `std.json`'s interpreter-lane `Value` | the native mirror of `wolf_mem::json` (**FIRED at the sc15 pin — s107**) plus the interpreter's own (in flight, is18) — AND a ruling on whether a query face belongs beside `std.json`'s DOM or is deleted in favour of it |

## Clocks

- **The five stdc01 residents** (`list_eq`, `deque_int`, `option_flatten`,
  `option_expect`, `testing_text`) were each extended by one campaign at the
  sc06 closeout, on the ruling that each is blocked on a filed upstream issue
  rather than an open design question and each has a named trigger. **Their
  clock expires at the stdc02 closeout**, which is this campaign's: a
  resident still here after it is deleted, and the reviewed contract in its
  header is what survives.
- **`json` entered at sc10**, mid-campaign. Its first closeout is stdc02's,
  and the honest reading is that it will be EXTENDED there rather than
  judged: its trigger is an upstream mirror nobody has scheduled, and one
  campaign is the review interval rather than the deadline. Whatever happens,
  it is written in that closeout with a reason — which is the whole mechanism.
- **`json`'s OTHER trigger clause fired at sc15**: s107 crossed the
  `json_*` kernels natively (`wolf_rt::json`, the hand mirror of
  `wolf_mem::json` the trigger named, parity-pinned upstream), so both
  compiler rungs execute the module and only the interpreter's own reading
  (is18, concurrent) is outstanding. sc15 also grew the DOM half — `Node`
  and its navigations, over the same kernels — which RESHAPES the deletion
  question the sc14 bullet below sharpened: `std.json`'s DOM executes on
  the interpreter alone (F-0029) and this resident's on the compilers
  alone, so today each read surface is the only one its lanes have.
  Deleting the resident in favour of the DOM now costs the compiled lanes
  every json read; the honest closeout question is no longer "query face
  or DOM" but "which surface graduates when the lanes converge". Written
  here so the closeout finds it measured rather than remembered.
- **HALF of `json`'s trigger fired at sc14**, and the closeout should read it
  as evidence rather than as a verdict. The trigger has two clauses: an
  upstream mirror of `wolf_mem::json` (unmoved — the kernels are still the
  checked tier's alone), and *a ruling on whether a query face belongs beside
  `std.json`'s DOM or is deleted in favour of it*. `std.json` is now a
  complete DOM in both directions — `parse` and `stringify`, `get`/`at`,
  `escape`/`unescape` — so the second clause is answerable for the first
  time, and the question it turns into is sharp: this resident's 11 functions
  re-parse the document on every read (its own header calls that the
  second-loudest argument for the DOM), which is exactly what the DOM now
  makes unnecessary. What it still has that the DOM has not is a dotted path
  and a one-call read. That is a convenience question, not a capability one,
  and a convenience that costs a quadratic read is the kind D31's nursery
  exists to delete rather than to graduate.
