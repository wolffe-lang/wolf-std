# sc81 prediction: sessions and pseudo-terminals

Written before any measurement and before any change under `std/`, `tests/`, `vendor/` or `xtask/`. Base: trunk `2411d86`. The toolchain this branch moves to: wolf 0.2.39 (tag commit `3fc86f77`) paired with lupin 0.1.61 (conformance pin `5f1f1c94`). Only `--version` has been run on either binary.

The work: a std surface over the compiler's eight new calls, so that an ssh server can run each remote command in a session of its own and answer a terminal request by running a login shell on a pseudo-terminal, relaying bytes and window-size changes. The sources are wolf-lang's `spec/11-os.md` at the tag (`[os.proc.session]`, `[os.term.session]`, `[os.term.pty]`, `[os.term.size.set]`, `[os.term.attr]` and the two design notes), its 0.2.39 release note and CHANGELOG, and the corpus rows `corpus/os/session_rows.lu` and `corpus/os/pty_rows.lu`.

## The surface, as predicted to land

`std.process` gains the three names the spec's design note gives it:

- `start_session(c: Command, defaults: int) -> Child ! {unsupported, invalid, not_found, denied, io}` over `os_spawn_session`;
- `lead_session() -> () ! {denied, io, unsupported}` over `os_setsid`;
- `own_session() -> int ! {unsupported}` over `os_sid`.

`std.term` gains the five, a pair type and one step for a child:

- `Pty { master: fs.File, slave: fs.File }` and `open_pty() -> Pty ! {io, unsupported}`;
- `pty_name(master: fs.File) -> str ! {io, unsupported}`;
- `set_size(tty: fs.File, s: Size, xpixels: int, ypixels: int) -> () ! {invalid, io, unsupported}`;
- `attr(tty: fs.File, op: int) -> int ! {invalid, io, unsupported}` and `set_attr(tty: fs.File, op: int, value: int) -> () ! {invalid, io, unsupported}`, by RFC 4254 opcode;
- `open_controlling(name: str) -> fs.File ! {denied, io, not_found, unsupported}`: a process that leads a session with no terminal opens the terminal `name` as its controlling terminal, and the call says so or answers a row.

Nothing new for the relay: the master is an `fs.File`, so `signal.wait_fds`, `fs.set_nonblock`, `fs.write_some` and `fs.read_chunk` serve it as they serve a pipe.

## P1. The pins alone

The binary pin (wolf 0.2.36 to 0.2.39, lupin 0.1.58 to 0.1.61) and the data pin (`5adf3d91` to `3fc86f77`), with no other change:

- `doctor` green with both binaries matched; `sync-pin` green at `3fc86f77`.
- The anchors snapshot grows to 742 entries (the CHANGELOG's count), all added, none removed or changed; the key sets are diffed both ways.
- `std-test` red on lupin cells only, and on exactly the seven rows wolf-std#125 names: `env/arg0_rows`, `fs/access_rows`, `os/host_rows`, `process/core_limit_rows`, `fs/fstat_record_rows` and `term/mode_bits_set` advance from `unsupported` to `run`, and `fs/nofollow_rows` from `divergent(stdout)` to `run`. A cell or two more may advance from lupin 0.1.61's own mirror; any such cell is named, not predicted. No `wolfc` or `native` cell moves: the three releases' notes say nothing in this repository moves.
- `doc-examples` red on the two `std/os/os.lu` mirror-lag entries (`os.name`, `os.arch`) and nothing else.
- Each moved cell is flipped in its own commit, and the two mirror-lag entries retire in the commit that flips `os/host_rows`.

## P2. Red first

Every new row is committed before the code it tests. Before the code, each new row reads `fail(E0301)` on both compiler lanes (a name the imported module does not declare) and a static refusal on lupin. Each row is red by its file's name in the rows runner and in `std-test`.

## P3. The rows, green

| row | what it holds | lanes |
|---|---|---|
| `process/session_shape_rows` | a map entry outside 0..255 and a `defaults` bit outside the meanings are `invalid`; an empty argv is `not_found`, decided by std on every host | all three |
| `process/session_spawn` | `start_session` with `/bin/sh`: the child's reported pid is `pid(ch)`; its group exists and lies in another session (a send of `continued()` to it is served; a spawn that joins it is `denied`, while a join of a group of this session is served); this process's session and group are unchanged; the map is placed | checked, native; lupin refuses by name |
| `process/session_child` | a fork's child: `lead_session` served, then `own_session` and `own_group` equal its pid, and a second `lead_session` is `denied`; a self-spawn under `start_session`: the child's session id equals its pid and `pid(ch)`, differs from the parent's, and its `lead_session` is `denied` | native; the checked machine and windows answer `fork`'s `unsupported` and print the lines |
| `term/pty_shape_rows` | the corpus's shape rows through std: sizes outside 0..65535, opcodes outside the vocabulary and values outside their op are `invalid`; each host's lacking ops and an unlisted speed are `unsupported`; a forged handle is `io` (windows `unsupported`) | all three |
| `term/open_controlling_rows` | a missing name is `not_found`; a regular file is `io` and stays a regular file; nothing is created for a missing name | checked, native; lupin refuses by name (at `os_pgid`) |
| `term/pty_pair` | the pair: two handles, the name `/dev/pts/N` (linux) or `/dev/ttysN` (macOS), a slave's name `io`; the size set on the master read on both ends; `ECHO` and a speed round-trip on the slave; a lacking op `unsupported` on a real slave; bytes both ways; `open_controlling` from a process that leads no session is `io` and takes nothing; after both ends close every call on them is `io` | checked, native; lupin refuses by name at the open |
| `term/pty_child` | `start_session` with the slave on 0, 1 and 2 and `/bin/sh`: three terminals, `stty size` reads the size set on the master, the shell reads what the master wrote and the master reads what it printed; after the master closes, the shell's next read ends | checked, native |
| `term/pty_session` | burrowd's shape, by fork: the child leads a session, takes the slave by name as its controlling terminal (its foreground is its own group), hears `resize()` once per change of the four fields (the pixels alone included) and not for the same four, reads the new size, and its read ends when the master closes; then the same with `/bin/sh` exec'd on the slave: three terminals and `/dev/tty` opens | native; the checked machine and windows print the lines |
| comptime rows | `os_spawn_session`, `os_setsid`, `os_sid`, `os_term_open_pty` and `os_term_set_size` refused at comptime (E0701), one level down | checked, native |

Host behaviour predicted, Linux: the master's read after the child's last slave descriptor closes is `io` (EIO), and the shell's read after the master's close ends without a signal (no session holds that terminal in `pty_child`). macOS (CI's `tier all` only): the name `/dev/ttysN`, the master's read at the end `eof`; the same rows otherwise.

## P4. lupin

lupin 0.1.61 answers the shape rows and refuses by name every call that reaches a session or a terminal. Each lupin cell of a row that reaches one is `unsupported`, and its refusal sentence names the builtin it stops at: `os_spawn_session`, `os_sid`, `os_fork`, `os_pgid` or `os_term_open_pty`. Each `lupin-windows` cell is `run`: on windows every new call answers `unsupported` as a row before anything is reached, on lupin as on the compiler.

## P5. The platforms

- **Linux**: every row runs in the org's Rust toolchain image (Ubuntu 24.04), in a rootless container on a six-CPU block, the record taken with the recorder.
- **macOS**: no local run (the Mac is under its disk floor). The full tier runs macOS's `gates` (the doc examples), not the rows; the rows run on macOS only in a `tier all` run.
- **PAX**: wolf-std has no PAX leg; PAX is stated from the spec only (the static linux binary through the kernel's `setsid`, `/dev/ptmx`, `TIOCGPTN` and `TIOCSPTLCK`), not measured here.
- **Windows**: the full tier's `rig (windows-latest)` runs every row; each prints its lines for the `unsupported` reason.

## P6. The plant

One commit, one file, one hunk on the final head: `start_session` calls `os_spawn_job` with group 0 in place of `os_spawn_session`, so the child leads a group of this process's session. `process/session_spawn` goes red by name (the join of the child's group is served), and so does `process/session_child`'s self-spawn line (the child's session is the parent's). The shape rows and the pty rows stay green.
