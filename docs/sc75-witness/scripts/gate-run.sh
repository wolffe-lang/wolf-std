#!/bin/bash
# sc75 on hasu block B (CPUs 6-11), both lock names, block name outside:
# fetch REF into a fresh copy, run gate/inner.sh in r41-toolchain:1 with a
# foreign init (--init) and its own session (setsid --fork --wait), 12g no
# swap. usage: run.sh LABEL REF   (launch with setsid -f; pid, log, status)
L=/home/mfwolffe/lanes/sc75; LABEL=$1; REF=$2
G=$L/logs/gate-$LABEL; mkdir -p $G
echo $$ > $G/pid
export PATH=$HOME/lanes/hasu-env/bin:$HOME/lanes/hasu-env/profile/bin:$PATH
{
  cd $L/wolf-std && git fetch -q origin && git fetch -q origin sc75 2>/dev/null
  rm -rf $L/g/$LABEL; mkdir -p $L/g
  git clone -q $L/wolf-std $L/g/$LABEL && cd $L/g/$LABEL && git checkout -q "$REF" || { echo "checkout of $REF failed"; exit 2; }
  while :; do
    flock -E 75 -w 240 /home/mfwolffe/lanes/hasu-block-B.lock flock -E 75 -w 240 /home/mfwolffe/lanes/hasu-B.lock \
      podman run --rm --init --memory 12g --memory-swap 12g --ulimit core=0:0 \
        --pids-limit 16384 --ulimit nproc=32768:32768 \
        -v $L:/lane -w /lane localhost/r41-toolchain:1 \
        /usr/bin/taskset.real -c 6-11 setsid --fork --wait bash /lane/gate/final-inner.sh /lane/g/$LABEL /lane/logs/gate-$LABEL
    rc=$?
    [ $rc = 75 ] && { echo "$(date +%T) block B busy, waiting again"; continue; }
    echo "EXIT $rc $(date -u +%FT%TZ)"; break
  done
} > $G/log 2>&1
echo done > $G/status
