# Lesson 5 — join! and select!

> **Goal**: Run multiple futures concurrently and react to whichever finishes first.

You know how to `.await` a future. You know how to spawn a task. But what if you have two futures and you want to run them at the same time — not one after the other? That is what `join!` and `select!` are for.

---

## The problem with sequential `.await`

Two awaits in sequence are sequential — the second does not start until the first finishes:

```rust
let a = fetch_a().await;   // starts; waits for it to finish
let b = fetch_b().await;   // only then starts; waits for it to finish
```

If `fetch_a` takes 1 second and `fetch_b` takes 1 second, this takes 2 seconds. You are leaving a full second on the table.

```
sequential .await:
  time ──────────────────────────────────────────────────────▶
  fetch_a  [────────── 1s ──────────]
  fetch_b                            [────────── 1s ──────────]
  total: 2 seconds
```

The runtime is capable of making progress on `fetch_b` while `fetch_a` is waiting — the thread is free. But sequential `.await` never gives it the chance.

---

## `tokio::join!` — wait for all

`tokio::join!(a, b, c)` runs all three futures *concurrently* and waits until every one of them has finished. It returns a tuple of results in the same order:

```rust
let (r1, r2, r3) = tokio::join!(fetch_a(), fetch_b(), fetch_c());
```

```
tokio::join!:
  time ──────────────────────────────────────────────────────▶
  fetch_a  [────────── 1s ──────────]
  fetch_b  [────────── 1s ──────────]
  fetch_c  [────────── 1s ──────────]
  total: 1 second (the slowest one, not the sum)
```

`join!` is a macro, not a function. It expands to polling all its futures inside a single task — no extra OS threads, no `tokio::spawn`. All three futures share the same task and the same thread (unless they yield to a task that happens to run on another thread, but that is a runtime detail).

---

## Step 1 — Simulating network calls

We will simulate slow operations with `tokio::time::sleep` to keep the examples self-contained. The pattern is identical to real `reqwest` calls:

```rust
use std::time::Duration;
use tokio::time::sleep;

async fn fetch(name: &str, ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await;
    format!("{name} ({ms}ms)")
}
```

---

## Step 2 — join! all at once

```rust
use std::time::Instant;

#[tokio::main]
async fn main() {
    let t0 = Instant::now();

    let (a, b, c) = tokio::join!(
        fetch("alpha",   800),
        fetch("beta",    400),
        fetch("gamma",  1200),
    );

    println!("all done in {:.0?}", t0.elapsed());
    println!("  {a}");
    println!("  {b}");
    println!("  {c}");
}
```

Output:
```
all done in 1.20s
  alpha (800ms)
  beta (400ms)
  gamma (1200ms)
```

Total time equals the slowest future, not the sum. The results come back in definition order regardless of which finished first.

---

## `join!` vs `spawn` — when to use which

Both run things concurrently. The difference is about structure and ownership:

| | `tokio::join!` | `tokio::spawn` |
|---|---|---|
| When it makes sense | Futures are part of the same logical operation | Tasks are independent, long-running, or need to outlive the current scope |
| Returns | Tuple of results | `JoinHandle` you must `.await` separately |
| Overhead | Minimal — no allocation, no task queue | Small allocation per task |
| Cancellation | All futures drop if the join! expression is dropped | Spawned tasks run until completion unless explicitly aborted |

Use `join!` when you want to say "do these three things and give me all three results." Use `spawn` when the task stands on its own — like a server loop that runs forever.

---

## `tokio::select!` — wait for the first

`select!` runs multiple futures concurrently but returns as soon as *one* of them finishes. The others are cancelled (dropped).

```rust
tokio::select! {
    result = fetch("fast", 300) => println!("fast won: {result}"),
    result = fetch("slow", 900) => println!("slow won: {result}"),
}
```

Output:
```
fast won: fast (300ms)
```

The slow future is dropped the moment the fast one resolves. Only one branch executes.

```
tokio::select!:
  time ──────────────────────────────────────────────────────▶
  fast  [── 300ms ──]
  slow  [── 300ms ──X  ← dropped here, never reaches 900ms
  total: 300ms, only the fast result is available
```

---

## Use case 1 — Race mirrors

You have two download mirrors. Take whichever responds first:

```rust
tokio::select! {
    data = download("https://mirror-eu.example.com/file") => data,
    data = download("https://mirror-us.example.com/file") => data,
}
```

This is genuinely useful for reliability. If one mirror is slow or down, you still get your data quickly.

---

## Use case 2 — Timeout (very common)

Race your operation against a timer. If the timer wins, the operation is cancelled:

```rust
use std::time::Duration;
use tokio::time::sleep;

async fn do_work() -> Result<String, String> {
    sleep(Duration::from_secs(3)).await;
    Ok("done".to_string())
}

#[tokio::main]
async fn main() {
    let result = tokio::select! {
        r = do_work() => r,
        _ = sleep(Duration::from_secs(1)) => Err("timed out".to_string()),
    };

    match result {
        Ok(v)  => println!("success: {v}"),
        Err(e) => println!("error: {e}"),  // prints "timed out"
    }
}
```

The `_` pattern in `_ = sleep(...) =>` means we do not care about the sleep's return value (it is `()`). If the sleep wins the race, we return an error. If `do_work` wins, we return its result.

This is the async equivalent of "give this 5 seconds or give up."

---

## Use case 3 — Cancellation signal

Race a task against a channel that signals "stop":

```rust
use tokio::sync::oneshot;

async fn background_work() {
    loop {
        // ... do some periodic work ...
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

let (cancel_tx, cancel_rx) = oneshot::channel::<()>();

tokio::select! {
    _ = background_work() => {},
    _ = cancel_rx => println!("cancelled"),
}

// somewhere else:
cancel_tx.send(()).unwrap();  // signals the select! to stop
```

---

## Step 3 — select! in a loop

The most powerful pattern: loop and handle whichever of several channels has data next. This is how you write a task that listens to multiple inputs simultaneously.

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = mpsc::channel::<i32>(4);
    let (tx2, mut rx2) = mpsc::channel::<i32>(4);

    // two producers
    tokio::spawn(async move {
        for i in 0..5 {
            tx1.send(i).await.unwrap();
            tokio::time::sleep(Duration::from_millis(70)).await;
        }
    });

    tokio::spawn(async move {
        for i in 100..105 {
            tx2.send(i).await.unwrap();
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    });

    // one task consuming from both
    let mut total = 0;
    loop {
        tokio::select! {
            Some(n) = rx1.recv() => {
                println!("from channel 1: {n}");
                total += n;
            }
            Some(n) = rx2.recv() => {
                println!("from channel 2: {n}");
                total += n;
            }
            else => break,  // both channels closed
        }
    }
    println!("total: {total}");
}
```

`select!` polls both channels concurrently. Whichever has a message first, that branch runs. The `else` branch fires when all other branches are disabled — here, when both `recv()` calls return `None` (channels closed).

This pattern — `loop { select! { ... } }` — is the backbone of the async chat client you will build later: one branch reads from stdin, another reads from the server socket.

---

## Full code — racing and joining fetches

```rust
use std::time::{Duration, Instant};
use tokio::time::sleep;

async fn fetch(name: &str, ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await;
    format!("{name} ({ms}ms)")
}

#[tokio::main]
async fn main() {
    // --- join!: get all three results ---
    println!("=== join! ===");
    let t0 = Instant::now();

    let (a, b, c) = tokio::join!(
        fetch("alpha",   800),
        fetch("beta",    400),
        fetch("gamma",  1200),
    );

    println!("all done in {:.0?}", t0.elapsed());
    println!("  {a}");
    println!("  {b}");
    println!("  {c}");

    println!();

    // --- select!: get only the fastest ---
    println!("=== select! ===");
    let t1 = Instant::now();

    let winner = tokio::select! {
        r = fetch("alpha",   800) => r,
        r = fetch("beta",    400) => r,
        r = fetch("gamma",  1200) => r,
    };

    println!("winner in {:.0?}: {winner}", t1.elapsed());

    println!();

    // --- select! with timeout ---
    println!("=== timeout ===");
    let t2 = Instant::now();

    let result: Result<String, &str> = tokio::select! {
        r = fetch("slow", 2000) => Ok(r),
        _ = sleep(Duration::from_millis(500)) => Err("timed out"),
    };

    println!("finished in {:.0?}: {:?}", t2.elapsed(), result);
}
```

Expected output:
```
=== join! ===
all done in 1200ms
  alpha (800ms)
  beta (400ms)
  gamma (1200ms)

=== select! ===
winner in 400ms: beta (400ms)

=== timeout ===
finished in 500ms: Err("timed out")
```

---

## Exercise

> **TODO 1**: Add a timeout to the `join!` version. If any individual fetch takes more than 1 second, cancel all three and print `"group timed out"`. Hint: wrap the entire `join!` expression in a `select!` against a `sleep`.
>
> **TODO 2**: Use `select!` in a loop to merge two streams of numbers. Spawn one task sending even numbers (0, 2, 4 ... 20) with 60ms gaps, and one sending odd numbers (1, 3, 5 ... 21) with 80ms gaps. Collect all 22 numbers in the order they arrive and print the final list.
>
> **TODO 3**: Implement a "first N results" combinator: spawn 10 tasks each sleeping a random duration (hint: use `(i * 137) % 1000` ms to get deterministic-ish variation). Collect results into an `mpsc` channel. Use `select!` to stop after receiving the first 3 results — abort or ignore the rest.

---

## Key APIs

| API | What it does |
|-----|-------------|
| `tokio::join!(a, b, c)` | Run all concurrently; returns a tuple when all finish |
| `tokio::select! { r = fut => expr, ... }` | Run all concurrently; executes the first branch that resolves |
| `_ = sleep(d) => { ... }` | Timeout branch — fires after duration `d` |
| `else => { ... }` | Fires in `select!` when all other branches are disabled (e.g. channels closed) |
| `tokio::time::sleep(d)` | Future that resolves after `d` — use as a timeout in `select!` |
| `tokio::time::timeout(d, fut)` | Wraps a future with a deadline; returns `Err` on timeout |
