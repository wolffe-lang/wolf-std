#!/usr/bin/env python3
"""sc70: system date is a black box, never an implementation source.

200 deterministic random instants in [2000, 2100), five host zones, ten
formats = 10,000 cells. TZDIR is removed so both programs use the host DB.
Historical offsets use vendored fixtures in local_tzif.lu, not Darwin
strftime, which misformats some historical offsets (see the lane note).
No numeric width in the cross-host set: Darwin strftime has flags but no
GNU widths. Widths have independent GNU-derived golden tests.
"""
import argparse
import json
import os
from pathlib import Path
import random
import shutil
import subprocess
import sys

ZONES = ("America/New_York", "Europe/London", "Asia/Kolkata", "Australia/Lord_Howe", "UTC")
FORMATS = ("%Y-%m-%d %H:%M:%S", "%e|%b|%B|%a|%A|%j", "%Z|%z",
           "%p|%I|%y|%C|%u|%w", "%F", "%T", "%D|%R", "%s",
           "%-d|%_d|%0e", "%c|%x|%X|%%")


def compare(got, want):
    if len(got) != len(want):
        raise ValueError(f"wrong cell count: {len(got)}, expected {len(want)}")
    return [(i, a, b) for i, (a, b) in enumerate(zip(got, want)) if a != b]


def self_test():
    want = [str(i) for i in range(10000)]
    assert not compare(want, want)
    broken = want.copy()
    broken[9999] = "planted"
    assert compare(broken, want) == [(9999, "planted", "9999")]
    for bad in (want[:-1], want + ["extra"]):
        try:
            compare(bad, want)
        except ValueError:
            pass
        else:
            raise AssertionError("truncated/extra output passed")
    assert len(ZONES) * len(FORMATS) * 200 == 10000
    print("time-oracle self-test: 5 checks, GREEN", flush=True)


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--self-test", action="store_true")
    p.add_argument("--wolf", type=Path)
    p.add_argument("--std-root", type=Path)
    p.add_argument("--work", type=Path)
    a = p.parse_args()
    self_test()
    if a.self_test:
        return
    if not all((a.wolf, a.std_root, a.work)):
        p.error("--wolf, --std-root and --work are required")
    if sys.platform not in ("darwin", "linux"):
        raise RuntimeError("time-oracle requires BSD or GNU date")
    work = a.work.resolve()
    work.mkdir(parents=True, exist_ok=True)
    pkg = work / "pkg"
    if pkg.exists():
        shutil.rmtree(pkg)
    pkg.mkdir()
    rng = random.Random(700054)
    instants = [rng.randrange(946684800, 4102444800) for _ in range(200)]
    def literal(xs):
        return "[" + ", ".join(json.dumps(x) for x in xs) + "]"
    src = ("use std.time\nfn main() -> !int {\n"
           f"    let zones = {literal(ZONES)}\n"
           f"    let instants = {literal(instants)}\n"
           f"    let formats = {literal(FORMATS)}\n"
           "    for z in zones {\n        for t in instants {\n"
           "            let l = time.local_in(z, t)?\n"
           "            for f in formats { print(time.format(l, f)?) }\n"
           "        }\n    }\n    0\n}\n")
    entry = pkg / "main.lu"
    entry.write_text(src)
    exe = work / "oracle"
    cmd = [str(a.wolf.resolve()), "build", "--no-cache", "--deny-warnings", "--std-root",
           str(a.std_root.resolve()), str(entry), "-o", str(exe)]
    env = dict(os.environ, LC_ALL="C", TZ="UTC")
    env.pop("TZDIR", None)
    print("time-oracle: compiling 200 x 5 x 10 witness", flush=True)
    built = subprocess.run(cmd, cwd=pkg, env=env, text=True, capture_output=True, timeout=120)
    (work / "build.log").write_text(built.stdout + built.stderr)
    if built.returncode != 0:
        raise RuntimeError(f"wolf build exited {built.returncode} (compile error is not a skip)\n"
                           + built.stdout + built.stderr)
    ran = subprocess.run([str(exe)], cwd=pkg, env=env, text=True, capture_output=True, timeout=120)
    (work / "wolf.stderr").write_text(ran.stderr)
    (work / "wolf.stdout").write_text(ran.stdout)
    if ran.returncode != 0 or ran.stderr or "SKIP" in ran.stdout:
        raise RuntimeError(f"wolf oracle exited {ran.returncode}\n{ran.stderr}")
    want = []
    for z in ZONES:
        print(f"time-oracle: {z} (2,000 cells)", flush=True)
        ze = dict(env, TZ=z)
        for t in instants:
            for fmt in FORMATS:
                date = ["/bin/date", "-r", str(t)] if sys.platform == "darwin" else ["/usr/bin/date", "-d", "@" + str(t)]
                out = subprocess.run(date + ["+" + fmt], env=ze, capture_output=True, text=True, timeout=5)
                if out.returncode or out.stderr:
                    raise RuntimeError(f"date failed: {date}, {fmt}: {out.stderr}")
                want.append(out.stdout.removesuffix("\n"))
    got = ran.stdout.splitlines()
    (work / "date.stdout").write_text("\n".join(want) + "\n")
    diffs = compare(got, want)
    for i, actual, expected in diffs[:20]:
        zi, rest = divmod(i, 200 * len(FORMATS))
        ti, fi = divmod(rest, len(FORMATS))
        print(f"DIFF {ZONES[zi]} t={instants[ti]} fmt={FORMATS[fi]!r}: wolf={actual!r} date={expected!r}")
    print(f"time-oracle: {sys.platform}, 200 instants x 5 zones x 10 formats, "
          f"{len(got)} cells, {len(diffs)} differences, 0 skips", flush=True)
    if diffs:
        raise RuntimeError("time-oracle: RED")


if __name__ == "__main__":
    main()
