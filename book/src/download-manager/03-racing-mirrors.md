# Lesson 3 — Racing Mirrors

> **Goal**: Given several mirror URLs for the same file, download from all of them simultaneously and keep whichever finishes first. This is the canonical use case for `tokio::select!`.

---

## The problem

Large files are often hosted on multiple **mirrors** — servers in different geographic regions, each holding an identical copy. You do not know in advance which mirror is fastest right now. The EU mirror might be under heavy load today. The US mirror might be physically closer to you.

The naive approach is to try mirrors one at a time and move to the next if the first is slow. But this means you waste time waiting for a slow mirror to time out.

The async approach: start all mirrors simultaneously and take whatever arrives first. The others are cancelled the moment a winner is found.

---

## Step 1 — `select!` with two mirrors

You already know `tokio::select!` from Lesson 5 of the async section. Applied to downloads:

```rust
async fn download_fastest_two(
    mirror_a: &str,
    mirror_b: &str,
    path: &str,
) -> Result<&'static str, Box<dyn std::error::Error>> {
    tokio::select! {
        r = download(mirror_a, path) => { r?; Ok("mirror-a") }
        r = download(mirror_b, path) => { r?; Ok("mirror-b") }
    }
}
```

Both `download` futures are started. `select!` polls them concurrently. The first to complete wins — its branch runs, and the other future is dropped. Dropping a future cancels it: any awaited operations inside it are abandoned immediately.

There is a subtlety here: if both mirrors write to the same `path`, you could have two tasks writing to the same file at the same time. In the real program, each mirror task should write to a temporary path and rename on success.

---

## Step 2 — `select!` with a timeout

Race the download against a deadline:

```rust
use std::time::Duration;
use tokio::time::sleep;

tokio::select! {
    r = download(mirror_a, path) => r?,
    _ = sleep(Duration::from_secs(30)) => {
        return Err("download timed out after 30s".into());
    }
}
```

If the download completes within 30 seconds, the first branch wins. If 30 seconds elapse, the `sleep` future completes, the download future is dropped and cancelled, and we return an error.

This pattern is so common that Tokio also provides `tokio::time::timeout`:

```rust
tokio::time::timeout(
    Duration::from_secs(30),
    download(mirror_a, path),
).await??
```

Both do the same thing. Use `select!` when you need the timeout to be one branch among several; use `tokio::time::timeout` when there is a single future and you just want a deadline.

---

## Step 3 — Dynamic number of mirrors

`select!` is a macro that requires you to list its arms at compile time. You cannot write a `select!` over a `Vec` of futures at runtime.

For a dynamic list of mirrors, use a different pattern: spawn one task per mirror, all racing to send on a channel with capacity 1. The first sender wins.

```rust
use tokio::sync::mpsc;

async fn download_fastest(
    mirrors: Vec<String>,
    path: String,
) -> Result<String, Box<dyn std::error::Error>> {
    // capacity 1: only the first message fits; later senders see the channel full
    let (tx, mut rx) = mpsc::channel::<String>(1);

    for mirror in mirrors {
        let tx   = tx.clone();
        let path = path.clone();
        tokio::spawn(async move {
            if download(&mirror, &path).await.is_ok() {
                // ok() silently ignores send errors — that just means someone
                // else already won and the receiver is gone
                tx.send(mirror).await.ok();
            }
        });
    }

    // Drop our own sender clone — when all tasks finish, the channel closes.
    // Without this, rx.recv() would wait forever even after all tasks exit.
    drop(tx);

    rx.recv()
        .await
        .ok_or_else(|| "all mirrors failed".into())
}
```

Why capacity 1? The winning task fills the slot immediately. By the time a second task tries to send, `rx.recv()` has already been called and the function returns — the receiver is dropped, so subsequent sends return `Err(SendError)`. The `.ok()` on `tx.send(...).await.ok()` silently discards that error. The losing tasks are still running, but their results are irrelevant.

---

## Step 4 — Aborting losing tasks

In Step 3 the losing tasks keep downloading even after a winner is found. They are doing unnecessary work. To stop them:

```rust
use tokio::task::JoinHandle;

async fn download_fastest(
    mirrors: Vec<String>,
    path: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let (tx, mut rx) = mpsc::channel::<String>(1);
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for mirror in mirrors {
        let tx   = tx.clone();
        let path = path.clone();
        let handle = tokio::spawn(async move {
            if download(&mirror, &path).await.is_ok() {
                tx.send(mirror).await.ok();
            }
        });
        handles.push(handle);
    }
    drop(tx);

    let winner = rx.recv().await.ok_or("all mirrors failed")?;

    // cancel everything that is still running
    for handle in handles {
        handle.abort();
    }

    Ok(winner)
}
```

`handle.abort()` signals the task to stop at its next `.await` point. The task does not stop instantly, but it will stop soon — and it will not do any more useful work after `abort()` is called.

---

## Putting it together — full program

```rust
use std::time::Instant;
use futures_util::StreamExt;
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

async fn download(
    url: &str,
    path: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let response = Client::new().get(url).send().await?;
    if !response.status().is_success() {
        return Err(format!("HTTP {}", response.status()).into());
    }
    let mut file = File::create(path).await?;
    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        file.write_all(&chunk?).await?;
    }
    Ok(())
}

async fn download_fastest(
    mirrors: Vec<String>,
    path: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let (tx, mut rx) = mpsc::channel::<String>(1);
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for mirror in mirrors {
        let tx   = tx.clone();
        let path = path.clone();
        let handle = tokio::spawn(async move {
            if download(&mirror, &path).await.is_ok() {
                tx.send(mirror).await.ok();
            }
        });
        handles.push(handle);
    }
    drop(tx);

    let winner = rx
        .recv()
        .await
        .ok_or("all mirrors failed — none responded successfully")?;

    for handle in handles {
        handle.abort();
    }

    Ok(winner)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Three mirrors serving the same file
    let mirrors = vec![
        "https://httpbin.org/bytes/204800".to_string(),
        "https://httpbin.org/bytes/204800".to_string(),
        "https://httpbin.org/bytes/204800".to_string(),
    ];

    let path = "output.bin".to_string();
    let t0   = Instant::now();

    match download_fastest(mirrors, path).await {
        Ok(winner)  => println!("winner: {} in {:.2?}", winner, t0.elapsed()),
        Err(e)      => println!("error: {}", e),
    }

    Ok(())
}
```

---

## Exercises

Open `lessons/15-download-manager/lesson-03/` and run:

```sh
rbb watch download-03
```

> **TODO 1**: Add a 10-second timeout to `download_fastest` using `tokio::select!`. If no mirror responds within 10 seconds, abort all tasks and return an error.

> **TODO 2**: Implement a staggered start: try the first mirror alone for 2 seconds. If it has not finished, start the second mirror. If neither is done after another 2 seconds, start the third. Use `tokio::time::sleep` and `select!`.

> **TODO 3**: Run `download_fastest` 10 times with the same mirrors and count which mirror URL wins most often. Print a leaderboard at the end.

---

## Key APIs

| API | What it does |
|-----|-------------|
| `tokio::select! { r = fut_a => ..., r = fut_b => ... }` | Run both; execute the first branch that resolves; drop the other |
| `tokio::time::sleep(dur)` | Future that completes after `dur` — useful as a `select!` timeout branch |
| `tokio::time::timeout(dur, fut)` | Wraps a future with a deadline; returns `Err(Elapsed)` if `dur` passes first |
| `tokio::spawn(...)` | Spawn a task; returns a `JoinHandle` |
| `handle.abort()` | Signal a task to stop at its next `.await` |
| `mpsc::channel(1)` | Bounded channel with capacity 1 — first sender wins, rest are ignored |
| `tx.send(...).await.ok()` | Send and discard any error (channel full or receiver gone) |
