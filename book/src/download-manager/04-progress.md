# Lesson 4 — Progress Bars

> **Goal**: Report download progress from many concurrent tasks back to a single display task using async channels. Draw a live ASCII progress bar for each download.

The new idea here is not the drawing — it is the **pattern**: tasks send events; one dedicated task receives them all and owns the display. This keeps terminal output orderly even when dozens of tasks are running simultaneously.

---

## Step 1 — Define the progress event type

Each download task needs to tell the display task three things:

- "I started" — so the display can reserve a row for this download
- "I received some bytes" — so the display can update the progress bar
- "I finished" — or "I failed"

Model this as an enum:

```rust
#[derive(Debug)]
enum Progress {
    Started { url: String, total: Option<u64> },
    Chunk   { url: String, bytes: u64 },
    Done    { url: String },
    Failed  { url: String, error: String },
}
```

Each variant carries enough information for the display task to update its state independently. The display task never needs to reach into the download tasks — it only sees these messages.

---

## Step 2 — The download function sends events

Modify `download` to accept a `Sender` and emit events as it runs:

```rust
use tokio::sync::mpsc;
use futures_util::StreamExt;
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

async fn download_with_progress(
    url: String,
    path: String,
    tx: mpsc::Sender<Progress>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let response = Client::new().get(&url).send().await?;

    if !response.status().is_success() {
        let msg = format!("HTTP {}", response.status());
        tx.send(Progress::Failed { url: url.clone(), error: msg.clone() }).await.ok();
        return Err(msg.into());
    }

    let total = response.content_length();
    tx.send(Progress::Started { url: url.clone(), total }).await.ok();

    let mut file   = File::create(&path).await?;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        let bytes = chunk.len() as u64;
        file.write_all(&chunk).await?;
        tx.send(Progress::Chunk { url: url.clone(), bytes }).await.ok();
    }

    tx.send(Progress::Done { url }).await.ok();
    Ok(())
}
```

Notice `.await.ok()` on every send. If the display task has exited (or the channel buffer is full), the send returns `Err`. Using `.ok()` discards that error — the download keeps going. Stopping a download because the display broke would be the wrong trade-off.

---

## Step 3 — The display task owns all state

The display task holds a `HashMap` mapping each URL to how many bytes have been received and the total size. It updates this map as events arrive.

```rust
use std::collections::HashMap;

struct DownloadState {
    received: u64,
    total:    Option<u64>,
    done:     bool,
}
```

The receive loop:

```rust
let mut state: HashMap<String, DownloadState> = HashMap::new();

while let Some(event) = rx.recv().await {
    match event {
        Progress::Started { url, total } => {
            state.insert(url, DownloadState { received: 0, total, done: false });
        }
        Progress::Chunk { url, bytes } => {
            if let Some(s) = state.get_mut(&url) {
                s.received += bytes;
            }
        }
        Progress::Done { url } => {
            if let Some(s) = state.get_mut(&url) {
                s.done = true;
            }
            println!("{}: done", url);
        }
        Progress::Failed { url, error } => {
            println!("{}: error — {}", url, error);
        }
    }

    redraw(&state);
}
```

`rx.recv().await` returns `None` when all senders have been dropped — meaning all download tasks have finished. The `while let` loop exits and the program ends.

---

## Step 4 — Drawing the progress bar

A simple ASCII progress bar that works in any terminal:

```rust
fn progress_bar(received: u64, total: Option<u64>, width: usize) -> String {
    match total {
        None => {
            // Unknown total: show bytes received with spinning indicator
            format!("[{:─<width$}] {} bytes", "", received, width = width)
        }
        Some(total) if total == 0 => {
            format!("[{:░<width$}]", "", width = width)
        }
        Some(total) => {
            let pct    = (received as f64 / total as f64).min(1.0);
            let filled = (pct * width as f64) as usize;
            let empty  = width - filled;
            let pct_int = (pct * 100.0) as u32;
            format!(
                "[{}{}] {:3}%  {}/{}",
                "█".repeat(filled),
                "░".repeat(empty),
                pct_int,
                received,
                total,
            )
        }
    }
}
```

The `redraw` function prints all active bars:

```rust
fn redraw(state: &HashMap<String, DownloadState>) {
    for (url, s) in state {
        let bar   = progress_bar(s.received, s.total, 20);
        let label = url.rsplit('/').next().unwrap_or(url.as_str());
        let status = if s.done { "done" } else { "..." };
        println!("{} {} {}", bar, label, status);
    }
}
```

For a real terminal UI you would use ANSI escape codes to move the cursor back up and overwrite previous lines. For simplicity, this version prints new lines every update. The exercises ask you to improve it.

---

## Step 5 — Wiring it all together

Create the channel, spawn one download task per URL, drop the extra sender, then run the display loop:

```rust
let (tx, mut rx) = mpsc::channel::<Progress>(128);

let urls = vec![
    ("https://httpbin.org/bytes/204800", "file1.bin"),
    ("https://httpbin.org/bytes/102400", "file2.bin"),
    ("https://httpbin.org/bytes/409600", "file3.bin"),
];

// spawn one task per URL
for (url, path) in urls {
    let tx = tx.clone();
    tokio::spawn(download_with_progress(
        url.to_string(),
        path.to_string(),
        tx,
    ));
}

// IMPORTANT: drop our own clone of the sender.
// If we do not do this, rx.recv() will never return None
// because our clone keeps the channel open even after
// all download tasks have exited.
drop(tx);
```

Why does the channel stay open if you do not drop `tx`? Because the channel closes only when **all** senders are dropped. You cloned `tx` once per download task (above), but the original `tx` from `mpsc::channel` is still alive in this scope. Drop it explicitly so the channel closes when the last task finishes.

---

## Putting it together — full program

```rust
use std::collections::HashMap;
use std::time::Instant;
use futures_util::StreamExt;
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;

#[derive(Debug)]
enum Progress {
    Started { url: String, total: Option<u64> },
    Chunk   { url: String, bytes: u64 },
    Done    { url: String },
    Failed  { url: String, error: String },
}

struct DownloadState {
    received: u64,
    total:    Option<u64>,
    done:     bool,
}

fn progress_bar(received: u64, total: Option<u64>, width: usize) -> String {
    match total {
        None => format!("[{:─<width$}] {} bytes", "", received, width = width),
        Some(0) => format!("[{:░<width$}]", "", width = width),
        Some(total) => {
            let pct    = (received as f64 / total as f64).min(1.0);
            let filled = (pct * width as f64) as usize;
            let empty  = width - filled;
            format!(
                "[{}{}] {:3}%",
                "█".repeat(filled),
                "░".repeat(empty),
                (pct * 100.0) as u32,
            )
        }
    }
}

fn redraw(state: &HashMap<String, DownloadState>) {
    for (url, s) in state {
        let bar    = progress_bar(s.received, s.total, 20);
        let label  = url.rsplit('/').next().unwrap_or(url.as_str());
        let status = if s.done { "done" } else { "..." };
        println!("{} {:12} {}", bar, label, status);
    }
    println!();
}

async fn download_with_progress(
    url: String,
    path: String,
    tx: mpsc::Sender<Progress>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let response = Client::new().get(&url).send().await?;
    if !response.status().is_success() {
        let msg = format!("HTTP {}", response.status());
        tx.send(Progress::Failed { url, error: msg.clone() }).await.ok();
        return Err(msg.into());
    }
    let total = response.content_length();
    tx.send(Progress::Started { url: url.clone(), total }).await.ok();

    let mut file   = File::create(&path).await?;
    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        let bytes = chunk.len() as u64;
        file.write_all(&chunk).await?;
        tx.send(Progress::Chunk { url: url.clone(), bytes }).await.ok();
    }
    tx.send(Progress::Done { url }).await.ok();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let downloads: &[(&str, &str)] = &[
        ("https://httpbin.org/bytes/204800", "file1.bin"),
        ("https://httpbin.org/bytes/102400", "file2.bin"),
        ("https://httpbin.org/bytes/409600", "file3.bin"),
    ];

    let (tx, mut rx) = mpsc::channel::<Progress>(128);
    let t0 = Instant::now();

    for (url, path) in downloads {
        let tx = tx.clone();
        tokio::spawn(download_with_progress(
            url.to_string(),
            path.to_string(),
            tx,
        ));
    }
    drop(tx);

    let mut state: HashMap<String, DownloadState> = HashMap::new();

    while let Some(event) = rx.recv().await {
        match event {
            Progress::Started { url, total } => {
                state.insert(url, DownloadState { received: 0, total, done: false });
            }
            Progress::Chunk { url, bytes } => {
                if let Some(s) = state.get_mut(&url) {
                    s.received += bytes;
                }
            }
            Progress::Done { url } => {
                if let Some(s) = state.get_mut(&url) {
                    s.done = true;
                }
            }
            Progress::Failed { url, error } => {
                eprintln!("error {}: {}", url, error);
            }
        }
        redraw(&state);
    }

    let total_bytes: u64 = state.values().map(|s| s.received).sum();
    println!(
        "\nall done — {} bytes in {:.2?}",
        total_bytes,
        t0.elapsed()
    );

    Ok(())
}
```

---

## Exercises

Open `lessons/15-download-manager/lesson-04/` and run:

```sh
rbb watch download-04
```

> **TODO 1**: Track the start time for each download in `DownloadState` (add a `started_at: Instant` field, set it in the `Started` branch). Display the elapsed time next to each progress bar.

> **TODO 2**: Compute and display the download speed in KB/s. Divide `received` bytes by the seconds elapsed since `Started`. Update it on every `Chunk` event.

> **TODO 3**: Use ANSI escape codes to update the progress bars in-place instead of printing new lines. The sequence `\x1b[{}A` moves the cursor up `{}` lines. Move up by the number of active downloads, then overwrite each line.

---

## Key APIs

| API | What it does |
|-----|-------------|
| `mpsc::channel::<T>(n)` | Create a multi-producer, single-consumer channel with buffer capacity `n` |
| `tx.clone()` | Clone the sender so multiple tasks can send to the same channel |
| `tx.send(value).await` | Send a value; `.await` waits if the buffer is full |
| `rx.recv().await` | Receive the next value; returns `None` when all senders are dropped |
| `drop(tx)` | Explicitly drop a sender clone so the channel can close |
| `HashMap::get_mut(&key)` | Get a mutable reference to a value by key |
| `state.values().map(...).sum()` | Aggregate a field across all entries |
