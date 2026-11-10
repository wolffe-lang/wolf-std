# s292 prediction: pseudo-terminals, in `std.term`

Written before any change under `std/` or `tests/`. Base: trunk `2411d86`. Toolchain unchanged (the
pins in `vendor/tools.toml`). The calls this surface wraps exist in no wolf release. They are
proposed in wolf-lang's `notes/s292-prediction.md` on its branch `s292` (base `878490c4`, landing
after the new-session change, wolf-lang#1089), where the shape is argued and measured; this file is
the std half, and nothing here lands before a wolf release carries the calls.

## P1. The surface

| function | wraps | rows | capability |
|---|---|---|---|
| `open_pty() -> Pty` | `os_term_open_pty()`, then `os_term_pty_name(master)` | `{io, unsupported}` | `Exec` |
| `pty_name(master: fs.File) -> str` | `os_term_pty_name` | `{io, unsupported}` | `io` (uncharged) |
| `set_size(tty: fs.File, s: Size) -> ()` | `os_term_set_size(tty.fd, s.rows, s.columns)` | `{invalid, io, unsupported}` | `io` (uncharged) |
| `attr(tty: fs.File, op: int) -> int` | `os_term_attr` | `{invalid, io, unsupported}` | `io` (uncharged) |
| `set_attr(tty: fs.File, op: int, value: int) -> ()` | `os_term_set_attr` | `{invalid, io, unsupported}` | `io` (uncharged) |

`Pty { master: fs.File, name: str }`: the master, granted and unlocked, close-on-exec, and its
slave's name, which the program opens with `fs.open_mode(name, 3)`; a process that leads a session
with no terminal (after `process.lead_session`, s290) takes the slave as its controlling terminal by
that open. If `open_pty` gets the master and the name call fails, it closes the master before it
answers the row.

`op` is RFC 4254 section 8's opcode (and RFC 8160's 42, IUTF8). Pure, comptime-safe helpers, so a
program never writes the numbers by hand:

- `attr_ops() -> List[int]`: the vocabulary, in opcode order (1..18, 30..42, 50..62, 70..75,
  90..93, 128, 129).
- `attr_name(op: int) -> str`: the RFC's mnemonic (`"VINTR"`, `"ICANON"`, `"TTY_OP_OSPEED"`), `""`
  for an op outside the vocabulary.
- `attr_from_wire(op: int, value: int) -> int`: the value an SSH client sent, as `set_attr` takes
  it: for a character op RFC 4254's 255 ("none") becomes `char_disabled()` (-1); every other value
  is answered as it came.

Each `term` doc states the per-host table (which ops linux, macOS and PAX lack), which end of a pty
takes the size and the attributes on each host (as the wolf-lang probe measures it), and that the
host signals the foreground group with `signal.resize()` when the size changes.

## P2. The rows

Committed red before the module changes, each `fail(E0301)` on the compiler lanes until the
builtins exist:

- `tests/term/pty_rows.lu`, every host, no terminal reached: `set_size` with -1 rows and with
  65536 columns is `invalid`; `attr` of op 0, 19 and 160 is `invalid`; `set_attr` of a character
  to 256 and of a flag to 2 is `invalid`; a closed handle is `io`; windows answers `unsupported` for
  all five.
- `tests/term/attr_names.lu`, every host, three lanes: `attr_ops()` has 56 entries in order,
  `attr_name` answers the mnemonics, `attr_from_wire` maps 255 to -1 for characters only.
- `tests/term/pty_child.lu`, unix, native: `open_pty`; the name has the host's form; a forked
  child leads a session, opens the slave, and reports `/dev/tty` open and its group in the
  foreground; a size set on the master reads back in the child, which hears `signal.resize()`;
  VINTR set to 7 makes a 0x07 written to the master interrupt the child; every op round-trips on
  the slave or is `unsupported`, by the host's table; the name is gone after the master's close.
- `tests/term/comptime_refuses_pty.lu`: D33 for the five, held one level down at the builtins.

lupin at the paired release resolves the five names, answers the rows decided before a terminal is
reached (the shapes, windows, a host's lacking ops, a closed handle) and refuses the rest by name,
so `pty_rows.lu` and `attr_names.lu` are three-lane and the rest record `lupin = "unsupported"`.

## P3. What lands when

Nothing in `std/` or `tests/` changes until a wolf release carries the five calls and a lupin
release pairs with it. Then one change bumps the pins, and the next adds the functions, their rows,
the ledger entries and the module header's paragraph. Before that, the rows are measured against a
wolf built from wolf-lang's branch through `WOLF_BIN`, recorded, not landed.

## P4. Found while reading, not this lane's change

`std.term`'s `Mode` carries no `raw_lf` (the runtime's bit 24, `RAW_LF`, since wolf 0.2.37): `unpack`
drops it and `pack` never sets it, so a mode read and set back through std turns `ONLCR` on for a
terminal that had it off. Named here for the module's next change.

## P5. After wolf-lang's probe (added once its outputs were read)

wolf-lang's phase-1 probe (`notes/s292-ptys.md` and `notes/s292-evidence/` on its branch `s292`)
moves its Q1 from a master alone to a pair: macOS puts a pty's settings and size back to its
defaults when the slave is opened while no descriptor of it is open, and its master takes neither
before the slave's first open, so a holder that sets them before the program runs needs a slave of
its own, opened with `O_NOCTTY`, which only the runtime can do. If that is ruled, `open_pty` answers
`Pty { master: fs.File, slave: fs.File, name: str }` (both close-on-exec), and its doc gives the
order every host keeps: open the pair; `set_size` and `set_attr` on either end; fork (or start the
program with the slave in its descriptor map); the child keeps the slave it inherited until its own
`fs.open_mode(name, 3)`, after `process.lead_session()`, makes the slave its controlling terminal;
the parent closes its `slave` after the fork, or the master never reads the end. The rows in P2 gain
the measured per-host answers: linux keeps CS8 and parity off on a pty and macOS keeps PENDIN to
itself, each `unsupported` from `set_attr`.

burrow's `notes/bw06-wolf-needs.md` (its branch `bw06`) then stated the consumer's needs. Two more
changes follow from it, if ruled with wolf-lang's revised Q3: `set_size(tty: fs.File, s: Size,
xpixels: int, ypixels: int)`, since the size set takes RFC 4254's two pixel sizes too (the read stays
`Size`); and `set_attrs(tty: fs.File, pairs: List[int]) -> List[int] ! {invalid, io, unsupported}`,
over `set_attr` one pair at a time, answering the ops it could not apply (`unsupported`) and
stopping at the first `invalid` or `io`, which is burrow's `set_modes`. `attr_from_wire` also maps a
flag's non-zero wire value to 1, so a decoded pty request passes through it unchanged otherwise.
