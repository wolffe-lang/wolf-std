#!/usr/bin/env python3
"""Paired reference insert scaling; children capped to 3 GiB and 60 seconds."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import time

p = argparse.ArgumentParser()
p.add_argument('--lupin', required=True)
p.add_argument('--repo', type=Path, required=True)
p.add_argument('--output', type=Path, required=True)
a = p.parse_args()
a.output.mkdir(parents=True, exist_ok=False)
program = (a.repo/'tests/map/get_many.lu').read_text()
insert = program[:program.index('    var hits = 0')] + '    print("{m.len}")\n    0\n}\n'
rows = []
for n in [1000, 10000]:
    stage = a.output/str(n)
    stage.mkdir()
    entry = stage/'main.lu'
    entry.write_text(insert.replace('10000', str(n)))
    start = time.monotonic()
    with (stage/'stdout.json').open('wb') as so, (stage/'stderr.txt').open('wb') as se:
        child = subprocess.Popen(['timeout', '--foreground', '--signal=TERM',
            '--kill-after=5', '60', 'prlimit', '--as=3221225472', '--core=0', '--',
            a.lupin, 'conform-run', '--json', '--std-root', str(a.repo/'std'), str(entry)],
            stdout=so, stderr=se)
        _, status, usage = os.wait4(child.pid, 0)
        child.returncode = os.waitstatus_to_exitcode(status)
    row = {'n': n, 'exit': child.returncode, 'seconds': time.monotonic()-start,
           'rss_kib': usage.ru_maxrss, 'stdout': (stage/'stdout.json').read_text(),
           'stderr': (stage/'stderr.txt').read_text()}
    rows.append(row)
    (a.output/'results.json').write_text(json.dumps(rows, indent=2)+'\n')
    print(json.dumps(row), flush=True)
    assert row['exit'] == 0, row
    record = json.loads(row['stdout'])
    assert record['verdict']=='exit(0)' and record['stdout_inline']==str(n)+'\n', row
print('PASS lupin-insert-pair')
