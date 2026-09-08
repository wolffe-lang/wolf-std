# wolf-std

<img src="assets/wolf-logo.svg" alt="the wolf mark" width="120" align="right"/>

The wolf standard library: the modules behind `use std.*`.

It ships separately from the compiler and on its own cadence, so no
package of `wolf` contains it. Until one does, `use std.list` resolves
against a small built-in stub and reports that module `std` has no
item named `list`. Point the compiler at a checkout and the real tree
answers:

```sh
git clone https://github.com/wolffe-lang/wolf-std
export WOLF_STD="$PWD/wolf-std/std"
wolf run main.lu
```

`--std-root <dir>` does the same for a single invocation and overrides
`WOLF_STD`. The directory tree is the namespace: `use std.list` names
`<root>/list/`.

## What's in it

Core: `list` `map` `set` `deque` `option` `errors` `iter` `range` `cmp`
`sort` `search` `mem` `pool` `prelude`

Text and data: `str` `strbuf` `fmt` `unicode` `bytes` `hex` `base64`
`json`

Numbers: `math` `rand`

The os tier: `fs` `io` `net` `time` `env` `process` `os`

Testing: `testing`

`std/x/` is the nursery. `crypto`, `tls`, `jose` and a few others live
there until their surfaces settle, and anything under `x` may change
without notice.

Every function names the capability it reaches and records which
execution rungs run it.

The library is pre-alpha, with the compiler. The os tier is the newest
part and the least settled.

## Testing

Library code here is executed by lupin (the reference interpreter), by
the compiler's checked tier, and by the compiler's native tier, which
compiles, links and runs it.

The three refuse different shapes of program, so each test records
what each rung achieved. `tests/ledger.toml` holds those records for
all 392 tests. A test that passes deeper than its ledger entry claims
fails CI, the same as one that passes shallower, so the ledger has to
be kept accurate in both directions.

When a test finds a gap in the language, the gap is filed in
`docs/findings.md` and upstream. The library does not work around it.

## Building

```sh
cargo xtask std-test      # stage + run every test on all three rungs
cargo xtask doc-examples  # every fenced ```wolf-doc-example, executed
cargo xtask ulp           # std.math.float accuracy + bit-for-bit agreement
cargo xtask doctor        # which binaries resolved, and whether they match the pins
cargo xtask ci            # all of it, behind fmt/clippy/test
```

Toolchain binaries are looked up at run time from `$LUPIN_BIN` /
`$WOLF_BIN`, then `.wolf-bin/`, then `PATH`. A rung whose binary is
missing reports `SKIP: no lupin at pin …` and is counted as skipped.
The native rung also needs `libwolf_rt.a` beside the `wolf` binary (or
`$WOLF_RT_LIB`) and skips the same way without it.

`upstream/` is the wolf-lang submodule (sparse `spec/` and `corpus/`),
`vendor/upstream/` the snapshot CI reads, and `vendor/tools.toml` the
binary pins. Surface conventions are in
[`API-CONVENTIONS.md`](API-CONVENTIONS.md).

## License

[GPL-3.0-or-later](LICENSE) with the
[wolf Runtime Library Exception](LICENSE-EXCEPTION). The standard
library compiles into your programs, and your programs are yours,
under any license you choose.
