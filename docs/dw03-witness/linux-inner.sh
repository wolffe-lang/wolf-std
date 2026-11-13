#!/usr/bin/env bash
set -eu
cd "$DW03_ROOT"
export WOLF_BIN="$DW03_TOOLS/wolf/wolf" LUPIN_BIN="$DW03_TOOLS/lupin/lupin"
export WOLF_RT_LIB="$DW03_TOOLS/wolf/libwolf_rt.a"
export TMPDIR="$DW03_OUT/scratch" CARGO_BUILD_JOBS=2 RAYON_NUM_THREADS=2 CARGO_INCREMENTAL=0
mkdir -p "$TMPDIR"
case "$1" in
    ci) timeout 5400 cargo xtask ci ;;
    selected) python3 -I "$GAUNTLET_RIG_DIR/run-selected.py" "$DW03_ROOT" "$DW03_OUT/selected" --wolf "$WOLF_BIN" --lupin "$LUPIN_BIN" --all-selected ;;
    release) python3 -I "$GAUNTLET_RIG_DIR/run-selected.py" "$DW03_ROOT" "$DW03_OUT/release" --wolf "$WOLF_BIN" --tiers release bytes/fixed_work.lu base64/raw_strict.lu bech32/bip173.lu bech32/age_keys.lu os/random/lengths.lu os/random/negative_trap.lu x/crypto/sha2/rfc4231_hmac.lu x/crypto/sha2/rfc5869_hkdf.lu x/crypto/chacha20/rfc8439_a5_aead.lu x/crypto/chacha20/tag_positions.lu x/crypto/chacha20/chunk_lifetimes.lu x/crypto/curve25519/x25519_rfc7748_s52.lu x/crypto/curve25519/rfc7748_mask_clamp.lu x/crypto/curve25519/rfc7748_noncanonical.lu ;;
    source) python3 -I "$GAUNTLET_RIG_DIR/source-obligations.py" "$DW03_ROOT" ;;
    guards) python3 -I "$GAUNTLET_RIG_DIR/private-guards.py" "$DW03_ROOT" "$DW03_OUT/guards" --wolf "$WOLF_BIN" ;;
    code) python3 -I "$GAUNTLET_RIG_DIR/capture-code.py" "$DW03_ROOT" "$DW03_OUT/code" --wolf "$WOLF_BIN" ;;
    bounded) python3 -I "$GAUNTLET_RIG_DIR/bounded-linux.py" "$DW03_ROOT" "$DW03_OUT/bounded" --wolf "$WOLF_BIN" ;;
    *) printf 'Unknown test stage\n' >&2; exit 2 ;;
esac
