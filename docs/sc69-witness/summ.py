import json, sys, os
# summ.py <probe-outdir>: one line per machine per staged test, from the saved records.
out = sys.argv[1]
for n in sorted(os.listdir(out)):
    d = os.path.join(out, n)
    if not os.path.isdir(d):
        continue
    print("== " + n)
    for m in ("lupin", "checked", "native", "release"):
        p = os.path.join(d, m + ".json")
        if not os.path.exists(p):
            continue
        try:
            j = json.load(open(p))
        except Exception:
            err = open(os.path.join(d, m + ".err")).read()[-400:]
            print(f"  {m:8} NOJSON {open(p).read()[:200]!r} err={err!r}")
            continue
        u = j.get("x-unsupported") or j.get("x-unsupported-construct") or ""
        diag = j.get("diagnostics") or j.get("x-diagnostics") or ""
        s = j.get("stdout_inline") or ""
        print(f"  {m:8} {j.get('verdict')} " + (f"[{u}] " if u else "") + repr(s) + (f" diag={str(diag)[:300]}" if diag and not s else ""))
