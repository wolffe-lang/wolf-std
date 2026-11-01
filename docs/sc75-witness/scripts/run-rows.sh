#!/bin/bash
# sc75 (inside the container): each named test row under the four
# machines, by conform-run, against STD. usage: run-rows.sh STD ROW...
set -u
STD=$1; shift
W=${WOLF:-/lane/bin/wolf-0.2.36-x86_64-unknown-linux-gnu/wolf}
LU=${LUPIN:-/lane/bin/lupin-0.1.58-x86_64-unknown-linux-gnu/lupin}
ulimit -c 0
for row in "$@"; do
  d=$(mktemp -d /tmp/row.XXXXXX); cp "$row" $d/main.lu
  for m in checked native release lupin; do
    t0=$(date +%s.%N)
    case $m in
      lupin) out=$(cd $d && timeout 300 $LU conform-run main.lu --std-root "$STD" --json 2>&1) ;;
      *) out=$(cd $d && timeout 300 $W conform-run main.lu --$m --std-root "$STD" --json 2>&1) ;;
    esac
    rc=$?; t1=$(date +%s.%N)
    v=$(echo "$out" | grep -o '"verdict":"[^"]*"' | head -1)
    so=$(echo "$out" | grep -o '"stdout_inline":"[^"]*"' | head -1 | cut -c1-160)
    un=$(echo "$out" | grep -o '"x-unsupported":"[^"]*"' | head -1 | cut -c1-200)
    dg=$(echo "$out" | grep -o '"message":"[^"]*"' | head -2 | tr '\n' ' ' | cut -c1-300)
    wn=$(echo "$out" | grep -q '"warnings":\[\]' && echo "warnings=0" || echo "WARNINGS=$(echo "$out" | grep -o '"warnings":\[[^]]*' | cut -c1-200)")
    printf '%s [%s] rc=%s %s %s %s %s %s %.2fs\n' "$(basename $(dirname $row))/$(basename $row)" $m $rc "$v" "$wn" "$so" "$un" "$dg" $(awk "BEGIN{print $t1 - $t0}")
  done
  rm -rf $d
done
