# wolf-std

The wolf standard library: the modules behind `use std.*`.

Written in wolf, tested from day one under two independent
implementations — lupin (the reference interpreter) executes every
module's tests now; the wolf compiler joins as it reaches native
execution. Library code lands here; the compiler-side half (prelude
wiring, intrinsics, `wolf build` integration) lives in wolf-lang.

Phase A covers the pure-computational core: collections, strings,
math, sorting, formatting, encodings, error and testing conventions.
OS-facing modules (io, fs, net, time, process) are scoped but wait on
the compiled runtime.

## The rig (sc00)

The tree is the namespace: `use std.fmt` names `std/fmt/` (D32). Tests
are directive-headed `.lu` entry files under `tests/`, one directory per
module, staged beside the `std/` tree and observed under both
implementations through the spec/06 record protocol; `tests/ledger.toml`
records what each implementation achieves per test, and passing deeper
than the ledger claims fails CI.

Imports are the real spelling — `use std.list` — from sc02 on: the
compiler resolves them against the staged tree with `--std-root` (s26).
lupin has no std root yet, so staging also mirrors each module directory
flat under its last segment; that mirror is the last interim, documented
in `xtask/src/stage.rs` and tracked as F-0010.

```sh
cargo xtask std-test      # stage + run every test under lupin and wolf
cargo xtask doctor        # which binaries resolved; do they match the pins
cargo xtask sync-pin      # vendor snapshot == upstream submodule at PIN
cargo xtask ledger-check  # tests and ledger are 1:1
cargo xtask ci            # all of the above behind fmt/clippy/test
```

Binaries are acquired, never vendored: `$LUPIN_BIN`/`$WOLF_BIN` →
`.wolf-bin/` → `PATH`; an absent binary turns its lane into a loud
`SKIP: no lupin at pin …`, never a silent pass. Pins: `upstream/` is the
wolf-lang submodule (sparse `spec/` + `corpus/`), `vendor/upstream/` the
CI-visible snapshot, `vendor/tools.toml` the binary pins. Surface
conventions live in `API-CONVENTIONS.md`; language gaps become filed
findings (`docs/findings.md`), never local inventions.

Sprint plan: the `std` track (`scNN`) in the wolf metarepo.
Dual-licensed MIT or Apache-2.0.
