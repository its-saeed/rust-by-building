# Lesson 4 — Going Live

The canvas works locally. Now add the two lines that turn it into a multiplayer app: send your strokes to the server, and receive the other player's strokes back.

This is the lesson where two separate windows become one shared canvas.

---

## Step 1 — Creating the client socket

```rust
let socket = UdpSocket::bind("0.0.0.0:0").expect("bind failed");
socket.set_nonblocking(true).expect("set_nonblocking failed");

const SERVER: &str = "127.0.0.1:9090";
```

This goes before the game loop, alongside the other `let mut` declarations.

`bind("0.0.0.0:0")` — bind to all interfaces, port zero. Port zero tells the OS: pick any free port for me. The client does not need a known port because nobody dials the client; the client dials the server. The OS assigns an ephemeral port (usually in the 49152–65535 range) and the client uses it for the lifetime of the process.

`set_nonblocking(true)` — exactly the same reason as the server: `recv_from` must not block the macroquad frame loop. If no packet has arrived since the last frame, `recv_from` returns `WouldBlock` immediately and the loop moves on.

`SERVER` is a constant string. Parsing `"127.0.0.1:9090"` into a `SocketAddr` happens inside `send_to` on each call — cheap enough that there is no reason to pre-parse it.

---

## Step 2 — Sending a stroke

Inside the mouse-input block from lesson 3, add one line:

```rust
if is_mouse_button_down(MouseButton::Left) && my < canvas_h {
    let ev = DrawEvent { x: mx, y: my, r, g, b, size: brush_size, pen_down: true };
    local_strokes.push(ev);
    let _ = socket.send_to(&ev.to_bytes(), SERVER); // ← new
}
```

That is the entire send side. One line, every frame the mouse is down.

`ev.to_bytes()` produces the 13-byte array defined in lesson 1. `send_to` passes it to the OS, which fires it off as a UDP datagram. The OS does not wait for delivery confirmation — `send_to` returns as soon as the packet is handed to the network stack, usually in microseconds.

`let _ =` discards the `Result`. UDP sends can fail if the OS buffer is full or the network is unavailable, but there is nothing useful to do about a lost drawing stroke. In production you might log errors; here silence is the right choice.

Notice that `local_strokes.push(ev)` happens **before** `send_to`. The client draws its own stroke locally without waiting for any confirmation from the server. If you waited for the server to echo back before drawing, every stroke would have at least one round-trip of lag (potentially 1–100 ms). Drawing locally and sending concurrently gives instant visual feedback — the stroke appears the moment the mouse moves.

---

## Step 3 — The receive drain loop

This loop runs at the **top** of every frame, before any input handling:

```rust
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

`recv_from` with `set_nonblocking(true)` has three possible outcomes:
- **`Ok((n, from))`** — a packet arrived; `n` bytes are in `buf[..n]`, sent from address `from`
- **`Err(WouldBlock)`** — nothing waiting in the OS receive buffer; the loop is done
- **`Err(_)`** — a real error (interface went down, etc.) — break and move on

The loop drains everything. If ten packets arrived since the last frame, all ten are processed before rendering. This is important: without the loop, you would process at most one packet per frame (60 per second), which could cause the remote canvas to lag behind.

`_from` is discarded. The client does not need to know which address sent the packet — the server guarantees that everything it relays came from another client, not a loop-back of your own strokes.

`DrawEvent::from_bytes` validates before pushing. Corrupt or truncated packets return `None` and are ignored.

---

## Step 4 — Why drain before input

The drain loop runs first, before mouse input. Here is why that order matters:

```
Frame N:
  1. drain:  receive 3 remote strokes → push to remote_strokes
  2. input:  record 1 local stroke → push to local_strokes
  3. render: draw 3 remote + 1 local = 4 new strokes visible
```

If you drained at the bottom of the frame instead:

```
Frame N:
  1. input:  record 1 local stroke
  2. render: draw 1 local (remote strokes from this frame invisible until frame N+1)
  3. drain:  receive 3 remote strokes (rendered next frame only)
```

Remote strokes would always be one frame behind. At 60 fps that is 16 ms of extra lag — imperceptible, but the principle matters: process incoming data before rendering so the frame you show is as current as possible.

---

## Step 5 — Connection indicator

A `bool` that flips the first time any remote packet arrives:

```rust
let mut connected = false;

// inside the receive loop, after a successful parse:
connected = true;

// in the render section:
let (label, color) = if connected {
    ("● LIVE", GREEN)
} else {
    ("○ waiting...", DARKGRAY)
};
draw_text(label, screen_width() - 140.0, 24.0, 20.0, color);
```

`connected` starts `false`. The first remote event flips it to `true` — it stays `true` for the rest of the session even if the other client disconnects. A more complete implementation would track `last_remote_packet: Option<Instant>` and show "waiting" again if no packet has arrived in the past few seconds. That is left as an exercise.

Why not check whether `send_to` succeeded? Because UDP `send_to` succeeds at the OS level even when the server is not running — the packet is handed to the network stack, which fires it into the void. The only reliable signal that a remote peer is present is receiving a packet from them.

---

## Step 6 — Running it

Open three terminals in the project directory:

```sh
# terminal 1 — start first
cargo run --bin server

# terminal 2
cargo run --bin client

# terminal 3
cargo run --bin client
```

Draw in client window 2. Watch the strokes appear in client window 3. Watch the server window show "peers connected: 2" in green and the packet counter climb.

If you only have one machine and one mouse, switch focus between the two client windows (click the title bar). Each window is an independent process with its own socket.

To test from two machines on the same network: change `SERVER` in `client.rs` to the server machine's IP address and run `cargo run --bin server` on that machine.

---

## Complete frame order

```
┌──────────────────────────────────────────┐
│  every frame                             │
│                                          │
│  1. drain recv loop                      │
│       → remote_strokes gets new events   │
│                                          │
│  2. mouse input                          │
│       → local_strokes gets new events    │
│       → send_to server                   │
│                                          │
│  3. palette clicks → update color_idx    │
│  4. scroll wheel  → update brush_size    │
│                                          │
│  5. clear_background                     │
│  6. draw local_strokes                   │
│  7. draw remote_strokes                  │
│  8. draw separator + palette + preview   │
│  9. draw connection indicator            │
│                                          │
│  10. next_frame().await                  │
└──────────────────────────────────────────┘
```

---

## Exercise

> **TODO 1**: Improve the connection indicator. Instead of a bool that never resets, track `last_remote: Option<Instant>`. Show "● LIVE" if a remote packet arrived within the last 3 seconds, "○ waiting..." otherwise. What `Instant` methods do you need?
>
> **TODO 2**: What happens if the server is not running when the client starts? Run the client without starting the server first. Does `send_to` error? Does `recv_from` error? Does anything crash? Based on what you observe, explain why UDP clients do not need the server to be running before they start.
>
> **TODO 3**: The server relays only to peers other than the sender. Trace through what happens if two clients send a packet in the same server frame: does either client receive its own event? Can double-drawing occur?

---

## Key APIs

| API | What it does |
|-----|-------------|
| `UdpSocket::bind("0.0.0.0:0")` | Bind to any interface, any free port |
| `socket.set_nonblocking(true)` | `recv_from` returns immediately if nothing waiting |
| `socket.send_to(&bytes, addr)` | Send a UDP datagram to a destination |
| `socket.recv_from(&mut buf)` | Receive one datagram — `Ok((n, from))` or `Err` |
| `ErrorKind::WouldBlock` | Returned when nothing is in the receive buffer |
| `let _ = expr` | Explicitly discard a `Result` without a warning |
