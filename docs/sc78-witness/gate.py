#!/usr/bin/env python3
"""Module-only witnesses; each process has a hard Linux address-space cap."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import statistics
import subprocess
import sys
import time

p = argparse.ArgumentParser()
p.add_argument('mode', choices=['maps', 'ratio', 'unchanged', 'lupin-small'])
p.add_argument('--repo', type=Path, required=True)
p.add_argument('--wolf', required=True)
p.add_argument('--lupin')
p.add_argument('--scratch', type=Path, required=True)
p.add_argument('--base', default='b1babae3e61458620b921a27ccd267b67dd4d52f')
a = p.parse_args()
a.scratch.mkdir(parents=True, exist_ok=False)

def run(cmd):
    return subprocess.run(['timeout', '--foreground', '--signal=TERM',
                           '--kill-after=5', '120', 'prlimit',
                           '--as=3221225472', '--core=0', '--', *map(str, cmd)],
                          capture_output=True, text=True)

def record_ok(got, expected):
    if got.returncode:
        return False
    try:
        record = json.loads(got.stdout)
    except ValueError:
        return False
    digest = hashlib.sha256(expected.encode()).hexdigest()
    return (record.get('verdict') == 'exit(0)'
            and not record.get('warnings')
            and (record.get('stdout_inline') or '') == expected
            and (not expected or record.get('stdout_sha256') == digest))

bad = False
if a.mode == 'maps':
    for src in sorted((a.repo / 'tests/map').glob('*.lu')):
        stage = a.scratch / src.stem
        stage.mkdir()
        entry = stage / src.name
        shutil.copyfile(src, entry)
        for machine, flag in [('native', '--native'), ('release', '--release'),
                              ('checked', '--checked')]:
            row = ('get-10000' if src.stem == 'get_many' else src.stem) + '-' + machine
            got = run([a.wolf, 'conform-run', '--json', flag, '--std-root',
                       a.repo / 'std', entry])
            print(f'row {row} tool_exit={got.returncode}')
            print(got.stdout.rstrip())
            print(got.stderr.rstrip())
            # Existing map rows have no stdout, except has's two-line ledger.
            expected = ''
            if src.stem == 'has_allocates_nothing':
                expected = ('hit=true miss=false charged_zero=true\n'
                            'lists hit=true miss=false\n')
            elif src.stem == 'get_many':
                expected = '10000\n'
            elif src.stem == 'get_many_smoke':
                expected = '1000\n'
            ok = record_ok(got, expected)
            print(('PASS ' if ok else 'FAIL ') + row)
            bad |= not ok
elif a.mode == 'lupin-small':
    import tomllib
    if not a.lupin:
        p.error('lupin-small requires --lupin')
    ledger = tomllib.loads((a.repo / 'tests/ledger.toml').read_text())['tests']
    reason = '`Eq.eq` does not resolve to an `impl Eq for …` method of char'
    for name in ['get_int_agreement', 'get_char_agreement', 'get_bool_agreement',
                 'get_str_agreement', 'get_list_independent', 'get_str_independent',
                 'get_many_smoke']:
        filename = name + '.lu'
        stage = a.scratch / name
        stage.mkdir()
        entry = stage / filename
        shutil.copyfile(a.repo / 'tests/map' / filename, entry)
        got = run([a.lupin, 'conform-run', '--json', '--std-root',
                   a.repo / 'std', entry])
        print(f'row {name}-lupin tool_exit={got.returncode}')
        print(got.stdout.rstrip())
        print(got.stderr.rstrip())
        want = ledger['map/' + filename]['lupin']
        expected = '1000\n' if name == 'get_many_smoke' else ''
        ok = want == 'run' and record_ok(got, expected)
        if name == 'get_char_agreement' and want == 'unsupported':
            try:
                record = json.loads(got.stdout)
            except ValueError:
                record = {}
            ok = (got.returncode == 0 and record.get('verdict') == 'unsupported'
                  and not record.get('warnings')
                  and record.get('x-unsupported') == reason)
            print('expected char refusal: ' + reason)
        print(('PASS ' if ok else 'FAIL ') + name + '-lupin')
        bad |= not ok
elif a.mode == 'ratio':
    # Same insert shape in both programs; only the lookup differs.
    src = (a.repo / 'tests/map/get_many.lu').read_text()
    binaries = {}
    for variant in ['get', 'index']:
        stage = a.scratch / variant
        stage.mkdir()
        source = src
        if variant == 'index':
            source = source.replace('map.get(m, "0#key{i}")', 'm["0#key{i}"]')
            if source == src:
                sys.exit('FAIL get-10000-native-ratio: lookup needle missing')
        entry = stage / 'main.lu'
        entry.write_text(source)
        binary = stage / 'main'
        got = run([a.wolf, 'build', '--std-root', a.repo / 'std', entry,
                   '-o', binary])
        if got.returncode:
            print(got.stdout, got.stderr)
            sys.exit('FAIL get-10000-native-ratio: build refused')
        binaries[variant] = binary
    times = {'get': [], 'index': []}
    # Interleave and alternate first position; compile time is excluded.
    for sample in range(5):
        order = ['get', 'index'] if sample % 2 == 0 else ['index', 'get']
        for variant in order:
            started = time.monotonic()
            got = run([binaries[variant]])
            elapsed = time.monotonic() - started
            if got.returncode or got.stdout != '10000\n':
                print(got.stdout, got.stderr)
                sys.exit('FAIL get-10000-native-ratio: wrong answer or process exit')
            times[variant].append(elapsed)
    ratio = statistics.median(times['get']) / statistics.median(times['index'])
    print(json.dumps({'seconds': times, 'median_get_over_index': ratio,
                      'bound': 2.0}, sort_keys=True))
    bad = ratio > 2.0
    print(('FAIL ' if bad else 'PASS ') + 'get-10000-native-ratio')
else:
    import re
    path = 'std/map/map.lu'
    before = subprocess.check_output(['git', '-C', str(a.repo), 'show',
                                      a.base + ':' + path], text=True)
    after = (a.repo / path).read_text()
    pattern = r'^pub fn (\w+).*?^}'
    old = {m.group(1): m.group(0) for m in re.finditer(pattern, before, re.M | re.S)}
    new = {m.group(1): m.group(0) for m in re.finditer(pattern, after, re.M | re.S)}
    bad = old.keys() != new.keys()
    same_signature = old['get'].partition('\n')[0] == new.get('get', '').partition('\n')[0]
    print(('PASS ' if same_signature else 'FAIL ') + 'unchanged-get-signature')
    bad |= not same_signature
    for name in old:
        if name == 'get':
            continue
        ok = old[name] == new.get(name)
        print(('PASS ' if ok else 'FAIL ') + 'unchanged-' + name)
        bad |= not ok
    changed = subprocess.check_output(['git', '-C', str(a.repo), 'diff',
                                      '--name-only', a.base, '--', 'std/'], text=True).splitlines()
    ok = changed == [path]
    print(('PASS ' if ok else 'FAIL ') + 'only-map-changed')
    bad |= not ok
sys.exit(1 if bad else 0)
