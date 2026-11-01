# sc75 — the builder and the pin

wolf-std at the published wolf 0.2.36 / lupin 0.1.58; `std.strbuf` over a
growable byte list (wolf-std#108); `std.env.arg0` and `std.env.args_bytes`
over s261's builtins (wolf-std#98). Contract:
`wolf/sprints/stdlib/sc75-the-builder-and-the-pin.md`, under
`the-lane-rules-1031.md`. Base: trunk `5f49efd8`.

## Forbidden

- The clean room: no libc's source was read. The UTF-8 width rule in
  `strbuf.push` is `[mem.str.chars]`'s and the arithmetic is the shape
  `std.json`'s `utf8_of` already used; the argv behaviour is
  `[os.env.argv]` and black-box runs.
- kasumi was never touched. Local runs were on the Mac behind the Mac
  build lock (`mkdir` protocol, the 8 GiB gate inside it) and on hasu
  block B holding `hasu-block-B.lock` outside `hasu-B.lock`, in rootless
  podman with `--memory 12g --memory-swap 12g`, pinned to CPUs 6-11
  inside the container.
- Trunk's quadratic builder was measured only inside that 12 GiB
  container on hasu, never on the Mac (it reaches 8.2 GB at 1 MiB).
- No label, merge, tag, release, issue filed or closed; no CI waited on;
  no attribution trailer; the planning repo was read only.

## Inputs, verified

- **The published pair, digests read twice.** The release API's eleven
  assets of `v0.2.36` (wolf-lang) and `v0.1.58` (wolf-interp) were read
  twice and each read was set-equal to `r41-evidence.md`'s eleven-row
  table. The four archives used here (macOS arm64 and linux x86-64, wolf
  and lupin) were downloaded into their own directories and hashed twice:
  `b79d5584…`, `d383c74c…`, `c1b02771…`, `34f52792…`, all matching. On
  hasu the two linux archives were hashed twice again after the copy.
- `--version`: `wolf 0.2.36 (wolfgang, pin 5adf3d9)` / `paired with
  lupin 0.1.58 (reference interpreter), pin a5378e3`; `lupin 0.1.58
  (wolf-interp, reference interpreter at pin a5378e3)`. The tag object
  `86d61559` peels to `5adf3d91`; lupin's conformance pin `a5378e3f` is
  v0.2.35 + 138, 123 commits short of v0.2.36, and contains s261's merge
  `7b7b1753`. Binaries by name: wolf linux `f7b73fba…`, macOS
  `6edc5016…`; lupin linux `47c646a6…`; the GNU `libwolf_rt.a`
  `47fb9ad0…` (r41's).
- The archive's bundled `std/` is wolf-std `78b9d3cb`; it was removed
  from every staged copy, as CI removes it.
- The data pin: anchors 693 -> 706, key sets diffed both ways, 13 added
  (`abi.c.variadic` and nine children, `abi.target.main`,
  `exec.checked.argv`, `os.env.argv`), none removed, no entry changed;
  `anchors.json` and `spec/05-conformance.md` byte-identical to the tag.
- lupin 0.1.58 does not mirror s261 (its own corpus record: "`env_arg0`
  does not resolve"); is82 mirrors it on wolf-interp's trunk without a
  release (wolf-interp#295 open), so the lupin cells flip at 0.1.59.

## Prediction

`docs/sc75-prediction.md`, committed second (`c8e24c6`), after the pin
and before any measurement at the new pair. Against the outcome:

- **P1, the binary pin.** `doctor` green with both binaries matched:
  **as predicted** (gate g0 at `f7c1180`, the two pins and nothing else
  moved; `docs/sc75-witness/gate-g0-f7c1180-summary.log`). The bare pins
  moved **two** lupin cells, not the one predicted:
  `fs/read_into_rows.lu` `unsupported` -> `run` (**predicted**: 0.1.57
  resolves `List.reset`, wolf-interp#289), and `time/local_format_rows.lu`
  `unsupported` -> `run` (**not predicted**: lupin 0.1.57 "resolves a
  declared return mark before a module function of the same name", which
  is the `format` mark/function collision sc70 ledgered). Each flipped in
  its own commit (`9c9f146`, `46ecebf`). Nothing else moved on lupin
  (the thirteen 0.2.35 names still refused by name, wolf-interp#277 and
  #278 open; `fs/nofollow_rows` still `divergent(stdout)`;
  `cavp_sha384_long` still `slow`), and nothing moved on `wolfc` or
  `native`: **as predicted**. g0 stopped at `std-test`, so its
  `doc-examples` and `ulp` were first read at the final head (g1).
- **P2, the data pin.** 13 anchors added, none removed, no entry changed;
  the trap set gains `argument-utf8` and `[exec.checked.argv]` arrives:
  **as predicted**. g0 measured the two pins together, so "no cell moves
  on the data pin alone" is not separable from P1; both moved cells are
  lupin-mirror explanations, on the binary side.
- **P3, the builder.** Trunk's table was **worse than predicted** on linux
  x86-64: 1 MiB took 3.78 s at 8,227 MiB resident (predicted under a
  second), and 2 MiB was already killed at the 12 GiB cap after 6.2 s
  (predicted: tens of seconds at 4 MiB, kills from 6 MiB). The issue's
  macOS numbers (0.5 s at 1 MiB) are the Mac's; the Mac was not asked to
  reproduce them. The `List[byte]` control: 64 MiB in 0.18 s at 194 MiB,
  **as predicted**. After: 64 MiB in 0.22 s debug / 0.15 s release at 258
  MiB on linux, and 0.08 s / 0.05 s at 258 MiB on the Mac: the 194 MiB the
  `List[byte]` control holds (the list and its doubling history) plus
  `to_str`'s one 64 MiB copy, about four times the content and **within
  the predicted bound**. The bound row was red at trunk as predicted,
  `trap(alloc-contract)` on `native` and `wolfc`, and also on release and
  lupin (its capped build is the first thing every machine runs). After,
  `native` and release run it in 0.16 s / 2.6 s (compile included);
  `wolfc` answers `unsupported` after 3.2 s of running (the trunk run
  shows that machine reaches the cap, so this is one of its two budgets,
  `[exec.checked.budget]`; the record's sentence is in `rows-code.log`);
  lupin passes 300 s (`slow`): the hedged prediction, measured. The plant
  (`sc75-plant-builder`, base + `push_str` copying the accumulator) runs
  by hand at the head (`rows-plant.log`) and on its draft PR's CI; both
  follow this commit, so their results are in the hand-back.
- **P4, the names.** `env.arg0` / `env.args_bytes`, lupin's refusal by
  name ("`env_arg0` does not resolve", and "`env_args_bytes` does not
  resolve" for a program that reads the list first), the checked
  machine's empty argv without `--argv-hex`, D33's E0701 at both
  builtins: **as predicted**. The argv rows by hand (below) are as
  predicted on native, release and checked, with one correction to my
  own witness script: an argument's index in the trap message is the OS
  argv index, so `./text ok $'\xff'` names argument 2, as the clause
  says.

## Evidence index

Every path is under `docs/sc75-witness/` unless it says otherwise.

- **The pins.** `gate-g0-f7c1180-summary.log`: the CI step list at the
  two pins and nothing else (hasu block B, `r41-toolchain:1`, a foreign
  init, its own session), `doctor` matching both pins, `sync-pin` at
  `5adf3d91`, `std-test` red on exactly the two lupin cells above.
- **#108, trunk.** `table-trunk-5f49efd-linux.log`: #108's two witnesses
  (`strbuf-repro/`, `strbuf-repro-list/`, the issue's sources verbatim)
  built against trunk's std on the debug and release tiers and run at 1,
  2, 4, 6, 8 and 64 MiB (`scripts/measure.py`: wall, the child's own peak
  resident set, bytes printed, stderr, outcome), in a 12 GiB container
  with no swap. `bound-row-new-and-trunk-linux.log`: the bound row on
  four machines against the new std and against trunk's
  (`trap(alloc-contract)` on all four at trunk).
- **#108, after.** `table-after-linux-dev1.log` (hasu, the same
  container; run against a synced std whose `strbuf.lu` is byte-identical
  to `65e1330`'s, checked with `cmp`, hence its `dev1` label) and
  `table-after-mac-46ecebf.log` (the Mac, the issue's host; its `af49941`
  label was set before the run, and the run's std was `46ecebf`, whose
  `std/` differs from `af49941`'s in comments only).
  `rows-strbuf-linux-dev1.log`: the five existing `tests/strbuf/` rows and
  `str/to_strbuf_round_trip` on checked, native, release and lupin, each
  record's warnings read, against that same `strbuf.lu`.
- **At the final head, on hasu** (`/home/mfwolffe/lanes/sc75/logs/gate-g1/`,
  after this note was committed, so cited by path): the gauntlet, then in
  the same container the rows by hand at the head (`rows-code.log`: the
  strbuf rows, the bound row and the three #98 rows on four machines),
  the plant's std on the bound row (`rows-plant.log`), the bound row's
  clock alone against trunk's std and the head's (`timed-only.log`,
  `timed-only/main.lu` here: the capped build taken out), and the argv
  witness on linux (`argv.log`).
- **#98.** `argv/` (`show/`, `text/`, `run.sh`), `argv-mac-run1.log`
  (the Mac) and the linux run above, each on the debug and release native
  tiers (`./prog`, an absolute path with an empty argument, a bare `PATH`
  name, a symlink, `exec -a`, raw bytes `ff fe`, the text reader's trap
  on argv[2] and on a name of byte `c3`) and on the checked machine
  through `conform-run --checked --argv-hex` (none, three, an empty one,
  raw bytes, the trap twice), `--argv-hex` refused without `--checked`,
  and lupin's refusal.
  The Mac run has one invalid row and one mislabelled one, both the
  script's: its checked "no `--argv-hex`" row never reached `conform-run`
  (an empty array expanded under bash 3.2's `set -u`; the two `err:`
  lines under it are the previous row's file), and the row it labels
  "argv[1] = ff" passes `ok` first, so the byte is argv[2], which is what
  the trap names. `run.sh` at this head fixes both; the empty-argv case is
  also `env/arg0_rows.lu`'s checked cell (`agree=true` with an empty name
  and list).
- **Scripts.** `scripts/`: the Mac build lock (`maclock.sh`), the hasu
  block-B runners, the table and row runners, the gauntlet wrapper
  (`gate-inner.sh`, `gate-run.sh`).

## Done-when

- [x] The pin first, alone (`6122240`): `vendor/tools.toml` to wolf
  0.2.36 / lupin 0.1.58 as CI reads it, digests read twice against r41.
- [x] The data pin, its own commit (`f7c1180`).
- [x] The bare pins measured (g0) and the two moved cells flipped, each
  in its own commit (`9c9f146`, `46ecebf`).
- [x] #108: the failing bound committed first (`2e62c8e`), red at trunk
  on every machine; the fix (`65e1330`); §8 amended (`32819ab`, R-sc75-1);
  the issue's table re-measured before (linux) and after (linux, Mac).
- [x] #98: the rows first (`8a3f188`), then the wrappers (`af49941`,
  R-sc75-2); witnesses on native, release and checked
  (`--argv-hex`), lupin `unsupported` by name.
- [x] The note (this file), the CHANGELOG entry, the README refresh
  (`fba76e4`, README-only).
- [ ] The plant `sc75-plant-builder` as a draft PR, red by name on its
  CI (the run id is in the hand-back).
- [ ] The PR, `Footprint: cross-cutting` (the pins); the orchestrator
  labels `full-matrix` and watches `tier full`.
- [ ] The local gauntlet at the final head and the audit, once (both in
  the hand-back and the PR body).

## Rulings asked (R-list)

- **R-sc75-1 — the builder's backing and API-CONVENTIONS §8.** (A) the
  byte list, §8 amended for builders, the public API unchanged —
  **recommended and built** (`65e1330`, `32819ab`); (B) keep the `str`
  backing and document `strbuf` as "small strings only" (#108's second
  ask), which leaves std's long-output builder unusable for long output;
  (C) the byte list plus `reserve` / a capacity constructor in the same
  pass, which changes the API in a lane whose question was cost. The
  capacity now exists, so `strbuf.reserve` is no longer forbidden by
  §8; adding it is left to its own ruling.
- **R-sc75-2 — the names for #98.** (A) `env.arg0()` and
  `env.args_bytes()` in `std.env`, beside `args()` and `arg(i)`, charging
  `env` — **recommended and built** (`af49941`): the arguments already
  live there, the capability is the one `env_args` charges, `std.env` is
  a capability module (its examples need no mirror-lag entry), and the
  leaves are the builtins' own words; (B) `os.arg0()` / `os.args_bytes()`
  as the issue suggests, which splits argv across two modules and puts
  an `env` capability in `std.os`, whose surface is the machine (`cpus`,
  `name`, `arch`); (C) `env.program()`, the module header's reviewed
  name, which means the resolved executable (`os_exe`) and is a different
  answer whenever a symlink or `exec -a` is involved.

## Issues to file, and what this lane did not do

- **To close at the merge (the orchestrator's):** wolf-std#108 and
  wolf-std#98.
- **To file, wolf-std:** `strbuf.remove`'s doc says it traps `bounds`
  when the range is out of order, and its body never has: `from > to`
  slices `[..from]` and `[to..]` and splices them, duplicating the bytes
  between (trunk's `"{b.s[..from]}{b.s[to..]}"` did the same; sc75 kept
  the behaviour, since the lane's question was the cost). One line
  (`cur[from..to]` read before the splice) would make the doc true.
- **To file, wolf-interp (or ask is82):** lupin's `mut` parameter
  writeback copies a struct's `List` field on every call, so a builder
  is quadratic there whatever its backing: a 16,384-call prototype
  (`List[byte]` field, 64 bytes a call) took 96.8 s and 5.4 GB resident
  under lupin 0.1.58 on the Mac, against 0.3 s natively. It is why the
  bound row is `slow` on lupin.
- **Not done:** trunk's table on the Mac (its builder reaches gigabytes
  in a second, which the Mac's rules forbid; the issue's own Mac numbers
  stand as the before there); windows measured only by CI; lupin's #98
  cells stay `unsupported` until lupin 0.1.59 carries is82's mirror;
  `strbuf.reserve` not added (R-sc75-1).
