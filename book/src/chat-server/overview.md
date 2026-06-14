# Project 10 — Chat Server

You have built a UDP relay (Tele-Sketch). Now build a TCP chat server where any number of clients can connect and every message one client types appears on all the others' screens — in real time.

---

## What you are building

Three programs in one crate:

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

Type a line in any client. It appears on all the others.

---

## What makes this different from Tele-Sketch

Tele-Sketch used UDP: fire-and-forget datagrams, a drain loop, implicit peer registration. This project uses TCP: reliable ordered streams, a blocking `accept` loop, and **one thread per client**.

The central challenge: `TcpListener::accept` blocks. `TcpStream::read` blocks. If you handle clients one at a time in a loop, the second client waits for the first to disconnect. The fix is the thread-per-client pattern — the same reason web servers spawn threads.

---

## Architecture

```
                 ┌─────────────────────────────┐
                 │           server            │
                 │                             │
  client A ──TCP──▶  thread A  ──msg──╮        │
                 │                    │        │
  client B ──TCP──▶  thread B  ──msg──┼──▶  broadcaster thread
                 │                    │     (main thread)      ──▶  all clients
  client C ──TCP──▶  thread C  ──msg──╯        │
                 │                             │
                 └─────────────────────────────┘
```

Each connection thread:
1. Reads lines from its `TcpStream`
2. Sends each line through a channel to the broadcaster

The broadcaster (main thread after accepting):
1. Holds a `Vec` of all connected streams
2. Receives messages from the channel
3. Writes each message to every stream

---

## Concepts applied

| Concept | Where it appears |
|---------|-----------------|
| `thread::spawn` | One thread per accepted connection |
| `move` closure | Connection stream moves into the thread |
| `mpsc::channel` | Connection threads send messages to the broadcaster |
| `tx.clone()` | Each connection thread gets its own sender |
| `rx` as a loop | Broadcaster loops on `rx` to receive and relay |
| TCP streams | `TcpListener`, `TcpStream`, `BufReader` |

---

## What you will build across the lessons

- **Lesson 1** — Server skeleton: accept loop, spawn a thread per client, echo back to the same client
- **Lesson 2** — Broadcast: add a channel, pass clones of `tx` to each thread, broadcaster relays to all clients
- **Lesson 3** — The client: connect to the server, read stdin in one thread, print incoming in another
