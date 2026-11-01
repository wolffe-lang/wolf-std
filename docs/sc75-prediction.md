# sc75 — the prediction, before the first change past the pin

The contract makes the binary pin the first commit with nothing else in
it, so this file is the second. It is written before any measurement at
the new pair (only the two `--version` lines and the archive digests were
read) and before any change to `std/`, `tests/`, the ledger or the data
pin. Every row named here is re-measured and the note says where the
prediction was wrong.

## P1. The binary pin: wolf 0.2.36, lupin 0.1.58

- `doctor` green with both binaries matched: `wolf 0.2.36 (wolfgang, pin
  5adf3d9)` against `5adf3d91…`, `lupin 0.1.58 (wolf-interp, reference
  interpreter at pin a5378e3)` against `a5378e3f…`.
- The bare pin (trunk's tree, only `vendor/tools.toml` moved) under
  `std-test` moves **exactly one cell**: `fs/read_into_rows.lu`, lupin
  `unsupported` -> `run`. lupin 0.1.57 mirrors `List.reset` (wolf-interp#289
  is closed) and `fs.read_into` is that row's only refusal.
- Nothing else moves on lupin: wolf-interp#277 and #278 are open, so the
  thirteen 0.2.35 names stay refused by name, every sc72 wrapper row keeps
  `unsupported`, `fs/nofollow_rows.lu` keeps `divergent(stdout)`, the two
  `os.name(` / `os.arch(` mirror-lag entries keep firing, and
  `x/crypto/sha2/cavp_sha384_long` keeps `slow`.
- Nothing moves on the compiled lanes (`wolfc`, `native`): 0.2.36's work
  (C variadics, the startup and byte-argv path, the SIGPIPE repair, the
  musl runtime) has no carrier in `std/` or `tests/`.
- Confidence: medium. lupin's 59 commits are the risk: the outer-region
  copy rows, `os_cwd`'s region charge and E0409 on a member read of an
  unhandled row could each move a row I have not named.

## P2. The data pin: v0.2.35 (`cf0c13d3`) -> v0.2.36 (`5adf3d91`)

- Anchors are only added (none removed, none moving file), `os.env.argv`
  among them, with the C variadic clauses; the count is read at the
  re-vendor and the key sets are diffed both ways.
- `spec/05-conformance.md` moves with the pin: the trap set gains
  `argument-utf8`.
- No ledger cell moves on the data pin alone.

## P3. wolf-std#108, the builder

- **Trunk's table** (hasu block B, a 12 GiB container with no swap, linux
  x86-64, the debug and release tiers): quadratic in time and memory as
  the issue measured on the Mac. 1 MiB under a second, 4 MiB tens of
  seconds, 6 and 8 MiB killed by the container's cap (SIGKILL, no bytes
  printed), 64 MiB killed. The `List[byte]` control linear: 64 MiB under a
  second, peak resident under 400 MiB.
- **After** (`StrBuf` over a growable `List[byte]`): linear. 64 MiB under
  two seconds on the debug tier and faster on release; peak resident at
  most four times the content plus the runtime's baseline. The
  intermediate copies are gone because there are none: `push_str` pushes
  the argument's bytes through `s.bytes()`'s view (`[mem.str.view.lend]`,
  no materialization) and `to_str` validates and copies once.
- **The bound row** (`tests/strbuf/build_bounded.lu`): 8 MiB pushed in
  64-byte lines inside a `region r(cap: …)` block, its length and its
  last line checked, the monotonic clock read around it. **Red at trunk**:
  the cap is breached by the copies, `trap(alloc-contract)` where the
  directive says `exit(0)`, on `native` and on `wolfc`. Green after on
  `native`. On `wolfc` I expect `run` if the checked machine's step and
  byte budgets hold 8 Mi pushes, else its honest `unsupported`; on lupin
  `unsupported` or `slow` (a mut parameter's writeback costs it the list
  per call).
- The five `tests/strbuf/*.lu` rows and `str/to_strbuf_round_trip.lu`
  are unchanged and green on every lane, and on `--release` beside the
  rig's two compiled lanes.
- **The plant** `sc75-plant-builder` (base + `push_str` copying the
  accumulator again): the bound row red by name, `trap(alloc-contract)`.
- R-sc75-1: API-CONVENTIONS §8's builder sentence is amended; the public
  API (`new`, `len`, `push_str`, `push`, `insert`, `remove`, `truncate`,
  `clear`, `to_str`) keeps every signature.

## P4. wolf-std#98, the invocation name and the byte arguments

- R-sc75-2: `env.arg0() -> str` and `env.args_bytes() -> List[List[byte]]`
  in `std.env`, beside `args` and `arg`.
- lupin 0.1.58 refuses both by name (its own corpus record says
  "`env_arg0` does not resolve"): `unsupported`. is82 mirrors them on
  wolf-interp's trunk with no release, so the cells flip at lupin 0.1.59.
- Checked, under the rig (no `--argv-hex`): `env.arg0() == ""` and
  `env.args_bytes()` empty. With `--argv-hex`: the first hex is argv[0],
  the name; the list holds every entry, name first, bytes verbatim.
- Native and release, run by hand: `./prog` names `./prog`; `exec -a NAME`
  names `NAME`; a symlink names the link; a non-UTF-8 argv[1] read by
  `env.args()` traps `argument-utf8`, exit 134, the message naming index
  1, while `env.args_bytes()` reads the same bytes.
- Both are refused at comptime (D33, E0701), as `env.args` is.
