# Lesson 2 — The Async Client

> **Goal**: Connect to the chat server, send messages typed by the user, and display messages from other clients — simultaneously, in one async task.

The server is done. Now build the client.

---

## The two-direction problem, revisited

In Project 11 the client needed two threads: one blocked on stdin waiting for the user to type, one blocked on the socket waiting for the server to send something. Neither could yield to the other. Two threads was the only solution.

```
Project 11 — two threads:

  thread 1:  [waiting for stdin────────────] [send to server]
  thread 2:  [waiting for server────] [print] [waiting─────] [print]
```

With async, blocking is replaced by suspension. While the client is waiting for stdin, the runtime can poll the socket future. While waiting for the socket, it can poll the stdin future. `tokio::select!` puts them both in one loop.

```
Project 14 — one task, select!:

  task:  [select! ─── stdin ready? → send]
         [select! ─── server ready? → print]
         [select! ─── stdin ready? → send]
         ...
```

Same result. No second thread.

---

## Step 1 — Connect to the server

```rust
use tokio::net::TcpStream;

let stream = TcpStream::connect("127.0.0.1:8080").await?;
println!("connected — type to chat, Ctrl+D to quit");
```

`TcpStream::connect` performs the TCP three-way handshake asynchronously. It suspends until the connection is established (or returns an error). Compare with Project 11:

```rust
// Project 11
let stream = std::net::TcpStream::connect("127.0.0.1:8080")?;  // blocks the thread

// Project 14
let stream = tokio::net::TcpStream::connect("127.0.0.1:8080").await?;  // suspends the task
```

The difference matters at scale. A blocked thread cannot do anything else. A suspended task releases the OS thread to run other tasks.

---

## Step 2 — Split the stream

```rust
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

let (reader, mut writer) = stream.into_split();
let mut server_lines = BufReader::new(reader).lines();
```

`into_split()` gives independent owned halves. `writer` goes to the branch that sends user input; `server_lines` goes to the branch that receives server messages. Both live in the same task.

In Project 11, `try_clone()` asked the OS for a second file descriptor to pass to a second thread. `into_split()` needs no system call — it splits at the Tokio level.

---

## Step 3 — Async stdin

`tokio::io::stdin()` is the async equivalent of `std::io::stdin()`. Wrapping it in `BufReader` and calling `.lines()` gives the same interface as the server's socket reader — but for the keyboard.

```rust
use tokio::io::{self, AsyncBufReadExt, BufReader};

let mut stdin_lines = BufReader::new(io::stdin()).lines();
```

`stdin_lines.next_line().await` suspends until the user presses Enter. Crucially, it suspends the *task*, not the OS thread — so the socket can still be polled while waiting.

This is why there was no async stdin equivalent in Project 11: the std `BufReader` blocks. Here, the tokio version yields.

---

## Step 4 — The `select!` loop

```rust
loop {
    tokio::select! {
        // user typed a line → send to server
        line = stdin_lines.next_line() => {
            match line? {
                Some(text) => writer.write_all(format!("{text}\n").as_bytes()).await?,
                None => break,  // EOF (Ctrl-D on Linux/macOS, Ctrl-Z on Windows)
            }
        }
        // server sent a message → print it
        msg = server_lines.next_line() => {
            match msg? {
                Some(text) => println!("{text}"),
                None => { println!("[server disconnected]"); break; }
            }
        }
    }
}
```

On each iteration `select!` polls both futures. Whichever one has data ready first is handled; the other is dropped and will be recreated fresh on the next loop iteration. Neither direction can starve the other.

```
loop iteration 1:
  → poll stdin_lines.next_line()  — no data yet
  → poll server_lines.next_line() — "alice: hello!" arrives
  → handle server branch: print "alice: hello!"

loop iteration 2:
  → poll stdin_lines.next_line()  — user typed "hi" and pressed Enter
  → poll server_lines.next_line() — no data yet
  → handle stdin branch: send "hi\n" to server
```

---

## Comparing the two clients

This is the heart of the rewrite. Both programs do the same thing — but the mechanism is different:

| | Project 11 | Project 14 |
|--|-----------|-----------|
| Read stdin | `io::stdin().lock().lines()` — blocks thread | `BufReader::new(io::stdin()).lines()` — yields task |
| Read socket | `BufReader::new(stream).lines()` — blocks thread | `BufReader::new(reader).lines()` — yields task |
| Run both at once | Two OS threads | One task, `select!` |
| Split socket | `stream.try_clone()` — OS system call | `stream.into_split()` — Tokio split |
| When server disconnects | Stuck waiting on `stdin.lock().lines()` until next keypress | `server_lines.next_line()` returns `None` immediately; `break` exits |

The Project 11 client had a documented edge case: when the server disconnected, the main thread was stuck on `stdin.lock().lines()` and the program appeared frozen until the user typed something. The async client does not have this problem — both branches are polled in the same `select!` loop, so a `None` from the server branch exits immediately.

---

## The full client

```rust
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("connected — type to chat, Ctrl+D to quit");

    let (reader, mut writer) = stream.into_split();
    let mut server_lines = BufReader::new(reader).lines();
    let mut stdin_lines  = BufReader::new(io::stdin()).lines();

    loop {
        tokio::select! {
            line = stdin_lines.next_line() => {
                match line? {
                    Some(text) => writer.write_all(format!("{text}\n").as_bytes()).await?,
                    None => break,
                }
            }
            msg = server_lines.next_line() => {
                match msg? {
                    Some(text) => println!("{text}"),
                    None => { println!("[server disconnected]"); break; }
                }
            }
        }
    }

    Ok(())
}
```

---

## Running the full system

```sh
# terminal 1 — start the server
cargo run --bin server

# terminal 2 — first client
cargo run --bin client
connected — type to chat, Ctrl+D to quit
hello from A

# terminal 3 — second client
cargo run --bin client
connected — type to chat, Ctrl+D to quit
127.0.0.1:XXXXX: hello from A   ← appeared here
hi back

# terminal 2 also sees:
127.0.0.1:YYYYY: hi back
```

What you will observe:
- Messages appear immediately in all other clients
- Disconnecting one client (`Ctrl+D`) does not affect the others
- Killing the server closes all clients cleanly — the `None` branch fires and the client exits

---

## Exercises

> **TODO 1**: Add a `/quit` command. If the user types `/quit`, print `"goodbye"` and `break` instead of sending the text to the server. Where does this check go inside the `select!` branch?
>
> **TODO 2**: Ask for a username at startup before connecting. Read one line from stdin synchronously with `std::io::stdin().read_line(&mut buf)`, then prefix every outgoing message with `"[username] "` inside the stdin branch. Does the server need to change?
>
> **TODO 3**: Colour incoming messages differently from your own sent messages. ANSI escape codes work in most terminals: `"\x1b[32m"` switches to green, `"\x1b[0m"` resets. Wrap the `println!("{text}")` in the server branch so received messages print in green. What about the server's own echo of your messages — can you filter those out on the client side?

---

## Key APIs

| API | What it does |
|-----|-------------|
| `TcpStream::connect(addr).await` | Perform TCP handshake asynchronously; returns `TcpStream` |
| `stream.into_split()` | Split into `OwnedReadHalf` and `OwnedWriteHalf` |
| `tokio::io::stdin()` | Async handle to standard input |
| `BufReader::new(reader).lines()` | Async line iterator — yields on each `\n` |
| `lines.next_line().await` | Suspend until a line arrives; `Ok(None)` on EOF |
| `writer.write_all(bytes).await` | Write all bytes to the socket asynchronously |
| `tokio::select!` | Poll multiple async branches; handle whichever is ready first |
