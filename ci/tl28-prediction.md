# tl28 — the prediction, before the first change (ruling #92)

Written before any tl28 change to this repository is committed, against
trunk `cceda63`.

**Before** (the ten completed push/pull_request runs of `ci` up to
39287505717): six jobs a run, `rig` and `gates` on ubuntu, macOS and
windows; **137.7 job-minutes** a run (median 139.9) and **57 minutes of
wall** (median; max 65.5). `on: push` had no branch filter, so a lane's push
to a branch with an open pull request ran the whole matrix TWICE (push and
pull_request): about **275 job-minutes a push**.

**After, predicted:**

1. A lane's push runs once (push is trunk's alone). Its pull-request run is
   `tier pr`: the plan (under 1 job-minute, the planner's test inside it),
   `rig (ubuntu-latest)` (26 to 44 minutes on the ten runs) and
   `gates (ubuntu-latest)` (3 to 4), the verdict (under 1): **31 to 50
   job-minutes, about 30 to 45 minutes of wall**, macOS and windows skipped.
   Against 275 a push, that is about **85% fewer job-minutes a push**.
2. The `full-matrix` label at the head is `tier full`: every job on every
   host, **about 140 job-minutes and 57 minutes of wall** (unchanged but for
   the plan and verdict, under 2 job-minutes).
3. A dispatch with `-f tier=pr` of a branch that adds a path no rule names
   comes out `tier full`, every job on every host.
4. The planted break (an unmapped path classified pr in `ci/tiers.py`) reds
   the plan job at "the planner can say no", skips every other job, and the
   verdict job, `tier unplanned`, is red: **1 to 2 job-minutes**.
5. xtask's selftest (`every_ci_step_runs_in_the_ci_workflow`,
   `every_advisory_step_is_blessed_with_an_issue`) stays green on every
   host: tl28 adds no `cargo` line and no `continue-on-error`.
