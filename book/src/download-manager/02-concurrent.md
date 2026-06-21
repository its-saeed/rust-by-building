# Lesson 2 — Concurrent Downloads

> **Goal**: Download many files at the same time. Measure the speedup. Prevent overwhelming the server by limiting how many downloads run at once.

---

## Step 1 — Read URLs from a file

Create a `urls.txt` with one URL per line:

```
https://httpbin.org/bytes/102400
https://httpbin.org/bytes/204800
https://httpbin.org/bytes/51200
https://httpbin.org/bytes/153600
```

Read them all at once:

```rust
let content = tokio::fs::read_to_string("urls.txt").await?;
let urls: Vec<String> = content
    .lines()
    .filter(|l| !l.trim().is_empty())
    .map(String::from)
    .collect();
```

`read_to_string` returns the whole file as a `String`. `.lines()` splits on newlines. The `filter` drops blank lines. The result is a `Vec<String>` of URLs.

---

## Step 2 — Sequential baseline

Before adding concurrency, download them one at a time so you have something to compare against:

```rust
use std::time::Instant;

let t0 = Instant::now();

for url in &urls {
    let filename = filename_from(url);
    download(url, &filename).await?;
    println!("done: {}", filename);
}

println!("sequential total: {:.2?}", t0.elapsed());
```

Where `filename_from` extracts the last path segment of the URL:

```rust
fn filename_from(url: &str) -> String {
    url.rsplit('/').next().unwrap_or("file").to_string()
}
```

Run this first. Note the time. Then switch to concurrent downloads and compare.

---

## Step 3 — Why sequential is slow here

When you `.await` each download in a loop, the second download does not start until the first is fully written to disk:

```
sequential (4 files):
  file1 [────────── 2.1s ──────────]
  file2                              [────── 1.4s ──────]
  file3                                                   [─── 0.9s ───]
  file4                                                                  [── 1.8s ──]
  total: 6.2s
```

The CPU is idle almost the entire time. The program waits on the network. Waiting is exactly what async was designed to do in parallel.

---

## Step 4 — Concurrent with `tokio::spawn`

Spawn one task per download, collect the handles, then await them all:

```rust
let t0 = Instant::now();

let handles: Vec<_> = urls.iter().map(|url| {
    let url = url.clone();
    tokio::spawn(async move {
        let filename = filename_from(&url);
        download(&url, &filename).await
    })
}).collect();

for handle in handles {
    handle.await??;
}

println!("concurrent total: {:.2?}", t0.elapsed());
```

Two things to notice:

- `url.clone()` — the spawned task is `'static`, so it cannot borrow `url`. Clone it first, then move the clone into the `async move` block.
- `handle.await??` — two `?` operators. The first unwraps the `Result<T, JoinError>` from `tokio::spawn` (which fails if the task panicked). The second unwraps the `Result` from your own `download` function.

The result:

```
concurrent (4 files):
  file1 [────────── 2.1s ──────────]
  file2 [────── 1.4s ──────]
  file3 [─── 0.9s ───]
  file4 [── 1.8s ──]
  total: 2.1s  ← the slowest, not the sum
```

All downloads run at the same time. The total time is determined by the slowest file, not by the sum of all files.

---

## Step 5 — The problem with unlimited concurrency

Spawning 4 tasks for 4 URLs is fine. Spawning 500 tasks for 500 URLs can cause problems:

- The server may rate-limit or ban your IP for opening too many connections at once.
- Your network connection has a finite bandwidth that all 500 tasks share.
- Each task holds an open file handle and a socket — system resources are not infinite.

A good default is to limit concurrent downloads to somewhere between 4 and 16.

---

## Step 6 — `Semaphore` for bounded concurrency

A `Semaphore` is a counter with a maximum value. Acquiring a permit decrements the counter; releasing a permit (by dropping it) increments it. If the counter is at zero, `acquire()` waits until a permit becomes available.

```rust
use std::sync::Arc;
use tokio::sync::Semaphore;

let sem = Arc::new(Semaphore::new(4));  // at most 4 concurrent downloads

let handles: Vec<_> = urls.iter().map(|url| {
    let url = url.clone();
    let sem = sem.clone();
    tokio::spawn(async move {
        let _permit = sem.acquire().await.unwrap();
        // ↑ waits here if 4 other downloads are already running

        let filename = filename_from(&url);
        download(&url, &filename).await
        // ↑ _permit drops here, freeing a slot for the next task
    })
}).collect();

for handle in handles {
    handle.await??;
}
```

`Arc` is needed because the semaphore is shared across tasks — multiple tasks hold a clone of the same `Arc<Semaphore>`.

The `_permit` binding is important. A binding named `_` (underscore alone) is dropped immediately. A binding named `_permit` lives until the end of the block. The slot is only freed when the download task finishes.

With this change, 500 tasks can be spawned immediately — but at most 4 will ever be downloading at the same time. The other 496 sit cheaply in the task queue, waiting for a permit.

---

## Putting it together — full program

```rust
use std::sync::Arc;
use std::time::Instant;
use futures_util::StreamExt;
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::Semaphore;

fn filename_from(url: &str) -> String {
    url.rsplit('/').next().unwrap_or("file").to_string()
}

async fn download(url: &str, path: &str) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    let response = Client::new().get(url).send().await?;
    if !response.status().is_success() {
        return Err(format!("HTTP {}", response.status()).into());
    }
    let mut file = File::create(path).await?;
    let mut stream = response.bytes_stream();
    let mut received: u64 = 0;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        received += chunk.len() as u64;
        file.write_all(&chunk).await?;
    }
    Ok(received)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = tokio::fs::read_to_string("urls.txt").await?;
    let urls: Vec<String> = content
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(String::from)
        .collect();

    println!("downloading {} files...", urls.len());

    let sem = Arc::new(Semaphore::new(4));
    let t0  = Instant::now();

    let handles: Vec<_> = urls.iter().map(|url| {
        let url = url.clone();
        let sem = sem.clone();
        tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            let filename = filename_from(&url);
            let t = Instant::now();
            match download(&url, &filename).await {
                Ok(bytes) => println!("ok  {} — {} bytes in {:.2?}", filename, bytes, t.elapsed()),
                Err(e)    => println!("err {} — {}", filename, e),
            }
        })
    }).collect();

    for handle in handles {
        handle.await?;
    }

    println!("total: {:.2?}", t0.elapsed());
    Ok(())
}
```

---

## Exercises

Open `lessons/15-download-manager/lesson-02/` and run:

```sh
rbb watch download-02
```

> **TODO 1**: Make the concurrency limit configurable. Read a `--jobs N` flag from the command line (e.g. `cargo run -- urls.txt --jobs 8`). Default to `4` if the flag is absent.

> **TODO 2**: After all handles are awaited, print a summary: how many succeeded, how many failed, total bytes downloaded.

> **TODO 3**: Retry each failed download up to 3 times before giving up. Add exponential backoff between attempts using `tokio::time::sleep(Duration::from_millis(200 * 2u64.pow(attempt)))`.

---

## Key APIs

| API | What it does |
|-----|-------------|
| `tokio::spawn(async move { ... })` | Spawn a new async task; returns a `JoinHandle` |
| `handle.await` | Wait for the task to finish; returns `Result<T, JoinError>` |
| `Arc::new(Semaphore::new(n))` | Create a semaphore permitting `n` concurrent holders |
| `sem.acquire().await` | Wait for a permit; blocks when the semaphore is at capacity |
| `_permit` binding | Holds the permit until dropped; release happens automatically at end of block |
| `url.clone()` | Required before moving a `String` into an `async move` block |
