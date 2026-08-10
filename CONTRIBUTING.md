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
fail(E…)`. The rig fails CI when reality is deeper OR shallower than the
ledger. Advancing an entry (an upstream pin bump taught a tool a new
trick) is deliberate: its own commit, saying which upstream change
earned it.

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
