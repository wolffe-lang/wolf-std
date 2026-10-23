#!/usr/bin/env bash
# sc72 acquire.sh: the published pair (and the old pair, the control), each
# asset's digest read twice from the release API, downloaded, hashed twice,
# unpacked; version lines; std tree against the tag's STD-REV.
set -u
L=~/lanes/sc72; A=$L/archives; mkdir -p $A $L/logs
grep -E 'SigBlk|SigIgn' /proc/self/status | tr '\n' ' '; echo; uptime
get() { # repo tag asset
  local repo=$1 tag=$2 asset=$3; local d=$A/$tag
  mkdir -p $d
  local w1 w2
  w1=$(gh api repos/wolffe-lang/$repo/releases/tags/$tag --jq ".assets[]|select(.name==\"$asset\")|.digest")
  w2=$(gh api repos/wolffe-lang/$repo/releases/tags/$tag --jq ".assets[]|select(.name==\"$asset\")|.digest")
  [ "$w1" = "$w2" ] || { echo "API READS DIFFER $asset $w1 $w2"; exit 1; }
  [ -f $d/$asset ] || gh release download $tag -R wolffe-lang/$repo -p "$asset" -D $d || exit 1
  local h1 h2
  h1=$(sha256sum $d/$asset | cut -d' ' -f1); h2=$(sha256sum $d/$asset | cut -d' ' -f1)
  [ "$h1" = "$h2" ] && [ "sha256:$h1" = "$w1" ] || { echo "MISMATCH $asset api=$w1 h1=$h1 h2=$h2"; exit 1; }
  echo "OK $tag $asset api=$w1 read1=$h1 read2=$h2 size=$(stat -c %s $d/$asset)"
  mkdir -p $d/unpacked; tar -xzf $d/$asset -C $d/unpacked
}
get wolf-lang v0.2.35 wolf-0.2.35-x86_64-unknown-linux-gnu.tar.gz
get wolf-interp v0.1.56 lupin-0.1.56-x86_64-unknown-linux-gnu.tar.gz
get wolf-lang v0.2.34 wolf-0.2.34-x86_64-unknown-linux-gnu.tar.gz
get wolf-interp v0.1.55 lupin-0.1.55-x86_64-unknown-linux-gnu.tar.gz
W=$A/v0.2.35/unpacked/wolf-0.2.35-x86_64-unknown-linux-gnu; LU=$A/v0.1.56/unpacked/lupin-0.1.56-x86_64-unknown-linux-gnu
for f in $W/wolf $W/libwolf_rt.a $LU/lupin $A/v0.2.34/unpacked/wolf-0.2.34-x86_64-unknown-linux-gnu/wolf $A/v0.1.55/unpacked/lupin-0.1.55-x86_64-unknown-linux-gnu/lupin; do sha256sum $f; done
$W/wolf --version; $LU/lupin --version
$A/v0.2.34/unpacked/wolf-0.2.34-x86_64-unknown-linux-gnu/wolf --version | head -1; $A/v0.1.55/unpacked/lupin-0.1.55-x86_64-unknown-linux-gnu/lupin --version
echo "STD-REV $(cat $W/std/STD-REV)"
objdump -T $W/wolf 2>/dev/null | grep -o 'GLIBC_[0-9.]*' | sort -uV | tail -1
ls $W
echo ACQUIRE_DONE
