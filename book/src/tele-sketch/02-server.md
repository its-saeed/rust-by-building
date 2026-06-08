# Lesson 2 — The Server

The server has one job: receive a `DrawEvent` from any client, then send it to every other client. It never stores the canvas, never validates moves, never interprets the drawing — it is a pure relay.

It also opens a macroquad window so you can see who is connected and watch packets flow.

---

## What the server tracks

```rust
let mut peers: HashMap<SocketAddr, Instant> = HashMap::new();
```

That is the entire server state. When a packet arrives from an address, that address is a peer. If a peer goes silent for five seconds, remove it. No handshake, no registration message — a client becomes known the moment it sends its first byte.

---

## The relay loop

```rust
loop {
    match socket.recv_from(&mut buf) {
        Ok((n, from)) => {
            if DrawEvent::from_bytes(&buf[..n]).is_some() {
                peers.insert(from, Instant::now());
                for (&addr, _) in &peers {
                    if addr != from {
                        let _ = socket.send_to(&buf[..n], addr);
                    }
                }
            }
        }
        Err(e) if e.kind() == WouldBlock => break,
        Err(_) => break,
    }
}
```

The server validates the packet (`from_bytes` must succeed) before registering the sender and relaying. Invalid packets are silently dropped. The server never echoes back to the sender — the client draws locally immediately, so it does not need confirmation.

---

## Non-blocking inside a game loop

The server's socket is set to non-blocking for the same reason as the client: `recv_from` with blocking would freeze the macroquad frame until a packet arrived. With `set_nonblocking(true)`, `recv_from` returns immediately with `WouldBlock` if nothing is waiting.

```rust
socket.set_nonblocking(true).expect("set_nonblocking failed");
```

The drain loop runs every frame, processing however many packets arrived since last frame (zero, one, or many), then moves on to rendering.

---

## Pruning inactive peers

```rust
peers.retain(|_, last_seen| {
    now.duration_since(*last_seen).as_secs() < 5
});
```

`HashMap::retain` removes all entries where the closure returns false. A peer that has not sent in five seconds is considered gone — its client crashed, closed, or went offline. Pruning matters because the server tries to relay to every known peer; dead peers would accumulate indefinitely otherwise.

---

## Full server

```rust
use macroquad::prelude::*;
use std::collections::HashMap;
use std::io::ErrorKind::WouldBlock;
use std::net::{SocketAddr, UdpSocket};
use std::time::Instant;
use tele_sketch::event::DrawEvent;

#[macroquad::main("Tele-Sketch Server")]
async fn main() {
    let socket = UdpSocket::bind("0.0.0.0:9090").expect("failed to bind :9090");
    socket.set_nonblocking(true).expect("set_nonblocking failed");

    let mut peers: HashMap<SocketAddr, Instant> = HashMap::new();
    let mut buf   = [0u8; 64];
    let mut relayed: u64 = 0;
    let mut last_active: Option<SocketAddr> = None;

    loop {
        let now = Instant::now();

        // drain incoming packets
        loop {
            match socket.recv_from(&mut buf) {
                Ok((n, from)) => {
                    if DrawEvent::from_bytes(&buf[..n]).is_some() {
                        peers.insert(from, now);
                        last_active = Some(from);
                        for (&addr, _) in &peers {
                            if addr != from {
                                let _ = socket.send_to(&buf[..n], addr);
                                relayed += 1;
                            }
                        }
                    }
                }
                Err(e) if e.kind() == WouldBlock => break,
                Err(_) => break,
            }
        }

        // prune peers silent for >5 s
        peers.retain(|_, t| now.duration_since(*t).as_secs() < 5);

        // render
        clear_background(Color::from_rgba(18, 18, 28, 255));

        draw_text("Tele-Sketch  Server", 40.0, 55.0, 36.0, WHITE);
        draw_line(40.0, 68.0, screen_width() - 40.0, 68.0, 1.0, DARKGRAY);

        let peer_color = if peers.is_empty() { GRAY } else { GREEN };
        draw_text(
            &format!("peers connected: {}", peers.len()),
            40.0, 110.0, 28.0, peer_color,
        );
        draw_text(
            &format!("packets relayed: {relayed}"),
            40.0, 142.0, 20.0, DARKGRAY,
        );

        for (i, (addr, _)) in peers.iter().enumerate() {
            let highlight = last_active == Some(*addr);
            let color = if highlight { YELLOW } else { LIGHTGRAY };
            draw_text(&format!("  ●  {addr}"), 40.0, 185.0 + i as f32 * 30.0, 20.0, color);
        }

        next_frame().await;
    }
}
```

Run it: `cargo run --bin server`. A window opens. Nothing interesting happens yet — there are no clients. Keep this running while you build the client.

---

## Exercise

> **TODO 1**: The server currently highlights the most recently active peer in yellow but only keeps the address for one frame. Change `last_active` to store `(SocketAddr, Instant)` and highlight for 200 ms after the last packet from that peer.
>
> **TODO 2**: Add a packets-per-second display. Track `packets_this_second: u64` and reset it every second using `Instant::elapsed()`.

---

## Key APIs

| API | What it does |
|-----|-------------|
| `UdpSocket::bind("0.0.0.0:9090")` | Listen on all interfaces, port 9090 |
| `socket.set_nonblocking(true)` | `recv_from` returns immediately if no data |
| `socket.recv_from(&mut buf)` | Receive one datagram; returns (bytes, sender addr) |
| `socket.send_to(&buf, addr)` | Send datagram to specific address |
| `HashMap::retain(\|k, v\| ...)` | Remove entries where closure returns false |
| `Instant::now()` / `.duration_since()` | Measure elapsed time |
