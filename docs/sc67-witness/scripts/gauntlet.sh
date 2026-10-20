#!/usr/bin/env bash
# sc67 gauntlet.sh <tag> <rev> [full|stdonly|taskset] (from sc66's gauntlet-v3.sh): CI's rig+gates steps at <rev> in
# the run clone ~/lanes/sc67/run-<tag>; .wolf-bin = the whole published wolf 0.2.33 archive + the published lupin 0.1.54
# beside it, .wolf-bin/std removed (CI's layout); no upstream submodule (CI's posture: vendor/ keeps sync-pin honest).
# Strict (wolf-lang#571): SKIP counted over the full logs, exit statuses read from the tool, never a pipe. std-test and
# doc-examples run INSIDE A THROWAWAY PODMAN CONTAINER (--rm --network none, --ulimit core=0:0, the host's /usr read-only)
# because the suite's signal rows signal their own processes; every other step runs on the host and signals nothing.
set -u
TAG=$1; REV=$2; MODE=${3:-full}
L=~/lanes/sc67; E=$L/evidence; D=$L/run-$TAG; W=$L/bin/wolf-0.2.33; mkdir -p $E
export CARGO_BUILD_JOBS=6 CARGO_TARGET_DIR=$L/target-$TAG CARGO_INCREMENTAL=0
unset WOLF_STD LUPIN_STD WOLF_BIN LUPIN_BIN WOLF_RT_LIB STD_TEST_TIMEOUT_SECS
grep -E 'SigBlk|SigIgn' /proc/self/status | tr '\n' ' '; echo
test -d $D || git clone -q https://github.com/wolffe-lang/wolf-std.git $D
cd $D && git fetch -q origin '+refs/heads/trunk:refs/remotes/origin/trunk' '+refs/heads/sc67*:refs/remotes/k/sc67*' || { echo FETCH_FAILED; exit 9; }
if [ -f $L/sc67.bundle ]; then git fetch -q $L/sc67.bundle '+refs/heads/*:refs/remotes/b/*' || { echo BUNDLE_FETCH_FAILED; exit 9; }; fi
git checkout -q -f --detach $REV || { echo CHECKOUT_FAILED; exit 9; }
git clean -q -fdx -e .wolf-bin
H=$(git rev-parse --short=7 HEAD); echo HEAD=$H
rm -rf .wolf-bin && mkdir .wolf-bin && cp -a $W/. .wolf-bin/ && cp $L/bin/lupin-0.1.54/lupin .wolf-bin/
rm -rf .wolf-bin/std; echo "wolf-bin/std present: $(test -d .wolf-bin/std && echo yes || echo no)"
sha256sum .wolf-bin/wolf .wolf-bin/lupin .wolf-bin/libwolf_rt.a
.wolf-bin/wolf --version | head -2; .wolf-bin/lupin --version; uptime
cargo build -q -p xtask || { echo XTASK_BUILD_FAILED; exit 9; }
X=$CARGO_TARGET_DIR/debug/xtask; sha256sum $X
date -u +START=%T
PRE=""; [ "$MODE" = taskset ] && PRE="taskset -c 0-3"
run() { local name=$1; shift; local f=$E/$name-$TAG-$H.log; local t0=$(date +%s)
  "$@" > $f 2>&1; local rc=$?; echo "${name}_EXIT=$rc ($(( $(date +%s)-t0 ))s) SKIP_LINES=$(grep -c 'SKIP' $f)"; tail -3 $f | cut -c1-220; }
# setsid -w: the rig must not be the container's process 1 (process/join_own_group.lu reads own_group > 1).
# STD_TEST_TIMEOUT_SECS=180: kasumi runs beside other lanes (CI's windows legs use 180 for the same reason).
box() { podman run --rm --network none --userns=keep-id --ulimit core=0:0 --pids-limit 4096 \
  -v /usr:/usr:ro -v $L:$L -w $D -e HOME=$HOME -e PATH=/usr/local/bin:/usr/bin -e LANG=C.UTF-8 -e STD_TEST_TIMEOUT_SECS=180 \
  docker.io/library/archlinux:base-devel setsid -w "$@"; }
if [ "$MODE" = stdonly ] || [ "$MODE" = taskset ]; then
  run std-test box $PRE $X std-test
  run doc-examples box $PRE $X doc-examples
else
run fmt cargo fmt --all --check
run clippy cargo clippy --workspace --all-targets -- -D warnings
run cargo-test cargo test --workspace -- --nocapture
run sync-pin $X sync-pin
run doctor $X doctor
run ledger-check $X ledger-check
run std-test box $X std-test
run lint-conventions $X std-test --lint-conventions
run fmt-lu $X fmt-lu
run gen-vectors $X gen-vectors --check
run doc-examples box $X doc-examples
run ulp $X ulp
fi
grep -E '^std-test: [0-9]+ test|RED|GREEN|directive mismatch|ledger says|timed out' $E/std-test-$TAG-$H.log | cut -c1-300
grep -E 'DOC BUG|block|GREEN|RED' $E/doc-examples-$TAG-$H.log | tail -6 | cut -c1-240
uptime
sha256sum $E/*-$TAG-$H.log
date -u +END=%T
echo JOB_DONE
