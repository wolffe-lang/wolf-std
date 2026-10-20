#!/usr/bin/env bash
# sc67 launch.sh <name> <script> [args...]: start a job detached (setsid + nohup, default dispositions),
# pid in ~/lanes/sc67/<name>.pid, output in ~/lanes/sc67/job-<name>.log.
set -eu
L=~/lanes/sc67; N=$1; shift
( setsid nohup env --default-signal=INT,QUIT,HUP,TERM,PIPE bash "$@" > $L/job-$N.log 2>&1 < /dev/null & echo $! > $L/$N.pid )
sleep 1; echo "launched $N pid $(cat $L/$N.pid)"; head -2 $L/job-$N.log
