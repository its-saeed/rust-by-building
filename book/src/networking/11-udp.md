# Lesson 4 — UDP

Everything so far has been TCP — connections, streams, guaranteed delivery. UDP is different in every way: no connection, no stream, no guarantees. You send a chunk of bytes to an address; it either arrives or it does not.

In this lesson you will write both a UDP echo server and a UDP client.

---

## Project setup

```sh
rbb start
```

`std::net::UdpSocket` — no external crates needed.

---

## UDP echo server

```rust
use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:7879")
        .expect("failed to bind");

    println!("UDP server listening on 127.0.0.1:7879");

    let mut buf = [0u8; 1472];   // stay under Ethernet MTU

    loop {
        let (n, from) = socket.recv_from(&mut buf)
            .expect("recv_from failed");

        println!("datagram from {from}: {} bytes", n);

        socket.send_to(&buf[..n], from)
            .expect("send_to failed");
    }
}
```

`recv_from` blocks until a datagram arrives. It returns the data length and the sender's address. `send_to` sends the echo back to that address.

Notice there is no `accept`, no `incoming`, no separate stream. The same socket receives from any address and sends to any address.

---

## UDP client

```rust
use std::net::UdpSocket;

fn main() {
    // bind to any free port on loopback
    let socket = UdpSocket::bind("127.0.0.1:0")
        .expect("bind failed");

    let server = "127.0.0.1:7879";
    let message = b"hello UDP!";

    socket.send_to(message, server)
        .expect("send_to failed");

    println!("sent: {:?}", std::str::from_utf8(message).unwrap());

    let mut buf = [0u8; 1472];
    let (n, from) = socket.recv_from(&mut buf)
        .expect("recv_from failed");

    println!("echo from {from}: {:?}", std::str::from_utf8(&buf[..n]).unwrap());
}
```

`bind("127.0.0.1:0")` asks the OS to pick any free port. The client does not need a known port — only servers do.

---

## Running the exercise

**Terminal 1** — start the UDP server:

```sh
cargo run --bin server
```

Or if it is a single binary, in the server project:

```sh
cargo run
```

**Terminal 2** — run the client:

```sh
cargo run
```

You should see the server print the received datagram and the client print the echo.

---

## Testing with nc

`nc -u` also works for UDP:

```sh
# connect to the UDP server
nc -u 127.0.0.1 7879
```

Type a line and press Enter. The server echoes it back. But UDP has a subtle behaviour here: `nc -u` may wait for your whole session before printing any response, depending on your platform. The Rust client above is more reliable for testing.

---

## The key difference from TCP

With TCP:

```
connect → write → read → write → read → close
```

With UDP:

```
send_to → recv_from → send_to → recv_from → (no close)
```

There is no connection. Each datagram is independent. The receiver does not know if more are coming. The sender does not know if the last one arrived.

---

## A more complete example: multiple messages

```rust
use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:0").expect("bind failed");
    let server = "127.0.0.1:7879";

    let messages = ["first", "second", "third"];
    let mut buf = [0u8; 1472];

    for msg in &messages {
        socket.send_to(msg.as_bytes(), server).expect("send failed");

        let (n, _) = socket.recv_from(&mut buf).expect("recv failed");
        let reply = std::str::from_utf8(&buf[..n]).unwrap();
        println!("{msg} → {reply}");
    }
}
```

Each send/recv pair is independent — no state carries between them.

---

## Exercise

> **TODO 1**: Add a sequence number to each message — prefix it with the message number as a single byte. Parse the sequence number on the server and print it alongside the datagram content.
>
> **TODO 2**: In the client, send all messages without waiting for replies, then read all replies. Does order change? Try with a small `sleep` between sends.
>
> **TODO 3**: Make the server count how many datagrams it has received from each address. Print the count alongside each echo. Use a `HashMap<SocketAddr, u32>`.

---

## When you see `Ok(0)` in UDP

`Ok(0)` from `recv_from` means a zero-length datagram arrived — it does not mean the sender closed the "connection" (there is no connection to close). Zero-length datagrams are unusual but valid.

Unlike TCP, there is no EOF signal in UDP. Your server loops forever, and your client just stops calling `recv_from` when it is done.

---

## Key APIs

| API | What it does |
|-----|-------------|
| `UdpSocket::bind(addr)` | Create a socket and bind to local address |
| `socket.send_to(&buf, addr)` | Send a datagram to a specific address |
| `socket.recv_from(&mut buf)` | Block until a datagram arrives; returns (n, from) |
| `socket.connect(addr)` | Set a default destination (enables `send`/`recv` without address) |

`connect` on a UDP socket does not establish a connection — it just sets a default destination address so you can call `send` and `recv` instead of `send_to` and `recv_from`. The socket remains connectionless; you can still receive from any address.

---

## What you have built

Across these four lessons you have:

1. A **TCP echo server** — `TcpListener`, `TcpStream`, `Read`, `Write`, `Ok(0)` for EOF
2. An **HTTP client** — `reqwest`, `serde`, JSON deserialisation, a real weather API
3. A **TCP client** — `TcpStream::connect`, `shutdown`, `read_to_end`
4. A **UDP echo server and client** — `UdpSocket`, `send_to`, `recv_from`

These are the building blocks of almost every networked Rust program. Larger systems — async servers, web frameworks, databases, game backends — are all variations on these primitives.
