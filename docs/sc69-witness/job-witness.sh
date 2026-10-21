#!/usr/bin/env bash
# sc69 job-witness.sh: pty.py in a throwaway podman container (--rm --network none --ulimit core=0:0, the
# host's /usr read-only) on kasumi, four runs: wolf 0.2.34 with trunk e036bec's std (trip; the red) and
# with this branch's std (all; the green), the same two under `taskset -c 0-3`, and wolf 0.2.33 with
# e036bec's std (trip; the control: 0.2.33 leaves bits 5-7 as found). Logs in ~/lanes/sc69/evidence/.
set -u
L=~/lanes/sc69; E=$L/evidence; WD=$L/witness; HEADSTD=${1:?head std}; TRUNKSTD=$L/trunk/std
grep -E 'SigBlk|SigIgn' /proc/self/status | tr '\n' ' '; echo; uptime
box() { podman run --rm --network none --userns=keep-id --ulimit core=0:0 --pids-limit 4096 \
  -v /usr:/usr:ro -v $L:$L -w $L -e HOME=$HOME -e PATH=/usr/local/bin:/usr/bin -e LANG=C.UTF-8 \
  docker.io/library/archlinux:base-devel setsid -w "$@"; }
P=$WD/pty.py
for run in "w1 0.2.34 $TRUNKSTD trip" "w2 0.2.34 $HEADSTD all" "w3 0.2.34 $TRUNKSTD trip taskset" "w4 0.2.34 $HEADSTD all taskset" "w5 0.2.33 $TRUNKSTD trip"; do
  set -- $run; tag=$1; v=$2; std=$3; mode=$4; pre=""; [ "${5:-}" = taskset ] && pre="taskset -c 0-3"
  f=$E/witness-$tag.log
  box $pre python3 -I $P $L/bin/wolf-$v/wolf $std $WD/work-$tag $mode $L/bin/lupin-0.1.55/lupin > $f 2>&1; rc=$?
  echo "$tag wolf=$v std=$(basename $(dirname $std)) mode=$mode ${pre:+taskset} EXIT=$rc $(tail -1 $f) sha256=$(sha256sum $f | cut -c1-8)"
done
uptime; echo WITNESS_DONE
