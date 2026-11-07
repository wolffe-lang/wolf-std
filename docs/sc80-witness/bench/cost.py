#!/usr/bin/env python3
"""cost.py -- sc80's cost witness: scrypt at N = 2^14 and 2^15, r = 8, p = 1.

    python3 -I docs/sc80-witness/bench/cost.py --wolf W --std-root std --scratch DIR [--runs 5]

Builds `bench/main.lu` twice (the default build, the rig's lane, and
`--release`), then runs every (build, cost) pair once per round for
`--runs` interleaved rounds, plus the floor (a run that derives nothing).
Each child is started by this small parent with fork/exec and reaped with
wait4, so its `ru_maxrss` is its peak resident set. That figure starts at
the parent's own resident set (it survives the exec), so the floor row
reads the parent, not the program; the column that isolates the program
is the peak minus V (128·r·N octets), which is the same at both costs
when V is all that grows. Prints the minimum wall time and the largest
peak per pair.
"""
import argparse
import os
import pathlib
import platform
import subprocess
import sys
import time

COSTS = [(14, 8, 1), (15, 8, 1)]


def child(exe, args):
    t0 = time.monotonic()
    pid = os.fork()
    if pid == 0:
        fd = os.open(os.devnull, os.O_WRONLY)
        os.dup2(fd, 1)
        os.execv(exe, [exe, *args])
    _, status, ru = os.wait4(pid, 0)
    wall = time.monotonic() - t0
    kib = ru.ru_maxrss // 1024 if sys.platform == "darwin" else ru.ru_maxrss
    return os.waitstatus_to_exitcode(status), wall, kib


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--wolf", required=True)
    ap.add_argument("--std-root", required=True)
    ap.add_argument("--scratch", required=True)
    ap.add_argument("--runs", type=int, default=5)
    a = ap.parse_args()
    d = pathlib.Path(a.scratch) / "bench"
    d.mkdir(parents=True, exist_ok=True)
    (d / "main.lu").write_bytes((pathlib.Path(__file__).parent / "main.lu").read_bytes())
    std = str(pathlib.Path(a.std_root).resolve())
    exes = {}
    for mode, flags in [("default", []), ("release", ["--release"])]:
        exe = str(d / f"bench-{mode}")
        subprocess.run([a.wolf, "build", *flags, "--std-root", std, str(d / "main.lu"), "-o", exe], check=True)
        exes[mode] = exe
    print(f"host {platform.system()} {platform.machine()}, {os.cpu_count()} cpus visible, runs {a.runs}")
    cases = [(m, c) for m in exes for c in [(0, 1, 1)] + COSTS]
    walls = {k: [] for k in cases}
    rss = {k: 0 for k in cases}
    for _ in range(a.runs):
        for k in cases:
            mode, (n, r, p) = k
            code, wall, kib = child(exes[mode], [str(n), str(r), str(p)])
            if code != 0:
                print(f"FAIL cost-run {mode} {n} {r} {p} exit {code}")
                return 1
            walls[k].append(wall)
            rss[k] = max(rss[k], kib)
    print(f"{'build':8} {'N, r, p':12} {'min wall':>10} {'peak KiB':>10} {'V KiB':>8} {'peak - V':>9}")
    for k in cases:
        mode, (n, r, p) = k
        label = "floor" if n == 0 else f"2^{n}, {r}, {p}"
        v = 0 if n == 0 else 128 * r * (1 << n) // 1024
        print(f"{mode:8} {label:12} {min(walls[k]):9.3f}s {rss[k]:10d} {v:8d} {rss[k] - v:9d}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
