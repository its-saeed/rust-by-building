# Chapter 2 — Non-blocking I/O

The OS has two modes for reading a socket — and switching from one to the other changes everything about how servers are built.

---

## Blocking I/O: the default

When a thread calls `read()` on a socket that has no data, the OS **blocks** the thread:

```
thread:   read() ──────────────────────────────▶ data arrives ──▶ returns
                  ↑ blocked here — thread is
                    parked by the kernel,
                    consumes no CPU,
                    but holds its 2 MB stack
```

This is exactly what the chat server threads do. Each thread calls `read()` and waits. The kernel is efficient about it — a blocked thread uses zero CPU — but as we saw in chapter 1, the stack and scheduling overhead of thousands of blocked threads still adds up.

The thread cannot do anything else while it is blocked. It just holds its resources and waits.

---

## Non-blocking I/O: the other mode

Every socket can be put into **non-blocking mode** with a single flag:

```c
// C equivalent — Rust's std and mio both do this internally
fcntl(fd, F_SETFL, O_NONBLOCK);
```

Now when a thread calls `read()` on a socket with no data, the call returns immediately with an error:

```
thread:   read() ──▶ returns immediately with EAGAIN
                     (or EWOULDBLOCK — same thing, different name)
```

`EAGAIN` means "try again later — there is nothing here right now." The thread is not blocked. It is free to go do something else.

| Mode | `read()` with no data | Thread state |
|------|----------------------|--------------|
| Blocking (default) | Sleeps until data arrives | Blocked — cannot do other work |
| Non-blocking | Returns `EAGAIN` immediately | Running — can do other work |

---

## The naive polling loop — and why it fails

The obvious first reaction: if `read()` returns `EAGAIN`, just try again.

```rust
// do NOT do this
loop {
    match socket.read(&mut buf) {
        Ok(n)  => process(&buf[..n]),
        Err(e) if e.kind() == WouldBlock => continue,  // try again immediately
        Err(e) => return Err(e),
    }
}
```

This **burns 100% of a CPU core** doing nothing useful — checking a socket thousands of times per second when there is nothing to read. It is the software equivalent of checking your phone every half-second waiting for a text message. Non-blocking I/O without any coordination mechanism is wasteful in a different way from blocking I/O.

We need a smarter approach: instead of checking constantly, tell the OS what we are waiting for and let it tell us when something is ready.

---

## Readiness notifications

Operating systems provide APIs that let a thread register interest in many file descriptors at once, then block on a single call until at least one of them is ready:

```
thread says:  "I care about sockets A, B, C, D, ..., Z"
              "Tell me when any of them have data to read or space to write"

OS says:      "OK, I will wake you when something happens"

thread:       epoll_wait() ──── blocks ────▶ socket C has data!
              (thread is sleeping — using no CPU)

OS wakes thread with list of ready sockets: [C]

thread:       read from C, process it, go back to epoll_wait()
```

The key difference from blocking I/O: one thread can watch thousands of sockets simultaneously. It blocks on `epoll_wait()` — not on reading any specific socket — and the OS returns a list of whichever sockets became ready.

---

## The OS APIs

Each major operating system has its own readiness notification API:

| OS | API | Notes |
|----|-----|-------|
| Linux | `epoll` | Added in kernel 2.5.44 (2002) |
| macOS, BSD | `kqueue` | Also handles files, processes, signals |
| Windows | `IOCP` | Completion-based rather than readiness-based |
| POSIX (all) | `select`, `poll` | Older, less scalable; limited to ~1024 fds |

`epoll` on Linux is the one you will see mentioned most often. It can watch millions of file descriptors efficiently — the kernel maintains a tree structure internally so the cost of calling `epoll_wait()` does not grow with the number of registered descriptors.

`kqueue` is macOS's equivalent and is similarly scalable. `IOCP` on Windows works differently (the OS calls you back when I/O *completes* rather than telling you when it *could* succeed), but serves the same purpose.

---

## One thread, many connections

Here is what this looks like with 10,000 connections:

```
┌────────────────────────────────────────────────────────────────┐
│  one thread watching 10,000 sockets via epoll                  │
│                                                                │
│  sockets:  A  B  C  D  E  F  G ... (10,000 total)             │
│            ░  ░  █  ░  ░  █  ░                                │
│               no data  ↑       ↑                               │
│                       has data has data                        │
│                                                                │
│  epoll_wait() returns: [C, F]                                  │
│                                                                │
│  thread:  read C → process C → read F → process F             │
│           → epoll_wait() again (blocks until more are ready)   │
│                                                                │
│  ░ = socket registered, watched, uses no stack, no scheduling  │
│  █ = socket has data — thread handles it now                   │
└────────────────────────────────────────────────────────────────┘
```

The 9,998 idle sockets consume almost no resources. They are not threads. They are entries in a kernel data structure, waiting for the OS to notice activity on their file descriptors.

---

## `mio` — the Rust bridge

Writing directly to `epoll` / `kqueue` / `IOCP` requires platform-specific C-style calls. The Rust crate **`mio`** (Metal I/O) wraps all three into a single cross-platform API:

```rust
use mio::{Events, Interest, Poll, Token};
use mio::net::TcpListener;

let mut poll = Poll::new()?;
let mut events = Events::with_capacity(128);

// register a listener socket with mio
poll.registry().register(&mut listener, Token(0), Interest::READABLE)?;

loop {
    poll.poll(&mut events, None)?;  // blocks until something is ready

    for event in events.iter() {
        match event.token() {
            Token(0) => { /* listener has a new connection */ }
            _        => { /* a client socket has data */ }
        }
    }
}
```

You will not write `mio` code directly in this course. But **Tokio**, the async runtime we use in the next project, is built on top of `mio`. Every `.await` on a network read in Tokio eventually becomes an `epoll_wait()` or equivalent.

Knowing that `mio` exists explains why Tokio works — it is not magic, it is a well-understood OS feature.

---

## What non-blocking I/O actually solves

| Problem from chapter 1 | Solution |
|------------------------|----------|
| 10,000 threads × 2 MB stack = 20 GB | One thread, no per-connection stacks |
| Context switching overhead | One thread — no switching needed |
| Threads idle 99.99% of the time | epoll only returns sockets that have work |
| Scheduler managing 10,000 entities | Scheduler only sees one thread |

One thread can handle 10,000 connections — but only if it never blocks on any one of them. Every `read()` must be non-blocking. Every `write()` must be non-blocking. The moment a handler blocks, the entire server stalls.

That constraint — handlers must not block — is the central challenge of building on top of non-blocking I/O. The next chapter shows the pattern that organises it.

---

## Key ideas

| Concept | What it means |
|---------|--------------|
| Blocking I/O | `read()` sleeps the thread until data arrives |
| Non-blocking I/O | `read()` returns `EAGAIN` immediately if no data; thread stays awake |
| Busy polling | Calling `read()` in a tight loop wastes CPU — do not do it |
| Readiness notification | OS APIs (`epoll`, `kqueue`, `IOCP`) that say "this fd is ready" |
| `mio` | Rust crate wrapping all platform readiness APIs into one interface |
| One thread, many connections | The key scalability win — no per-connection stack or scheduling overhead |
