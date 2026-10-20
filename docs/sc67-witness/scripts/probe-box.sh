#!/usr/bin/env bash
# sc67 probe-box.sh <script> [args...]: run one of the lane's scripts inside a throwaway podman container
# (--rm --network none, the host's /usr read-only, the lane directory the only writable mount, --ulimit core=0:0).
set -u
L=~/lanes/sc67
podman run --rm --network none --userns=keep-id --ulimit core=0:0 --pids-limit 4096 \
  -v /usr:/usr:ro -v $L:$L -w $L -e HOME=$HOME -e PATH=/usr/local/bin:/usr/bin -e LANG=C.UTF-8 -e PRE="${PRE:-}" -e TIERS="${TIERS:-}" -e W="${W:-}" -e LU="${LU:-}" \
  docker.io/library/archlinux:base-devel bash "$@"
echo "BOX_EXIT=$?"
