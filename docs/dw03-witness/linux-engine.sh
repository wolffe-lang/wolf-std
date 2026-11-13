#!/usr/bin/env bash
# Only this lane's containers; deadline expiry is a nonzero test outcome.
# The sum of stage caps plus preflight/startup margin is below the two-hour
# lock hold. Podman's container lifetime cap avoids orphaning a compiler
# when a userspace test supervisor times out. Never signal a host group.
set -eu
if [ "${1:-}" != run ]; then exec podman "$@"; fi
shift
case "${GAUNTLET_STEP:-preflight}" in
    standard-ci) seconds=3600 ;;
    primitive-selected) seconds=1500 ;;
    release-selected) seconds=600 ;;
    generated-code) seconds=180 ;;
    bounded-lifetimes) seconds=600 ;;
    private-guards) seconds=60 ;;
    source-obligations) seconds=30 ;;
    *) seconds=120 ;;
esac
exec podman run --timeout "$seconds" "$@"
