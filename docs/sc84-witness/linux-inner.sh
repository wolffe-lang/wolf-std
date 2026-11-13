#!/usr/bin/env bash
# sc84: inside the container. Refuses unless the affinity it reads back is
# the block's; then runs one stage of the gate from the checkout.
set -eu
cd "$SC84_ROOT"
got=$(awk '/Cpus_allowed_list/ {print $2}' /proc/self/status)
if [ "$got" != "$SC84_CPUS" ]; then
    printf 'AFFINITY MISMATCH want=%s got=%s: refusing\n' "$SC84_CPUS" "$got"
    exit 70
fi
printf 'AFFINITY %s\n' "$got"
n=$(awk -F- '{print $2 - $1 + 1}' <<<"$SC84_CPUS")
export WOLF_BIN="$SC84_TOOLS/wolf/wolf" LUPIN_BIN="$SC84_TOOLS/lupin"
export WOLF_RT_LIB="$SC84_TOOLS/wolf/libwolf_rt.a"
export TMPDIR="$SC84_OUT/scratch" CARGO_BUILD_JOBS="$n" RAYON_NUM_THREADS="$n"
"$WOLF_BIN" --version | head -1
"$LUPIN_BIN" --version | head -1
case "$1" in
    codegen) timeout 1700 cargo run --frozen -q -p xtask -- codegen ;;
    ci) timeout 6400 cargo run --frozen -q -p xtask -- ci ;;
    *) printf 'unknown stage %s\n' "$1" >&2; exit 2 ;;
esac
