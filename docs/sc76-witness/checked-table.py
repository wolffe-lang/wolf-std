#!/usr/bin/env python3
"""Observe checked long-stream budgets, not a substitute for native RSS."""
import json
from pathlib import Path
import re
import subprocess
import sys
import time

root = Path(sys.argv[1])
for family in ('256', '512'):
    for mode in ('update', 'sum'):
        for mib in (16, 64, 1024):
            args = ['rss', str(mib * 1024**2), family, mode]
            cmd = ['prlimit', '--as=2147483648', '--', '/usr/bin/time', '-v',
                   '/home/mfwolffe/lanes/sc76/bin/wolf', 'conform-run',
                   str(root / 'docs/sc76-witness/rss/main.lu'), '--checked', '--json',
                   '--std-root', str(root / 'std')]
            cmd += ['--argv-hex=' + s.encode().hex() for s in args]
            t0 = time.monotonic()
            p = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            while True:
                try:
                    out, err = p.communicate(timeout=30)
                    break
                except subprocess.TimeoutExpired:
                    print(f'WAIT checked {family} {mode} {mib} pid={p.pid}', flush=True)
            match = re.search(rb'Maximum resident set size \(kbytes\):\s*(\d+)', err)
            try:
                record = json.loads(out)
            except ValueError:
                record = None
            print(json.dumps(dict(family=family, mode=mode, mib=mib,
                                  tier='checked', peak_kib=int(match[1]) if match else None,
                                  wall_s=round(time.monotonic()-t0, 3), exit=p.returncode,
                                  record=record, stderr=err.decode(errors='replace') if record is None else None)), flush=True)
            if record is not None:
                assert p.returncode == 0 and not record['warnings']
                assert record['verdict'] in ('exit(0)', 'unsupported'), record
            else:
                # A large checked caller-owned list can hit the process cap
                # before JSON is emitted. Preserve it as failed observation,
                # never as passing bound evidence.
                assert p.returncode != 0, (out, err)
