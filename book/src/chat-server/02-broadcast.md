# Lesson 2 — Broadcasting with Channels

Echo is working: each client gets their own messages back. Now make it a real chat server: any message one client sends should appear on every other client's screen.

The challenge is coordination. Multiple threads — one per client — need to deliver messages to a single list of connected clients. How do threads share that list safely?

---

## Why not a shared Vec?

The obvious approach: put the writers in a `Vec`, wrap it in a `Mutex` so only one thread touches it at a time, wrap that in an `Arc` so multiple threads can hold a reference to it.

```rust
// the "obvious" approach — not what we will do
let writers = Arc::new(Mutex::new(Vec::<TcpStream>::new()));
```

This works, but it has a problem: every time a client sends a message, its thread has to lock the mutex, write to all writers, and unlock. If ten clients send messages simultaneously, nine threads are blocked waiting for the one holding the lock. Under load, this serialises all writes through a single bottleneck.

There is a cleaner design. Instead of sharing the `Vec` across threads, keep it in exactly one place — the main thread — and let all other threads talk to it through a channel. Only one thread ever touches the `Vec`; no locking needed at all.

---

## The design

```
                         ┌─────────────────────────────────────┐
                         │             server                  │
                         │                                     │
  client A connects ───▶ │  accept thread                      │
  client B connects ───▶ │    for each connection:             │
  client C connects ───▶ │      try_clone → send NewClient ──╮ │
                         │      spawn reader thread           │ │
                         │                                    │ │
                         │  reader A  ──── Message("hi") ────┤ │
                         │  reader B  ──── Message("hey") ───┤ │
                         │  reader C  ──── Message("!") ─────┘ │
                         │                    channel (mpsc)    │
                         │                         │            │
                         │  broadcaster (main) ◀───┘            │
                         │    match event:                      │
                         │      NewClient → writers.push()      │
                         │      Message   → write to all        │
                         │                                      │
                         └─────────────────────────────────────┘
```

Three roles:
1. **Accept thread** — waits for connections; for each, sends a writer through the channel and spawns a reader thread
2. **Reader threads** — one per client; reads lines and sends them through the channel
3. **Broadcaster (main thread)** — receives everything from the channel; maintains the writer list; writes each message to all clients

All communication flows through one channel. The broadcaster never shares state with anyone — it only receives events and acts on them.

---

## Step 1 — The Event enum

The channel carries two kinds of things: a new writer when a client connects, and a message when a client types. An enum unifies them into one type:

```rust
use std::net::TcpStream;

enum Event {
    NewClient(TcpStream),  // a writer for a newly connected client
    Message(String),       // text to broadcast to everyone
}
```

A single `mpsc::channel::<Event>()` carries both. The broadcaster `match`es on the variant to know what to do.

Without the enum, you would need two separate channels — one for writers, one for messages — and the broadcaster would need to check both on every iteration, which is awkward with `std::sync::mpsc`.

---

## Step 2 — The broadcaster loop

The broadcaster is the simplest part. It keeps a `Vec<TcpStream>` of all active writers and processes events one at a time:

```rust
use std::io::Write;

let mut writers: Vec<TcpStream> = Vec::new();

for event in rx {
    match event {
        Event::NewClient(writer) => {
            writers.push(writer);
        }
        Event::Message(msg) => {
            writers.retain_mut(|w| writeln!(w, "{msg}").is_ok());
        }
    }
}
```

**`retain_mut` is doing double duty here.** It iterates the vec and keeps the element only if the closure returns `true`. Writing to a disconnected client returns an `Err`, so `.is_ok()` returns `false` — and that client is silently removed from the list. No separate disconnection handling, no tombstone flags, no second pass to clean up. The write attempt and the cleanup happen in one step.

```
before Message("hello"):
  writers: [A, B, C]     (C disconnected without us knowing yet)

retain_mut tries each:
  write to A → Ok(())    → keep
  write to B → Ok(())    → keep
  write to C → Err(...)  → remove

after:
  writers: [A, B]
```

---

## Step 3 — The accept thread and sender tree

The accept loop must run concurrently with the broadcaster, so it lives in its own thread. Each reader thread also needs a sender. This means `tx` gets cloned into a tree:

```
tx (original) ── drop(tx) immediately
  │
  ├── tx_accept (clone) → accept thread
  │     │
  │     ├── tx_reader_A (clone of tx_accept) → reader thread A
  │     ├── tx_reader_B (clone of tx_accept) → reader thread B
  │     └── tx_reader_C (clone of tx_accept) → reader thread C
  │
  └── rx → broadcaster (main thread)
```

The channel stays open as long as at least one `tx` clone exists. When a reader thread finishes, its `tx_reader` is dropped. When all clients disconnect and the accept thread exits (if the server is shutting down), `tx_accept` is dropped. At that point, `rx` ends and the broadcaster's `for event in rx` loop terminates.

```rust
use std::sync::mpsc;
use std::thread;
use std::net::TcpListener;
use std::io::{BufRead, BufReader};

let (tx, rx) = mpsc::channel::<Event>();

let tx_accept = tx.clone();
thread::spawn(move || {
    let listener = TcpListener::bind("0.0.0.0:8080").expect("bind failed");
    println!("listening on :8080");

    for stream in listener.incoming() {
        let stream = stream.expect("accept failed");

        // clone stream: writer goes to broadcaster, reader stays here
        let writer = stream.try_clone().expect("clone failed");
        tx_accept.send(Event::NewClient(writer)).ok();

        // each reader thread gets its own sender clone
        let tx_reader = tx_accept.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stream);
            for line in reader.lines() {
                match line {
                    Ok(text) => { tx_reader.send(Event::Message(text)).ok(); }
                    Err(_)   => break,  // client disconnected
                }
                // tx_reader dropped when thread exits
            }
        });
    }
    // tx_accept dropped when accept thread exits
});

drop(tx);  // ← critical: drop the original so rx can end
```

`drop(tx)` is easy to forget and causes a subtle bug: if the original `tx` is never dropped, `rx` never ends — the broadcaster's `for event in rx` loops forever even after every client disconnects and every thread exits.

---

## Full server

```rust
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;

enum Event {
    NewClient(TcpStream),
    Message(String),
}

fn main() {
    let (tx, rx) = mpsc::channel::<Event>();

    let tx_accept = tx.clone();
    thread::spawn(move || {
        let listener = TcpListener::bind("0.0.0.0:8080").expect("bind failed");
        println!("listening on :8080");

        for stream in listener.incoming() {
            let stream = stream.expect("accept failed");
            let writer = stream.try_clone().expect("clone failed");
            tx_accept.send(Event::NewClient(writer)).ok();

            let tx_reader = tx_accept.clone();
            thread::spawn(move || {
                let reader = BufReader::new(stream);
                for line in reader.lines() {
                    match line {
                        Ok(text) => { tx_reader.send(Event::Message(text)).ok(); }
                        Err(_)   => break,
                    }
                }
            });
        }
    });
    drop(tx);

    let mut writers: Vec<TcpStream> = Vec::new();

    for event in rx {
        match event {
            Event::NewClient(writer) => {
                println!("client connected  (total: {})", writers.len() + 1);
                writers.push(writer);
            }
            Event::Message(msg) => {
                writers.retain_mut(|w| writeln!(w, "{msg}").is_ok());
            }
        }
    }
}
```

---

## Test it

```sh
# terminal 1
cargo run --bin server

# terminal 2
nc 127.0.0.1 8080

# terminal 3
nc 127.0.0.1 8080
```

Type in terminal 2 — the message appears in terminal 3, and vice versa. Disconnect one with `Ctrl+C` — the other keeps working. The server log shows connection counts going up and down.

---

## What the design buys you

| | Arc<Mutex<Vec>> | Channel + broadcaster |
|--|--|--|
| Synchronisation | Lock on every message | None — single owner |
| Disconnection cleanup | Separate pass or lock-while-iterating | `retain_mut` handles it inline |
| Threads blocked waiting | Yes — under load | No — broadcaster is never locked |
| Code complexity | Higher — shared state across threads | Lower — each thread has one job |

The channel design is not always better — if you need many threads to both read and write the list concurrently, `Arc<RwLock<>>` might be cleaner. Here, one thread owns the list and everyone else just sends events, which is a natural fit for channels.

---

## Exercise

> **TODO 1**: Add a `Disconnect(SocketAddr)` variant to `Event`. Send it after the reader loop exits. In the broadcaster, print `"{addr} left — {n} clients remaining"`.
>
> **TODO 2**: Prefix each broadcast with the sender's address: `"[127.0.0.1:54321] hello"`. Change `Message(String)` to `Message { text: String, from: SocketAddr }`. Capture `stream.peer_addr()` before `try_clone`.
>
> **TODO 3**: Currently a client receives its own messages back. Fix it: skip the writer whose address matches the sender. You will need the sender's `SocketAddr` in the `Message` variant and the receiver's address stored alongside each writer.

---

## Key APIs

| API | What it does |
|-----|-------------|
| `mpsc::channel::<Event>()` | Channel where messages carry an `Event` enum |
| `tx.clone()` | Second sender to the same channel |
| `drop(tx)` | Close original — `rx` ends when all clones are dropped |
| `for event in rx` | Block until next event; ends when channel closes |
| `writers.retain_mut(\|w\| ...)` | Keep only writers for which the closure returns `true` |
| `writeln!(w, "{msg}").is_ok()` | Write a line; `false` if client is disconnected |
