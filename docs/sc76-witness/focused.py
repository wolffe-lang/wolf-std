#!/usr/bin/env python3
"""Small pure SHA-2 rows, with real record and warning checks."""
import json
import os
from pathlib import Path
import subprocess
import sys

root = Path(sys.argv[1])
wolf = Path('/home/mfwolffe/lanes/sc76/bin/wolf')
lupin = wolf.with_name('lupin')
rows = ['sha2/bounded256', 'sha2/bounded512', 'sha2/known_answers',
        'sha2/incremental_chunking', 'sha2/rfc4231_hmac', 'sha2/rfc5869_hkdf',
        'curve25519/ed25519_smoke']
failed = False
for row in rows:
    for tier in ('native', 'release', 'checked', 'lupin'):
        cmd = [str(lupin if tier == 'lupin' else wolf), 'conform-run',
               str(root / 'tests/x/crypto' / (row + '.lu')),
               '--std-root', str(root / 'std'), '--json']
        if tier != 'lupin':
            cmd += ['--' + tier]
        p = subprocess.Popen(['prlimit', '--as=2147483648', '--', *cmd],
                             stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        while True:
            try:
                out, err = p.communicate(timeout=30)
                break
            except subprocess.TimeoutExpired:
                print(f'WAIT {row} {tier} pid={p.pid}', flush=True)
        print(f'ROW {row} {tier} process-exit={p.returncode}', flush=True)
        print(out.decode(), flush=True)
        if err:
            print(err.decode(), flush=True)
        try:
            record = json.loads(out)
            ok = p.returncode == 0 and record['verdict'] == 'exit(0)' and not record['warnings']
        except (ValueError, KeyError):
            ok = False
        print(f'END {row}-{tier} exit={0 if ok else 1}', flush=True)
        failed |= not ok
sys.exit(failed)
