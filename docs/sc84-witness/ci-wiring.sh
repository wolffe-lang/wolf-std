#!/usr/bin/env bash
# sc84: the hosted plan job's two wiring proofs, run from the checkout on
# the host (they read git history), so a local take sees what the plan job
# sees: ci/tiers-test.py (the planner can say no) and ci/local-gate-test.py
# (every job against its recorded reference in ci/local-gate-baseline.json;
# a deliberate change to a job updates that reference in the same commit).
set -u
rc=0
python3 ci/tiers-test.py || rc=1
python3 -I ci/local-gate-test.py || rc=1
exit "$rc"
