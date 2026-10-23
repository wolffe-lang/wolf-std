#!/usr/bin/env bash
# sc72 probe.sh: probe.py in a throwaway podman container (--rm --network
# none, cores off, its own session), cores 0-3. Args: probe files under
# /lane/probes/. WOLFV/LUPV pick the pair (default the published 0.2.35/0.1.56).
set -u
L=/home/mfwolffe/lanes/sc72; WV=${WOLFV:-0.2.35}; LV=${LUPV:-0.1.56}
W=/lane/archives/v$WV/unpacked/wolf-$WV-x86_64-unknown-linux-gnu/wolf
LU=/lane/archives/v$LV/unpacked/lupin-$LV-x86_64-unknown-linux-gnu/lupin
grep -E 'SigBlk|SigIgn' /proc/self/status | tr '\n' ' '; echo; uptime
podman run --rm --network none --ulimit core=0:0 -v $L:/lane -w /lane \
  -e WOLF_BIN=$W -e LUPIN_BIN=$LU -e LANES=${LANES:-lupin,checked,native} -e EXTRA="${EXTRA:-}" \
  -e STD_SRC=${STD_SRC:-/lane/wolf-std/std} \
  localhost/px57-rust setsid -w taskset -c 0-3 python3 -I /lane/scripts/probe.py "$@"
