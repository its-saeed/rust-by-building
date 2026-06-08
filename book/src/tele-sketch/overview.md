# Project 9 — Tele-Sketch

You have built single-player games and learned how networks work. Now you combine both: a real-time collaborative drawing app where two people share a canvas across the network.

Open two windows. Draw on one — the strokes appear on the other instantly. Draw on the other — they appear back. That is it. Simple idea, immediate payoff.

---

## What you will build

Three programs in a single Cargo package:

- **`cargo run --bin server`** — a macroquad window showing who is connected and packets being relayed
- **`cargo run --bin client`** — the drawing canvas, colour palette, brush size control

Run the server once. Run the client on two machines (or two terminals on the same machine). Draw.

---

## What makes this interesting

Previous projects communicated with an API or tested things with `nc`. Here your two programs talk to each other continuously, 60 times per second. The challenge is fitting network I/O into a game loop without blocking the frame — which you solve with `set_nonblocking(true)` and a drain loop, the same pattern you would use in any real-time multiplayer game.

The server also uses macroquad, which shows that macroquad is just a windowed loop — it works fine for tools and dashboards, not only games.

---

## Concepts covered

- Multiple binaries in one Cargo package (`[[bin]]`, `[lib]`)
- Fixed-size binary protocol — `f32::to_le_bytes()`, `u8` packing
- Non-blocking UDP sockets in a game loop — `set_nonblocking`, `WouldBlock`
- UDP broadcast pattern — server tracks clients, relays to all except sender
- Integrating network I/O with macroquad's frame loop

No new Rust concepts — everything here uses tools you already have.

---

## Project structure

```
src/
  lib.rs          ← re-exports the shared event module
  event.rs        ← DrawEvent: the protocol type, serialisation
  bin/
    server.rs     ← UDP relay + macroquad dashboard
    client.rs     ← macroquad canvas + UDP send/receive
```

`event.rs` is shared between both binaries through the crate's library. Neither binary knows anything about the other — they only agree on the format of a `DrawEvent`.
