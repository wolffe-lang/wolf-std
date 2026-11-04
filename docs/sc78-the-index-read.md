# sc78: `map.get` by the index read (wolf-std#109)

Footprint: module

The classifier reads std/map as std, tests as rig gates, docs as prose; no CI, pin, spec or runtime path.

## Prediction
- `std.map.get` is `let v = m[k] else { return none }; copy v`: the language's index read, as `has` already does, with `none` for a missing key exactly as before. Signature and `cmp.Eq` bound unchanged; `get_or` and `tally` unchanged.
- It no longer builds a `pairs()` list or compares every entry through trait dispatch. Its algorithm is the runtime's, so a hashed runtime benefits it directly.
- Side effect (a correction): the old walk handed back an alias for List-valued maps on the compiler tiers; `get` now returns an independent copy.
- Tests: 8 new files in `tests/map/` (four key-type agreement rows: int, char, bool, str; two independent-value rows; `get_many` at 10k and a 1k smoke). Count 569 -> 577. Docs: CHANGELOG Unreleased, README, `docs/sc78-prediction.txt`, `docs/sc78-witness/`.

Base program from #109 (N inserts of `"0#key{i}"`, then N lookups), same caps for both sides.
- Base, 10k, native get vs index: 1.209 s vs 0.179 s (ratio 6.75); checked get aborts under the 3 GiB cap, checked index 0.247 s. At 100k get aborts on all three tiers.
- Head, 1k, lupin, paired: base 3.442587 / 3.390245 s, head 0.533907 / 0.543253 s (6.34x faster). RSS 182 MiB -> 13 MiB.
- Head, 10k, lupin: 59.327597 s (exit 0) and 60.002804 s CENSORED at the 60 s ceiling; base 60.038733 / 60.037478 s both CENSORED. Censored times establish no relative 10k claim.
- Lupin's own `std.map.set` insert-only, paired: N=1000 0.258153 s, N=10000 29.761094 s. 10x N costs about 115x, so insertion is superlinear there. Lupin's 10k cell is therefore reference-only "slow"; the gate is the checked and native rows plus a 1k companion on every machine.
- Gates (red at base by name, green at head): checked 10k must finish and return 10000; native get/index ratio <= 2.0.

## Forbidden
- Clean room: no libc or crypto-library source; none was needed.
- No change to `Map`'s semantics, to any other module, or to the runtime (the runtime's hashed index is a separate wolf-lang change).
- No closing keyword in any commit or in this body; issue 109 is handled by hand at the merge.
- Nothing labelled, merged, tagged or closed by the lane; no attribution trailers.

## Inputs verified
- Base: trunk `b1babae3`. Head: `68efc2a13a7aab275be3cf23947802e063026ff6`, a fast-forward of the base.
- Interpreters and compilers: the repository's own pins (lupin 0.1.58 for the reference tier), unchanged by this branch.
- The green record and the plant record were each verified by the job that wrote them (`PASS local-green-and-plant-verified`) and again by the local-record audit.

- R-sc78-1: use the language's index equality and keep the `Eq` bound. For today's allowed keys (int, char, bool, str) each `std.cmp` Eq impl is exactly `self == other`; floats are rejected as Map keys. Any future key-protocol expansion must revisit this.
- R-sc78-2: `get` copies only the selected value. Returning `m[k]` directly is E1002 (a move out of a `read` parameter), and `copy m[k]` on a heap-bearing row is unsupported natively at this pin, hence the `let ... else` form.
- R-sc78-3 (operational): a queued job is withdrawn by stopping its own lock waiter by pid, so its wrapper still writes the job's marker; never by process group.
- R-sc78-4: one agreement row per key type. Lupin 0.1.58 cannot resolve `impl Eq for char` through `cmp.Eq.eq` (the old walk's comparison), so only the char cell is `unsupported`, with that exact reason gated; any other lupin outcome fails the row. The 10k `get_many` source is unchanged; its lupin cell is slow with censored times.

## Evidence index
Green gauntlet, Linux, `RECORD-END worst=0 steps=18 end=2026-11-04T12:17:28Z`:
```
END toolchain exit=0 secs=1
END map-answers-three-machines exit=0 secs=4
END map-lupin-small exit=0 secs=1
END get-10000-native-ratio exit=0 secs=2
END map-other-bodies-unchanged exit=0 secs=0
END time-oracle exit=0 secs=19
END fmt exit=0 secs=0
END clippy exit=0 secs=2
END test exit=0 secs=1
END sync-pin exit=0 secs=0
END doctor exit=0 secs=0
END ledger-check exit=0 secs=1
END lint-conventions exit=0 secs=0
END fmt-lu exit=0 secs=0
END gen-vectors exit=0 secs=1
END std-test exit=0 secs=1848
END doc-examples exit=0 secs=153
END ulp exit=0 secs=0
RECORD-END worst=0 steps=18 end=2026-11-04T12:17:28Z
```
Plant `sc78-plant-absent-r4` at 75a5397af6bbee42c734e7e1d247bd0e8adaefa8 (one commit, one file, one hunk: a missing key answers `m.pairs()[0].1` instead of `none`; pushed, no PR). Record:
```
END toolchain exit=0 secs=1
END map-answers-three-machines exit=1 secs=0
END record-wrapper exit=1 secs=0
RECORD-END worst=1 steps=3 end=2026-11-04T12:17:30Z
```
Red by name, e.g. `FAIL get_hit_miss-native` and `FAIL get_int_agreement-checked` (every map row fails on each machine). The job's verifier: `PASS local-green-and-plant-verified`.
Red-first for R-sc78-4 (before the lupin posture): `FAIL get_char_agreement-lupin`, `END map-lupin-small exit=1`.

Witness files on the branch: `docs/sc78-prediction.txt`, `docs/sc78-witness/` (the diagnostic data, the completed after-rows and the reference timing correction), and the eight new files in `tests/map/`.

## Done-when
- [x] Local gauntlet green at the exact head (18 steps, every END parsed)
- [x] Plant red by name
- [x] Red-first gates at base (checked 10k, native ratio, char lupin)
- [x] Key-type agreement on native, release, checked, lupin, macOS
- [x] All existing map tests unchanged
- [x] Local-record audit (`lane-audit.sh --local`, run at the merge)

Issue 109 is handled by hand at the merge.

Issue 109 is handled by hand at the merge.

