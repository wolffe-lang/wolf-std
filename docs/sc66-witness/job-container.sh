#!/usr/bin/env bash
# sc66's container witness: the end by a signal (`process.exit_signal`),
# one meaning ignored or reset (`signal.ignore_one`, `reset_one`) under a
# signal the program sends itself, and the core siblings
# (`process.wait_status_core`, `poll_core`) beside the released calls --
# each only inside a throwaway container, never on a host.
#
# usage: job-container.sh <wolf-dir> <lupin> <std-root> <workdir> <core-limit> [taskset]
#
# Builds core.lu, exit_by.lu and parent.lu for native and release ON THE
# HOST (a build signals nothing), then runs each machine in a container of
# its own: `podman run --rm --network none --ulimit core=<core-limit>`
# with NO `--init`, every bind mount read-only, the program in a directory
# it made under /tmp. Every process signalled is one the witness started:
# `core.lu`'s `sh` children end themselves, `exit_by`'s modes end or
# signal only themselves. Machines: the native and release binaries, the
# checked machine (`wolf conform-run --json --checked`) and lupin (`lupin
# conform-run --json`); `parent.lu` starts the compiled `exit_by` of its
# own tier (the checked parent starts the native child). Roles: the
# container's super-user, and user 1000:1000 (`--user`). Each run first
# prints the shell's SigBlk and SigIgn and `ulimit -c`. With a sixth word
# `taskset`, each program runs under `taskset -c 0-3` asked from INSIDE the
# container. After every run, every file under the mounted directories must
# still be this user's.
#
# kasumi's `kernel.core_pattern` is a pipe to systemd-coredump, which
# core(5) says ignores RLIMIT_CORE for the kernel's decision to dump: the
# core-limit is given to the container so that the host's helper, which
# reads the limit, stores nothing. `coredumpctl` is read before and after.
set -u
W=$(realpath "$1"); LU=$(realpath "$2"); S=$(realpath "$3"); K=$(realpath -m "$4"); LIM=$5; PRE=""
[ "${6:-}" = taskset ] && PRE="taskset -c 0-3"
HERE=$(dirname "$(realpath "$0")")
IMAGE=${SC66_IMAGE:-docker.io/library/ubuntu:24.04}
ENGINE=${SC66_ENGINE:-podman}
MODES="exit-segv exit-rt exit-term-listened exit-resize sent-segv ignored-sent-segv ignored-sent-rt reset-sent-rt"
rm -rf "$K"; mkdir -p "$K/bin" "$K/src"
grep -E 'SigBlk|SigIgn' /proc/self/status | tr '\n' ' '; echo
echo "engine: $($ENGINE --version)  image: $IMAGE $($ENGINE image inspect --format '{{.Id}}' $IMAGE | cut -c1-12)  core-limit: $LIM  host core_pattern: $(cat /proc/sys/kernel/core_pattern | cut -c1-48)"
echo "wolf: $($W/wolf --version | head -1)   lupin: $($LU --version)"
since=$(date '+%Y-%m-%d %H:%M:%S')
for p in core exit_by parent; do
  mkdir -p "$K/src/$p"; cp "$HERE/$p.lu" "$K/src/$p/entry.lu"
  "$W/wolf" build "$K/src/$p/entry.lu" -o "$K/bin/${p}_native" --std-root "$S" || { echo "HARNESS: build $p"; exit 2; }
  "$W/wolf" build "$K/src/$p/entry.lu" -o "$K/bin/${p}_release" --std-root "$S" --release || { echo "HARNESS: build $p release"; exit 2; }
done
me=$(id -u)
mounts=(-v "$K/bin:/w/bin:ro" -v "$K/src:/w/src:ro" -v "$W:/w/wolf:ro" -v "$S:/w/std:ro" -v "$(dirname "$LU"):/w/lupin:ro")
mask='grep -E "SigBlk|SigIgn" /proc/self/status | tr "\n" " " >&2; echo "ulimit -c $(ulimit -c)" >&2'
run() {  # role machine program
  local role=$1 m=$2 p=$3 user=() cmd setup child
  [ "$role" = user ] && user=(--user 1000:1000)
  child=/w/bin/exit_by_native; [ "$m" = release ] && child=/w/bin/exit_by_release
  setup="mkdir /tmp/s && cd /tmp/s && echo $child > sc66.child && echo '$MODES' > sc66.modes"
  case $m in
    native|release) cmd="$mask; $setup && $PRE /w/bin/${p}_$m" ;;
    checked) cmd="$mask; $setup && cp /w/src/$p/entry.lu . && $PRE /w/wolf/wolf conform-run entry.lu --json --checked --std-root /w/std" ;;
    lupin) cmd="$mask; $setup && cp /w/src/$p/entry.lu . && $PRE /w/lupin/lupin conform-run --std-root /w/std entry.lu --json" ;;
  esac
  local out err rc
  err=$(mktemp)
  out=$($ENGINE run --rm --network none --ulimit core=$LIM "${user[@]}" "${mounts[@]}" -w /tmp "$IMAGE" sh -c "$cmd" 2>"$err"); rc=$?
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
  for p in core parent; do
    for m in native release checked lupin; do
      run $role $m $p
    done
  done
done
echo "coredumpctl since $since (the host helper's record; a core the limit refused is listed with no file):"
mine='dash|/sh |exit_by_|core_native|core_release|parent_native|parent_release'
coredumpctl list --no-pager --since "$since" 2>/dev/null | grep -E "COREFILE|$mine" || echo "  (no entry since $since)"
echo "entries since $since naming this witness's programs: $(coredumpctl list --no-pager -q --since "$since" 2>/dev/null | grep -cE "$mine"), with a stored file: $(coredumpctl list --no-pager -q --since "$since" 2>/dev/null | grep -E "$mine" | grep -c ' present ')"
echo CONTAINER_DONE
