# sc70 — local time

GPT-6.1-Sol, crawl 54. Base `3bacc5f`; wolf 0.2.34 / lupin 0.1.55 unchanged.

## Prediction (before implementation)

Aspen’s `src/clock/clock.lu` at `ap01` is sound enough to port for valid
zone files, including Lord Howe’s half-hour DST. It is not a hostile-input
parser: counts, type indexes, abbreviation bounds, trailing POSIX text and
some time fields need validation. Its unnamed default US DST rule should
not become a portable promise. Recommend rejecting DST strings without
explicit rules.

I predict 0 of 10,000 oracle cells disagree on the first complete run
(random seconds in 1900–2100; five zones, ten formats). Failures, if any,
will be historical `%Z`/`%z` or padding, not the calendar conversion.
Lord Howe should not expose a one-hour assumption: offsets and change
instants are seconds throughout the imported arithmetic.

## Rulings requested

- R-sc70-1: recommend `std.time` for `Local`, `local`, `local_in`,
  `offset_at`, `to_unix`, `format`; keep the TZif/POSIX machinery private.
  This is one calendar surface, not a second public zone object API.
- R-sc70-2: recommend that `to_unix` trusts `Local.offset_s`. A fold’s two
  values retain their own offset and invert exactly; a forged gap value
  is interpreted at its supplied offset without consulting the host.
  No guessed one-hour correction and no implicit earliest/latest choice.

## Inputs

The contract’s `sprints/stdlib/index.md` is named `README.md` in this tree.
There is no sc69 prose note: its record is CHANGELOG’s sc69 section,
`docs/findings.md` F-0156 and `docs/sc69-witness/`.
The shared aspen trunk has no src yet; the reader is in
`/Users/mfwolffe/lanes/ap01/aspen/src/clock/clock.lu` (read-only), also on
`origin/ap01`. Source is the maintainer’s own code, not libc or tzcode.

## Surface and policy

`std/time/local.lu` is a member of `std.time`, not a second zone facade.
The public additions are `Local` and five functions:

- `local(unix_s: int) -> Local ! {zone, io}`
- `local_in(zone: str, unix_s: int) -> Local ! {zone, io}`
- `offset_at(zone: str, unix_s: int) -> int ! {zone, io}`
- `to_unix(l: Local) -> int`
- `format(l: Local, fmt: str) -> str ! {format}`

`Local` has `year`, `month`, `day`, `hour`, `minute`, `second`, `nanos`,
`offset_s`, `is_dst`, `abbrev`. Seconds are Unix seconds, offsets seconds
east of UTC; `nanos` is zero for lookups. The inverse ignores nanos.
`to_unix` and `format` are pure, comptime-safe arithmetic (the pin still
refuses cross-module comptime calls: F-0051). The three readers charge
`Fs`/`Env`, not `Clock`, and are refused in the comptime sandbox. The
module now has seven clock functions, three zone readers and twenty
pure functions.

Lookup uses the host's files; there is no compiled-in tzdata and no
cache. `$TZDIR` or `/usr/share/zoneinfo` holds named files; colon names,
absolute paths and explicit relative colon paths work. `local` reads
`$TZ`, or `/etc/localtime` when unset; empty/unresolved settings are UTC.
`local_in` reports `zone` instead of silently changing the requested
zone; host access failures report `io`. A complete bare POSIX rule wins over a file lookup; a colon or absolute path selects a file.
Missing files become `zone`; denied/other I/O errors become `io`, the
contract's intentional two-row boundary. `local` handles `zone` as UTC
but keeps its declared row for the requested surface.

R-sc70-1 and R-sc70-2 remain recommendations for the final review; both
are implemented as above. R-sc70-3 was accepted before the first oracle:
2000–2100 for the cross-host comparison; historical edges from the TZif
fixture fields. R-sc70-4 and R-sc70-5 were accepted: retain the specified
`format` name/row despite lupin's collision, and refuse widths above
4096 with `format`. R-sc70-6 was accepted: explicit DST change rules, not
an implicit US default; leap-aware TZif files answer `zone`, rather than
silently interpreting leap-time transitions as Unix seconds.

## Fixtures and clean-room source

Aspen's reader was read from `ap01` at
`476edee3bb0ddbf2afef0681ec74fdd211d4d3da`,
`src/clock/clock.lu:19–405` (zone/rule/civil arithmetic and POSIX parsing)
and `:407–666` (the TZif design). Its valid-data arithmetic was reusable;
std adds rows, extent/index/ordering/designation/flag validation, explicit
rules, and a neighboring-year search for signed v3 transition times.
There is no fixed one-hour transition correction. Source authority is
the maintainer's own code plus RFC 8536 §§3.1–3.3 and POSIX; no foreign
libc, tzcode or coreutils source was read.

Five files were copied from this Mac's database, byte-for-byte with
symlinks dereferenced. The fixture README and Rust staging self-test
hold the sizes: New York 3552, London 3664, Kolkata 285, Lord Howe 1846,
UTC 114; total **9461 bytes**. The rig copies a sibling `fixtures/` tree
alongside an entry, recursively and byte-exactly; symlinks are refused.

The edge comments in `local_tzif.lu` derive expectations directly from
the fixture fields, not `date`: New York's first 64-bit transition is
-2717650800 (type 0 LMT -17762 before it; EST -18000 after it), UTC0 at
2^31 seconds is 2038-01-19 03:14:08, New York's footer puts July 2100 in
EDT and Lord Howe's puts January 2100 at +11. The other rows hold both
sides of New York's spring gap and autumn fold, both sides of Lord
Howe's half-hour changes, an explicit-offset inverse for a fabricated
gap, POSIX M/J/ordinal rules, equal-offset DST, signed change times that
cross New Year, malformed v1/v2 data and a synthetic valid v3 header.

## Oracle and prediction

`cargo xtask time-oracle` is part of the local gauntlet and CI's `gates`
job, including macOS at `tier full`. It builds one native wolf program,
compares exactly 200 deterministic random instants × five zones × ten
formats with the host's `date`, checks both cell count and every byte,
and fails on a compile error (including exit 2), runtime failure, skip,
truncation or difference. Its comparator self-test plants a wrong last
cell and truncated/extra outputs. Windows prints one named host skip:
that platform has neither BSD nor GNU date. Fixture tests run there.
Numeric widths have GNU-derived golden tests; Darwin supports the
padding flags but not numeric strftime widths.

The first complete GNU run (`oracle-first.log`, container, then-affinity
0–3) had **10000 cells, 0 differences, 0 skips**. This confirms the
zero-difference part of the prediction for the accepted 2000–2100 range;
the original 1900–2100 range was withdrawn under R-sc70-3 before that
run, not silently rescored. The macOS leg is owed by the final full run.
Lord Howe did not break a one-hour assumption. The extra cross-year
POSIX test did expose aspen's same-year rule selection (and the overly
tight 24-hour offset bound); it was seen red before the fix and is green
on all four machines.

The exact host anomalies that prompted R-sc70-3, on this Mac:

```
TZ=Europe/London LC_ALL=C date -r 0 '+%F %T %z %Z'
1970-01-01 01:00:00 +0000 BST
TZ=Australia/Lord_Howe LC_ALL=C date -r 0 '+%F %T %z %Z'
1970-01-01 10:00:00 +1030 AEST
```

The civil fields imply +0100 and +1000 respectively, and the fixture's
offsets confirm them. These are host formatting bugs, not semantics to
copy into std. Numeric widths also differ (Darwin prints `5Y` for `%5Y`),
so GNU width behavior is held in independent golden rows.

## Red-first evidence and downstream ledger

The prediction was committed first (`d08ec3d`). The nine initial rows
were absent on trunk in the container: wolf answered `fail(E0301)`,
lupin named the missing public function. The finalized thirteen files
were repeated against trunk (`red-final.log`); the pure trap and Fs
sandbox witness changed from E0301 to their intended trap/E0701. Lupin's
ordinary comptime refusal exists on both trees and is not called a red.
The rule-edge file was also seen red against the first implementation
(`rule-red.log`: `exit(1)`, `error: zone`) before the cross-year fix.

The CI plant is **39794499889**, head `5cddb6f`, `gates (ubuntu-latest)`'s
`time-oracle` step: `RuntimeError: wolf build exited 2 (compile error is
not a skip)`, E0301 at the missing `local_in` and `format`. The initial
unformatted plant run 39794197134 failed style first and is not the
oracle's proof. The plant's evidence is retained, never deleted.

At the wolf 0.2.35 std pin, aspen can delete its own clock reader and
strftime copies in favor of this surface; boreutils' `date` and `ls -l`
can use it. No downstream pin was changed here. #85's time half is
addressed; its locale-aware collation half stays open. The lupin issue
to file is the `format` mark/function collision described below.

The stricter flag/footer witnesses were also planted against the first
reader (`flags-red-4-7-real.log`): `exit(1)` on all four machines. The
fix checks UT=>standard indicators, v2 versus v3 rule-time bounds, and
agreement of a nonempty footer with the last transition. `g6.log` has
all thirteen files at their intended verdict on checked/native/release;
lupin runs eleven and names its two unsupported cells. These probes
run on cores 4–7 per the updated pool assignment, with uptime printed
before and after (load about 31–33); there were no timeouts to retake.

The first full gauntlet stopped on the workflow-history self-test: its
frozen pre-sc41 workflow correctly lacked the new oracle too, and the
expected missing-step list had not yet added it. That expectation was
fixed in `4633d5b`. The second gauntlet is a new run after the parser
and rig edits, not a reuse of an earlier green.

The second (pre-precedence-fix) gauntlet's executable inputs are at `4633d5b`. The source,
pins, fixtures and rig on kasumi matched the Mac's 684-file digest
`4f0f77a7c71692fb85de8b9bab5efddf83b2203821bd661fba52982ea613cc0e`
before the prose-only follow-ups. Before R-sc70-7, the only later Rust edit was the
module comment in `xtask/src/main.rs`, naming the new thirteenth step;
the later R-sc70-7 precedence fix changes an executable body and is owed a fresh final gauntlet. The digest above is retained as pre-fix evidence, not called final.


R-sc70-7 was accepted after review: a complete bare POSIX TZ rule wins
before file lookup; a colon or absolute path still selects a file.
Trying a rule such as `<+0530>-5:30` as a filename first would fail as
`io` on Windows, whose path grammar forbids those characters. It should
not depend on a readable TZDIR at all. `local_environment.lu` plants a
regular file as TZDIR (ENOTDIR) and offers that valid rule: all four
machines answered `exit(1)`, `error: io` before the fix
(`posix-precedence-red.log`, cores 4–7, uptime load 18.59). The new rule
priority is arithmetic, not an emulation of a missing host capability.


## Final gate record

Final executable code is `a514a3b`; later commits only add this note and
its evidence. The final focused table is `docs/sc70-witness/g7.log`:
13 files × four machines, all 52 observations at their intended verdicts
(including lupin's two named unsupported cells). `local_format` executes
its full subset and width goldens on all four. The final code digest is
recorded in `final-code-digest-local.log`; the queued fresh gauntlet
verifies the same source tree before running.

A serial final gauntlet is queued behind the pre-fix run, so an active
runner never reads replaced input files. It uses `taskset -c 4-7`,
`ulimit -c 0`, `--network none` and the pinned archive pair in a podman
container. Uptime brackets every gate table. Its final status, counts
and zero-skip oracle line are recorded in PR #90's Evidence index and
the hand-back after completion; they are not claimed in advance here.
The canonical audit's last line and the final PR/full run ids belong to
that same hand-back. The macOS oracle is CI's `gates` leg at `tier full`;
no native build or oracle program ran on this Mac.

Downstream adoption ledger (no pin changed here):

| Consumer | Pin that unlocks adoption | Work owed |
|---|---|---|
| aspen | std bundled with wolf 0.2.35 | replace its clock reader and calendar formatting with std.time |
| boreutils date | std bundled with wolf 0.2.35 | local calendar fields, explicit-offset inverse and format |
| boreutils ls -l | std bundled with wolf 0.2.35 | local timestamps through the same surface |

The pending issue is wolf-interp's `format` mark/function collision
(F-0157). #85's time checklist can be resolved; the issue itself remains
open for locale-aware collation. No issue was filed or closed.

## The final conform-tag correction

Tier pr 39804035303 at 062c375 was red on five invented anchor names:
`os.env.vars` and `os.fs.bytes`. This was my error: passing runtime
probes and the conventions lint did not prove registered tag validity.
No registry or pin changes. The snapshot actually publishes `os.fs`;
`tests/fs/binary_round_trip.lu` uses the forward `std.fs` with error
rows. Environment get/set has no `os.env.vars` anchor (the only env
anchor here is `os.env.unset`, which this test does not exercise), and
`tests/env/variable_round_trip.lu` cites `std.env` and the error rows.
The five headers now follow those real references.

The isolated lint in `tag-lint.rs` imports the rig's actual
`anchors::Registry` and `directive` parser, rather than copying or
weakening their rules. Against the unchanged pinned snapshot it
reproduced the exact five errors before the fix: 524 files, 1338 tags,
5 errors, 0 skips. After retagging: **524 files, 1352 tags, 1055 forward,
0 errors, 0 skips** (`tag-lint-green.log`). All thirteen focused files
were then rerun on all four machines at their intended verdicts
(`g8-retag.log`). The post-retag digest is recorded separately; its only
executable-input differences from the previous digest are the five
`conforms:` header lines. No wolf function body changed. The
orchestrator watches CI at the new labelled final head.
