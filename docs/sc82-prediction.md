# sc82 prediction: one wait over files, sockets and signals

Written before any measurement and before any change under `std/`, `tests/` or `xtask/`. Base: trunk `77cf321`. The toolchain is the one the repository pins: wolf 0.2.39 (tag commit `3fc86f77`) and lupin 0.1.61 (conformance pin `5f1f1c94`). The sources read are wolf-lang's `spec/11-os.md` and `spec/prelude.json` at the tag (`[os.signal.wait.fds]`, `[os.signal.wait.write]`, `[os.signal.hub]`, `[os.net]` through `[os.net.reactor.note]`, `[os.proc.inherit]`, `[os.proc.adopt]`, `[os.proc.fds]`, `[os.proc.pipe]`, `[os.fs.std]`, `[os.fs.nonblock]`), and this repository's `std.signal`, `std.net`, `std.net.unix`, `std.term`, `std.fs` and `std.process`.

The ask: a program that owns pseudo-terminal masters, a listening Unix socket and its clients, a pipe, a timer and signals (a terminal multiplexer's server) waits on all of them in one call, learns of every ready one, writes without blocking, and asks a client's credentials.

## P0. What the pin cannot do

Read from the spec and the prelude, not yet measured:

- fs handles and net handles are numbers of two tables the runtime keeps apart (`[os.fs.std]`; `[os.net]`: "a dense small `int` into the runtime's own socket table, never the OS fd"). `os_signal_wait_fds` takes fs handles and `net_wait` net handles, and no call carries a handle from one table to the other (`fs_adopt` takes an inherited descriptor that is not close-on-exec; `net_adopt_listener` takes a listener only). So **no std code can wait on a file and a socket in one call** at this pin.
- `net_wait` hears no signal and has no write interest; `os_signal_wait_fds` refuses an empty set, so a file wait with no meanings cannot be spelled.
- `os_signal_wait_fds` answers one index, the lowest ready; it reports a hangup or an error as "ready", never by name.
- No net call writes part of a list and says how much: `net_write`, `net_write_bytes` and `net_writev` drain everything or answer a row.
- No call answers a Unix-domain peer's credentials.

The lane names the missing calls exactly (the note's "Missing below std") and builds what the pin serves.

## The surface, as predicted to land

A new module, `std.poll`, over `std.fs`, `std.net` and `std.signal`:

- `READ = 1`, `WRITE = 2`, `HANGUP = 4`, `ERROR = 8`, the facts of a ready entry and the interests of an entry;
- `Waiter`, an owned set of entries, `Ready { key, facts }` and `Events { meaning, ready }`;
- `open() -> Waiter ! {io}` and `close(take w: Waiter) -> () ! {io}`;
- `add_file(mut w, f: fs.File, interest: int) -> int ! {invalid, io, unsupported}`, `add_socket(mut w, s: net.Socket, interest: int) -> int ! {invalid, io, unsupported}` and `add_listener(mut w, l: net.Listener) -> int ! {invalid, io, unsupported}`, each answering the entry's key;
- `modify(mut w, key: int, interest: int) -> () ! {invalid, io}`, `remove(mut w, key: int) -> () ! {invalid, io}` and `len(w) -> int`;
- `wait(mut w, meanings: int, deadline_ms: int) -> Events ! {invalid, io, unsupported}`, which answers **every** ready entry with its facts, and one meaning taken (0 when none).

The `Waiter` is shaped for a set the kernel holds (epoll, kqueue), which the note asks of the runtime: `open` and `close` own it, `add_*`, `modify` and `remove` change it, and `wait` reads what is ready, so a wait costs O(ready) once the runtime has it. At this pin the `Waiter` holds a list and owns nothing of the host's.

At this pin `wait` serves two shapes. Files (pty masters, pipes, a terminal, a regular file) with read and write interest, a non-empty set of meanings and a deadline are served over `os_signal_wait_fds`, looked at again past each ready entry until none is left, so every ready entry is answered. Sockets and listeners with read interest, no meanings and a deadline are served over `net_wait`. Four combinations answer `unsupported`, decided before the host: files and sockets in one waiter, sockets with meanings, a socket with write interest, and files with no meanings. `HANGUP` and `ERROR` are declared and never set; a hangup reads as `READ`, and the read that follows names it. `signal.wait_fds` and `net.wait` are unchanged and documented as the narrow forms.

The partial write: a pty master and a pipe already have it (`fs.set_nonblock`, `fs.write_some`: a count, or `busy`). A socket's (`net.write_some`) and the peer's credentials (`unix.peer_credentials`) are contracts in the `std.net` and `std.net.unix` headers, with the missing call named. No function ships that the pin cannot serve.

## P1. Red first

The rows are committed before `std.poll` exists. Each one reads a static refusal on both compiler lanes at the `use std.poll` (an `E03xx` code, the module not found), and lupin refuses it at resolve. Each row is red by its file's name.

## P2. The rows, green

| row | what it holds | lanes |
|---|---|---|
| `poll/waiter_rows` | an interest outside 1..3, a handle the waiter already holds, a listener added twice, an unknown key, a deadline below -1, a bit that is no listenable meaning, an empty waiter with no meanings and no deadline: `invalid`; the four combinations: `unsupported`; keys stable across a remove, the lowest free key reused; `len` | all three |
| `poll/files_all_ready` | two pty masters (a byte from each slave), a pipe holding a byte and a regular file, all ready at once: all four keys answered with `READ`, in one wait; the narrow form answers index 0 alone for the same set | checked, native |
| `poll/sockets_all_ready` | a TCP listener with a connection pending and two accepted clients each holding bytes: all three keys answered with `READ`; a quiet set answers nothing at the deadline | all three |
| `poll/mixed_refused` | the brief's set (two pty masters, a Unix listener with a dial pending, two clients holding bytes, a pipe holding a byte): one waiter answers `unsupported`; the files alone answer three keys, the sockets alone three | checked, native |
| `poll/fairness` | a pipe kept readable at key 0 and a pty master at key 1 that gets a byte at the fourth turn: each of ten waits answers key 0, and every wait from the fourth answers key 1 too, until it is read; the narrow form over the same set answers index 0 every turn | checked, native |
| `poll/deadline` | deadline 0 over quiet entries answers nothing at once; 25 ms passes no sooner than 25 ms on the monotonic clock, on the files path and the sockets path; -1 with a ready entry answers at once | checked, native (the files path); lupin the sockets path only |
| `poll/signal_during_wait` | `child()` listened for; a child that sleeps 300 ms and ends; a wait over a quiet pipe with a 10 s deadline answers `child()` and no entry, after at least 200 ms; a meaning raised before the wait answers at once, and a ready file then answers the next wait | checked, native |
| `poll/write_partial` | a pty master, the slave raw and quiet, the master non-blocking: `write_some` takes bytes until `busy`; a write wait over the full master passes its 25 ms deadline; a read of the slave makes room and the wait answers `WRITE`; a queue three times the room drains through the loop (write what fits, wait for `WRITE`, read the slave), and the slave reads every byte in order | checked, native |
| `poll/pty_hangup` | `start_session` with a pty slave on 0, 1 and 2 and `/bin/sh -c 'printf x'`; the parent closes its slave; the waiter answers the master `READ`, the read gives `x`, the next wait answers `READ` again, and the read then answers `io` (linux); `HANGUP` is not set at this pin; status 0 | checked, native |

Host behaviour predicted, Linux: a pty master is ready for `WRITE` until the slave's input queue fills (some kilobytes); a master whose last slave descriptor closed is ready, and its read is `io`. macOS (CI's `tier all` only): the read is `eof`; the same rows otherwise.

No new comptime row: `std.poll` reaches the host only through `os_signal_wait_fds` and `net_wait`, whose comptime refusals the tree already holds.

## P3. lupin

lupin 0.1.61 runs `poll/waiter_rows` and `poll/sockets_all_ready` (`net_wait` resolves there) and the sockets half of `poll/deadline`. It refuses the rest by name: `os_term_open_pty` where a row opens a pty first, `os_signal_wait_fds` where a row reaches the files path first.

## P4. The platforms

- **Linux:** every row in the org's Rust toolchain image (Ubuntu 24.04), in a rootless container on a six-CPU block, recorded with the recorder.
- **macOS:** CI's `tier all` runs the rows; the full tier runs the doc examples. No local run.
- **Windows:** the files path answers `unsupported` (`os_signal_wait_fds`'s posture), and the rows print their lines for that reason; the sockets path is served (`WSAPoll`).
- **PAX:** no leg in this repository. The note says which rows a PAX lane runs and how.

## P5. The plant

One commit, one hunk, on the final head: the files path stops after the first ready entry, so a wait answers the lowest ready entry alone, as the narrow form does. `poll/files_all_ready` and `poll/fairness` go red by name on both compiler lanes, and `poll/mixed_refused`'s files half too. The sockets rows and the refusal rows stay green.
