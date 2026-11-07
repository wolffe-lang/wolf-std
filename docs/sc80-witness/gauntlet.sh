#!/usr/bin/env bash
# sc80's local gauntlet, run from the root of a clean checkout at the head
# under test (the recorder refuses a dirty tree). sc80's own steps first,
# then every `cargo xtask ci` step, then the cost witness. Each step prints
# `START <name>`, its output indented two spaces except `FAIL ` lines (the
# recorder's row marker, kept at column 0), and
# `END <name> exit=<n> secs=<s>`; the first nonzero step stops the run.
#
#   bash docs/sc80-witness/gauntlet.sh            the full gauntlet (Linux,
#                                                 sc80's pinned Ubuntu image)
#   bash docs/sc80-witness/gauntlet.sh --plant    toolchain + the sc80 rows
#   bash docs/sc80-witness/gauntlet.sh --mac      toolchain, the sc80 rows
#                                                 and the cost witness (macOS)
#
# Needs WOLF_BIN, LUPIN_BIN and WOLF_RT_LIB at the pins, python3 and a
# TMPDIR of the caller's own; the full run also needs cargo at the
# rust-toolchain.toml channel, Ubuntu's crypt (Python's `crypt`, Perl,
# `mkpasswd`), /oracle-versions.txt and prlimit.
set -u
cd "$(git rev-parse --show-toplevel)" || exit 2
: "${WOLF_BIN:?}" "${LUPIN_BIN:?}" "${WOLF_RT_LIB:?}" "${TMPDIR:?}"
mode=${1:-full}
scratch=$(mktemp -d "$TMPDIR/sc80-gauntlet.XXXXXX") || exit 2
trap 'rm -rf "$scratch" target/stage' EXIT

step() {
    local name=$1 rc started=$SECONDS
    shift
    printf 'START %s\n' "$name"
    set -o pipefail
    "$@" 2>&1 | sed -E '/^FAIL /!s/^/  /'
    rc=$?
    set +o pipefail
    printf 'END %s exit=%s secs=%s\n' "$name" "$rc" "$((SECONDS - started))"
    [ "$rc" -eq 0 ] || exit "$rc"
}

digest() {
    if command -v sha256sum > /dev/null 2>&1; then sha256sum "$@"; else shasum -a 256 "$@"; fi
}

toolchain() {
    echo "head $(git rev-parse HEAD)"
    "$WOLF_BIN" --version
    "$LUPIN_BIN" --version
    python3 --version
    digest "$WOLF_BIN" "$LUPIN_BIN" "$WOLF_RT_LIB"
    uname -sm
    if [ "$mode" = --mac ]; then
        sysctl -n machdep.cpu.brand_string hw.ncpu hw.memsize
        uptime
    else
        cargo --version
        cat /oracle-versions.txt
        grep -m1 'model name' /proc/cpuinfo
        grep Cpus_allowed_list /proc/self/status
        cat /proc/loadavg
    fi
}

# The sc80 rows, natively, outside the rig: each program alone in its own
# directory, its stdout passed through, so a row that breaks prints its
# own `FAIL <row>` line here.
rows() {
    local rc=0 t d got
    for t in tests/x/crypto/scrypt/*.lu tests/x/crypto/passwd/*.lu; do
        d="$scratch/rows/$(basename "$t" .lu)"
        mkdir -p "$d"
        cp "$t" "$d/"
        "$WOLF_BIN" run --std-root "$PWD/std" "$d/$(basename "$t")"
        got=$?
        echo "row file $t: exit $got"
        case "$t" in
            *_trap.lu) [ "$got" -ne 0 ] || rc=1 ;;
            *) [ "$got" -eq 0 ] || rc=1 ;;
        esac
    done
    return "$rc"
}

# RFC 7914's two N = 2^20 vectors (1 GiB of V each), under a hard 3 GiB
# address-space cap (the lane rules' memory-growing-run rule).
n20() {
    local d="$scratch/n20" rc
    mkdir -p "$d"
    cp docs/sc80-witness/rfc7914_n20/main.lu "$d/"
    "$WOLF_BIN" build --release --std-root "$PWD/std" "$d/main.lu" -o "$d/n20" || return 1
    echo "cap: prlimit --as=3221225472"
    python3 -I - "$d/n20" <<'PY'
import os, resource, subprocess, sys, time
t0 = time.monotonic()
p = subprocess.run(["prlimit", "--as=3221225472", "--", sys.argv[1]])
ru = resource.getrusage(resource.RUSAGE_CHILDREN)
print(f"exit {p.returncode} wall {time.monotonic() - t0:.2f}s child peak {ru.ru_maxrss} KiB")
sys.exit(p.returncode)
PY
}

cost() {
    python3 -I docs/sc80-witness/bench/cost.py --wolf "$WOLF_BIN" --std-root std --scratch "$scratch/cost"
}

step toolchain toolchain
step sc80-rows-native rows
[ "$mode" != --plant ] || exit 0
if [ "$mode" = --mac ]; then
    step cost-witness cost
    exit 0
fi
step ubuntu-fixture python3 -I docs/sc80-witness/interop/ubuntu_fixture.py --check tests/x/crypto/passwd/ubuntu_strings.lu
step ubuntu-interop python3 -I docs/sc80-witness/interop/run.py --wolf "$WOLF_BIN" --std-root std --scratch "$scratch/interop"
step rfc7914-n20 n20
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
step cost-witness cost
