# sc72 probe.py: run probe programs on lupin, checked and native (and
# release when asked), each in its own staged root: the repo's std plus an
# overlay of probe modules. Runs INSIDE the container. Prints one line per
# lane: verdict, inline stdout, the first stderr line.
import json, os, shutil, subprocess, sys
from pathlib import Path
L = Path('/lane'); W = Path(os.environ['WOLF_BIN']); LU = Path(os.environ['LUPIN_BIN'])
STD = Path(os.environ.get('STD_SRC', '/lane/wolf-std/std'))
lanes = os.environ.get('LANES', 'lupin,checked,native').split(',')
extra = os.environ.get('EXTRA', '')
for probe in sys.argv[1:]:
    p = Path(probe)
    work = L / 'work' / p.parent.name / p.stem
    if work.exists(): shutil.rmtree(work)
    work.mkdir(parents=True)
    shutil.copytree(STD, work / 'std')
    ov = p.parent / 'std-overlay'
    if ov.exists():
        shutil.copytree(ov, work / 'std', dirs_exist_ok=True)
    shutil.copy(p, work / p.name)
    for lane in lanes:
        if lane == 'lupin':
            cmd = [str(LU), 'conform-run', '--std-root', str(work / 'std'), str(work / p.name), '--json']
        else:
            cmd = [str(W), 'conform-run', '--' + lane, '--std-root', str(work / 'std'), str(work / p.name), '--json']
        if extra: cmd += extra.split()
        r = subprocess.run(cmd, cwd=work, capture_output=True, timeout=300)
        try:
            rec = json.loads(r.stdout)
            v = rec.get('verdict'); out = rec.get('stdout_inline'); reason = {k: rec[k] for k in rec if k not in ('verdict','stdout_inline','stdout_sha256','impl','version','phase','file','seed','warnings') and rec[k] not in (None, [], '')}
        except Exception:
            v = f'NO-RECORD(exit {r.returncode})'; out = r.stdout[:300]; reason = ''
        err = r.stderr.decode(errors='replace').strip().splitlines()
        first = [l for l in err if l.strip()][:3]
        print(f'{p.parent.name}/{p.name} [{lane}] verdict={v} stdout={out!r} reason={reason!r} stderr={first!r}', flush=True)
print('PROBE_DONE')
