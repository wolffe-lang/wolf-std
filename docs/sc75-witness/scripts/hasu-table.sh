#!/bin/bash
# sc75 on hasu, block B (CPUs 6-11), both lock names held, the block's name outside.
# usage: hasu-table.sh LABEL STD_SUBDIR TIMEOUT   (STD_SUBDIR relative to /home/mfwolffe/lanes/sc75)
L=/home/mfwolffe/lanes/sc75; LABEL=$1; STD=$2; TO=$3
LOG=$L/logs/table-$LABEL.log
echo $$ > $L/logs/table-$LABEL.pid
export PATH=$HOME/lanes/hasu-env/bin:$HOME/lanes/hasu-env/profile/bin:$PATH
{
  until flock -n /home/mfwolffe/lanes/hasu-block-B.lock true; do echo "$(date +%T) waiting on hasu-block-B.lock"; sleep 60; done
  flock /home/mfwolffe/lanes/hasu-block-B.lock flock /home/mfwolffe/lanes/hasu-B.lock \
    podman run --rm --network none --memory 12g --memory-swap 12g --ulimit core=0:0 \
      -v $L:/lane -w /lane localhost/px75-ubuntu:2 \
      taskset -c 6-11 bash /lane/witness/run-table.sh "$LABEL" "/lane/$STD" "/lane/out/$LABEL" "$TO"
  echo "EXIT $? $(date -u +%FT%TZ)"
} > $LOG 2>&1
echo done > $L/logs/table-$LABEL.status
