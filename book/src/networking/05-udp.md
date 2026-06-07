# Chapter 5 — UDP: Fast Datagrams

TCP's guarantees — reliability, ordering, flow control — come at a price. Every packet must be acknowledged. Lost packets are retransmitted. If a packet is delayed by 50 ms, every packet behind it waits. For applications that cannot afford to wait, there is **UDP**.

---

## What UDP does (and does not do)

UDP (User Datagram Protocol) is minimal. It adds port numbers to IP — that is almost it. No connection, no acknowledgement, no retransmission, no ordering guarantee.

A UDP **datagram** is self-contained: you send a chunk of data, it either arrives at the destination or it does not. You are not told which. The receiver gets the data as a single unit — not a stream of bytes.

What UDP gives you instead:

- **Low latency** — no handshake, no waiting for ACKs
- **Discrete messages** — each send is a separate datagram; boundaries are preserved
- **No connection overhead** — you can send a datagram to any address at any time

---

## When to use UDP

The question is not "which is better" but "which fits the application."

**Use TCP when:**
- Data loss is unacceptable (file transfer, database queries, HTTP)
- Order matters and you want the OS to handle it
- You need a persistent, reliable channel

**Use UDP when:**
- Latency matters more than reliability (online games, voice calls, video streaming)
- You can handle loss yourself (e.g. just use the next frame)
- You need to broadcast to many addresses at once
- Individual messages are small and self-contained (DNS queries are UDP)

DNS is a good example: a DNS query is a single small packet. If it gets lost, the client just asks again after a short timeout. A full TCP handshake before every name lookup would be wasteful.

---

## UDP in Rust

There is no `connect` step in UDP. You create a `UdpSocket`, bind it to a local address, and immediately send or receive:

```rust
use std::net::UdpSocket;

// Sender
let socket = UdpSocket::bind("127.0.0.1:0")?;  // 0 = pick any free port
socket.send_to(b"hello", "127.0.0.1:9090")?;

// Receiver
let socket = UdpSocket::bind("127.0.0.1:9090")?;
let mut buf = [0u8; 1024];
let (n, from) = socket.recv_from(&mut buf)?;
println!("received {} bytes from {}: {:?}", n, from, &buf[..n]);
```

`bind("127.0.0.1:0")` asks the OS to assign any free port — useful for clients that do not need a known port.

Unlike TCP, `recv_from` returns *one complete datagram* per call, along with the sender's address. If you receive 100 bytes in one send, you get all 100 bytes in one `recv_from` — not a partial read like you might get with TCP streams.

---

## The catch: datagram size

UDP datagrams travel inside IP packets. IP packets travel inside Ethernet frames. An Ethernet frame can carry at most 1500 bytes (the MTU from chapter 2). After IP and UDP headers, you have about **1472 bytes** of usable payload per datagram.

Technically, IP can fragment larger datagrams across multiple packets — but this is slow and unreliable, and you should avoid it. Keep UDP payloads under 1472 bytes, or better, under 512 bytes to leave room for tunnels and VPNs.

If you need to send larger data over UDP — as game engines sometimes do — you implement your own fragmentation and reassembly. This is complex, which is why most applications that need large reliable messages use TCP.

---

## Terminal — `nc -u`

Netcat supports UDP with the `-u` flag:

```sh
# receiver
nc -u -l 9090

# sender (in another terminal)
nc -u 127.0.0.1 9090
```

Type in the sender window. Each line arrives as a datagram. Notice that unlike TCP, there is no connection — you can terminate and restart the sender and the receiver keeps listening. There is also no indication if a datagram was lost.

---

## What to carry forward

- UDP is **connectionless, unordered, unreliable** — it adds port numbers to IP and nothing else
- Each send is a **discrete datagram**; boundaries are preserved on receive
- Choose UDP for **low latency**, **small self-contained messages**, or when **you handle loss yourself**
- DNS uses UDP; voice/video streaming uses UDP; online games often use UDP
- Keep datagrams under ~1472 bytes to avoid IP fragmentation
- In Rust: `UdpSocket::bind`, `send_to`, `recv_from`

The next chapter covers **HTTP** — the application-layer protocol that powers the web, and the one you will use to call real APIs.
