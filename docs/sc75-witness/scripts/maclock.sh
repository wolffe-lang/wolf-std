#!/bin/bash
# sc75: run one command behind the Mac build lock (the-lane-rules-1031.md).
# usage: maclock.sh <cmd> [args...]
# The lock is the directory /Users/mfwolffe/lanes/mac-build.lock, taken
# with mkdir; pid + lane inside; a dead holder's directory is removed and
# the take retried at once; a live holder is waited on, a line every 30 s.
# Inside the lock: the 8 GiB gate (free + inactive + speculative pages x
# page size), a printing wait. Then the command, then the lock removed.
L=/Users/mfwolffe/lanes/mac-build.lock
until mkdir "$L" 2>/dev/null; do
  hp=$(cat "$L/pid" 2>/dev/null)
  hl=$(cat "$L/lane" 2>/dev/null)
  if [ -n "$hp" ] && ! kill -0 "$hp" 2>/dev/null; then
    echo "maclock: holder pid $hp ($hl) is dead: removing the lock"
    rm -rf "$L"
    continue
  fi
  echo "maclock: $(date +%H:%M:%S) held by ${hl:-?} pid ${hp:-?}; waiting"
  sleep 30
done
echo $$ > "$L/pid"
echo sc75 > "$L/lane"
echo "maclock: $(date +%H:%M:%S) taken by sc75 pid $$"
need=$((8 * 1024 * 1024 * 1024))
while :; do
  ps=$(vm_stat | sed -n 's/.*page size of \([0-9]*\) bytes.*/\1/p')
  fr=$(vm_stat | awk '/Pages free/ {gsub(/\./,"",$3); print $3}')
  in=$(vm_stat | awk '/Pages inactive/ {gsub(/\./,"",$3); print $3}')
  sp=$(vm_stat | awk '/Pages speculative/ {gsub(/\./,"",$3); print $3}')
  have=$(( (fr + in + sp) * ps ))
  if [ "$have" -ge "$need" ]; then
    echo "maclock: memory gate OK ($((have / 1048576)) MiB free+inactive+speculative)"
    break
  fi
  echo "maclock: $(date +%H:%M:%S) memory gate waiting ($((have / 1048576)) MiB < 8192 MiB)"
  sleep 30
done
"$@"
rc=$?
rm -rf "$L"
echo "maclock: released (command exit $rc)"
exit $rc
