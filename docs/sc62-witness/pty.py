#!/usr/bin/env python3
"""sc62's witness: term.size, term.chars and signal.wait_fd on a real pty.

usage: pty.py <wolf> <std-root> <workdir> [lupin]

Opens a pty pair and holds both ends. Each program runs with the slave as
its standard input AND its controlling terminal (a new session, then
TIOCSCTTY), so its process group is the terminal's foreground group and a
window change sends it SIGWINCH, as an emulator's does. Machines: the
native and release binaries (`wolf build`, `wolf build --release`) and
`wolf conform-run --json` with `--checked`, `--native` and `--release`
(the checked machine answers only through conform-run). Steps:

  untold   size.lu on a pty never sized            -> rows=0 columns=0
  told     TIOCSWINSZ 24 x 80, size.lu             -> rows=24 columns=80
  chars    stty sane; intr ^X erase ^H quit undef  -> 24, 8, -1, the rest
  resize   wait.lu; after its mark, TIOCSWINSZ 30 x 100
                                                    -> answer=16384, 30 100
  key      wait.lu; after its mark, the byte `k` (-icanon min 1)
                                                    -> answer=0, read=k

Every machine must print EXPECT's text for every step. With a fourth
argument, lupin (`lupin conform-run --json`) runs each program once and
its refusal is recorded; each must be `unsupported` naming its builtin.
Exit 0 if every machine is GREEN, 1 if any is RED, 2 on a harness fault.
"""
import fcntl
import json
import os
import shutil
import struct
import subprocess
import sys
import termios
import time

EXPECT = {
    "untold": "rows=0 columns=0",
    "told": "rows=24 columns=80",
    "chars": "erase=8 kill=21 interrupt=24 quit=-1\n"
             "suspend=26 end_of_file=4 word_erase=23\n"
             "literal_next=22 start=17 stop=19\n"
             "reprint=18 discard=15",
    "resize": "answer=16384\nrows=30 columns=100",
    "key": "answer=0\nread=k",
}
PROGRAM = {"untold": "size", "told": "size", "chars": "chars", "resize": "wait", "key": "wait"}
LUPIN_NAMES = {"size": "os_term_size", "chars": "os_term_chars", "lupin_wait": "os_signal_wait_fd"}
# conform-run --native and --release give the program no terminal on
# standard input (measured, sc62: `io` from the size, `eof` from a read),
# so those two machines name the pty by path; the binaries and the
# checked machine read it as standard input.
BY_PATH = ("native", "release")


def winsz(fd, rows, cols):
    fcntl.ioctl(fd, termios.TIOCSWINSZ, struct.pack("HHHH", rows, cols, 0, 0))


def stty(path, *args):
    subprocess.run(["stty", "-F", path, *args], check=True)


def main():
    if len(sys.argv) < 4:
        print(__doc__)
        sys.exit(2)
    wolf, std, work = (os.path.abspath(a) for a in sys.argv[1:4])
    lupin = os.path.abspath(sys.argv[4]) if len(sys.argv) > 4 else None
    here = os.path.dirname(os.path.abspath(__file__))
    os.makedirs(work, exist_ok=True)
    ver = subprocess.run([wolf, "--version"], capture_output=True, text=True).stdout.splitlines()[0]
    rev = os.path.join(std, "STD-REV")
    print(f"wolf: {ver}")
    print(f"std:  {std} (STD-REV {open(rev).read().strip() if os.path.exists(rev) else 'none'})")
    print(f"affinity: {sorted(os.sched_getaffinity(0))}")
    for line in open("/proc/self/status"):
        if line.startswith(("SigBlk", "SigIgn")):
            print(line.strip())

    srcs, bins = {}, {}
    for p in ("size", "chars", "wait", "lupin_wait"):
        d = os.path.join(work, "src-" + p)
        shutil.rmtree(d, ignore_errors=True)
        os.makedirs(d)
        shutil.copy(os.path.join(here, p + ".lu"), os.path.join(d, "entry.lu"))
        srcs[p] = d
        for flag in ((None, "--release") if p != "lupin_wait" else ()):
            out = os.path.join(work, p + ("_release" if flag else "_native"))
            cmd = [wolf, "build", os.path.join(d, "entry.lu"), "-o", out, "--std-root", std]
            if flag:
                cmd.append(flag)
            r = subprocess.run(cmd, capture_output=True, text=True)
            if r.returncode != 0:
                print("HARNESS: build failed", p, flag, r.returncode, r.stderr[-2000:])
                sys.exit(2)
            bins[(p, flag)] = out

    machines = [
        ("native-bin", lambda p: ([bins[(p, None)]], None, False)),
        ("release-bin", lambda p: ([bins[(p, "--release")]], None, False)),
    ]
    for t in ("checked", "native", "release"):
        machines.append((t, (lambda t: lambda p: (
            [wolf, "conform-run", "entry.lu", "--json", "--" + t, "--std-root", std], srcs[p], True))(t)))

    reds = 0
    for name, cmd_of in machines:
        ok = True
        for step in ("untold", "told", "chars", "resize", "key"):
            master, slave = os.openpty()
            path = os.ttyname(slave)
            stty(path, "sane", "min", "1", "time", "0")
            if step == "told":
                winsz(master, 24, 80)
            if step == "chars":
                stty(path, "intr", "^X", "erase", "^H", "quit", "undef")
            if step in ("resize", "key"):
                winsz(master, 24, 80)
                stty(path, "-icanon", "min", "1", "time", "0")
            mark = os.path.join(work, f"mark-{name}-{step}")
            if os.path.exists(mark):
                os.remove(mark)
            argv, cwd, conform = cmd_of(PROGRAM[step])
            env = dict(os.environ, SC62_MARK=mark)
            if name in BY_PATH:
                env["SC62_TTY"] = path

            def child_setup(fd=slave):
                os.setsid()
                fcntl.ioctl(0, termios.TIOCSCTTY, 0)

            t0 = time.time()
            proc = subprocess.Popen(argv, cwd=cwd, env=env, stdin=slave, stdout=subprocess.PIPE,
                                    stderr=subprocess.PIPE, text=True, preexec_fn=child_setup)
            acted = ""
            if step in ("resize", "key"):
                deadline = time.time() + 240
                while not os.path.exists(mark) and time.time() < deadline and proc.poll() is None:
                    time.sleep(0.05)
                if not os.path.exists(mark):
                    acted = "NO-MARK "
                time.sleep(0.5)
                if step == "resize":
                    winsz(master, 30, 100)
                else:
                    os.write(master, b"k")
            try:
                out, err = proc.communicate(timeout=300)
            except subprocess.TimeoutExpired:
                proc.kill()
                out, err = proc.communicate()
                acted += "TIMEOUT "
            rc = proc.returncode
            if conform:
                try:
                    j = json.loads(out)
                    text = (j.get("stdout_inline") or "").strip()
                    verdict = j.get("verdict")
                    rc = 0 if verdict == "exit(0)" else 1
                    extra = j.get("x-unsupported") or ""
                    if verdict != "exit(0)":
                        text += f" [{verdict} {extra}]"
                except ValueError:
                    text = "NOJSON " + out[-300:] + err[-300:]
                    rc = 1
            else:
                text = out.strip()
            good = rc == 0 and text == EXPECT[step] and not acted
            ok = ok and good
            shown = text.replace("\n", " | ")
            print(f"[{name}] {step:6} rc={rc} {acted}{time.time() - t0:5.1f}s {shown!r} {'ok' if good else 'RED'}")
            os.close(master)
            os.close(slave)
        print(f"[{name}] {'GREEN' if ok else 'RED'}")
        reds += 0 if ok else 1

    lupin_ok = True
    if lupin:
        for p in ("size", "chars", "lupin_wait"):
            master, slave = os.openpty()
            r = subprocess.run([lupin, "conform-run", "--std-root", std, "entry.lu", "--json"],
                               cwd=srcs[p], stdin=slave, capture_output=True, text=True, timeout=300)
            os.close(master)
            os.close(slave)
            try:
                j = json.loads(r.stdout)
            except ValueError:
                print(f"[lupin] {p}: NOJSON {r.stdout[-200:]} {r.stderr[-200:]}")
                lupin_ok = False
                continue
            extra = j.get("x-unsupported") or ""
            named = j.get("verdict") == "unsupported" and f"`{LUPIN_NAMES[p]}`" in extra
            lupin_ok = lupin_ok and named
            print(f"[lupin] {p:10} {j.get('verdict')} {'names ' + LUPIN_NAMES[p] if named else 'RED'}: {extra}")
    print(f"\nWITNESS {'GREEN' if reds == 0 and lupin_ok else 'RED'} "
          f"({reds} machine(s) red of {len(machines)}; lupin {'by name' if lupin_ok else 'RED'})")
    sys.exit(0 if reds == 0 and lupin_ok else 1)


if __name__ == "__main__":
    main()
