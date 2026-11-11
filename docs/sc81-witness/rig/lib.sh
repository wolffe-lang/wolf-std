# docs/sc81-witness/rig/lib.sh — sc81's Linux rig, sourced by step.sh through GAUNTLET_RIG_DIR (the
# recorder runs a stored copy of step.sh, never this tree's). The rig runs from a clean checkout of the
# commit under test. The host's facts are in the file SC81_HOST names (shell assignments kept beside
# the lane, never in the tree):
#   L      the lane's directory (everything the rig writes is under it; TMPDIR is $L/tmp/<head>)
#   P      the container engine (rootless podman)
#   IMAGE  the org's Rust toolchain image (Ubuntu 24.04, rust 1.97.1 under /opt/s263-rust)
#   WOLF   the directory of the wolf 0.2.39 archive; LUPIN the lupin 0.1.61 binary
# and, from the take: CPUS (the slot's CPUs, taskset inside the container) and MEM.
: "${SC81_HOST:?SC81_HOST names the host file}"
# shellcheck disable=SC1090
. "$SC81_HOST"
: "${L:?}" "${P:?}" "${IMAGE:?}" "${WOLF:?}" "${LUPIN:?}" "${CPUS:?}" "${MEM:?}"
CO=$(git rev-parse --show-toplevel)
TMP=$L/tmp/$(git rev-parse --short=12 HEAD)
# ctr CMD...: CMD in IMAGE, in the checkout, on the slot's CPUs. The container's pid 1 is bash as the
# container's root (a foreign uid, which reaps); CMD runs as the lane's uid in a session of its own,
# so a test's process is neither pid 1 nor a session leader, and has no controlling terminal.
ctr() {
    mkdir -p "$L/home" "$L/cargo-home" "$TMP"
    "$P" run --rm --network none --memory "$MEM" \
        --pids-limit 16384 --ulimit nproc=32768:32768 --ulimit core=0 \
        --userns keep-id:uid=1000,gid=1000 --user 0 \
        -v "$L:$L:rw" -w "$CO" \
        -e HOME="$L/home" -e TMPDIR="$TMP" \
        -e PATH=/opt/s263-rust/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin \
        -e RUSTUP_HOME=/opt/s263-rustup -e RUSTUP_TOOLCHAIN=1.97.1 \
        -e CARGO_HOME="$L/cargo-home" -e CARGO_NET_OFFLINE=true -e CARGO_INCREMENTAL=0 \
        -e CARGO_BUILD_JOBS=4 -e RUST_TEST_THREADS=4 -e CARGO_PROFILE_DEV_DEBUG=0 -e CARGO_PROFILE_TEST_DEBUG=0 \
        -e GIT_CONFIG_COUNT=2 -e GIT_CONFIG_KEY_0=commit.gpgsign -e GIT_CONFIG_VALUE_0=false \
        -e GIT_CONFIG_KEY_1=safe.directory -e 'GIT_CONFIG_VALUE_1=*' \
        -e WOLF_BIN="$WOLF/wolf" -e WOLF_RT_LIB="$WOLF/libwolf_rt.a" -e LUPIN_BIN="$LUPIN" \
        "$IMAGE" bash -c '
            cpus=$1; shift
            /usr/bin/taskset.real -c "$cpus" setsid --fork --wait \
                setpriv --reuid=1000 --regid=1000 --clear-groups --bounding-set=-all "$@"
            rc=$?
            exit "$rc"
        ' bash "$CPUS" "$@" < /dev/null
}
