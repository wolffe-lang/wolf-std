#!/usr/bin/env bash
# sc84: the container engine for the Linux takes. `run` gets a lifetime
# cap per step, so a userspace timeout can never orphan a compiler; every
# other verb (image inspect, the recorder's tool probe) passes through.
set -eu
if [ "${1:-}" != run ]; then exec podman "$@"; fi
shift
case "${GAUNTLET_STEP:-preflight}" in
    standard-ci) seconds=6600 ;;
    codegen) seconds=1800 ;;
    *) seconds=300 ;;
esac
exec podman run --timeout "$seconds" "$@"
