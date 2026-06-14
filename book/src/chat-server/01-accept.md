# Lesson 1 — Accept and Echo

The server's first job is accepting connections and handling each one independently. This lesson builds that foundation: a server that echoes back whatever a client sends. No broadcasting yet — just one thread per client, running in parallel.

---

## The single-threaded problem

Before writing any threading code, it is worth understanding exactly why a single-threaded server breaks down.

Imagine a server that handles one client at a time:

```rust
// single-threaded — broken for multiple clients
loop {
    let (stream, addr) = listener.accept()?;  // step 1: wait for connection
    handle_client(stream);                     // step 2: handle it fully
    // only now do we loop back to accept()
}
```

Here is what happens when two clients connect:

```
time ──────────────────────────────────────────────────────────────▶

server:   [accept]──[handling client A────────────────]──[accept]──[handling B]

client A:            ████ typing and receiving ████

client B: connects ──────────────────────────────────▶ finally gets a response
                     (waiting the whole time client A is being served)
```

`handle_client` blocks the thread. While it runs, `accept()` is never called. Client B's TCP connection request sits in the OS's listen backlog — the kernel queues it — but no one picks it up until client A is finished.

This is not a flaw in the code; it is the fundamental nature of blocking I/O on a single thread. The fix is to handle each client on its own thread so they run in parallel.

---

## What `TcpListener::bind` does

```rust
use std::net::TcpListener;

let listener = TcpListener::bind("0.0.0.0:8080").expect("bind failed");
```

Under the hood this makes two system calls:

1. `socket()` — asks the kernel to create a TCP socket
2. `bind()` — reserves port 8080 on all interfaces (`0.0.0.0`)
3. `listen()` — tells the kernel to start accepting connections into a queue

After `bind`, clients can connect. The kernel completes the TCP three-way handshake with them and queues the established connections. Your program has not called `accept` yet — that just pops one connection off the queue.

---

## The accept loop

`listener.incoming()` is an iterator. Each time you call `next()` on it (which `for` does automatically) it calls `accept()` on the OS — blocking until a client is in the queue, then returning the connection as a `TcpStream`:

```rust
for stream in listener.incoming() {
    let stream = stream.expect("accept failed");
    let addr   = stream.peer_addr().unwrap();
    println!("new connection from {addr}");
    handle_client(stream);  // ← this blocks; we will fix it next
}
```

`peer_addr()` gives the client's IP and ephemeral port — useful for logging.

---

## Step 3 — One thread per client

Replace the blocking call with a `thread::spawn`:

```rust
use std::thread;

for stream in listener.incoming() {
    let stream = stream.expect("accept failed");
    println!("connection from {}", stream.peer_addr().unwrap());

    thread::spawn(move || {
        handle_client(stream);
    });
}
```

Now the timeline looks like this:

```
time ──────────────────────────────────────────────────────────────▶

main:      [accept]──[spawn]──[accept]──[spawn]──[accept]──[spawn]──...
                        │                  │
thread A:               └──[handle A───────────────────]
                                           │
thread B:                                  └──[handle B──────────]
```

The main thread does nothing but accept connections and spawn threads. Each client gets its own thread with its own stack, running `handle_client` independently. When client A is slow, client B is unaffected.

The `move` keyword is required: `stream` must be *owned* by the closure because the thread might outlive the current loop iteration. See the Closures chapter for a full explanation.

---

## Step 4 — Reading lines from a TCP stream

`TcpStream` implements `Read`, which gives you raw bytes. Reading byte by byte to find newlines would be tedious. Instead, wrap the stream in a `BufReader`:

```rust
use std::io::{BufRead, BufReader};

fn handle_client(stream: TcpStream) {
    let reader = BufReader::new(stream);

    for line in reader.lines() {
        match line {
            Ok(text) => println!("received: {text}"),
            Err(_)   => break,
        }
    }

    println!("client disconnected");
}
```

**What `BufReader` does:**

Instead of asking the OS for one byte at a time (one system call each), `BufReader` asks for a large chunk — typically 8 KB — and stores it internally. Calls to `.lines()` then read from that internal buffer, only going back to the OS when the buffer is empty. Far fewer system calls, much faster.

```
without BufReader:         with BufReader:

'h' ← syscall             'h','e','l','l','o','\n' ← one syscall (8KB chunk)
'e' ← syscall             then served from buffer:
'l' ← syscall               → "hello"
'l' ← syscall
'o' ← syscall
'\n' ← syscall
 → "hello"
```

`reader.lines()` returns an iterator of `Result<String>`. Each iteration blocks until a `\n` arrives. When the client closes the connection, the OS signals EOF — the next read returns an error, and `break` exits the loop cleanly.

---

## Step 5 — Echoing back

To write back to the client we need a second handle to the same connection. `BufReader::new(stream)` consumes `stream` — we can no longer write to it directly. The solution is `try_clone()`, which asks the OS for a second file descriptor pointing to the same socket:

```
kernel socket table:
  fd 5 ──▶  [TCP socket: 127.0.0.1:8080 ↔ 127.0.0.1:54321]
  fd 6 ──▶  [same socket]   ← try_clone() creates this

reader uses fd 5 (via BufReader)
writer uses fd 6 (directly)
```

Both file descriptors read from and write to the same underlying TCP connection. Closing either one does not close the socket — the OS keeps it open until all file descriptors for it are closed.

```rust
use std::io::{BufRead, BufReader, Write};

fn handle_client(stream: TcpStream) {
    let mut writer = stream.try_clone().expect("clone failed");
    let reader     = BufReader::new(stream);  // stream moved here

    for line in reader.lines() {
        match line {
            Ok(text) => {
                writeln!(writer, "{text}").ok();
            }
            Err(_) => break,
        }
    }
}
```

`writeln!(writer, "{text}")` sends the text followed by `\n`. `.ok()` discards errors silently — if the client disconnected, the write will fail, and the reader loop will also fail on its next iteration and break.

---

## Full server

```rust
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(stream: TcpStream) {
    let peer = stream.peer_addr().unwrap();
    let mut writer = stream.try_clone().expect("clone failed");
    let reader     = BufReader::new(stream);

    println!("{peer} connected");

    for line in reader.lines() {
        match line {
            Ok(text) => { writeln!(writer, "{text}").ok(); }
            Err(_)   => break,
        }
    }

    println!("{peer} disconnected");
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").expect("bind failed");
    println!("listening on :8080");

    for stream in listener.incoming() {
        let stream = stream.expect("accept failed");
        thread::spawn(move || handle_client(stream));
    }
}
```

---

## Test it with `nc`

```sh
# terminal 1 — start the server
cargo run --bin server

# terminal 2 — connect client A
nc 127.0.0.1 8080
hello
hello          ← echoed back

# terminal 3 — connect client B (while A is still connected)
nc 127.0.0.1 8080
world
world          ← also echoed back, independently
```

Both clients work at the same time. The server log shows both connections and disconnections interleaved.

---

## Exercise

> **TODO 1**: Add a per-client message counter. Print `"{addr} disconnected after {n} messages"` when the loop ends. Where does the counter variable live?
>
> **TODO 2**: Prefix each echoed line with the sender's address: `"[127.0.0.1:54321] hello"`. You need `peer_addr()` before moving `stream` into `BufReader`.
>
> **TODO 3**: Connect with `nc`, send a few lines, then kill `nc` with `Ctrl+C`. What does the server log? Now kill the server while `nc` is connected — what happens in the `nc` terminal?

---

## Key APIs

| API | What it does |
|-----|-------------|
| `TcpListener::bind(addr)` | Reserve a port and start listening |
| `listener.incoming()` | Blocking iterator — yields one connection per `next()` |
| `stream.peer_addr()` | The remote client's IP and port |
| `stream.try_clone()` | Ask the OS for a second file descriptor to the same socket |
| `BufReader::new(stream)` | Buffer reads for efficiency; enables `.lines()` |
| `reader.lines()` | Iterator of text lines; `Err` when connection closes |
| `writeln!(writer, "...")` | Write a line followed by `\n` |
