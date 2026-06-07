# Chapter 2 — The Layered Model

In chapter 1, a packet traveled from your laptop to a server in another country, hopping through a dozen routers along the way. But your laptop does not know anything about those routers. And the routers do not know anything about the webpage you were loading. Each piece of the journey is handled by a different layer of software, and each layer only does its own job.

This is the same reason your Rust projects use modules: separate concerns, hide complexity, let each part be understood and changed independently. Networks are built on exactly the same principle.

> **Think of it like this:** Imagine international shipping. You hand a parcel to your local courier — they don't care what's inside. The courier hands it to the national postal service — they don't care who the local courier was. The national post handles customs — customs doesn't care about postal routes. Finally, a courier in the destination country delivers it — they don't know it crossed an ocean. Each organisation only talks to the one above and below it. If the airline changes, the local courier doesn't notice. If you switch from FedEx to DHL domestically, customs doesn't care. That independence is exactly what the network layer model gives you.

---

## The TCP/IP model

There are two common ways to describe network layers. The **OSI model** has seven layers and is used in textbooks and certifications. The **TCP/IP model** has four layers and is what the internet actually uses. We will use TCP/IP — it maps directly to what you will program.

```
┌─────────────────────────┐
│   Application Layer     │  HTTP, DNS, SSH, your protocol
├─────────────────────────┤
│   Transport Layer       │  TCP, UDP          ← your Rust code lives here
├─────────────────────────┤
│   Internet Layer        │  IP, routing
├─────────────────────────┤
│   Link Layer            │  Ethernet, Wi-Fi, cables
└─────────────────────────┘
```

Each layer talks only to the layer directly above and below it. The application layer has no idea whether it is running over Wi-Fi or Ethernet. The link layer has no idea whether it is carrying HTTP or SSH.

---

## What each layer does

### Link Layer — the physical journey

The link layer moves bits between two machines that are directly connected — your laptop to your router, your router to the next router along the path. It deals with MAC addresses, Ethernet frames, Wi-Fi signals, and fibre optic pulses.

As a programmer, you never touch this layer. Your operating system handles it. The only reason to know it exists is so you understand why your code works the same whether you are on Wi-Fi or plugged into Ethernet — the layers above the link layer do not care.

### Internet Layer — routing across the world

The internet layer moves packets between machines that are *not* directly connected. It uses **IP addresses** to identify machines and makes the routing decisions: given a packet addressed to `142.250.185.14`, which router should I forward this to?

IP is a best-effort, unreliable protocol. It delivers packets if it can, but makes no promises about whether they arrive, in what order, or even once. Reliability is someone else's problem — the transport layer's problem.

This is the layer `traceroute` revealed. Each hop you saw in chapter 1 is the internet layer at work: a router reading the IP destination, deciding the next hop, forwarding the packet.

### Transport Layer — from machine to program

IP gets a packet to the right machine. But a machine runs dozens of programs simultaneously — a web browser, a terminal, a chat app, a game. The transport layer gets the packet to the right *program* on that machine. It does this with **port numbers**: every network connection has a source port and a destination port.

Two protocols live here:

- **TCP** — reliable, ordered delivery. Guarantees that data arrives, that it arrives in order, and that lost packets are retransmitted. Covered in depth in chapter 4.
- **UDP** — fast, unreliable delivery. No guarantees. Covered in chapter 5.

**This is the layer where your Rust code lives.** `TcpStream`, `TcpListener`, `UdpSocket` — all transport layer. When you write a TCP server in Rust, you are programming the transport layer.

### Application Layer — your protocol

The application layer is what programs actually say to each other. HTTP is an application layer protocol: "GET /index.html HTTP/1.1\r\nHost: example.com\r\n\r\n". DNS is an application layer protocol. SSH is too.

When you design your own message format — how a game server tells clients where the ball is, or how your weather program asks for forecast data — you are designing an application layer protocol, even if you never call it that.

---

## Envelopes inside envelopes

When your computer sends data, each layer wraps the data in its own envelope before handing it down:

```
Your data:  "Hello!"

Transport adds ports:    [ TCP: src=54321 dst=80 | "Hello!" ]

Internet adds addresses: [ IP: src=1.2.3.4 dst=8.8.8.8 | TCP header | "Hello!" ]

Link adds MAC addresses: [ Ethernet: src=AA:BB dst=CC:DD | IP header | TCP header | "Hello!" ]
```

This wrapping is called **encapsulation**. The router at each hop receives the outermost envelope (Ethernet frame), strips it, reads the IP address inside, wraps it in a new Ethernet frame addressed to the next hop, and sends it on. The router never looks inside the TCP header. The TCP layer on the receiving machine never looks at the IP addresses. Each layer peels exactly its own envelope and passes the rest up.

On the receiving end, the process reverses: the link layer strips the Ethernet frame, the internet layer strips the IP header, the transport layer strips the TCP header, and your application gets just `"Hello!"`.

---

## Why this matters for you

The layered model has one practical consequence that shapes how you write networked programs:

**You describe what you want, not how to deliver it.** When you call `TcpStream::connect("google.com:80")`, you are not thinking about MAC addresses, routing tables, or which physical cable carries the signal. The OS handles all of that. You work at the transport layer and trust that the layers below do their job.

The same program works over Wi-Fi, Ethernet, a mobile hotspot, or a VPN — because the layers below the transport layer handle the translation. Your code does not change.

---

## Terminal — `netstat -i`

`netstat -i` shows your network **interfaces** — the link layer devices on your machine:

```sh
netstat -i
```

```
Name    Mtu   Network         Address            Ipkts Ierrs    Opkts Oerrs  Coll
lo0     16384 127.0.0.1       127.0.0.1         125043     0   125043     0     0
en0     1500  192.168.1.0/24  192.168.1.12      984201     0   312847     0     0
```

`lo0` is the **loopback** interface — a virtual network device that loops traffic back to the same machine. When you connect to `127.0.0.1`, the packet never leaves your computer; the OS routes it straight back. This is how you will test your servers in the upcoming lessons — client and server on the same machine, communicating through loopback.

`en0` is your physical network interface (Ethernet port or Wi-Fi). `Mtu` is the maximum transmission unit — the largest packet this interface can carry (1500 bytes is the Ethernet standard).

---

## Terminal — `traceroute`, revisited

Run traceroute again with the layered model in mind:

```sh
traceroute google.com
```

Each hop is the **internet layer** working. The routers are reading IP addresses (internet layer) and making forwarding decisions. They are completely unaware of TCP, HTTP, or anything you are doing — that is handled end-to-end between your machine and the server.

The latency you see accumulating with each hop is IP routing at work: packet in, lookup, packet out.

---

## What to carry forward

- Networks are organised in **four layers**: link, internet, transport, application
- Each layer only talks to the layers immediately above and below it
- Data is **encapsulated** — each layer wraps the data in its own header going down, unwraps it going up
- **Transport layer** (TCP/UDP) is where your Rust networking code lives
- **IP addresses** identify machines (internet layer); **port numbers** identify programs (transport layer)
- `lo0` / loopback (`127.0.0.1`) lets client and server run on the same machine — essential for local testing

The next chapter digs into addresses: what IP addresses mean, what port numbers are, and how the name `google.com` becomes `142.250.185.14`.
