#!/usr/bin/env python3
"""sc61's witness: std.term's round trip on a real pseudo-terminal.

usage: pty.py <wolf> <std-root> <workdir> [lupin]

Opens a pty pair and holds both ends, so the terminal's settings outlive
each program. For each machine — the native and release binaries
(`wolf build`, `wolf build --release`) and `wolf conform-run --json` with
`--checked`, `--native` and `--release` (the checked machine answers only
through conform-run) — it runs flow.lu's steps against the slave and reads
the slave from outside with `stty -a -F` before and after each:

  fresh    `stty sane ixon iexten min 1 time 0`, read
  trip     mode read, unpack, pack, set back      -> must equal fresh
  raw      raw(mode) set (what raw does, recorded)
  back     the mode saved before raw, set again   -> must equal fresh
  hand     unpack(287) set (flow and extended on by hand), recorded

A machine is GREEN when trip and back both give back icanon, echo, isig,
ixon, iexten, min and time exactly as fresh had them; RED otherwise.
With a fourth argument, lupin (`lupin conform-run --json`) runs `trip`
once and its answer is recorded (it refuses the terminal calls by name).
Exit 0 if every machine is GREEN, 1 if any is RED, 2 on a harness fault.
"""
import json
import os
import re
import shutil
import subprocess
import sys

FLAGS = ["icanon", "echo", "isig", "ixon", "iexten"]


def stty(path):
    r = subprocess.run(["stty", "-a", "-F", path], capture_output=True, text=True)
    if r.returncode != 0 or not r.stdout:
        print("HARNESS: stty failed:", r.stderr)
        sys.exit(2)
    toks = set(re.split(r"[\s;]+", r.stdout))
    st = {}
    for f in FLAGS:
        if f in toks:
            st[f] = True
        elif "-" + f in toks:
            st[f] = False
        else:
            print("HARNESS: stty shows neither", f, "nor -" + f)
            sys.exit(2)
    for k in ("min", "time"):
        m = re.search(r"\b" + k + r" = (\d+)", r.stdout)
        if not m:
            print("HARNESS: no", k)
            sys.exit(2)
        st[k] = int(m.group(1))
    return st


def show(st):
    fl = " ".join(f if st[f] else "-" + f for f in FLAGS)
    return f"{fl} min={st['min']} time={st['time']}"


def main():
    if len(sys.argv) < 4:
        print(__doc__)
        sys.exit(2)
    wolf, std, work = (os.path.abspath(a) for a in sys.argv[1:4])
    lupin = os.path.abspath(sys.argv[4]) if len(sys.argv) > 4 else None
    here = os.path.dirname(os.path.abspath(__file__))
    master, slave = os.openpty()
    path = os.ttyname(slave)
    os.makedirs(work, exist_ok=True)
    src = os.path.join(work, "src")
    shutil.rmtree(src, ignore_errors=True)
    os.makedirs(src)
    shutil.copy(os.path.join(here, "flow.lu"), os.path.join(src, "entry.lu"))
    ver = subprocess.run([wolf, "--version"], capture_output=True, text=True).stdout.splitlines()[0]
    print(f"wolf: {ver}")
    print(f"std:  {std} (STD-REV {open(os.path.join(std, 'STD-REV')).read().strip() if os.path.exists(os.path.join(std, 'STD-REV')) else 'none'})")
    print(f"tty:  {path}")

    def build(flag):
        out = os.path.join(work, "flow" + (flag or "").replace("-", "_"))
        cmd = [wolf, "build", os.path.join(src, "entry.lu"), "-o", out, "--std-root", std]
        if flag:
            cmd.append(flag)
        r = subprocess.run(cmd, capture_output=True, text=True)
        if r.returncode != 0:
            print("HARNESS: build failed", r.returncode, r.stderr[-2000:])
            sys.exit(2)
        return out

    machines = []
    nb = build(None)
    rb = build("--release")
    machines.append(("native-bin", lambda env: run_bin(nb, env)))
    machines.append(("release-bin", lambda env: run_bin(rb, env)))
    for t in ("checked", "native", "release"):
        machines.append((t, (lambda t: lambda env: run_conform(
            [wolf, "conform-run", "entry.lu", "--json", "--" + t, "--std-root", std], src, env))(t)))

    reds = 0
    for name, run in machines:
        subprocess.run(["stty", "-F", path, "sane", "ixon", "iexten", "min", "1", "time", "0"], check=True)
        fresh = stty(path)
        print(f"\n[{name}] fresh  {show(fresh)}")
        ok = True
        saved = None
        for step in ("trip", "raw", "back", "hand"):
            env = dict(os.environ, SC61_TTY=path, SC61_STEP=step)
            if step == "back":
                if saved is None:
                    print(f"[{name}] back   HARNESS: raw printed no saved=")
                    sys.exit(2)
                env["SC61_SAVED"] = saved
            if step == "hand":
                subprocess.run(["stty", "-F", path, "sane", "ixon", "iexten", "min", "1", "time", "0"], check=True)
            rc, out = run(env)
            after = stty(path)
            m = re.search(r"saved=(\d+)", out)
            if step == "raw" and m:
                saved = m.group(1)
            same = after == fresh
            mark = ""
            if step in ("trip", "back"):
                mark = "ok" if (same and rc == 0) else "RED"
                ok = ok and same and rc == 0
            print(f"[{name}] {step:6} {show(after)}  rc={rc} out={out.strip()!r} {mark}")
        print(f"[{name}] {'GREEN' if ok else 'RED'}")
        reds += 0 if ok else 1

    if lupin:
        subprocess.run(["stty", "-F", path, "sane", "ixon", "iexten", "min", "1", "time", "0"], check=True)
        env = dict(os.environ, SC61_TTY=path, SC61_STEP="trip")
        rc, out = run_conform([lupin, "conform-run", "--std-root", std, "entry.lu", "--json"], src, env)
        print(f"\n[lupin] trip   {show(stty(path))}  rc={rc} {out.strip()!r}")
    os.close(master)
    os.close(slave)
    print(f"\nWITNESS {'GREEN' if reds == 0 else 'RED'} ({reds} machine(s) red of {len(machines)})")
    sys.exit(0 if reds == 0 else 1)


def run_bin(exe, env):
    r = subprocess.run([exe], capture_output=True, text=True, env=env, stdin=subprocess.DEVNULL, timeout=60)
    return r.returncode, r.stdout


def run_conform(cmd, cwd, env):
    r = subprocess.run(cmd, capture_output=True, text=True, env=env, cwd=cwd, stdin=subprocess.DEVNULL, timeout=300)
    try:
        j = json.loads(r.stdout)
    except ValueError:
        return r.returncode, "NOJSON " + r.stdout[-300:] + r.stderr[-300:]
    out = j.get("stdout_inline") or ""
    extra = j.get("x-unsupported") or j.get("x-unsupported-construct") or ""
    verdict = j.get("verdict")
    code = 0 if verdict == "exit(0)" else 1
    tail = f" [{verdict}{' ' + extra if extra else ''}]"
    return code, out + tail


if __name__ == "__main__":
    main()
