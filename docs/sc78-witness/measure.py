#!/usr/bin/env python3
"""Same-process elapsed and wait4 RSS; every child under Linux AS/time caps."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import time

p = argparse.ArgumentParser()
p.add_argument('--wolf', required=True)
p.add_argument('--std', required=True)
p.add_argument('--witness', required=True)
p.add_argument('--output', required=True)
p.add_argument('--repeats', type=int, default=1)
a = p.parse_args()
out = Path(a.output)
out.mkdir(parents=True, exist_ok=False)
source = Path(a.witness).read_text()
rows = []

def run(cmd, label):
    stdout = out / (label + '.out')
    stderr = out / (label + '.err')
    # prlimit constrains the child (not the shell or other lane jobs).
    cmd = ['timeout', '--foreground', '--signal=TERM', '--kill-after=5', '120',
           'prlimit', '--as=3221225472', '--core=0', '--', *cmd]
    start = time.monotonic()
    with stdout.open('wb') as so, stderr.open('wb') as se:
        child = subprocess.Popen(cmd, stdout=so, stderr=se)
        _, status, usage = os.wait4(child.pid, 0)
        child.returncode = os.waitstatus_to_exitcode(status)
    row = {'label': label, 'exit': child.returncode,
           'seconds': round(time.monotonic() - start, 6),
           'rss_kib': usage.ru_maxrss,
           'stdout': stdout.read_text(errors='replace').strip(),
           'stderr': stderr.read_text(errors='replace').strip()}
    print(json.dumps(row), flush=True)
    return row

for n in [10000, 100000]:
    for variant in ['get', 'index']:
        d = out / (variant + '-' + str(n))
        d.mkdir()
        src = source.replace('10000', str(n))
        if variant == 'index':
            src = src.replace('map.get(m, "0#key{i}")', 'm["0#key{i}"]')
        path = d / 'main.lu'
        path.write_text(src)
        for machine in ['native', 'release', 'checked']:
            label = f'{variant}-{n}-{machine}'
            if machine == 'checked':
                cmd = [a.wolf, 'conform-run', '--json', '--checked',
                       '--std-root', a.std, str(path)]
            else:
                binary = d / machine
                flags = ['--release'] if machine == 'release' else []
                build = run([a.wolf, 'build', *flags, '--std-root', a.std,
                             str(path), '-o', str(binary)], label + '-build')
                if build['exit']:
                    rows.append(build)
                    continue
                cmd = [str(binary)]
            for rep in range(a.repeats):
                row = run(cmd, label + '-' + str(rep))
                row.update(n=n, variant=variant, machine=machine)
                rows.append(row)
                (out / 'results.json').write_text(json.dumps(rows, indent=2) + '\n')
