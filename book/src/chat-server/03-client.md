# Lesson 3 — The Client

The server is done. Now build a proper client so users do not need `nc`. The client has two jobs that must happen at the same time: reading what the user types and printing what arrives from the server. Two jobs in parallel — two threads.

---

## The problem

A chat client needs to do two things concurrently:

1. Read lines from `stdin` → send to server
2. Read lines from the server → print to terminal

Both operations block. `stdin.lock().lines()` blocks until the user presses Enter. `reader.lines()` blocks until the server sends something. If you run them sequentially, one always blocks the other: either you cannot receive while typing, or you cannot type while waiting to receive.

The solution is the same as the server's: one thread per blocking operation.

---

## Step 1 — Connect

```rust
use std::net::TcpStream;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:8080").expect("could not connect");
    println!("connected to server");
}
```

`TcpStream::connect` blocks until the TCP handshake completes (or fails). After this, `stream` is a bidirectional channel to the server.

---

## Step 2 — Split the stream

Just like in the server, `try_clone` creates two handles:

```rust
let mut writer = stream.try_clone().expect("clone failed");
// stream moves into the reader thread
// writer stays in the main thread (or vice versa)
```

One handle goes to the thread that reads from the server; the other stays for writing to the server.

---

## Step 3 — Reader thread

Spawn a thread that prints everything arriving from the server:

```rust
use std::io::{BufRead, BufReader};
use std::thread;

thread::spawn(move || {
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        match line {
            Ok(text) => println!("{text}"),
            Err(_)   => {
                println!("disconnected from server");
                break;
            }
        }
    }
});
```

When the server closes the connection, `lines()` returns an error, the thread prints a message and exits.

---

## Step 4 — Stdin loop

The main thread reads from stdin and sends each line to the server:

```rust
use std::io::{self, BufRead, Write};

let stdin = io::stdin();
for line in stdin.lock().lines() {
    let line = line.expect("stdin error");
    writeln!(writer, "{line}").expect("server disconnected");
}
```

`stdin.lock().lines()` is the same `BufRead` iterator as `BufReader::new(stream).lines()` — it blocks until the user presses Enter, yields the line, repeats. It ends when stdin closes (the user presses `Ctrl+D`).

If `writeln!` fails (server disconnected), `expect` exits the process.

---

## Full client

```rust
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:8080").expect("could not connect");
    println!("connected — type to chat, Ctrl+D to quit");

    let mut writer = stream.try_clone().expect("clone failed");

    thread::spawn(move || {
        let reader = BufReader::new(stream);
        for line in reader.lines() {
            match line {
                Ok(text) => println!("{text}"),
                Err(_)   => { println!("server closed connection"); break; }
            }
        }
    });

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
# terminal 1
cargo run --bin server

# terminal 2
cargo run --bin client

# terminal 3
cargo run --bin client
```

Type in either client — the message appears in both. Kill one client with `Ctrl+D`. The other keeps working.

---

## The two-thread client pattern

This pattern — one thread for each direction of a blocking I/O pair — appears everywhere:

- CLI tools that stream results while accepting commands
- Game clients that receive world state while sending input
- Any interactive network program

The shape is always the same: `try_clone` to split the stream, `spawn` for the inbound direction, main thread for the outbound direction (or vice versa).

---

## Exercise

> **TODO 1**: Accept the server address as a command-line argument: `cargo run --bin client -- 127.0.0.1:8080`. Use `std::env::args()`. Default to `127.0.0.1:8080` if no argument is given.
>
> **TODO 2**: Ask for a username at startup (before connecting). Prefix every sent line with `"[username] "` so the server broadcasts it with the name attached.
>
> **TODO 3**: When the reader thread detects that the server closed the connection, the main thread is still blocked on `stdin.lock().lines()`. The program appears to hang until the user presses Enter. How would you fix this? (Hint: consider what happens if the writer end of the stream is shut down.)

---

## Key APIs

| API | What it does |
|-----|-------------|
| `TcpStream::connect(addr)` | Open a TCP connection to a server |
| `stream.try_clone()` | Second handle for bidirectional split |
| `io::stdin().lock().lines()` | Read lines from the terminal, one per Enter |
| `writeln!(writer, "{text}")` | Send a line to the server |
