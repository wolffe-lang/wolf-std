#!/usr/bin/env python3
"""Black-box wcwidth census in the recorded glibc 2.39 / C.UTF-8 image.

No implementation source is read. The portable Wolf fixtures enumerate the
same assigned scalars and expected answers on each of the three machines.
"""
import ctypes
import hashlib
import json
import locale
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]


def main():
    data = json.loads((ROOT / 'vendor/unicode/derived.json').read_text())
    libc = ctypes.CDLL(None)
    libc.gnu_get_libc_version.restype = ctypes.c_char_p
    version = libc.gnu_get_libc_version().decode('ascii')
    if version != '2.39':
        raise ValueError('oracle needs recorded glibc 2.39 image, got ' + version)
    selected = locale.setlocale(locale.LC_CTYPE, 'C.UTF-8')
    libc.wcwidth.argtypes = [ctypes.c_wchar]
    libc.wcwidth.restype = ctypes.c_int
    expected = [-1] * 0x110000
    for low, high, width in data['width_ranges']:
        expected[low:high + 1] = [width] * (high - low + 1)
    named = {int(row['scalar'][2:], 16): row for row in data['exceptions']}
    seen, mismatches, base_differences = 0, [], []
    vector = bytearray()
    for low, high, assigned in data['assigned_ranges']:
        if not assigned:
            raise ValueError('false assigned range')
        for cp in range(low, high + 1):
            got = libc.wcwidth(chr(cp))
            vector.append(got + 1)
            want = expected[cp]
            if got != want:
                mismatches.append(('U+%04X' % cp, want, got))
            base = named[cp]['base'] if cp in named else want
            if got != base:
                base_differences.append(cp)
            seen += 1
    if seen != 287346 or seen != data['assigned_scalars']:
        raise ValueError('assigned scalar count ' + str(seen))
    if mismatches:
        raise ValueError('wcwidth disagreements: ' + repr(mismatches[:30]) + ' total=' + str(len(mismatches)))
    if set(base_differences) != set(named) or len(named) != 350:
        raise ValueError('350 exception census differs')
    print('PASS width-c-oracle assigned=%d exceptions=%d libc=%s locale=%s vector-sha256=%s' %
          (seen, len(named), version, selected, hashlib.sha256(vector).hexdigest()))


if __name__ == '__main__':
    try:
        main()
    except Exception as error:
        print('FAIL width-c-oracle ' + str(error), flush=True)
        sys.exit(1)
