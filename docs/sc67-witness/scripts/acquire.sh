#!/usr/bin/env bash
# sc67 acquire.sh: wolf 0.2.33 and lupin 0.1.54 (linux x86-64) by digest (release API), each archive read twice;
# the macOS arm64 digests recorded from the API (CI fetches those itself). Extracted under ~/lanes/sc67/bin.
set -euo pipefail
L=~/lanes/sc67; cd $L; mkdir -p dl bin
fetch() { local repo=$1 tag=$2 asset=$3
  want=$(gh release view $tag --repo $repo --json assets --jq ".assets[]|select(.name==\"$asset\")|.digest")
  test -n "$want" || { echo "NO DIGEST for $asset"; exit 1; }
  test -f dl/$asset || gh release download $tag --repo $repo --pattern $asset --dir dl
  got=$(sha256sum dl/$asset | cut -d' ' -f1)
  test -n "$got" && test "$got" = "${want#sha256:}" || { echo "MISMATCH $asset reported ${want#sha256:} got $got"; exit 1; }
  got2=$(sha256sum dl/$asset | cut -d' ' -f1); test -n "$got2" && test "$got2" = "$got" || { echo "REREAD MISMATCH $asset"; exit 1; }
  echo "$asset $(stat -c %s dl/$asset) B sha256 $got OK (read twice)"; }
date -u +START=%FT%TZ
gh api repos/wolffe-lang/wolf-lang/releases/tags/v0.2.33 --jq '"wolf release id \(.id) published \(.published_at) target \(.target_commitish)"'
gh api repos/wolffe-lang/wolf-interp/releases/tags/v0.1.54 --jq '"lupin release id \(.id) published \(.published_at) target \(.target_commitish)"'
for repo_tag_asset in "wolffe-lang/wolf-lang v0.2.33 wolf-0.2.33-aarch64-apple-darwin.tar.gz" "wolffe-lang/wolf-interp v0.1.54 lupin-0.1.54-aarch64-apple-darwin.tar.gz"; do
  set -- $repo_tag_asset
  echo "API digest $3: $(gh release view $2 --repo $1 --json assets --jq ".assets[]|select(.name==\"$3\")|.digest")"
done
fetch wolffe-lang/wolf-lang v0.2.33 wolf-0.2.33-x86_64-unknown-linux-gnu.tar.gz
fetch wolffe-lang/wolf-interp v0.1.54 lupin-0.1.54-x86_64-unknown-linux-gnu.tar.gz
rm -rf bin/wolf-0.2.33 bin/lupin-0.1.54; mkdir -p bin/wolf-0.2.33 bin/lupin-0.1.54
tar -xzf dl/wolf-0.2.33-x86_64-unknown-linux-gnu.tar.gz -C bin/wolf-0.2.33 --strip-components=1
tar -xzf dl/lupin-0.1.54-x86_64-unknown-linux-gnu.tar.gz -C bin/lupin-0.1.54 --strip-components=1
ls bin/wolf-0.2.33
sha256sum bin/wolf-0.2.33/wolf bin/wolf-0.2.33/libwolf_rt.a bin/lupin-0.1.54/lupin
sha256sum bin/wolf-0.2.33/wolf bin/lupin-0.1.54/lupin | cut -c1-64 > /tmp/sc67-acq-reread.$$
sha256sum bin/wolf-0.2.33/wolf bin/lupin-0.1.54/lupin | cut -c1-64 | cmp - /tmp/sc67-acq-reread.$$ && echo "binaries read twice OK"; rm -f /tmp/sc67-acq-reread.$$
bin/wolf-0.2.33/wolf --version; bin/lupin-0.1.54/lupin --version; echo "STD-REV $(cat bin/wolf-0.2.33/std/STD-REV)"
rm -f dl/*.tar.gz
date -u +END=%FT%TZ; echo ACQUIRE_DONE
