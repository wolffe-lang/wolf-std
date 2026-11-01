"""sc75: wolf-std#108's witness table. Runs one built witness at each push
count, reading wall time, peak resident set (the child's own rusage),
the bytes it printed and its stderr. Usage:
  python3 -I measure.py LABEL BINARY TIMEOUT N [N ...]
Prints one tab-separated row per N: label, pushes, MiB, bytes printed,
stderr, wall seconds, max resident MiB, outcome."""
import os, sys, time, signal, select

def run(binary, n, timeout):
    r_out, w_out = os.pipe()
    r_err, w_err = os.pipe()
    t0 = time.monotonic()
    pid = os.fork()
    if pid == 0:
        os.dup2(w_out, 1); os.dup2(w_err, 2)
        os.close(r_out); os.close(r_err); os.close(w_out); os.close(w_err)
        signal.signal(signal.SIGPIPE, signal.SIG_DFL)
        os.execv(binary, [binary, str(n)])
    os.close(w_out); os.close(w_err)
    nout, err = 0, b""
    open_fds = {r_out, r_err}
    killed = False
    while open_fds:
        left = timeout - (time.monotonic() - t0)
        if left <= 0 and not killed:
            os.kill(pid, signal.SIGKILL); killed = True
        ready, _, _ = select.select(list(open_fds), [], [], max(0.1, min(left, 5.0)) if left > 0 else 0.5)
        for fd in ready:
            chunk = os.read(fd, 1 << 20)
            if not chunk:
                open_fds.discard(fd); os.close(fd)
            elif fd == r_out:
                nout += len(chunk)
            else:
                err += chunk
    _, status, ru = os.wait4(pid, 0)
    wall = time.monotonic() - t0
    if os.WIFSIGNALED(status):
        sig = os.WTERMSIG(status)
        outcome = "timeout (killed at %ds)" % timeout if killed else "killed: %s, exit %d" % (signal.Signals(sig).name, 128 + sig)
    else:
        outcome = "exit %d" % os.WEXITSTATUS(status)
    rss = ru.ru_maxrss / (1024 * 1024) if sys.platform == "darwin" else ru.ru_maxrss / 1024
    return nout, err.decode(errors="replace").strip() or "(none)", wall, rss, outcome

label, binary, timeout = sys.argv[1], sys.argv[2], int(sys.argv[3])
for a in sys.argv[4:]:
    n = int(a)
    nout, err, wall, rss, outcome = run(binary, n, timeout)
    print("\t".join([label, str(n), "%g" % (n * 64 / 1048576), str(nout), err, "%.2f" % wall, "%.0f" % rss, outcome]), flush=True)
