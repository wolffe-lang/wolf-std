#!/usr/bin/env bash
# sc67 mktree.sh <name> <rev>: trees/<name> = std/ and tests/ of <rev> (origin or the lane bundle), by git archive.
set -eu
L=~/lanes/sc67; test -d $L/run-g || git clone -q https://github.com/wolffe-lang/wolf-std.git $L/run-g
cd $L/run-g; git fetch -q origin '+refs/heads/trunk:refs/remotes/origin/trunk' '+refs/heads/sc67*:refs/remotes/k/sc67*' || true
if [ -f $L/sc67.bundle ]; then git fetch -q $L/sc67.bundle '+refs/heads/*:refs/remotes/b/*'; fi
rm -rf $L/trees/$1; mkdir -p $L/trees/$1; git archive $2 std tests | tar -x -C $L/trees/$1; echo "tree $1 = $(git rev-parse --short=7 $2)"
