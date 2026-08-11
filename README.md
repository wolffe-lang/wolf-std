# wolf-std

The wolf standard library: the modules behind `use std.*`.

Written in wolf, tested from day one under two independent
implementations at three execution rungs — lupin (the reference
interpreter), the compiler's checked tier (`conform-run --checked`), and
since sc04 the compiler's NATIVE rung (`conform-run --native`: compile,
link, run). The three refuse different shapes, so each test records what
each achieved. Library code lands here; the compiler-side half (prelude
wiring, intrinsics, `wolf build` integration) lives in wolf-lang.

Phase A covers the pure-computational core: collections, strings,
math, sorting, formatting, encodings, error and testing conventions.
OS-facing modules (io, fs, net, time, process) are scoped but wait on
the compiled runtime.

## The rig (sc00)

The tree is the namespace: `use std.fmt` names `std/fmt/` (D32). Tests
are directive-headed `.lu` entry files under `tests/`, one directory per
module, staged beside the `std/` tree and observed on all three lanes
through the spec/06 record protocol; `tests/ledger.toml` records what
each lane achieves per test, and passing deeper than the ledger claims
fails CI.

Imports are the real spelling — `use std.list` — and every lane resolves
them against one staged tree with `--std-root` (s26 compiler-side,
wolf-interp#6 interpreter-side). The flat mirror sc02 needed is retired.

```sh
cargo xtask std-test      # stage + run every test on all three lanes
cargo xtask doc-examples  # every fenced ```wolf-doc-example, executed
cargo xtask ulp           # std.math.float's accuracy + bit-for-bit agreement
cargo xtask doctor        # which binaries resolved; do they match the pins
cargo xtask sync-pin      # vendor snapshot == upstream submodule at PIN
cargo xtask ledger-check  # tests and ledger are 1:1
cargo xtask ci            # all of the above behind fmt/clippy/test
```

Binaries are acquired, never vendored: `$LUPIN_BIN`/`$WOLF_BIN` →
`.wolf-bin/` → `PATH`; an absent binary turns its lane into a loud
`SKIP: no lupin at pin …`, never a silent pass. The native rung
additionally wants `libwolf_rt.a` beside the `wolf` binary (or
`$WOLF_RT_LIB`) and goes dark just as loudly without it. Pins: `upstream/` is the
wolf-lang submodule (sparse `spec/` + `corpus/`), `vendor/upstream/` the
CI-visible snapshot, `vendor/tools.toml` the binary pins. Surface
conventions live in `API-CONVENTIONS.md`; language gaps become filed
findings (`docs/findings.md`), never local inventions.

Sprint plan: the `std` track (`scNN`) in the wolf metarepo.
Dual-licensed MIT or Apache-2.0.
