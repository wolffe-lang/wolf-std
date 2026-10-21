#!/usr/bin/env bash
# sc69 final-probes.sh: the 14 new rows and term/rows.lu on four machines at the head (30e719f) std, plain and under
# taskset -c 0-3, each in the throwaway container (probe.sh); then the summaries diffed. A summary that names fewer
# than the 15 rows is a harness fault, not a pass.
cd ~/lanes/sc69
R="process/fork_rows.lu term/restore_rows.lu term/mode_bits.lu term/mode_bits_set.lu term/rows.lu signal/wait_fds_rows.lu fs/sync_rows.lu fs/realpath_rows.lu fs/create_new_rows.lu time/monotonic_ns.lu process/comptime_refuses_fork.lu fs/comptime_refuses_sync.lu fs/comptime_refuses_sync_dir.lu fs/comptime_refuses_realpath.lu time/comptime_refuses_clock_ns.lu"
rm -rf probes/h-plain probes/h-taskset
bash scripts/probe.sh h-plain 0.2.34 ~/lanes/sc69/tree/final/std ~/lanes/sc69/tree/final/tests $R
PRE="taskset -c 0-3" bash scripts/probe.sh h-taskset 0.2.34 ~/lanes/sc69/tree/final/std ~/lanes/sc69/tree/final/tests $R
python3 -I scripts/summ.py probes/h-plain > evidence/probe-head-plain.txt
python3 -I scripts/summ.py probes/h-taskset > evidence/probe-head-taskset.txt
python3 -I scripts/summ.py probes/p6-red-trunk-std > evidence/probe-red-trunk-std.txt
for f in evidence/probe-head-plain.txt evidence/probe-head-taskset.txt evidence/probe-red-trunk-std.txt; do
  n=$(grep -c '^== ' $f); echo "$f rows=$n"; [ "$n" = 15 ] || echo "HARNESS_FAULT $f has $n rows, not 15"; done
cmp evidence/probe-head-plain.txt evidence/probe-head-taskset.txt && echo PLAIN_EQ_TASKSET
sha256sum evidence/probe-*.txt
echo FINAL_PROBES_DONE
