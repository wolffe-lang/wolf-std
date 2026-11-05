#!/usr/bin/env bash
# sc79's local gauntlet, run from the root of a clean checkout at the head
# under test (the recorder refuses a dirty tree). sc79's own steps first,
# then every `cargo xtask ci` step, then the cost witness. Each step prints
# `START <name>`, its output indented two spaces except `FAIL ` lines (the
# recorder's row marker, kept at column 0), and
# `END <name> exit=<n> secs=<s>`; the first nonzero step stops the run.
#
#   bash docs/sc79-witness/gauntlet.sh            the full gauntlet
#   bash docs/sc79-witness/gauntlet.sh --plant    toolchain + the sc79 rows
#
# Needs WOLF_BIN, LUPIN_BIN and WOLF_RT_LIB at the pins, cargo at the
# rust-toolchain.toml channel, python3, the pinned OpenSSL binary on PATH
# (vendor/vectors/pki/sc79/openssl.txt) and a TMPDIR of the caller's own.
# The github.com row needs the network; without it the row is a named SKIP.
set -u
cd "$(git rev-parse --show-toplevel)" || exit 2
: "${WOLF_BIN:?}" "${LUPIN_BIN:?}" "${WOLF_RT_LIB:?}" "${TMPDIR:?}"
scratch=$(mktemp -d "$TMPDIR/sc79-gauntlet.XXXXXX") || exit 2
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

toolchain() {
    echo "head $(git rev-parse HEAD)"
    "$WOLF_BIN" --version
    "$LUPIN_BIN" --version
    cargo --version
    python3 --version
    openssl version
    sha256sum "$WOLF_BIN" "$LUPIN_BIN" "$WOLF_RT_LIB" "$(readlink -f "$(command -v openssl)")"
    grep Cpus_allowed_list /proc/self/status
    cat /proc/loadavg
}

# The two checkers decide every vendored case again; their output must
# match the committed record byte for byte.
checker() {
    local py=$1 want=$2 got rc
    got=$(python3 -I "$py" "${@:3}") || return 1
    printf '%s\n' "$got"
    if [ "$got" = "$(cat "$want")" ]; then
        echo "ok   output identical to $want"
    else
        echo "FAIL checker-drift $py"
        diff <(printf '%s\n' "$got") "$want"
        return 1
    fi
}

# The sc79 rows, natively, outside the rig: each program alone in its own
# directory, its stdout passed through, so a row that breaks prints its
# own `FAIL <row>` line here.
rows() {
    local rc=0 t d got
    for t in tests/x/crypto/rsa/*.lu tests/x/crypto/p384/*.lu \
        tests/x/tls/cert/rsa_chain_valid.lu tests/x/tls/cert/p384_chain_valid.lu \
        tests/x/tls/cert/root_set_rows.lu tests/x/tls/cert/public_pki_refusals.lu \
        tests/x/tls/cert/san_matches_rows.lu tests/x/tls/client/cert_verify_schemes.lu; do
        d="$scratch/rows/$(basename "$t" .lu)"
        mkdir -p "$d"
        cp "$t" "$d/"
        "$WOLF_BIN" run --std-root "$PWD/std" "$d/$(basename "$t")"
        got=$?
        echo "row file $t: exit $got"
        if [ "$t" = tests/x/crypto/rsa/hash_selector_trap.lu ]; then
            [ "$got" -ne 0 ] || rc=1
        else
            [ "$got" -eq 0 ] || rc=1
        fi
    done
    return "$rc"
}

interop() {
    python3 -I docs/sc79-witness/interop/run.py --wolf "$WOLF_BIN" --std-root std \
        --scratch "$scratch/interop" "$@"
}

cost() {
    local d="$scratch/cost" mode flag rc=0
    mkdir -p "$d"
    cp docs/sc79-witness/bench/bench.lu "$d/"
    for mode in default release; do
        flag=""
        [ "$mode" = release ] && flag=--release
        echo "== bench.lu, $mode build"
        "$WOLF_BIN" run $flag --std-root "$PWD/std" "$d/bench.lu" || rc=1
    done
    cat /proc/loadavg
    return "$rc"
}

step toolchain toolchain
step sc79-rows-native rows
[ "${1:-}" != --plant ] || exit 0
step rsa-checker checker tests/x/crypto/rsa/rsa_check.py tests/x/crypto/rsa/rsa_check.txt --list
step p384-checker checker tests/x/crypto/p384/p384_check.py tests/x/crypto/p384/p384_check.txt
step hex-provenance python3 -I docs/sc79-witness/hex_provenance.py
step interop-local interop --local
if getent hosts github.com > /dev/null 2>&1; then
    step github-handshake interop --github
else
    echo "SKIP github-handshake reason=github.com-does-not-resolve"
fi
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
