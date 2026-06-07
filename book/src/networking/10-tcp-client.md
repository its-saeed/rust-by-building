# Lesson 3 — TCP Client

Lesson 1 wrote a server. Now you will write the client side — a program that connects to the echo server, sends a message, and reads the reply. You have all the pieces; this lesson shows how they fit together from the client's perspective.

---

## Project setup

```sh
rbb start
```

No external dependencies — everything is in `std::net` and `std::io`.

---

## Step 1 — Connect

```rust
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878")
        .expect("could not connect — is the echo server running?");

    println!("connected to {}", stream.peer_addr().unwrap());
}
```

`TcpStream::connect` performs the three-way handshake and returns a connected stream. If the server is not running, it returns an error immediately.

---

## Step 2 — Send and receive

```rust
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878")
        .expect("could not connect");

    let message = b"hello from the client!\n";
    stream.write_all(message).expect("send failed");
    println!("sent: {:?}", std::str::from_utf8(message).unwrap().trim());

    let mut buf = [0u8; 4096];
    let n = stream.read(&mut buf).expect("receive failed");
    println!("received: {:?}", std::str::from_utf8(&buf[..n]).unwrap().trim());
}
```

After `write_all`, the server echoes the bytes back. `read` blocks until they arrive.

---

## Step 3 — A two-way conversation

To send multiple messages, use a loop:

```rust
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:7878")
        .expect("could not connect");

    let mut writer = stream.try_clone().expect("clone failed");
    let reader = BufReader::new(stream);

    let messages = ["first message", "second message", "done"];

    for msg in &messages {
        let line = format!("{}\n", msg);
        writer.write_all(line.as_bytes()).expect("send failed");

        let mut reply = String::new();
        reader.lines().next().unwrap().expect("read failed").clone_into(&mut reply);
        println!("echo: {reply}");
    }
}
```

`try_clone` creates a second handle to the same socket — one for reading, one for writing. `BufReader` wraps the stream and lets us read line by line with `.lines()`.

---

## A simpler approach: write-then-read

For the exercises below, keep it simple — write all messages first, then close the write half, then read all the replies:

```rust
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878")
        .expect("could not connect");

    let payload = b"hello, echo server!";
    stream.write_all(payload).expect("write failed");

    // signal we are done writing
    stream.shutdown(std::net::Shutdown::Write).expect("shutdown failed");

    let mut response = Vec::new();
    stream.read_to_end(&mut response).expect("read failed");

    println!("echo: {}", String::from_utf8_lossy(&response));
}
```

`shutdown(Shutdown::Write)` sends a FIN for the write direction. The server sees `Ok(0)`, finishes echoing, and closes its end. Our `read_to_end` then gets all the echoed bytes followed by `Ok(0)`, and returns.

---

## Running the exercise

Start the echo server from Lesson 1 in one terminal:

```sh
# in the echo-server project
cargo run
```

Then run the client in another:

```sh
# in this project
cargo run
```

---

## Exercise

> **TODO 1**: Modify the client to read the message to send from command-line arguments (`std::env::args().skip(1).collect::<Vec<_>>().join(" ")`). Pass any text and see it echoed back.
>
> **TODO 2**: Send 5 different messages in a loop, printing each echo as it arrives. Add a 100ms sleep between sends (`std::thread::sleep(std::time::Duration::from_millis(100))`).
>
> **TODO 3**: Handle the case where the server is not running: instead of panicking, print a helpful message and exit with code 1.

---

## Reading a fixed amount

Sometimes you know exactly how many bytes to expect. Instead of `read` (which may return fewer bytes than the buffer), use `read_exact`:

```rust
let mut buf = [0u8; 19];   // expect exactly 19 bytes
stream.read_exact(&mut buf).expect("did not receive expected bytes");
```

`read_exact` keeps reading until the buffer is full or an error occurs. Use it when you know the response size in advance — for example, if your protocol sends a fixed-size header before the variable-length body.

---

## Key APIs

| API | What it does |
|-----|-------------|
| `TcpStream::connect(addr)` | Connect to a server; performs the handshake |
| `stream.write_all(&buf)` | Send all bytes |
| `stream.read(&mut buf)` | Read up to buf.len() bytes |
| `stream.read_to_end(&mut vec)` | Read until EOF into a Vec |
| `stream.read_exact(&mut buf)` | Read exactly buf.len() bytes |
| `stream.shutdown(Write)` | Send FIN for the write direction only |
| `stream.try_clone()` | Create a second handle to the same socket |
| `BufReader::new(stream)` | Add line-buffered reading |
