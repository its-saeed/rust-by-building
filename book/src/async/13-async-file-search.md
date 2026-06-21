# Mini Project 2 — Async File Search

You built a parallel file search with threads in the threading chapter. Now rebuild it with async. Same task: search for a word across all `.rs` files in a directory, one task per file. The comparison is the lesson.

```sh
cargo run -- "fn main" ./src
```

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

---

## What we built before

The threaded version (from "Mini Project 2 — Parallel File Search") spawned one OS thread per file. Each thread read its file with `std::fs::read_to_string` (blocking), scanned for matches, and sent results through `std::sync::mpsc`.

```rust
// threaded — key lines
thread::spawn(move || {
    let matches = search(&path, &query);       // blocking fs read
    if !matches.is_empty() {
        tx.send((path, matches)).unwrap();
    }
});
```

The async version replaces every blocking piece with an async equivalent:

| Threaded | Async |
|---|---|
| `thread::spawn(move \|\| { ... })` | `tokio::spawn(async move { ... })` |
| `std::fs::read_to_string(path)` | `tokio::fs::read_to_string(path).await` |
| `std::fs::read_dir(dir)` | `tokio::fs::read_dir(dir).await` |
| `std::sync::mpsc::channel()` | `tokio::sync::mpsc::channel(n)` |
| `tx.send(v).unwrap()` | `tx.send(v).await.unwrap()` |

The search logic itself — lines, enumerate, filter, collect — does not change at all. Only the I/O calls change.

---

## Step 1 — Command-line arguments

Identical to the threading version:

```rust
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("usage: {} <query> <directory>", args[0]);
        std::process::exit(1);
    }

    let query = args[1].clone();
    let dir   = args[2].clone();
}
```

---

## Step 2 — Collect files with async `read_dir`

`tokio::fs::read_dir` returns a handle you step through with `.next_entry().await` in a loop:

```rust
use std::path::PathBuf;
use tokio::fs;

let mut entries = fs::read_dir(&dir).await.expect("could not read directory");
let mut files: Vec<PathBuf> = Vec::new();

while let Some(entry) = entries.next_entry().await.expect("entry error") {
    let path = entry.path();
    if path.extension().map(|e| e == "rs").unwrap_or(false) {
        files.push(path);
    }
}

println!("searching {} file(s) for {:?}", files.len(), query);
```

This is the main structural difference from the threading version, which used `fs::read_dir` as a blocking iterator. Tokio's `read_dir` yields to the runtime between entries.

---

## Step 3 — The search function

The search logic is identical to the threading version. Only the file read is async:

```rust
use std::path::Path;

async fn search(path: &Path, query: &str) -> Vec<(usize, String)> {
    let content = match tokio::fs::read_to_string(path).await {
        Ok(c)  => c,
        Err(_) => return vec![],  // skip unreadable files
    };

    content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(query))
        .map(|(i, line)| (i + 1, line.to_string()))
        .collect()
}
```

The `.lines().enumerate().filter().map().collect()` chain is pure Rust iterator code — synchronous, no `.await` needed. Only reading the file from disk requires an `.await`.

---

## Step 4 — Spawn one task per file

```rust
use tokio::sync::mpsc;

let (tx, mut rx) = mpsc::channel::<(PathBuf, Vec<(usize, String)>)>(32);

for path in files {
    let tx    = tx.clone();
    let query = query.clone();

    tokio::spawn(async move {
        let matches = search(&path, &query).await;
        if !matches.is_empty() {
            tx.send((path, matches)).await.unwrap();
        }
    });
}

drop(tx);  // close the original sender so rx knows when all tasks are done
```

Compare with the threading version:

```rust
// threaded
thread::spawn(move || {
    let matches = search(&path, &query);       // sync
    if !matches.is_empty() {
        tx.send((path, matches)).unwrap();     // sync send
    }
});

// async
tokio::spawn(async move {
    let matches = search(&path, &query).await; // async
    if !matches.is_empty() {
        tx.send((path, matches)).await.unwrap(); // async send
    }
});
```

Four changes: `thread::spawn` → `tokio::spawn`, `move ||` → `async move`, `search(...)` → `search(...).await`, and `tx.send(...).unwrap()` → `tx.send(...).await.unwrap()`.

---

## Step 5 — Print results as they arrive

```rust
let mut total = 0;

while let Some((path, matches)) = rx.recv().await {
    println!("\n{}", path.display());
    for (n, line) in &matches {
        println!("  {n:>4}: {line}");
    }
    total += matches.len();
}

println!("\n{total} match(es) found");
```

Compare with the threading version, which used `for (path, matches) in rx`. The async version uses `while let Some(...) = rx.recv().await` — the same pattern you learned in the channels lesson.

---

## Full program

```rust
use std::env;
use std::path::{Path, PathBuf};
use std::time::Instant;
use tokio::fs;
use tokio::sync::mpsc;

async fn search(path: &Path, query: &str) -> Vec<(usize, String)> {
    let content = match fs::read_to_string(path).await {
        Ok(c)  => c,
        Err(_) => return vec![],
    };
    content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(query))
        .map(|(i, line)| (i + 1, line.to_string()))
        .collect()
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: {} <query> <directory>", args[0]);
        std::process::exit(1);
    }

    let query = args[1].clone();
    let dir   = args[2].clone();

    let mut entries = fs::read_dir(&dir).await.expect("could not read directory");
    let mut files: Vec<PathBuf> = Vec::new();

    while let Some(entry) = entries.next_entry().await.expect("entry error") {
        let path = entry.path();
        if path.extension().map(|e| e == "rs").unwrap_or(false) {
            files.push(path);
        }
    }

    println!("searching {} file(s) for {:?}", files.len(), query);

    let t0 = Instant::now();
    let (tx, mut rx) = mpsc::channel::<(PathBuf, Vec<(usize, String)>)>(32);

    for path in files {
        let tx    = tx.clone();
        let query = query.clone();
        tokio::spawn(async move {
            let matches = search(&path, &query).await;
            if !matches.is_empty() {
                tx.send((path, matches)).await.unwrap();
            }
        });
    }
    drop(tx);

    let mut total = 0;
    while let Some((path, matches)) = rx.recv().await {
        println!("\n{}", path.display());
        for (n, line) in &matches {
            println!("  {n:>4}: {line}");
        }
        total += matches.len();
    }

    println!("\n{total} match(es) in {:.2?}", t0.elapsed());
}
```

---

## Async vs threads for file I/O — the honest comparison

For file search on a local SSD, async and threads have similar performance. Both approaches overlap I/O operations and use multiple CPU cores for the scanning step.

```
10 files, SSD:
  threaded:  ~8ms
  async:     ~8ms
  difference: negligible

10,000 files, SSD:
  threaded:  spawns 10,000 OS threads → ~20 GB of stack memory pressure
             scheduler managing 10,000 entities → overhead grows
  async:     10,000 tasks on ~4 threads → constant memory usage
             scheduler overhead stays flat
```

Async shines most when there are **many** tasks and each task is mostly waiting. For 10 files, the difference is invisible. For 10,000 files, the threaded version starts showing memory and scheduling pain that the async version avoids entirely.

The other advantage is the **programming model**: the async version composes better. You can add a timeout to each task, cancel groups of tasks, report progress — all using the tools from the previous lessons, without threads and mutexes.

---

## Discussion questions

**What happens if you do not drop `tx` before the receive loop?**

The `while let Some(...) = rx.recv().await` loop never ends. The receiver is waiting for more messages, but the last sender (the original `tx` that was not dropped) is still alive. Dropping `tx` is what signals "no more senders — close the channel." Without it, the program hangs after all the tasks finish.

**How would you add a per-file timeout?**

Wrap the `search` call in `tokio::time::timeout`:

```rust
use tokio::time::{timeout, Duration};

let result = timeout(
    Duration::from_millis(200),
    search(&path, &query),
).await;

let matches = result.unwrap_or_else(|_| vec![]);  // empty on timeout
```

**Could you run thousands of files without spawning thousands of tasks?**

Yes — use a bounded worker pool. Create a small number of tasks (say, one per CPU core) and feed them file paths through a channel. Each worker reads files and sends results until the path channel is empty. This pattern limits peak memory and context-switching overhead regardless of how many files there are.

---

## Key APIs

| API | What it does |
|-----|-------------|
| `tokio::fs::read_to_string(path).await` | Read a file to `String` without blocking the thread |
| `tokio::fs::read_dir(dir).await` | Open a directory for listing; returns an async iterator handle |
| `entries.next_entry().await` | Yield the next directory entry without blocking |
| `tokio::spawn(async move { ... })` | Spawn a task that runs concurrently with the caller |
| `tx.send(v).await` | Send through a `tokio::sync::mpsc` channel; yields if buffer full |
| `rx.recv().await` | Receive the next message; `None` when all senders are dropped |
| `drop(tx)` | Drop the original sender to signal the channel is closed |
| `tokio::time::timeout(d, fut).await` | Cancel `fut` if it does not resolve within `d` |
