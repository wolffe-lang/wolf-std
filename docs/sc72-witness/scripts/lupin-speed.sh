#!/usr/bin/env bash
# sc72 lupin-speed.sh: wall time of two slow crypto rows on lupin 0.1.55 and
# 0.1.56, the same std (trunk's, eff61c12's tree), alternated, in a
# container on cores 0-3. Then the head std on 0.1.56 (does sc72's std move it?).
set -u
L=/home/mfwolffe/lanes/sc72
echo $$ > $L/logs/lupin-speed.pid
trap 'echo $? > $L/logs/lupin-speed.status' EXIT
uptime
podman run --rm --network none --ulimit core=0:0 -v $L:/lane -w /lane localhost/px57-rust setsid -w taskset -c 0-3 bash -c '
for rep in 1 2; do
 for v in 0.1.55 0.1.56; do
  for t in cavp_sha384_long cavp_sha256_long; do
   for std in wolf-std wip; do
    [ $v = 0.1.55 ] && [ $std = wip ] && continue
    rm -rf /lane/work/speed && mkdir -p /lane/work/speed && cp -r /lane/$std/std /lane/work/speed/std && cp /lane/$std/tests/x/crypto/sha2/$t.lu /lane/work/speed/
    s=$(date +%s.%N)
    /lane/archives/v$v/unpacked/lupin-$v-x86_64-unknown-linux-gnu/lupin conform-run --std-root /lane/work/speed/std /lane/work/speed/$t.lu --json > /lane/work/speed/out.json 2>/dev/null
    e=$(date +%s.%N)
    echo "rep=$rep lupin=$v std=$std $t $(python3 -c "print(round($e-$s,1))")s $(python3 -c "import json;print(json.load(open(\"/lane/work/speed/out.json\"))[\"verdict\"])")"
   done
  done
 done
done'
uptime
echo SPEED_DONE
