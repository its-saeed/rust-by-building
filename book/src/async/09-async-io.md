# Lesson 3 — Async I/O

> **Goal**: Accept TCP connections and read/write data asynchronously.

The previous lessons used `tokio::time::sleep` to simulate async work. Now we do real async I/O — accepting TCP connections and exchanging bytes with clients. This is where async shines: one thread handling thousands of simultaneous connections.

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

---

## Step 1 — Async TCP listener

The standard library's blocking version:

```rust
use std::net::TcpListener;

let listener = TcpListener::bind("127.0.0.1:8080")?;
let (stream, addr) = listener.accept()?;  // blocks until a client connects
```

Tokio's async version looks nearly identical:

```rust
use tokio::net::TcpListener;

let listener = TcpListener::bind("127.0.0.1:8080").await?;
let (stream, addr) = listener.accept().await?;  // suspends until a client connects
```

The difference is `.await` on `accept()`. When no client is waiting, the async version suspends the current task — the thread stays free to handle other work. The blocking version would stall the entire thread.

---

## Step 2 — Reading and writing

`tokio::net::TcpStream` is the async counterpart of `std::net::TcpStream`. To read and write, you need two trait extensions in scope:

```rust
use tokio::io::{AsyncReadExt, AsyncWriteExt};
```

Without these `use` statements, the `.read()` and `.write_all()` methods will not be available, even though the stream compiles. This is a common stumbling block — if you get "method not found", check your imports first.

Basic read into a buffer:

```rust
let mut buf = vec![0u8; 1024];
let n = stream.read(&mut buf).await?;
println!("read {} bytes: {:?}", n, &buf[..n]);
```

Write bytes back:

```rust
stream.write_all(b"hello\n").await?;
```

---

## Step 3 — Line-by-line reading with `BufReader`

Reading raw bytes is fine for binary protocols. For line-based text protocols (like our echo server), wrap the stream in a buffered reader:

```rust
use tokio::io::{AsyncBufReadExt, BufReader};

let reader = BufReader::new(stream);
let mut lines = reader.lines();

while let Some(line) = lines.next_line().await? {
    println!("got: {line}");
}
```

`BufReader` accumulates bytes internally and surfaces complete lines. `lines.next_line().await?` suspends until a full line (ending in `\n`) is available, then returns it as a `String` without the newline. When the client disconnects, it returns `None`.

The trait that provides `lines()` is `AsyncBufReadExt`. The import is:

```rust
use tokio::io::AsyncBufReadExt;
```

---

## Step 4 — Echo server for one client

Build it up piece by piece. First: accept a single connection, echo each line, close:

```rust
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("listening on :8080");

    // accept exactly one connection
    let (stream, addr) = listener.accept().await.unwrap();
    println!("client connected: {addr}");

    let (reader, mut writer) = stream.into_split();
    let mut lines = BufReader::new(reader).lines();

    while let Some(line) = lines.next_line().await.unwrap() {
        let response = format!("{line}\n");
        writer.write_all(response.as_bytes()).await.unwrap();
    }

    println!("client disconnected");
}
```

`stream.into_split()` splits the stream into an independent read half and write half. This is necessary here because `BufReader` takes ownership of the read half, and we still need to write back on the same connection.

Test it:
```sh
cargo run &
echo "hello world" | nc localhost 8080
# output: hello world
```

---

## Step 5 — Multiple clients with tasks

The single-client version blocks on one connection at a time. A real server needs to handle many clients simultaneously. The pattern: accept in a loop, spawn a task per connection.

```rust
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

async fn handle_client(stream: tokio::net::TcpStream) {
    let (reader, mut writer) = stream.into_split();
    let mut lines = BufReader::new(reader).lines();

    while let Some(line) = lines.next_line().await.unwrap_or(None) {
        let response = format!("{line}\n");
        if writer.write_all(response.as_bytes()).await.is_err() {
            break;
        }
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("listening on :8080");

    loop {
        let (stream, addr) = listener.accept().await.unwrap();
        println!("client connected: {addr}");
        tokio::spawn(handle_client(stream));
    }
}
```

Each `tokio::spawn(handle_client(stream))` creates a new task. The main loop immediately returns to `accept().await` — it never blocks on a client. All tasks run concurrently on the Tokio runtime's thread pool.

Compare with the threading version from the chat server chapter:

```rust
// threading version
loop {
    let (stream, addr) = listener.accept()?;
    thread::spawn(move || {
        handle_client(stream);
    });
}

// async version
loop {
    let (stream, addr) = listener.accept().await?;
    tokio::spawn(handle_client(stream));
}
```

The structure is identical. The difference is cost: a thread costs ~2 MB of stack; a Tokio task costs a few kilobytes. At 10,000 concurrent connections, that is 20 GB of stack vs roughly 100 MB of task memory.

---

## Step 6 — `tokio::io::copy`

For a pure echo server — copying all bytes from reader to writer without inspecting them — `tokio::io::copy` is the simplest approach:

```rust
use tokio::io;

async fn handle_client(mut stream: tokio::net::TcpStream) {
    let (mut reader, mut writer) = stream.split();
    io::copy(&mut reader, &mut writer).await.ok();
}
```

`io::copy` pumps bytes from reader to writer until the reader closes (the client disconnects). It handles buffering internally and never allocates a line buffer — ideal for binary protocols or raw proxying.

`stream.split()` gives you borrowed halves (unlike `into_split()` which gives owned halves). Borrowed halves are simpler when both halves are used in the same function.

---

## Full code

```rust
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

async fn handle_client(stream: TcpStream, id: usize) {
    println!("[{id}] connected");

    let (reader, mut writer) = stream.into_split();
    let mut lines = BufReader::new(reader).lines();

    while let Ok(Some(line)) = lines.next_line().await {
        let response = format!("{line}\n");
        if writer.write_all(response.as_bytes()).await.is_err() {
            break;
        }
    }

    println!("[{id}] disconnected");
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("echo server listening on :8080");
    println!("test with: nc localhost 8080");

    let mut next_id = 0usize;

    loop {
        let (stream, _addr) = listener.accept().await.unwrap();
        let id = next_id;
        next_id += 1;
        tokio::spawn(handle_client(stream, id));
    }
}
```

Run it and connect with multiple `nc` clients in separate terminals:

```sh
# terminal 1
cargo run

# terminal 2
nc localhost 8080
hello
# ← echoes: hello

# terminal 3 (at the same time as terminal 2)
nc localhost 8080
world
# ← echoes: world
```

Both clients are served concurrently by a single OS thread.

---

## Exercise

> **TODO 1**: Count how many clients are currently connected. Add an `Arc<AtomicUsize>` counter that increments when a client connects and decrements when it disconnects. Print the count on each connection and disconnection event.
>
> **TODO 2**: Prefix each echoed line with a timestamp. Use `std::time::SystemTime::now()` to get the current time, and format it as `[HH:MM:SS] {line}\n`. (Hint: `SystemTime` gives you seconds since epoch; you can format it manually or use the `chrono` crate.)
>
> **TODO 3**: Limit the server to a maximum of 10 concurrent connections. Use an `Arc<Semaphore>` from `tokio::sync::Semaphore` — acquire a permit before spawning, release it when the client disconnects. When the limit is reached, new connections should be accepted but immediately closed with a `"server full\n"` message.

---

## Key APIs

| API | What it does |
|-----|-------------|
| `TcpListener::bind(addr).await` | Bind a socket to an address; async version of `std::net::TcpListener::bind` |
| `listener.accept().await` | Wait for the next incoming connection; returns `(TcpStream, SocketAddr)` |
| `AsyncReadExt::read(&mut buf).await` | Read bytes into a buffer; returns number of bytes read |
| `AsyncWriteExt::write_all(&bytes).await` | Write all bytes to the stream |
| `BufReader::new(reader).lines()` | Wrap a reader for line-by-line access |
| `lines.next_line().await` | Read one line; returns `Ok(None)` on disconnect |
| `stream.into_split()` | Split stream into owned read and write halves |
| `stream.split()` | Split stream into borrowed read and write halves |
| `tokio::io::copy(&mut r, &mut w).await` | Copy all bytes from reader to writer |
