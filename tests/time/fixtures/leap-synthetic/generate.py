#!/usr/bin/env python3
"""RFC 8536 section 3, synthetic data only; never implementation source."""
from pathlib import Path
import struct


def block(size, version, off, leaps, transitions=(), types=None):
    if types is None:
        types = [(off, 0, 0)]
    chars = b'UTC\0EST\0EDT\0'
    header = b'TZif' + version + b'\0' * 15 + struct.pack('>6I', 0, 0, len(leaps),
                                                                   len(transitions), len(types), len(chars))
    signed = '>q' if size == 8 else '>i'
    body = b''.join(struct.pack(signed, t) for t, i in transitions)
    body += bytes(i for t, i in transitions)
    body += b''.join(struct.pack('>iBB', *t) for t in types) + chars
    body += b''.join(struct.pack(signed, t) + struct.pack('>i', c) for t, c in leaps)
    return header + body


def generate():
    out = Path(__file__).resolve().parent
    (out / 'negative-v1').write_bytes(block(4, b'\0', 0, [(78796799, -1)]))
    (out / 'positive-odd-v1').write_bytes(block(4, b'\0', 30, [(78796800, 1)]))
    prefix = block(4, b'3', 0, [])
    (out / 'negative-v3').write_bytes(prefix + block(8, b'3', 0, [(2208988799, -1)]) + b'\nUTC0\n')
    # One positive leap, then POSIX footer rules beyond a real transition.
    # July 1 2000 is EDT, so the footer agrees with the last transition.
    (out / 'footer-v3').write_bytes(prefix + block(8, b'3', 0, [(78796800, 1)],
        [(962409601, 2)], [(0, 0, 0), (-18000, 0, 4), (-14400, 1, 8)])
        + b'\nEST5EDT,M3.2.0,M11.1.0\n')


if __name__ == '__main__':
    generate()
