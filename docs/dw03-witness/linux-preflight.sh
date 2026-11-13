#!/usr/bin/env bash
set -eu
: "${DW03_ROOT:?}" "${DW03_TOOLS:?}" "${DW03_OUT:?}" "${DW03_IMAGE:?}" "${DW03_CARGO_HOME:?}"
python3 -I - "$DW03_ROOT" <<'PY'
import os
from pathlib import Path
import shutil
import shlex
import subprocess
import sys
root=Path(sys.argv[1])
assert shutil.disk_usage(root).free >= 35*1024**3, 'disk floor 35 GiB'
mem={x.split(':')[0]:int(x.split()[1]) for x in Path('/proc/meminfo').read_text().splitlines()}
assert mem['MemAvailable'] >= 8*1024**2, 'memory floor 8 GiB'
assert not subprocess.check_output(['git','-C',str(root),'status','--porcelain'],text=True).strip(), 'dirty snapshot'
for file in (root/'docs/dw03-witness').iterdir():
 if file.suffix in ['.py','.sh','.manifest']:
  tracked=subprocess.check_output(['git','-C',str(root),'show','HEAD:'+str(file.relative_to(root))])
  assert file.read_bytes()==tracked, 'rig drift '+file.name
print('PASS clean committed rigs and host floors', flush=True)
manifest=root/'docs/dw03-witness/linux.manifest'
lines=[shlex.split(line, comments=True) for line in manifest.read_text().splitlines()]
engine=next(words[1:] for words in lines if words and words[0]=='engine')
# Match the recorder: engine paths containing '/' resolve at the MANIFEST,
# not at the checkout or at the current working directory.
if '/' in engine[0]:
 engine[0]=str((manifest.parent/engine[0]).resolve())
 assert Path(engine[0]).is_file() and os.access(engine[0], os.X_OK), 'manifest engine missing or not executable: '+engine[0]
images={word.split('=',1)[1] for words in lines if words and words[0]=='step' for word in words[1:] if word.startswith('image=')}
assert images, 'manifest has no declared images'
for image in sorted(images):
 actual=subprocess.check_output([engine[0], 'image', 'inspect', image, '--format', '{{.Id}}'], text=True).strip()
 assert actual.removeprefix('sha256:')==os.environ['DW03_IMAGE'].removeprefix('sha256:'), 'manifest image differs from the pinned take image'
 print('PASS manifest engine image-known '+image+' id='+actual, flush=True)
 probe='for t in bash python3 git clang gcc cargo rustc rustfmt cargo-clippy timeout prlimit nm objdump setsid; do command -v "$t" >/dev/null || exit 1; done; python3 -c "import tomllib"; cargo --version; rustc --version'
 subprocess.run([engine[0], 'run', '--rm', '--network', 'none', '--memory', '256m', *engine[1:], actual, 'bash', '-c', probe], check=True)
print('PASS resolved manifest engine and declared image tool probe', flush=True)
cargo_home=Path(os.environ['DW03_CARGO_HOME']).resolve()
assert (cargo_home/'registry').is_dir(), 'prepare the lane-owned Cargo home before the take'
for image in sorted(images):
 subprocess.run([engine[0], 'run', '--rm', '--network', 'none', '--memory', '256m', *engine[1:], '--volume', str(root)+':'+str(root), '--workdir', str(root), '--volume', str(cargo_home)+':'+str(cargo_home)+':ro', '--env', 'CARGO_HOME='+str(cargo_home), '--env', 'CARGO_NET_OFFLINE=true', image, 'cargo', 'metadata', '--offline', '--locked', '--format-version', '1'], stdout=subprocess.DEVNULL, check=True)
print('PASS build dependencies resolve offline inside the declared engine', flush=True)
PY
test -x "$DW03_TOOLS/wolf/wolf"
test -r "$DW03_TOOLS/wolf/libwolf_rt.a"
test -x "$DW03_TOOLS/lupin/lupin"
printf 'PASS Linux preflight only; no build or test has run\n'
