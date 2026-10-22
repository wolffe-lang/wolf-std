#!/usr/bin/env bash
set -u
L=$HOME/lanes/sc70
W=$L/bin/wolf-0.2.34-x86_64-unknown-linux-gnu/wolf
P=$L/bin/lupin-0.1.55-x86_64-unknown-linux-gnu/lupin
O=$L/evidence/${1:?tag}
STD=${STD:-$L/repo/std}
mkdir -p "$O"
uptime
for t in "$L"/repo/tests/time/local_*.lu; do
 n=$(basename "$t" .lu); d=$O/$n; mkdir -p "$d"; cp "$t" "$d/main.lu"; cp -r "$L/repo/tests/time/fixtures" "$d/"
 cd "$d"
 for m in lupin checked native release; do
   if [ "$m" = lupin ]; then taskset -c 4-7 "$P" conform-run --std-root "$STD" main.lu --json > "$d/$m.json" 2> "$d/$m.err"
   else taskset -c 4-7 "$W" conform-run --std-root "$STD" main.lu --json --$m > "$d/$m.json" 2> "$d/$m.err"; fi
   echo "$n $m rc=$?"
 done
done
uptime
python3 -I - "$O" <<PY
import json,pathlib,sys
for f in sorted(pathlib.Path(sys.argv[1]).glob("*/*.json")):
 try:
  d=json.loads(f.read_text()); print(f.parent.name, f.stem, str(d.get("verdict")) + " " + str(d.get("x-unsupported", "")) + " " + str(d.get("diagnostics", []))[:300] + " " + str(d.get("stdout_inline",""))[:300])
 except Exception: print(f, f.read_text()[:1000])
PY
