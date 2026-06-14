# Lesson 3 — The Client

The server is complete. Now build a proper client: a terminal program that connects, lets the user type, and shows incoming messages as they arrive. The interesting part is that these two things — typing and receiving — must happen at the same time, and both operations block.

---

## The two-direction problem

A TCP connection is **full-duplex**: data flows in both directions simultaneously. The client needs to:

1. Read lines from `stdin` → send to server
2. Receive lines from the server → print to terminal

The problem: both operations are blocking.

- `stdin.lock().lines()` blocks until the user presses Enter
- `socket.read()` blocks until the server sends something

If you handle them sequentially, whichever runs first blocks the other:

```
sequential — broken:

time ──────────────────────────────────────────────────────────▶

read stdin:  [waiting for user to type────────────────────────]
             (cannot receive server messages while waiting)

─ OR ─

read socket: [waiting for server message──────────────────────]
             (cannot read user input while waiting)
```

There is no way to interleave two blocking calls on one thread. The solution is the same one used throughout this project: **one thread per blocking operation**.

```
two threads — correct:

time ──────────────────────────────────────────────────────────▶

reader thread:  [recv───print] [recv──────print] [recv──print]
main thread:    [user types──send] [user types──send]
```

Both run in parallel. The user can type at any time, and server messages print immediately regardless of what the user is doing.

---

## Step 1 — Connect to the server

```rust
use std::net::TcpStream;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:8080")
        .expect("could not connect to server");

    println!("connected — type to chat, Ctrl+D to quit");
}
```

`TcpStream::connect` performs the TCP three-way handshake:

```
client          server
  │──── SYN ────▶│
  │◀─── SYN-ACK ─│
  │──── ACK ────▶│
  │    connected  │
```

The call blocks until this handshake completes (or fails with a timeout). After it returns, `stream` is a bidirectional byte channel to the server.

---

## Step 2 — Splitting the stream with `try_clone`

One `TcpStream` handle cannot be used from two threads at the same time — moving it into the reader thread would make it unavailable for writing. `try_clone()` asks the OS for a second file descriptor pointing to the same underlying socket:

```
OS socket table (after try_clone):

  fd 4 ──▶ ┌────────────────────────────────────────┐
            │  TCP socket                            │
            │  local:  127.0.0.1:54321              │
  fd 5 ──▶ │  remote: 127.0.0.1:8080               │
            └────────────────────────────────────────┘

fd 4 → reader thread (receives from server)
fd 5 → main thread   (sends to server)
```

Both file descriptors read from and write to the same connection. The OS keeps the socket open until both are closed.

```rust
let mut writer = stream.try_clone().expect("clone failed");
// stream → reader thread
// writer → main thread
```

---

## Step 3 — The reader thread

Spawn a thread that sits in a loop printing everything the server sends:

```rust
use std::io::{BufRead, BufReader};
use std::thread;

thread::spawn(move || {
    let reader = BufReader::new(stream);  // stream moved here
    for line in reader.lines() {
        match line {
            Ok(text) => println!("{text}"),
            Err(_)   => {
                println!("[server closed connection]");
                break;
            }
        }
    }
});
```

This thread blocks on `reader.lines()` until the server sends a newline, prints it, and loops. It exits when the server closes the connection (the `Err` branch).

The `move` keyword is required: `stream` must be owned by the closure because the thread may outlive the `main` function's current point of execution. Without `move`, the closure would borrow `stream` — and the compiler would reject it because that borrow might outlive the owner.

---

## Step 4 — The stdin loop

The main thread reads from the terminal and sends each line to the server:

```rust
use std::io::{self, BufRead, Write};

let stdin = io::stdin();
for line in stdin.lock().lines() {
    let line = line.expect("stdin error");
    writeln!(writer, "{line}").expect("lost connection to server");
}
```

`stdin.lock().lines()` is the same `BufRead` iterator as `BufReader::new(socket).lines()` — it blocks until the user presses Enter, then yields the typed line. It ends when stdin is closed (`Ctrl+D` on Linux/macOS, `Ctrl+Z` on Windows).

`writeln!(writer, "{line}")` sends the text to the server, followed by a newline. The server's reader loop uses `.lines()` too, so it waits for the newline to know the message is complete.

If `writeln!` fails — the server disconnected — `.expect` exits the process. That is fine for a simple client.

---

## Full client

```rust
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:8080")
        .expect("could not connect to server");

    println!("connected — type to chat, Ctrl+D to quit");

    let mut writer = stream.try_clone().expect("clone failed");

    // reader thread: print everything from the server
    thread::spawn(move || {
        let reader = BufReader::new(stream);
        for line in reader.lines() {
            match line {
                Ok(text) => println!("{text}"),
                Err(_)   => { println!("[server closed connection]"); break; }
            }
        }
    });

    // main thread: read stdin, send to server
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("stdin error");
        writeln!(writer, "{line}").expect("lost connection to server");
    }
}
```

---

## Running the full system

```sh
# terminal 1 — the server
cargo run --bin server

# terminal 2 — first client
cargo run --bin client
> hello from client A

# terminal 3 — second client (sees the message above)
cargo run --bin client
hello from client A     ← arrived from the server
> hello back
```

What you will observe:
- Every message typed in one client appears in the other(s) almost instantly
- Disconnecting a client (`Ctrl+D`) does not affect the others
- The server logs connections, messages, and disconnections

---

## An edge case: the hanging client

When the server closes the connection, the reader thread exits. But the main thread is blocked on `stdin.lock().lines()` — it is still waiting for the user to press Enter. The program appears frozen until the user types something, at which point `writeln!` fails and `.expect` exits.

This is a known limitation of this design. Fixing it properly requires either:
- A non-blocking stdin read (platform-specific)
- Shutting down the `writer` from the reader thread with `writer.shutdown(Shutdown::Both)` — this causes `writeln!` to fail immediately
- A more sophisticated event loop

For a learning project this behaviour is acceptable. The exercise below asks you to think through the fix.

---

## The two-thread client pattern

This pattern — one thread per direction on a blocking bidirectional channel — appears in many real programs:

| Program | Inbound thread | Outbound thread |
|---------|---------------|----------------|
| Chat client (this) | Receive messages → print | Read stdin → send |
| Game client | Receive world state → update | Read input → send commands |
| SSH client | Receive remote output → display | Read local input → send |
| Database client | Receive query results → parse | Send queries |

The shape is always the same: `try_clone` (or equivalent), `spawn` for the inbound direction, main thread for the outbound direction.

---

## Exercise

> **TODO 1**: Accept the server address as a command-line argument — `cargo run --bin client -- 192.168.1.5:8080`. Use `std::env::args().nth(1)` with a default of `"127.0.0.1:8080"`.
>
> **TODO 2**: Ask for a username before connecting. Read it from stdin, then prefix every outgoing line with `"[username] "`. Does the server need to change?
>
> **TODO 3**: When the server disconnects, the reader thread prints a message and exits — but the main thread is stuck on `stdin.lock().lines()`. Use `writer.shutdown(std::net::Shutdown::Both)` inside the reader thread (you will need to `try_clone` the writer and move it into the reader thread) to unblock the main thread. Does this solve the problem cleanly?

---

## Key APIs

| API | What it does |
|-----|-------------|
| `TcpStream::connect(addr)` | Perform TCP handshake; block until connected |
| `stream.try_clone()` | Second OS file descriptor to the same socket |
| `BufReader::new(stream)` | Buffer reads; enables `.lines()` |
| `io::stdin().lock().lines()` | Blocking iterator of lines from the terminal |
| `writeln!(writer, "{text}")` | Send a line to the server (with newline) |
| `stream.shutdown(Shutdown::Both)` | Close both read and write halves immediately |
