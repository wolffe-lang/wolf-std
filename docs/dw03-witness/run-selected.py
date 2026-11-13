#!/usr/bin/env python3
"""Run selected committed rows, checking protocol verdicts, warnings and goldens."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys

p = argparse.ArgumentParser()
p.add_argument('repo', type=Path)
p.add_argument('out', type=Path)
p.add_argument('--wolf', required=True)
p.add_argument('--lupin')
p.add_argument('--tiers', default='checked,native,lupin')
p.add_argument('--all-selected', action='store_true')
p.add_argument('rows', nargs='*')
a = p.parse_intermixed_args()
a.out.mkdir(parents=True, exist_ok=True)
rows = a.rows or ['bytes/fixed_work.lu', 'base64/raw_strict.lu', 'bech32/bip173.lu', 'bech32/age_keys.lu', 'os/random/lengths.lu', 'os/random/negative_trap.lu']
if a.all_selected:
    rows = [str(f.relative_to(a.repo/'tests')) for family in ['bytes', 'base64', 'bech32', 'os/random', 'x/crypto/sha2', 'x/crypto/chacha20', 'x/crypto/curve25519'] for f in sorted((a.repo/'tests'/family).glob('*.lu')) if 'ed25519' not in f.name and 'non_byte' not in f.name and 'use_after' not in f.name and '512' not in f.name and '384' not in f.name]
import tomllib
ledger = tomllib.loads((a.repo/'tests/ledger.toml').read_text())['tests']
failed = False
for row in rows:
    text = (a.repo/'tests'/row).read_text()
    want = re.search(r'check: run\(exit=(trap\([a-z_-]+\)|[0-9]+)', text)
    if not want:
        print('FAIL unsupported-test-directive '+row, flush=True)
        failed = True
        continue
    expected = want[1] if want[1].startswith('trap(') else 'exit('+want[1]+')'
    golden = re.search(r'stdout=("(?:[^"\\]|\\.)*")', text)
    for tier in a.tiers.split(','):
        case = a.out / row.replace('/', '_').replace('.lu','') / tier
        case.mkdir(parents=True, exist_ok=True)
        staged = case/'main.lu'
        staged.write_text(text)
        cmd = [a.lupin if tier=='lupin' else a.wolf, 'conform-run', str(staged), '--std-root', str(a.repo/'std'), '--json']
        if tier == 'release': cmd += ['--native', '--release']
        elif tier != 'lupin': cmd.append('--'+tier)
        env = os.environ.copy()
        env['TMPDIR'] = str(case)
        try:
            result = subprocess.run(cmd, cwd=case, env=env, capture_output=True, text=True, timeout=180)
            (case/'stdout.json').write_text(result.stdout)
            (case/'stderr.txt').write_text(result.stderr)
            record = json.loads(result.stdout)
            posture = ledger.get(row, {}).get('wolfc' if tier=='checked' else 'native' if tier=='release' else tier, 'run')
            want_verdict = 'unsupported' if posture=='unsupported' else expected
            ok = result.returncode==0 and record['verdict']==want_verdict and not record.get('warnings')
            if golden and want_verdict.startswith('exit('):
                # Match xtask/record.rs: exact bytes, optionally ONE added newline.
                want_stdout = json.loads(golden[1])
                actual_hash = record.get('stdout_sha256') or hashlib.sha256(b'').hexdigest()
                allowed = {hashlib.sha256(s.encode()).hexdigest() for s in [want_stdout, want_stdout+'\n']}
                ok &= actual_hash in allowed
            detail = record['verdict']
            if posture == 'unsupported': detail += ' (not measured: ledger budget posture)'
            if not ok: detail += ' expected='+want_verdict+' warnings='+str(record.get('warnings'))
        except (subprocess.TimeoutExpired, ValueError, TypeError) as e:
            ok, detail = False, type(e).__name__
        print(('PASS ' if ok else 'FAIL ')+row+'/'+tier+' '+detail, flush=True)
        failed |= not ok
sys.exit(bool(failed))
