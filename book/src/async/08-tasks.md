# Lesson 2 — Tasks

> **Goal**: Spawn independent async tasks the way threading used `thread::spawn`.

The previous lesson ran multiple futures concurrently using `tokio::join!`. `join!` is convenient when you know all the futures up front and want to wait for all of them together. Tasks are different — they are independent units of work that run in the background, like threads, but far cheaper.

---

## Step 1 — `tokio::spawn`

`tokio::spawn` is to async what `thread::spawn` is to threads:

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        sleep(Duration::from_millis(100)).await;
        println!("task finished");
    });

    println!("main continues");

    handle.await.unwrap();
}
```

The spawned task starts immediately and runs independently. The main task keeps going. `handle.await` waits for the spawned task to complete.

Notice the difference from threading:
- `thread::spawn` → `.join()` — synchronous wait
- `tokio::spawn` → `.await` — async wait

The `JoinHandle` returned by `tokio::spawn` is itself a future. Awaiting it waits for the task. This keeps everything in the async world.

---

## Step 2 — Tasks return values

Like threads, tasks can return values:

```rust
let handle = tokio::spawn(async {
    42
});

let value = handle.await.unwrap();
println!("task returned: {value}");  // 42
```

`handle.await` yields `Result<T, JoinError>`. The `Ok` branch holds the return value. The `Err` branch fires if the task panicked. `.unwrap()` re-panics in the caller — which is usually what you want.

Compare to the threading equivalent:
```rust
// threading
let handle = thread::spawn(|| 42);
let value = handle.join().unwrap();

// async
let handle = tokio::spawn(async { 42 });
let value = handle.await.unwrap();
```

The shape is identical. The difference is that `.await` is async — the calling task is suspended, not the thread.

---

## Step 3 — Tasks vs sequential await

There is a subtle but important difference between spawning a task and just awaiting a future:

```rust
// Option A: sequential — runs one after the other
let a = fetch("api",  500).await;
let b = fetch("db",   500).await;
// total: ~1000 ms

// Option B: concurrent with join!
let (a, b) = tokio::join!(fetch("api", 500), fetch("db", 500));
// total: ~500 ms

// Option C: tasks — also concurrent
let ha = tokio::spawn(fetch("api", 500));
let hb = tokio::spawn(fetch("db",  500));
let a = ha.await.unwrap();
let b = hb.await.unwrap();
// total: ~500 ms
```

Options B and C both run concurrently. The difference: `join!` keeps both futures on the same task; `spawn` creates two separate tasks that can be scheduled independently. For most use-cases the timing is the same. Tasks become important when you want the work to keep running even if the caller stops waiting — for example, when handling a client connection in a server.

---

## Step 4 — Tasks are cheap

OS threads cost ~2 MB of stack each. Tokio tasks cost kilobytes. You can spawn tens of thousands without issue:

```rust
use std::time::Instant;

#[tokio::main]
async fn main() {
    let t0 = Instant::now();

    let handles: Vec<_> = (0..10_000)
        .map(|i| {
            tokio::spawn(async move {
                // tiny bit of async work
                tokio::task::yield_now().await;
                i
            })
        })
        .collect();

    let mut total = 0u64;
    for h in handles {
        total += h.await.unwrap() as u64;
    }

    println!("sum: {total}");
    println!("10,000 tasks in {:.2?}", t0.elapsed());
}
```

Try this with `thread::spawn` instead: 10,000 OS threads will either be very slow to start, exhaust stack memory, or hit the OS thread limit. Tokio tasks have none of these problems — the runtime multiplexes them over a small fixed pool of OS threads.

---

## Step 5 — Tasks must be `'static`

Spawned tasks cannot borrow from the scope that spawned them:

```rust
let name = String::from("Alice");

let handle = tokio::spawn(async {
    println!("hello, {name}");  // error!
});
```

```
error[E0373]: async block may outlive the current function,
              but it borrows `name`, which is owned by the current function
```

This is the same constraint as `thread::spawn`. The task may outlive the scope where `name` was defined, so borrowing would be unsound. The fix is to move owned data into the task:

```rust
let name = String::from("Alice");

let handle = tokio::spawn(async move {
    println!("hello, {name}");  // name is owned by the task now
});
```

If you need the same data in multiple tasks, clone it or wrap it in `Arc`:

```rust
use std::sync::Arc;

let config = Arc::new(String::from("prod"));

for i in 0..3 {
    let config = Arc::clone(&config);
    tokio::spawn(async move {
        println!("task {i} using config: {config}");
    });
}
```

---

## Step 6 — Handling task panics

If a task panics, its `JoinHandle` returns `Err`. Other tasks are unaffected:

```rust
let bad = tokio::spawn(async {
    panic!("something went wrong");
});

let good = tokio::spawn(async {
    42
});

match bad.await {
    Ok(_)  => println!("fine"),
    Err(e) => println!("task panicked: {e}"),
}

println!("good task: {}", good.await.unwrap());
// prints: good task: 42
```

The panic is caught at the task boundary. This is similar to `thread::spawn` — a panicked thread does not kill the process; a panicked task does not kill the runtime.

---

## Full code

Spawn `N` tasks that each simulate some work, then collect all results:

```rust
use std::time::{Duration, Instant};
use tokio::time::sleep;

async fn do_work(id: u32) -> u64 {
    // simulate work that takes a variable amount of time
    let delay = Duration::from_millis(100 + (id % 5) as u64 * 50);
    sleep(delay).await;
    (id as u64) * (id as u64)  // return id squared
}

#[tokio::main]
async fn main() {
    const N: u32 = 8;

    let t0 = Instant::now();

    // spawn all tasks immediately
    let handles: Vec<_> = (0..N)
        .map(|i| tokio::spawn(do_work(i)))
        .collect();

    // collect all results
    let mut results = Vec::new();
    for (i, handle) in handles.into_iter().enumerate() {
        match handle.await {
            Ok(value) => {
                println!("task {i} → {value}");
                results.push(value);
            }
            Err(e) => println!("task {i} panicked: {e}"),
        }
    }

    let sum: u64 = results.iter().sum();
    println!("\nsum of squares 0..{N}: {sum}");
    println!("done in {:.2?}", t0.elapsed());
}
```

All 8 tasks start at once. The slowest one determines the total time. Compare with running them sequentially (remove `tokio::spawn`, just `.await` each call in a loop) to see the difference.

---

## Exercise

> **TODO 1**: Spawn 8 tasks with different simulated delays. Instead of collecting results in spawn order, collect them in the order they finish. Use a `tokio::sync::mpsc` channel: each task sends `(id, result)` when it completes, and the main task receives them in arrival order.
>
> **TODO 2**: Spawn a task that panics after 100 ms (`tokio::time::sleep(Duration::from_millis(100)).await; panic!("oops")`). Make the main task handle the error gracefully and continue. Verify that a second task running concurrently is unaffected by the panic.
>
> **TODO 3**: Spawn 100,000 tasks, each sleeping for 1 second, then awaiting all of them. Measure the total wall-clock time. Now try the same thing with `thread::spawn` (you will need to reduce the count significantly). Compare the behaviour.

---

## Key APIs

| API | What it does |
|-----|-------------|
| `tokio::spawn(async { ... })` | Spawn an independent task; returns `JoinHandle<T>` |
| `handle.await` | Wait for the task to finish; yields `Result<T, JoinError>` |
| `tokio::task::yield_now()` | Suspend the current task, letting other tasks run |
| `async move { ... }` | Move owned data into an async block (required to satisfy `'static`) |
| `Arc::clone(&arc)` | Cheap reference-counted clone for sharing across tasks |
| `JoinError` | Returned when a task panics; inspect with `.is_panic()` |
