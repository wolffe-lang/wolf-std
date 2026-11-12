#!/usr/bin/env python3
"""sc82's rows (std.poll), each on every lane, named line by line.

    python3 -I docs/sc82-witness/rows.py [TEST ...]

Runs each sc82 test (or the ones named, paths under tests/) the way the
rig does (`conform-run`, the checked machine with `--checked`, the native
one with `--native`, lupin bare; `--std-root` this checkout's std/; each
test alone in a directory of its own) and holds each lane to its ledger
cell for this host:

- `run`: the record's verdict is the directive's and its stdout is the
  directive's, line by line; a line that differs prints
  `FAIL <test>:<lane>:<key>` at column 0, where <key> is the line's text
  before its first `=` or `: `;
- `unsupported` on lupin: the verdict is `unsupported` and its refusal
  sentence (`x-unsupported`) names the builtin REFUSED_AT gives the row,
  so a refusal is checked by name, never by verdict alone.

Every other line is indented two spaces. Exit 0 when every lane of every
row holds, 1 otherwise. Reads WOLF_BIN and LUPIN_BIN (and the native
rung's runtime beside wolf or at WOLF_RT_LIB), and writes only under
TMPDIR.
"""
import json
import os
import re
import shutil
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

ROWS = [
    "poll/waiter_rows.lu",
    "poll/files_all_ready.lu",
    "poll/sockets_all_ready.lu",
    "poll/mixed_refused.lu",
    "poll/fairness.lu",
    "poll/deadline.lu",
    "poll/signal_during_wait.lu",
    "poll/write_partial.lu",
    "poll/pty_hangup.lu",
]

# The builtin each lupin refusal must name: the first call of the row that
# lupin 0.1.61 cannot serve (a pty, a file wait, a listen).
REFUSED_AT = {
    "poll/files_all_ready.lu": "os_term_open_pty",
    "poll/mixed_refused.lu": "os_term_open_pty",
    "poll/fairness.lu": "os_term_open_pty",
    "poll/deadline.lu": "os_signal_wait_fds",
    "poll/signal_during_wait.lu": "os_signal_listen",
    "poll/write_partial.lu": "os_term_open_pty",
    "poll/pty_hangup.lu": "os_term_open_pty",
}

LANES = [("lupin", "LUPIN_BIN", []), ("wolfc", "WOLF_BIN", ["--checked"]), ("native", "WOLF_BIN", ["--native"])]
CEILING = int(os.environ.get("STD_TEST_TIMEOUT_SECS", "180"))


def ledger_cells(test):
    text = open(os.path.join(ROOT, "tests/ledger.toml"), encoding="utf-8").read()
    m = re.search(r'^\[tests\."' + re.escape(test) + r'"\]\n((?:[^\[\n].*\n?)*)', text, re.M)
    if m is None:
        raise SystemExit(f"rows: no ledger entry for {test}")
    cells = {}
    for line in m[1].splitlines():
        kv = re.match(r'^([a-z-]+) = "([^"]*)"', line)
        if kv:
            cells[kv[1]] = kv[2]
    return cells


def directive(test):
    src = open(os.path.join(ROOT, "tests", test), encoding="utf-8").read()
    m = re.search(r'^//! check: (.*)$', src, re.M)
    check = m[1].strip()
    run = re.fullmatch(r'run\(exit=(\d+)(?:, stdout="(.*)")?\)', check)
    if run:
        out = run[2] if run[2] is not None else ""
        return f"exit({run[1]})", bytes(out, "utf-8").decode("unicode_escape").split("\n")
    fail = re.fullmatch(r'fail\((E\d{4})\)', check)
    if fail:
        return f"fail({fail[1]})", None
    raise SystemExit(f"rows: {test}: a check this runner does not read: {check}")


def key_of(line):
    m = re.match(r'([A-Za-z0-9_.-]+)(?:=|: )', line)
    return m[1] if m else "stdout"


def run_lane(test, lane, binvar, flags, scratch):
    stem = test.replace("/", "_")[:-3]
    d = os.path.join(scratch, lane, stem)
    shutil.rmtree(d, ignore_errors=True)
    os.makedirs(d)
    entry = os.path.join(d, os.path.basename(test))
    shutil.copy(os.path.join(ROOT, "tests", test), entry)
    cmd = [os.environ[binvar], "conform-run", *flags, "--std-root", os.path.join(ROOT, "std"), entry, "--json"]
    try:
        p = subprocess.run(cmd, cwd=d, capture_output=True, text=True, timeout=CEILING)
    except subprocess.TimeoutExpired:
        return None, f"timed out after {CEILING}s"
    if p.returncode != 0:
        return None, f"tool exit {p.returncode}: {p.stderr.strip()[-400:]}"
    try:
        return json.loads(p.stdout.strip()), ""
    except ValueError as e:
        return None, f"no record: {e}"


def main():
    tests = sys.argv[1:] or ROWS
    scratch = os.path.join(os.environ["TMPDIR"], "sc82-rows")
    windows = os.name == "nt"
    red = 0
    for test in tests:
        cells = ledger_cells(test)
        want_verdict, want_lines = directive(test)
        for lane, binvar, flags in LANES:
            cell = cells.get(lane + "-windows", cells[lane]) if windows else cells[lane]
            rec, why = run_lane(test, lane, binvar, flags, scratch)
            label = f"{test}:{lane}"
            if rec is None:
                print(f"FAIL {label}:record {why}")
                red += 1
                continue
            verdict = rec.get("verdict")
            if cell == "unsupported":
                reason = rec.get("x-unsupported") or rec.get("x-unsupported-construct") or ""
                named = REFUSED_AT.get(test)
                if verdict != "unsupported":
                    print(f"FAIL {label}:verdict the ledger says unsupported, observed {verdict}")
                    red += 1
                elif named is not None and f"`{named}`" not in reason:
                    print(f"FAIL {label}:refusal names no `{named}`: {reason}")
                    red += 1
                else:
                    print(f"  ok   {label}: unsupported, by name: {reason[:160]}")
                continue
            if cell != "run":
                print(f"FAIL {label}:cell this runner does not read: {cell}")
                red += 1
                continue
            if verdict != want_verdict:
                extra = rec.get("x-unsupported") or rec.get("x-unsupported-construct") or ""
                diags = "; ".join(str(x)[:200] for x in rec.get("diagnostics", [])[:2])
                print(f"FAIL {label}:verdict want {want_verdict}, observed {verdict} {extra} {diags}".rstrip())
                red += 1
                continue
            if want_lines is not None:
                got = (rec.get("stdout_inline") or "").rstrip("\n").split("\n")
                bad = 0
                for i in range(max(len(got), len(want_lines))):
                    w = want_lines[i] if i < len(want_lines) else "(none)"
                    g = got[i] if i < len(got) else "(none)"
                    if w != g:
                        print(f"FAIL {label}:{key_of(w if w != '(none)' else g)} want {w!r}, got {g!r}")
                        bad += 1
                if bad:
                    red += 1
                    continue
            if rec.get("warnings"):
                print(f"FAIL {label}:warnings {rec['warnings']}")
                red += 1
                continue
            print(f"  ok   {label}: {verdict}, {len(want_lines or [])} lines as the directive says")
    print(f"  rows: {len(tests)} tests, {red} red lane(s)")
    return 1 if red else 0


if __name__ == "__main__":
    sys.exit(main())
