# wolf-std

<img src="assets/wolf-logo.svg" alt="the wolf mark" width="120" align="right"/>

The wolf standard library: the modules behind `use std.*`.

The library has its own release cadence. Each `wolf` release ships a
copy of `std/` beside the compiler binary, pinned by the compiler's
`crates/wolf_driver/STD-PIN`. To use a checkout instead:

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
`sort` `search` `mem` `pool` `ops` `prelude`

Text and data: `str` `strbuf` `fmt` `unicode` `bytes` `hex` `base64`
`bech32` `json`

Numbers: `math` `rand`

The os tier: `fs` `io` `net` `time` `env` `process` `os` `signal` `term` `poll`

Testing: `testing`

`std/x/` is the nursery. `crypto`, `tls`, `jose` and a few others live
there until their surfaces settle, and anything under `x` may change
without notice.

Every function names the capability it reaches and records which
execution rungs run it.

The library is pre-alpha, with the compiler. The newest additions are
Unicode 15.1.0 extended graphemes and additive display columns:
`unicode.display_width`, `str.display_width`, `str.truncate_display` and
`str.pad_display_left`/`pad_display_right`. They use C-compatible scalar
widths, not emoji shaping; byte lengths and `fmt` padding are unchanged.
See `docs/sc86-display-width.md` for the policy and exact exceptions.
Before them, strict unpadded standard base64, BIP-173 Bech32, and
`bytes.is_equal_fixed_work` (no early exit at source level, not a timing
certificate). `std.os.random` wraps the language's OS bytes with platform
admission: Linux/macOS use the documented OS contract; PAX refuses until
its strict capability adapter lands. Provider failure still traps; no
recoverable provider-error guarantee or fallback is claimed. See
`docs/dw03-std-for-age.txt` for the selected primitive audit and limits.
Before them, `std.poll`: an owned waiter over files and sockets that answers every
ready entry, never only the first, with a set of signals and a deadline,
for a server that waits in one place on pty masters, pipes, a listener
and its clients. At wolf 0.2.40 it uses the runtime's readiness sets:
files, sockets and signal meanings together, real hangup/error facts,
and at most 256 entries per waiter. `net.write_some` sends what fits
without parking, and `net.unix.peer_credentials` reports a Unix peer's
pid and effective ids. Host-dependent facts and the measured tier
postures are in `docs/sc83-poll-on-the-one-wait.txt`.
Before it, wolf 0.2.39's sessions and
pseudo-terminals: `std.process`'s
`start_session`, `lead_session` and `own_session` put a child, or this
process, in a session of its own, and `std.term`'s `open_pty`,
`pty_name`, `set_size`, `attr`, `set_attr` and `open_controlling` give a
program a terminal of its own, which an ssh server's terminal request
needs. Before them, wolf 0.2.36's: `std.env`'s `arg0` and `args_bytes`,
the name a program was invoked by and every argument as bytes.
`std.strbuf` builds over a growable byte list, so a long build is linear
in time and memory. Before them, wolf 0.2.35's: `std.fs`'s advisory locks, fifos, handle records,
non-blocking writes and reads into a caller's buffer; `std.process`'s
core limits; `std.os`'s `name` and `arch`. `std.list`'s mutators work in
place. `std.x.crypto.sha2` reuses its message schedules and block
buffers, so incremental SHA-256 and SHA-512 hold bounded working storage.
`std.x.crypto.curve25519.verify_strict` is `verify` plus the refusal of a
public key or commitment of small order. `std.map.get` uses the language's
index read, as `has` does, without copying an entry list per lookup.
`std.x.crypto.rsa` and `std.x.crypto.p384` verify RSA (PKCS#1 v1.5 and
PSS) and ECDSA P-384 signatures, and `std.x.tls.client` validates a
public-PKI chain to roots its caller supplies, so a handshake with
github.com completes. `std.x.crypto.scrypt` is RFC 7914's memory-hard
key derivation, and `std.x.crypto.passwd` writes and checks crypt(5)'s
`$7$` scrypt strings, the ones Ubuntu's crypt reads.

`std.time` reads TZif zones, including leap-aware `right/` files. With
leap records, the integer timestamp is UNIX leap time, as with the host
`date` command. `Local.leap_correction_s` keeps `to_unix` exact; set it to
zero in ordinary `Local` literals. An unreadable or malformed zone returns
an error instead of silently selecting UTC.

## Testing

Library code here is executed by lupin (the reference interpreter), by
the compiler's checked tier, and by the compiler's native tier, which
compiles, links and runs it.

The three refuse different shapes of program, so each test records
what each rung achieved. `tests/ledger.toml` holds those records for
all 741 tests. A test that passes deeper than its ledger entry claims
fails CI, the same as one that passes shallower, so the ledger has to
be kept accurate in both directions.

When a test finds a gap in the language, the gap is filed in
`docs/findings.md` and upstream. The library does not work around it.

## Building

```sh
cargo xtask std-test                     # stage + run every test on all three rungs
cargo xtask doc-examples                 # every fenced ```wolf-doc-example, executed
cargo xtask ulp                          # std.math.float accuracy + bit-for-bit agreement
cargo xtask doctor                       # which binaries resolved, and whether they match the pins
cargo xtask ledger-check                 # every test has a ledger row, and every row has a test
cargo xtask std-test --lint-conventions  # test-authoring conventions, without running tests
cargo xtask fmt-lu                       # wolf formatting, checked without rewriting
cargo xtask sync-pin                     # verify the vendored upstream snapshot against PIN
cargo xtask gen-vectors --check          # verify crypto vectors and Unicode tables/corpora
cargo xtask time-oracle                  # local time against the host's date command
cargo xtask ci                           # all of it, behind fmt/clippy/test
```

Toolchain binaries are looked up at run time from `$LUPIN_BIN` /
`$WOLF_BIN`, then `.wolf-bin/`, then `PATH`. A rung whose binary is
missing reports `SKIP: no lupin at pin …` and is counted as skipped.
The native rung also needs the runtime staticlib beside the `wolf`
binary (or `$WOLF_RT_LIB`) and skips the same way without it. Its name
is rustc's own spelling per target — `wolf_rt.lib` on windows,
`libwolf_rt.a` everywhere else — which is what every release archive
ships; a rig that knew only one of the two reported a lane dark at a
file it could see (wolf-std#18).

`upstream/` is the wolf-lang submodule (sparse `spec/` and `corpus/`),
`vendor/upstream/` the snapshot CI reads, and `vendor/tools.toml` the
binary pins. Surface conventions are in
[`API-CONVENTIONS.md`](API-CONVENTIONS.md).

## License

[GPL-3.0-or-later](LICENSE) with the
[wolf Runtime Library Exception](LICENSE-EXCEPTION). The standard
library compiles into your programs, and your programs are yours,
under any license you choose. The
[wolf Training Data Permission](LICENSE-TRAINING-DATA) lets you train
models on this repository's text and ship excerpts of it in datasets
under CC BY 4.0.
