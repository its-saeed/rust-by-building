# Lesson 2 — Broadcasting with Channels

Echo works, but every client only sees their own messages. The goal is: anything one client sends, every connected client receives. That requires the connection threads to share information — and the cleanest way to share information between threads is a channel.

---

## The design

Each connection thread reads lines and sends them somewhere central. That central place — the **broadcaster** — holds a list of all connected clients and writes each message to all of them.

```
accept thread
  │  for each new connection:
  │    clone stream → send writer to broadcaster
  │    spawn reader thread
  │
  ├── reader thread A ──"hello"──╮
  ├── reader thread B ──"world"──┤──▶  broadcaster (main thread)
  └── reader thread C ──"!"─────╯       │
                                         ├── write to client A
                                         ├── write to client B
                                         └── write to client C
```

The broadcaster needs two things from the outside: new client writers (when someone connects) and messages (when someone types). Both arrive through one channel as an **enum**.

---

## Step 1 — The Event enum

```rust
use std::net::TcpStream;

enum Event {
    NewClient(TcpStream),  // a new writer to add to the list
    Message(String),       // a line to broadcast to everyone
}
```

A single channel of `Event` carries both kinds of updates. The broadcaster switches on the variant to decide what to do.

---

## Step 2 — The broadcaster loop

The broadcaster is the main thread after the accept thread is spawned. It maintains a `Vec` of writers — one per connected client:

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

`retain_mut` iterates the vec and keeps only elements for which the closure returns `true`. Writing to a disconnected client returns an error — `is_ok()` returns false — so that writer is automatically removed. No cleanup needed elsewhere.

---

## Step 3 — The accept thread

The accept loop needs to run concurrently with the broadcaster, so it goes in a thread. For each new connection it:
1. Clones the stream and sends the writer to the broadcaster
2. Spawns a reader thread with the original stream

Both the accept thread and each reader thread need a sender — so we clone `tx` freely:

```rust
use std::sync::mpsc;
use std::thread;
use std::net::TcpListener;

let (tx, rx) = mpsc::channel::<Event>();

let tx_accept = tx.clone();
thread::spawn(move || {
    let listener = TcpListener::bind("0.0.0.0:8080").expect("bind failed");
    println!("listening on :8080");

    for stream in listener.incoming() {
        let stream = stream.expect("accept failed");
        let writer = stream.try_clone().expect("clone failed");

        // send the writer to the broadcaster
        tx_accept.send(Event::NewClient(writer)).ok();

        // spawn a reader thread for this connection
        let tx_reader = tx_accept.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stream);
            for line in reader.lines() {
                match line {
                    Ok(text) => { tx_reader.send(Event::Message(text)).ok(); }
                    Err(_)   => break,
                }
            }
            // tx_reader is dropped here — one fewer sender
        });
    }
    // tx_accept is dropped here when listener exits
});

drop(tx); // close the original — broadcaster ends when all threads finish
```

`drop(tx)` is important: if we keep the original sender, `rx` would never end — even after every thread exits. Dropping it means `rx` ends as soon as the last thread drops its clone.

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
                println!("new client connected");
                writers.push(writer);
            }
            Event::Message(msg) => {
                println!("broadcasting: {msg}");
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

Type in either terminal. The message appears in the other. The server logs each broadcast.

---

## What the channel buys you

Notice what is *not* in this code: no `Mutex`, no `Arc`, no locks. The `writers` vec lives entirely in the main thread — no other thread touches it. The channel is the only point of coordination. Ownership of data moves cleanly: each string moves from the reader thread into the channel, arrives in the broadcaster, gets written to each client.

This is the message-passing model from lesson 2 applied to a real problem.

---

## Exercise

> **TODO 1**: Add a `Disconnect` variant to `Event`. Send it when a reader thread's loop ends. In the broadcaster, print the client count when someone disconnects.
>
> **TODO 2**: Prefix each broadcast with a client identifier so recipients can tell who sent it. Add `addr: SocketAddr` to `Message(String)` → `Message { text: String, from: SocketAddr }`. Get the address before moving the stream into BufReader.
>
> **TODO 3**: The server broadcasts to all clients including the sender. Change it so the sender's own message is not echoed back. You will need to include the sender's address in `Message` and skip the matching writer.

---

## Key APIs

| API | What it does |
|-----|-------------|
| `enum Event { ... }` | Unified message type for the channel |
| `tx.clone()` | Give each thread its own sender |
| `drop(tx)` | Close the original sender so `rx` ends when all threads finish |
| `writers.retain_mut(\|w\| ...)` | Keep only writers where the closure returns `true` |
| `writeln!(writer, "{msg}")` | Write a line to a TCP stream |
