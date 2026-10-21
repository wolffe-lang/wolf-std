#!/usr/bin/env python3
"""sc69's witness: std.term's bits 5-7 and restore_at_exit on a real pseudo-terminal.

usage: pty.py <wolf> <std-root> <workdir> <mode> [lupin]
  mode `trip`: trip.lu only (runs against any std: trunk e036bec's and the head's)
  mode `all`:  trip.lu, then restore.lu (head std only)

Opens a pty pair and holds both ends, so the terminal's settings outlive
each program. For each machine — the native and release binaries
(`wolf build`, `wolf build --release`) and `wolf conform-run --json` with
`--checked`, `--native` and `--release` (the checked machine answers only
through conform-run) — it runs each step against the slave and reads the
slave from outside with `stty -a -F` before and after:

  usual    `stty sane ixon iexten min 1 time 0 icrnl opost -istrip`
  unusual  the same, then `-icrnl -opost istrip`           (bits 5-7 all set)

  trip      (both states) mode read, unpack, pack, set back -> must equal the state
  back      (both states) the mode the state reads, set     -> must equal the state
  raw       (all mode) raw_bytes(mode) set: recorded (icrnl and opost off, istrip off)
  restore, restore_exit, restore_trap (all mode, both states): register the mode
            read, set raw_bytes, end by a return / exit(3) / a bounds trap ->
            the terminal after must equal the state, and the pty's master must
            have received `SC69-RESTORED`

A machine is GREEN when every `must` holds; RED otherwise. With a fifth
argument lupin (`lupin conform-run --json`) runs `trip` once and its answer
is recorded (it refuses the terminal calls by name). Exit 0 if every machine
is GREEN, 1 if any is RED, 2 on a harness fault.
"""
import json
import os
import re
import select
import shutil
import subprocess
import sys

FLAGS = ["icanon", "echo", "isig", "ixon", "iexten", "icrnl", "opost", "istrip"]


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


def drain(master):
    out = b""
    while True:
        r, _, _ = select.select([master], [], [], 0.2)
        if not r:
            return out
        try:
            chunk = os.read(master, 4096)
        except OSError:
            return out
        if not chunk:
            return out
        out += chunk


def put(path, state):
    # `sane` sets icrnl and opost but leaves istrip as found (GNU stty), so
    # the usual state names all three.
    base = ["stty", "-F", path, "sane", "ixon", "iexten", "min", "1", "time", "0"]
    if state == "unusual":
        base += ["-icrnl", "-opost", "istrip"]
    else:
        base += ["icrnl", "opost", "-istrip"]
    subprocess.run(base, check=True)


def main():
    if len(sys.argv) < 5:
        print(__doc__)
        sys.exit(2)
    wolf, std, work = (os.path.abspath(a) for a in sys.argv[1:4])
    mode = sys.argv[4]
    lupin = os.path.abspath(sys.argv[5]) if len(sys.argv) > 5 else None
    here = os.path.dirname(os.path.abspath(__file__))
    master, slave = os.openpty()
    path = os.ttyname(slave)
    os.makedirs(work, exist_ok=True)
    ver = subprocess.run([wolf, "--version"], capture_output=True, text=True).stdout.splitlines()[0]
    rev = os.path.join(std, "STD-REV")
    print(f"wolf: {ver}")
    print(f"std:  {std} (STD-REV {open(rev).read().strip() if os.path.exists(rev) else 'none'})")
    print(f"tty:  {path}  mode: {mode}")
    progs = ["trip"] + (["restore"] if mode == "all" else [])
    srcs = {}
    for p in progs:
        src = os.path.join(work, "src-" + p)
        shutil.rmtree(src, ignore_errors=True)
        os.makedirs(src)
        shutil.copy(os.path.join(here, p + ".lu"), os.path.join(src, "entry.lu"))
        srcs[p] = src

    def build(p, flag):
        out = os.path.join(work, p + (flag or "").replace("-", "_"))
        cmd = [wolf, "build", os.path.join(srcs[p], "entry.lu"), "-o", out, "--std-root", std]
        if flag:
            cmd.append(flag)
        r = subprocess.run(cmd, capture_output=True, text=True)
        if r.returncode != 0:
            print(f"HARNESS: build of {p} failed", r.returncode, r.stderr[-2000:])
            sys.exit(2)
        return out

    runners = {}
    for p in progs:
        nb, rb = build(p, None), build(p, "--release")
        runners[(p, "native-bin")] = (lambda e, x=nb: run_bin(x, e))
        runners[(p, "release-bin")] = (lambda e, x=rb: run_bin(x, e))
        for t in ("checked", "native", "release"):
            runners[(p, t)] = (lambda e, t=t, p=p: run_conform(
                [wolf, "conform-run", "entry.lu", "--json", "--" + t, "--std-root", std], srcs[p], e))
    machines = ["native-bin", "release-bin", "checked", "native", "release"]
    reds = 0
    for name in machines:
        ok = True
        for state in ("usual", "unusual"):
            put(path, state)
            fresh = stty(path)
            print(f"\n[{name}] {state:7} fresh   {show(fresh)}")
            env = dict(os.environ, SC69_TTY=path, SC69_STEP="trip")
            drain(master)
            rc, out = runners[("trip", name)](env)
            after = stty(path)
            m = re.search(r"read=(\d+)", out)
            good = after == fresh and rc == 0
            ok = ok and good
            print(f"[{name}] {state:7} trip    {show(after)}  rc={rc} out={out.strip()!r} {'ok' if good else 'RED'}")
            if m:
                put(path, state)
                env = dict(os.environ, SC69_TTY=path, SC69_STEP="back", SC69_SAVED=m.group(1))
                rc, out = runners[("trip", name)](env)
                after = stty(path)
                good = after == fresh and rc == 0
                ok = ok and good
                print(f"[{name}] {state:7} back    {show(after)}  rc={rc} out={out.strip()!r} {'ok' if good else 'RED'}")
            if mode != "all":
                continue
            put(path, state)
            env = dict(os.environ, SC69_TTY=path, SC69_STEP="raw")
            rc, out = runners[("restore", name)](env)
            after = stty(path)
            print(f"[{name}] {state:7} raw     {show(after)}  rc={rc} out={out.strip()!r}")
            for step, want_rc in (("restore", 0), ("restore_exit", 3), ("restore_trap", None)):
                put(path, state)
                drain(master)
                env = dict(os.environ, SC69_TTY=path, SC69_STEP=step)
                rc, out = runners[("restore", name)](env)
                wire = drain(master)
                after = stty(path)
                rc_ok = (rc == want_rc) if want_rc is not None else (rc not in (0, 3))
                good = after == fresh and b"SC69-RESTORED" in wire and rc_ok
                ok = ok and good
                print(f"[{name}] {state:7} {step:12} {show(after)}  rc={rc} wire={wire!r} out={out.strip()!r} {'ok' if good else 'RED'}")
        print(f"[{name}] {'GREEN' if ok else 'RED'}")
        reds += 0 if ok else 1
    if lupin:
        put(path, "unusual")
        env = dict(os.environ, SC69_TTY=path, SC69_STEP="trip")
        rc, out = run_conform([lupin, "conform-run", "--std-root", std, "entry.lu", "--json"], srcs["trip"], env)
        print(f"\n[lupin] trip   {show(stty(path))}  rc={rc} {out.strip()!r}")
        if mode == "all":
            env = dict(os.environ, SC69_TTY=path, SC69_STEP="restore")
            rc, out = run_conform([lupin, "conform-run", "--std-root", std, "entry.lu", "--json"], srcs["restore"], env)
            print(f"[lupin] restore {show(stty(path))}  rc={rc} {out.strip()!r}")
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
    verdict = j.get("verdict") or ""
    m = re.match(r"exit\((\d+)\)", verdict)
    code = int(m.group(1)) if m else (134 if verdict.startswith("trap") else 1)
    tail = f" [{verdict}{' ' + extra if extra else ''}]"
    return code, out + tail


if __name__ == "__main__":
    main()
