# Lesson 4 — Going Live

The canvas draws locally. Now add five things: a UDP socket, a receive loop, a send call, a status indicator, and a remote-strokes list that actually fills up. By the end, two windows share a canvas.

Strokes are still white and fixed-size — palette and brush size come in lessons 5 and 6.

---

## Step 1 — Creating the socket

Before the game loop, alongside the stroke vecs:

```rust
use std::net::UdpSocket;

let socket = UdpSocket::bind("0.0.0.0:0").expect("bind failed");
socket.set_nonblocking(true).expect("set_nonblocking failed");

const SERVER: &str = "127.0.0.1:9090";
```

`bind("0.0.0.0:0")` — bind to all interfaces, any free port. The client does not need a fixed port; only the server does. The OS picks one in the ephemeral range (49152–65535) and the client uses it for the whole session.

`set_nonblocking(true)` is non-negotiable. Without it, `recv_from` blocks the thread until a packet arrives — freezing the macroquad window for however long that takes. With it, `recv_from` returns `WouldBlock` immediately when nothing is waiting.

---

## Step 2 — The receive drain loop

At the **top** of every frame, before input or rendering:

```rust
use std::io::ErrorKind;

let mut buf = [0u8; 64];

loop {
    match socket.recv_from(&mut buf) {
        Ok((n, _from)) => {
            if let Some(ev) = DrawEvent::from_bytes(&buf[..n]) {
                remote_strokes.push(ev);
            }
        }
        Err(e) if e.kind() == ErrorKind::WouldBlock => break,
        Err(_) => break,
    }
}
```

This loop empties the OS receive buffer completely in one burst. Three outcomes per call:

- `Ok((n, from))` — a packet arrived; parse it and push to `remote_strokes`
- `WouldBlock` — nothing left; break and continue the frame
- Other error — something is wrong with the socket; break rather than loop forever

Why drain before input? So that all remote strokes that arrived since the last frame are drawn in this frame, not the next. Draining at the bottom of the frame instead would add one frame of lag to every remote stroke.

`_from` is discarded — the client does not need to know the sender's address. Every packet it receives was relayed by the server, so it came from another client.

---

## Step 3 — Sending a stroke

In the mouse-input block, add one line after the push:

```rust
if is_mouse_button_down(MouseButton::Left) {
    let ev = DrawEvent {
        x: mx, y: my,
        r: 255, g: 255, b: 255,
        size: 8,
        pen_down: true,
    };
    local_strokes.push(ev);
    let _ = socket.send_to(&ev.to_bytes(), SERVER); // ← new
}
```

`to_bytes()` produces the 13-byte array defined in lesson 1. `send_to` hands it to the OS network stack — it returns in microseconds, without waiting for delivery.

`let _ =` discards the `Result`. A lost drawing stroke is not worth a crash. In production you might log errors; here silence is correct.

The local push happens **before** `send_to` and you draw locally without waiting for the server to echo back. If you waited for a round-trip before drawing, every stroke would have visible lag. Draw immediately, send concurrently.

---

## Step 4 — Connection indicator

Declare one boolean before the loop:

```rust
let mut connected = false;
```

Set it inside the receive loop on the first successful packet:

```rust
Ok((n, _from)) => {
    if let Some(ev) = DrawEvent::from_bytes(&buf[..n]) {
        connected = true;   // ← add this
        remote_strokes.push(ev);
    }
}
```

In the render section, draw a status string in the top-right corner:

```rust
let (label, color) = if connected {
    ("● LIVE", GREEN)
} else {
    ("○ waiting...", DARKGRAY)
};
draw_text(label, screen_width() - 140.0, 24.0, 20.0, color);
```

`connected` flips to `true` the first time a remote packet arrives and stays true — it is a "we have talked at least once" flag. A more robust version would use `Option<Instant>` and revert to "waiting" if no packet arrives for a few seconds (exercise below).

---

## Full client

```rust
use macroquad::prelude::*;
use std::io::ErrorKind;
use std::net::UdpSocket;
use tele_sketch::event::DrawEvent;

const SERVER: &str = "127.0.0.1:9090";

#[macroquad::main("Tele-Sketch")]
async fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("bind failed");
    socket.set_nonblocking(true).expect("set_nonblocking failed");

    let mut local_strokes:  Vec<DrawEvent> = Vec::new();
    let mut remote_strokes: Vec<DrawEvent> = Vec::new();
    let mut connected = false;
    let mut buf = [0u8; 64];

    loop {
        // ── receive ───────────────────────────────────────────────────────
        loop {
            match socket.recv_from(&mut buf) {
                Ok((n, _)) => {
                    if let Some(ev) = DrawEvent::from_bytes(&buf[..n]) {
                        connected = true;
                        remote_strokes.push(ev);
                    }
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => break,
                Err(_) => break,
            }
        }

        // ── input ─────────────────────────────────────────────────────────
        let (mx, my) = mouse_position();

        if is_mouse_button_down(MouseButton::Left) {
            let ev = DrawEvent {
                x: mx, y: my,
                r: 255, g: 255, b: 255,
                size: 8,
                pen_down: true,
            };
            local_strokes.push(ev);
            let _ = socket.send_to(&ev.to_bytes(), SERVER);
        }

        // ── render ────────────────────────────────────────────────────────
        clear_background(Color::from_rgba(30, 30, 35, 255));

        for ev in local_strokes.iter().filter(|e| e.pen_down) {
            draw_circle(ev.x, ev.y, ev.size as f32, Color::from_rgba(ev.r, ev.g, ev.b, 255));
        }
        for ev in remote_strokes.iter().filter(|e| e.pen_down) {
            draw_circle(ev.x, ev.y, ev.size as f32, Color::from_rgba(ev.r, ev.g, ev.b, 200));
        }

        let (label, color) = if connected { ("● LIVE", GREEN) } else { ("○ waiting...", DARKGRAY) };
        draw_text(label, screen_width() - 140.0, 24.0, 20.0, color);

        next_frame().await;
    }
}
```

---

## Running it

```sh
# terminal 1
cargo run --bin server

# terminal 2
cargo run --bin client

# terminal 3
cargo run --bin client
```

Draw in one client. Watch the strokes appear in the other. The server window shows peer count climb to 2 and the packet counter start moving.

---

## Exercise

> **TODO 1**: Improve the connection indicator. Replace `connected: bool` with `last_remote: Option<Instant>`. Show "● LIVE" if a packet arrived within the last 3 seconds, "○ waiting..." otherwise. What happens to the indicator if you close one client?
>
> **TODO 2**: Start the client without starting the server. Does `send_to` return an error? Does `recv_from` error? Explain why UDP clients can start before the server without crashing.
>
> **TODO 3**: What does the canvas look like when a second client joins mid-session? The new client's `local_strokes` and `remote_strokes` start empty — it sees no history. How would you fix this? (Think about what the server would need to store and send on connection.)

---

## Key APIs

| API | What it does |
|-----|-------------|
| `UdpSocket::bind("0.0.0.0:0")` | Bind to any interface, any free port |
| `socket.set_nonblocking(true)` | `recv_from` returns immediately if nothing waiting |
| `socket.send_to(&bytes, addr)` | Fire a UDP datagram — returns before delivery |
| `socket.recv_from(&mut buf)` | One datagram or `Err(WouldBlock)` |
| `ErrorKind::WouldBlock` | Nothing in the receive buffer right now |
| `let _ = expr` | Explicitly discard a `Result` without a warning |
