#!/usr/bin/env bash
# sc84: one step of a Linux take, in the toolchain image. Network off,
# Cargo offline from a prepared home, pinned with taskset.real to the
# block's CPUs (never the image's taskset shim), affinity read back by the
# inner script. Bash stays pid 1 and reaps; the tests run under setsid in
# a session of their own.
set -eu
: "${SC84_ROOT:?}" "${SC84_TOOLS:?}" "${SC84_OUT:?}" "${SC84_CPUS:?}" "${SC84_CARGO_HOME:?}"
: "${GAUNTLET_RIG_DIR:?}"
mkdir -p "$SC84_OUT/scratch"
"$GAUNTLET_RIG_DIR/linux-engine.sh" run --rm --network none --memory 12g \
    --pids-limit 16384 --ulimit nproc=32768:32768 \
    --env SC84_ROOT --env SC84_TOOLS --env SC84_OUT --env SC84_CPUS \
    --env GAUNTLET_RIG_DIR --env GAUNTLET_STEP \
    --env CARGO_HOME="$SC84_CARGO_HOME" --env CARGO_NET_OFFLINE=true \
    --env CARGO_INCREMENTAL=0 --env PATH=/opt/s263-rust/bin:/usr/local/bin:/usr/bin:/bin \
    --volume "$SC84_CARGO_HOME:$SC84_CARGO_HOME:ro" --volume "$SC84_ROOT:$SC84_ROOT" \
    --volume "$SC84_TOOLS:$SC84_TOOLS:ro" --volume "$SC84_OUT:$SC84_OUT" \
    localhost/r41-toolchain:1 \
    /usr/bin/taskset.real -c "$SC84_CPUS" bash -c \
    'setsid --fork --wait bash "$GAUNTLET_RIG_DIR/linux-inner.sh" "$1"; exit $?' bash "$1"
