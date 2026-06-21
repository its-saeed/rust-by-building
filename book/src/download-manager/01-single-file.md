# Lesson 1 — Downloading a Single File

> **Goal**: Download one file from a URL and save it to disk. Understand how `reqwest` delivers a response body as a stream of chunks.

---

## Setup

Add these dependencies to `Cargo.toml`:

```toml
[dependencies]
tokio  = { version = "1",    features = ["full"] }
reqwest = { version = "0.12", features = ["stream"] }
futures-util = "0.3"
```

`reqwest` handles HTTP. The `stream` feature unlocks the streaming API we will use in Step 3. `futures-util` provides the `StreamExt` trait that adds `.next()` to byte streams.

---

## Step 1 — Make the GET request

```rust
use reqwest::Client;

async fn download(url: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    // ...
    Ok(())
}
```

`client.get(url).send().await?` does two things: it sends the HTTP request and waits for the **response headers** to arrive. The body has not been read yet — only the status code and headers are available at this point.

This matters because:

- The status code tells you whether the server found the file before you spend time reading it.
- The `Content-Length` header (when present) tells you the total file size before a single byte of data arrives.

---

## Step 2 — Check the status code

Always check before reading the body:

```rust
let response = client.get(url).send().await?;
if !response.status().is_success() {
    return Err(format!("HTTP {}", response.status()).into());
}
```

`is_success()` returns `true` for any 2xx status code. A 404 Not Found or 403 Forbidden would reach this check and return an error early — before wasting time waiting for a body that may not exist.

---

## Step 3 — Read the body: simple approach

For small files (a few MB at most), read the whole body into memory:

```rust
let bytes = response.bytes().await?;
tokio::fs::write(path, &bytes).await?;
```

`.bytes().await` waits until the entire body is in memory, then returns a `Bytes` value. `tokio::fs::write` writes it to disk in one call.

This is fine for small files. For a 2 GB file it would use 2 GB of RAM before writing anything.

---

## Step 4 — Read the body: streaming approach

For large files, read and write chunk by chunk as data arrives:

```rust
use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;
use tokio::fs::File;

let mut file = File::create(path).await?;
let mut stream = response.bytes_stream();

while let Some(chunk) = stream.next().await {
    let chunk = chunk?;
    file.write_all(&chunk).await?;
}
```

Each iteration of the loop:

1. `.next().await` — waits for the next chunk of data from the network. This yields to the runtime while waiting; other tasks can run.
2. `chunk?` — unwraps the result; a network error here becomes an early return.
3. `file.write_all(&chunk).await?` — writes that chunk to disk before fetching the next one.

Memory usage stays low: at most one chunk in RAM at a time (typically 8–64 KB per chunk, depending on the network stack).

---

## Step 5 — Read the total size

The server may tell you the file size in advance via the `Content-Length` header:

```rust
let total_bytes: Option<u64> = response.content_length();
```

This is `Option<u64>` because not all servers include `Content-Length`. Servers that use chunked transfer encoding, for example, do not know the total size when they start sending.

When it is `Some(n)`, you can compute a progress percentage: `bytes_received * 100 / total`.
When it is `None`, you know how many bytes you have received but not what fraction of the file that is.

---

## Putting it together — full program

This program takes a URL and output path from the command line, downloads the file in streaming mode, and reports how long it took:

```rust
use std::time::Instant;
use futures_util::StreamExt;
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

async fn download(url: &str, path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(format!("HTTP {}", response.status()).into());
    }

    let total = response.content_length();
    println!(
        "downloading: {}",
        match total {
            Some(n) => format!("{} bytes", n),
            None    => "unknown size".to_string(),
        }
    );

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
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: {} <url> <output-path>", args[0]);
        std::process::exit(1);
    }

    let url  = &args[1];
    let path = &args[2];

    let t0 = Instant::now();
    let bytes = download(url, path).await?;
    let elapsed = t0.elapsed();

    println!(
        "saved {} bytes to {} in {:.2?}",
        bytes, path, elapsed
    );

    Ok(())
}
```

Run it:

```sh
cargo run -- https://httpbin.org/bytes/1024 output.bin
```

Expected output:

```
downloading: 1024 bytes
saved 1024 bytes to output.bin in 312ms
```

---

## Exercises

Open `lessons/15-download-manager/lesson-01/` and run:

```sh
rbb watch download-01
```

> **TODO 1**: Before starting the stream loop, print the `Content-Length` from the response headers. If the server did not send one, print `"size unknown"` instead.

> **TODO 2**: If the output file already exists on disk, skip the download entirely and print `"already exists, skipping"`. Check with `tokio::fs::try_exists(path).await?`.

> **TODO 3**: Add a 5-second timeout to the entire download using `tokio::time::timeout`. If the download takes longer, print `"timed out"` and return an error.

---

## Key APIs

| API | What it does |
|-----|-------------|
| `Client::new()` | Create a reusable HTTP client |
| `client.get(url).send().await` | Send GET request; resolves when response headers arrive |
| `response.status().is_success()` | `true` for any 2xx status code |
| `response.content_length()` | Returns `Option<u64>` — the file size if the server provided it |
| `response.bytes().await` | Read entire body into memory at once |
| `response.bytes_stream()` | Return the body as an async stream of `Bytes` chunks |
| `stream.next().await` | Get the next chunk; `None` when the body is fully received |
| `File::create(path).await` | Create (or truncate) a file for writing |
| `file.write_all(&chunk).await` | Write bytes to the file |
| `tokio::time::timeout(dur, fut)` | Wraps a future; returns `Err` if `dur` elapses first |
