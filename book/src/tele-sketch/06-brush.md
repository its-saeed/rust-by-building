# Lesson 6 — Brush Size & Polish

The last lesson adds three independent improvements: a scroll-wheel brush size control, a networked canvas clear, and a per-player identity colour. It also introduces a protocol version byte — the right way to handle the breaking change that adding `clear` creates.

---

## Step 1 — Brush size with the scroll wheel

Add one variable before the loop:

```rust
let mut brush_size: u8 = 8;
```

At the top of each frame, read the scroll wheel and adjust:

```rust
let scroll = mouse_wheel().1;
if scroll > 0.0 { brush_size = (brush_size + 2).min(40); }
if scroll < 0.0 { brush_size = brush_size.saturating_sub(2).max(2); }
```

`mouse_wheel()` returns `(horizontal_delta, vertical_delta)`. Positive vertical = scroll up = bigger brush.

`saturating_sub(2)` prevents underflow: subtracting 2 from a `u8` that is already 0 would wrap to 254. `saturating_sub` stops at 0, then `.max(2)` lifts the minimum to 2 px.

`brush_size` is a `u8` because it maps directly to `DrawEvent.size` — same type, no cast needed.

Update the stroke creation to use the variable:

```rust
let ev = DrawEvent { x: mx, y: my, r, g, b, size: brush_size, pen_down: true };
```

Add a preview circle in the palette row showing current size in current colour:

```rust
draw_circle(
    screen_width() - 50.0,
    screen_height() - 30.0,
    brush_size as f32,
    Color::from_rgba(r, g, b, 255),
);
```

---

## Step 2 — Identity colour

Both players might pick the same palette colour. Give each client a random colour at startup that persists for the whole session — an **identity colour** — and tag every outgoing event with it instead of the palette colour.

```rust
use macroquad::rand::gen_range;

let id_r = gen_range(80u8, 220u8);
let id_g = gen_range(80u8, 220u8);
let id_b = gen_range(80u8, 220u8);
```

80–220 avoids colours too dark to see on a dark background and too close to white.

Use the identity colour when sending:

```rust
let ev = DrawEvent { x: mx, y: my, r: id_r, g: id_g, b: id_b, size: brush_size, pen_down: true };
```

Draw local strokes in the palette colour (what the user chose visually) and remote strokes in `ev.r/g/b` (the remote player's identity colour):

```rust
let (pr, pg, pb) = PALETTE[color_idx];
for ev in local_strokes.iter().filter(|e| e.pen_down) {
    draw_circle(ev.x, ev.y, ev.size as f32, Color::from_rgba(pr, pg, pb, 255));
}
for ev in remote_strokes.iter().filter(|e| e.pen_down) {
    draw_circle(ev.x, ev.y, ev.size as f32, Color::from_rgba(ev.r, ev.g, ev.b, 200));
}
```

---

## Step 3 — Canvas clear: extending the protocol

Pressing `C` locally is simple — clear both vecs. But it needs to travel to the other client, which means it needs to be a `DrawEvent`.

The current 13-byte protocol has no room for a "clear" signal. Add a field:

```rust
// in event.rs:
pub struct DrawEvent {
    pub x:        f32,
    pub y:        f32,
    pub r:        u8,
    pub g:        u8,
    pub b:        u8,
    pub size:     u8,
    pub pen_down: bool,
    pub clear:    bool,  // ← new
}
// 14 bytes now
```

Update `to_bytes`:
```rust
// add at the end:
buf[13] = self.clear as u8;
```

Update `from_bytes`:
```rust
// change the length check:
if buf.len() < 14 { return None; }
// add the field:
clear: buf[13] != 0,
```

The server and client buffers are already `[0u8; 64]` — no change needed there.

---

## Step 4 — Sending a clear event

When `C` is pressed, clear locally and send a clear event to the server, which relays it to the other client:

```rust
if is_key_pressed(KeyCode::C) {
    local_strokes.clear();
    remote_strokes.clear();

    let ev = DrawEvent {
        x: 0.0, y: 0.0,
        r: 0, g: 0, b: 0, size: 0,
        pen_down: false,
        clear: true,
    };
    let _ = socket.send_to(&ev.to_bytes(), SERVER);
}
```

The non-`clear` fields are all zero — the receiver ignores them when `clear` is true.

Handle it in the receive loop:

```rust
if let Some(ev) = DrawEvent::from_bytes(&buf[..n]) {
    if ev.clear {
        local_strokes.clear();
        remote_strokes.clear();
    } else {
        connected = true;
        remote_strokes.push(ev);
    }
}
```

Both lists clear on receipt — a clear is a mutual reset of the shared canvas.

---

## Step 5 — Protocol version byte

Adding `clear` changed the wire format. A client or server built before this lesson would misread the 14th byte. Real protocols handle this with a version number at the start.

Shift all fields by one byte and put the version at position 0:

```rust
// to_bytes:
buf[0]    = 1;                              // version
buf[1..5].copy_from_slice(&self.x.to_le_bytes());
buf[5..9].copy_from_slice(&self.y.to_le_bytes());
buf[9]    = self.r;
buf[10]   = self.g;
buf[11]   = self.b;
buf[12]   = self.size;
buf[13]   = self.pen_down as u8;
buf[14]   = self.clear as u8;
// 15 bytes total

// from_bytes:
if buf.len() < 15 { return None; }
if buf[0] != 1    { return None; }   // wrong version → None → dropped
// read fields from buf[1..]
```

`from_bytes` returning `None` on a version mismatch means the server drops the packet silently (its `is_some()` guard) and the client ignores it (its `if let Some` guard). A stale client connecting to a new server fails gracefully rather than drawing garbage.

Update the test in `event.rs` to exercise both a valid packet and a wrong-version packet:

```rust
#[test]
fn rejects_wrong_version() {
    let ev = DrawEvent { x: 1.0, y: 2.0, r: 255, g: 0, b: 0, size: 5, pen_down: true, clear: false };
    let mut bytes = ev.to_bytes();
    bytes[0] = 99;   // corrupt the version byte
    assert!(DrawEvent::from_bytes(&bytes).is_none());
}
```

---

## Full client

```rust
use macroquad::prelude::*;
use macroquad::rand::gen_range;
use std::io::ErrorKind;
use std::net::UdpSocket;
use tele_sketch::event::DrawEvent;

const SERVER: &str = "127.0.0.1:9090";

const PALETTE: [(u8, u8, u8); 8] = [
    (255, 255, 255),
    (20,  20,  20 ),
    (220, 60,  60 ),
    (60,  200, 80 ),
    (60,  110, 240),
    (240, 165, 30 ),
    (200, 60,  200),
    (40,  200, 210),
];

#[macroquad::main("Tele-Sketch")]
async fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("bind failed");
    socket.set_nonblocking(true).expect("set_nonblocking failed");

    let id_r = gen_range(80u8, 220u8);
    let id_g = gen_range(80u8, 220u8);
    let id_b = gen_range(80u8, 220u8);

    let mut local_strokes:  Vec<DrawEvent> = Vec::new();
    let mut remote_strokes: Vec<DrawEvent> = Vec::new();
    let mut connected   = false;
    let mut color_idx:  usize = 0;
    let mut brush_size: u8    = 8;
    let mut buf = [0u8; 64];

    loop {
        // ── derived values ────────────────────────────────────────────────
        let (r, g, b)  = PALETTE[color_idx];
        let canvas_h   = screen_height() - 60.0;
        let palette_y  = screen_height() - 52.0;

        // ── receive ───────────────────────────────────────────────────────
        loop {
            match socket.recv_from(&mut buf) {
                Ok((n, _)) => {
                    if let Some(ev) = DrawEvent::from_bytes(&buf[..n]) {
                        if ev.clear {
                            local_strokes.clear();
                            remote_strokes.clear();
                        } else {
                            connected = true;
                            remote_strokes.push(ev);
                        }
                    }
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => break,
                Err(_) => break,
            }
        }

        // ── clear ─────────────────────────────────────────────────────────
        if is_key_pressed(KeyCode::C) {
            local_strokes.clear();
            remote_strokes.clear();
            let ev = DrawEvent {
                x: 0.0, y: 0.0, r: 0, g: 0, b: 0, size: 0,
                pen_down: false, clear: true,
            };
            let _ = socket.send_to(&ev.to_bytes(), SERVER);
        }

        // ── palette clicks ────────────────────────────────────────────────
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mpx, mpy) = mouse_position();
            for (i, _) in PALETTE.iter().enumerate() {
                let px = 10.0 + i as f32 * 50.0;
                if mpx >= px && mpx < px + 40.0 && mpy >= palette_y {
                    color_idx = i;
                }
            }
        }

        // ── brush size ────────────────────────────────────────────────────
        let scroll = mouse_wheel().1;
        if scroll > 0.0 { brush_size = (brush_size + 2).min(40); }
        if scroll < 0.0 { brush_size = brush_size.saturating_sub(2).max(2); }

        // ── stroke input ──────────────────────────────────────────────────
        let (mx, my) = mouse_position();
        if is_mouse_button_down(MouseButton::Left) && my < canvas_h {
            let ev = DrawEvent {
                x: mx, y: my,
                r: id_r, g: id_g, b: id_b,
                size: brush_size,
                pen_down: true,
                clear: false,
            };
            local_strokes.push(ev);
            let _ = socket.send_to(&ev.to_bytes(), SERVER);
        }

        // ── render ────────────────────────────────────────────────────────
        clear_background(Color::from_rgba(30, 30, 35, 255));

        for ev in local_strokes.iter().filter(|e| e.pen_down) {
            draw_circle(ev.x, ev.y, ev.size as f32, Color::from_rgba(r, g, b, 255));
        }
        for ev in remote_strokes.iter().filter(|e| e.pen_down) {
            draw_circle(ev.x, ev.y, ev.size as f32, Color::from_rgba(ev.r, ev.g, ev.b, 200));
        }

        draw_line(0.0, canvas_h, screen_width(), canvas_h, 1.0, DARKGRAY);

        for (i, &(pr, pg, pb)) in PALETTE.iter().enumerate() {
            let px = 10.0 + i as f32 * 50.0;
            draw_rectangle(px, palette_y, 40.0, 40.0, Color::from_rgba(pr, pg, pb, 255));
            if color_idx == i {
                draw_rectangle_lines(px - 2.0, palette_y - 2.0, 44.0, 44.0, 3.0, WHITE);
            }
        }

        draw_circle(screen_width() - 50.0, screen_height() - 30.0,
                    brush_size as f32, Color::from_rgba(r, g, b, 255));

        let (label, lc) = if connected { ("● LIVE", GREEN) } else { ("○ waiting...", DARKGRAY) };
        draw_text(label, screen_width() - 140.0, 24.0, 20.0, lc);

        draw_text("C = clear", 10.0, 24.0, 18.0, DARKGRAY);

        next_frame().await;
    }
}
```

---

## Exercise

> **TODO 1**: Implement the version byte (step 5). After rebuilding, intentionally test a version mismatch: start the new server, then connect with a client that does not send the version byte. What do you observe in the server window?
>
> **TODO 2**: The canvas replays all events every frame. For a long session this becomes slow — O(n) per frame. Describe (no implementation needed) how you would fix it with a render texture: when do you draw to the texture, when do you blit it, and what events require a full redraw?
>
> **TODO 3**: Right now `C` clears both `local_strokes` and `remote_strokes` locally before the event even reaches the server. What happens if the server is unreachable when `C` is pressed? Is this behaviour correct? Argue for or against clearing locally before confirming the server relayed the clear.

---

## What you built

Six lessons — from blank Cargo.toml to live shared drawing:

1. **Protocol** — 13-byte `DrawEvent`, `to_le_bytes`/`from_le_bytes`, round-trip test
2. **Server** — UDP relay, implicit peer registration, macroquad dashboard
3. **Basic Canvas** — event-list canvas, white strokes, fixed brush
4. **Going Live** — non-blocking drain loop, send on draw, connection indicator
5. **Colour Palette** — 8 colours, palette UI, colour baked into events
6. **Brush & Polish** — scroll-wheel size, identity colour, networked clear, version byte

The patterns here — compact binary messages, drain loop in a game frame, implicit peer registration, version byte at position zero — appear in virtually every real-time multiplayer system.

---

## Key APIs

| API | What it does |
|-----|-------------|
| `mouse_wheel()` | `(horizontal, vertical)` scroll delta this frame |
| `u8::saturating_sub(n)` | Subtract without underflowing — clamps to 0 |
| `macroquad::rand::gen_range(lo, hi)` | Random value in `[lo, hi)` — no extra crate |
| `is_key_pressed(KeyCode::C)` | `true` on the single frame the key first goes down |
| `HashMap::entry(k).and_modify(\|v\| ...).or_insert(val)` | Update-or-insert in one pass |
