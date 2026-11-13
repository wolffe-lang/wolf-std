#!/usr/bin/env bash
set -eu
: "${WIDTH_ROOT:?}" "${WIDTH_TOOLS:?}" "${WIDTH_OUT:?}" "${WIDTH_CPUS:?}" "${WIDTH_CARGO_HOME:?}"
: "${GAUNTLET_RIG_DIR:?}"
mkdir -p "$WIDTH_OUT/scratch"
"$GAUNTLET_RIG_DIR/linux-engine.sh" run --rm --network none --memory 12g \
    --pids-limit 16384 --ulimit nproc=32768:32768 \
    --env WIDTH_ROOT --env WIDTH_TOOLS --env WIDTH_OUT --env WIDTH_CPUS \
    --env GAUNTLET_RIG_DIR --env GAUNTLET_STEP \
    --env CARGO_HOME="$WIDTH_CARGO_HOME" --env CARGO_NET_OFFLINE=true \
    --env CARGO_INCREMENTAL=0 --env PATH=/opt/s263-rust/bin:/usr/local/bin:/usr/bin:/bin \
    --volume "$WIDTH_CARGO_HOME:$WIDTH_CARGO_HOME:ro" --volume "$WIDTH_ROOT:$WIDTH_ROOT" \
    --volume "$WIDTH_TOOLS:$WIDTH_TOOLS:ro" --volume "$WIDTH_OUT:$WIDTH_OUT" \
    localhost/r41-toolchain:1 /usr/bin/taskset.real -c "$WIDTH_CPUS" bash -c \
    'setsid --fork --wait bash "$GAUNTLET_RIG_DIR/linux-inner.sh" "$1"; exit $?' bash "$1"
