#!/usr/bin/env bash
# sc84: what a Linux take needs before it may hold a block. Builds and
# runs nothing of the change.
set -eu
: "${SC84_ROOT:?}" "${SC84_TOOLS:?}" "${SC84_OUT:?}" "${SC84_CPUS:?}" "${SC84_CARGO_HOME:?}"
test -x "$SC84_TOOLS/wolf/wolf" && test -r "$SC84_TOOLS/wolf/libwolf_rt.a" && test -x "$SC84_TOOLS/lupin"
# The pins are the checkout's own: lupin's version first, wolf's last, as CI reads them.
pins="$SC84_ROOT/vendor/tools.toml"
lver=$(sed -n 's/^version = "\(.*\)"$/\1/p' "$pins" | head -1)
wver=$(sed -n 's/^version = "\(.*\)"$/\1/p' "$pins" | tail -1)
wpin=$(sed -n 's/^pin = "\(.*\)"$/\1/p' "$pins" | tail -1)
"$SC84_TOOLS/wolf/wolf" --version | head -1 | grep -qF "wolf $wver (wolfgang, pin ${wpin:0:7})"
"$SC84_TOOLS/lupin" --version | head -1 | grep -qF "lupin $lver "
test -d "$SC84_CARGO_HOME/registry"
avail=$(awk '/MemAvailable/ {print int($2 / 1048576)}' /proc/meminfo)
test "$avail" -ge 12
podman run --rm --network none --memory 1g \
    --volume "$SC84_ROOT:$SC84_ROOT:ro" --volume "$SC84_CARGO_HOME:$SC84_CARGO_HOME:ro" \
    --env CARGO_HOME="$SC84_CARGO_HOME" --env PATH=/opt/s263-rust/bin:/usr/bin:/bin \
    --workdir "$SC84_ROOT" localhost/r41-toolchain:1 \
    cargo metadata --offline --locked --format-version 1 > /dev/null
printf 'PASS preflight: pinned wolf and lupin, runtime, offline Cargo, %s GiB available\n' "$avail"
