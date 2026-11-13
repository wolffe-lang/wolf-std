#!/usr/bin/env bash
# sc84: what a Linux take needs before it may hold a block. Builds and
# runs nothing of the change.
set -eu
: "${SC84_ROOT:?}" "${SC84_TOOLS:?}" "${SC84_OUT:?}" "${SC84_CPUS:?}" "${SC84_CARGO_HOME:?}"
test -x "$SC84_TOOLS/wolf/wolf" && test -r "$SC84_TOOLS/wolf/libwolf_rt.a" && test -x "$SC84_TOOLS/lupin"
"$SC84_TOOLS/wolf/wolf" --version | head -1 | grep -q 'wolf 0.2.39 (wolfgang, pin 3fc86f7)'
"$SC84_TOOLS/lupin" --version | head -1 | grep -q 'lupin 0.1.61'
test -d "$SC84_CARGO_HOME/registry"
avail=$(awk '/MemAvailable/ {print int($2 / 1048576)}' /proc/meminfo)
test "$avail" -ge 12
podman run --rm --network none --memory 1g \
    --volume "$SC84_ROOT:$SC84_ROOT:ro" --volume "$SC84_CARGO_HOME:$SC84_CARGO_HOME:ro" \
    --env CARGO_HOME="$SC84_CARGO_HOME" --env PATH=/opt/s263-rust/bin:/usr/bin:/bin \
    --workdir "$SC84_ROOT" localhost/r41-toolchain:1 \
    cargo metadata --offline --locked --format-version 1 > /dev/null
printf 'PASS preflight: pinned wolf and lupin, runtime, offline Cargo, %s GiB available\n' "$avail"
