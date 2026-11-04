#!/usr/bin/env bash
# Run from a clean checkout of the exact committed head under test.
# The caller supplies the pinned tools and a lane-owned TMPDIR.
set -u
cd "$(git rev-parse --show-toplevel)" || exit 2
: "${WOLF_BIN:?}" "${LUPIN_BIN:?}" "${WOLF_RT_LIB:?}" "${TMPDIR:?}"
scratch=$(mktemp -d "$TMPDIR/sc78-gauntlet.XXXXXX") || exit 2
trap 'rm -rf "$scratch" target/stage' EXIT

step() {
    local name=$1 rc started=$SECONDS
    shift
    printf 'START %s\n' "$name"
    set -o pipefail
    # Ordinary output is indented; FAIL is the recorder's row marker.
    "$@" 2>&1 | sed -E '/^FAIL /!s/^/  /'
    rc=$?
    set +o pipefail
    printf 'END %s exit=%s secs=%s\n' "$name" "$rc" "$((SECONDS-started))"
    [ "$rc" -eq 0 ] || exit "$rc"
}

toolchain() {
    echo "head $(git rev-parse HEAD)"
    "$WOLF_BIN" --version
    "$LUPIN_BIN" --version
    cargo --version
    python3 --version
    sha256sum "$WOLF_BIN" "$LUPIN_BIN" "$WOLF_RT_LIB"
    grep Cpus_allowed_list /proc/self/status
    cat /proc/loadavg
}

witness() {
    local mode=$1
    python3 -I docs/sc78-witness/gate.py "$mode" --repo "$PWD" \
        --wolf "$WOLF_BIN" --lupin "$LUPIN_BIN" --scratch "$scratch/$mode"
}

step toolchain toolchain
if [ "${1:-}" = --ratio-only ]; then
    step get-10000-native-ratio witness ratio
    exit 0
fi
if [ "${1:-}" = --lupin-only ]; then
    step map-lupin-small witness lupin-small
    exit 0
fi
step map-answers-three-machines witness maps
# A local one-hunk plant needs only the module rows that expose the break.
[ "${1:-}" != --plant ] || exit 0
step map-lupin-small witness lupin-small
# A red-first take runs the same two new gates before changing get.
step get-10000-native-ratio witness ratio
[ "${1:-}" != --red-first ] || exit 0
step map-other-bodies-unchanged witness unchanged
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
rm -rf target/stage
step doc-examples cargo xtask doc-examples
rm -rf target/stage
step ulp cargo xtask ulp
