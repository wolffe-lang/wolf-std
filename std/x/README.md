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
2. **A resident and its facade successor cannot be imported by one
   program.** Module identity is the last path segment (F-0034), so
   `std.x.json` and `std.json` are both `json` to an importer and the pair is
   `E0306` on both compiler rungs, `unsupported` under lupin (F-0058). That
   is a constraint on D31's graduation story itself, filed; until it is
   ruled, a resident sharing a facade's leaf name divides the work with it
   strictly by direction and both headers say so.

## Residents

| module | fns | why it is here | graduates when |
|---|---|---|---|
| `list_eq` | 6 | `std.list`'s element-comparing family needs `cmp.Eq`; importing `std.cmp` used to flip every std.list wolfc row to `unsupported` (F-0012), and trait dispatch executes nowhere (F-0002 / F-0004) | dispatch runs — wolf-lang#5 / #12 |
| `deque_int` | 11 | the monomorphic proof that `std.deque`'s contract is implementable; `struct X[T]` does not parse (F-0011) | wolf-lang#11 / s16 |
| `option_flatten` | 1 | `flatten`'s nested row (`T ! {none} ! {none}`) is `fail(E0201)` at parse in wolfc (F-0039) | wolf-lang#34 |
| `option_expect` | 1 | `expect`'s diverging handler needs a bottom type; wolfc says `E0401: this is (), but the else fallback must produce T` (F-0040) | wolf-lang#35 |
| `testing_text` | 1 | a builtin `str` method used to make every importer `unsupported` at resolve, which would take `std.testing`'s whole floor off the compiler lane (F-0018) | wolf-lang#17 — **re-measure at the next closeout: the `str` ceiling is gone, so this one may simply graduate** |
| `json` | 11 | **the nursery's first tenant, and the only one D31 named in advance.** Two measured reasons, both in its header: (1) ONE executing lane — the `json_*` kernels are the checked tier's alone at s40, the native rung refusing them by name and lupin being three waves behind its pin; (2) a dotted path is not the API json ends up with — no key enumeration, keys containing `.` unreachable, every read a fresh parse (F-0058) | the native mirror of `wolf_mem::json` plus the interpreter's own — the two-consumer shape every builtin tier goes through — AND a ruling on whether a query face belongs beside `std.json`'s DOM or is deleted in favour of it |

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
