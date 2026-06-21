# Lesson 1 — First Async Program

> **Goal**: Run your first async function and understand what the runtime does.

The theory chapters explained what futures are, what `.await` means, and what the runtime does. Now you write code.

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

---

## Step 1 — Hello, async world

A synchronous `main` function looks like this:

```rust
fn main() {
    println!("hello");
}
```

An async `main` needs one extra annotation:

```rust
#[tokio::main]
async fn main() {
    println!("hello from async");
}
```

`#[tokio::main]` is a procedural macro. It rewrites your `async fn main()` into a synchronous `main` that starts the Tokio runtime and runs your async code inside it. The runtime is the event loop — the machinery that polls futures and drives them to completion.

Without the macro, you cannot call `.await` inside `main`. The compiler will not let you define `async fn main()` without something to run it.

---

## Step 2 — The simplest async operation

The async equivalent of `thread::sleep` is `tokio::time::sleep`:

```rust
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("before");
    sleep(Duration::from_secs(1)).await;
    println!("after");
}
```

Both versions pause for one second. The key difference:

```
std::thread::sleep(Duration::from_secs(1))
  → blocks the OS thread entirely
  → no other work can happen on this thread while sleeping

tokio::time::sleep(Duration::from_secs(1)).await
  → suspends the current async task
  → the thread is free to run other tasks while waiting
```

This is the heart of async: when a future is not ready (the timer hasn't fired, the socket has no data), the runtime puts that task aside and runs something else on the thread. When the timer fires, the runtime wakes the task back up and continues from exactly where it left off.

---

## Step 3 — Forgetting `.await`

What happens if you forget `.await`?

```rust
#[tokio::main]
async fn main() {
    println!("before");
    sleep(Duration::from_secs(1));  // missing .await
    println!("after");
}
```

The program prints both lines instantly — no delay at all. The compiler also warns you:

```
warning: unused `Sleep` that must be used
  --> src/main.rs:5:5
   |
 5 |     sleep(Duration::from_secs(1));
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: futures do nothing unless you `.await` or poll them
```

`sleep(...)` returns a `Sleep` future. Calling it creates the future, but does nothing. `.await` is what hands the future to the runtime and waits for it to finish. Without `.await`, the future is created and immediately dropped — no timer is set, no waiting happens.

This is a common mistake. If something should be async but runs instantly, check for a missing `.await`.

---

## Step 4 — Two sleeps, done sequentially

If you await two sleeps one after the other, they run in sequence — 2 seconds total:

```rust
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    let t0 = Instant::now();

    sleep(Duration::from_secs(1)).await;  // wait 1 second
    sleep(Duration::from_secs(1)).await;  // then wait 1 more second

    println!("done in {:.2?}", t0.elapsed());  // ~2.00s
}
```

This is correct and it does what it looks like: the second sleep only starts after the first one finishes. Sequential `.await` chains are sequential execution.

---

## Step 5 — Two tasks at once with `tokio::spawn`

You already know `thread::spawn` from the threading chapter. `tokio::spawn` is the async equivalent: it hands a future to the runtime and runs it independently, returning a `JoinHandle` you can `.await` later.

```rust
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let t0 = Instant::now();

    let handle_a = tokio::spawn(sleep(Duration::from_secs(1)));
    let handle_b = tokio::spawn(sleep(Duration::from_secs(1)));

    handle_a.await.unwrap();
    handle_b.await.unwrap();

    println!("done in {:.2?}", t0.elapsed());  // ~1.00s
}
```

Compare this directly with the threading version you already wrote:

```rust
// threads
let h1 = thread::spawn(|| thread::sleep(Duration::from_secs(1)));
let h2 = thread::spawn(|| thread::sleep(Duration::from_secs(1)));
h1.join().unwrap();
h2.join().unwrap();

// async tasks
let h1 = tokio::spawn(sleep(Duration::from_secs(1)));
let h2 = tokio::spawn(sleep(Duration::from_secs(1)));
h1.await.unwrap();
h2.await.unwrap();
```

The shape is identical. The difference: `thread::spawn` creates an OS thread (stack, scheduler slot, ~2 MB); `tokio::spawn` creates a task (a struct on the heap, no OS thread).

```
sequential:
  time ──────────────────────────────────────────▶
  sleep A  [────────── 1s ──────────]
  sleep B                            [────────── 1s ──────────]
  total: ~2 seconds

concurrent (tokio::spawn):
  time ──────────────────────────────────────────▶
  sleep A  [────────── 1s ──────────]
  sleep B  [────────── 1s ──────────]
  total: ~1 second
```

Both tasks start immediately. Both suspend, leaving the thread free. When the first timer fires the runtime resumes that task; same for the second. Total time is the duration of the longest sleep, not their sum.

---

## Step 6 — Simulating real work

Real programs wait for networks, disks, and databases. We can simulate this with sleep. Here is a small async function that pretends to fetch something:

```rust
async fn fetch(name: &str, delay: Duration) -> String {
    sleep(delay).await;
    format!("{name}: done")
}
```

Sequential — each fetch waits for the previous to finish:

```rust
let a = fetch("api",      Duration::from_millis(1000)).await;
let b = fetch("database", Duration::from_millis(500)).await;
// total: ~1500ms
```

Concurrent with `tokio::spawn` — both start at the same time:

```rust
let ha = tokio::spawn(fetch("api",      Duration::from_millis(1000)));
let hb = tokio::spawn(fetch("database", Duration::from_millis(500)));
let a = ha.await.unwrap();
let b = hb.await.unwrap();
// total: ~1000ms
```

The database fetch completes while we are waiting for the API. No wasted time.

---

## Full code

```rust
use std::time::{Duration, Instant};
use tokio::time::sleep;

async fn fetch(name: &str, delay: Duration) -> String {
    sleep(delay).await;
    format!("{name}: done")
}

#[tokio::main]
async fn main() {
    // --- sequential ---
    let t0 = Instant::now();
    let a = fetch("api",      Duration::from_millis(1000)).await;
    let b = fetch("database", Duration::from_millis(500)).await;
    println!("[sequential]");
    println!("  {a}");
    println!("  {b}");
    println!("  elapsed: {:.2?}", t0.elapsed());

    println!();

    // --- concurrent ---
    let t1 = Instant::now();
    let ha = tokio::spawn(fetch("api",      Duration::from_millis(1000)));
    let hb = tokio::spawn(fetch("database", Duration::from_millis(500)));
    let a = ha.await.unwrap();
    let b = hb.await.unwrap();
    println!("[concurrent]");
    println!("  {a}");
    println!("  {b}");
    println!("  elapsed: {:.2?}", t1.elapsed());
}
```

Expected output:
```
[sequential]
  api: done
  database: done
  elapsed: 1.50s

[concurrent]
  api: done
  database: done
  elapsed: 1.00s
```

---

## Exercise

> **TODO 1**: Change the two delays so one is 200 ms and the other is 800 ms. Predict the sequential and concurrent times before running. Verify your prediction.
>
> **TODO 2**: Spawn five tasks — `"api"`, `"database"`, `"cache"`, `"auth"`, `"logger"` — with delays of 300, 700, 100, 500, and 200 ms. Collect all five `JoinHandle`s in a `Vec`, then `.await` each one. What is the total time?
>
> **TODO 3**: Spawn 1000 tasks, each sleeping for 1 second. How long does it take? Compare to spawning 1000 OS threads doing `thread::sleep` — try both and time them.

---

## Key APIs

| API | What it does |
|-----|-------------|
| `#[tokio::main]` | Starts the Tokio runtime and runs `async fn main()` inside it |
| `async fn f() { }` | Declares a function that returns a future when called |
| `f().await` | Polls the future to completion; suspends the task if not yet ready |
| `tokio::time::sleep(d)` | Returns a future that resolves after duration `d` |
| `tokio::spawn(future)` | Hands a future to the runtime as an independent task; returns a `JoinHandle` |
| `handle.await.unwrap()` | Waits for a spawned task to finish and unwraps its result |
| `Instant::now()` / `.elapsed()` | Measure wall-clock time |
