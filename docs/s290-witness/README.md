# s290 witness: the session rows, recorded before a release carries them

`rows-wolf-68f33aba.log` is a record (START/END steps, exit codes) of this branch's four session
rows run on a Linux host against a wolf built from wolf-lang's branch `s290` at `68f33aba`, each
lane as `cargo xtask std-test` calls it (`conform-run [--checked|--native] --std-root STD FILE
--json`), plus lupin 0.1.60:

| row | checked | native | lupin 0.1.60 |
|---|---|---|---|
| `tests/process/session_rows.lu` | exit 0, the five lines | exit 0, the five lines | unsupported (does not resolve) |
| `tests/process/session_child.lu` | exit 0 (no program file: prints the lines) | exit 0, the child leads its session and group, no `/dev/tty` | unsupported |
| `tests/process/lead_session_rows.lu` | exit 0 | exit 0, a forked child moves, then is `denied` | unsupported |
| `tests/process/comptime_refuses_session.lu` | E0701 at `os_setsid` | E0701 at `os_setsid` | unsupported |

The same record's `pax-fixture` step is wolf-lang's PAX fixture, which is that repository's
evidence. `wolf fmt --check` passes on the five changed files. Nothing here lands until a wolf
release carries `os_spawn_session`, `os_setsid` and `os_sid` and the pin moves to it
(`docs/s290-prediction.md`, P3).
