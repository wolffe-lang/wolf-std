# std.x — the nursery (D31)

Experimental modules land here as `std.x.<name>` with a **graduation
clock**: at each campaign closeout every resident either graduates into
the facade (a reviewed move — the path is the API) or is deleted.
Nothing lives in `x` across two closeouts. No module may depend on an
`x` resident except through its own tests; a resident MAY depend on the
facade.

In practice the residents are not experiments: every one is a complete,
tested body kept out of the facade because **one refused or rejected body
costs every importer of its module a lane**. Budgeting a module by its
worst body is house doctrine (sc01), and this directory is where the bill
goes instead.

## Residents at the stdc01 closeout (sc06) — all five extended one campaign

| module | fns | why it is here | graduates when |
|---|---|---|---|
| `list_eq` | 6 | `std.list`'s element-comparing family needs `cmp.Eq`; importing `std.cmp` used to flip every std.list wolfc row to `unsupported` (F-0012), and trait dispatch executes nowhere (F-0002 / F-0004) | dispatch runs — wolf-lang#5 / #12 |
| `deque_int` | 11 | the monomorphic proof that `std.deque`'s contract is implementable; `struct X[T]` does not parse (F-0011) | wolf-lang#11 / s16 |
| `option_flatten` | 1 | `flatten`'s nested row (`T ! {none} ! {none}`) is `fail(E0201)` at parse in wolfc (F-0039) | wolf-lang#34 |
| `option_expect` | 1 | `expect`'s diverging handler needs a bottom type; wolfc says `E0401: this is (), but the else fallback must produce T` (F-0040) | wolf-lang#35 |
| `testing_text` | 1 | a builtin `str` method makes every importer `unsupported` at resolve, which would take `std.testing`'s whole floor off the compiler lane (F-0018) | wolf-lang#17 |

The ruling is recorded in the campaign closeout: each is blocked on a
filed upstream issue rather than an open design question, and each has a
named trigger, so all five get their one campaign. At the stdc02 closeout
D31 bites — a resident still here is deleted, and the reviewed contract
in its header is what survives.
