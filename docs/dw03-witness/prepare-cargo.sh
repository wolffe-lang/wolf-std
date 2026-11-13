#!/usr/bin/env bash
# Preparation ONLY, on the host before preflight/queueing, never under a block.
# Like the existing std pool rigs, keep a lane-owned Cargo home and run the
# actual container gate offline. Do not read third-party dependency sources.
set -eu
: "${DW03_ROOT:?}" "${DW03_CARGO_HOME:?}" "${DW03_OUT:?}"
mkdir -p "$DW03_CARGO_HOME" "$DW03_OUT"
export CARGO_HOME="$DW03_CARGO_HOME"
CARGO_NET_OFFLINE=false cargo fetch --locked --manifest-path "$DW03_ROOT/Cargo.toml"
cargo metadata --offline --locked --manifest-path "$DW03_ROOT/Cargo.toml" --format-version 1 > "$DW03_OUT/dependencies.json"
printf 'PASS locked dependencies fetched outside the container before the take\n'
