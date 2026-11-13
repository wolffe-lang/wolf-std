#!/usr/bin/env python3
"""Generator invariants, including a corrupted-input red witness."""
from pathlib import Path
import re
import runpy
import shutil
import tempfile
import unittest
from unittest.mock import patch

GEN = runpy.run_path(str(Path(__file__).with_name('generate.py')))


class Generation(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.pin, cls.data = GEN['sources']()
        cls.derived = GEN['derive'](cls.data)

    def test_assigned_and_nonprinting(self):
        widths, assigned, *_ = self.derived
        self.assertEqual(sum(assigned), 287346)
        self.assertEqual(widths[0], 0)
        for cp in (9, 0x7F, 0x2028, 0x2029, 0x378, 0xFFFF, 0x10FFFF):
            self.assertEqual(widths[cp], -1, hex(cp))
        for cp in (0xE000, 0xF0000, 0x10FFFD):
            self.assertEqual(widths[cp], 1, hex(cp))

    def test_exact_350_exceptions(self):
        widths, _, _, _, _, exceptions = self.derived
        self.assertEqual(len(exceptions), 350)
        self.assertEqual(len({e['scalar'] for e in exceptions}), 350)
        for name, spans, before, after, count in GEN['EXCEPTIONS']:
            selected = [e for e in exceptions if e['class'] == name]
            self.assertEqual(len(selected), count)
            self.assertEqual({(e['base'], e['width']) for e in selected}, {(before, after)})
            self.assertEqual({int(e['scalar'][2:], 16) for e in selected},
                             {cp for low, high in spans for cp in range(low, high + 1)})
            for row in selected:
                self.assertEqual(widths[int(row['scalar'][2:], 16)], after)

    def test_generated_char_literals_are_scalars(self):
        widths, _, gcb, incb, ep, _ = self.derived
        text = GEN['table'](widths, gcb, incb, ep)
        for value in re.findall(r"'\\u\{([0-9A-F]+)\}'", text):
            cp = int(value, 16)
            self.assertTrue(cp <= 0x10FFFF and not 0xD800 <= cp <= 0xDFFF)

    def test_complete_grapheme_corpus(self):
        fixtures = dict(GEN['grapheme_tests'](self.data['GraphemeBreakTest.txt']))
        self.assertEqual(len(fixtures), 19)
        self.assertEqual(sum(text.count(' = str.graphemes(') for text in fixtures.values()), 1187)

    def test_corrupted_source_is_rejected_before_derivation(self):
        root = GEN['ROOT']
        scratch = root / 'target/unicode-test'
        scratch.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=scratch) as directory:
            copy = Path(directory)
            shutil.copytree(root / 'vendor/unicode', copy / 'vendor/unicode')
            source = copy / 'vendor/unicode/15.1.0/UnicodeData.txt'
            source.write_bytes(source.read_bytes() + b'\n')
            with patch.dict(GEN['sources'].__globals__, ROOT=copy):
                with self.assertRaisesRegex(ValueError, 'UnicodeData.txt'):
                    GEN['sources']()


if __name__ == '__main__':
    unittest.main(verbosity=2)
