# Chapter 7 — Sockets

Every network connection your programs make — TCP or UDP, client or server — goes through a **socket**. A socket is an endpoint for communication: a combination of an IP address and a port number, managed by the operating system.

Sockets are the bridge between your code and the network. Everything in the networking chapters so far — TCP streams, UDP datagrams, HTTP — is built on sockets.

---

## What a socket is

When your program opens a network connection, it asks the OS to create a socket. The OS assigns a **file descriptor** (a number) that represents this connection. From that point on, your program sends and receives data through the file descriptor, exactly like reading and writing a file.

This is not an accident. On Unix systems, "everything is a file" — network connections, regular files, pipes, and devices all speak the same `read`/`write` language. In Rust, this is why `TcpStream` implements `Read` and `Write`: it is a file descriptor under the hood.

---

## Types of sockets

| Type | Protocol | What you send/receive |
|------|---------|----------------------|
| Stream socket | TCP | Continuous byte stream |
| Datagram socket | UDP | Discrete packets with preserved boundaries |

Rust's standard library gives you:

- `TcpListener` — stream socket, server side (waits for connections)
- `TcpStream` — stream socket, client side (one connection)
- `UdpSocket` — datagram socket (send to / receive from any address)

---

## The socket lifecycle

### TCP server

```
TcpListener::bind(addr)      ← create socket, bind to address, start listening
    │
    ▼
listener.accept()            ← block until a client connects
    │                           returns TcpStream + client address
    ▼
stream.read(&mut buf)        ← receive data
stream.write_all(&data)      ← send data
    │
    ▼
drop(stream)                 ← close connection (sends FIN)
```

### TCP client

```
TcpStream::connect(addr)     ← create socket, connect to server (3-way handshake)
    │
    ▼
stream.write_all(&data)      ← send data
stream.read(&mut buf)        ← receive response
    │
    ▼
drop(stream)                 ← close connection
```

### UDP

```
UdpSocket::bind(addr)        ← create socket, bind to local address
    │
    ▼
socket.send_to(&data, addr)  ← send datagram to destination
socket.recv_from(&mut buf)   ← receive datagram (blocks until one arrives)
```

---

## Blocking by default

By default, socket operations in Rust's `std::net` are **blocking**:

- `TcpListener::accept()` blocks until a client connects
- `TcpStream::read()` blocks until data arrives
- `UdpSocket::recv_from()` blocks until a datagram arrives

This means your program pauses at these calls and waits. For simple programs — including everything in this section — that is fine. Your server handles one request at a time, finishes, then waits for the next one.

For servers that need to handle many connections simultaneously, blocking is a problem: while you are serving one client, all others wait. The solutions — threads, async/await — are beyond the scope of this course. But single-threaded, sequential servers are completely legitimate for low-traffic services, development tools, and learning.

---

## Binding to a port

When you call `TcpListener::bind("127.0.0.1:7878")`, the OS:

1. Creates a socket
2. **Binds** it to port 7878 on the loopback interface
3. Puts it in the **listen** state — ready to accept connections

If port 7878 is already in use by another program, `bind` returns an error. This is the "address already in use" error you will see when you try to restart a server too quickly (the OS may hold the port briefly after it closes). You can see listening ports with `netstat -an` or `ss -tn`.

---

## `127.0.0.1` vs `0.0.0.0`

When you bind a server:

- `127.0.0.1:PORT` — only accepts connections from the **same machine**. Nothing from outside can reach it. Safe for development.
- `0.0.0.0:PORT` — accepts connections on **all network interfaces** — loopback, Wi-Fi, Ethernet. Other machines on your network can connect.

For the lessons in this book, use `127.0.0.1`. When you deploy a real server, you will typically use `0.0.0.0` (or let a framework handle it).

---

## What to carry forward

- A **socket** is an OS-managed endpoint: an IP address + port number, accessed via a file descriptor
- `TcpListener` binds and accepts; `TcpStream` sends and receives; `UdpSocket` does both without a connection
- Socket operations are **blocking** by default — your program pauses until data arrives
- Bind to `127.0.0.1` for local-only servers; `0.0.0.0` to accept from any interface
- The OS rejects `bind` if the port is already in use

---

That is the conceptual foundation. You now know what a packet is, how layers cooperate, what addresses and ports mean, how TCP builds reliability, when to use UDP instead, what HTTP looks like, and how sockets connect your code to all of it.

The next section puts this into practice: you will write real Rust programs that open sockets, speak protocols, and exchange data.
