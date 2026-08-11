# Contributing to wolf-std

## Orientation

What lives where: `std/` is the library — wolf source, and the directory
tree *is* the `use std.*` namespace (D32: moving a file is a facade
decision, not a refactor). `tests/` holds directive-headed `.lu` entry
files, one directory per module, plus `ledger.toml` (per-test,
per-implementation expectations). `xtask/` is the rig — plain Rust, no
build scripts ever (D33). `upstream/` (submodule, sparse `spec/` +
`corpus/`) and `vendor/upstream/` (tracked PIN + anchors snapshot) are
the pinned wolf-lang data. `.wolf-bin/` (gitignored) caches the two
implementation binaries. `API-CONVENTIONS.md` is the binding surface
document — read it before writing any `pub` item.

The gates, all green before any push:

```sh
cargo xtask ci     # fmt --check, clippy -D warnings, cargo test,
                   # sync-pin, doctor, ledger-check, std-test
```

## The coupling doctrine

This repository consumes the compiler and the interpreter as **binaries
and pinned data only** (the ls00 honesty):

- Never a source dependency on any `wolf_*` crate or on wolf-interp.
  Never a git dependency on either repo. The rig invokes `lupin` and
  `wolf` through `[proto.invoke]` and reads their spec/06 records — the
  record is the whole interface; front-door exit codes are never parsed.
- Binary resolution: `$LUPIN_BIN` / `$WOLF_BIN` → `.wolf-bin/` → `PATH`.
  Absence is legal and LOUD (`SKIP: no lupin at pin …`); a binary that
  contradicts `vendor/tools.toml` fails `cargo xtask doctor`.
- **No invented surface, ever.** A gap in what the language can express
  is a finding filed to wolf-lang (see `docs/findings.md`), not a
  workaround here. sc00's F-0001 (the std search path) is the founding
  example.

## The ledger ritual

`tests/ledger.toml` records what each implementation achieves on each
test *today* — `lupin = run | unsupported`, `wolfc = run | unsupported |
fail(E…)`, and, where a pin answers the SAME program two ways,
`unstable(run|unsupported)`. That last spelling exists because sc07's pin
needed it (F-0048) and is used by NOTHING at the sc08 pin: the finding closed,
the two rows that carried it were re-measured deterministic (14 consecutive
runs each) and narrowed back to `run`. It stays available and stays shocking.
The rig fails CI when reality is deeper OR shallower than the ledger. Advancing an entry (an upstream pin bump taught a tool a new
trick) is deliberate: its own commit, saying which upstream change
earned it.

## The os tier: working directories and honest lanes (sc07, re-measured sc08)

Two postures the rig carries so that no test has to restate them.

**Every lane runs in the staged package root.** `stage::Staged::root` is
the per-test scratch directory the runner and the doc-example extractor
`current_dir` into. A filesystem test therefore writes RELATIVE,
forward-slashed paths (`"round-trip.tmp"`) and gets: isolation from other
tests, a directory staging wipes before every run, nothing written into
the source tree, and no wolf literal carrying a host temp path — where a
backslash would be a string escape and `C:\Users` would not lex. That
last point is the s38 lesson, paid for upstream seven times in one file
and now structural here.

**A capability module's lanes are unequal, and that is the ledger's
truth.** `std.fs`, `std.io` and `std.net` reach the host, and an
implementation may honestly not go there. The posture is recorded here and
once per tier in `tests/ledger.toml`, never per file — and it is re-measured
at every pin, because a capability lane is exactly the kind of claim that
rots:

- **fs and io, at the sc08 pin: two executing lanes.** The checked tier
  performs REAL host operations and the native rung lowers both tiers as of
  #40 (six of seven `fs` tests and both `io` tests run natively, where every
  one of them was `unsupported` in sc07). lupin has no `fs_*` builtins and no
  injectable stdin by design, so it refuses those at RESOLVE — but it resolves
  module bodies LAZILY, so `fs/path_helpers.lu` runs there, and lupin 0.1.5's
  `eprint`/`eprint_raw` moved `io/writers_and_streams.lu` to `run` too.
- **net, at the sc08 pin: one executing lane.** The checked tier opens real
  sockets; lupin has no `net_*` builtins by design; the native rung refuses
  the tier by name ("net builtins in native lowering (checked lane only at
  s39)"). The pure address helpers run on lupin for the lazy-resolution reason
  above, which is why `net/address_helpers.lu` is the module's one two-lane
  row.
- **The comptime sandbox is the one place any of it is refused** (D33), and
  that refusal is held as a rejection test per tier
  (`tests/net/comptime_refuses.lu`).
- **Every net test is loopback and port 0**, learns its port through
  `net.port`, and dials before it accepts. No fixed ports, no foreign hosts,
  no name resolution, no external network EVER — and no timeout to rescue a
  blocked accept at this pin (F-0049).

For doc examples the same honesty is a `CAPABILITY_MODULES` list in the
extractor (`fs`, `io`, `net`): an `unsupported` verdict is acceptable on any
lane for those modules, and in exchange at least one lane must reach
`exit(0)` — an example nobody ran is not documentation.

## The pin ritual

- Data pin bump: `git -C upstream fetch && git -C upstream checkout
  <sha>`, write the sha to `vendor/upstream/PIN`, re-vendor
  `cp upstream/spec/anchors.json vendor/upstream/anchors.json`, run
  `cargo xtask sync-pin`, land as its own commit with the ledger deltas
  the new pin causes.
- Binary pin bump: update `vendor/tools.toml`, re-acquire the binaries,
  run `cargo xtask doctor` + `std-test`. The two binaries may sit at
  different wolf-lang pins; nightly watches the drift and never gates.
- NEVER edit `vendor/upstream/` snapshots by hand — `sync-pin` treats a
  hand edit as corruption.

## Commits

Chunked (one logical change), terse imperative, <250 chars, tests in the
same commit as the code. No coauthor lines, no generated-with trailers,
never `git add -A`.
