#!/usr/bin/env python3
"""Regenerate glibc.tsv using only RFC 8536 data and black-box date.

Run on kasumi under the lane's CPU flock. The Ubuntu image is content
addressed; tzdata is the byte-identical system copy beside this script.
No runtime/source dependency and no package installation.
"""
from pathlib import Path
import struct
import subprocess

IMAGE = '69f919497cada9a8611f3de61df902d9f94bf20f84552ef5bd93125c38608ef9'


def main():
    here = Path(__file__).resolve().parent
    data = (here / 'UTC').read_bytes()
    ut, st, leaps, times, types, chars = struct.unpack_from('>6I', data, 20)
    at = 44 + times * 5 + types * 6 + chars + leaps * 8 + st + ut
    ut, st, leaps, times, types, chars = struct.unpack_from('>6I', data, at + 20)
    start = at + 44 + times * 9 + types * 6 + chars
    records = [struct.unpack_from('>qi', data, start + i * 12) for i in range(leaps)]
    assert leaps == 27
    instants = [-1, 0, 63072000] + [t + d for t, _ in records for d in (-1, 0, 1)]
    instants += [1783000020, 1710054026, 1710054027, 1710054028]
    script = 'set -eu\n'
    for zone in ('right/UTC', 'right/America/New_York', 'right/Europe/London'):
        for t in instants:
            script += f"printf '{zone}\\t{t}\\t'; TZDIR=/zones TZ={zone} date -d @{t} +%F_%T_%Z_%z\n"
    result = subprocess.run(['podman', 'run', '--rm', '--network', 'none', '-i',
        '-v', str(here.parent) + ':/zones:ro', IMAGE, 'sh'],
        input=script, capture_output=True, text=True, check=True, timeout=120)
    if result.stderr or len(result.stdout.splitlines()) != 264:
        raise RuntimeError('oracle output count/stderr mismatch: ' + result.stderr)
    (here / 'glibc.tsv').write_text(result.stdout)
    print('right-zone oracle: 264 rows, 0 skips')


if __name__ == '__main__':
    main()
