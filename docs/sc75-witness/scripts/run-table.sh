#!/bin/bash
# sc75 (inside the container, cpus pinned by the caller): build the two
# #108 witnesses against STD (a std/ directory) on the debug and release
# tiers, then run the table. usage: run-table.sh LABEL STD OUTDIR TIMEOUT
set -u
LABEL=$1; STD=$2; OUT=$3; TO=$4
W=${WOLF:-/lane/bin/wolf-0.2.36-x86_64-unknown-linux-gnu/wolf}
mkdir -p "$OUT"
ulimit -c 0
echo "host $(uname -srm) nproc=$(nproc) $(grep -E '^(SigBlk|SigIgn)' /proc/self/status 2>/dev/null | tr '\n' ' ')"
echo "wolf $($W --version | head -1); std $STD"
B=$OUT/bin; mkdir -p $B
for w in strbuf-repro strbuf-repro-list; do
  for tier in debug release; do
    flag=; [ $tier = release ] && flag=--release
    cp -R /lane/witness/$w $B/$w-$tier-src
    $W build $B/$w-$tier-src/main.lu -o $B/$w-$tier $flag --std-root "$STD" --no-cache > $B/$w-$tier.build.log 2>&1
    echo "build $w $tier exit=$? $(sha256sum $B/$w-$tier 2>/dev/null | cut -c1-16)"
  done
done
SIZES="16384 32768 65536 98304 131072 1048576"
printf 'label\tpushes\tMiB\tbytes_printed\tstderr\twall_s\tmax_rss_MiB\toutcome\n'
for w in strbuf-repro strbuf-repro-list; do
  for tier in debug release; do
    python3 -I /lane/witness/measure.py "$LABEL $w $tier" $B/$w-$tier $TO $SIZES
  done
done
echo "TABLE_DONE $LABEL"
