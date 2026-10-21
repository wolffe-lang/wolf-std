# tl30 — the macOS cap: the prediction, before the first change

Written before any tl30 change to this repository is committed, against
trunk `e036bec`. Contract: `sprints/tooling/tl30-footprints-and-the-macos-cap.md`
(planning repo `wolffe-lang/wolf`, trunk `809e40f`), under `sprints/RULES.md`.
The org-wide census (every repository's macOS jobs, the last 60 runs each
ran one, every red macOS job against its Linux counterpart) is wolf-lang's
`ci/tl30.md`; this is wolf-std's slice.

**Why.** Since the Team plan (2026-10-20) the Linux pool is 60 slots and the
macOS pool is still 5: tl29 measured all five macOS slots busy 55 % of the
time after the switch, a median macOS wait of 17.6 minutes. wolf-std's merge
gate (`tier full`: the `full-matrix` label, and again on the trunk push)
holds two macOS slots, `rig (macos-latest)` for 28 to 35 minutes and
`gates (macos-latest)` for 5 to 6.

**What the macOS jobs caught.** The last 60 runs with a macOS job
(38339430209 .. 39529846846, 2026-10-13 to 10-21): 22 red macOS jobs, and
in every one the same job on ubuntu was red in the same attempt (the
planted breaks of sc61 to sc68 and s236's). **No macOS job caught a red
that no Linux job caught.** So no macOS job stays at the merge for its
record; the merge keeps one macOS smoke and the rest goes to the nightly.

**The smoke** is `gates (macos-latest)`: the pinned wolf and lupin for
aarch64-apple-darwin, and `doc-examples`, which compiles and runs every
example of the library on the macOS host (4.6 of its 5 to 6 minutes), the
conventions, `fmt-lu`, the vectors and `ulp`. `rig (macos-latest)` (the
three-lane differential `std-test` on macOS, 34 of its 35 minutes) leaves
the merge for the nightly. No `cargo` line and no `continue-on-error`
changes (xtask's selftest diffs both against `workflow::CI_STEPS`).

## The shape

| tier | when | rig | gates |
|---|---|---|---|
| `all` (new) | the nightly; `workflow_dispatch -f tier=all` | ubuntu, macOS, windows | ubuntu, macOS, windows |
| `full` | the `full-matrix` label (the merge gate); a trunk push; a dispatch (the default); a path no rule names | ubuntu, windows | ubuntu, macOS, windows |
| `pr` | any other pull-request run; `-f tier=pr` | ubuntu | ubuntu |
| `noop` | prose alone; another label | — | — |

The nightly is `tier all`, every leg as `tier full` was before tl30, and its
red still opens the `nightly-red` issue (`ci/nightly-red.sh`, unchanged).
`lane-audit.sh` accepts a green `tier all` as the superset of `tier full`
(tl29); the merge rule is unchanged: the label at the final head, a green
`tier full`.

## Prediction

Before (the six labelled and trunk-push `tier full` runs 39361104357 ..
39529846846): **39.5 macOS-minutes a run** (median; `rig` 33.3, `gates`
5.8), two macOS slots, 134.6 job-minutes, 57 minutes of wall.

1. `tier full` (the label and every trunk push): `rig` on ubuntu and
   windows, `gates` on all three: **5 to 7 macOS-minutes and one macOS slot
   a run** (falsified above 10), about **100 job-minutes** (falsified
   above 115); wall the windows rig's, 45 to 60 minutes with a free runner.
   About **−33 macOS-minutes a run, −66 a merge** (the label's run and the
   trunk push's).
2. `tier all` (the nightly, `-f tier=all`): unchanged, every job on every
   host, 125 to 150 job-minutes, 34 to 41 of them macOS.
3. `tier pr`: unchanged (ubuntu alone).
4. The planted break (`full_leg` answering every leg, the merge's shape
   before tl30) reds `ci/tiers-test.py` in the plan job of a dispatch
   `-f tier=pr` on its own branch; nothing else runs; under 2 job-minutes.
5. This pull request touches `ci/` and `.github/`, so its pull-request run
   is `tier pr`; the labelled run at the final head is the first `tier
   full` of the new shape and is the measurement (its macOS job is read
   when it ends; the orchestrator watches the run, RULES §6).

## Evidence

**Seen red locally, then in CI.** Branch `tl30-plant` (`25f6c6a`, never
merged) plants one break over `edef735`: `full_leg` answers True for every
leg, so the merge tier runs the macOS rig again (the merge's shape before
tl30). Locally, `ci/tiers-test.py` failed on exactly 24 checks (T1 five,
T3 ten, T6 two, T4 one, T7 six: every `full` plan's legs, the verdict's
green full run and its red full-on-every-leg run, the merge's legs);
unplanted it passes, 144 checks; trunk's planner (`e036bec`) under the new
test fails 27. The dispatch `gh workflow run ci --ref tl30-plant -f
tier=pr` is **run 39608592855**: the plan job (119251190133) failed at "the
planner can say no" on the same 24 (`tiers-test: FAIL (24)`), `rig` and
`gates` skipped, `tier unplanned` red, the nightly's issue skipped (a
dispatch opens none).

The pull request's own runs (its `tier pr`, and the labelled `tier full` at
the final head, the first of the new shape and the measurement) are named
in the pull request's body.
