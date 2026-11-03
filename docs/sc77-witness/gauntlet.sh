#!/usr/bin/env bash
# sc77's local gauntlet, run from the root of a clean checkout at the head
# under test. Every `cargo xtask ci` step, then sc77's own steps, then the
# cost witness. Each step prints `START <name>`, its output, and
# `END <name> exit=<n> secs=<s>`; the first nonzero step stops the run.
# Needs WOLF_BIN, LUPIN_BIN and WOLF_RT_LIB at the pins, cargo at the
# rust-toolchain.toml channel, python3, and a TMPDIR of the caller's own.
set -u
cd "$(git rev-parse --show-toplevel)" || exit 2
: "${WOLF_BIN:?}" "${LUPIN_BIN:?}" "${WOLF_RT_LIB:?}" "${TMPDIR:?}"
# The trunk this branch is based on (rebased from d407430, whose std/,
# tests/, vendor/ and xtask/ are identical to it).
base=c9da7e7
c25519=std/x/crypto/curve25519/curve25519.lu
scratch=$(mktemp -d "$TMPDIR/sc77-gauntlet.XXXXXX") || exit 2
trap 'rm -rf "$scratch" target/stage' EXIT

# A step's own output never starts a line with a word the record reads.
quote() { sed -E 's/^(START|END|SKIP|RECORD|PLANT)/  \1/'; }

step() {
    local name=$1 started=$SECONDS rc
    shift
    printf 'START %s\n' "$name"
    set -o pipefail
    "$@" 2>&1 | quote
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
    sha256sum "$WOLF_BIN" "$LUPIN_BIN" "$WOLF_RT_LIB"
    grep Cpus_allowed_list /proc/self/status
    cat /proc/loadavg
}

# `verify` and every other pre-existing pub fn of the module, byte for
# byte against the base, and no consumer of `verify` touched.
fn_body() { awk -v f="$1" '$0 ~ "^pub fn " f "\\(" {on=1} on {print} on && /^}/ {exit}'; }
unchanged() {
    local rc=0 f a b
    for f in verify sign public x25519 x25519_base field_self_test to_hex; do
        a=$(git show "$base:$c25519" | fn_body "$f" | sha256sum | cut -c1-64)
        b=$(fn_body "$f" < "$c25519" | sha256sum | cut -c1-64)
        if [ "$a" = "$b" ] && [ -n "$(fn_body "$f" < "$c25519")" ]; then
            echo "ok   pub fn $f byte-identical to $base ($b)"
        else
            echo "FAIL pub fn $f differs from $base"; rc=1
        fi
    done
    if git diff --quiet "$base" -- std/x/jose std/x/tls std/x/crypto/sha2 std/x/crypto/p256 \
        std/x/crypto/chacha20 vendor xtask ci .github rust-toolchain.toml; then
        echo "ok   no consumer of verify, vendored vector, rig, CI file or pin changed since $base"
    else
        echo "FAIL a path outside the module changed:"; git diff --stat "$base" -- std/x/jose std/x/tls \
            std/x/crypto/sha2 std/x/crypto/p256 std/x/crypto/chacha20 vendor xtask ci .github rust-toolchain.toml
        rc=1
    fi
    git diff --stat "$base"
    return "$rc"
}

# The sc77 rows on the native lane, outside the rig, so a row that
# breaks prints its own `FAIL <row>` line here verbatim.
strict_rows() {
    local rc=0 t d got
    for t in ed25519_strict_small_order ed25519_strict_mixed_order ed25519_small_order_verify \
        wycheproof_ed25519_strict_smoke wycheproof_ed25519_strict_p1 wycheproof_ed25519_strict_p2 \
        wycheproof_ed25519_strict_p3 wycheproof_ed25519_strict_p4; do
        d="$scratch/rows/$t"
        mkdir -p "$d"
        cp "tests/x/crypto/curve25519/$t.lu" "$d/"
        "$WOLF_BIN" run --std-root "$PWD/std" "$d/$t.lu"
        got=$?
        echo "row file $t: exit $got"
        [ "$got" -eq 0 ] || rc=1
    done
    return "$rc"
}

cost() {
    local a="$scratch/cost/a" b="$scratch/cost/b" mode flag rc=0
    mkdir -p "$a" "$b"
    cp docs/sc77-witness/bench.lu "$a/"
    cp -r std "$b/std"
    (cd "$b" && patch -s -p1 < "$OLDPWD/docs/sc77-witness/torsion-free.patch") || return 3
    cp docs/sc77-witness/bench_full.lu "$b/"
    for mode in debug release; do
        flag=""
        [ "$mode" = release ] && flag=--release
        echo "== bench.lu, $mode, std = this head"
        "$WOLF_BIN" run $flag --std-root "$PWD/std" "$a/bench.lu" || rc=1
        echo "== bench_full.lu, $mode, std = this head + torsion-free.patch"
        "$WOLF_BIN" run $flag --std-root "$b/std" "$b/bench_full.lu" || rc=1
    done
    cat /proc/loadavg
    return "$rc"
}

step toolchain toolchain
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
step small-order-check python3 -I tests/x/crypto/curve25519/small_order.py --wolf "$WOLF_BIN" --check
step verify-unchanged unchanged
step strict-rows-native strict_rows
step std-test cargo xtask std-test
rm -rf target/stage
step doc-examples cargo xtask doc-examples
rm -rf target/stage
step ulp cargo xtask ulp
step cost-witness cost
