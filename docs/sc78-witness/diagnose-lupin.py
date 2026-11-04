#!/usr/bin/env python3
"""Read-only Lupin diagnostics from immutable std/test snapshots."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import time

p = argparse.ArgumentParser()
p.add_argument('--base', type=Path, required=True)
p.add_argument('--head', type=Path, required=True)
p.add_argument('--lupin', required=True)
p.add_argument('--output', type=Path, required=True)
a = p.parse_args()
a.output.mkdir(parents=True, exist_ok=False)
results = []

def run(label, source, std, seconds=60):
    stage = a.output / label
    stage.mkdir()
    entry = stage / 'main.lu'
    entry.write_text(source)
    cmd = ['timeout', '--foreground', '--signal=TERM', '--kill-after=5',
           str(seconds), a.lupin, 'conform-run', '--json', '--std-root',
           str(std), str(entry)]
    start = time.monotonic()
    with (stage/'stdout.json').open('wb') as so, (stage/'stderr.txt').open('wb') as se:
        child = subprocess.Popen(cmd, stdout=so, stderr=se)
        _, status, usage = os.wait4(child.pid, 0)
        child.returncode = os.waitstatus_to_exitcode(status)
    row = {'label': label, 'exit': child.returncode,
           'seconds': round(time.monotonic()-start, 6),
           'rss_kib': usage.ru_maxrss,
           'stdout': (stage/'stdout.json').read_text(errors='replace').strip(),
           'stderr': (stage/'stderr.txt').read_text(errors='replace').strip()}
    try:
        obs = json.loads(row['stdout'])
        row['verdict'] = obs.get('verdict')
        row['unsupported'] = obs.get('x-unsupported') or obs.get('x-unsupported-construct')
    except ValueError:
        pass
    results.append(row)
    (a.output/'results.json').write_text(json.dumps(results, indent=2)+'\n')
    print(json.dumps(row), flush=True)

print('Diagnostic memory cap: enclosing container MemoryMax=12 GiB; child wall cap=60 seconds.', flush=True)
for label, repo in [('base', a.base), ('head', a.head)]:
    sha = subprocess.check_output(['git','-c','commit.gpgsign=false','-C',str(repo),'rev-parse','HEAD'],text=True).strip()
    print(label+' '+sha, flush=True)
print(subprocess.check_output([a.lupin,'--version'],text=True).strip(), flush=True)
agreement = (a.head/'tests/map/get_index_agreement.lu').read_text()
run('agreement-full-head', agreement, a.head/'std')
helpers = agreement[:agreement.index('fn main()')]
for name in ['int_keys', 'char_keys', 'bool_keys', 'str_keys',
             'list_value_independent', 'str_value_independent']:
    source = helpers + '\nfn main() -> !int {\n    if '+name+'() { return 0 }\n    return 1\n}\n'
    run('agreement-'+name, source, a.head/'std')
program = (a.head/'tests/map/get_many.lu').read_text()
for n in [1000, 10000]:
    source = program.replace('10000', str(n))
    for rep in range(2):
        order = [('base', a.base), ('head', a.head)] if rep == 0 else [('head', a.head), ('base', a.base)]
        for label, repo in order:
            run(f'get-{n}-{label}-{rep}', source, repo/'std')
index = program.replace('map.get(m, "0#key{i}")', 'm["0#key{i}"]')
assert index != program
run('index-10000-head', index, a.head/'std')
setonly = program[:program.index('    var hits = 0')] + '    print("{m.len}")\n    0\n}\n'
run('setonly-10000-head', setonly, a.head/'std')
