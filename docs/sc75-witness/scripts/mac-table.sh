#!/bin/bash
# sc75: #108's after-table on the Mac (macOS arm64), the branch's std, debug
# and release, behind the Mac build lock (the caller). usage: mac-table.sh LABEL
set -u
LABEL=$1
R=/Users/mfwolffe/lanes/sc75; W=$R/wolf-std/.wolf-bin/wolf; STD=$R/wolf-std/std
O=$R/out/$LABEL; rm -rf $O; mkdir -p $O/bin
echo "host $(uname -srm); $($W --version | head -1); std $(git -C $R/wolf-std rev-parse HEAD)"
for w in strbuf-repro strbuf-repro-list; do
  for tier in debug release; do
    flag=; [ $tier = release ] && flag=--release
    cp -R $R/witness/$w $O/bin/$w-$tier-src
    $W build $O/bin/$w-$tier-src/main.lu -o $O/bin/$w-$tier $flag --std-root $STD --no-cache > $O/bin/$w-$tier.build.log 2>&1
    echo "build $w $tier exit=$? $(shasum -a 256 $O/bin/$w-$tier | cut -c1-16)"
  done
done
printf 'label\tpushes\tMiB\tbytes_printed\tstderr\twall_s\tmax_rss_MiB\toutcome\n'
for w in strbuf-repro strbuf-repro-list; do
  for tier in debug release; do
    python3 -I $R/witness/measure.py "$LABEL $w $tier" $O/bin/$w-$tier 120 16384 32768 65536 98304 131072 1048576
  done
done
echo "TABLE_DONE $LABEL"
