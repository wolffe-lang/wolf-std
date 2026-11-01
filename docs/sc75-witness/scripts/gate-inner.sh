#!/bin/bash
# sc75 gauntlet, inside the container (r41-toolchain:1, pinned to block B by
# the caller): wolf-std's CI step list (xtask workflow::CI_STEPS, in order),
# each step's exit printed as `END <step> exit=N`, stopping on the first
# nonzero. usage: inner.sh CHECKOUT
set -u
cd "$1" || exit 2
export PATH=/opt/s263-rust/bin:$PATH CARGO_HOME=/lane/cargo-home CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=6 CARGO_TERM_COLOR=never
export WOLF_BIN=/lane/wolfbin/wolf LUPIN_BIN=/lane/wolfbin/lupin
ulimit -c 0
echo "HEAD $(git rev-parse HEAD) $(git log -1 --format=%s | cut -c1-90)"
echo "$(grep -E '^(SigBlk|SigIgn)' /proc/self/status | tr '\n' ' ') pid=$$ sid=$(ps -o sid= -p $$) pgid=$(ps -o pgid= -p $$)"
cargo --version; $WOLF_BIN --version | head -1; $LUPIN_BIN --version
step() { # name, command...
  local name=$1; shift
  echo "==> $name ($(date -u +%T))"
  "$@"
  local rc=$?
  echo "END $name exit=$rc ($(date -u +%T))"
  [ $rc -eq 0 ] || { echo "GAUNTLET STOPPED at $name"; exit $rc; }
}
step time-oracle cargo xtask time-oracle
step fmt cargo fmt --all --check
step clippy cargo clippy --workspace --all-targets -- -D warnings
step test cargo test --workspace
step sync-pin cargo xtask sync-pin
step doctor cargo xtask doctor
step ledger-check cargo xtask ledger-check
step lint-conventions cargo xtask std-test --lint-conventions
step fmt-lu cargo xtask fmt-lu
step gen-vectors cargo xtask gen-vectors --check
step std-test cargo xtask std-test
step doc-examples cargo xtask doc-examples
step ulp cargo xtask ulp
echo "GAUNTLET GREEN"
