#!/usr/bin/env python3
"""Derive committed Wolf tables and exhaustive fixtures from pinned UCD data.

Use Python -I. This reads data only, never third-party implementation source.
The five compatibility classes are black-box observations, not UCD properties.
"""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
LIMIT = 0x110000
GCB = {name: i for i, name in enumerate((
    'Other', 'CR', 'LF', 'Control', 'Extend', 'ZWJ', 'Regional_Indicator',
    'Prepend', 'SpacingMark', 'L', 'V', 'T', 'LV', 'LVT'))}
INCB = {'None': 0, 'Consonant': 1, 'Extend': 2, 'Linker': 3}
EXCEPTIONS = [
    ('soft-hyphen', [(0xAD, 0xAD)], 0, 1, 1),
    ('hangul-v-t-jamo', [(0x1160, 0x11FF), (0xD7B0, 0xD7C6),
                         (0xD7CB, 0xD7FB)], 1, 0, 232),
    ('format-controls', [(0xFFF9, 0xFFFB), (0x13430, 0x1343F)], 1, 0, 19),
    ('enclosed-numbers-and-hexagrams', [(0x3248, 0x324F), (0x4DC0, 0x4DFF)], 1, 2, 72),
    ('regional-indicators', [(0x1F1E6, 0x1F1FF)], 2, 1, 26),
]


def require(condition, message):
    if not condition:
        raise ValueError(message)


def sources():
    pin = json.loads((ROOT / 'vendor/unicode/sources.json').read_text())
    data = {}
    for source in pin['sources']:
        path = ROOT / 'vendor/unicode' / pin['version'] / source['name']
        raw = path.read_bytes()
        for content in (raw, path.read_bytes()):
            require(hashlib.sha256(content).hexdigest() == source['sha256'], source['name'])
        data[source['name']] = raw.decode('utf-8')
    return pin, data


def properties(text):
    for line in text.splitlines():
        body = line.partition('#')[0].strip()
        if body:
            fields = [f.strip() for f in body.split(';')]
            ends = fields[0].split('..')
            low, high = int(ends[0], 16), int(ends[-1], 16)
            require(0 <= low <= high < LIMIT, 'property bounds')
            yield low, high, fields[1:]


def categories(text):
    cats = ['Cn'] * LIMIT
    first = None
    for line in text.splitlines():
        fields = line.split(';')
        require(len(fields) == 15, 'UnicodeData field count')
        cp, name, category = int(fields[0], 16), fields[1], fields[2]
        if name.endswith(', First>'):
            require(first is None, 'nested range')
            first = (cp, category)
        elif name.endswith(', Last>'):
            require(first is not None and first[1] == category, 'unpaired range')
            cats[first[0]:cp + 1] = [category] * (cp - first[0] + 1)
            first = None
        else:
            require(first is None, 'unfinished range')
            cats[cp] = category
    require(first is None, 'unfinished final range')
    return cats


def ranges(values, default=None):
    result = []
    at = 0
    while at < len(values):
        end = at
        while end + 1 < len(values) and values[end + 1] == values[at]:
            end += 1
        if values[at] != default:
            result.append([at, end, values[at]])
        at = end + 1
    return result


def derive(data):
    cats = categories(data['UnicodeData.txt'])
    widths = [1] * LIMIT
    for name, wanted in [('EastAsianWidth.txt', ('W', 'F')),
                         ('emoji-data.txt', ('Emoji_Presentation',))]:
        for low, high, prop in properties(data[name]):
            if prop[0] in wanted:
                widths[low:high + 1] = [2] * (high - low + 1)
    for cp, cat in enumerate(cats):
        if cat in ('Mn', 'Me'):
            widths[cp] = 0
    for low, high, prop in properties(data['DerivedCoreProperties.txt']):
        if prop[0] == 'Default_Ignorable_Code_Point':
            for cp in range(low, high + 1):
                if cats[cp] == 'Cf':
                    widths[cp] = 0
    # Control and assignment policy is separate from the 350 exceptions.
    for cp, cat in enumerate(cats):
        if cat in ('Cn', 'Cs', 'Cc'):
            widths[cp] = -1
    widths[0] = 0
    exceptions = []
    for name, spans, before, after, count in EXCEPTIONS:
        points = [cp for low, high in spans for cp in range(low, high + 1)]
        require(len(points) == count, name + ' count')
        for cp in points:
            require(widths[cp] == before, name + ' base at U+%04X' % cp)
            widths[cp] = after
            exceptions.append({'scalar': 'U+%04X' % cp, 'class': name,
                               'base': before, 'width': after})
    require(len(exceptions) == 350 and len({e['scalar'] for e in exceptions}) == 350,
            'exception set')
    gcb, incb, ep = [0] * LIMIT, [0] * LIMIT, [0] * LIMIT
    for low, high, prop in properties(data['GraphemeBreakProperty.txt']):
        gcb[low:high + 1] = [GCB[prop[0]]] * (high - low + 1)
    for low, high, prop in properties(data['DerivedCoreProperties.txt']):
        if prop[0] == 'InCB':
            incb[low:high + 1] = [INCB[prop[1]]] * (high - low + 1)
    for low, high, prop in properties(data['emoji-data.txt']):
        if prop[0] == 'Extended_Pictographic':
            ep[low:high + 1] = [1] * (high - low + 1)
    assigned = [cat not in ('Cn', 'Cs') for cat in cats]
    require(sum(assigned) == 287346, 'assigned scalar census')
    return widths, assigned, gcb, incb, ep, exceptions


def literal(cp):
    return "'\\u{%X}'" % cp


def tree(spans, default, depth=1):
    pad = '    ' * depth
    if not spans:
        return [pad + 'return %d' % default]
    middle = len(spans) // 2
    low, high, value = spans[middle]
    return ([pad + 'if c < %s {' % literal(low)] + tree(spans[:middle], default, depth + 1)
            + [pad + '} else if c > %s {' % literal(high)]
            + tree(spans[middle + 1:], default, depth + 1)
            + [pad + '} else {', pad + '    return %d' % value, pad + '}'])


def table(widths, gcb, incb, ep):
    lines = ['//! member: true', '//! Generated by xtask/unicode/generate.py; do not edit.',
             '//! Unicode 15.1.0 data, copyright Unicode, Inc. Unicode-3.0.',
             '//! Internal data classifiers behind the core facade; see vendor/unicode.', '']
    for name, values, default, doc in [
        ('column_class', widths, 1, 'Width class: -1 nonprinting, otherwise 0, 1 or 2 columns.'),
        ('grapheme_class', gcb, 0, 'GCB class: Other=0, CR=1, LF=2, Control=3, Extend=4, ZWJ=5,'),
        ('conjunct_class', incb, 0, 'InCB class: None=0, Consonant=1, Extend=2, Linker=3.'),
        ('pictographic_class', ep, 0, 'Extended_Pictographic membership: 0 absent, 1 present.')]:
        lines += ['/// ' + doc]
        if name == 'grapheme_class':
            lines += ['/// RI=6, Prepend=7, SpacingMark=8, L=9, V=10, T=11, LV=12, LVT=13.']
        example = {'column_class': "tables.column_class('A') == 1",
                   'grapheme_class': "tables.grapheme_class('\\r') == 1",
                   'conjunct_class': "tables.conjunct_class('A') == 0",
                   'pictographic_class': "tables.pictographic_class('🐺') == 1"}[name]
        lines += ['///', '/// ```wolf-doc-example', '/// ' + example, '/// ```',
                  'pub fn %s(c: char) -> int {' % name]
        # Fast ASCII path keeps the very common case out of the large tree.
        if name == 'column_class':
            lines += ["    if c == '\\0' { return 0 }", "    if c < '\\u{20}' { return 0 - 1 }",
                      "    if c <= '~' { return 1 }"]
        spans = []
        for low, high, value in ranges(values, default):
            # The table is over char, never surrogate integers. Do not emit
            # a surrogate literal even when an unassigned interval includes it.
            if low < 0xD800:
                spans.append([low, min(high, 0xD7FF), value])
            if high > 0xDFFF:
                spans.append([max(low, 0xE000), high, value])
        lines += tree(spans, default)
        lines += ['}', '']
    return '\n'.join(lines)


def header(anchors):
    return ['//! check: run(exit=0)', '//! phase: run', '//! conforms: ' + anchors,
            '//! Generated by xtask/unicode/generate.py; do not edit.', '']


def scalar_tests(widths, assigned):
    # Each part enumerates every assigned scalar it owns, not just endpoints.
    points = [cp for cp in range(LIMIT) if assigned[cp]]
    for part, start in enumerate(range(0, len(points), 8192), 1):
        subset = points[start:start + 8192]
        lines = header('std.unicode, type.char') + ['use std.unicode', '',
            'fn same(low: int, high: int, want: int) -> bool {',
            '    var cp = low', '    while cp <= high {',
            '        let c = unicode.from_code(cp) else { return false }',
            '        if (unicode.display_width(c) else (0 - 1)) != want { return false }',
            '        cp += 1', '    }', '    true', '}', '', 'fn main() -> int {']
        at = 0
        while at < len(subset):
            end = at
            while end + 1 < len(subset) and subset[end + 1] == subset[end] + 1 \
                    and widths[subset[end + 1]] == widths[subset[at]]:
                end += 1
            lines += ['    assert(same(%d, %d, %d))' %
                      (subset[at], subset[end], widths[subset[at]] if widths[subset[at]] >= 0 else -1)]
            at = end + 1
        lines += ['    0', '}', '']
        yield 'tests/unicode/display_assigned_%02d.lu' % part, '\n'.join(lines)


def string(points):
    return '"' + ''.join('\\u{%X}' % cp for cp in points) + '"'


def grapheme_tests(text):
    cases = []
    for line in text.splitlines():
        tokens = line.partition('#')[0].split()
        if not tokens:
            continue
        require(tokens[0] == '÷' and tokens[-1] == '÷', 'grapheme test framing')
        clusters, current = [], []
        for token in tokens[1:]:
            if token == '÷':
                require(bool(current), 'empty corpus cluster')
                clusters.append(current)
                current = []
            elif token != '×':
                current.append(int(token, 16))
        cases.append(clusters)
    require(len(cases) == 1187, 'grapheme case census')
    for part, start in enumerate(range(0, len(cases), 64), 1):
        lines = header('std.str, type.char') + ['use std.str', '', 'fn main() -> int {']
        for index, clusters in enumerate(cases[start:start + 64], start + 1):
            name = 'case_%d' % index
            lines += ['    let %s = str.graphemes(%s)' %
                      (name, string([cp for c in clusters for cp in c])),
                      '    assert(%s.len == %d)' % (name, len(clusters))]
            lines += ['    assert(%s[%d] == %s)' % (name, n, string(c))
                      for n, c in enumerate(clusters)]
        yield 'tests/str/grapheme_ucd_%02d.lu' % part, '\n'.join(lines + ['    0', '}', ''])


def outputs():
    pin, data = sources()
    widths, assigned, gcb, incb, ep, exceptions = derive(data)
    result = {'std/unicode/tables/tables.lu': table(widths, gcb, incb, ep)}
    result.update(scalar_tests(widths, assigned))
    result.update(grapheme_tests(data['GraphemeBreakTest.txt']))
    metadata = {'version': pin['version'], 'assigned_scalars': sum(assigned),
                'width_ranges': ranges(widths), 'assigned_ranges': ranges(assigned, False),
                'exceptions': exceptions,
                'classification_ranges': {'width': len(ranges(widths, 1)),
                    'grapheme': len(ranges(gcb, 0)), 'conjunct': len(ranges(incb, 0)),
                    'pictographic': len(ranges(ep, 0))}}
    result['vendor/unicode/derived.json'] = json.dumps(metadata, indent=2) + '\n'
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    parser.add_argument('--wolf', type=Path)
    parser.add_argument('--presence-only', action='store_true')
    args = parser.parse_args()
    result = outputs()
    wanted = {name for name in result if name.endswith('.lu')}
    existing = {str(p.relative_to(ROOT)) for pattern in (
        'tests/unicode/display_assigned_*.lu', 'tests/str/grapheme_ucd_*.lu',
        'std/unicode/tables/*.lu') for p in ROOT.glob(pattern)}
    require(not (existing - wanted), 'stale generated files: ' + str(existing - wanted))
    # The formatter is the pinned binary, not a source dependency. Scratch is
    # inside target, never the host's shared temporary directory.
    scratch = ROOT / 'target/unicode-gen'
    scratch.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(dir=scratch) as directory:
        path = Path(directory) / 'generated.lu'
        for name, text in result.items():
            if name.endswith('.lu') and args.wolf:
                path.write_text(text)
                got = subprocess.run([str(args.wolf), 'fmt', str(path)], capture_output=True, timeout=60)
                require(got.returncode == 0 and b'W0301' not in got.stderr,
                        'formatter refused ' + name + ': ' + got.stderr.decode())
                text = path.read_text()
            destination = ROOT / name
            if args.check:
                require(destination.is_file(), 'missing ' + name)
                if not args.presence_only or not name.endswith('.lu'):
                    require(destination.read_text() == text, 'generated drift: ' + name)
            else:
                destination.parent.mkdir(parents=True, exist_ok=True)
                destination.write_text(text)
    print('PASS unicode-generation files=%d assigned=287346 grapheme-cases=1187 exceptions=350' % len(result))


if __name__ == '__main__':
    main()
