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

Build each row's native binary on linux with the pinned wolf (`conform-run --native` builds and runs it; `wolf build` gives the binary), copy it to PAX, run it in a scratch directory, and compare its standard output with the row's directive line by line. The rows need `/bin/sh`, `printf` and `sleep` on PAX. Run: `poll/waiter_rows`, `poll/files_all_ready`, `poll/sockets_all_ready`, `poll/mixed_refused`, `poll/fairness`, `poll/deadline`, `poll/signal_during_wait`, `poll/write_partial` and `poll/pty_hangup`. `poll/write_partial` does not print the room it measured, so PAX's 20480-byte pty master holds it as linux's does.

## Forbidden

- No libc source, no kernel source and no source of the compiler's or the interpreter's crates was read. wolf-lang was read at the tag only in `spec/11-os.md` and `spec/prelude.json`, and its trunk's log since the tag.
- Host behaviour is measured black-box through the rows.
