#!/usr/bin/env bash
set -eu
: "${DW03_ROOT:?}" "${DW03_TOOLS:?}" "${DW03_OUT:?}" "${DW03_IMAGE:?}" "${DW03_CPUS:?}"
"${GAUNTLET_RIG_DIR:?}/linux-engine.sh" run --rm --network none --memory 4g --cpus 2 --pids-limit 4096 \
    --env DW03_ROOT --env DW03_TOOLS --env DW03_OUT --env GAUNTLET_RIG_DIR --env RAYON_NUM_THREADS=2 \
    --env CARGO_BUILD_JOBS=2 --env CARGO_INCREMENTAL=0 \
    --env PATH=/opt/s263-rust/bin:/usr/local/bin:/usr/bin:/bin \
    --volume "$DW03_ROOT:$DW03_ROOT" --volume "$DW03_TOOLS:$DW03_TOOLS:ro" \
    --volume "$DW03_OUT:$DW03_OUT" "$DW03_IMAGE" \
    /usr/bin/taskset.real -c "$DW03_CPUS" bash -c \
    'setsid --fork --wait bash "$GAUNTLET_RIG_DIR/linux-inner.sh" "$1"; exit $?' bash "$1"
