#!/usr/bin/env bash
set -eu
: "${DW03_ROOT:?}" "${DW03_TOOLS:?}" "${DW03_OUT:?}" "${DW03_IMAGE:?}"
python3 -I - "$DW03_ROOT" <<'PY'
import os
from pathlib import Path
import shutil
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
print('PASS clean committed rigs and host floors')
PY
podman run --rm --network none --memory 256m --env PATH=/opt/s263-rust/bin:/usr/local/bin:/usr/bin:/bin "$DW03_IMAGE" bash -c 'for t in bash python3 git clang gcc cargo rustc rustfmt cargo-clippy timeout prlimit nm objdump setsid; do command -v "$t" >/dev/null || exit 1; done; python3 -c "import tomllib"; cargo --version; rustc --version'
test -x "$DW03_TOOLS/wolf/wolf"
test -r "$DW03_TOOLS/wolf/libwolf_rt.a"
test -x "$DW03_TOOLS/lupin/lupin"
printf 'PASS Linux preflight only; no build or test has run\n'
