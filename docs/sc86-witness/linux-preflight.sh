#!/usr/bin/env bash
set -eu
: "${WIDTH_ROOT:?}" "${WIDTH_TOOLS:?}" "${WIDTH_OUT:?}" "${WIDTH_ENGINE:?}"
python3 -I - "$WIDTH_ROOT" "$WIDTH_TOOLS" "$WIDTH_ENGINE" <<'PY'
import hashlib
from pathlib import Path
import subprocess
import sys
root, tools, engine = Path(sys.argv[1]), Path(sys.argv[2]), sys.argv[3]
expected = {'wolf/wolf': 'f4e785da2ce111929f1dbd0ea313441f247b2d5ad016c9417d6cf95cf48ab1de',
            'wolf/libwolf_rt.a': '2bdfd7cdac7d2f169bf19bbfd3bf29a788476ecc3c3a100e9a4cb1b89f64b1e7',
            'lupin': '1aff4d2a0681ea20ca6469e4b6fe5e8bfdf3bcc124c445a2d7ce56d74a80eeb7'}
for name, digest in expected.items():
    for _ in range(2):
        if hashlib.sha256((tools / name).read_bytes()).hexdigest() != digest:
            raise ValueError('tool hash mismatch: ' + name)
image = subprocess.check_output([engine, 'image', 'inspect', 'localhost/r41-toolchain:1', '--format', '{{.Id}}'], text=True).strip()
if image != '77fdad6b841abfdb3968ea6137b1edf7248578bdfc4f92e4aab6b2a05f241485':
    raise ValueError('execution image mismatch: ' + image)
print('PASS width-preflight binary hashes twice; execution image=' + image)
PY
