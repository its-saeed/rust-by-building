# Lesson 2 — The Server

The server has one job: receive a `DrawEvent` from any client, then send it to every other client. It never stores the canvas, never validates moves, never interprets the drawing — it is a pure relay.

It also opens a macroquad window so you can see who is connected and watch packets flow in real time. Both server and client share the same overall frame shape: drain incoming packets, update state, render, repeat.

---

## Step 1 — Binding the socket

```rust
let socket = UdpSocket::bind("0.0.0.0:9090").expect("failed to bind :9090");
```

`0.0.0.0` means "listen on every network interface" — loopback (`127.0.0.1`), Wi-Fi, Ethernet, all at once. Port `9090` is the address clients will send to. If you wanted only local connections you would bind `127.0.0.1:9090` instead, but then a client on another machine could not reach the server.

Clients, by contrast, bind to `0.0.0.0:0`. The `0` port tells the OS to pick any free port — clients do not need a fixed, known port because nobody dials them; they dial the server.

---

## Step 2 — Non-blocking mode

```rust
socket.set_nonblocking(true).expect("set_nonblocking failed");
```

By default, `recv_from` blocks: your thread freezes and waits until a packet arrives. That would be fine for a dedicated server process sitting in an infinite loop, but this server runs inside macroquad's frame loop. If no packet arrives for 50 ms, the macroquad window would hang — no redraws, no responses to the OS.

`set_nonblocking(true)` changes `recv_from` so that it returns immediately with `Err(WouldBlock)` when nothing is waiting. The frame loop keeps running, the window keeps drawing, and packets are picked up whenever they happen to arrive.

---

## Step 3 — Tracking peers

```rust
let mut peers: HashMap<SocketAddr, Instant> = HashMap::new();
```

That is the entire server state. `SocketAddr` is the combined IP address and port of a remote endpoint — it uniquely identifies a connected client. `Instant` records the last time we heard from that client.

There is no handshake. No "hello" message. No registration. The moment a packet arrives from a new address, that address becomes a peer. The moment a peer goes silent for five seconds, it is removed. This is called **implicit registration** — clients register themselves by sending, not by asking permission.

Why `Instant` and not a counter or a flag? Because we need to know *how long ago* we last heard from a peer, not just *whether* we heard from them. `Instant::now().duration_since(last_seen)` gives us elapsed time in any unit.

---

## Step 4 — Receiving a packet

Inside the game loop, at the top of every frame:

```rust
let now = Instant::now();

loop {
    match socket.recv_from(&mut buf) {
        Ok((n, from)) => {
            // a packet arrived: n bytes from address `from`
        }
        Err(e) if e.kind() == WouldBlock => break, // nothing waiting — move on
        Err(_) => break,                            // real error — ignore for now
    }
}
```

This is the drain loop. It keeps calling `recv_from` until nothing is left. Because `set_nonblocking(true)` is set, each call either returns a packet immediately or returns `WouldBlock`. The loop empties the OS receive buffer in one burst and then stops — it never waits.

One subtlety: multiple packets can arrive between frames. At 60 fps, one frame lasts ~16 ms. If 10 clients are drawing simultaneously, dozens of packets could queue up during that window. The drain loop handles all of them before rendering — no packet is ever left sitting in the buffer until the next frame.

---

## Step 5 — Validating and registering

Not every UDP packet is a valid `DrawEvent`. Any program on the network can send bytes to port 9090. Before doing anything with a packet, verify it:

```rust
Ok((n, from)) => {
    if DrawEvent::from_bytes(&buf[..n]).is_some() {
        peers.insert(from, now);
        // relay...
    }
    // invalid packets are silently dropped
}
```

`from_bytes` returns `None` if the buffer is too short or the data does not make sense. Only on `Some(_)` do we update the peer map and relay. This is the entire security model — minimal, but enough for a classroom demo on a local network.

`peers.insert(from, now)` does two things at once: if `from` is a new address, it adds it; if `from` is already known, it updates the timestamp. This is why `Instant` is the value — every valid packet refreshes the peer's "last seen" time.

---

## Step 6 — Relaying

Now the interesting part: send the packet to everyone except the sender.

```rust
for (&addr, _) in &peers {
    if addr != from {
        let _ = socket.send_to(&buf[..n], addr);
    }
}
```

Why not echo back to `from`? Because the client that sent this packet already drew the stroke locally — it does not need a copy of its own event. Echoing back would cause double-drawing.

`let _ = socket.send_to(...)` discards the `Result`. UDP sends can fail silently (the peer's buffer is full, the OS is busy) and there is nothing useful to do about it in a relay — if a frame is dropped, the drawing is slightly incomplete on that client, and life goes on.

One thing to notice: we relay the raw bytes `&buf[..n]`, not a re-serialised `DrawEvent`. The server never needs to deserialise the event fully — `from_bytes` is called only to validate. The bytes travel through unchanged, which means the relay adds zero transformation overhead.

---

## Step 7 — Pruning inactive peers

After the drain loop, before rendering:

```rust
peers.retain(|_, last_seen| {
    now.duration_since(*last_seen).as_secs() < 5
});
```

`HashMap::retain` removes every entry where the closure returns `false`. Any peer whose last packet was more than five seconds ago is considered disconnected — its client crashed, closed the window, or lost the network.

Why does this matter? If you never prune, dead peers accumulate. The relay loop tries to `send_to` each known peer every frame. Sending to a dead address is harmless (UDP does not error on unreachable destinations) but wasteful, and on a long-running server it would grow without bound.

Five seconds is a generous threshold — a client sends 60 events per second while drawing, so even a client sitting idle and not drawing would need to send a heartbeat to stay registered. For this project, we skip heartbeats: a client that stops drawing for 5 seconds disappears from the server's peer list, and reappears the moment it draws again.

---

## Step 8 — The macroquad window

The render section turns the server from a headless relay into a visible dashboard. It runs every frame, after the drain loop and pruning:

```rust
clear_background(Color::from_rgba(18, 18, 28, 255));

draw_text("Tele-Sketch  Server", 40.0, 55.0, 36.0, WHITE);
draw_line(40.0, 68.0, screen_width() - 40.0, 68.0, 1.0, DARKGRAY);
```

A dark background, a title, a separator line. Nothing unusual — same macroquad calls you used in previous projects.

The peer count changes colour based on whether anyone is connected:

```rust
let peer_color = if peers.is_empty() { GRAY } else { GREEN };
draw_text(
    &format!("peers connected: {}", peers.len()),
    40.0, 110.0, 28.0, peer_color,
);
```

Gray means waiting. Green means live. Students watching the server can tell at a glance when their clients connect.

List each peer's address, and briefly highlight the most recently active one in yellow:

```rust
for (i, (addr, _)) in peers.iter().enumerate() {
    let color = if last_active == Some(*addr) { YELLOW } else { LIGHTGRAY };
    draw_text(
        &format!("  ●  {addr}"),
        40.0,
        185.0 + i as f32 * 30.0,
        20.0,
        color,
    );
}
```

`last_active` is set to `Some(from)` each time a valid packet arrives. Because it is updated inside the drain loop (which processes all queued packets in one burst), the last packet processed wins the yellow highlight for that frame.

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

    let mut peers:       HashMap<SocketAddr, Instant> = HashMap::new();
    let mut buf          = [0u8; 64];
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
        draw_text(&format!("peers connected: {}", peers.len()), 40.0, 110.0, 28.0, peer_color);
        draw_text(&format!("packets relayed: {relayed}"), 40.0, 142.0, 20.0, DARKGRAY);

        for (i, (addr, _)) in peers.iter().enumerate() {
            let color = if last_active == Some(*addr) { YELLOW } else { LIGHTGRAY };
            draw_text(&format!("  ●  {addr}"), 40.0, 185.0 + i as f32 * 30.0, 20.0, color);
        }

        next_frame().await;
    }
}
```

Run it: `cargo run --bin server`. A dark window opens showing "peers connected: 0" in gray. Keep it running while you work on the next lesson — when clients connect, you will see the count turn green and their addresses appear.

---

## Exercise

> **TODO 1**: `last_active` currently flips to the most recent sender inside the drain loop, so the yellow highlight only lasts one frame (by the next drain loop it has moved on). Change `last_active` to `Option<(SocketAddr, Instant)>` and highlight any peer whose last packet arrived within the past 200 ms, using `now.duration_since(instant).as_millis() < 200`.
>
> **TODO 2**: Add a packets-per-second counter. Introduce `let mut pps_count: u64 = 0` and `let mut pps_timer = Instant::now()`. Increment `pps_count` each relay. Every time `pps_timer.elapsed().as_secs() >= 1`, save the count to a `pps_display` variable and reset both. Draw `pps_display` on screen.
>
> **TODO 3**: What happens if three clients are connected and one of them crashes mid-drawing? Trace through the pruning logic: how many frames pass before the dead peer is removed? What do the other two clients experience in the meantime?

---

## Key APIs

| API | What it does |
|-----|-------------|
| `UdpSocket::bind("0.0.0.0:9090")` | Listen on all interfaces, port 9090 |
| `socket.set_nonblocking(true)` | `recv_from` returns immediately if no data |
| `socket.recv_from(&mut buf)` | Receive one datagram; returns `(n_bytes, sender_addr)` |
| `socket.send_to(&buf[..n], addr)` | Send `n` bytes to a specific address |
| `peers.insert(addr, instant)` | Add or update a peer's last-seen time |
| `HashMap::retain(\|k, v\| ...)` | Remove all entries where the closure returns `false` |
| `now.duration_since(t).as_secs()` | Seconds elapsed since a past `Instant` |
