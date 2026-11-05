#!/usr/bin/env python3
"""lanes.py -- run test files on the three lanes the rig runs, outside the rig.

    python3 -I docs/sc79-witness/lanes.py [--lanes lupin,checked,native]
        [--timeout S] --wolf W --lupin L --scratch DIR TEST.lu...

Each test is copied alone into its own directory under DIR (a directory
is one module), then run as the rig runs it: `lupin conform-run`,
`wolf conform-run --checked` and `wolf conform-run --native`, each with
`--std-root` at this checkout's `std/` and `--json`. One line per lane:
the verdict the record names, its exit or trap, and the wall seconds.
A test's own `FAIL ...` lines from stdout are passed through unindented
at column 0, so a gauntlet step that runs this can name its red rows.
A lane's word is `run` when its record satisfies the file's own
`check: run(exit=..., stdout="...")` directive (exit code or trap kind,
and stdout when the directive names it), as the rig decides it.
Exit status: 0 when every lane's word is in `--want` (default `run`),
1 otherwise.
"""

import argparse
import ast
import hashlib
import json
import re
import pathlib
import shutil
import subprocess
import sys
import time

ROOT = pathlib.Path(__file__).resolve().parents[2]


def directive(src):
    """(exit, stdout) from `//! check: run(exit=E, stdout="S")`."""
    for line in src.splitlines():
        m = re.match(r"//! check: run\((.*)\)\s*$", line)
        if m:
            body = m.group(1)
            em = re.search(r"exit=(trap\([a-z_]+\)|\d+)", body)
            sm = re.search(r'stdout=("(?:[^"\\]|\\.)*")', body)
            return (em.group(1) if em else "0", ast.literal_eval(sm.group(1)) if sm else None)
    return ("0", None)


def run_lane(lane, wolf, lupin, entry, timeout):
    if lane == "lupin":
        cmd = [lupin, "conform-run"]
    else:
        cmd = [wolf, "conform-run", "--checked" if lane == "checked" else "--native"]
    cmd += ["--std-root", str(ROOT / "std"), str(entry), "--json"]
    start = time.monotonic()
    try:
        p = subprocess.run(cmd, cwd=entry.parent, capture_output=True, timeout=timeout, text=True)
    except subprocess.TimeoutExpired:
        return {"verdict": "timeout"}, time.monotonic() - start
    secs = time.monotonic() - start
    rec = None
    for line in p.stdout.splitlines():
        if line.startswith("{"):
            try:
                rec = json.loads(line)
            except ValueError:
                pass
    if rec is None:
        return {"verdict": f"tool-error(exit {p.returncode})", "stderr": p.stderr[-400:]}, secs
    return rec, secs


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--lanes", default="lupin,checked,native")
    ap.add_argument("--want", default="run")
    ap.add_argument("--timeout", type=float, default=300)
    ap.add_argument("--wolf", required=True)
    ap.add_argument("--lupin", required=True)
    ap.add_argument("--scratch", required=True)
    ap.add_argument("tests", nargs="+")
    args = ap.parse_args()
    scratch = pathlib.Path(args.scratch)
    want = set(args.want.split(","))
    rc = 0
    for t in args.tests:
        src = pathlib.Path(t)
        d = scratch / src.stem
        shutil.rmtree(d, ignore_errors=True)
        d.mkdir(parents=True)
        entry = d / src.name
        shutil.copy(src, entry)
        want_exit, want_stdout = directive(src.read_text())
        for lane in args.lanes.split(","):
            rec, secs = run_lane(lane, args.wolf, args.lupin, entry, args.timeout)
            verdict = rec.get("verdict")
            stdout = rec.get("stdout_inline") or ""
            want_v = want_exit if want_exit.startswith("trap") else f"exit({want_exit})"
            word = verdict
            # The rig's rule (xtask record.rs): the stdout hash equals the
            # directive's text, or that text plus one final newline.
            same = want_stdout is None or want_v.startswith("trap")
            if not same:
                h = rec.get("stdout_sha256") or hashlib.sha256(b"").hexdigest()
                same = h in (hashlib.sha256(want_stdout.encode()).hexdigest(),
                             hashlib.sha256((want_stdout + "\n").encode()).hexdigest())
            if verdict == want_v and same:
                word = "run"
            elif verdict == want_v:
                word = "stdout-differs"
                extra_out = stdout.replace("\n", " | ")[:400]
                print(f"    observed stdout: {extra_out}")
            for line in stdout.splitlines():
                if line.startswith("FAIL "):
                    print(line)
            extra = ""
            if "x-unsupported-construct" in rec:
                extra = f" ({rec['x-unsupported-construct']})"
            if rec.get("diagnostics"):
                d0 = rec["diagnostics"][0]
                extra = f" ({d0.get('code', '')} {d0.get('message', '')[:120]})"
            if "stderr" in rec:
                extra = " " + rec["stderr"].strip().replace("\n", " | ")[:300]
            print(f"  {src.as_posix()} {lane}: {word}{extra} {secs:.1f}s")
            if word not in want:
                rc = 1
    return rc


if __name__ == "__main__":
    sys.exit(main())
