#!/usr/bin/env python3
"""sc77: the small-order points of edwards25519, and the strict-verify rows.

This generator is independent of std's wolf code. Everything it knows about
the curve comes from the curve equation and RFC 8032's text:

    -x^2 + y^2 = 1 + d x^2 y^2   over GF(p), p = 2^255 - 19, d = -121665/121666

It derives the points P with [8]P = O (O = (0, 1), the identity), proves each
one's order by its own group law, lists their canonical and non-canonical
32-byte encodings (RFC 8032 s5.1.2 and s5.1.3), and builds the rows that
`std.x.crypto.curve25519.verify_strict` must refuse. Every Wycheproof ed25519
case std vendors gets its expected strict verdict from its `result` and from
this file's own decoding, never from std.

The only library call with cryptographic meaning is SHA-512, from Python's
`hashlib`, used through its API as an oracle (k = SHA-512(R || A || M) and the
RFC 8032 s5.1.5 key expansion).

Why exactly eight points (the derivation `derive_small_order` follows). With
a = -1 the doubling map is

    x(2P) = 2xy / (1 + d x^2 y^2),    y(2P) = (y^2 + x^2) / (1 - d x^2 y^2).

    [2]P = O    iff  x(2P) = 0 and y(2P) = 1  iff  P = (0, 1) or (0, -1)
    [4]P = O    iff  [2]P is one of those      iff  x(2P) = 0  iff  x = 0 or y = 0
    [8]P = O    iff  [2]P is 4-torsion         iff  x = 0, y = 0, or x^2 = -y^2

x = 0 gives y = +-1. y = 0 gives x^2 = -1, so x = +-sqrt(-1). x^2 = -y^2 put
into the curve equation gives d y^4 + 2 y^2 - 1 = 0, a quadratic in y^2; each
square root y of a square solution gives x = +-sqrt(-1) y. The generator
solves all three cases, so the list is complete by construction. A second,
independent derivation projects ordinary curve points into the torsion
subgroup with [L]Q and must find the same eight.

Usage (from the repository root):

    python3 tests/x/crypto/curve25519/small_order.py --wolf PATH          # write
    python3 tests/x/crypto/curve25519/small_order.py --wolf PATH --check  # compare

`wolf fmt -` (the pinned formatter, D34) canonicalises every emitted `.lu`
before it is written or compared. `--check` writes nothing and exits 1 on any
byte of drift.
"""

import argparse
import hashlib
import json
import os
import subprocess
import sys
import textwrap

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.normpath(os.path.join(HERE, "..", "..", "..", ".."))
WYCHEPROOF = os.path.join(REPO, "vendor", "vectors", "wycheproof", "ed25519_test.json")
SELF = "tests/x/crypto/curve25519/small_order.py"

# ------------------------------------------------------------------ field

P = 2**255 - 19
L = 2**252 + 27742317777372353535851937790883648493


def inv(a):
    """a^-1 = a^(p-2) mod p (RFC 8032 s5.1.1). Inverting zero is an error."""
    a %= P
    assert a != 0, "inverse of zero"
    return pow(a, P - 2, P)


D = (-121665 * inv(121666)) % P
SQRT_M1 = pow(2, (P - 1) // 4, P)
assert SQRT_M1 * SQRT_M1 % P == P - 1


def sqrt(a):
    """A square root of a mod p, or None (RFC 8032 s5.1.1, p = 5 mod 8)."""
    a %= P
    x = pow(a, (P + 3) // 8, P)
    if x * x % P == a:
        return x
    if x * x % P == (-a) % P:
        return x * SQRT_M1 % P
    return None


# ------------------------------------------------------------------ group

O = (0, 1)


def on_curve(pt):
    x, y = pt
    return (-x * x + y * y - 1 - D * x * x * y * y) % P == 0


def add(p1, p2):
    """Affine twisted-Edwards addition, a = -1 (complete: d is not a square)."""
    x1, y1 = p1
    x2, y2 = p2
    t = D * x1 * x2 * y1 * y2 % P
    x3 = (x1 * y2 + y1 * x2) * inv(1 + t) % P
    y3 = (y1 * y2 + x1 * x2) * inv(1 - t) % P
    return (x3, y3)


def mul(n, pt):
    """[n]pt by double-and-add, n >= 0."""
    q = O
    r = pt
    while n:
        if n & 1:
            q = add(q, r)
        r = add(r, r)
        n >>= 1
    return q


def order(pt):
    """The least k in 1..8 with [k]pt = O, or None if [8]pt is not O."""
    q = pt
    for k in range(1, 9):
        if q == O:
            return k
        q = add(q, pt)
    return None


def is_small(pt):
    return mul(8, pt) == O


# --------------------------------------------------------------- encoding


def encode(pt):
    """RFC 8032 s5.1.2: y little-endian, x's low bit in bit 255."""
    x, y = pt
    b = bytearray(y.to_bytes(32, "little"))
    b[31] |= (x & 1) << 7
    return bytes(b)


def decode(b):
    """RFC 8032 s5.1.3, step by step. None when decoding fails."""
    if len(b) != 32:
        return None
    n = int.from_bytes(b, "little")
    x0 = n >> 255
    y = n & ((1 << 255) - 1)
    if y >= P:
        return None
    u = (y * y - 1) % P
    v = (D * y * y + 1) % P
    x = u * pow(v, 3, P) * pow(u * pow(v, 7, P), (P - 5) // 8, P) % P
    vxx = v * x * x % P
    if vxx == u:
        pass
    elif vxx == (-u) % P:
        x = x * SQRT_M1 % P
    else:
        return None
    if x == 0 and x0 == 1:
        return None
    if x & 1 != x0:
        x = P - x
    return (x, y)


def base_point():
    """B: y = 4/5 and the even x (RFC 8032 s5.1), checked against Table 1."""
    y = 4 * inv(5) % P
    b = decode(y.to_bytes(32, "little"))
    assert b is not None and b[0] % 2 == 0
    assert b == (
        15112221349535400772501151409588531511454012693041857206046113283949847762202,
        46316835694926478169428394003475163141307993866256225615783033603165251855960,
    ), "B differs from RFC 8032 Table 1"
    assert on_curve(b) and mul(L, b) == O
    return b


B = base_point()

# ------------------------------------------------------- the eight points


def derive_small_order():
    """Solve [8]P = O case by case (the module docstring's derivation)."""
    pts = []
    # x = 0: y^2 = 1.
    for y in (1, P - 1):
        pts.append((0, y))
    # y = 0: -x^2 = 1.
    i = sqrt(P - 1)
    assert i is not None
    pts += [(i, 0), (P - i, 0)]
    # x^2 = -y^2: d (y^2)^2 + 2 (y^2) - 1 = 0, discriminant 4 + 4d.
    disc = sqrt(4 + 4 * D)
    assert disc is not None, "4 + 4d is not a square"
    squares = 0
    for root in ((-2 + disc) * inv(2 * D) % P, (-2 - disc) * inv(2 * D) % P):
        y = sqrt(root)
        if y is None:
            continue
        squares += 1
        for yy in (y, P - y):
            x = SQRT_M1 * yy % P
            for xx in (x, P - x):
                pts.append((xx, yy))
    assert squares == 1, "expected exactly one square root of y^2"
    assert len(set(pts)) == 8 == len(pts)
    for pt in pts:
        assert on_curve(pt), pt
        assert mul(8, pt) == O
    return pts


def project_torsion():
    """Second derivation: [L]Q for ordinary curve points Q, until eight."""
    seen = set()
    tries = 0
    y = 2
    while len(seen) < 8:
        q = decode(y.to_bytes(32, "little"))
        y += 1
        if q is None:
            continue
        tries += 1
        t = mul(L, q)
        assert mul(8, t) == O
        seen.add(t)
        assert tries < 400, "projection did not find eight points"
    return seen, tries


def name_of(pt, k):
    """A stable row name: the order, and for order >= 4 the encoding."""
    enc = encode(pt)
    if k == 1:
        return "identity"
    if k == 2:
        return "order2"
    return "order%d-%s" % (k, enc.hex()[:4] + ("x1" if enc[31] & 0x80 else "x0"))


def small_order_table():
    pts = derive_small_order()
    proj, tries = project_torsion()
    assert proj == set(pts), "the two derivations disagree"
    rows = []
    for pt in pts:
        k = order(pt)
        assert k is not None and 8 % k == 0
        chain = [encode(mul(j, pt)).hex() for j in range(1, 9)]
        assert chain[7] == encode(O).hex()
        rows.append({"point": pt, "order": k, "name": name_of(pt, k), "enc": encode(pt), "chain": chain})
    rows.sort(key=lambda r: (r["order"], r["enc"]))
    gen = [r for r in rows if r["order"] == 8][0]["point"]
    assert {mul(j, gen) for j in range(8)} == set(pts), "the torsion is not cyclic"
    return rows, tries


def non_canonical(rows):
    """Every 32-byte string whose y (mod p) and sign bit name a small-order
    point but which is not that point's canonical encoding."""
    out = []
    small = {r["point"] for r in rows}
    ys = sorted({pt[1] for pt in small})
    for y in ys:
        for y_enc in (y, y + P):
            if y_enc >= 1 << 255:
                continue
            for sign in (0, 1):
                b = bytearray(y_enc.to_bytes(32, "little"))
                b[31] |= sign << 7
                b = bytes(b)
                if any(b == r["enc"] for r in rows):
                    continue
                # Why RFC 8032 s5.1.3 refuses the bytes.
                why = []
                if y_enc >= P:
                    why.append("y = %s >= p" % ("p" if y == 0 else "p + %d" % y))
                if sign == 1 and any(pt[0] == 0 for pt in small if pt[1] == y):
                    why.append("x = 0 with x_0 = 1")
                why = ", and ".join(why)
                assert decode(b) is None, b.hex()
                out.append({"enc": b, "why": why, "name": "nc-" + b.hex()[:4] + b.hex()[-2:]})
    names = [r["name"] for r in out]
    assert len(set(names)) == len(names)
    return out


# ----------------------------------------------------------- verification


def sha512(b):
    return hashlib.sha512(b).digest()


def ref_verify(pk, msg, sig, strict=False):
    """RFC 8032 s5.1.7, cofactorless (std's equation), S < L strict. With
    strict, also refuse A or R of small order."""
    if len(pk) != 32 or len(sig) != 64:
        return False
    a = decode(pk)
    r = decode(sig[:32])
    s = int.from_bytes(sig[32:], "little")
    if a is None or r is None or s >= L:
        return False
    if strict and (is_small(a) or is_small(r)):
        return False
    k = int.from_bytes(sha512(sig[:32] + pk + msg), "little") % L
    return mul(s, B) == add(r, mul(k, a))


def expand(sk):
    """RFC 8032 s5.1.5: the clamped scalar of a 32-byte secret."""
    h = bytearray(sha512(sk)[:32])
    h[0] &= 248
    h[31] &= 127
    h[31] |= 64
    return int.from_bytes(bytes(h), "little")


TEST1_SK = bytes.fromhex("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60")
TEST1_PK = bytes.fromhex("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a")
MESSAGES = [("empty", b""), ("72", bytes([0x72]))]


def build_rows(points, ncs):
    a1 = expand(TEST1_SK)
    assert encode(mul(a1, B)) == TEST1_PK, "TEST 1's public key"
    ident = encode(O)
    zero = bytes(32)
    rows = []
    for pt in points:
        for tag, msg in MESSAGES:
            pk = pt["enc"]
            sig = ident + zero
            k = int.from_bytes(sha512(ident + pk + msg), "little") % L
            rows.append({
                "kind": "a", "name": "a-%s-%s" % (pt["name"], tag), "point": pt, "pk": pk, "msg": msg,
                "sig": sig, "k_mod_order": k % pt["order"],
                "verify": ref_verify(pk, msg, sig), "strict": ref_verify(pk, msg, sig, strict=True),
            })
    for pt in points:
        for tag, msg in MESSAGES:
            rb = pt["enc"]
            k = int.from_bytes(sha512(rb + TEST1_PK + msg), "little") % L
            s = k * a1 % L
            sig = rb + s.to_bytes(32, "little")
            rows.append({
                "kind": "r", "name": "r-%s-%s" % (pt["name"], tag), "point": pt, "pk": TEST1_PK, "msg": msg,
                "sig": sig, "verify": ref_verify(TEST1_PK, msg, sig),
                "strict": ref_verify(TEST1_PK, msg, sig, strict=True),
            })
    for nc in ncs:
        sig = ident + zero
        rows.append({
            "kind": "nc-a", "name": "%s-as-a" % nc["name"], "nc": nc, "pk": nc["enc"], "msg": b"",
            "sig": sig, "verify": ref_verify(nc["enc"], b"", sig),
            "strict": ref_verify(nc["enc"], b"", sig, strict=True),
        })
        sig = nc["enc"] + zero
        rows.append({
            "kind": "nc-r", "name": "%s-as-r" % nc["name"], "nc": nc, "pk": TEST1_PK, "msg": b"",
            "sig": sig, "verify": ref_verify(TEST1_PK, b"", sig),
            "strict": ref_verify(TEST1_PK, b"", sig, strict=True),
        })
    for r in rows:
        assert r["strict"] is False, r["name"]
    return rows


def wycheproof_cases():
    with open(WYCHEPROOF) as f:
        doc = json.load(f)
    cases = []
    for g in doc["testGroups"]:
        pk = bytes.fromhex(g["publicKey"]["pk"])
        for t in g["tests"]:
            if t["result"] not in ("valid", "invalid"):
                raise SystemExit("tcId %d: result %r is not decided by this generator" % (t["tcId"], t["result"]))
            msg = bytes.fromhex(t["msg"])
            sig = bytes.fromhex(t["sig"])
            valid = t["result"] == "valid"
            a = decode(pk)
            r = decode(sig[:32]) if len(sig) >= 32 else None
            a_small = a is not None and is_small(a)
            r_small = r is not None and is_small(r)
            strict = valid and not a_small and not r_small
            ref = ref_verify(pk, msg, sig)
            assert ref == valid, "tcId %d: the reference verify disagrees with Wycheproof" % t["tcId"]
            assert ref_verify(pk, msg, sig, strict=True) == strict, t["tcId"]
            cases.append({
                "tc": t["tcId"], "pk": pk, "msg": msg, "sig": sig, "valid": valid, "flags": t["flags"],
                "a_small": a_small, "r_small": r_small, "strict": strict,
            })
    assert len(cases) == doc["numberOfTests"]
    return cases


# --------------------------------------------------------------- emitters

HEAD = (
    "//! check: run(exit=0)\n//! phase: run\n//! conforms: std.x.crypto.curve25519, arith.checked\n//!\n"
    "//! GENERATED by `%s`\n"
    "//! (python3, from the curve equation and RFC 8032's text, independent\n"
    "//! of std's wolf code). Do not edit by hand; `small_order.py --check`\n"
    "//! holds this file byte-identical to its generator.\n//!\n" % SELF
)

HELPERS = (
    "\nuse std.hex\nuse std.x.crypto.curve25519\n\n"
    "fn bytes_of(h: str) -> List[byte] {\n    hex.decode(h) else List[byte]()\n}\n\n"
)


def row_fn(func):
    return (
        "// One row: prints `FAIL <row>` and counts 1 when `%s` disagrees.\n"
        "fn row(name: str, pk_hex: str, msg_hex: str, sig_hex: str, want: bool) -> int {\n"
        "    if curve25519.%s(bytes_of(pk_hex), bytes_of(msg_hex), bytes_of(sig_hex)) == want {\n"
        "        return 0\n    }\n"
        "    print(\"FAIL {name}\")\n    1\n}\n\n" % (func, func)
    )


def emit(rows, prose, func, prefix):
    s = HEAD + prose + HELPERS + row_fn(func)
    s += "fn main() -> int {\n    var fails: int = 0\n"
    for r in rows:
        s += '    fails = fails + row("%s%s", "%s", "%s", "%s", %s)\n' % (
            prefix, r["name"], r["pk"].hex(), r["msg"].hex(), r["sig"].hex(), "true" if r["want"] else "false")
    s += "    if fails > 0 {\n        return 1\n    }\n    0\n}\n"
    return s


def prose(paras):
    """Doc-comment prose: each entry is a paragraph (a `- ` entry is a bullet),
    wrapped at 72 columns."""
    out = []
    for para in paras:
        bullet = para.startswith("- ")
        out += textwrap.wrap(para, 68, subsequent_indent="  " if bullet else "",
                             break_on_hyphens=False, break_long_words=False)
    return "".join(("//! " + ln).rstrip() + "\n" for ln in out)


def files(points, ncs, rows, cases):
    out = {}
    strict_rows = [dict(r, want=r["strict"]) for r in rows]
    out["ed25519_strict_small_order.lu"] = emit(strict_rows, prose([
        "`verify_strict` against the eight small-order points of edwards25519 and their six "
        "non-canonical encodings: every row must answer false.",
        "- `a-*`: each point as the public key A, with R = the identity and S = 0, over the empty "
        "message and the one-byte message 72.",
        "- `r-*`: each point as R under RFC 8032 s7.1 TEST 1's key, with S = k * a mod L, so the "
        "identity rows satisfy the cofactorless equation `verify` checks.",
        "- `nc-*`: each non-canonical encoding as A and as R; decoding refuses them before the "
        "small-order check is reached.",
        "Every refusal happens before a scalar multiplication. `verify`'s answers on the same rows "
        "are `ed25519_small_order_verify.lu`.",
    ]), "verify_strict", "strict-")
    verify_rows = [dict(r, want=r["verify"]) for r in rows]
    out["ed25519_small_order_verify.lu"] = emit(verify_rows, prose([
        "`verify` (cofactorless, RFC 8032 s5.1.7, unchanged by sc77) on the rows of "
        "`ed25519_strict_small_order.lu`, pinned at the verdicts this file's generator computes "
        "from the RFC: %d of the %d rows are accepted. The A rows accept exactly when ord(A) "
        "divides k, and the R rows exactly when R is the identity. `verify_strict` refuses them "
        "all." % (sum(r["verify"] for r in rows), len(rows)),
    ]), "verify", "verify-")
    chunk = 40
    parts = [cases[i:i + chunk] for i in range(0, len(cases), chunk)]
    for n, part in enumerate(parts, 1):
        out["wycheproof_ed25519_strict_p%d.lu" % n] = emit(
            [dict(c, name="tc%d" % c["tc"], want=c["strict"]) for c in part],
            prose(wp_prose(part, "part %d of %d" % (n, len(parts)), len(cases))), "verify_strict", "strict-wp-")
    first_valid = next(c for c in cases if c["valid"])
    first_invalid = next(c for c in cases if not c["valid"])
    first_small = next(c for c in cases if c["a_small"] or c["r_small"])
    smoke = [first_valid, first_invalid] + ([first_small] if first_small is not first_invalid else [])
    out["wycheproof_ed25519_strict_smoke.lu"] = emit(
        [dict(c, name="tc%d" % c["tc"], want=c["strict"]) for c in smoke],
        prose(wp_prose(smoke, "the differential smoke subset", len(cases))), "verify_strict", "strict-wp-")
    return out


def wp_prose(part, which, total):
    nv = sum(c["strict"] for c in part)
    small = [c["tc"] for c in part if c["a_small"] or c["r_small"]]
    differ = [c["tc"] for c in part if c["strict"] != c["valid"]]
    return [
        "Wycheproof v1 Ed25519 under `verify_strict`, %s (%d of the set's %d: %d accepted, %d refused). "
        "The expected verdict is the vector's `result`, and a vector whose A or R has small order is "
        "refused as well (this file's generator decodes both). Cases here with a small-order A or R: %s. "
        "Cases whose verdict differs from `verify`'s: %s."
        % (which, len(part), total, nv, len(part) - nv,
           ", ".join("tc %d" % t for t in small) or "none",
           ", ".join("tc %d" % t for t in differ) or "none"),
    ]


def record(points, tries, ncs, rows, cases):
    """The human-readable derivation, committed beside the tests."""
    w = []
    w.append("sc77 small-order record, written by %s" % SELF)
    w.append("p = 2^255 - 19; d = -121665/121666 mod p; L = 2^252 + 27742317777372353535851937790883648493")
    w.append("B from y = 4/5 and the even x matches RFC 8032 Table 1; [L]B = O.")
    w.append("")
    w.append("THE EIGHT POINTS ([8]P = O), solved from the curve equation (x = 0, y = 0, x^2 = -y^2)")
    w.append("cross-check: [L]Q over %d ordinary curve points found the same 8; the order-8 points generate all 8" % tries)
    for r in points:
        x, y = r["point"]
        w.append("")
        w.append("%-18s order %d  encoding %s" % (r["name"], r["order"], r["enc"].hex()))
        w.append("  x = %064x" % x)
        w.append("  y = %064x" % y)
        w.append("  [1..8]P: " + " ".join(c[:8] for c in r["chain"]))
    w.append("")
    w.append("NON-CANONICAL ENCODINGS (RFC 8032 s5.1.3 decoding fails): %d" % len(ncs))
    for nc in ncs:
        w.append("  %s  %-14s %s" % (nc["enc"].hex(), nc["name"], nc["why"]))
    w.append("")
    w.append("ROWS (verify = cofactorless RFC 8032 s5.1.7, verify_strict = the same plus [8]A != O and [8]R != O)")
    w.append("  %-28s %-7s %-6s %s" % ("row", "verify", "strict", "detail"))
    for r in rows:
        detail = ""
        if r["kind"] == "a":
            detail = "ord(A) = %d, k mod ord(A) = %d" % (r["point"]["order"], r["k_mod_order"])
        elif r["kind"] == "r":
            detail = "ord(R) = %d, S = k*a mod L" % r["point"]["order"]
        else:
            detail = r["nc"]["why"]
        w.append("  %-28s %-7s %-6s %s" % (r["name"], str(r["verify"]).lower(), str(r["strict"]).lower(), detail))
    w.append("  accepted by verify: %s" % ", ".join(r["name"] for r in rows if r["verify"]))
    w.append("")
    nv = sum(c["valid"] for c in cases)
    w.append("WYCHEPROOF ed25519_test.json: %d cases, %d valid, %d invalid; the reference verify agrees on all"
             % (len(cases), nv, len(cases) - nv))
    w.append("  small-order A: %s" % (", ".join("tc %d" % c["tc"] for c in cases if c["a_small"]) or "none"))
    w.append("  small-order R: %s" % (", ".join("tc %d (%s)" % (c["tc"], "valid" if c["valid"] else "invalid")
                                              for c in cases if c["r_small"]) or "none"))
    diff = [c for c in cases if c["strict"] != c["valid"]]
    w.append("  verdict differs between verify and verify_strict: %s"
             % (", ".join("tc %d" % c["tc"] for c in diff) or "none (0 of %d)" % len(cases)))
    w.append("  strict accepts %d, refuses %d" % (sum(c["strict"] for c in cases), sum(not c["strict"] for c in cases)))
    return "\n".join(w) + "\n"


def wolf_fmt(wolf, name, text):
    got = subprocess.run([wolf, "fmt", "-"], input=text, capture_output=True, text=True)
    if got.returncode != 0 or "W0301" in got.stderr:
        raise SystemExit("wolf fmt failed on generated %s:\n%s%s" % (name, got.stdout, got.stderr))
    return got.stdout


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--wolf", default=os.environ.get("WOLF_BIN"), help="the pinned wolf (default $WOLF_BIN)")
    ap.add_argument("--check", action="store_true", help="compare, write nothing")
    args = ap.parse_args()
    if not args.wolf:
        raise SystemExit("small_order.py needs the pinned wolf for `wolf fmt` (--wolf or $WOLF_BIN)")
    points, tries = small_order_table()
    ncs = non_canonical(points)
    rows = build_rows(points, ncs)
    cases = wycheproof_cases()
    want = {"small_order.txt": record(points, tries, ncs, rows, cases)}
    for name, text in files(points, ncs, rows, cases).items():
        want[name] = wolf_fmt(args.wolf, name, text)
    drift = []
    for name, text in sorted(want.items()):
        path = os.path.join(HERE, name)
        have = None
        if os.path.exists(path):
            with open(path) as f:
                have = f.read()
        if args.check:
            if have != text:
                drift.append(name)
        elif have != text:
            with open(path, "w") as f:
                f.write(text)
            print("small_order.py: wrote %s" % name)
    if drift:
        print("small_order.py --check: %d file(s) drift from the generator: %s" % (len(drift), ", ".join(drift)))
        return 1
    print("small_order.py%s: %d file(s) %s" % (" --check" if args.check else "", len(want),
                                              "match the generator" if args.check else "current"))
    return 0


if __name__ == "__main__":
    sys.exit(main())
