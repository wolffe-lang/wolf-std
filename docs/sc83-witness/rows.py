#!/usr/bin/env python3
"""Run sc83's predicted surface independently of the not-yet-updated ledger.

Use WOLF_BIN, LUPIN_BIN and an explicit TMPDIR. Runs are sequential, each
row/lane in its own directory. The final gauntlet also runs std-test, whose
ledger classification is deliberately a separate, measured gate. A refused
reference run must reach run and name the builtin, not merely say unsupported.
"""
import argparse
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
REFUSALS = {
    "poll/waiter_rows.lu": "os_ready_open",
    "poll/files_all_ready.lu": "os_term_open_pty",
    "poll/sockets_all_ready.lu": "os_ready_open",
    "poll/mixed_refused.lu": "os_term_open_pty",
    "poll/fairness.lu": "os_term_open_pty",
    "poll/deadline.lu": "os_ready_open",
    "poll/signal_during_wait.lu": "os_signal_listen",
    "poll/write_partial.lu": "os_term_open_pty",
    "poll/pty_hangup.lu": "os_term_open_pty",
    "poll/capacity.lu": "os_ready_open",
    "poll/set_mutation.lu": "os_ready_open",
    "poll/error_hangup.lu": "os_ready_open",
    "poll/signal_socket.lu": "os_signal_listen",
    "poll/socket_queue.lu": "os_ready_open",
    "net/unix/peer_credentials.lu": "net_peer_cred",
    "net/unix/peer_credentials_child.lu": "os_fork",
}
ROWS = list(REFUSALS) + [
    "net/write_some.lu",
    "net/write_some_invalid_row.lu",
    "poll/comptime_refuses.lu",
    "net/comptime_refuses_write_some.lu",
    "net/unix/comptime_refuses_credentials.lu",
]
LANES = {
    "lupin": ("LUPIN_BIN", []),
    "wolfc": ("WOLF_BIN", ["--checked"]),
    "native": ("WOLF_BIN", ["--native"]),
}
# These rows take a named host refusal or explicit Windows SKIP before
# reaching an operation the reference machine declines. signal_during_wait
# still reaches os_signal_listen on every host, as its existing row does.
WINDOWS_RUN = set(REFUSALS) - {"poll/signal_during_wait.lu"}


def expectation(source):
    check = re.search(r"^//! check: (.*)$", source, re.M)[1]
    run = re.fullmatch(r'run\(exit=(\d+), stdout="(.*)"\)', check)
    if run:
        lines = bytes(run[2], "utf8").decode("unicode_escape").splitlines()
        return f"exit({run[1]})", lines
    reject = re.fullmatch(r"fail\((E\d{4})\)", check)
    if reject:
        return f"fail({reject[1]})", None
    raise ValueError(f"unsupported directive shape: {check}")


def key(line):
    return re.split(r"=|: ", line, maxsplit=1)[0]


def expected_refusal(test, lane, windows=False):
    if test == "net/comptime_refuses_write_some.lu" and lane in ("wolfc", "native"):
        # [type.list.lit.value], current-edition scalar CTFE: argument
        # construction fails before the send's own comptime barrier.
        return "resolve", "`List` construction at comptime"
    if lane == "lupin" and test in {
        "poll/comptime_refuses.lu", "net/comptime_refuses_write_some.lu",
        "net/unix/comptime_refuses_credentials.lu",
    }:
        # Measured at released 0.1.62, not inferred from the compiler.
        return "run", "`probe` is a `comptime fn`"
    if lane == "lupin" and test in REFUSALS and not (windows and test in WINDOWS_RUN):
        return "run", f"`{REFUSALS[test]}`"
    return None


def matches_refusal(record, phase, named):
    reason = record.get("x-unsupported") or record.get("x-unsupported-construct") or ""
    # Lupin reports its last completed rung separately from the point of
    # refusal: released 0.1.62 has phase_reached=resolve and x-unsupported-at=run.
    at = record.get("x-unsupported-at") or record.get("phase_reached")
    return (record.get("verdict") == "unsupported" and at == phase
            and named in reason and not record.get("warnings"))


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("rows", nargs="*")
    ap.add_argument("--lanes", default="lupin,wolfc,native")
    ap.add_argument("--timeout", type=int, default=180)
    args = ap.parse_args()
    if not set(args.rows) <= set(ROWS):
        ap.error("unknown row (paths are relative to tests/)")
    lanes = args.lanes.split(",")
    if not set(lanes) <= set(LANES):
        ap.error("lanes must be lupin,wolfc,native")
    scratch = Path(os.environ["TMPDIR"]).resolve() / "sc83-rows" / str(time.time_ns())
    scratch.mkdir(parents=True)
    print(f"  observations: {scratch}", flush=True)
    failures = 0
    for test in args.rows or ROWS:
        source = (ROOT / "tests" / test).read_text()
        verdict, expected = expectation(source)
        for lane in lanes:
            label = f"{test}:{lane}"
            envvar, flags = LANES[lane]
            work = scratch / lane / test.replace("/", "_").removesuffix(".lu")
            work.mkdir(parents=True)
            entry = work / "main.lu"
            entry.write_text(source)
            command = [os.environ[envvar], "conform-run", *flags, "--std-root", str(ROOT / "std"), str(entry), "--json"]
            try:
                p = subprocess.run(command, cwd=work, capture_output=True, text=True, timeout=args.timeout)
                (work / "tool-stdout.txt").write_text(p.stdout)
                (work / "tool-stderr.txt").write_text(p.stderr)
                if p.returncode != 0:
                    raise ValueError(f"tool exit {p.returncode}: {p.stderr[-500:]}")
                record = json.loads(p.stdout.strip())
                (work / "record.json").write_text(json.dumps(record, indent=2))
            except (subprocess.TimeoutExpired, ValueError, OSError) as error:
                print(f"FAIL {label}:record {error}", flush=True)
                failures += 1
                continue
            for line in p.stderr.splitlines():
                print(f"  {label} stderr: {line}", flush=True)
            refusal = expected_refusal(test, lane, windows=os.name == "nt")
            if refusal is not None:
                phase, named = refusal
                reason = record.get("x-unsupported") or record.get("x-unsupported-construct") or ""
                if not matches_refusal(record, phase, named):
                    print(f"FAIL {label}:refusal expected {phase} refusal naming {named}, got {record}", flush=True)
                    failures += 1
                else:
                    print(f"  PASS {label}: unsupported at {phase} by name: {reason}", flush=True)
                continue
            if record.get("verdict") != verdict:
                print(f"FAIL {label}:verdict want {verdict}, got {record}", flush=True)
                failures += 1
                continue
            bad = 0
            if expected is not None:
                got = (record.get("stdout_inline") or "").splitlines()
                for i in range(max(len(expected), len(got))):
                    want = expected[i] if i < len(expected) else "(none)"
                    actual = got[i] if i < len(got) else "(none)"
                    if want != actual:
                        print(f"FAIL {label}:{key(want)} want {want!r}, got {actual!r}", flush=True)
                        bad += 1
            if record.get("warnings"):
                print(f"FAIL {label}:warnings {record['warnings']}", flush=True)
                bad += 1
            if bad:
                failures += 1
            else:
                print(f"  PASS {label}: {verdict}, directive satisfied", flush=True)
    print(f"  surface: {len(args.rows or ROWS)} rows, {failures} red lane cells", flush=True)
    return int(failures != 0)


if __name__ == "__main__":
    sys.exit(main())
