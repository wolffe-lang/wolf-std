#!/usr/bin/env python3
"""p384_check.py -- an independent ECDSA P-384 verifier, from the standards alone.

sc79's checker for the vendored Wycheproof P-384 files. It shares no code
with std.x.crypto.p384 and no library's elliptic-curve code. The curve is
SP 800-186 section 3.2.1.4's (P-384, a = -3); verification is FIPS 186-5
section 6.4.2 (bits2int, r and s in [1, n), R = u1 G + u2 Q, v = x(R) mod n);
public-key validation is SEC 1 v2 section 3.2.2 (coordinates below p, the
point on the curve); signatures are parsed under X.690's DER rules. The
group law is the affine one over GF(p), each inverse Python's built-in
modular inverse (`pow(x, -1, m)`, integer arithmetic). Python's
hashlib is the one call with cryptographic meaning, the SHA-2 oracle.

Before any vector it checks its own constants: p's closed form, p and n
prime (Miller-Rabin over the first 40 primes as bases), G on the curve and
[n]G the identity.

For every case of every vendored file it computes a verdict and compares it
with the generator's rule (`valid` accepts, `invalid` rejects; these files
have no `acceptable`). A disagreement is an error naming file and tcId.

    python3 -I tests/x/crypto/p384/p384_check.py [--vectors DIR]
"""

import argparse
import hashlib
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[4]

FILES = [
    ("ecdsa_secp384r1_sha384_test.json", "der"),
    ("ecdsa_secp384r1_sha384_p1363_test.json", "p1363"),
    ("ecdsa_secp384r1_sha256_test.json", "der"),
    ("ecdsa_secp384r1_sha512_test.json", "der"),
]

HASHES = {"SHA-256": hashlib.sha256, "SHA-384": hashlib.sha384, "SHA-512": hashlib.sha512}

# SP 800-186 section 3.2.1.4.
P = 2**384 - 2**128 - 2**96 + 2**32 - 1
N = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFC7634D81F4372DDF581A0DB248B0A77AECEC196ACCC52973
A = P - 3
B = 0xB3312FA7E23EE7E4988E056BE3F82D19181D9C6EFE8141120314088F5013875AC656398D8A2ED19D2A85C8EDD3EC2AEF
GX = 0xAA87CA22BE8B05378EB1C71EF320AD746E1D3B628BA79B9859F741E082542A385502F25DBF55296C3A545E3872760AB7
GY = 0x3617DE4A96262C6F5D9E98BF9292DC29F8F41DBD289A147CE9DA3113B5F0B8C00A60B1CE1D7E819D7A431D7C90EA0E5F
G = (GX, GY)
O = None


def is_probable_prime(n):
    if n < 2:
        return False
    small = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
             73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151,
             157, 163, 167, 173]
    for q in small:
        if n % q == 0:
            return n == q
    d, r = n - 1, 0
    while d % 2 == 0:
        d //= 2
        r += 1
    for a in small:
        x = pow(a, d, n)
        if x in (1, n - 1):
            continue
        for _ in range(r - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True


def on_curve(pt):
    x, y = pt
    return (y * y - (x * x * x + A * x + B)) % P == 0


def add(p1, p2):
    if p1 is O:
        return p2
    if p2 is O:
        return p1
    x1, y1 = p1
    x2, y2 = p2
    if x1 == x2:
        if (y1 + y2) % P == 0:
            return O
        lam = (3 * x1 * x1 + A) * pow(2 * y1, -1, P) % P
    else:
        lam = (y2 - y1) * pow(x2 - x1, -1, P) % P
    x3 = (lam * lam - x1 - x2) % P
    return (x3, (lam * (x1 - x3) - y1) % P)


def mul(k, pt):
    acc = O
    for bit in bin(k)[2:]:
        acc = add(acc, acc)
        if bit == "1":
            acc = add(acc, pt)
    return acc


def self_check():
    # p's closed form against its published hexadecimal (SP 800-186).
    assert P == int("ffffffff" * 7 + "fffffffe" + "ffffffff" + "00000000" * 2 + "ffffffff", 16)
    assert is_probable_prime(P), "p is not prime"
    assert is_probable_prime(N), "n is not prime"
    assert on_curve(G), "G is not on the curve"
    assert mul(N, G) is O, "[n]G is not the identity"
    assert mul(N - 1, G) == (GX, (-GY) % P), "[n-1]G is not -G"


def decode_point(raw):
    """SEC 1 section 2.3.4 uncompressed form, validated per section 3.2.2."""
    if len(raw) != 97 or raw[0] != 4:
        return None
    x = int.from_bytes(raw[1:49], "big")
    y = int.from_bytes(raw[49:], "big")
    if x >= P or y >= P or not on_curve((x, y)):
        return None
    return (x, y)


def parse_der_signature(sig):
    """SEQUENCE { r INTEGER, s INTEGER } under DER: one definite minimal
    length per element, minimal non-negative INTEGERs, nothing trailing."""
    def tlv(buf, at):
        if at + 2 > len(buf):
            return None
        tag, first = buf[at], buf[at + 1]
        at += 2
        if first < 0x80:
            n = first
        else:
            k = first & 0x7F
            if k == 0 or k > 2 or at + k > len(buf) or buf[at] == 0:
                return None
            n = int.from_bytes(buf[at:at + k], "big")
            if n < 128:
                return None
            at += k
        if at + n > len(buf):
            return None
        return tag, buf[at:at + n], at + n

    top = tlv(sig, 0)
    if top is None or top[0] != 0x30 or top[2] != len(sig):
        return None
    body = top[1]
    vals = []
    at = 0
    while at < len(body):
        el = tlv(body, at)
        if el is None or el[0] != 0x02 or not el[1]:
            return None
        v = el[1]
        if v[0] & 0x80:
            return None
        if len(v) > 1 and v[0] == 0 and not (v[1] & 0x80):
            return None
        vals.append(int.from_bytes(v, "big"))
        at = el[2]
    if len(vals) != 2:
        return None
    return vals[0], vals[1]


def bits2int(h):
    e = int.from_bytes(h, "big")
    hbits = 8 * len(h)
    return e >> (hbits - 384) if hbits > 384 else e


def verify(q, hname, msg, r, s):
    if not (1 <= r < N and 1 <= s < N):
        return False
    e = bits2int(HASHES[hname](msg).digest())
    w = pow(s, -1, N)
    u1 = e * w % N
    u2 = r * w % N
    pt = add(mul(u1, G), mul(u2, q))
    if pt is O:
        return False
    return pt[0] % N == r


def check_file(path, form):
    data = json.loads(path.read_text())
    rows = []
    for group in data["testGroups"]:
        hname = group["sha"]
        q = decode_point(bytes.fromhex(group["publicKey"]["uncompressed"]))
        if q is None:
            raise SystemExit(f"{path.name}: a vendored key fails SEC 1 validation")
        for t in group["tests"]:
            sig = bytes.fromhex(t["sig"])
            if form == "der":
                rs = parse_der_signature(sig)
            else:
                rs = (int.from_bytes(sig[:48], "big"), int.from_bytes(sig[48:], "big")) if len(sig) == 96 else None
            got = rs is not None and verify(q, hname, bytes.fromhex(t["msg"]), *rs)
            if t["result"] not in ("valid", "invalid"):
                raise SystemExit(f"{path.name}: tcId {t['tcId']} is `{t['result']}`: no rule")
            want = t["result"] == "valid"
            if got != want:
                raise SystemExit(f"{path.name}: tcId {t['tcId']} computes {got}, the rows expect {want}")
            rows.append(got)
    return rows


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--vectors", default=str(ROOT / "vendor/vectors/wycheproof"))
    args = ap.parse_args()
    self_check()
    print("ok   P-384 constants: p and n prime, G on the curve, [n]G = O, [n-1]G = -G")
    vec = pathlib.Path(args.vectors)
    total = accept = 0
    for name, form in FILES:
        rows = check_file(vec / name, form)
        a = sum(rows)
        print(f"ok   {name}: {len(rows)} cases, {a} accept, {len(rows) - a} reject")
        total += len(rows)
        accept += a
    print(f"ok   P-384: {total} cases, {accept} accept, {total - accept} reject, every verdict as the rows expect")
    return 0


if __name__ == "__main__":
    sys.exit(main())
