#!/usr/bin/env python3
"""Measure same-profile native consumer sizes; source bytes are not binary bytes."""
import json
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]


def main():
    out = Path(os.environ['WIDTH_OUT']) / 'size'
    out.mkdir(parents=True, exist_ok=True)
    prefix = ('//! member: false\nuse std.env\n')
    setup = ('    let args = env.args()\n'
             '    let text = if args.len > 1 { args[1] } else { "A" }\n')
    sources = {
        'control': prefix + '\nfn main() -> int {\n' + setup + '    text.len\n}\n',
        'scalar': prefix + 'use std.unicode\n\nfn main() -> int {\n' + setup +
                  '    for c in text.chars() { return unicode.display_width(c) else 99 }\n    0\n}\n',
        'string': prefix + 'use std.str\n\nfn main() -> int {\n' + setup +
                  '    let cut = str.truncate_display(text, 8) else ""\n'
                  '    let padded = str.pad_display_right(cut, 8) else ""\n'
                  '    str.display_width(padded) else 99\n}\n'}
    rows = {}
    for name, text in sources.items():
        source = out / (name + '.lu')
        source.write_text(text)
        row = {}
        for mode, suffix in [('executable', '.bin'), ('object', '.o')]:
            target = out / (name + suffix)
            command = [os.environ['WOLF_BIN'], 'build', '--std-root', str(ROOT / 'std'),
                       str(source), '--no-cache', '-o', str(target)]
            if mode == 'object':
                command += ['--emit=obj']
            result = subprocess.run(command, capture_output=True, text=True, timeout=120)
            if result.returncode:
                raise ValueError('size build ' + name + ': ' + result.stdout + result.stderr)
            artifacts = [target] if target.is_file() else []
            if mode == 'object':
                artifacts += sorted(out.glob(name + '.*.o'))
            if not artifacts:
                raise ValueError('size build emitted no ' + mode + ' for ' + name)
            row[mode + '_bytes'] = sum(p.stat().st_size for p in artifacts)
            row[mode + '_sections'] = {p.name: subprocess.check_output(
                ['size', '-A', str(p)], text=True) for p in artifacts}
            if mode == 'object':
                symbols = '\n'.join(subprocess.check_output(
                    ['nm', '-S', '--size-sort', str(p)], text=True) for p in artifacts)
                row['table_symbols'] = [line for line in symbols.splitlines()
                    if any(word in line for word in ('column_class', 'grapheme_class',
                                                      'conjunct_class', 'pictographic_class'))]
            else:
                expected = 8 if name == 'string' else 1
                observed = subprocess.run([str(target), 'A'], capture_output=True, timeout=60)
                if observed.returncode != expected:
                    raise ValueError('size consumer behavior differs: ' + name)
        rows[name] = row
    rows['generated_source_bytes'] = (ROOT / 'std/unicode/tables/tables.lu').stat().st_size
    rows['compiler'] = subprocess.check_output([os.environ['WOLF_BIN'], '--version'], text=True).splitlines()[0]
    rows['profile'] = 'default, same target and compiler for all three consumers'
    (out / 'result.json').write_text(json.dumps(rows, indent=2) + '\n')
    print('PASS width-binary-size ' + json.dumps(rows, sort_keys=True), flush=True)


if __name__ == '__main__':
    main()
