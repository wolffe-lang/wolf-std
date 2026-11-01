#!/bin/bash
# sc75 on hasu block B, both locks: run a command line inside px75-ubuntu:2 pinned to 6-11, 12g no swap.
L=/home/mfwolffe/lanes/sc75; TAG=$1; shift
export PATH=$HOME/lanes/hasu-env/bin:$HOME/lanes/hasu-env/profile/bin:$PATH
until flock -n /home/mfwolffe/lanes/hasu-block-B.lock true; do echo "$(date +%T) waiting on hasu-block-B.lock"; sleep 60; done
flock /home/mfwolffe/lanes/hasu-block-B.lock flock /home/mfwolffe/lanes/hasu-B.lock \
  podman run --rm --network none --memory 12g --memory-swap 12g --ulimit core=0:0 \
    -v $L:/lane -w /lane localhost/px75-ubuntu:2 taskset -c 6-11 bash -c "$*"
echo "EXIT $? $TAG $(date -u +%FT%TZ)"
