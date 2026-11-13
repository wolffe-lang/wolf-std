#!/usr/bin/env bash
set -eu
cd "$WIDTH_ROOT"
got=$(python3 -I -c 'from pathlib import Path; print(next(line.split()[1] for line in Path("/proc/self/status").read_text().splitlines() if line.startswith("Cpus_allowed_list:")))')
[ "$got" = "$WIDTH_CPUS" ] || { printf 'AFFINITY MISMATCH want=%s got=%s\n' "$WIDTH_CPUS" "$got"; exit 70; }
printf 'AFFINITY %s\n' "$got"
export WOLF_BIN="$WIDTH_TOOLS/wolf/wolf" LUPIN_BIN="$WIDTH_TOOLS/lupin"
export WOLF_RT_LIB="$WIDTH_TOOLS/wolf/libwolf_rt.a"
export TMPDIR="$WIDTH_OUT/scratch" CARGO_BUILD_JOBS=6 RAYON_NUM_THREADS=6
case "$1" in
    format)
        python3 -I xtask/unicode/generate.py --wolf "$WOLF_BIN"
        "$WOLF_BIN" fmt std/str/display.lu std/str/str.lu std/unicode/unicode.lu std/unicode/display.lu tests/str/display_columns.lu tests/unicode/display_scalar.lu tests/str/display_*_trap.lu
        cargo fmt --all
        ;;
    oracle) python3 -I xtask/unicode/oracle.py ;;
    assigned|grapheme|family) python3 -I xtask/unicode/rows.py "$1" ;;
    generation) python3 -I xtask/unicode/generate.py --check --wolf "$WOLF_BIN" ;;
    invariants) python3 -I xtask/unicode/generate-test.py ;;
    size) python3 -I xtask/unicode/size.py ;;
    ci) timeout 6500 cargo run --frozen -q -p xtask -- ci ;;
    *) printf 'unknown step %s\n' "$1"; exit 64 ;;
esac
