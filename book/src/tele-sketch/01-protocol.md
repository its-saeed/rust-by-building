# Lesson 1 — Protocol & Project Setup

Before any networking code, you need to answer one question: **what bytes go over the wire?** This is the protocol. Get it wrong and nothing works. Get it right and everything else is plumbing.

In Tele-Sketch, every drawing action is a single `DrawEvent`. When your pen touches the canvas, you send one. When it moves, you send another. When it lifts, you send one more. The server relays each to every other client.

---

## Big picture

Three programs, all running simultaneously:

```
 ┌──────────────────────────────────┐
 │         cargo run --bin server   │
 │                                  │
 │   macroquad window               │
 │   ┌────────────────────────┐     │
 │   │  peers connected: 2   │     │
 │   │  ● 127.0.0.1:54321    │     │
 │   │  ● 127.0.0.1:54322    │     │
 │   └────────────────────────┘     │
 │                                  │
 │   UdpSocket bound to :9090       │
 │   non-blocking, drain each frame │
 └──────────────┬───────────────────┘
                │
       relay DrawEvent
       to all peers
       except sender
                │
       ┌────────┴────────┐
       │                 │
       ▼                 ▼
┌─────────────┐   ┌─────────────┐
│  client A   │   │  client B   │
│             │   │             │
│  macroquad  │   │  macroquad  │
│  canvas     │   │  canvas     │
│             │   │             │
│  draw ───── │──▶│ ─── draw   │
│         DrawEvent (via server)│
│             │   │             │
│  UdpSocket  │   │  UdpSocket  │
│  :0 (any)   │   │  :0 (any)  │
│  non-block  │   │  non-block  │
└─────────────┘   └─────────────┘
```

**What flows:**
- Client A draws → sends `DrawEvent` → server → server relays to Client B → Client B draws the stroke
- Client B draws → sends `DrawEvent` → server → server relays to Client A → Client A draws the stroke
- Server **never echoes back to the sender** — each client draws its own strokes locally without waiting for the server

**Per-frame loop (same shape on all three programs):**

```
┌─────────────────────────────────────────────────────┐
│  every frame (~60 fps)                              │
│                                                     │
│  1. drain UDP receive loop                          │
│     loop { recv_from() → Ok(data) or WouldBlock }  │
│                                                     │
│  2. update state                                    │
│     client: record mouse input, send DrawEvent      │
│     server: update peer map, relay packets          │
│                                                     │
│  3. render                                          │
│     client: draw strokes                            │
│     server: draw dashboard                          │
│                                                     │
│  4. next_frame().await                              │
└─────────────────────────────────────────────────────┘
```

The only new idea in this project is step 1: `recv_from` with `set_nonblocking(true)` returns immediately with `WouldBlock` when nothing is waiting, keeping the frame loop running at full speed.

---

## The DrawEvent

```rust
#[derive(Debug, Clone, Copy)]
pub struct DrawEvent {
    pub x:        f32,  // canvas position
    pub y:        f32,
    pub r:        u8,   // stroke colour
    pub g:        u8,
    pub b:        u8,
    pub size:     u8,   // brush radius in pixels
    pub pen_down: bool, // true = draw, false = pen lifted
}
```

Seven fields, 13 bytes:

```
[x: 4 bytes][y: 4 bytes][r][g][b][size][pen_down as u8]
```

Fixed size is important. UDP preserves message boundaries — one `send_to` produces exactly one `recv_from` — so a fixed-size struct means you never need to frame or length-prefix your messages.

---

## Serialisation with `to_le_bytes`

`f32` values are stored in memory as 4 bytes. `to_le_bytes()` extracts those bytes in little-endian order; `from_le_bytes()` reconstructs the `f32` from them. You have already used this pattern with integers — it works identically for floats.

```rust
impl DrawEvent {
    pub fn to_bytes(self) -> [u8; 13] {
        let mut buf = [0u8; 13];
        buf[0..4].copy_from_slice(&self.x.to_le_bytes());
        buf[4..8].copy_from_slice(&self.y.to_le_bytes());
        buf[8]  = self.r;
        buf[9]  = self.g;
        buf[10] = self.b;
        buf[11] = self.size;
        buf[12] = self.pen_down as u8;
        buf
    }

    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < 13 { return None; }
        Some(DrawEvent {
            x:        f32::from_le_bytes(buf[0..4].try_into().ok()?),
            y:        f32::from_le_bytes(buf[4..8].try_into().ok()?),
            r:        buf[8],
            g:        buf[9],
            b:        buf[10],
            size:     buf[11],
            pen_down: buf[12] != 0,
        })
    }
}
```

`try_into().ok()?` converts `&[u8]` to `[u8; 4]` — it fails if the slice is the wrong length, which `from_bytes` turns into `None`.

---

## Multiple binaries in one package

So far every project had one binary: `src/main.rs`. Tele-Sketch has two (`client` and `server`) plus a library (`event.rs`). The layout:

```
src/
  lib.rs          ← declares the library (re-exports event)
  event.rs        ← DrawEvent lives here
  bin/
    client.rs     ← binary 1
    server.rs     ← binary 2
```

The `Cargo.toml` declares them explicitly:

```toml
[lib]
name = "tele_sketch"

[[bin]]
name = "client"
path = "src/bin/client.rs"

[[bin]]
name = "server"
path = "src/bin/server.rs"
```

Both binaries access `DrawEvent` through the library:

```rust
// in client.rs and server.rs
use tele_sketch::event::DrawEvent;
```

`src/lib.rs` simply makes the module public:

```rust
pub mod event;
```

Run each binary with:

```sh
cargo run --bin server
cargo run --bin client
```

---

## Exercise

> **TODO 1**: Add the `DrawEvent` struct to `src/event.rs` with the seven fields above.
>
> **TODO 2**: Implement `to_bytes` and `from_bytes` as shown.
>
> **TODO 3**: Add a round-trip test in `src/event.rs`:
>
> ```rust
> #[test]
> fn round_trip() {
>     let ev = DrawEvent { x: 100.5, y: 200.0, r: 255, g: 128, b: 0, size: 10, pen_down: true };
>     let bytes = ev.to_bytes();
>     let decoded = DrawEvent::from_bytes(&bytes).unwrap();
>     assert_eq!(ev.x, decoded.x);
>     assert_eq!(ev.pen_down, decoded.pen_down);
> }
> ```
>
> Run it with `cargo test`. If it passes, your protocol is correct.

---

## Key APIs

| API | What it does |
|-----|-------------|
| `f32::to_le_bytes()` | Extract the 4 raw bytes of an f32 (little-endian) |
| `f32::from_le_bytes([u8; 4])` | Reconstruct an f32 from 4 bytes |
| `buf[0..4].copy_from_slice(&bytes)` | Write a byte slice into a fixed buffer at an offset |
| `slice.try_into().ok()?` | Convert `&[u8]` to `[u8; N]`, returning `None` if wrong length |
