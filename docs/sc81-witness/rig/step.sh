#!/usr/bin/env bash
# docs/sc81-witness/rig/step.sh WHAT [ARGS…] — one step of sc81's local gauntlet, run by the recorder
# (gauntlet-record.sh --steps, a manifest beside this file) from a clean checkout of the head, under
# take.sh's locks. lib.sh (GAUNTLET_RIG_DIR's) gives the host's facts and `ctr`.
#   preflight      the host has what the steps use: the engine, the image, the pinned binaries by
#                  digest, the crates cargo reads offline
#   toolchain      what ran: the head, both binaries' versions and digests, the CPUs and the load
#   rows           docs/sc81-witness/rows.py: every sc81 row on lupin, checked and native, each line
#                  held to its directive, each lupin refusal held to the builtin it names; a red line
#                  is `FAIL <test>:<lane>:<key>` at column 0 (the recorder keeps it there)
#   cargo ARGS     cargo ARGS (fmt, clippy, test)
#   xtask ARGS     cargo xtask ARGS (the rig's own steps, `cargo xtask ci`'s list)
set -uo pipefail
trap 'exit 143' TERM; trap 'exit 130' INT; trap 'exit 129' HUP
# shellcheck disable=SC1091
. "${GAUNTLET_RIG_DIR:?run by the recorder}/lib.sh"
cd "$CO" || exit 2
what=${1:?}; shift
case $what in
    preflight)
        [ -x "$P" ] || echo "REFUSED no-engine $P"
        "$P" image inspect "$IMAGE" > /dev/null 2>&1 || echo "REFUSED no-image $IMAGE"
        want="cf00e296959282c1afc1902909dc5c5898da9773aa23b7f90a5dfbe7be87b964  $WOLF/wolf
0d6e9ecdd6928da72cfc750f5880da8934f56e08ad772e0ab69986ba510065d3  $WOLF/libwolf_rt.a
486234aef146b8f61a4800b7251a2192a765c961705f0f6d1b26c89a3370bca4  $LUPIN"
        [ "$(sha256sum "$WOLF/wolf" "$WOLF/libwolf_rt.a" "$LUPIN" 2>/dev/null)" = "$want" ] \
            || echo "REFUSED toolchain-digest the binaries are not wolf 0.2.39 and lupin 0.1.61 by digest"
        [ -d "$L/cargo-home/registry" ] || echo "REFUSED no-crates $L/cargo-home has no registry (cargo runs offline)"
        command -v python3 > /dev/null || echo "REFUSED no-python3 on the host"
        echo "preflight: engine, image, binaries by digest and the offline crates looked at; CPUs $CPUS, memory $MEM"
        exit 0 ;;
    toolchain)
        ctr bash -c 'echo "head $(git rev-parse HEAD)"; "$WOLF_BIN" --version; "$LUPIN_BIN" --version
            sha256sum "$WOLF_BIN" "$WOLF_RT_LIB" "$LUPIN_BIN"; cargo --version; python3 --version
            . /etc/os-release; echo "$PRETTY_NAME"; uname -srm; grep -m1 "model name" /proc/cpuinfo
            grep Cpus_allowed_list /proc/self/status; cat /proc/loadavg; ls -l /dev/ptmx'
        ;;
    rows) ctr python3 -I docs/sc81-witness/rows.py ;;
    cargo) ctr cargo "$@" ;;
    xtask) ctr cargo xtask "$@"; rc=$?; rm -rf target/stage; exit "$rc" ;;
    *) echo "step.sh: no step $what"; exit 2 ;;
esac
