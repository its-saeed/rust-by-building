# Lesson 1 — Accept and Echo

The server's first job is to accept connections and handle each one on its own thread. Start simple: whatever a client sends, echo it back. No broadcasting yet — just one thread per client.

---

## Step 1 — Listening for connections

```rust
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").expect("bind failed");
    println!("listening on :8080");
}
```

`bind` reserves port 8080 on all network interfaces. The call fails if something else is already using that port.

---

## Step 2 — The accept loop

`listener.incoming()` is an iterator that blocks until the next client connects, then yields a `TcpStream`:

```rust
for stream in listener.incoming() {
    let stream = stream.expect("accept failed");
    println!("client connected: {:?}", stream.peer_addr());
    // handle the client here
}
```

`peer_addr()` gives the client's IP address and port — useful for logging.

The problem: `handle_client` will block until that client disconnects. While it is running, the loop cannot call `accept` again. A second client gets no response until the first leaves.

---

## Step 3 — Spawn a thread per client

Move the client handling into a separate thread so the accept loop keeps running:

```rust
use std::thread;

for stream in listener.incoming() {
    let stream = stream.expect("accept failed");
    thread::spawn(move || {
        handle_client(stream);
    });
}
```

Now: client connects → thread spawned → loop immediately calls `accept` again. Each client gets its own thread and they run in parallel.

---

## Step 4 — Reading lines from a stream

`TcpStream` implements `Read`, but reading byte by byte is tedious. Wrap it in a `BufReader` to get line-by-line reading:

```rust
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

fn handle_client(stream: TcpStream) {
    let reader = BufReader::new(stream);

    for line in reader.lines() {
        match line {
            Ok(text) => println!("received: {text}"),
            Err(_)   => break,  // client disconnected
        }
    }

    println!("client disconnected");
}
```

`reader.lines()` returns an iterator. Each call blocks until a newline arrives. When the client closes the connection, the next `lines()` call returns an error — the `break` exits the loop cleanly.

---

## Step 5 — Echoing back

To write back to the client we need a writer. But `BufReader` consumed the stream. Solution: `try_clone()` creates a second handle to the same connection — one for reading, one for writing:

```rust
use std::io::{BufRead, BufReader, Write};

fn handle_client(stream: TcpStream) {
    let mut writer = stream.try_clone().expect("clone failed");
    let reader     = BufReader::new(stream);

    for line in reader.lines() {
        match line {
            Ok(text) => {
                println!("received: {text}");
                writeln!(writer, "{text}").ok();  // echo back
            }
            Err(_) => break,
        }
    }
}
```

`writeln!(writer, "{text}")` sends the line back with a newline appended. `.ok()` discards the error — if the client already disconnected, the write will fail silently, and the reader will also fail on the next iteration.

---

## Full server

```rust
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(stream: TcpStream) {
    let mut writer = stream.try_clone().expect("clone failed");
    let reader     = BufReader::new(stream);

    for line in reader.lines() {
        match line {
            Ok(text) => { writeln!(writer, "{text}").ok(); }
            Err(_)   => break,
        }
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").expect("bind failed");
    println!("listening on :8080");

    for stream in listener.incoming() {
        let stream = stream.expect("accept failed");
        println!("client connected: {}", stream.peer_addr().unwrap());
        thread::spawn(move || handle_client(stream));
    }
}
```

---

## Test it with `nc`

```sh
# terminal 1
cargo run --bin server

# terminal 2
nc 127.0.0.1 8080
hello        ← you type this
hello        ← server echoes back
```

Open two `nc` sessions — both work simultaneously.

---

## Exercise

> **TODO 1**: Add a message counter per client. Print "client X disconnected after N messages" when the loop ends.
>
> **TODO 2**: Prefix each echoed line with the client's address: `"[127.0.0.1:54321] hello"`. Use `stream.peer_addr()` before moving the stream into `BufReader`.
>
> **TODO 3**: What happens if you connect with `nc`, type some text, then press `Ctrl+C`? What does the server print? What does the `Err(_)` branch receive?

---

## Key APIs

| API | What it does |
|-----|-------------|
| `TcpListener::bind(addr)` | Reserve a port, ready to accept connections |
| `listener.incoming()` | Iterator that blocks until the next connection arrives |
| `stream.peer_addr()` | The remote client's IP address and port |
| `stream.try_clone()` | Second handle to the same TCP connection |
| `BufReader::new(stream)` | Wrap a stream for line-by-line reading |
| `reader.lines()` | Iterator of lines; ends when connection closes |
