# Host facts are supplied outside the repository through SC83_HOST:
# L (owned workspace), P (container engine), IMAGE, WOLF (directory with
# wolf and libwolf_rt.a), LUPIN (binary), DIGESTS (sha256sum check file),
# CPUS and MEM. Every take uses a clean checkout pinned to its commit.
: "${SC83_HOST:?name the host facts file}"
. "$SC83_HOST"
: "${L:?}" "${P:?}" "${IMAGE:?}" "${WOLF:?}" "${LUPIN:?}" "${DIGESTS:?}" "${CPUS:?}" "${MEM:?}"
CO=$(git rev-parse --show-toplevel)
TMP="$L/tmp/$(git rev-parse --short=12 HEAD)"
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
        -e CARGO_BUILD_JOBS=4 -e RUST_TEST_THREADS=4 \
        -e CARGO_PROFILE_DEV_DEBUG=0 -e CARGO_PROFILE_TEST_DEBUG=0 \
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
