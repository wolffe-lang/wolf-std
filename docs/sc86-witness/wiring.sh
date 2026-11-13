#!/usr/bin/env bash
set -eu
cd "${WIDTH_ROOT:?}"
python3 -I ci/local-gate-test.py
python3 -I ci/tiers-test.py
