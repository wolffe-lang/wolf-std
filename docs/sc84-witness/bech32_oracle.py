#!/usr/bin/env python3
"""Expected strings for tests/bech32/alphabet_and_widths.lu.

Written from BIP-173's specification text alone: its character table, its
checksum definition (the generator constants, the HRP expansion, the six
zero words and the final XOR with 1), and its 8-to-5 regrouping with zero
padding. It shares no code with std.bech32. Run with `python3 -I`; it
prints the wolf literals the row pins.
"""

CHARSET = "qpzry9x8gf2tvdw0s3jn54khce6mua7l"
GEN = [0x3B6A57B2, 0x26508E6D, 0x1EA119FA, 0x3D4233DD, 0x2A1462B3]


def polymod(values):
    chk = 1
    for v in values:
        top = chk >> 25
        chk = (chk & 0x1FFFFFF) << 5 ^ v
        for i in range(5):
            if (top >> i) & 1:
                chk ^= GEN[i]
    return chk


def hrp_expand(hrp):
    return [ord(c) >> 5 for c in hrp] + [0] + [ord(c) & 31 for c in hrp]


def encode(hrp, words):
    pm = polymod(hrp_expand(hrp) + list(words) + [0] * 6) ^ 1
    check = [(pm >> 5 * (5 - i)) & 31 for i in range(6)]
    return hrp + "1" + "".join(CHARSET[w] for w in list(words) + check)


def regroup(data):
    acc, bits, out = 0, 0, []
    for b in data:
        acc = (acc << 8) | b
        bits += 8
        while bits >= 5:
            bits -= 5
            out.append((acc >> bits) & 31)
    if bits:
        out.append((acc << (5 - bits)) & 31)
    return out


def pattern(n):
    return [(i * 37 + n) % 256 for i in range(n)]


def main():
    print("// every word value in order, HRP a")
    print('"%s"' % encode("a", range(32)))
    for n in (0, 1, 2, 3, 4, 5, 31, 32, 33, 51):
        print("// %d pattern bytes, HRP a" % n)
        print('"%s"' % encode("a", regroup(pattern(n))))
    for n in (1, 32):
        print("// %d bytes of 0xff, HRP age" % n)
        print('"%s"' % encode("age", regroup([255] * n)))


if __name__ == "__main__":
    main()
