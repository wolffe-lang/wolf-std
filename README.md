# wolf-std

<img src="assets/wolf-logo.svg" alt="the wolf mark" width="120" align="right"/>

The wolf standard library — the modules behind `use std.*`.

It ships separately from the compiler and on its own cadence, so no
package of `wolf` contains it. Until one does, `use std.list` answers
from a small built-in stub and reports that module `std` has no item
named `list`. Point the compiler at a checkout and the real tree
answers:

```sh
git clone https://github.com/wolffe-lang/wolf-std
export WOLF_STD="$PWD/wolf-std/std"
wolf run main.lu
```

`--std-root <dir>` does the same for a single invocation and beats
`WOLF_STD`. The tree *is* the namespace: `use std.list` names
`<root>/list/`.

## What's in it

**Core** — `list` `map` `set` `deque` `option` `errors` `iter` `range`
`cmp` `sort` `search` `mem` `pool` `prelude`

**Text and data** — `str` `strbuf` `fmt` `unicode` `bytes` `hex`
`base64` `json`

**Numbers** — `math` `rand`

**The os tier** — `fs` `io` `net` `time` `env` `process` `os`

**Testing** — `testing`

`std/x/` is the nursery: `crypto`, `tls`, `jose` and others live there
until their surface settles. Treat anything under `x` as unstable by
definition.

Every function names the capability it reaches, and records which
execution rungs actually run it.

> **Pre-alpha, with the compiler.** The surface moves. The os tier is
> the newest part and the least settled.

## Tested under two implementations, at three rungs

Library code here is executed by **lupin** (the reference interpreter),
by the compiler's **checked** tier, and by the compiler's **native**
tier — which compiles, links and runs it.

The three refuse different shapes, so each test records what each rung
achieved rather than assuming they agree. `tests/ledger.toml` holds
those claims for all 392 tests, and **a test that passes deeper than
its ledger claims fails CI** — the ledger is a contract in both
directions, not a floor.

A gap in the language becomes a filed finding in `docs/findings.md`
and never a local workaround.

## Building and testing

```sh
cargo xtask std-test      # stage + run every test on all three rungs
cargo xtask doc-examples  # every fenced ```wolf-doc-example, executed
cargo xtask ulp           # std.math.float accuracy + bit-for-bit agreement
cargo xtask doctor        # which binaries resolved, and do they match the pins
cargo xtask ci            # all of it, behind fmt/clippy/test
```

Toolchain binaries are **acquired, never vendored** — `$LUPIN_BIN` /
`$WOLF_BIN`, then `.wolf-bin/`, then `PATH`. A missing binary turns its
rung into a loud `SKIP: no lupin at pin …`, never a silent pass. The
native rung also wants `libwolf_rt.a` beside the `wolf` binary (or
`$WOLF_RT_LIB`) and goes dark just as loudly without it.

`upstream/` is the wolf-lang submodule (sparse `spec/` and `corpus/`),
`vendor/upstream/` the CI-visible snapshot, and `vendor/tools.toml`
the binary pins. Surface conventions are in
[`API-CONVENTIONS.md`](API-CONVENTIONS.md).

## License

[GPL-3.0-or-later](LICENSE) with the
[wolf Runtime Library Exception](LICENSE-EXCEPTION): the standard
library compiles into your programs, and your programs are yours,
under any license you choose.
