#!/usr/bin/env bash
# sc72 gate.sh LABEL REF [STEP...]: the CI step list (workflow::CI_STEPS)
# at REF (a commit fetched from GitHub), each step in a throwaway podman
# container (--rm, cores off, its own session via setsid -w, cores 0-3;
# --network none for every step but the cargo ones, which may fetch
# crates), the published wolf 0.2.35 and lupin 0.1.56 by path. One log per
# step, a .steps summary, a pid file and a status file. Launch with
# `setsid -f`.
set -u
L=/home/mfwolffe/lanes/sc72; LABEL=$1; REF=$2; shift 2
STEPS=${*:-time-oracle fmt clippy test sync-pin doctor ledger-check lint-conventions fmt-lu gen-vectors std-test doc-examples ulp}
G=$L/logs/$LABEL; mkdir -p $G
echo $$ > $G/pid
trap 'echo $? > $G/status' EXIT
: > $G/steps
WV=${WOLFV:-0.2.35}; LV=${LUPV:-0.1.56}
W=/lane/archives/v$WV/unpacked/wolf-$WV-x86_64-unknown-linux-gnu/wolf
LU=/lane/archives/v$LV/unpacked/lupin-$LV-x86_64-unknown-linux-gnu/lupin
CO=${CHECKOUT:-wolf-std}; TG=${TARGET:-target}
cd $L/$CO || exit 2
git fetch -q origin || exit 2
git fetch -q origin sc72-wip 2>/dev/null || true
git checkout -q -f "$REF" || exit 2
git clean -q -fdx -e target || true
echo "HEAD $(git rev-parse HEAD) $(git log -1 --format=%s | cut -c1-80)" | tee $G/head
grep -E 'SigBlk|SigIgn' /proc/self/status | tr '\n' ' '; echo; uptime | tee -a $G/uptime
box() { # net args...
  local net=$1; shift
  podman run --rm $net --ulimit core=0:0 -e CARGO_INCREMENTAL=0 -e CARGO_BUILD_JOBS=4 \
    -e CARGO_TARGET_DIR=/lane/$TG -e WOLF_BIN=$W -e LUPIN_BIN=$LU -e CARGO_TERM_COLOR=never \
    -v $L:/lane -w /lane/$CO localhost/px57-rust setsid -w taskset -c 0-3 "$@"
}
box "" cargo build --release --locked -p xtask > $G/build.log 2>&1 || { echo "build=FAIL" | tee -a $G/steps; exit 3; }
X=/lane/$TG/release/xtask
for s in $STEPS; do
  date -u +%H:%M:%S | tr '\n' ' '
  case $s in
    fmt) box "" cargo fmt --all --check ;;
    clippy) box "" cargo clippy --workspace --all-targets -- -D warnings ;;
    test) box "" cargo test --workspace ;;
    lint-conventions) box "--network none" $X std-test --lint-conventions ;;
    gen-vectors) box "--network none" $X gen-vectors --check ;;
    *) box "--network none" $X $s ;;
  esac > $G/$s.log 2>&1
  rc=$?
  echo "$s=$rc $(sha256sum $G/$s.log | cut -c1-8) $(grep -c 'SKIP' $G/$s.log) skip-lines" | tee -a $G/steps
done
uptime | tee -a $G/uptime
echo GATE_DONE
