#!/bin/bash
# sc75 evidence by hand at a checkout, inside the caller's container (CPUs
# pinned by the caller): each part's output in its own log under OUT.
# usage: ev-inner.sh CHECKOUT OUT
S=$1; O=$2; mkdir -p $O
W=/lane/bin/wolf-0.2.36-x86_64-unknown-linux-gnu/wolf
LU=/lane/bin/lupin-0.1.58-x86_64-unknown-linux-gnu/lupin
part() { local name=$1; shift; echo "== $name $(date -u +%T)"; "$@" > $O/$name.log 2>&1; echo "END $name exit=$? $(date -u +%T)"; }
part rows-code bash /lane/witness/run-rows.sh $S/std $S/tests/strbuf/build_and_read.lu $S/tests/strbuf/build_bounded.lu $S/tests/strbuf/insert_remove_truncate.lu $S/tests/strbuf/push_char.lu $S/tests/strbuf/region_build_and_freeze.lu $S/tests/strbuf/truncate_split_codepoint_traps.lu $S/tests/str/to_strbuf_round_trip.lu $S/tests/env/arg0_rows.lu $S/tests/env/comptime_refuses_arg0.lu $S/tests/env/comptime_refuses_args_bytes.lu
part rows-plant bash /lane/witness/run-rows.sh /lane/plant-std $S/tests/strbuf/build_bounded.lu $S/tests/strbuf/build_and_read.lu
timed() {
  mkdir -p $O/timed && cp /lane/witness/timed-only/main.lu $O/timed/ && cd $O/timed || return 2
  for m in native release; do
    for which in trunk:/lane/wolf-std/std head:$S/std; do
      echo "== ${which%%:*} std, $m"
      timeout 300 $W conform-run main.lu --$m --std-root ${which#*:} --json; echo " rc=$?"
    done
  done
}
part timed-only timed
part argv env WOLF=$W LUPIN=$LU bash /lane/witness/argv/run.sh $S/std $O/argv-work
part table-after bash /lane/witness/run-table.sh after-head $S/std $O/table-work 300
echo EVIDENCE_INNER_DONE
