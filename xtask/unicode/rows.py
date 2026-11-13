#!/usr/bin/env python3
"""Execute the complete width/segmentation rows, requiring all three lanes.

These are also ordinary ledgered std-test fixtures. This focused invocation
makes missing binaries and unsupported observations hard errors, not skips.
"""
import argparse
import json
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('family', choices=('assigned', 'grapheme', 'family'))
    args = parser.parse_args()
    patterns = {'assigned': ['tests/unicode/display_assigned_*.lu'],
                'grapheme': ['tests/str/grapheme_ucd_*.lu'],
                'family': ['tests/unicode/display_scalar.lu', 'tests/str/display_columns.lu',
                           'tests/str/display_*_trap.lu']}
    tests = sorted({p for pattern in patterns[args.family] for p in ROOT.glob(pattern)})
    expected_count = {'assigned': 36, 'grapheme': 19, 'family': 5}[args.family]
    if len(tests) != expected_count:
        raise ValueError('fixture census differs: ' + str(len(tests)))
    failures = []
    for name, variable, flags in [('native', 'WOLF_BIN', ['--native']),
                                  ('checked', 'WOLF_BIN', ['--checked']),
                                  ('lupin', 'LUPIN_BIN', [])]:
        binary = Path(os.environ[variable]).resolve(strict=True)
        for test in tests:
            want = 'trap(assert)' if test.name.endswith('_trap.lu') else 'exit(0)'
            command = [str(binary), 'conform-run', *flags, '--std-root', str(ROOT / 'std'),
                       str(test), '--json']
            got = subprocess.run(command, cwd=ROOT, capture_output=True, text=True, timeout=120)
            try:
                record = json.loads(got.stdout)
            except (ValueError, TypeError):
                failures.append('%s %s invalid record rc=%s %s %s' %
                                (name, test.name, got.returncode, got.stdout[:2000], got.stderr[:2000]))
                continue
            if got.returncode or record['verdict'] != want or record['warnings']:
                failures.append('%s %s want=%s got=%s stderr=%s' %
                                (name, test.name, want, record, got.stderr[:3000]))
            else:
                print('PASS width-%s %s %s' % (args.family, name, test.name), flush=True)
        print('MEASURE width-%s %s files=%d failures=%d' %
              (args.family, name, len(tests), len(failures)), flush=True)
    if failures:
        raise ValueError('\n'.join(failures))


if __name__ == '__main__':
    try:
        main()
    except Exception as error:
        print('FAIL width-' + (sys.argv[1] if len(sys.argv) > 1 else 'rows') + ' ' + str(error), flush=True)
        sys.exit(1)
