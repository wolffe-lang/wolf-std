# wolf-std

<img src="assets/wolf-logo.svg" alt="the wolf mark" width="120" align="right"/>

The wolf standard library: the modules behind `use std.*`.

It is written in wolf and tested under two independent implementations at
three execution rungs. The rungs are lupin (the reference interpreter), the
compiler's checked tier (`conform-run --checked`), and since sc04 the
compiler's native rung (`conform-run --native`, which compiles, links and
runs). The three refuse different shapes, so each test records what each one
achieved. Library code lands here. The compiler-side half (prelude wiring,
intrinsics, `wolf build` integration) lives in wolf-lang.

Phase A covers the pure-computational core: collections, strings, math,
sorting, formatting, encodings, and the error and testing conventions.
Phase B is the os tier. `std.fs`, `std.io` (sc07), `std.net` (sc08),
`std.time`, `std.env` (sc10) and `std.process` (sc11) are written. Each
function names the capability it reaches, per I13, and each says which of
the three rungs executes it.

## The rig (sc00)

The tree is the namespace: `use std.fmt` names `std/fmt/` (D32). Tests are
directive-headed `.lu` entry files under `tests/`, one directory per module,
staged beside the `std/` tree and observed on all three lanes through the
spec/06 record protocol. `tests/ledger.toml` records what each lane achieves
per test, and passing deeper than the ledger claims fails CI.

Imports use the real spelling, `use std.list`, and every lane resolves them
against one staged tree with `--std-root` (s26 compiler-side, wolf-interp#6
interpreter-side). The flat mirror sc02 needed is retired.

```sh
cargo xtask std-test      # stage + run every test on all three lanes
cargo xtask doc-examples  # every fenced ```wolf-doc-example, executed
cargo xtask ulp           # std.math.float's accuracy + bit-for-bit agreement
cargo xtask doctor        # which binaries resolved; do they match the pins
cargo xtask sync-pin      # vendor snapshot == upstream submodule at PIN
cargo xtask ledger-check  # tests and ledger are 1:1
cargo xtask std-test --lint-conventions   # the §13 test-authoring rules
cargo xtask ci            # all of the above behind fmt/clippy/test
```

Binaries are acquired, never vendored: `$LUPIN_BIN`/`$WOLF_BIN`, then
`.wolf-bin/`, then `PATH`. A missing binary turns its lane into a loud
`SKIP: no lupin at pin …` and never into a silent pass. The native rung also
wants `libwolf_rt.a` beside the `wolf` binary, or `$WOLF_RT_LIB`, and goes
dark just as loudly without it.

The pins: `upstream/` is the wolf-lang submodule (sparse `spec/` and
`corpus/`), `vendor/upstream/` is the CI-visible snapshot, and
`vendor/tools.toml` holds the binary pins. Surface conventions live in
`API-CONVENTIONS.md`. A gap in the language becomes a filed finding in
`docs/findings.md` and never a local invention.

Phase A closed with sc06: 253 public functions across 26 facade modules and
5 nursery residents, 127 ledgered tests, 211 executed doc examples, and 98
named-and-unbuilt functions whose blockers are filed upstream and counted by
blocker in `docs/phase-a-census.md`.

Sprint plan: the `std` track (`scNN`) in the wolf metarepo.
Licensed under [GPL-3.0-or-later](LICENSE) with the
[wolf Runtime Library Exception](LICENSE-EXCEPTION): the standard library compiles into
your programs, and your programs are yours.
