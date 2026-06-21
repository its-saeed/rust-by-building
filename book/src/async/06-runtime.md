# Chapter 6 — The Runtime

Think of a factory floor manager: the workers know their jobs, but someone has to assign them to machines, tell them when to start, and notice when a machine becomes available. Futures are the workers — they know what to do. The **runtime** is the manager.

---

## The problem: who calls `poll()`?

An `async fn` returns a future. A future does nothing until polled. But `fn main()` is synchronous — it has no runtime behind it, no scheduler to call `poll()`. If you try to `.await` directly in `main()`, the compiler refuses:

```rust
fn main() {
    let result = greet("world").await;   // compile error
}
```

```
error[E0728]: `await` is only allowed inside `async` functions and blocks
```

Something has to bridge the gap between synchronous `main()` and the async world. That something is the runtime.

---

## What a runtime provides

A Rust async runtime has two main parts:

```
┌─────────────────────────────────────────────────────────────┐
│                    async runtime                            │
│                                                             │
│  ┌───────────────────────┐  ┌───────────────────────────┐  │
│  │       executor        │  │         reactor           │  │
│  │                       │  │                           │  │
│  │  Holds a queue of     │  │  Watches OS events:       │  │
│  │  ready tasks.         │  │  epoll (Linux),           │  │
│  │                       │  │  kqueue (macOS).          │  │
│  │  Pulls tasks off the  │  │                           │  │
│  │  queue and calls      │  │  When a socket has data,  │  │
│  │  poll() on them.      │  │  fires the waker for the  │  │
│  │                       │  │  task waiting on it.      │  │
│  │  If poll() returns    │  │                           │  │
│  │  Pending, the task    │  │  This moves that task     │  │
│  │  leaves the queue     │  │  back into the executor   │  │
│  │  until woken.         │  │  queue.                   │  │
│  └───────────────────────┘  └───────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

The **executor** drives futures. The **reactor** watches the OS for I/O events. Together they form the event loop described in Chapter 3 — the executor is the loop, the reactor is the `epoll`/`kqueue` integration.

---

## tokio

tokio is the dominant Rust async runtime. It is what the ecosystem — `reqwest`, `axum`, `sqlx`, `tonic` — assumes you are using.

What tokio provides beyond a bare executor:

| Component | What it does |
|-----------|-------------|
| Thread pool executor | Multiple worker threads; tasks are stolen between threads for balance |
| Reactor | OS event integration (`epoll`, `kqueue`, IOCP) for zero-overhead I/O waiting |
| `tokio::net` | Async `TcpListener`, `TcpStream`, `UdpSocket` |
| `tokio::fs` | Async file I/O (runs on a blocking thread pool behind the scenes) |
| `tokio::time` | Async `sleep`, `timeout`, `interval` |
| `tokio::sync` | Async-aware `Mutex`, `RwLock`, channels (`mpsc`, `broadcast`, `oneshot`) |

You could use Rust async without tokio — but you would need to replace all of these yourself.

---

## `#[tokio::main]`

The `#[tokio::main]` attribute macro is how you start the tokio runtime and enter the async world from `main()`:

```rust
#[tokio::main]
async fn main() {
    let result = greet("world").await;
    println!("{result}");
}
```

The macro expands to approximately this:

```rust
fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main())
}
```

`block_on` is the bridge: it runs a future on the current thread, blocking until the future completes. This is the one place where blocking is acceptable — we have to start somewhere.

`enable_all()` turns on the reactor (I/O and timer support). Without it, `tokio::net` and `tokio::time` would panic.

---

## Tasks vs futures

A `Future` is just a value — an inert struct on the heap. It does nothing until polled.

A **task** is a future that has been handed to the runtime to drive independently, like a lightweight thread. You create a task with `tokio::spawn`:

```rust
let handle = tokio::spawn(async {
    do_something().await;
});

// handle is a JoinHandle — you can .await it to get the result
let result = handle.await?;
```

The difference:

| | Future | Task |
|---|--------|------|
| Created by | `async fn` call or `async {}` | `tokio::spawn(future)` |
| Driven by | Whatever `.await`s it | The runtime, independently |
| Runs when | Its parent is polled | Immediately, in parallel with other tasks |
| Analogy | A function call | A thread |

When you `.await` a future in your current task, execution suspends until that future is done. When you `tokio::spawn` a future, it runs concurrently — your current task and the spawned task make progress independently on the runtime's thread pool.

---

## Why not just use threads?

| | OS thread | tokio task |
|---|-----------|------------|
| Stack size | ~2 MB (reserved) | A few hundred bytes (the future struct) |
| Scheduled by | OS kernel | tokio runtime (user space) |
| Switch cost | ~1–10 µs, cache flush | Pulling from a queue — nanoseconds |
| Practical limit | Thousands | Hundreds of thousands |
| Best for | CPU-bound work, blocking I/O | I/O-bound work, many concurrent connections |

A tokio task is cheap enough that you can spawn one per incoming connection — exactly the one-task-per-connection pattern that was impossible with OS threads. The difference is that tasks share a small pool of threads. If there are 100,000 tasks and 8 worker threads, only 8 tasks run at any instant, but all 100,000 make progress as their I/O becomes ready.

---

## Other runtimes

tokio is not the only option:

| Runtime | Character |
|---------|-----------|
| `tokio` | Multi-threaded, production-grade, huge ecosystem |
| `async-std` | Mirrors `std` API naming, single or multi-threaded |
| `smol` | Tiny, embeddable, often used in libraries |

**You can only use one runtime per program.** Mixing runtimes (calling tokio code inside an async-std executor, for instance) causes panics or deadlocks. Most library crates are runtime-agnostic — they use abstract futures — but crates that do I/O (`reqwest`, `axum`, `sqlx`) typically require tokio.

When starting a project, pick tokio and stay with it.

---

## Adding tokio to a project

In `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

`features = ["full"]` enables everything: the multi-thread scheduler, the reactor, and all of `tokio::net`, `tokio::fs`, `tokio::time`, and `tokio::sync`. For production binaries you might trim this to only the features you use. For learning, `"full"` is fine.

---

## Key ideas

| Concept | What it means |
|---------|---------------|
| Runtime | Drives futures to completion; bridges synchronous `main()` and the async world |
| Executor | Maintains a queue of ready tasks; calls `poll()` on them |
| Reactor | Watches OS I/O events; wakes tasks when data arrives |
| `#[tokio::main]` | Macro that starts the tokio runtime and runs `async fn main()` |
| Task | A future handed to the runtime to run independently; created with `tokio::spawn` |
| Task vs thread | Tasks share a small thread pool; you can have hundreds of thousands of tasks |
| One runtime | Mixing runtimes in one program causes panics; pick tokio and stay with it |
| `features = ["full"]` | Enables all of tokio — the right default while learning |
