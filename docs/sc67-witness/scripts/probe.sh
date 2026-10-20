#!/usr/bin/env bash
# sc67 probe.sh (from sc66's) <outdir> <stdtree> <file>...: each file staged alone with <stdtree>/std, run on lupin
# (`lupin conform-run --json`), checked, native and release (`wolf conform-run --json --<tier>`); prints verdict,
# stdout and the unsupported reason per machine. Records kept under <outdir>. PRE (env) prefixes each run, e.g.
# PRE="taskset -c 0-3". W / LU name the toolchain (default: the published 0.2.33 / 0.1.54 pair). Run it inside the
# lane's throwaway container (probe-box.sh).
set -u
L=~/lanes/sc67; O=$(realpath -m $1); T=$(realpath $2); shift 2; W=${W:-$L/bin/wolf-0.2.33/wolf}; LU=${LU:-$L/bin/lupin-0.1.54/lupin}
unset WOLF_STD LUPIN_STD WOLF_BIN LUPIN_BIN WOLF_RT_LIB
mkdir -p $O
echo "wolf: $($W --version | head -1)   lupin: $($LU --version)   std: $T   PRE=${PRE:-}"
grep -E 'SigBlk|SigIgn' /proc/self/status | tr '\n' ' '; echo "ulimit -c $(ulimit -c) uid $(id -u)"
for f in "$@"; do
  k=$(basename $(dirname $f))__$(basename $f .lu)
  for t in ${TIERS:-lupin checked native release}; do
    s=$L/scratch/probe/$$/$k.$t; rm -rf $s; mkdir -p $s; cp $f $s/entry.lu; cp -r $T/std $s/std
    if [ $t = lupin ]; then (cd $s && ${PRE:-} timeout 120 $LU conform-run --std-root $s/std $s/entry.lu --json > $O/$k.$t.json 2> $O/$k.$t.err)
    else (cd $s && ${PRE:-} timeout 300 $W conform-run entry.lu --json --$t --std-root $s/std > $O/$k.$t.json 2> $O/$k.$t.err); fi
    rc=$?
    python3 - "$O/$k.$t.json" "$k" "$t" "$rc" <<'PY'
import json,sys
p,k,t,rc=sys.argv[1:]
try: r=json.load(open(p))
except Exception as e: print(f"{k:36} {t:8} rc={rc} NOJSON"); sys.exit()
d='; '.join((x.get('code') or '')+' '+(x.get('message') or '')[:200] for x in (r.get('diagnostics') or [])[:2] if isinstance(x,dict))
u=r.get('x-unsupported') or r.get('x-unsupported-construct') or ''
w=r.get('warnings') or []
print(f"{k:36} {t:8} rc={rc} {r.get('verdict')} out={r.get('stdout_inline')!r} {('diag='+d) if d else ''} {('unsupported='+u) if u else ''} {('warnings='+str(w)[:200]) if w else ''} ver={r.get('impl_version')}")
PY
    rm -rf $s
  done
done
rm -rf $L/scratch/probe/$$
