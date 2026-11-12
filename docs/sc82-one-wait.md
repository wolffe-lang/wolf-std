# sc82 — wolf-std: one wait over files, sockets and signals

Footprint: cross-cutting (a new public module, `std.poll`)

A program that owns pseudo-terminal masters (`term.open_pty`, fs handles), a listening Unix socket and its clients (`std.net.unix`, net handles), a pipe, a timer and signals (child exit, hangup) cannot wait on all of them in one call at wolf 0.2.39: `signal.wait_fds` takes fs handles and `net.wait` net handles, each answers `io` for the other's, and `signal.wait_fds` answers only the lowest ready index, so a busy low descriptor starves the rest unless the caller rotates. There is no non-blocking socket write that says how much it took, and no call for a Unix-domain peer's credentials. A terminal multiplexer (one server, an event loop over dozens of pty masters, a listener and its clients, a timer and signals) is blocked on all three.

This lane gives the wait a std surface, `std.poll`, shaped for a readiness set the kernel holds, and serves today what the pin can: every ready entry (never only the lowest) over files and signals, and over sockets alone. It names, exactly, the three calls the runtime lacks (below), so that a runtime lane can add them and a follow-on can retire this module's refusals without changing a signature.

The prediction was committed first, at `497f8dd` (`docs/sc82-prediction.md`).

## Design

### The loop a server writes

```text
var w = poll.open()?
let k_master = poll.add_file(mut w, p.master, poll.READ)?
let k_listen = poll.add_listener(mut w, srv)?
signal.listen(signal.child() | signal.reload())?
loop {
    let ev = poll.wait(mut w, signal.child() | signal.reload(), next_timer_ms)?
    if ev.meaning == signal.child() { reap every child }
    for r in ev.ready {
        if r.key == k_listen { accept, add the client with READ }
        else if (r.facts & poll.READ) != 0 { read; an end (`eof`, `closed`, a pty's `io`) removes the entry }
        if (r.facts & poll.WRITE) != 0 { write what fits from the entry's queue; an empty queue goes back to READ }
    }
}
poll.close(take w)?
```

A write that would block is not a failure but a count: the handle is non-blocking, `write_some` takes what fits and answers the count, or `busy` when nothing fits, and the server keeps the rest in the client's queue and asks for `WRITE`. A pty master and a pipe have that write today (`fs.set_nonblock`, `fs.write_some`); a socket's is the second missing call (M2).

### The surface

`std.poll`:

- `READ = 1`, `WRITE = 2`, `HANGUP = 4`, `ERROR = 8`: an entry's interests (`READ`, `WRITE` or both) and a ready entry's facts.
- `Waiter`, an owned set of entries; `Ready { key: int, facts: int }`; `Events { meaning: int, ready: List[Ready] }`.
- `open() -> Waiter ! {io}`; `close(take w: Waiter) -> () ! {io}`.
- `add_file(mut w: Waiter, f: fs.File, interest: int) -> int ! {invalid, io, unsupported}`, `add_socket(mut w: Waiter, s: net.Socket, interest: int) -> int ! {invalid, io, unsupported}`, `add_listener(mut w: Waiter, l: net.Listener) -> int ! {invalid, io, unsupported}`: each answers the entry's key.
- `modify(mut w: Waiter, key: int, interest: int) -> () ! {invalid, io}`, `remove(mut w: Waiter, key: int) -> () ! {invalid, io}`, `len(w: Waiter) -> int`.
- `wait(mut w: Waiter, meanings: int, deadline_ms: int) -> Events ! {invalid, io, unsupported}`.

`wait` answers every entry that is ready, each once, with the facts it has, and the meaning it took (0 when none); an empty `ready` and meaning 0 is the deadline. `meanings` 0 asks for none; `deadline_ms` is `signal.wait_fds`'s (0 looks, -1 never passes). Readiness is level-triggered, as in both narrow forms.

**What the pin serves**, and what it does not, decided before the host so a caller can tell the two apart:

- files (a pty master, a pipe, a terminal, a regular file), read and write interest, with a non-empty set of meanings: served over `os_signal_wait_fds` (the files path, below);
- sockets and listeners with read interest, no meanings: served over `net_wait`;
- an empty waiter: the meanings and the deadline alone;
- `unsupported`, every host: files and sockets in one waiter; sockets with meanings; a socket with write interest; files with no meanings. Each is the missing call M1's, each is held by a row, and each flips to served without a signature change when M1 lands.

**The files path at this pin.** `os_signal_wait_fds` answers the lowest ready element. `wait` asks it once with the caller's deadline, then again with deadline 0 over the elements past each one it answered, until it answers none: a wake with k ready entries costs k + 1 looks. An entry with both interests is two elements. The list starts one past the last entry the previous wait answered, so the look's order rotates and no entry waits behind others even when meanings keep arriving. A meaning taken ends the sweep: the wait answers it with the entries found before it, and the rest answer the next wait at once.

### Rulings, each with the option implemented

- **R-sc82-1, where it lives.** (A) A new module, `std.poll`, over `std.fs`, `std.net` and `std.signal`. (B) In `std.signal`, beside `wait_fds`. (C) In `std.net`, beside `wait`. **Recommended and implemented: A.** The wait takes handles of both modules and signals; (B) makes `std.signal` depend on `std.net`, (C) makes `std.net` depend on `std.signal` and `std.fs`, and either buries the wide form under one of its inputs. `poll` is the noun POSIX gives the call.
- **R-sc82-2, the handle.** (A) An owned `Waiter`: `open`, `add_*`, `modify`, `remove`, `wait`, `close`, shaped for a set the kernel holds (epoll on linux and PAX, kqueue on macOS), so that a wait costs O(ready) once the runtime has the set (M1), and a list where it does not. (B) A one-shot call over a list each turn (`signal.wait_fds`'s shape). (C) Both in std. **Recommended and implemented: A,** with the one-shot as the runtime's simple form under it. A server keeps dozens of entries across thousands of waits: (B) rebuilds and re-sends the list every turn, and cannot become O(ready) without changing its signature. `close` exists now, though it frees nothing at this pin, so a program written today closes the kernel set it will hold tomorrow.
- **R-sc82-3, what `wait` answers.** (A) Every ready entry, with its facts, and one meaning. (B) The lowest ready, as the narrow form. (C) A bitset of keys. **Recommended and implemented: A.** (B) is the starvation this lane exists to end; (C) caps the set at 63 entries.
- **R-sc82-4, the combinations the pin cannot serve.** (A) `unsupported`, decided before the host, each held by a row. (B) A time-slice inside std: the two narrow waits alternating under a short deadline. (C) A helper task that waits in `net_wait` and writes a pipe the files wait holds. **Recommended and implemented: A.** (B) hides a wakeup every slice and a slice of latency on every socket behind a call that looks like one wait. (C) rests on how the task layer maps tasks onto threads while one of them blocks in `poll(2)`, which the spec does not state and this repository cannot read. A caller who accepts (B)'s cost can still write it, in the open, over the narrow forms.
- **R-sc82-5, hangup and error.** (A) Declared now, never set at this pin, and once the runtime reports them, always set together with the interest the entry asked (a read-interest entry that hung up answers `READ | HANGUP`). (B) Left out until then. (C) Probed with a read. **Recommended and implemented: A.** Under (A) a loop that tests `READ` alone keeps working when `HANGUP` appears: it reads and meets the end (`eof`; a pty master's `io` on linux). (C) consumes data the caller has not asked for.
- **R-sc82-6, the files path's sweep.** (A) Look again past each ready element (k + 1 looks), the start rotated each wait, a meaning ending the sweep. (B) One look per element (N looks every wait). **Recommended and implemented: A.** (A) costs one look when one entry is ready, which is the common wake; both are O(N) per look in descriptors duplicated, which M1 removes.
- **R-sc82-7, the shape.** `wait` decides `invalid` itself, before the host: a deadline below -1, a set with a bit that is no listenable meaning, an empty waiter with no meanings and no deadline (nothing could end it). So the narrow form's spelling of a bad set (`io`) never reaches a caller of `std.poll`, and M1's spelling of it does not matter to std.
- **R-sc82-8, keys.** An `int`, the lowest free one, stable while the entry lives; a handle is in a waiter once (`invalid` for a second add: `modify` changes an entry). Remove an entry before closing its handle; at this pin a closed handle in a waiter makes the wait `io`.
- **R-sc82-9, the partial write.** (A) Nothing new for files: `fs.set_nonblock` and `fs.write_some` are the write, `std.poll`'s header shows the loop, and `fs.write_some` gains a runnable partial-write example (a fifo opened read-write, filled to `busy`). The socket's write is a contract in `std.net`'s header (M2). (B) A std output-queue type. **Recommended and implemented: A.** A queue is a `List[byte]` and an offset, and a server owns its own.
- **R-sc82-10, peer credentials.** (A) A contract in `std.net.unix`'s header, naming M3; no function and no type until it lands. (B) Ship the answer's type now, as `std.net` shipped `Addr` before a resolver. **Recommended and implemented: A** (API-CONVENTIONS §14, sc11: a function the pin cannot serve is a contract, shipped in the sprint after its blocker closes). Nothing needs to hold a credential before something can ask for one.
- **R-sc82-11, the narrow forms.** `signal.wait_fds` and `net.wait` are unchanged, and their docs name `std.poll` as the wide form.

## Missing below std

Three calls, none in wolf 0.2.39's prelude (`spec/prelude.json`) or in `spec/11-os.md` at the tag, nor on trunk after it. Each is stated as the contract std's surface is built for. Where a sentence says "std decides", std's surface never reaches the case, and the runtime may answer it as it likes.

### M1. One wait over both tables, every ready entry, facts per entry

**Why it is the runtime's.** fs handles and net handles are numbers of two tables the runtime keeps apart (`[os.fs.std]`, `[os.net]`), and only the runtime holds the descriptors behind both. No std code can wait on both at once; `os_signal_wait_fds` reports one index and calls a hangup "ready"; `net_wait` hears no signal and has no write interest; and `os_signal_wait_fds` refuses an empty set.

**The set the kernel holds** (the surface `std.poll.Waiter` is designed around):

```text
os_ready_open() -> int ! {io, unsupported}
os_ready_set(r: int, handle: int, interest: int, key: int) -> () ! {invalid, io, unsupported}
os_ready_wait(r: int, set: int, deadline_ms: int) -> (List[int], int) ! {invalid, io, unsupported}
os_ready_close(r: int) -> () ! {io}
```

- `os_ready_open` makes an empty readiness set and answers its handle, close-on-exec, a number of the runtime's choosing that no other family takes. `io` when the host will not give one (the descriptor limit, its number in `os_error()`); `unsupported` by name where the host has no set (windows; the freestanding target).
- `os_ready_set` adds the entry for `handle`, or changes it if the set holds that handle; `interest` 0 removes it. `handle` is an fs handle as `os_signal_wait_fds` takes it (0..2 the standard streams, 3 and up the fs table) or, with **bit 61** set (`h + (1 << 61)`, 2305843009213693952), a net handle: a listener or a stream. `interest` is 1 read, 2 write, 3 both. `key` is the caller's, 0 or more, and is what the wait answers. Order of refusal: `invalid` for an interest outside 0..3 or a negative key (std decides both); `io` for a closed or forged set, a closed or forged handle of either table, and a removal of a handle the set does not hold; `unsupported` where `os_ready_open` is.
- `os_ready_wait` parks until at least one entry is ready, or a meaning in `set` is delivered, or the deadline passes, and answers **`(ready, meaning)`**: `ready` a flat list of pairs `[key, facts, key, facts, …]` naming **every** ready entry once, in no promised order, and `meaning` the meaning taken off the queue as `os_signal_wait` takes it (0 when none). A meaning does not hide the entries: both are answered in one return. `set` 0 asks for no meanings; a non-zero set is held to `[os.signal.set]`'s shape rule (std decides it). `deadline_ms` is `os_signal_wait_fds`'s: 0 looks once, -1 never passes, any other is measured on the monotonic clock and never answered early; below -1 `invalid` (std decides). An empty set with no meanings and -1 could never end (std decides). Level-triggered: an entry stays ready until it is read, written or accepted.
- **Facts:** 1 read (a byte, an end of input, a pending connection on a listener), 2 write (room for at least one byte, or a write that would fail at once), 4 hangup (`POLLHUP`, `POLLRDHUP`, `EV_EOF`), 8 error (`POLLERR`, `EV_ERROR`). **Hangup and error are answered together with every interest the entry asked**: a read-interest entry that hung up answers 5, so a loop that tests read alone reads and meets the end. A pty master whose last slave descriptor closed is hangup (and its read answers `io` on linux, `eof` on macOS, as now).
- **A handle's close removes its entry from every set that holds it**, so no wait answers the key of a handle the program closed. (epoll alone keeps an entry while any descriptor of the open file lives, a child's copy included; the runtime, which owns both tables, removes it at the close.)
- `os_ready_close` closes the set. `io` for a closed or forged one.
- The signal hub's wake reaches the set (its pipe registered in it), so a meaning ends a park as it does `os_signal_wait_fds`'s. Inside a scope that can be killed, the wait looks for a kill as `os_signal_wait_fds` does. A fork's child does not wait on its parent's set (the readiness-set rule `[os.proc.fork]` states for the runtime's own set).
- **Hosts, and the cost:** linux and PAX `epoll` (`epoll_ctl` per change, one `epoll_wait` per wait: O(ready)); macOS and freebsd `kqueue` (one `kevent` per change, one per wait: O(ready)), and where the host's kqueue refuses a descriptor (as macOS's `poll(2)` refuses `/dev/tty`, `[os.signal.wait.fd]`) the runtime's choice of a fallback for that entry, stated as its cost; windows `unsupported` by name; the checked machine over its own tables (a `poll(2)` per wait is acceptable there); lupin refuses by name (no `poll`). Capability: `exec` with the signal waits if a set may hold meanings (`os_signal_wait_fds`'s charge), refused at comptime.

**The one-shot form**, the same answer without a set, for a host with no set and for the checked machine:

```text
os_wait_ready(entries: List[int], set: int, deadline_ms: int) -> (List[int], int) ! {invalid, io, unsupported}
```

- An element is a handle as `os_ready_set` takes it (bit 61 for a net handle), with **bit 62** for write interest, `[os.signal.wait.write]`'s mark, kept; a handle may be listed twice, once each way. `ready` is a flat list of pairs `[index, facts, …]`, every ready element once, in list order; the meaning, the deadline, the facts and level-triggering are `os_ready_wait`'s.
- Hosts: linux, macOS, freebsd and PAX by `poll(2)` over a duplicate of each element's descriptor, as `os_signal_wait_fds` waits now (O(N) per wait); windows serves a list of net handles alone by `WSAPoll` (as `net_wait`) and answers `unsupported` by name for any fs element; the checked machine on its own tables; lupin refuses by name.

**What std then does.** `poll.open` calls `os_ready_open` and, where it answers `unsupported`, keeps a list for `os_wait_ready`; `add_*`, `modify` and `remove` call `os_ready_set`; `wait` calls `os_ready_wait` (or the one-shot); `close` calls `os_ready_close`. The four `unsupported` combinations above retire, `HANGUP` and `ERROR` are set, and a meaning arrives with the entries. No signature changes; `add_*` then answers `io` for a closed or forged handle at the add instead of at the wait.

### M2. A socket write that says what it took

```text
net_write_some(fd: int, bytes: List[byte]) -> int ! {busy, closed, invalid, io}
```

- ONE send of `bytes`, all of them or a first part, and how many the host took: at least 1 for a non-empty list, 0 for an empty one (the handle still checked). It never parks: the runtime's sockets already live non-blocking on the reactor hosts (`[os.net.io]`), and a host that keeps blocking sockets sends with `MSG_DONTWAIT` or answers `unsupported`. A `net_deadline` budget does not apply.
- `busy` when the socket can take nothing now (`EAGAIN`, `EWOULDBLOCK`, `WSAEWOULDBLOCK`; the host's number in `os_error()`), `[os.fs.nonblock]`'s mark; `closed` for a peer that has gone, with no `SIGPIPE`, as `net_write`; `invalid` for a list of the wrong element width, as `net_write_bytes`; `io` for a listener, a forged or closed handle, and the rest.
- Hosts: linux, macOS, freebsd, windows (a non-blocking `WSASend`) and PAX; the checked machine on its own socket; lupin can mirror it (it polls non-blocking sockets already). The cost: one `send(2)`, no copy of the list.
- std then ships `net.write_some(mut s: Socket, b: List[byte]) -> int ! {busy, closed, invalid, io}`, the contract in `std.net`'s header. A unix stream is a `net.Socket`, so it serves both families. PAX's Unix stream holds 4096 bytes in each direction and a pty master 20480, so partial writes are the normal case there.

### M3. The peer's credentials on a Unix-domain stream

```text
net_peer_cred(fd: int) -> (int, int, int) ! {io, unsupported}
```

- `(pid, uid, gid)` of the process at the other end of a connected `AF_UNIX` stream (accepted or dialed), as the host recorded them when the connection was made; `pid` is -1 where the host gives none.
- linux and PAX: `SO_PEERCRED`. macOS: `LOCAL_PEERCRED` (uid and the first group) and `LOCAL_PEERPID` (pid). freebsd: `LOCAL_PEERCRED` (its pid where the release has it, else -1). windows: `unsupported` by name, as the family is there; the freestanding target the same.
- `io` for a TCP stream, a listener, a forged or a closed handle (a wrong kind, the family's rule).
- Charged `net`, refused at comptime. The checked machine on its own socket. lupin refuses by name (stable Rust has no call for it).
- std then ships `unix.peer_credentials(s: net.Socket) -> Credentials ! {io, unsupported}` and `Credentials { pid: int, uid: int, gid: int }`, the contract in `std.net.unix`'s header. Its row: a client started under another uid where the test host allows it (else the same uid), its pid, uid and gid read back from the server's end.

## Hosts at this pin

- **linux, macOS, freebsd:** the files path and the sockets path. The files path's cost is k + 1 calls to `os_signal_wait_fds` for k ready entries, each a `poll(2)` over the remaining elements with a duplicate of each; the sockets path's one `net_wait` (`poll(2)` over N).
- **windows:** the files path answers `unsupported` (`os_signal_wait_fds`'s posture); the sockets path is served (`WSAPoll`).
- **PAX:** the static linux binary, the same as linux; not measured here (no leg in this repository). A PAX lane runs the rows below as the linux native binaries.

### Rows for a PAX lane

Build each row's native binary on linux with the pinned wolf and this branch's `std/` (`wolf build --std-root <checkout>/std tests/poll/<row>.lu` in a scratch directory of its own), copy it to PAX, run it in a scratch directory, and compare its standard output with the row's directive line by line (`docs/sc82-witness/rows.py`'s rule). The rows need `/bin/sh`, `printf` and `sleep` on PAX. Run: `poll/waiter_rows`, `poll/files_all_ready`, `poll/sockets_all_ready`, `poll/mixed_refused`, `poll/fairness`, `poll/deadline`, `poll/signal_during_wait`, `poll/write_partial` and `poll/pty_hangup`. `poll/write_partial` prints no room, so PAX's 20480-byte pty master holds it as linux's 14 KiB does. `poll/pty_hangup` expects a pty master's read at the end to be `io` on every host but macOS; if PAX answers `eof`, that is a host cell to record, not a fix to make.

## Forbidden

- No libc source, no kernel source and no source of the compiler's or the interpreter's crates was read. wolf-lang was read at the tag only in `spec/11-os.md` and `spec/prelude.json`, and its trunk's log since the tag.
- Host behaviour is measured black-box through the rows.
- **Unchanged:** CI, the rig's code, the pins, and every module's behaviour; `std.fs`, `std.signal`, `std.net` and `std.net.unix` change only in their docs.

## Inputs verified

- **Base:** trunk `77cf321`, branch `sc82`.
- **Toolchain:** the binaries the repository pins, by digest: `wolf` `cf00e296…`, `libwolf_rt.a` `0d6e9ecd…`, `lupin` `486234ae…`; `--version` reads `wolf 0.2.39 (wolfgang, pin 3fc86f7)` and `lupin 0.1.61 (wolf-interp, reference interpreter at pin 5f1f1c9)`. The rig's preflight checks the three digests before every take.
- **Read first:** API-CONVENTIONS §1–§4, §13 and §14 (the refusal rule and its sc11 debt clause, the sc08 rule on unreachable tags, the sc36 module-per-family rule); `std.signal` (`wait_fds`, `WAIT_WRITE`), `std.net` (`wait`, the handle types, what is deliberately absent), `std.net.unix`, `std.term` (`Pty`), `std.fs` (`set_nonblock`, `write_some`), `std.process` (`pipe`, `start_session`); the ledger's vocabulary; sc81's note and rig.

## Prediction

Committed first, at `497f8dd` (`docs/sc82-prediction.md`), before any measurement; the design and the missing calls followed at `ea8e952`.

- **P0, what the pin cannot do:** held, and measured where it could be: no std code waits on both tables. One correction to the starting brief, found by a probe: a net handle given to `signal.wait_fds` is not always `io`. The two tables number from 0 independently, so a net handle's number is often an open fs handle's too (the first listener a program makes is 0, which is standard input as an fs handle), and the wait then waits on that other file and answers it ready. It is `io` only when the number is not open in the fs table. The `Waiter` keeps each entry's table, which is the reason it has three add calls.
- **The surface:** held as predicted, name for name, rows as listed.
- **P1, red first:** held on the compiler lanes, `fail(E0301)` at the `use std.poll` for every row at the rows tip `31edb4d`. **Missed on lupin for the five rows that open a pty and `poll/signal_during_wait`:** lupin resolves lazily and reaches `os_term_open_pty` (or `os_signal_listen`) before the first `poll` name, so its refusal there already names the builtin the ledger expects, and those cells were green at the rows tip. lupin's cells for `poll/waiter_rows`, `poll/sockets_all_ready` and `poll/deadline` were red (`poll.open` does not resolve).
- **P2, the rows:** held on linux, every line as predicted, on both compiler lanes, from the first run. `poll/deadline`'s lupin cell is `unsupported` at `os_signal_wait_fds`, as P3 says, so its sockets half is held on lupin by `poll/sockets_all_ready`'s 25 ms line instead.
- **P3, lupin:** held, and each refusal names the predicted builtin: `os_term_open_pty` (the five rows that open a pty), `os_signal_wait_fds` (`poll/deadline`), `os_signal_listen` (`poll/signal_during_wait`, not in the prediction's list: lupin never listens, on any host, so that cell carries no `lupin-windows`).
- **P4, the platforms:** linux measured; macOS by CI's `tier all`, windows by the full tier's rig; PAX not measured (the rows above are for a PAX lane).
- **P5, the plant:** see the PR's records.

## Evidence index

### Rows

| row | holds | lanes |
|---|---|---|
| `poll/waiter_rows` | the add, modify and remove rows; keys in add order, a removed key reused; `wait`'s shape (`invalid`); the four combinations (`unsupported`); an empty waiter's no-deadline `invalid` and its look | all three, every host |
| `poll/files_all_ready` | two pty masters, a pipe's two ends and a regular file: one wait answers all five, master1 with `READ \| WRITE`; the narrow form answers index 0; level-triggered; the read facts go after the reads | checked, native |
| `poll/sockets_all_ready` | a TCP listener with a dial pending and two clients with bytes: all three in one wait; nothing at 0 and at 25 ms (no sooner); the read, the accept and the peer's close | all three, every host |
| `poll/mixed_refused` | the multiplexer's six (two pty masters, a Unix listener with a dial pending, two clients, a pipe), all ready: one waiter `unsupported`, each half served alone, three and three | checked, native |
| `poll/fairness` | a pipe kept readable at key 0 beside a pty master: every turn answers the pipe, the turns the master holds a byte answer it too; the narrow form answers index 0 on all ten | checked, native |
| `poll/deadline` | 0, 25 ms and -1 on the sockets path, an empty waiter, the files path, and meanings alone | checked, native (lupin refuses at the files path) |
| `poll/signal_during_wait` | `child()` ends a 10 s wait on a quiet pipe after the child's 300 ms; a meaning pending before the wait is answered alone and the ready file next | checked, native |
| `poll/write_partial` | a pty master non-blocking: room, `write_some` until `busy`, the 25 ms deadline over a full master, room after a read; a 65536-byte queue drained through the loop, waiting for room at least once, read back in order | checked, native |
| `poll/pty_hangup` | a pty child that prints `x` and ends: `READ`, `x`, `READ` again, the read's `io` (linux; `eof` on macOS), no `HANGUP` fact at this pin, status 0 | checked, native |

`docs/sc82-witness/rows.py` runs each row on lupin, the checked machine and native, holds each line to its directive (`FAIL <test>:<lane>:<key>` at column 0 for a red line), and holds each lupin refusal to its builtin by name. `docs/sc82-witness/rig/` is the recorder's rig, sc81's by value: `gauntlet.manifest` (the rows, then every `cargo xtask ci` step and `ulp`) and `short.manifest` (the rows alone, for the red take and the plant).

### What the rows and probes showed on linux

- A pty master holds 13824 to 15360 bytes before `write_some` answers `busy` (the slave raw and unread); a fifo opened read and write holds 65536.
- A pty master whose last slave descriptor closed is ready to read, and its read is `io` (5 in `os_error()`), with or without a child.
- `net.wait` over an empty set with a positive deadline answers at once rather than waiting it out, so `std.poll` sleeps that deadline itself for a waiter with no entries and no meanings.
- `child()` from the row's own child wakes a files-path wait, and a meaning pending before a wait is answered on the first look with no entry.

## Done-when

- [x] Prediction first (`497f8dd`), the design and the missing calls next (`ea8e952`), scored above.
- [x] The rows runner and rig (`5bfa2ae`); the rows red first (`31edb4d`), recorded red by name there.
- [x] `std.poll` (`2d86e26`); every row green on linux on the lanes its ledger names.
- [x] The partial-write example (`441c6e1`, its handler put on one statement line at `dfb13b9`: §4's extractor reads any line holding `>`, a `match` arm's `=>` included, as an assertion); the narrow forms' docs (`abb7e48`, `bf8dabf`); the M2 and M3 contracts (`bf8dabf`, `d5d842e`).
- [x] README (`b64d13f`, the newest surface and the count, 656) and CHANGELOG (`a159ec0`); lupin never listens, so `poll/signal_during_wait` has no `lupin-windows` cell (`5135a08`).
- [ ] The green record at the final head and the plant's record (red by name): the PR's.
- [ ] macOS: CI's `tier all`. windows: the full tier's rig. PAX: the rows above, for a PAX lane.
- [ ] M1, M2 and M3 in the runtime, then the follow-on that retires `std.poll`'s four refusals and ships `net.write_some` and `unix.peer_credentials`.
