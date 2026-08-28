# Contributing to wolf-std

## Orientation

What lives where. `std/` is the library: wolf source, and the directory
tree *is* the `use std.*` namespace (D32, which makes moving a file a
facade decision). `tests/` holds directive-headed `.lu` entry files, one
directory per module, plus `ledger.toml`, the per-test,
per-implementation expectations. `xtask/` is the rig, plain Rust, no
build scripts ever (D33). `upstream/` (submodule, sparse `spec/` +
`corpus/`) and `vendor/upstream/` (tracked PIN + anchors snapshot) hold
the pinned wolf-lang data. `.wolf-bin/` (gitignored) caches the two
implementation binaries. `API-CONVENTIONS.md` is the binding surface
document. Read it before writing any `pub` item.

The gates, all green before any push:

```sh
cargo xtask ci     # fmt --check, clippy -D warnings, cargo test, sync-pin,
                   # doctor, ledger-check, lint-conventions, std-test,
                   # doc-examples, ulp
```

## The coupling doctrine

This repository consumes the compiler and the interpreter as **binaries
and pinned data only** (the ls00 honesty):

- Never a source dependency on any `wolf_*` crate or on wolf-interp.
  Never a git dependency on either repo. The rig invokes `lupin` and
  `wolf` through `[proto.invoke]` and reads their spec/06 records. The
  record is the whole interface, and front-door exit codes are never
  parsed.
- Binary resolution: `$LUPIN_BIN` / `$WOLF_BIN` → `.wolf-bin/` → `PATH`.
  Absence is legal and LOUD (`SKIP: no lupin at pin …`). A binary that
  contradicts `vendor/tools.toml` fails `cargo xtask doctor`.
- **No invented surface, ever.** A gap in what the language can express
  is a finding filed to wolf-lang (see `docs/findings.md`), not a
  workaround here. sc00's F-0001 (the std search path) is the founding
  example.

## The ledger ritual

`tests/ledger.toml` records what each implementation achieves on each
test *today*. `lupin` is `run` or `unsupported` (plus two lupin-only
words below); `wolfc` and `native` add `fail(E…)`; and where a pin
answers the SAME program two ways there is `unstable(run|unsupported)`.
That last spelling exists because sc07's pin needed it (F-0048), and
nothing uses it at the sc08 pin. The finding closed, the two rows that
carried it were re-measured deterministic (14 consecutive runs each)
and narrowed back to `run`. It stays available and it stays shocking.

Two more lupin-only words, each a dated necessity rather than a
relaxation, each printed loudly by every `std-test` run. `slow` (sc16):
the lane's SEMANTICS reach the program — its fast siblings are `run` —
but a tree-walk of the full input blows the 60s ceiling; the lane is
skipped, named, and owed a re-measure at every pin bump.
`divergent(stdout)` / `divergent(exit(N))` / `divergent(trap(kind))`
(sc24): the lane EXECUTES the program and its honest observation cannot
satisfy the directive — the first lupin release carrying the
net/process tiers surfaced two such shapes, a filed wrong answer
(F-0097: a handler over a builtin-raised row takes its first arm, both
orders measured) and a filed static/dynamic split (F-0098: the
compilers reject take-mode reuse as E1001, the interpreter executes it
to its honest dynamic outcome). The runner demands EXACTLY the named
observation, so a heal reads as a red and the flip to `run` is
deliberate; each row's comment cites its finding.

The rig fails CI when reality is deeper OR shallower than the ledger.
Advancing an entry (an upstream pin bump taught a tool a new trick) is
deliberate: its own commit, saying which upstream change earned it.

## The os tier: working directories and honest lanes (sc07, re-measured sc11)

Two postures the rig carries so that no test has to restate them.

**Every lane runs in the staged package root.** `stage::Staged::root` is
the per-test scratch directory the runner and the doc-example extractor
`current_dir` into. A filesystem test therefore writes RELATIVE,
forward-slashed paths (`"round-trip.tmp"`). What it gets back: isolation
from other tests, a directory staging wipes before every run, nothing
written into the source tree, and no wolf literal carrying a host temp
path, where a backslash would be a string escape and `C:\Users` would
not lex. That last point is the s38 lesson, paid for upstream seven
times in one file and now structural here.

**A capability module's lanes are unequal, and that is the ledger's
truth.** `std.fs`, `std.io` and `std.net` reach the host, and an
implementation may honestly not go there. The posture is recorded here
and once per tier in `tests/ledger.toml`, never per file. It is
re-measured at every pin, because a capability lane is the kind of claim
that rots.

- **fs and io, at the sc12 (02-os) pin: two executing lanes, over a
  surface that TRIPLED.** The checked tier performs REAL host operations
  and the native rung lowers both tiers — all twenty-four `fs_*` builtins
  as of s90, in the same wave that added fifteen of them, so nothing
  `std.fs` gained this sprint has an unequal lane. Thirteen of the
  fourteen `fs` tests execute natively and so do all three `io` tests;
  the fourteenth `fs` row is a rejection witness that never reaches
  lowering. In sc07 the native rung refused both tiers by name. lupin has
  no `fs_*` builtins and no `read_line`, so it refuses those at RESOLVE.
  It resolves module bodies LAZILY, so `fs/path_helpers.lu` runs there,
  and lupin 0.1.5's `eprint` and `eprint_raw` moved
  `io/writers_and_streams.lu` to `run` too.

  One wrinkle worth recording where the posture is recorded: **lupin's
  refusal for the s90 names is not the same SENTENCE as for the s38
  ones.** The old nine get its reasoned decline ("this machine has no
  filesystem … by design, so the fs tier is declined rather than
  mocked"); the new fifteen get the generic ``unsupported: `fs_read_dir`
  does not resolve``. No lane moves either way, so no row records it —
  which is exactly why it is written down here and filed as F-0081: this
  repository's whole method for reading a dark column is telling a
  posture from drift, and the refusal text is where that is decided.
- **net, at the sc11 pin: one executing lane.** The checked tier opens
  real sockets. lupin has no `net_*` builtins by design. The native rung
  refuses the tier by name ("net builtins in native lowering (checked
  lane only at s39)"). The pure address helpers run on lupin for the
  lazy-resolution reason above, which is why `net/address_helpers.lu` is
  the module's one two-lane row.
- **The comptime sandbox is the one place any of it is refused** (D33),
  and that refusal is held as a rejection test per tier
  (`tests/net/comptime_refuses.lu`).
- **Every net test is loopback and port 0**, learns its port through
  `net.port`, and dials before it accepts. No fixed ports, no foreign
  hosts, no name resolution, no external network EVER, and no timeout to
  rescue a blocked accept at this pin (F-0049).
- **time and env, at the sc11 pin: three executing lanes.** lupin
  0.1.8's conformance pin carries the s40 `time_*` and `env_*` families,
  so what sc10 recorded as a two-wave DRIFT is closed, by a release,
  exactly as that note predicted. `x/json` is still one lane, and its
  dark columns changed KIND: the native rung refuses the tier by name,
  and lupin now DECLINES the surface by design ("rather than risk a
  second, guessed RFC 8259 reading"). That is a posture, like `fs` and
  `net`. It is not a wave to wait out.
- **process, at the sc12 (02-os) pin: one executing lane, and no test
  starts a program.** The checked tier spawns real children. lupin declines the
  trio by design ("this machine runs no child processes by design, so
  the tier is declined rather than mocked"), and the native rung refuses
  it by name. No `.lu` test here starts a real program, and the reason
  is portability: no program exists on every tier-1 host and the
  directive schema has no per-platform gate (F-0066). One of that
  finding's three legs came off at the sc12 (02-os) pin — s90's `os_exe`
  means a program CAN learn its own path — and the finding stays open,
  because a self-spawn would still run on the checked lane alone and
  still needs a way for the child to know it is the child. `std.process`
  says the same thing in its header. Every witness is deterministic on all of
  them: a name no host has, an empty argv, a forged handle. The
  exit-code path rests on the toolchain's own unix-gated tests, which
  the module header says out loud.

**Re-measured at the sc24 pin (lupin 0.1.14, the is14-is25 catch-up
release): three of the postures above CLOSED ON A RELEASE.** The
interpreter now opens real sockets (is18 — 4 `net` rows flip to `run`;
"no `net_*` builtins by design" is history), runs the process trio
(is18 — `wait` reaps, `kill` never tombstones; 2 rows flip), and reads
json on its own RFC 8259 reading (is18 — the "declines rather than risk
a second, guessed reading" posture withdrawn upstream; 10 `x/json` rows
flip). The paragraphs above are kept as the history they are, with this
note as their sunset: `net` and `x/json` are two-lane now (native still
refuses both tiers by name), `process` is checked+lupin. The first
release with those tiers also surfaced two lupin divergences the ledger
carries as `divergent(…)` rows (F-0097, F-0098 — see the ledger ritual
above), and F-0049's no-deadline caveat is unchanged: dial-then-accept,
loopback, port 0, forever.

For doc examples the same honesty is a `CAPABILITY_MODULES` list in the
extractor (`fs`, `io`, `net`, `time`, `env`, `process`, `x.json`). An
`unsupported` verdict is acceptable on any lane for those modules, and
in exchange at least one lane must reach `exit(0)`. An example nobody
ran is not documentation.

## The pin ritual

- Data pin bump: `git -C upstream fetch && git -C upstream checkout
  <sha>`, write the sha to `vendor/upstream/PIN`, re-vendor
  `cp upstream/spec/anchors.json vendor/upstream/anchors.json`, run
  `cargo xtask sync-pin`, land as its own commit with the ledger deltas
  the new pin causes.
- Binary pin bump: update `vendor/tools.toml`, re-acquire the binaries,
  run `cargo xtask doctor` + `std-test`. The two binaries may sit at
  different wolf-lang pins; nightly watches the drift and never gates.
- NEVER edit `vendor/upstream/` snapshots by hand. `sync-pin` treats a
  hand edit as corruption.

## Commits

Chunked (one logical change), terse imperative, <250 chars, tests in the
same commit as the code. Never `git add -A`.
