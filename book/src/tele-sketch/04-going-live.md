# Lesson 4 — Going Live

The canvas works locally. Now wire it to the server: send your strokes, receive theirs.

This is the lesson where two windows become one shared canvas.

---

## Creating the socket

```rust
let socket = UdpSocket::bind("0.0.0.0:0").expect("bind failed");
socket.set_nonblocking(true).expect("set_nonblocking failed");

const SERVER: &str = "127.0.0.1:9090";
```

`bind("0.0.0.0:0")` — any interface, any free port. The client does not need a known port; only the server does. `set_nonblocking(true)` is essential: without it, `recv_from` would freeze the game loop every frame waiting for a packet that might never come.

---

## Sending

Each frame, when the pen is down, you already push an event to `local_strokes`. Add one line to also send it:

```rust
if is_mouse_button_down(MouseButton::Left) && my < canvas_h {
    let ev = DrawEvent { x: mx, y: my, r, g, b, size: brush_size, pen_down: true };
    local_strokes.push(ev);
    let _ = socket.send_to(&ev.to_bytes(), SERVER); // ← new
}
```

The `let _ =` discards the result — UDP send errors are usually transient and not worth crashing over. In production code you would log them; here silence is fine.

---

## Receiving — the drain loop

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

This loop runs at the top of every frame. It drains all packets that arrived since last frame in one go. When there is nothing left, `recv_from` returns `WouldBlock` and the loop breaks — zero blocking, zero delay.

`_from` is discarded because the client does not need to know which address the event came from — that is the server's concern. Every event that arrives was relayed by the server, so it came from some other client.

---

## Connection indicator

Add a small status indicator so you can tell at a glance whether the server is reachable:

```rust
let mut connected = false;

// inside the receive loop, after a successful recv:
connected = true;

// in the render section:
let (label, color) = if connected {
    ("● LIVE", GREEN)
} else {
    ("○ waiting...", DARKGRAY)
};
draw_text(label, screen_width() - 140.0, 24.0, 20.0, color);
```

`connected` flips to `true` the first time any remote packet arrives. It never flips back — in a finished app you would also detect disconnection (e.g. by noting when the last remote packet was received), but for a classroom demo this is enough.

---

## Running it

Open three terminals:

```sh
# terminal 1
cargo run --bin server

# terminal 2
cargo run --bin client

# terminal 3
cargo run --bin client
```

Draw in one client window. Watch the strokes appear in the other. Watch the server window update its peer count.

If you only have one machine, both clients share the same keyboard and mouse — just switch focus between windows.

---

## Exercise

> **TODO 1**: Track when the last remote packet arrived. If more than 3 seconds have passed since the last remote packet, show "○ waiting..." even if `connected` was previously true. Use `Instant::now()` and `Option<Instant>`.
>
> **TODO 2**: The server relays to all peers except the sender. But what if two clients send at exactly the same time and both receive each other's strokes? Is there any risk of double-drawing? Think through the timing and explain why or why not.
>
> **TODO 3**: What happens if the server is not running when the client starts? Test it. What does `send_to` return? Should the client crash, print a warning, or do nothing?

---

## The complete frame

In the right order:

```
1. drain receive loop   (remote strokes arrive)
2. handle mouse input   (local strokes recorded + sent)
3. handle UI input      (colour, brush size)
4. clear background
5. draw local_strokes
6. draw remote_strokes
7. draw palette + UI
8. next_frame().await
```

Input before drawing, drawing before UI. This order ensures that events recorded this frame are drawn this frame — no one-frame lag between click and stroke.
