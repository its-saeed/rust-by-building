# Lesson 1 — Threads and Spawn

The conceptual chapters explained what threads are and why they exist. Now let's use them.

---

## Spawning a thread

`std::thread::spawn` takes a closure and runs it on a new OS thread:

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("hello from the spawned thread");
    });

    println!("hello from the main thread");

    handle.join().unwrap();
}
```

`spawn` returns a `JoinHandle`. Calling `.join()` on it blocks the current thread until the spawned thread finishes. Without `join`, the main thread might exit before the spawned thread prints anything — or mid-sentence.

The order of the two `println!` calls is not guaranteed. The OS decides which thread runs first.

---

## Why `move`

Closures passed to `spawn` almost always need the `move` keyword:

```rust
let name = String::from("Alice");

let handle = thread::spawn(move || {
    println!("hello, {name}");  // name was moved into the closure
});

handle.join().unwrap();
```

Without `move`, the closure would capture `name` by reference. But the spawned thread might outlive the current function — at which point the reference would point to freed stack memory. The compiler rejects this:

```
error[E0382]: borrow of moved value: `name`
```

`move` transfers ownership of captured variables into the closure. The spawned thread owns them outright — no dangling references possible.

---

## Returning a value from a thread

The closure can return a value. `.join()` returns it wrapped in a `Result`:

```rust
let handle = thread::spawn(|| {
    42 + 1
});

let result = handle.join().unwrap();
println!("result: {result}");  // 43
```

`.join()` returns `Err` if the thread panicked. `.unwrap()` re-panics in the calling thread, which is usually the right behaviour.

---

## Spawning multiple threads

```rust
use std::thread;

fn main() {
    let handles: Vec<_> = (0..4)
        .map(|i| {
            thread::spawn(move || {
                println!("thread {i} running");
                i * i
            })
        })
        .collect();

    let results: Vec<_> = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect();

    println!("squares: {results:?}");
}
```

All four threads run concurrently (and in parallel if cores are available). The order they print is non-deterministic. The `results` vec is deterministic — we join them in order.

---

## Thread naming

Threads can be given a name, which appears in panic messages and makes debugging easier:

```rust
thread::Builder::new()
    .name("worker".to_string())
    .spawn(|| {
        // ...
    })
    .unwrap();
```

---

## What happens on panic

If a spawned thread panics, the panic is caught at the thread boundary. The other threads keep running. `.join()` on a panicked thread returns `Err`:

```rust
let handle = thread::spawn(|| {
    panic!("oops");
});

match handle.join() {
    Ok(_)  => println!("finished fine"),
    Err(_) => println!("thread panicked"),
}
```

This is different from a panic in the main thread — that terminates the whole process.

---

## Exercise

> **TODO 1**: Spawn 4 threads. Each thread receives a `Vec<i64>` (a quarter of a large input) and returns the sum of its slice. Collect the results with `join` and add them together. Compare the answer with a single-threaded sum of the same data.
>
> **TODO 2**: Add a name to each thread using `Builder::new().name(...)`. Then cause one thread to panic intentionally (`if i == 2 { panic!(...) }`). What does the output look like? Does the main thread crash?
>
> **TODO 3**: Spawn two threads that each try to print 5 lines with a small `thread::sleep`. Run the program several times. Is the interleaving of output the same each time? Why not?

---

## Key APIs

| API | What it does |
|-----|-------------|
| `thread::spawn(closure)` | Run closure on a new OS thread, returns `JoinHandle` |
| `handle.join()` | Block until thread finishes, return its result or panic payload |
| `move \|\| { ... }` | Move captured variables into the closure (required for spawn) |
| `thread::Builder::new().name(...).spawn(...)` | Spawn with a name |
| `thread::sleep(Duration)` | Pause the current thread for a duration |
