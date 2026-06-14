# Lesson 2 — Message Passing with Channels

Spawning threads is easy. The harder question is: how do threads communicate? The cleanest answer is **channels** — one thread sends a value, another receives it.

The idea comes from a simple principle: instead of sharing data and hoping two threads do not collide, transfer ownership of data from one thread to another. At any moment, exactly one thread owns the data. No two threads are ever touching it at the same time.

---

## Creating a channel

`std::sync::mpsc::channel` creates a connected sender/receiver pair:

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();
```

`tx` (transmitter) sends values. `rx` (receiver) receives them. They are linked — values sent through `tx` come out of `rx`.

`mpsc` stands for *multiple producer, single consumer*: you can clone `tx` to get multiple senders, but there is only one `rx`.

---

## Sending from a thread

Move `tx` into a spawned thread, send a value, let the thread finish:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("hello from thread").unwrap();
    });

    let message = rx.recv().unwrap();
    println!("received: {message}");
}
```

`tx.send(value)` moves `value` into the channel. Ownership transfers — the sending thread no longer has it. `rx.recv()` blocks until a message is available, then returns it.

The spawned thread moves `tx` in (`move ||`), sends, and exits. The main thread receives. Two threads, clean handoff, no shared mutable state.

---

## Multiple messages

Send as many values as you like. `rx.recv()` returns `Err` when all senders are dropped (the channel is closed):

```rust
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    for i in 0..5 {
        tx.send(i).unwrap();
    }
    // tx is dropped here → channel closes
});

for message in rx {          // iterating rx calls recv() in a loop
    println!("got: {message}");
}
// loop ends when channel is closed
```

Iterating over `rx` is the idiomatic way to receive all messages until the sender is gone.

---

## Multiple senders

Clone `tx` before moving it to give each thread its own sender:

```rust
let (tx, rx) = mpsc::channel::<String>();

let tx1 = tx.clone();
let tx2 = tx.clone();
drop(tx); // drop the original so the channel closes when tx1 and tx2 are done

thread::spawn(move || tx1.send("from thread 1".into()).unwrap());
thread::spawn(move || tx2.send("from thread 2".into()).unwrap());

for msg in rx {
    println!("{msg}");
}
```

The channel stays open until every clone of `tx` is dropped. Drop the original `tx` after cloning if you want the channel to close when the threads finish.

---

## Bounded vs unbounded channels

`mpsc::channel()` creates an **unbounded** channel — senders never block (memory grows until the receiver catches up). This is fine for most cases.

`mpsc::sync_channel(n)` creates a **bounded** channel — senders block if there are more than `n` messages waiting:

```rust
let (tx, rx) = mpsc::sync_channel(8); // buffer up to 8 messages
```

Bounded channels add **back-pressure**: fast producers are slowed down when the consumer falls behind, instead of letting memory grow without limit.

---

## A real-world shape: worker pool

Channels naturally express a worker pool — one thread produces work items, multiple workers consume them:

```rust
let (tx, rx) = mpsc::channel::<String>();
let rx = Arc::new(Mutex::new(rx)); // skip for now — covered when needed

// producer
thread::spawn(move || {
    for url in urls {
        tx.send(url).unwrap();
    }
});

// workers
for _ in 0..4 {
    // each worker calls rx.recv() in a loop
}
```

You will build exactly this pattern in the chat server project, but instead of a worker pool the shape is: each connection thread sends received messages to one broadcaster thread.

---

## `recv` vs `try_recv`

| Method | Behaviour |
|--------|-----------|
| `rx.recv()` | Block until a message is available or the channel is closed |
| `rx.try_recv()` | Return immediately: `Ok(msg)` if available, `Err(TryRecvError::Empty)` if not |

`try_recv` is useful in a game loop or any context where you do not want to block:

```rust
loop {
    match rx.try_recv() {
        Ok(msg)  => handle(msg),
        Err(_)   => {} // nothing waiting
    }
    // continue with other work
}
```

This is the same non-blocking drain pattern from Tele-Sketch — applied to channels instead of UDP sockets.

---

## Exercise

> **TODO 1**: Create a channel where the main thread sends five `String` messages to a spawned thread that prints them. Then reverse it: the spawned thread generates five strings and the main thread prints them.
>
> **TODO 2**: Spawn 3 producer threads, each sending 4 numbers. Main thread receives all 12 and computes the total. Make sure the channel closes cleanly after all producers finish.
>
> **TODO 3**: Implement a simple pipeline: thread A generates numbers 1–10, sends them to thread B, which doubles them and sends to thread C, which prints them. Three threads, two channels.

---

## Key APIs

| API | What it does |
|-----|-------------|
| `mpsc::channel()` | Create an unbounded sender/receiver pair |
| `mpsc::sync_channel(n)` | Create a bounded channel (senders block when buffer full) |
| `tx.send(value)` | Send a value — moves ownership into the channel |
| `rx.recv()` | Block until a message arrives; `Err` when all senders dropped |
| `rx.try_recv()` | Non-blocking receive; `Err(Empty)` if nothing waiting |
| `tx.clone()` | Create a second sender to the same channel |
| `for msg in rx` | Receive all messages until channel closes |
