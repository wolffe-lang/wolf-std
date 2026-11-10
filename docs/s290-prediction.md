# s290 prediction: a new session, in `std.process`

Written before any change under `std/` or `tests/`. Base: trunk `2411d86`. Toolchain unchanged:
wolf 0.2.36, lupin 0.1.58 (the pins in `vendor/tools.toml`). The calls this surface wraps do not
exist in any wolf release yet. They are proposed in wolf-lang's `notes/s290-prediction.md` on its
branch `s290` (base `bca5dbc5`, 0.2.38), where the shape is measured and argued; this file is the
std half of that prediction, and nothing here lands before a wolf release carries the calls.

## P1. The surface

Three functions, each the twin of one this module already has:

| function | wraps | twin | rows | capability |
|---|---|---|---|---|
| `start_session(c: Command, defaults: int) -> Child` | `os_spawn_session(exe, args, c.fds, defaults)` | `start_job` without the group | `{unsupported, invalid, not_found, denied, io}` | `Exec` |
| `lead_session() -> ()` | `os_setsid()` | `lead_group` | `{denied, io, unsupported}` | `Exec` |
| `own_session() -> int` | `os_sid()` | `own_group` | `{unsupported}` | `env` |

`start_session` starts `c` as the leader of a new session, with no controlling terminal: its session
id, its group id and `pid(child)` are one number, and `group_of(child)` names its group, so
`signal.send_group` reaches it. `lead_session` moves THIS process into a session of its own; it is
`denied` for a process that leads a group (a session leader included), and the doc says the three
sound ways to call it: a process started in its parent's group, the child of `fork`, or a process
that starts itself again with `start_session`. `own_session` answers an int, not a struct: no call
takes a session. All three are refused at comptime (D33), with the module's other calls.

If wolf-lang's ruling takes its option A instead (a session as `os_spawn_job`'s group -2),
`start_session` passes -2 to `start_job`'s builtin and the table above is otherwise unchanged, and
**`tests/process/job_rows.lu` must move with the pin bump**: `process.start_job(c, -2, 0)` reads
`group=invalid` at 0.2.36 and would read `group=not_found` (the shape passes and the program is
missing) at a pin carrying A. The row moves to -3, which stays `invalid`.

## P2. The rows

Committed red before the module changes, each `fail(E0301)` on the compiler lanes until the
functions exist:

- `tests/process/session_rows.lu`, every host: `start_session` of a program no host has is
  `not_found`; with a `defaults` bit outside the meanings it is `invalid` before the program is
  looked for; `own_session()` answers a positive id and moves nothing; on windows the three are
  `unsupported`.
- `tests/process/session_child.lu`, unix, native: the program starts itself again with
  `start_session` and a role argument; the child prints whether `own_session() == env.pid()`,
  whether its group is its own, and whether opening `/dev/tty` fails (`io`); the parent prints its
  own session unchanged and waits for the child's status 0.
- `tests/process/lead_session_rows.lu`, unix, native: a forked child's `lead_session()` is `ok`, and
  `own_session() == env.pid()` after; a second call is `denied`.
- `tests/process/comptime_refuses_session.lu`: D33 for the three, held one level down at the
  builtins (F-0051), as `comptime_refuses_set_pgid.lu` holds `lead_group`.

lupin at the paired release refuses the three builtins by name (it links no C library, and Rust's
standard library has no stable new-session spawn and no `getsid`), so the ledger records the rows
that reach them as `unsupported` on lupin, and the rows that start nothing stay three-lane where
the shape answers first.

## P3. What lands when

Nothing in `std/` or `tests/` changes until a wolf release carries `os_spawn_session`, `os_setsid`
and `os_sid` and a lupin release pairs with it. Then one change bumps `vendor/tools.toml` and
`vendor/upstream/` to that release (with the ledger deltas the bump causes), and the next adds the
three functions, their rows and the module header's paragraph. Before that, the rows above are
measured against a wolf built from wolf-lang's branch through `WOLF_BIN`, and recorded, not landed.
