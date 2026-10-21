#!/usr/bin/env bash
# sc69 probe.sh <label> <wolf-version> <std-root> <tests-dir> <rel-test>... : probe-in.sh inside a throwaway podman
# container (--rm --network none --ulimit core=0:0, no --init, setsid -w), output in ~/lanes/sc69/probes/<label>/.
set -u
LBL=$1; V=$2; STD=$3; TD=$4; shift 4
L=~/lanes/sc69; OUT=$L/probes/$LBL; mkdir -p $OUT; W=$L/bin/wolf-$V
T=(); for r in "$@"; do T+=("$TD/$r"); done
uptime
podman run --rm --network none --userns=keep-id --ulimit core=0:0 --pids-limit 4096 \
  -v /usr:/usr:ro -v $L:$L -w $OUT -e HOME=$HOME -e PATH=/usr/local/bin:/usr/bin -e LANG=C.UTF-8 -e PRE="${PRE:-}" -e SHARED="${SHARED:-}" \
  docker.io/library/archlinux:base-devel setsid -w bash $L/scripts/probe-in.sh $STD $OUT $W "${T[@]}"
echo "podman rc=$?"
