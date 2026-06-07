# Chapter 4 — TCP: Reliable Streams

IP delivers packets best-effort: they can arrive out of order, get dropped by a congested router, or be duplicated. For most applications — loading a webpage, downloading a file, making a database query — "probably arrives" is not good enough. You need a guarantee.

**TCP** (Transmission Control Protocol) provides that guarantee. It runs on top of IP and makes it look like a reliable, ordered stream of bytes from one point to another.

---

## The connection

TCP is connection-oriented. Before any data flows, the two sides must **establish** a connection; after they are done, they **tear it down**. A connection is identified by four values:

```
(source IP, source port, destination IP, destination port)
```

No two active connections on the same machine can have all four values identical.

### The three-way handshake

Establishing a connection requires three messages, called the **handshake**:

```
Client                          Server
  │                               │
  │──── SYN ──────────────────────▶│   "I want to connect"
  │◀─── SYN-ACK ──────────────────│   "OK, I'm ready"
  │──── ACK ──────────────────────▶│   "Great, let's go"
  │                               │
  │  ← data flows in both directions →  │
```

- **SYN** (synchronise): client picks a random starting sequence number and sends it
- **SYN-ACK**: server acknowledges the client's sequence number and sends its own
- **ACK**: client acknowledges the server's sequence number

Only after this exchange can data flow. This is why connecting to a server that is unreachable produces a noticeable delay — your OS is waiting for the SYN-ACK that will never arrive, retrying a few times, then giving up.

---

## How reliability works

TCP assigns a **sequence number** to every byte of data. The receiver sends back **acknowledgements** (ACKs) confirming which bytes it has received. If the sender does not receive an ACK within a timeout, it **retransmits** the data.

This means:

- **No data is lost** — anything unacknowledged gets retransmitted
- **Data arrives in order** — the receiver buffers out-of-order segments and delivers them in sequence
- **No duplicates** — the receiver discards segments it has already seen

The cost is **latency**: every packet must be acknowledged before the sender knows it arrived safely. For applications that need speed over reliability — video streaming, online games, voice calls — this cost is too high. Those use UDP instead.

---

## Flow control and backpressure

The receiver tells the sender how much buffer space it has with a **window size**. The sender cannot have more unacknowledged bytes in flight than the window size. If your program reads from a TCP socket slowly, the receive buffer fills up, the window size drops to zero, and the sender pauses. This is **flow control** — the receiver controls the rate.

As a programmer, you mostly do not think about this. It is handled by the OS. But it explains why a fast sender can back off automatically when a slow receiver gets overwhelmed — a property that makes TCP-based systems self-regulating.

---

## Closing a connection

Closing is also a multi-step process. Either side can send a **FIN** (finish) to say "I am done sending data." The other side acknowledges it and eventually sends its own FIN. Until both FINs are exchanged and acknowledged, the connection is not fully closed.

In Rust, this happens when a `TcpStream` is dropped. You can also call `shutdown()` explicitly to signal that you have finished writing while still reading.

---

## What TCP looks like from Rust

In Rust's standard library, a TCP connection is a `TcpStream`. It implements `Read` and `Write` — the same traits as files. You read from it and write to it with the same API you already know.

A server uses `TcpListener` to accept incoming connections:

```rust
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

let listener = TcpListener::bind("127.0.0.1:7878")?;

for stream in listener.incoming() {
    let mut stream = stream?;
    let mut buf = [0u8; 1024];
    let n = stream.read(&mut buf)?;
    stream.write_all(&buf[..n])?;  // echo it back
}
```

`listener.incoming()` blocks until a client connects, then yields a `TcpStream`. The rest is just `Read` and `Write`.

A client connects with:

```rust
let mut stream = TcpStream::connect("127.0.0.1:7878")?;
stream.write_all(b"hello")?;
```

The full implementations — with error handling and real exercises — are in the Network Programming lessons.

---

## Terminal — `netcat` (nc)

`netcat` is a raw TCP/UDP tool for the terminal. You can use it to connect to any TCP server and type directly:

```sh
nc 127.0.0.1 7878
```

Type something and press Enter. If a server is listening, it will receive your text. When you write your echo server in the upcoming lesson, `nc` is how you will test it.

To close the connection, press `Ctrl-D` (sends EOF) or `Ctrl-C`.

---

## Terminal — `ss` / `netstat -an`

To see all open TCP connections on your machine:

```sh
# Linux
ss -tn

# macOS
netstat -an -p tcp
```

Look for `LISTEN` state — those are servers waiting for connections. Look for `ESTABLISHED` — those are active connections. When you run your echo server, you will see it appear as `LISTEN` on port 7878.

---

## What to carry forward

- TCP provides **reliable, ordered, byte-stream** delivery on top of unreliable IP
- Connection setup is a **three-way handshake** (SYN, SYN-ACK, ACK)
- **Sequence numbers + ACKs + retransmission** make reliability possible; the cost is latency
- **Flow control** prevents a fast sender from overwhelming a slow receiver
- In Rust: `TcpListener` on the server side, `TcpStream` on both sides; both implement `Read` and `Write`
- `nc` (netcat) is the easiest way to test a TCP server from the terminal

The next chapter covers **UDP** — the alternative when you need speed more than reliability.
