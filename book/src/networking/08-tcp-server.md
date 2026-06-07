# Lesson 1 — TCP Echo Server

The simplest useful server is an **echo server**: it reads whatever a client sends and sends it straight back. Every byte in, every byte out. It is trivial to write and immediately testable from the terminal.

You will write one in Rust using nothing but the standard library.

---

## Project setup

```sh
rbb start    # or: cargo new tcp-echo && cd tcp-echo
```

The only dependency is Rust's standard library — no crates needed.

---

## Step 1 — Bind and listen

```rust
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878")
        .expect("failed to bind port 7878");

    println!("listening on 127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.expect("incoming connection failed");
        println!("client connected: {}", stream.peer_addr().unwrap());
        handle(stream);
    }
}
```

`TcpListener::bind` creates a socket and starts listening. `listener.incoming()` is an iterator that blocks until a client connects and then yields a `TcpStream`. We call `handle` on each connection — one at a time.

---

## Step 2 — Echo the connection

```rust
use std::io::{Read, Write};
use std::net::TcpStream;

fn handle(mut stream: TcpStream) {
    let mut buf = [0u8; 4096];

    loop {
        let n = match stream.read(&mut buf) {
            Ok(0) => break,        // client closed the connection
            Ok(n) => n,
            Err(e) => {
                eprintln!("read error: {e}");
                break;
            }
        };

        if let Err(e) = stream.write_all(&buf[..n]) {
            eprintln!("write error: {e}");
            break;
        }
    }

    println!("client disconnected");
}
```

`read` fills the buffer and returns how many bytes arrived. **`Ok(0)` means the client closed the connection** — the remote end sent a FIN. We break out of the loop, `handle` returns, and the `TcpStream` is dropped, closing our end too.

`write_all` sends all `n` bytes. Unlike `write`, it does not return early — it keeps writing until all bytes are sent or an error occurs.

---

## Full program

```rust
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle(mut stream: TcpStream) {
    let mut buf = [0u8; 4096];
    loop {
        let n = match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => { eprintln!("read error: {e}"); break; }
        };
        if let Err(e) = stream.write_all(&buf[..n]) {
            eprintln!("write error: {e}"); break;
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878")
        .expect("failed to bind port 7878");
    println!("listening on 127.0.0.1:7878");
    for stream in listener.incoming() {
        let stream = stream.expect("incoming connection failed");
        handle(stream);
    }
}
```

---

## Test it with `nc`

Open two terminals.

**Terminal 1** — run the server:

```sh
cargo run
```

```
listening on 127.0.0.1:7878
```

**Terminal 2** — connect with netcat:

```sh
nc 127.0.0.1 7878
```

Type anything and press Enter. You should see the same text echoed back:

```
hello
hello
world
world
```

Press `Ctrl-D` to close the connection. In Terminal 1, you will see:

```
client connected: 127.0.0.1:54321
client disconnected
```

The server is now waiting for the next client.

---

## Exercise

> **TODO 1**: Print the number of bytes received for each read, alongside the client's address.
>
> **TODO 2**: In `handle`, count the total bytes echoed over the whole connection and print it when the client disconnects.
>
> **TODO 3**: Convert the echoed data to uppercase before sending it back. (`buf[..n].iter_mut().for_each(|b| b.make_ascii_uppercase())`)

---

## What just happened

- `TcpListener::bind` — created a socket, bound to port 7878, started listening
- `listener.incoming()` — blocked until a client connected; yielded a `TcpStream`
- `stream.read` — blocked until the client sent data; returned how many bytes arrived
- `Ok(0)` — the client's FIN arrived; the connection is closed
- `stream.write_all` — sent data back
- Dropping `stream` — sent our own FIN, completing the four-way close

The server handles one client at a time. While it is in `handle`, a second client attempting to connect will wait in the OS's accept queue. When `handle` returns, the server loops and accepts the next client.

---

## Key APIs

| API | What it does |
|-----|-------------|
| `TcpListener::bind(addr)` | Create and bind a server socket |
| `listener.incoming()` | Iterator over incoming connections |
| `stream.peer_addr()` | Address of the connected client |
| `stream.read(&mut buf)` | Read up to buf.len() bytes; returns bytes read |
| `stream.write_all(&buf)` | Write all bytes; retries internally |
| `Ok(0)` from `read` | Client closed the connection |
