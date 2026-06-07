# Chapter 1 — What is a Network?

At its most basic, a network is two or more computers that can send data to each other. That is it. Everything else — the internet, HTTP, DNS, sockets, Rust's `TcpStream` — is built on top of that single idea.

---

## Data travels in packets

When one computer sends data to another, it does not flow as a continuous stream the way water flows through a pipe. It travels as **packets**: small, fixed-size chunks of data. Each packet carries:

- the data itself (a fragment of whatever is being sent)
- where it came from (source address)
- where it is going (destination address)
- its position in the sequence, so the receiver can reassemble the original message

A single image or webpage is split into many packets, sent across the network independently, and put back together at the other end. One image might arrive as 50 separate packets, some taking slightly different paths, arriving slightly out of order — the destination sorts them out.

**Why not send it all at once?** Because a network is shared. If one computer could hold the wire while sending a large file, everyone else would have to wait. Packets let many conversations happen simultaneously — each gets brief, interleaved access to the shared connection.

---

## Routers: the post offices of the internet

When you connect more than two computers, you need something to direct traffic. That is a **router**. A router reads the destination address on each packet and decides which direction to forward it — like a postal sorting office reading an address and putting the letter in the right bin.

Your home Wi-Fi router sits between your devices and your ISP. Every device on your local network has a private local address (something like `192.168.1.5`). When a packet leaves your home for the internet, your router replaces that private address with your public IP address — the one the rest of the internet uses to find you.

---

## The internet: a network of networks

The internet is not one network. It is tens of thousands of networks — home ISPs, universities, cloud providers, mobile carriers, backbone operators — all agreeing to talk to each other using the same rules. When a packet travels from your laptop to a server in another country, it passes through many of these networks, handed from router to router at each step.

Each handoff is called a **hop**. A typical request to a server a continent away passes through 10–20 hops.

---

## Latency: why it takes time

Even at the speed of light, distance costs time. A round trip from Europe to the US east coast and back is roughly 12,000 km — light covers that in about 40 ms. Real network cables are slower than light in a vacuum, and every router along the way adds a small processing delay. The total round-trip time from your machine to a server and back is called **latency**, measured in milliseconds.

Latency matters differently for different applications:

- **Real-time games and video calls** — even 100 ms feels sluggish; 20–50 ms is comfortable
- **Web browsing** — a few hundred milliseconds is acceptable
- **Downloading a large file** — latency barely matters; what matters is bandwidth (how much data per second fits through the pipe)

Bandwidth and latency are independent. A satellite connection can have high bandwidth but terrible latency (the signal travels to space and back). A thin fibre link can have low latency but limited bandwidth.

---

## Terminal — `ping`

`ping` sends a small packet to a server and times how long it takes to receive a reply. Try it:

```sh
ping google.com
```

You will see output like this:

```
PING google.com (142.250.185.14): 56 data bytes
64 bytes from 142.250.185.14: icmp_seq=0 ttl=117 time=13.8 ms
64 bytes from 142.250.185.14: icmp_seq=1 ttl=117 time=14.1 ms
64 bytes from 142.250.185.14: icmp_seq=2 ttl=117 time=13.9 ms
```

Two things worth noticing:

**`time=13.8 ms`** — that is the round-trip latency. 13.8 milliseconds to reach Google and come back.

**`142.250.185.14`** — before the first packet was sent, your computer converted the name `google.com` into this IP address. You will learn exactly how that works in Chapter 3.

Press `Ctrl+C` to stop. Now try pinging a server far away — a Japanese or Australian host if you are in Europe, for instance. Watch the latency climb.

```sh
ping -c 5 google.com      # send exactly 5 packets, then stop (-c = count)
```

---

## Terminal — `traceroute`

`ping` shows the total round-trip. `traceroute` reveals every hop along the way:

```sh
traceroute google.com
```

```
traceroute to google.com (142.250.185.14), 64 hops max
 1  192.168.1.1       1.1 ms   0.8 ms   1.0 ms
 2  10.10.0.1         4.3 ms   4.1 ms   4.4 ms
 3  84.116.12.45      7.8 ms   7.6 ms   7.9 ms
 4  72.14.234.21      9.1 ms   9.0 ms   9.3 ms
 ...
13  142.250.185.14   13.8 ms  13.7 ms  13.9 ms
```

Each line is one router. Hop 1 is your home router (`192.168.1.1`). The last hop is Google's server. The three times are three separate measurements — they vary because network conditions fluctuate.

Watch the latency climb with each hop. The biggest jumps usually happen when the packet crosses an undersea cable from one continent to another.

Some hops show `* * *` — those routers are configured to ignore `traceroute` packets, usually for security or traffic management reasons. That is normal.

---

## What to carry forward

- A network is computers exchanging **packets**
- Each packet carries a source address, a destination address, and a slice of data
- **Routers** forward packets hop by hop toward the destination
- The internet is many networks connected together
- **Latency** is round-trip time; **bandwidth** is throughput — they are independent
- `ping` measures latency; `traceroute` shows the path

The next chapter explains why network software is built in layers — and why that layering is what makes it possible to write a Rust program without knowing anything about the Wi-Fi radio or fibre cable doing the actual transmission.
