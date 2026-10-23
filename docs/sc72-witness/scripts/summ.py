# sc72 summ.py LOG TESTDIR: compare probe.py's lines against each test's
# check directive; one compact line per lane.
import re, sys, ast
from pathlib import Path
log, tdir = sys.argv[1], Path(sys.argv[2])
pat = re.compile(r"^(\S+) \[(\w+)\] verdict=(\S+) stdout=(None|'.*?'|\".*?\") reason=(\{.*\}) stderr=(\[.*\])$")
for line in open(log):
    m = pat.match(line.strip())
    if not m: continue
    f, lane, v, out, reason, err = m.groups()
    src = (tdir / Path(f).name).read_text()
    d = re.search(r'^//! check: (.*)$', src, re.M).group(1)
    out = None if out == 'None' else ast.literal_eval(out)
    want = None
    em = re.match(r'run\(exit=(\d+)(?:, stdout="(.*)")?\)', d)
    if em:
        wexit = f'exit({em.group(1)})'
        wout = em.group(2)
        if wout is not None: wout = wout.encode().decode('unicode_escape')
        ok = v == wexit and (wout is None or (out or '').rstrip('\n') == wout.rstrip('\n'))
    else:
        ok = v == d
    try: r = ast.literal_eval(reason)
    except Exception: r = {}
    why = r.get('x-unsupported', '') if v == 'unsupported' else ''
    first_err = ''
    try:
        e = ast.literal_eval(err); first_err = e[0] if e else ''
    except Exception: pass
    tag = 'OK ' if ok else ('UNS' if v == 'unsupported' else 'BAD')
    extra = why[:110] if why else (first_err[:110] if not ok else '')
    print(f'{tag} {Path(f).name:45s} {lane:8s} {v:16s} {extra}')
    if not ok and v != 'unsupported' and out is not None:
        print('      got:', repr(out)[:600])
        print('     want:', repr(wout)[:600] if em else d)
