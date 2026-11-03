#!/usr/bin/env python3
"""Linux RSS bound: cap every child, verify digests, print every measurement."""
import hashlib
import json
import re
import subprocess
import sys
import time


def main():
    binary = sys.argv[1]
    failures = []
    for family in ('256', '512'):
        for mode in ('update', 'sum'):
            rss = {}
            for mib in (16, 64, 1024):
                size = mib * 1024 * 1024
                cmd = ['prlimit', '--as=2147483648', '--', '/usr/bin/time', '-v',
                       binary, str(size), family, mode]
                started = time.monotonic()
                p = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
                while True:
                    try:
                        out, err = p.communicate(timeout=30)
                        break
                    except subprocess.TimeoutExpired:
                        print(f'WAIT family={family} mode={mode} mib={mib} pid={p.pid}', flush=True)
                match = re.search(rb'Maximum resident set size \(kbytes\):\s*(\d+)', err)
                peak = int(match[1]) if match else None
                ok = p.returncode == 0
                if ok:
                    h = hashlib.new('sha' + family)
                    chunk = bytes(65536)
                    for _ in range(size // len(chunk)):
                        h.update(chunk)
                    ok = out.decode().strip() == h.hexdigest()
                print(json.dumps(dict(family=family, mode=mode, mib=mib, peak_kib=peak,
                                      wall_s=round(time.monotonic()-started, 3),
                                      exit=p.returncode, digest_ok=ok)), flush=True)
                if not ok:
                    print(err.decode(errors='replace'), flush=True)
                    failures.append(f'{family}-{mode}-{mib}')
                else:
                    rss[mib] = peak
            # sum necessarily holds caller input: report its raw RSS but do not
            # pretend its 960 MiB input growth is hash auxiliary storage.
            if mode == 'update':
                ok = 64 in rss and 1024 in rss and rss[1024] - rss[64] < 8192
                print(f'END SHA{family}-RSS-64MiB-1GiB exit={0 if ok else 1}', flush=True)
                if not ok:
                    failures.append(f'{family}-RSS-bound')
    print('FAILURES ' + repr(failures), flush=True)
    return bool(failures)


if __name__ == '__main__':
    sys.exit(main())
