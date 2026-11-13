#!/usr/bin/env bash
# sc84: the gate on an arm64 Mac, natively, with the pinned macOS wolf and
# lupin (SC84_MAC_TOOLS: a directory holding the extracted wolf archive as
# wolf/ and the lupin binary). At most two build workers; the caller holds
# the machine's build lock and its disk and memory floors.
set -eu
: "${SC84_MAC_TOOLS:?}"
export PATH="$HOME/.cargo/bin:$PATH"
export WOLF_BIN="$SC84_MAC_TOOLS/wolf/wolf" LUPIN_BIN="$SC84_MAC_TOOLS/lupin"
export WOLF_RT_LIB="$SC84_MAC_TOOLS/wolf/libwolf_rt.a"
export CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0
uname -sm
"$WOLF_BIN" --version | head -1
"$LUPIN_BIN" --version | head -1
case "$1" in
    codegen) cargo run -q -p xtask -- codegen ;;
    *) printf 'unknown stage %s\n' "$1" >&2; exit 2 ;;
esac
