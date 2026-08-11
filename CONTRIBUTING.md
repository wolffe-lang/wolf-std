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
`unstable(run|unsupported)` (sc07, F-0048 — accepted, printed loudly in
every `std-test` summary, and narrowed back to one value the day the
finding closes; two rows carry it and nothing else should). The rig fails
CI when reality is deeper OR shallower than the ledger. Advancing an entry (an upstream pin bump taught a tool a new
trick) is deliberate: its own commit, saying which upstream change
earned it.

## The os tier: working directories and honest lanes (sc07)

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
truth.** `std.fs` and `std.io` reach the host, and an implementation may
honestly not go there: at these pins lupin has no `fs_*` builtins, no
`read_line` and no `eprint`/`eprint_raw`, so its column for the fs tier is
`unsupported` at RESOLVE, and the native rung refuses the tier by name
("io/fs builtins in native lowering (checked lane only at s38)"). The
compiler's checked tier is the executing lane — it performs REAL host
operations; the comptime sandbox is the one place they are refused (D33).
This posture is recorded here and once in `tests/ledger.toml`'s sc07
section, never per file. For doc examples the same honesty is a
`CAPABILITY_MODULES` list in the extractor: an `unsupported` verdict is
acceptable on any lane for those modules, and in exchange at least one
lane must reach `exit(0)` — an example nobody ran is not documentation.

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
