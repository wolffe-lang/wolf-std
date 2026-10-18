#!/usr/bin/env bash
# sc64's container witness: the poll, the group sends and this process's
# group, each only inside a throwaway container, never on a host.
#
# usage: job-container.sh <wolf-dir> <lupin> <std-root> <workdir> [taskset]
#
# Builds jobs.lu, lead.lu and all.lu for native and release ON THE HOST
# (a build signals nothing), then runs each machine in a container of its
# own: `podman run --rm --network none` with NO `--init` (podman's init
# blocks the stop signals and its children inherit the block, s242), every
# bind mount read-only, the program in a directory it made under /tmp.
# Every process the witnesses signal is one they started, or (all.lu) one
# of the container's own: the container holds only `sh` (its process 1,
# which kill(-1) leaves out), the machine and what the witness starts.
# Machines: the native and release binaries, the checked machine (`wolf
# conform-run --json --checked`, the wolf binary mounted) and lupin
# (`lupin conform-run --json`). Roles: the container's super-user, and
# user 1000:1000 (`--user`). Each run first prints the shell's SigBlk and
# SigIgn: a signal row is read only beside them. With a fifth word
# `taskset`, each program runs under `taskset -c 0-3` asked from INSIDE
# the container. After every run, every file under the mounted
# directories must still be this user's.
set -u
W=$(realpath "$1"); LU=$(realpath "$2"); S=$(realpath "$3"); K=$(realpath -m "$4"); PRE=""
[ "${5:-}" = taskset ] && PRE="taskset -c 0-3"
HERE=$(dirname "$(realpath "$0")")
IMAGE=${SC64_IMAGE:-docker.io/library/ubuntu:24.04}
ENGINE=${SC64_ENGINE:-podman}
rm -rf "$K"; mkdir -p "$K/bin" "$K/src"
grep -E 'SigBlk|SigIgn' /proc/self/status | tr '\n' ' '; echo
echo "engine: $($ENGINE --version)  image: $IMAGE $($ENGINE image inspect --format '{{.Id}}' $IMAGE | cut -c1-12)"
echo "wolf: $($W/wolf --version | head -1)   lupin: $($LU --version)"
for p in jobs lead all; do
  mkdir -p "$K/src/$p"; cp "$HERE/$p.lu" "$K/src/$p/entry.lu"
  "$W/wolf" build "$K/src/$p/entry.lu" -o "$K/bin/${p}_native" --std-root "$S" || { echo "HARNESS: build $p"; exit 2; }
  "$W/wolf" build "$K/src/$p/entry.lu" -o "$K/bin/${p}_release" --std-root "$S" --release || { echo "HARNESS: build $p release"; exit 2; }
done
me=$(id -u)
mounts=(-v "$K/bin:/w/bin:ro" -v "$K/src:/w/src:ro" -v "$W:/w/wolf:ro" -v "$S:/w/std:ro" -v "$(dirname "$LU"):/w/lupin:ro")
mask='grep -E "SigBlk|SigIgn" /proc/self/status | tr "\n" " " >&2; echo >&2'
run() {  # role machine program
  local role=$1 m=$2 p=$3 user=() cmd
  [ "$role" = user ] && user=(--user 1000:1000)
  case $m in
    native|release) cmd="$mask; mkdir /tmp/s && cd /tmp/s && $PRE /w/bin/${p}_$m" ;;
    checked) cmd="$mask; mkdir /tmp/s && cp /w/src/$p/entry.lu /tmp/s/ && cd /tmp/s && $PRE /w/wolf/wolf conform-run entry.lu --json --checked --std-root /w/std" ;;
    lupin) cmd="$mask; mkdir /tmp/s && cp /w/src/$p/entry.lu /tmp/s/ && cd /tmp/s && $PRE /w/lupin/lupin conform-run --std-root /w/std entry.lu --json" ;;
  esac
  local out err rc
  err=$(mktemp)
  out=$($ENGINE run --rm --network none "${user[@]}" "${mounts[@]}" -w /tmp "$IMAGE" sh -c "$cmd" 2>"$err"); rc=$?
  if [ "$m" = checked ] || [ "$m" = lupin ]; then
    out=$(printf '%s' "$out" | python3 -I -c 'import json,sys
t=sys.stdin.read()
try: j=json.loads(t)
except Exception: print("NOJSON "+t[-400:]); sys.exit()
o=(j.get("stdout_inline") or "").rstrip("\n")
v=j.get("verdict"); x=j.get("x-unsupported") or ""
print(o + ("" if v=="exit(0)" else " ["+v+" "+x+"]"))')
  fi
  printf '[%s %s %s] rc=%s  %s\n%s\n' "$role" "$m" "$p" "$rc" "$(head -1 "$err")" "$out"
  tail -n +2 "$err" | head -5 | sed 's/^/  stderr: /'; rm -f "$err"
  local moved; moved=$(find "$K" "$S" -not -user "$me" 2>/dev/null | head -3)
  [ -z "$moved" ] && echo "  host unchanged: every mounted file is still uid $me" || { echo "  HOST CHANGED: $moved"; exit 3; }
}
for role in root user; do
  for p in jobs lead all; do
    for m in native release checked lupin; do
      run $role $m $p
    done
  done
done
echo CONTAINER_DONE
