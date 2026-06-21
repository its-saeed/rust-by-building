# Project 14 — Async Chat Server

> **What you'll build**: An async TCP chat server where multiple clients connect, send messages, and every connected client receives every message. Same visible behaviour as Project 11 — different internals.
>
> **Lessons**: 2 lessons.
>
> **Rust concepts covered**: `tokio::net`, `tokio::spawn`, `tokio::sync::broadcast`, `tokio::select!`, async I/O, split TCP streams.

---

## What you are building

The same three-program layout as Project 11:

```
src/
  bin/
    server.rs   ← listens for TCP connections, broadcasts messages
    client.rs   ← connects, reads stdin, prints incoming messages
```

Run one server, as many clients as you like:

```sh
cargo run --bin server

# in separate terminals:
cargo run --bin client
cargo run --bin client
cargo run --bin client
```

Type a line in any client. It appears on all the others. Disconnect one — the rest keep working.

---

## Why redo it with async?

Project 11 solved the two-direction problem — reading from a client while broadcasting to all others — with threads. One thread per client, each thread blocking on `read()`.

That works. But threads are expensive. Each one allocates a stack (typically 2–8 MB), an OS entry in the thread table, and a kernel scheduling slot. A server with 10,000 connected clients needs 10,000 threads — and most of them are just sleeping, waiting for a byte that has not arrived yet.

Async tasks are lighter. They do not block the OS thread. When an async task is waiting for I/O, the runtime parks it and runs another task on the same thread. A single OS thread can juggle thousands of waiting tasks.

```
threaded:  client1 → thread1 (sleeping on read)
           client2 → thread2 (sleeping on read)
           client3 → thread3 (sleeping on read)

async:     client1 ─╮
           client2 ─┤→ tokio task pool (2–8 threads total)
           client3 ─╯
```

The code looks similar. The runtime behaviour is fundamentally different.

---

## What is new compared to Project 11

| Project 11 | Project 14 |
|-----------|-----------|
| `std::net::TcpListener` | `tokio::net::TcpListener` |
| `thread::spawn` | `tokio::spawn` |
| Custom `Event` enum + `mpsc` channel | `tokio::sync::broadcast` channel |
| Two threads in the client (one per direction) | `tokio::select!` in one task |

Each change is a direct swap. The shape of the program stays the same; async is the engine underneath.

---

## The new tool: `tokio::select!`

In Project 11 the client used two threads: one blocking on stdin, one blocking on the socket. With async, both are futures. `tokio::select!` waits for whichever future is ready first:

```rust
tokio::select! {
    line = stdin_lines.next_line() => { /* user typed something */ }
    msg  = server_lines.next_line() => { /* server sent something */ }
}
```

Neither branch blocks the other. One task handles both directions.

---

## What you will build across the lessons

- **Lesson 1** — The async server: `TcpListener::bind().await`, `tokio::spawn`, `broadcast::channel`, `select!` inside `handle_client`
- **Lesson 2** — The async client: `TcpStream::connect().await`, async stdin, `select!` to read and send simultaneously
