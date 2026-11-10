#!/usr/bin/env python3
"""stdrows.py WOLF STDROOT FILE... : each wolf-std row on the checked machine and natively,
held to its `//! check: run(exit=N, stdout="...")` line. Prints one line per lane and an
END line; exit 1 when any lane differs."""
import json, re, subprocess, sys
wolf, root, files = sys.argv[1], sys.argv[2], sys.argv[3:]
bad = 0
for f in files:
    head = open(f).read().split("\n", 1)[0]
    m = re.match(r'//! check: run\(exit=(\d+), stdout="(.*)"\)$', head)
    want = (f"exit({m[1]})", m[2].encode().decode("unicode_escape") + "\n") if m else None
    fm = re.match(r'//! check: (fail\([A-Z0-9]+\))$', head)
    if fm:
        want = (fm[1], None)
    for lane in ("--checked", "--native"):
        out = subprocess.run([wolf, "conform-run", lane, "--std-root", root, f, "--json"], capture_output=True, text=True).stdout
        try:
            d = json.loads(out[out.index("{"):])
        except ValueError:
            d = {"verdict": "unreadable"}
        got = (d.get("verdict"), d.get("stdout_inline"))
        ok = want is not None and got[0] == want[0] and got[1] == want[1]
        if want is None:
            ok = None
        print(f"{'PASS' if ok else ('NOCHECK' if ok is None else 'FAIL')} {f} {lane[2:]}: {got[0]}" + ("" if ok else f" got={got[1]!r} want={want!r}"))
        bad += 0 if ok else 1
print(f"END rows exit={1 if bad else 0}")
sys.exit(1 if bad else 0)
