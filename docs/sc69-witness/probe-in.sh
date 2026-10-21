#!/usr/bin/env bash
# sc69 probe-in.sh <std-root> <outdir> <wolf-dir> <test.lu>... — runs INSIDE the throwaway container (probe.sh starts it).
# Each test is staged alone in <outdir>/<dir>__<name>/ (the rig's posture: a package of one file) and run on
# lupin 0.1.55 (conform-run --json), the checked machine (--checked), native (--native) and release (--release);
# one line per machine: verdict, x-unsupported, stdout_inline. PRE (env) prefixes each run (taskset -c 0-3).
set -u
STD=$1; OUT=$2; W=$3; shift 3
L=~/lanes/sc69; LUPIN=$L/bin/lupin-0.1.55/lupin
grep -E 'SigBlk|SigIgn' /proc/self/status | tr '\n' ' '; echo; echo "std $(cat $STD/STD-REV 2>/dev/null || echo "(tree) $STD")"; $W/wolf --version | head -1
for t in "$@"; do
  n=$(basename $(dirname $t))__$(basename $t .lu); d=$OUT/$n; rm -rf $d; mkdir -p $d; cp $t $d/main.lu
  for m in lupin checked native release; do
    # SHARED=1: one root for the four machines, in the rig's order (the rig stages a row once for its lanes).
    r=$d/$m-root; [ "${SHARED:-}" = 1 ] && r=$d/shared-root
    mkdir -p $r; cp $d/main.lu $r/main.lu; cd $r
    if [ $m = lupin ]; then ${PRE:-} $LUPIN conform-run --std-root $STD $r/main.lu --json > $d/$m.json 2> $d/$m.err
    else ${PRE:-} $W/wolf conform-run --$m --std-root $STD $r/main.lu --json > $d/$m.json 2> $d/$m.err; fi
    rc=$?
    echo "  $m rc=$rc"
  done
  echo "== $n"
done
python3 -I ~/lanes/sc69/scripts/summ.py $OUT
echo PROBE_DONE
