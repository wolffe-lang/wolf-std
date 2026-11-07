#!/usr/bin/env python3
"""run.py -- sc80's interop rows: std.x.crypto.passwd against Ubuntu's crypt.

    python3 -I docs/sc80-witness/interop/run.py --wolf W --std-root std --scratch DIR

Run inside sc80's pinned Ubuntu 24.04 image (libxcrypt 4.4.36; Python
3.12's `crypt` module, Perl's `crypt`, and `mkpasswd` from `whois`), the
black-box oracles. Python's `hashlib.scrypt` is the black-box oracle for
raw scrypt output. Every password and salt is drawn at run time from
`secrets` and never written to disk; the wolf side is
`interop/probe/main.lu`, built once with the default build (the rig's
lane) and fed one request a line.

Rows, each printed `ok   <row>` or `FAIL <row>` (column 0):
  mine-*        a string passwd.hash made (salt from passwd.gen_salt over 16
                random octets) is accepted by Ubuntu's crypt with its
                password and refused with a wrong one;
  ubuntu-*      a string Ubuntu's crypt made from a setting with a random
                salt is accepted by passwd.verify, refused with a wrong
                password, and is byte-identical to passwd.hash of the same
                inputs;
  mkpasswd-*    `mkpasswd -m scrypt` strings at its default cost and at -R 6;
  perl-*        Perl's crypt (raw octets, non-UTF-8 passwords) both ways;
  salt-len-*    salts of 0, 1, 43 and 86 characters;
  method-*      every other method mkpasswd offers is `unsupported`;
  malformed-*   mutated strings: refused by name here, and Ubuntu's crypt
                does not accept them either (or a named difference);
  zero-octet    the named difference: crypt stops at a zero octet;
  hashlib-*     raw scrypt against hashlib.scrypt over random parameters.
Exit 0 when every row is ok, 1 otherwise.
"""

import argparse
import hashlib
import os
import pathlib
import secrets
import subprocess
import sys
import warnings

warnings.simplefilter("ignore", DeprecationWarning)
import crypt  # noqa: E402  (Ubuntu's libxcrypt, the black box)

A = "./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
HERE = pathlib.Path(__file__).resolve().parent
COSTS = [(14, 8, 1), (15, 8, 1)]
WIDE = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 !\"#%&'()*+,-./:;<=>?@[]^_`{|}~é狼ß"

rows = []


def row(name, ok, detail=""):
    rows.append((name, ok))
    if ok:
        print(f"ok   {name}{(' ' + detail) if detail else ''}")
    else:
        print(f"FAIL {name}{(' ' + detail) if detail else ''}")


def enc(b):
    out = []
    for i in range(0, len(b), 3):
        chunk = b[i:i + 3]
        v = sum(x << (8 * j) for j, x in enumerate(chunk))
        for k in range({1: 2, 2: 3, 3: 4}[len(chunk)]):
            out.append(A[(v >> (6 * k)) & 63])
    return "".join(out)


def enc30(v):
    return "".join(A[(v >> (6 * k)) & 63] for k in range(5))


def setting(n, r, p, salt):
    return "$7$" + A[n] + enc30(r) + enc30(p) + salt


def random_password():
    n = 1 + secrets.randbelow(32)
    return "".join(secrets.choice(WIDE) for _ in range(n))


def ubuntu(pw, s):
    """Ubuntu's crypt(pw, s) through Python, or None when it refuses."""
    try:
        out = crypt.crypt(pw, s)
    except OSError:
        return None
    if out is None or out.startswith("*"):
        return None
    return out


def perl_crypt(pw_octets, s):
    """Ubuntu's crypt through Perl, raw octets (a C string: stops at NUL)."""
    p = subprocess.run(["perl", "-e", 'print crypt(pack("H*", $ARGV[0]), $ARGV[1]) // "*undef"',
                        pw_octets.hex(), s], capture_output=True, text=True)
    out = p.stdout
    if p.returncode != 0 or out.startswith("*"):
        return None
    return out


class Probe:
    def __init__(self, exe):
        self.exe = exe
        self.reqs = []

    def ask(self, line):
        self.reqs.append(line)
        return len(self.reqs) - 1

    def run(self):
        text = "\n".join(self.reqs + ["end"]) + "\n"
        p = subprocess.run([self.exe], input=text.encode(), capture_output=True, timeout=3600)
        out = p.stdout.decode().splitlines()
        if p.returncode != 0 or len(out) != len(self.reqs):
            raise SystemExit(f"FAIL probe-run exit {p.returncode}, {len(out)} answers for {len(self.reqs)} requests: {p.stderr.decode()[-400:]}")
        return out


def hx(s):
    return s.encode().hex() if isinstance(s, str) else s.hex()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--wolf", required=True)
    ap.add_argument("--std-root", required=True)
    ap.add_argument("--scratch", required=True)
    ap.add_argument("--per-cost", type=int, default=20)
    args = ap.parse_args()
    scratch = pathlib.Path(args.scratch)
    (scratch / "probe").mkdir(parents=True, exist_ok=True)
    src = scratch / "probe" / "main.lu"
    src.write_bytes((HERE / "probe" / "main.lu").read_bytes())
    exe = scratch / "probe" / "probe"
    subprocess.run([args.wolf, "build", "--std-root", str(pathlib.Path(args.std_root).resolve()),
                    str(src), "-o", str(exe)], check=True)
    print(open("/oracle-versions.txt").read().strip())
    pr = Probe(str(exe))
    plan = []

    # mine-* and ubuntu-*: at least 20 random passwords and salts per cost.
    for (n, r, p) in COSTS:
        for k in range(args.per_cost):
            pw = random_password()
            wrong = pw + secrets.choice("xyz!")
            rnd = secrets.token_bytes(16)
            i_salt = pr.ask(f"gensalt {rnd.hex()}")
            plan.append(("gensalt", f"gen-salt-{n}-{k}", i_salt, enc(rnd)))
            # the salt the probe will make is enc(rnd) if gen_salt agrees; the
            # hash request uses that same salt so a disagreement shows twice.
            i_mine = pr.ask(f"hash {hx(pw)} {enc(rnd)} {n} {r} {p}")
            plan.append(("mine", f"mine-{n}-{r}-{p}-{k}", i_mine, pw, wrong))
            us = enc(secrets.token_bytes(16))
            uline = ubuntu(pw, setting(n, r, p, us))
            i_v = pr.ask(f"verify {hx(pw)} {uline}")
            i_w = pr.ask(f"verify {hx(wrong)} {uline}")
            i_h = pr.ask(f"hash {hx(pw)} {us} {n} {r} {p}")
            plan.append(("ubuntu", f"ubuntu-{n}-{r}-{p}-{k}", (i_v, i_w, i_h), uline))

    # mkpasswd-*: its default cost (N = 2^14, r = 32) and -R 6 (2^13, r = 32).
    for k in range(6):
        pw = random_password().replace(" ", "_")
        extra = [] if k < 4 else ["-R", "6"]
        mline = subprocess.run(["mkpasswd", "-m", "scrypt", *extra, pw], capture_output=True,
                               text=True, check=True).stdout.strip()
        i_v = pr.ask(f"verify {hx(pw)} {mline}")
        i_w = pr.ask(f"verify {hx(pw + 'q')} {mline}")
        plan.append(("mkpasswd", f"mkpasswd-{'default' if k < 4 else 'R6'}-{k}", (i_v, i_w), mline))

    # perl-*: raw octets, including ones that are not UTF-8, both ways.
    for k in range(6):
        pw = secrets.token_bytes(1 + secrets.randbelow(24)).replace(b"\x00", b"\x01")
        s = enc(secrets.token_bytes(16))
        pline = perl_crypt(pw, setting(14, 8, 1, s))
        i_h = pr.ask(f"hash {pw.hex()} {s} 14 8 1")
        i_v = pr.ask(f"verify {pw.hex()} {pline}")
        plan.append(("perl", f"perl-octets-{k}", (i_h, i_v), pw, pline))

    # salt-len-*: Ubuntu accepts these salt lengths, and so does passwd.
    for length in [0, 1, 43, 86]:
        pw = random_password()
        s = "".join(secrets.choice(A) for _ in range(length))
        uline = ubuntu(pw, setting(10, 8, 1, s))
        i_h = pr.ask(f"hash {hx(pw)} {s or '-'} 10 8 1")
        i_v = pr.ask(f"verify {hx(pw)} {uline}")
        plan.append(("saltlen", f"salt-len-{length}", (i_h, i_v), uline))
    pw87 = random_password()
    s87 = "".join(secrets.choice(A) for _ in range(87))
    i_h87 = pr.ask(f"hash {hx(pw87)} {s87} 10 8 1")
    plan.append(("salt87", "salt-len-87", i_h87, ubuntu(pw87, setting(10, 8, 1, s87))))

    # method-*: every other method is `unsupported`, from parse and verify.
    for m in ["yescrypt", "gost-yescrypt", "bcrypt", "bcrypt-a", "sha512crypt", "sha256crypt",
              "sunmd5", "md5crypt", "bsdicrypt", "descrypt", "nt"]:
        pw = secrets.token_urlsafe(8)
        line = subprocess.run(["mkpasswd", "-m", m, pw], capture_output=True, text=True,
                              check=True).stdout.strip()
        i_p = pr.ask(f"parse {line}")
        i_v = pr.ask(f"verify {hx(pw)} {line}")
        plan.append(("method", f"method-{m}", (i_p, i_v), line))

    # malformed-*: mutations of a string Ubuntu made.
    pw = random_password()
    base = ubuntu(pw, setting(10, 8, 1, enc(secrets.token_bytes(16))))
    params, salt_s, key = base[3:14], base[14:base.rindex("$")], base[base.rindex("$") + 1:]
    head = "$7$"
    # (row, string, passwd's refusal, which part the mutation is in)
    muts = [
        ("n-zero", head + "." + params[1:] + salt_s + "$" + key, "bad_n", "setting"),
        ("n-16-at-r-1", head + "E/..../...." + salt_s + "$" + key, "bad_n", "setting"),
        ("r-zero", head + params[0] + "....." + params[6:] + salt_s + "$" + key, "bad_rp", "setting"),
        ("p-zero", head + params[:6] + "....." + salt_s + "$" + key, "bad_rp", "setting"),
        ("rp-2^30", head + params[0] + "zzzzz" + "0...." + salt_s + "$" + key, "bad_rp", "setting"),
        ("salt-hyphen", head + params + salt_s[:5] + "-" + salt_s[6:] + "$" + key, "bad_salt", "setting"),
        ("salt-87", head + params + (salt_s * 4)[:87] + "$" + key, "bad_salt", "setting"),
        ("key-42", head + params + salt_s + "$" + key[:42], "bad_hash", "key"),
        ("key-44", head + params + salt_s + "$" + key + ".", "bad_hash", "key"),
        ("key-noncanonical", head + params + salt_s + "$" + key[:42] + "z", "bad_hash", "key"),
        ("no-key", head + params + salt_s, "malformed", "key"),
        ("short", head + params[:5], "malformed", "setting"),
    ]
    for name, s, want, part in muts:
        i_v = pr.ask(f"verify {hx(pw)} {s}")
        plan.append(("malformed", f"malformed-{name}", i_v, pw, s, want, part))

    # zero-octet: crypt reads a C string; passwd hashes every octet.
    zs = enc(secrets.token_bytes(16))
    zpw = secrets.token_bytes(6).replace(b"\x00", b"\x01") + b"\x00" + secrets.token_bytes(5).replace(b"\x00", b"\x01")
    i_full = pr.ask(f"hash {zpw.hex()} {zs} 10 8 1")
    i_cut = pr.ask(f"hash {zpw[:6].hex()} {zs} 10 8 1")
    plan.append(("zero", "zero-octet-named-difference", (i_full, i_cut), perl_crypt(zpw, setting(10, 8, 1, zs))))

    # hashlib-*: raw scrypt against hashlib.scrypt.
    for k in range(24):
        r = 1 + secrets.randbelow(8)
        n = 1 + secrets.randbelow(12)
        p = 1 + secrets.randbelow(4)
        dk = 1 + secrets.randbelow(128)
        P = secrets.token_bytes(secrets.randbelow(41))
        S = secrets.token_bytes(secrets.randbelow(41))
        i_s = pr.ask(f"scrypt {P.hex() or '-'} {S.hex() or '-'} {n} {r} {p} {dk}")
        want = hashlib.scrypt(P, salt=S, n=1 << n, r=r, p=p, dklen=dk, maxmem=(1 << 31) - 1).hex()
        plan.append(("hashlib", f"hashlib-{n}-{r}-{p}-{dk}-{k}", i_s, want))

    out = pr.run()
    diffs = []
    agree = []
    for item in plan:
        kind, name = item[0], item[1]
        if kind == "gensalt":
            row(name, out[item[2]] == f"salt {item[3]}")
        elif kind == "mine":
            got = out[item[2]]
            line = got[5:] if got.startswith("hash $7$") else None
            ok = line is not None and ubuntu(item[3], line) == line and ubuntu(item[4], line) != line
            row(name + "-ubuntu-verifies", ok, line or got)
        elif kind == "ubuntu":
            i_v, i_w, i_h = item[2]
            ok = item[3] is not None and out[i_v] == "verify true" and out[i_w] == "verify false"
            row(name + "-passwd-verifies", ok, item[3] or "ubuntu refused its own setting")
            row(name + "-hash-identical", out[i_h] == f"hash {item[3]}")
        elif kind == "mkpasswd":
            i_v, i_w = item[2]
            row(name, out[i_v] == "verify true" and out[i_w] == "verify false", item[3])
        elif kind == "perl":
            i_h, i_v = item[2]
            pw, pline = item[3], item[4]
            mine = out[i_h][5:]
            row(name, pline is not None and out[i_v] == "verify true" and mine == pline
                and perl_crypt(pw, mine) == mine, mine)
        elif kind == "saltlen":
            i_h, i_v = item[2]
            row(name, item[3] is not None and out[i_h] == f"hash {item[3]}" and out[i_v] == "verify true",
                item[3] or "ubuntu refused")
        elif kind == "salt87":
            ours = out[item[2]]
            row(name + "-refused", ours == "hash refused bad_salt", ours)
            if item[3] is not None:
                diffs.append(f"salt of 87 characters: Ubuntu's crypt accepts it ({item[3][:24]}...), passwd refuses bad_salt")
        elif kind == "method":
            i_p, i_v = item[2]
            row(name, out[i_p] == "parse refused unsupported" and out[i_v] == "verify refused unsupported", item[3])
        elif kind == "malformed":
            pw, s, want, part = item[3], item[4], item[5], item[6]
            ours = out[item[2]]
            row(name, ours == f"verify refused {want}", ours)
            theirs = ubuntu(pw, s)
            if theirs == s:
                diffs.append(f"{name}: Ubuntu's crypt VERIFIES {s}; passwd refuses it, {want}")
            elif theirs is None:
                agree.append(f"{name}: Ubuntu's crypt refuses the setting; passwd refuses {want}")
            elif part == "key":
                agree.append(f"{name}: the setting is sound, so Ubuntu's crypt hashes under it and the "
                             f"stored key does not match; passwd refuses {want}")
            else:
                diffs.append(f"{name}: Ubuntu's crypt takes this setting ({theirs[:24]}...); passwd refuses "
                             f"it by name, {want}")
        elif kind == "zero":
            i_full, i_cut = item[2]
            pl = item[3]
            ok = pl is not None and out[i_cut] == f"hash {pl}" and out[i_full] != f"hash {pl}"
            row(name, ok, "crypt(pw with a zero octet) = passwd.hash(pw cut at the octet), and passwd.hash(pw) differs")
        elif kind == "hashlib":
            row(name, out[item[2]] == f"scrypt {item[3]}")
    print("agreements on strings passwd refuses:")
    for a in agree:
        print(f"  {a}")
    print("named differences:")
    for d in diffs or ["(none)"]:
        print(f"  {d}")
    bad = [n for n, ok in rows if not ok]
    print(f"interop: {len(rows)} rows, {len(rows) - len(bad)} ok, {len(bad)} failed")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
