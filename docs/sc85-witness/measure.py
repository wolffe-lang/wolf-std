#!/usr/bin/env python3
"""One bounded native measurement; no retries and no timer-resolution changes."""
import collections
import ctypes
import os
from pathlib import Path
import shutil
import socket
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]


def summarize(samples):
    for path, values in sorted(samples.items()):
        values = sorted(values)
        print(f"distribution path={path} n={len(values)} min={values[0]} "
              f"p25={values[(len(values)-1)//4]} median={values[(len(values)-1)//2]} "
              f"p75={values[3*(len(values)-1)//4]} max={values[-1]} "
              f"early={sum(x < 25_000_000 for x in values)} unit=ns", flush=True)


def main():
    stage = ROOT / "target" / "sc85-measure"
    stage.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(ROOT / "docs/sc85-witness/deadline-measure.lu", stage / "main.lu")
    exe = stage / ("probe.exe" if os.name == "nt" else "probe")
    wolf = ROOT / ".wolf-bin" / ("wolf.exe" if os.name == "nt" else "wolf")
    env = dict(os.environ)
    env["WOLF_RT_LIB"] = str(ROOT / ".wolf-bin" / ("wolf_rt.lib" if os.name == "nt" else "libwolf_rt.a"))
    subprocess.run([str(wolf), "build", str(stage / "main.lu"), "--std-root", str(ROOT / "std"), "-o", str(exe)],
                   cwd=stage, env=env, check=True, timeout=180)
    result = subprocess.run([str(exe)], cwd=stage, text=True, capture_output=True, timeout=90)
    print(result.stdout, end="", flush=True)
    print(result.stderr, end="", file=sys.stderr, flush=True)
    result.check_returncode()
    samples = collections.defaultdict(list)
    for line in result.stdout.splitlines():
        if line.startswith("sample "):
            fields = dict(s.split("=", 1) for s in line.split()[1:])
            assert int(fields["end"]) - int(fields["start"]) == int(fields["elapsed"])
            assert fields["ready"] == "0" and fields["meaning"] == "0", line
            samples[fields["path"]].append(int(fields["elapsed"]))
    for path in ("sockets", "empty", "net"):
        assert len(samples[path]) == 128, (path, len(samples[path]))
    summarize(samples)
    if os.name == "nt":
        windows_timers()


def windows_timers():
    from ctypes import wintypes
    kernel = ctypes.WinDLL("kernel32", use_last_error=True)
    adjustment, increment, disabled = wintypes.DWORD(), wintypes.DWORD(), wintypes.BOOL()
    assert kernel.GetSystemTimeAdjustment(ctypes.byref(adjustment), ctypes.byref(increment), ctypes.byref(disabled))
    freq = ctypes.c_longlong()
    assert kernel.QueryPerformanceFrequency(ctypes.byref(freq))
    print(f"windows clock_increment_100ns={increment.value} qpc_frequency_hz={freq.value} "
          f"python_clock={time.get_clock_info('perf_counter')}", flush=True)
    # Public WSAPoll API, independently invoked, not implementation source.
    class PollFd(ctypes.Structure):
        _fields_ = [("fd", ctypes.c_size_t), ("events", ctypes.c_short), ("revents", ctypes.c_short)]
    ws = ctypes.WinDLL("Ws2_32", use_last_error=True)
    ws.WSAPoll.argtypes = [ctypes.POINTER(PollFd), wintypes.ULONG, ctypes.c_int]
    ws.WSAPoll.restype = ctypes.c_int
    import select
    values = collections.defaultdict(list)
    with socket.socket() as listener:
        listener.bind(("127.0.0.1", 0))
        listener.listen()
        for i in range(128):
            for name in ("WSAPoll", "select", "Sleep"):
                fd = PollFd(listener.fileno(), 0x0100, 0)
                start = time.perf_counter_ns()
                if name == "WSAPoll":
                    assert ws.WSAPoll(ctypes.byref(fd), 1, 25) == 0
                elif name == "select":
                    assert select.select([listener], [], [], 0.025) == ([], [], [])
                else:
                    kernel.Sleep(25)
                end = time.perf_counter_ns()
                values[name].append(end - start)
                print(f"native_sample path={name} i={i} start={start} end={end} elapsed={end-start}", flush=True)
    summarize(values)


if __name__ == "__main__":
    main()
