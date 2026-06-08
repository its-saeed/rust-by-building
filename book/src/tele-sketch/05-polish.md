# Lesson 5 — Polish

The app works. Now add three features that turn it from a proof-of-concept into something worth showing: per-player identity colour, a networked canvas clear, and a protocol version byte. Each one teaches something about the full lifecycle of a real protocol.

---

## Step 1 — The identity colour problem

Right now both clients choose their drawing colour from the same palette. Nothing stops them from both picking red. When that happens, you cannot tell whose strokes are whose.

The fix: each client picks a random colour once at startup — its **identity colour** — and uses it for all strokes. The palette still works for selecting what you draw locally, but remote strokes always appear in the remote client's identity colour, making the two players permanently distinguishable.

macroquad ships its own random number functions, so no new dependency is needed:

```rust
use macroquad::rand::gen_range;

let my_r = gen_range(80u8, 220u8);
let my_g = gen_range(80u8, 220u8);
let my_b = gen_range(80u8, 220u8);
```

The range 80–220 avoids colours that are too dark (hard to see on a dark background) or too bright (hard to distinguish from white).

---

## Step 2 — Encoding identity in each event

The identity colour travels inside every `DrawEvent` as the `r`, `g`, `b` fields. Change the send block so that instead of using the palette colour, you send the identity colour:

```rust
if is_mouse_button_down(MouseButton::Left) && my < canvas_h {
    let ev = DrawEvent {
        x: mx, y: my,
        r: my_r, g: my_g, b: my_b,  // identity colour, not palette
        size: brush_size,
        pen_down: true,
    };
    local_strokes.push(ev);
    let _ = socket.send_to(&ev.to_bytes(), SERVER);
}
```

Local strokes are still drawn with the palette colour — use `PALETTE[color_idx]` when rendering `local_strokes`. Remote strokes use `ev.r`, `ev.g`, `ev.b` directly, which decode as the remote player's identity colour.

```rust
// local strokes — use palette colour
let (pr, pg, pb) = PALETTE[color_idx];
for ev in local_strokes.iter().filter(|e| e.pen_down) {
    draw_circle(ev.x, ev.y, ev.size as f32, Color::from_rgba(pr, pg, pb, 255));
}

// remote strokes — use the colour embedded in the event
for ev in remote_strokes.iter().filter(|e| e.pen_down) {
    draw_circle(ev.x, ev.y, ev.size as f32, Color::from_rgba(ev.r, ev.g, ev.b, 200));
}
```

No extra field needed. The `r`, `g`, `b` fields already exist in the protocol — we just repurpose them to carry identity rather than palette choice.

---

## Step 3 — Canvas clear must travel over the network

Pressing `C` to clear your canvas is easy — just `local_strokes.clear()`. But if you do that locally without telling the other client, the canvases go out of sync: you see a blank canvas, they still see everything you drew.

Clear needs to travel as a `DrawEvent`. There is no "clear" concept in the current 13-byte protocol, so you need to add one.

The cleanest solution is a dedicated boolean field. Add it to `DrawEvent`:

```rust
pub struct DrawEvent {
    pub x:        f32,
    pub y:        f32,
    pub r:        u8,
    pub g:        u8,
    pub b:        u8,
    pub size:     u8,
    pub pen_down: bool,
    pub clear:    bool,   // ← new
}
```

The struct is now 14 bytes. Update serialisation in both directions:

```rust
// to_bytes — add at the end:
buf[13] = self.clear as u8;

// from_bytes — change the size check and add the field:
if buf.len() < 14 { return None; }
// ...
clear: buf[13] != 0,
```

The buffer in both client and server is already `[0u8; 64]` — no change needed there.

---

## Step 4 — Sending a clear event

When `C` is pressed:

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

The non-`clear` fields are all zero — they are ignored by the receiver. The server validates the packet with `from_bytes` (which now requires 14 bytes and returns `Some` for any valid event), then relays it to the other client. The other client sees `ev.clear == true` and clears its canvas too.

Both canvases — local and remote — are cleared on the sending side before the event even reaches the server. This gives instant local feedback. The other client's canvas clears a round-trip later (a few milliseconds), which is imperceptible.

---

## Step 5 — Handling clear on receive

The receive loop needs one extra branch:

```rust
if let Some(ev) = DrawEvent::from_bytes(&buf[..n]) {
    if ev.clear {
        local_strokes.clear();
        remote_strokes.clear();
    } else if ev.pen_down {
        remote_strokes.push(ev);
    }
}
```

When `ev.clear` is true, both lists are cleared — you clear everything, not just the remote player's strokes, because a clear is a mutual reset of the shared canvas.

The `else if ev.pen_down` guard skips lift events (pen-up without drawing). These are not currently sent, but the field exists in the struct for future use.

---

## Step 6 — This is a breaking protocol change

Adding a byte changes the wire format. A client built against the old 13-byte protocol would:
- Send 13 bytes; a 14-byte server would see `buf.len() < 14` and return `None` → silently dropped
- Receive 14 bytes; old `from_bytes` would see only the first 13 → `clear` would never be set, which is wrong but harmless for regular strokes

In the classroom setting, the fix is simple: rebuild all three programs together. In a production system, every participant must agree on the same version before exchanging messages. Which brings us to versioning.

---

## Step 7 — Protocol version byte

Add a version number at byte 0. Shift everything else by one:

```rust
// to_bytes:
buf[0]   = 1;                              // version
buf[1..5].copy_from_slice(&self.x.to_le_bytes());
buf[5..9].copy_from_slice(&self.y.to_le_bytes());
buf[9]   = self.r;
buf[10]  = self.g;
buf[11]  = self.b;
buf[12]  = self.size;
buf[13]  = self.pen_down as u8;
buf[14]  = self.clear as u8;
// 15 bytes total

// from_bytes:
if buf.len() < 15 { return None; }
if buf[0] != 1 { return None; }   // wrong version → drop silently
// ... read fields from buf[1..] ...
```

`from_bytes` returning `None` on version mismatch means the server drops the packet (its `is_some()` guard), and the client ignores it (its `if let Some` guard). A stale client connecting to a new server — or vice versa — fails gracefully rather than drawing garbage.

This is how virtually every binary protocol handles versioning: a fixed byte (or two) at the start, checked immediately, reject anything that does not match. TLS, Postgres wire protocol, and most game networking protocols all do this.

---

## Step 8 — Upgrading the server display

The server can now show not just that a peer is connected, but how long it has been connected. Change the peer map value from a single `Instant` (last seen) to a pair:

```rust
let mut peers: HashMap<SocketAddr, (Instant, Instant)> = HashMap::new();
//                                  ╰─ connected_at    ╰─ last_seen
```

On a new packet from a known peer, update only `last_seen`. On a brand-new peer, set both:

```rust
peers
    .entry(from)
    .and_modify(|(_, last_seen)| *last_seen = now)
    .or_insert((now, now));
```

`entry` / `and_modify` / `or_insert` is the idiomatic Rust way to update-or-insert in a single pass through the hash map:
- If the key exists: `and_modify` runs, updating `last_seen`
- If the key is new: `or_insert` runs, inserting `(now, now)`

In the render section, display the uptime alongside the address:

```rust
for (addr, (connected_at, _)) in &peers {
    let secs = connected_at.elapsed().as_secs();
    let label = format!("  ●  {addr}   ({secs}s)");
    draw_text(&label, 40.0, y, 20.0, LIGHTGRAY);
}
```

`connected_at.elapsed()` is equivalent to `Instant::now().duration_since(*connected_at)` — a convenient shorthand when you just need the duration from a past instant to now.

---

## Exercise

> **TODO 1**: Implement the `clear: bool` extension. Add the field, update `to_bytes` and `from_bytes`, update the client to send a clear event on `C`, update the receive loop to handle it. Test: press `C` in one client and verify the other's canvas clears.
>
> **TODO 2**: Implement the version byte (step 7). After updating both client and server, intentionally run a client built without the version byte against the new server. What does the server log (packet counter, peer list)? Does the client appear as a peer?
>
> **TODO 3**: The canvas replays all events every frame — rendering cost is O(n) in total strokes. Describe (no implementation required) how you would fix this with a render texture: what macroquad API would you use, when would you draw to the texture versus blit it, and what events would require redrawing from scratch?

---

## What you built

In five lessons, from blank Cargo.toml to live shared drawing:

1. **Protocol** — a fixed-size binary struct, `to_bytes`/`from_bytes`, verified with a unit test
2. **Server** — UDP relay with implicit peer registration, pruning, and a macroquad dashboard
3. **Canvas** — event-driven drawing, colour palette, brush size, replay rendering
4. **Going live** — non-blocking drain loop, send on draw, connection indicator
5. **Polish** — identity colour, networked clear, protocol versioning, server uptime display

The patterns here — compact binary messages, drain loop in a frame, implicit registration, version byte at position zero — appear in virtually every real-time multiplayer system, from game engines to collaborative editors.

---

## Key APIs

| API | What it does |
|-----|-------------|
| `macroquad::rand::gen_range(lo, hi)` | Random value in `[lo, hi)`, no extra crate |
| `is_key_pressed(KeyCode::C)` | True on the single frame the key first goes down |
| `HashMap::entry(key).and_modify(\|v\| ...).or_insert(val)` | Update existing or insert new in one pass |
| `instant.elapsed()` | Duration from `instant` to now — shorthand for `now.duration_since(instant)` |
