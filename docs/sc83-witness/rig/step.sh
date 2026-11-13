#!/usr/bin/env bash
# Run by gauntlet-record.sh --steps with --keep-going and --preflight.
# The caller holds the block lock, then slot lock; never build a live tree.
set -uo pipefail
trap 'exit 143' TERM
trap 'exit 130' INT
trap 'exit 129' HUP
. "${GAUNTLET_RIG_DIR:?run through the recorder}/lib.sh"
cd "$CO" || exit 2
what=${1:?}
shift
case "$what" in
    preflight|release-preflight)
        bad=0
        [ "$IMAGE" = localhost/r41-toolchain:1 ] || { printf 'REFUSED manifest-image-mismatch\n'; bad=1; }
        [ -x "$P" ] || { printf 'REFUSED no-engine\n'; bad=1; }
        "$P" image inspect "$IMAGE" >/dev/null 2>&1 || { printf 'REFUSED no-image\n'; bad=1; }
        [ -f "$DIGESTS" ] && sha256sum -c "$DIGESTS" || { printf 'REFUSED toolchain-digest\n'; bad=1; }
        [ -d "$L/cargo-home/registry" ] || { printf 'REFUSED no-offline-crates\n'; bad=1; }
        for tool in git python3 sha256sum; do
            command -v "$tool" >/dev/null || { printf 'REFUSED no-tool %s\n' "$tool"; bad=1; }
        done
        [ "$bad" -eq 0 ] || exit 75
        if [ "$what" = release-preflight ]; then
            wolf_version=$("$WOLF/wolf" --version) || exit 75
            lupin_version=$("$LUPIN" --version) || exit 75
            case "$wolf_version" in
                'wolf 0.2.40 ('*) ;;
                *) printf 'REFUSED released-wolf-0.2.40-required\n'; exit 75 ;;
            esac
            case "$lupin_version" in
                'lupin 0.1.62 ('*) ;;
                *) printf 'REFUSED released-lupin-0.1.62-required\n'; exit 75 ;;
            esac
            # A registry directory alone does not prove this lockfile's
            # dependency manifests and sources are all available offline.
            ctr cargo metadata --offline --locked --format-version=1 >/dev/null || {
                printf 'REFUSED incomplete-offline-std-dependencies\n'
                exit 75
            }
        fi
        printf 'preflight: engine, image, toolchain digests and offline registry checked\n'
        ;;
    toolchain)
        ctr bash -c 'git rev-parse HEAD; "$WOLF_BIN" --version; "$LUPIN_BIN" --version
            sha256sum "$WOLF_BIN" "$WOLF_RT_LIB" "$LUPIN_BIN"
            cargo --version; python3 --version; uname -srm
            grep Cpus_allowed_list /proc/self/status'
        ;;
    wolf-fmt) ctr bash -c '"$WOLF_BIN" fmt --check std tests' ;;
    pty-kernel) ctr bash -c 'set -e
        cc -std=c11 -Wall -Wextra -Werror docs/sc83-witness/pty-write-room.c -o "$TMPDIR/pty-write-room"
        for delay in 0 100 1000; do
            printf "START kernel-probe chunk=4096 delay=%s\n" "$delay"
            "$TMPDIR/pty-write-room" 4096 "$delay"
            printf "END kernel-probe chunk=4096 delay=%s exit=0\n" "$delay"
        done' ;;
    row-unit) ctr python3 -I docs/sc83-witness/test_rows.py ;;
    rows) ctr python3 -I docs/sc83-witness/rows.py "$@" ;;
    cargo) ctr cargo "$@" ;;
    xtask) ctr cargo xtask "$@" ;;
    *) printf 'unknown step: %s\n' "$what"; exit 2 ;;
esac
