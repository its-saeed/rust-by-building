# Lesson 1 — The Async Server

> **Goal**: Accept multiple TCP clients with tokio, spawn a task per client, and broadcast every message to all connected clients.

The threaded server in Project 11 worked like this: spawn a thread per client, send messages through an `mpsc` channel, have the main thread relay them to a `Vec` of writers. It required a custom `Event` enum, a `retain_mut` cleanup pass, and the broadcaster lived in a dedicated thread.

The async version is shorter. `tokio::sync::broadcast` replaces the custom channel design, `tokio::select!` handles the two-direction problem inside each task, and `tokio::spawn` replaces `thread::spawn`.

---

## Step 1 — Binding the listener

The async `TcpListener` looks almost identical to the std one:

```rust
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on :8080");
    Ok(())
}
```

The only differences from the Project 11 version:

- `use tokio::net::TcpListener` instead of `use std::net::TcpListener`
- `.await?` after `bind` — it is an async operation
- `main` is `async fn` and returns a `Result`

The `#[tokio::main]` attribute starts the Tokio runtime. Without it, `.await` has nowhere to run.

---

## Step 2 — The broadcast channel

Project 11 used `mpsc` (multiple producers, single consumer) plus a custom `Event` enum. The broadcaster received everything and forwarded it to a `Vec` of writers.

This project uses `tokio::sync::broadcast` instead. A broadcast channel is different: **every receiver gets every message**. That is exactly what a chat server needs.

```rust
use tokio::sync::broadcast;

let (tx, _rx) = broadcast::channel::<String>(32);
// 32 = buffer size: how many messages can queue before older ones are dropped
```

One `Sender`, any number of `Receiver`s. Each client task gets its own receiver by calling `tx.subscribe()`. When `tx.send(msg)` is called, every subscriber's receiver gets a copy.

Compare with Project 11:

| Project 11 | Project 14 |
|-----------|-----------|
| `mpsc::channel::<Event>()` — one receiver | `broadcast::channel::<String>(32)` — many receivers |
| Broadcaster holds a `Vec<TcpStream>`, loops `retain_mut` | Each task has its own `rx`; `rx.recv()` blocks until a message arrives |
| `NewClient(TcpStream)` event to register writers | `tx.subscribe()` in the accept loop — no registration needed |

The broadcast design eliminates the broadcaster thread entirely. There is no central relay; the channel machinery handles fan-out.

---

## Step 3 — The accept loop

```rust
loop {
    let (socket, addr) = listener.accept().await?;
    let tx = tx.clone();
    let rx = tx.subscribe();

    tokio::spawn(async move {
        handle_client(socket, addr, tx, rx).await;
    });
}
```

`listener.accept().await` suspends until a client connects — without blocking the OS thread. When it returns, the new socket and the client's address are available.

`tx.clone()` gives the new task a sender it can use to broadcast messages. `tx.subscribe()` creates a new receiver scoped to this client — it will get every message sent after this moment.

`tokio::spawn` is to async what `thread::spawn` is to threads. It schedules the task on Tokio's thread pool and returns immediately, letting the accept loop go straight back to waiting for the next connection.

---

## Step 4 — `handle_client` with `select!`

Each client task needs to do two things at once:

1. Read lines arriving from this client → broadcast them
2. Receive broadcast messages → write them to this client

In Project 11 the threaded server handled this by having the reader thread send messages to the broadcaster, which then wrote to all writers. Two separate concerns in two separate threads.

With async, both happen inside one task using `tokio::select!`.

First, split the socket into independent read and write halves:

```rust
use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

let (reader, mut writer) = socket.into_split();
let mut lines = BufReader::new(reader).lines();
```

`into_split()` is the async equivalent of `try_clone()`. Instead of asking the OS for a second file descriptor, it splits the tokio `TcpStream` into an `OwnedReadHalf` and an `OwnedWriteHalf` that can be used independently — without any OS overhead.

Then loop with `select!`:

```rust
async fn handle_client(
    socket: TcpStream,
    addr: SocketAddr,
    tx: broadcast::Sender<String>,
    mut rx: broadcast::Receiver<String>,
) {
    let (reader, mut writer) = socket.into_split();
    let mut lines = BufReader::new(reader).lines();

    loop {
        tokio::select! {
            // incoming line from this client → broadcast to all
            result = lines.next_line() => {
                match result {
                    Ok(Some(line)) => { tx.send(format!("{addr}: {line}")).ok(); }
                    _ => break,  // client disconnected or read error
                }
            }
            // message from another client → write to this client
            result = rx.recv() => {
                match result {
                    Ok(msg) => { writer.write_all(format!("{msg}\n").as_bytes()).await.ok(); }
                    Err(_) => break,
                }
            }
        }
    }

    println!("{addr} disconnected");
}
```

`tokio::select!` runs both futures simultaneously and handles whichever one completes first. The loop then continues, waiting again on both.

```
task A is running handle_client:

  waiting for... ─╮ lines.next_line()  ← user has not typed yet
                  ╰ rx.recv()          ← another client sends "hello"

  rx.recv() wins → write "hello\n" to this client
  loop → wait again on both
```

Neither branch blocks the other. One task, two simultaneous directions.

---

## The full server

```rust
use std::net::SocketAddr;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;

async fn handle_client(
    socket: TcpStream,
    addr: SocketAddr,
    tx: broadcast::Sender<String>,
    mut rx: broadcast::Receiver<String>,
) {
    let (reader, mut writer) = socket.into_split();
    let mut lines = BufReader::new(reader).lines();

    println!("{addr} connected");

    loop {
        tokio::select! {
            result = lines.next_line() => {
                match result {
                    Ok(Some(line)) => { tx.send(format!("{addr}: {line}")).ok(); }
                    _ => break,
                }
            }
            result = rx.recv() => {
                match result {
                    Ok(msg) => { writer.write_all(format!("{msg}\n").as_bytes()).await.ok(); }
                    Err(_) => break,
                }
            }
        }
    }

    println!("{addr} disconnected");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on :8080");

    let (tx, _rx) = broadcast::channel::<String>(32);

    loop {
        let (socket, addr) = listener.accept().await?;
        let tx = tx.clone();
        let rx = tx.subscribe();

        tokio::spawn(async move {
            handle_client(socket, addr, tx, rx).await;
        });
    }
}
```

Add to `Cargo.toml`:

```toml
[dependencies]
tokio  = { version = "1", features = ["full"] }
anyhow = "1"
```

---

## Test it with `nc`

```sh
# terminal 1
cargo run --bin server

# terminal 2
nc 127.0.0.1 8080
hello
# (your message appears in terminal 3)

# terminal 3
nc 127.0.0.1 8080
# (sees: "127.0.0.1:XXXXX: hello")
world
# (terminal 2 sees: "127.0.0.1:YYYYY: world")
```

Disconnect terminal 2 with `Ctrl+C`. Terminal 3 keeps working. The server log shows both connections and disconnections.

---

## Exercises

> **TODO 1**: Track the number of currently connected clients. Add `Arc<AtomicUsize>` to `main`, clone it into each task, increment on entry and decrement (`fetch_sub`) before `return`. Print the count in the connect and disconnect log lines. What import do you need?
>
> **TODO 2**: Prefix every broadcast message with a timestamp. Use `chrono::Local::now().format("[%H:%M:%S]")` (add `chrono = "0.4"` to `Cargo.toml`). Where is the best place to add the prefix — in `handle_client` or at the `tx.send` call site?
>
> **TODO 3**: Broadcast `"{addr} has joined"` immediately after a client connects and `"{addr} has left"` just before `handle_client` returns. Where should these `tx.send` calls go? Does the join message need to go through the broadcast channel, or can it be written directly to `writer`?

---

## Key APIs

| API | What it does |
|-----|-------------|
| `TcpListener::bind(addr).await` | Reserve a port and start listening; async |
| `listener.accept().await` | Suspend until a client connects; returns `(TcpStream, SocketAddr)` |
| `broadcast::channel(capacity)` | Create a broadcast channel; every subscriber gets every message |
| `tx.subscribe()` | Create a new `Receiver` for this client |
| `tx.send(msg)` | Send `msg` to all current subscribers |
| `rx.recv().await` | Suspend until the next broadcast message arrives |
| `socket.into_split()` | Split a `TcpStream` into owned read and write halves |
| `BufReader::new(reader).lines()` | Buffer reads; gives an async line iterator |
| `lines.next_line().await` | Suspend until a `\n`-terminated line arrives; `None` on disconnect |
| `tokio::select!` | Run multiple async branches concurrently; handle whichever completes first |
