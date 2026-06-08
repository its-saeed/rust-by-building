# Lesson 5 — Polish

The app works. Now add three things that turn a proof-of-concept into something you want to actually use: per-player colour, canvas clear, and a protocol version field.

---

## Per-player colour

Right now both clients choose their own colour, which means they might pick the same one. A nicer approach: assign each client a random colour on startup, and use it for all strokes regardless of what is in the palette.

Or, keep the palette for local preference but also tag every event with the sender's **identity colour** — a random `(r, g, b)` chosen once at launch — and use that identity colour to tint remote strokes:

```rust
// on startup, pick a random identity colour
let my_color = (
    rand::gen_range(80u8, 220),
    rand::gen_range(80u8, 220),
    rand::gen_range(80u8, 220),
);
```

macroquad exposes `rand::gen_range` — no extra crate needed. Use your chosen palette colour for local drawing, but when drawing remote strokes, use the remote's `r/g/b` directly (they encode their identity colour).

---

## Canvas clear — extending the protocol

Clear needs to travel over the network: if you clear your canvas, you want to clear the remote one too. That means it needs to be a `DrawEvent`. But there is no natural "clear" in the current seven fields.

Add one byte:

```rust
pub struct DrawEvent {
    pub x:        f32,
    pub y:        f32,
    pub r:        u8,
    pub g:        u8,
    pub b:        u8,
    pub size:     u8,
    pub pen_down: bool,
    pub clear:    bool,  // ← new: if true, receiver clears their canvas
}
// now 14 bytes — update to_bytes / from_bytes
```

Update `to_bytes` to write `buf[13] = self.clear as u8;` and `from_bytes` to read `clear: buf[13] != 0`. Update the buffer declaration in server and client to `[0u8; 64]` — both already use that size.

Send it when `C` is pressed:

```rust
if is_key_pressed(KeyCode::C) {
    remote_strokes.clear();
    local_strokes.clear();
    let ev = DrawEvent {
        x: 0.0, y: 0.0, r: 0, g: 0, b: 0, size: 0,
        pen_down: false, clear: true,
    };
    let _ = socket.send_to(&ev.to_bytes(), SERVER);
}
```

Handle it on receive:

```rust
if let Some(ev) = DrawEvent::from_bytes(&buf[..n]) {
    if ev.clear {
        remote_strokes.clear();
        local_strokes.clear();
    } else {
        remote_strokes.push(ev);
    }
}
```

This is a protocol-breaking change — a server or client that does not know about `clear` will misread the 14th byte. Real protocols handle this with version numbers. For a classroom project, just restart everything.

---

## Protocol version field

While you are adding a byte, consider adding a version byte at the front:

```rust
// to_bytes: buf[0] = 1;  // version
// from_bytes: if buf[0] != 1 { return None; }
```

`from_bytes` returning `None` on version mismatch means the server silently drops incompatible packets rather than relaying garbage. The `is_some()` check in the server already handles this.

---

## A nicer server display

Now that the server shows peer addresses, you can make it more useful. Display how long each peer has been connected:

```rust
// store connect time alongside last-seen time
let mut peers: HashMap<SocketAddr, (Instant, Instant)> = HashMap::new();
// (connected_at, last_seen)

// on new peer:
peers.entry(from).or_insert((now, now)).1 = now;

// on render:
for (addr, (connected_at, _)) in &peers {
    let secs = connected_at.elapsed().as_secs();
    draw_text(&format!("  ●  {addr}  ({secs}s)"), 40.0, y, 20.0, LIGHTGRAY);
}
```

---

## Exercise

> **TODO 1**: Add the `clear` field and update the protocol. Test with two clients: press `C` in one and watch the other's canvas clear.
>
> **TODO 2**: Add a version byte at position 0 and shift all other fields by 1. Update both to_bytes and from_bytes. Confirm that starting a mismatched client and server results in the server dropping the packets silently.
>
> **TODO 3**: The canvas replays all events every frame. For a long session with many strokes this becomes slow. How would you fix it? (Hint: macroquad has render textures — `render_target()` lets you draw to an offscreen buffer once and blit it each frame.) You do not need to implement this; just describe the approach.

---

## What you built

In five lessons, from protocol design to live shared drawing:

1. Defined a binary protocol and tested it in isolation
2. Built a relay server with a visual dashboard (macroquad + non-blocking UDP)
3. Built a local drawing canvas (macroquad, event-driven)
4. Wired them together with a non-blocking drain loop
5. Extended the protocol with clear and player identity

The patterns here — fixed-size binary messages, drain loop in a game frame, implicit peer registration — appear in virtually every real-time multiplayer system.
