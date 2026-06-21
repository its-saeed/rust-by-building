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

## Step 5 — Two sleeps, done concurrently

`tokio::join!` runs multiple futures at the same time and waits for all of them:

```rust
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let t0 = Instant::now();

    let _ = tokio::join!(
        sleep(Duration::from_secs(1)),
        sleep(Duration::from_secs(1)),
    );

    println!("done in {:.2?}", t0.elapsed());  // ~1.00s
}
```

Both sleeps start at the same instant. Both yield control to the runtime. The runtime sets two timers. When either fires, it wakes the relevant future. When both have fired, `join!` returns. Total time: ~1 second, not 2.

```
sequential:
  time ──────────────────────────────────────────▶
  sleep A  [────────── 1s ──────────]
  sleep B                            [────────── 1s ──────────]
  total: ~2 seconds

concurrent (tokio::join!):
  time ──────────────────────────────────────────▶
  sleep A  [────────── 1s ──────────]
  sleep B  [────────── 1s ──────────]
  total: ~1 second
```

This is the point of async. You can do many things at once on a single thread, because waiting is cheap — the thread is free while a future is suspended.

---

## Step 6 — Simulating real work

Real programs wait for networks, disks, and databases. We simulate this with sleep. Here is a small program that "fetches" two things at once:

```rust
async fn fetch(name: &str, delay: Duration) -> String {
    sleep(delay).await;
    format!("{name}: done")
}
```

Call it sequentially, then concurrently, and compare the times.

Sequential — about 1.5 seconds:
```rust
let a = fetch("api",      Duration::from_millis(1000)).await;
let b = fetch("database", Duration::from_millis(500)).await;
println!("{a}");
println!("{b}");
```

Concurrent with `join!` — about 1 second:
```rust
let (a, b) = tokio::join!(
    fetch("api",      Duration::from_millis(1000)),
    fetch("database", Duration::from_millis(500)),
);
println!("{a}");
println!("{b}");
```

The concurrent version is faster because the database fetch completes during the time we're waiting for the API — no wasted waiting.

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
    let (a, b) = tokio::join!(
        fetch("api",      Duration::from_millis(1000)),
        fetch("database", Duration::from_millis(500)),
    );
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
> **TODO 2**: Add five `fetch` calls to the `tokio::join!` — one for each of: `"api"`, `"database"`, `"cache"`, `"auth"`, `"logger"` — with delays of 300, 700, 100, 500, and 200 ms. What is the total concurrent time? What would the sequential time be?
>
> **TODO 3**: Try 1000 concurrent sleeps. Replace `tokio::join!` with a `Vec` of futures and use `futures::future::join_all`. Does it stay fast? (Add `futures = "0.3"` to Cargo.toml, then `futures::future::join_all(vec![...]).await`.)

---

## Key APIs

| API | What it does |
|-----|-------------|
| `#[tokio::main]` | Starts the Tokio runtime and runs `async fn main()` inside it |
| `async fn f() { }` | Declares a function that returns a future when called |
| `f().await` | Polls the future to completion; suspends the task if not yet ready |
| `tokio::time::sleep(d)` | Returns a future that resolves after duration `d` |
| `tokio::join!(a, b, ...)` | Runs multiple futures concurrently; returns when all complete |
| `Instant::now()` / `.elapsed()` | Measure wall-clock time |
