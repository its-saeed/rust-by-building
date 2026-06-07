# Chapter 3 — Addresses and Names

Every letter you post needs an address. Network packets are no different. The internet layer uses **IP addresses** to route packets to the right machine, and the transport layer uses **port numbers** to route them to the right program. And because numeric addresses are hard to remember, the internet has a phone book: **DNS**.

> **Think of it like this:** An IP address is like a building's street address — it gets mail to the right building. A port number is like an apartment number — it gets the mail to the right door inside that building. The full address `192.168.1.12:8080` reads as "building 192.168.1.12, apartment 8080." DNS is your phone's contacts app: instead of remembering your friend's number (+1-555-0193), you just search "Alice." Instead of remembering `142.250.185.14`, you just type `google.com`.

---

## IP addresses

An **IP address** is a number assigned to a machine on a network. Routers use these numbers to decide where to forward packets.

The version you will encounter most often is **IPv4**, which represents an address as four numbers separated by dots:

```
192.168.1.12
```

Each number is one byte: a value from 0 to 255. So an IPv4 address is 32 bits — roughly four billion possible addresses. That seemed like plenty in the 1980s.

The newer **IPv6** uses 128 bits, written as eight groups of four hexadecimal digits separated by colons:

```
2001:0db8:85a3:0000:0000:8a2e:0370:7334
```

You will mostly see IPv4 in local examples and a mix of both on the public internet. Rust's `std::net` handles both transparently.

---

## Special addresses

A few IP addresses have fixed meanings you will use constantly:

| Address | Meaning |
|---------|---------|
| `127.0.0.1` | Loopback — routes to the same machine. Never leaves your computer. |
| `localhost` | The hostname that resolves to `127.0.0.1`. |
| `0.0.0.0` | "All interfaces" — when a server binds to this, it listens on every network interface on the machine. |
| `192.168.x.x` / `10.x.x.x` | Private addresses — used inside home and office networks, not routable on the public internet. |

When you write a server in the upcoming lessons, you will bind to `127.0.0.1:PORT` for local testing. The `127.0.0.1` means "only accept connections from this machine" — safe while developing.

---

## Port numbers

An IP address gets a packet to a machine. But that machine is running many programs at once. **Port numbers** identify which program should receive the packet.

A port is a 16-bit number: 0 to 65535. Every TCP or UDP connection has both a source port and a destination port.

Some ports are assigned to specific protocols by convention:

| Port | Protocol |
|------|---------|
| 80   | HTTP |
| 443  | HTTPS |
| 22   | SSH |
| 53   | DNS |

When you type `http://example.com` in a browser, it connects to port 80 on that server. When you run an SSH server, it listens on port 22.

When *you* write a server, you choose the port. A common choice for development is something in the 8000–9000 range (8080, 8000, 9000) — high enough to avoid conflicts with system services, easy to remember.

Ports below 1024 are **privileged** on most systems: only root / Administrator can bind to them. Your development servers will use higher ports.

---

## A full address: `host:port`

In Rust code and most tools, you write addresses as `host:port`:

```
127.0.0.1:8080
google.com:443
192.168.1.12:3000
```

This is the pair that uniquely identifies a network endpoint: which machine, which program.

---

## DNS — the internet's phone book

Remembering `142.250.185.14` is hard. Remembering `google.com` is easy. **DNS** (Domain Name System) translates names to IP addresses.

When you type `google.com` into a browser:

1. Your OS asks a **DNS resolver** (usually your router) "what is the IP address for google.com?"
2. The resolver asks the **root name servers** — authoritative servers that know who handles each top-level domain (`.com`, `.org`, `.io`)
3. The root server says "ask the `.com` name server"
4. The `.com` name server says "ask Google's name servers"
5. Google's name server returns an IP address
6. The resolver caches the answer and returns it to your OS
7. Your browser connects to that IP address

The answer is cached at each step with a **TTL** (time to live, in seconds), so most lookups are fast — they hit a nearby cache rather than walking the whole chain.

---

## Terminal — `dig`

`dig` is a DNS lookup tool. It shows you exactly what name resolution returns.

```sh
dig google.com
```

Look for the **ANSWER SECTION**:

```
;; ANSWER SECTION:
google.com.		60	IN	A	142.250.185.14
```

- `google.com.` — the name you asked about (note the trailing dot — the full form of every DNS name ends in `.`, representing the root)
- `60` — the TTL in seconds; the answer can be cached for 60 seconds
- `IN A` — an IPv4 address record (IN = internet class, A = address)
- `142.250.185.14` — the IP address

Run it a few times. The TTL counts down between queries. When it reaches zero, the next query goes back to the DNS server for a fresh answer.

---

## Terminal — `dig +trace`

To see the full delegation chain from root servers down to the answer:

```sh
dig +trace google.com
```

This makes `dig` walk the resolution chain itself rather than using your local resolver. You will see:

1. Queries to the root servers (`.`) returning the `.com` name servers
2. Queries to `.com` name servers returning Google's name servers
3. Queries to Google's name servers returning the IP address

It is slow — many round trips — but it makes the DNS system visible rather than invisible.

---

## Terminal — `ifconfig` / `ip addr`

To see your machine's own IP addresses:

```sh
# macOS
ifconfig

# Linux
ip addr
```

Look for your main network interface (`en0` on Mac, `eth0` or `enp3s0` on Linux). Under it you will see your IP address (`inet 192.168.x.x`) — this is your address on the local network.

Also look for `lo` or `lo0` (loopback): `inet 127.0.0.1`. This is always present on every machine.

---

## What to carry forward

- An **IP address** identifies a machine; a **port number** identifies a program on that machine
- `127.0.0.1` (loopback) routes traffic back to the same machine — essential for local testing
- `0.0.0.0` means "listen on all interfaces"
- Servers bind to `host:port`; ports below 1024 require root privileges
- **DNS** translates names like `google.com` to IP addresses; results are cached with a TTL
- `dig google.com` shows the DNS answer; `dig +trace google.com` walks the full delegation chain

The next chapter covers **TCP** — how the transport layer builds a reliable, ordered connection on top of the unreliable internet layer.
