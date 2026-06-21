# Lesson 4 — Async Channels

> **Goal**: Pass data between tasks using Tokio's async channels.

You already know `std::sync::mpsc` from the threading chapter. Async code has its own channel family — same concept, async-safe methods. This lesson covers when to use each kind and builds a task dispatcher that sends jobs to workers and collects results.

---

## Why not `std::sync::mpsc` in async code?

The threading channel works by *blocking* the calling thread:

```rust
let msg = rx.recv().unwrap();  // thread sleeps here until a message arrives
```

Blocking an OS thread is fine in threaded code — the OS just parks that thread and runs another one. But in async code, **one OS thread may be running many tasks at once**. When you block that thread, every task on it freezes:

```
Tokio thread #1 is running tasks A, B, C, D ...

task A calls std::sync::mpsc::rx.recv()
  → the THREAD blocks
  → tasks B, C, D all stop too — they never get polled
  → the runtime is stuck
```

The rule: **async code must never block the thread**. Every operation that waits must yield back to the runtime instead.

---

## `tokio::sync::mpsc` — the async equivalent

Tokio's channel looks familiar:

```rust
use tokio::sync::mpsc;

let (tx, rx) = mpsc::channel(32);  // 32 is the buffer size
```

But `send` and `recv` are both async:

```rust
tx.send(value).await    // yields until there is room in the buffer
rx.recv().await         // yields until a message arrives (or returns None if closed)
```

Neither blocks the thread. If the channel is full or empty, the calling task suspends and the runtime runs something else.

The `32` is the buffer capacity. Unlike `std::sync::mpsc::channel()` (which is unbounded), Tokio's `mpsc` is always bounded. This gives you back-pressure: a fast producer is slowed down when the consumer falls behind, rather than letting memory grow without limit.

---

## Step 1 — Send from a spawned task

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<String>(8);

    tokio::spawn(async move {
        tx.send("hello from task".to_string()).await.unwrap();
    });

    if let Some(msg) = rx.recv().await {
        println!("received: {msg}");
    }
}
```

Notice `mut rx` — the receiver needs to be mutable because `recv` advances its internal state. Also notice that the sender is moved into the task with `async move`, same as `move ||` with threads.

---

## Step 2 — Multiple senders, one receiver

Clone `tx` before spawning each task, same as with `std::sync::mpsc`:

```rust
let (tx, mut rx) = mpsc::channel::<i32>(16);

for i in 0..5 {
    let tx = tx.clone();
    tokio::spawn(async move {
        tx.send(i * i).await.unwrap();
    });
}

drop(tx);  // drop the original so recv() returns None when all tasks finish

while let Some(n) = rx.recv().await {
    println!("got: {n}");
}
```

`rx.recv()` returns `None` when all senders are dropped — the same closing semantics as `std::sync::mpsc`. Drop the original `tx` after cloning or the channel never closes.

---

## `tokio::sync::broadcast` — one sender, all receivers

`mpsc` delivers each message to exactly one receiver. `broadcast` delivers every message to *every* receiver. This is the right tool when multiple tasks all need to see the same events — for example, broadcasting a message to all connected clients.

```rust
use tokio::sync::broadcast;

let (tx, mut rx1) = broadcast::channel::<String>(16);
let mut rx2 = tx.subscribe();  // second receiver — same messages

tx.send("hello everyone".to_string()).unwrap();  // no .await needed for broadcast::send

let m1 = rx1.recv().await.unwrap();
let m2 = rx2.recv().await.unwrap();

println!("rx1 got: {m1}");
println!("rx2 got: {m2}");  // same message
```

`broadcast::Sender::send` is synchronous (it does not need to wait for a receiver to be ready — it writes into the ring buffer). Each call to `tx.subscribe()` creates a new receiver that starts receiving messages sent *after* it subscribed.

---

## `tokio::sync::oneshot` — a single reply

`oneshot` is for the request/reply pattern: send a question, get one answer. It is one sender, one receiver, one message.

```rust
use tokio::sync::oneshot;

let (reply_tx, reply_rx) = oneshot::channel::<u64>();

tokio::spawn(async move {
    // do some work...
    let result = 42_u64;
    reply_tx.send(result).unwrap();
});

let answer = reply_rx.await.unwrap();
println!("answer: {answer}");
```

`reply_rx.await` is a future that resolves when the sender sends its value (or returns `Err` if the sender was dropped without sending).

You will use this pattern in the full program below: the main task sends a job *plus* a `oneshot` sender, and the worker replies on that sender.

---

## When to use which

| Channel | Senders | Receivers | Each message goes to |
|---------|---------|-----------|----------------------|
| `mpsc` | Many | One | One receiver |
| `broadcast` | One (cloneable) | Many | All receivers |
| `oneshot` | One | One | One receiver, exactly once |

The threading chapter used `std::sync::mpsc`. In async code, use these Tokio equivalents — same ideas, different types.

---

## Closing channels

Drop the sender(s). The receiver's `.recv().await` returns `None`:

```rust
let (tx, mut rx) = mpsc::channel::<i32>(4);
drop(tx);

let result = rx.recv().await;
assert!(result.is_none());  // channel is closed
```

This is identical to how `std::sync::mpsc` signals closure — the mechanism transfers cleanly from threads to tasks.

---

## Full code — task dispatcher

The main task sends `Job`s (a number to square) down a channel. Two worker tasks race to pick them up, process them, and reply on a per-job `oneshot` channel. The main task collects all the results.

```rust
use tokio::sync::{mpsc, oneshot};

struct Job {
    input: u64,
    reply: oneshot::Sender<u64>,
}

async fn worker(id: u32, mut jobs: tokio::sync::mpsc::Receiver<Job>) {
    while let Some(job) = jobs.recv().await {
        let result = job.input * job.input;
        println!("worker {id}: {} * {} = {result}", job.input, job.input);
        let _ = job.reply.send(result);
    }
    println!("worker {id}: channel closed, exiting");
}

#[tokio::main]
async fn main() {
    let (job_tx, job_rx) = mpsc::channel::<Job>(8);

    // spawn two workers that share the same receiver
    // Note: mpsc::Receiver cannot be cloned — only one worker can hold it.
    // To fan out to multiple workers we use a second channel, or Arc<Mutex<rx>>.
    // For simplicity here: one receiver, two tasks taking turns.
    // We restructure: one channel per worker, main round-robins.
    let (tx1, job_rx1) = mpsc::channel::<Job>(4);
    let (tx2, job_rx2) = mpsc::channel::<Job>(4);

    tokio::spawn(worker(1, job_rx1));
    tokio::spawn(worker(2, job_rx2));

    let inputs = [3_u64, 7, 12, 5, 9, 2];
    let mut reply_receivers = Vec::new();

    for (i, &n) in inputs.iter().enumerate() {
        let (reply_tx, reply_rx) = oneshot::channel::<u64>();
        let job = Job { input: n, reply: reply_tx };

        // round-robin: even indices go to worker 1, odd to worker 2
        if i % 2 == 0 {
            tx1.send(job).await.unwrap();
        } else {
            tx2.send(job).await.unwrap();
        }

        reply_receivers.push(reply_rx);
    }

    drop(tx1);
    drop(tx2);

    // collect results in submission order (each reply_rx matches its job)
    println!("\nResults:");
    for rx in reply_receivers {
        match rx.await {
            Ok(result) => println!("  → {result}"),
            Err(_)     => println!("  → worker dropped the reply sender"),
        }
    }
}
```

Expected output (worker numbers may interleave differently):
```
worker 1: 3 * 3 = 9
worker 2: 7 * 7 = 49
worker 1: 12 * 12 = 144
worker 2: 5 * 5 = 25
worker 1: 9 * 9 = 81
worker 2: 2 * 2 = 4
worker 1: channel closed, exiting
worker 2: channel closed, exiting

Results:
  → 9
  → 49
  → 144
  → 25
  → 81
  → 4
```

The results arrive in submission order because we await each `reply_rx` in sequence — the `oneshot` channels track which result belongs to which job.

---

## Exercise

> **TODO 1**: Implement a rate-limited channel: create an `mpsc::channel(4)` and spawn 20 tasks that each try to send immediately. Observe how the buffer limits how many are in flight at once. Add a `tokio::time::sleep(Duration::from_millis(50))` to the receiver loop and watch the senders back up.
>
> **TODO 2**: Add a timeout to receiving: wrap `rx.recv().await` in `tokio::time::timeout(Duration::from_secs(1), rx.recv())`. If the timeout fires before a message arrives, print `"timed out"` and break. Test it by having the sender sleep for 2 seconds before sending.
>
> **TODO 3**: Implement a simple pub/sub system using `broadcast`: spawn 3 subscriber tasks that each call `tx.subscribe()`, then the main task sends 5 events. Each subscriber should print every event it receives with its own ID. Verify that all 3 subscribers see all 5 events.

---

## Key APIs

| API | What it does |
|-----|-------------|
| `mpsc::channel(n)` | Bounded channel — `n` messages fit in the buffer before senders yield |
| `tx.send(v).await` | Send a value; yields if the buffer is full |
| `rx.recv().await` | Receive a value; yields if empty; `None` when all senders dropped |
| `tx.clone()` | Create a second sender to the same `mpsc` channel |
| `broadcast::channel(n)` | Every receiver gets every message; ring buffer of size `n` |
| `tx.subscribe()` | Create a new `broadcast` receiver |
| `oneshot::channel()` | Single-use channel for one request/reply pair |
| `reply_rx.await` | Wait for the oneshot sender to send (or drop) |
