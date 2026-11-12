#!/usr/bin/env python3
"""Small protocol/fixture checks; no wolf program or host resource runs."""
from pathlib import Path
import runpy
import unittest

HERE = Path(__file__).resolve().parent
ROWS = runpy.run_path(str(HERE / "rows.py"))


class RowReader(unittest.TestCase):
    def test_output_escapes(self):
        verdict, lines = ROWS["expectation"]('//! check: run(exit=0, stdout="x=true\\ny=closed")\n')
        self.assertEqual(verdict, "exit(0)")
        self.assertEqual(lines, ["x=true", "y=closed"])

    def test_rejection(self):
        self.assertEqual(ROWS["expectation"]('//! check: fail(E0701)\n'), ("fail(E0701)", None))

    def test_unknown_directive(self):
        with self.assertRaises(ValueError):
            ROWS["expectation"]('//! check: run(exit=trap(bounds))\n')

    def test_named_line(self):
        self.assertEqual(ROWS["key"]("one_waiter=unsupported"), "one_waiter")
        self.assertEqual(ROWS["key"]("row: false"), "row")

    def test_catalogue(self):
        rows = ROWS["ROWS"]
        self.assertEqual(len(rows), len(set(rows)))
        for row in rows:
            source = (ROWS["ROOT"] / "tests" / row).read_text()
            ROWS["expectation"](source)
        self.assertEqual(ROWS["REFUSALS"]["poll/waiter_rows.lu"], "os_ready_open")
        self.assertEqual(ROWS["REFUSALS"]["net/unix/peer_credentials.lu"], "net_peer_cred")
        self.assertNotIn("net/write_some.lu", ROWS["REFUSALS"])

    def test_windows_postures_are_explicit(self):
        self.assertNotIn("poll/signal_during_wait.lu", ROWS["WINDOWS_RUN"])
        self.assertIn("poll/signal_socket.lu", ROWS["WINDOWS_RUN"])
        self.assertIn("poll/sockets_all_ready.lu", ROWS["WINDOWS_RUN"])


if __name__ == "__main__":
    unittest.main()
