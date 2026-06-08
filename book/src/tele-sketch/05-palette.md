# Lesson 5 — Colour Palette

Both clients draw in white. That works, but you cannot tell whose strokes are whose. Add a colour palette: eight buttons at the bottom of the window, click to switch colour. Each `DrawEvent` already carries `r`, `g`, `b` — the field is there, the value just has not been hooked up to anything yet.

---

## Step 1 — The palette constant and state

Before the game loop:

```rust
const PALETTE: [(u8, u8, u8); 8] = [
    (255, 255, 255), // white
    (20,  20,  20 ), // near-black
    (220, 60,  60 ), // red
    (60,  200, 80 ), // green
    (60,  110, 240), // blue
    (240, 165, 30 ), // orange
    (200, 60,  200), // magenta
    (40,  200, 210), // cyan
];

let mut color_idx: usize = 0;
```

`PALETTE` is a module-level constant — it does not change, so it belongs outside `main`. `color_idx` is mutable state that changes when the user clicks a button; it starts at 0 (white).

---

## Step 2 — Reserve canvas space

The palette row occupies the bottom 60 pixels. Compute the boundary once at the top of each frame:

```rust
let canvas_h  = screen_height() - 60.0;
let palette_y = screen_height() - 52.0;
```

`canvas_h` is used in two places: to guard mouse input (so clicks in the palette area do not produce strokes) and to draw the separator line. `palette_y` positions the buttons.

---

## Step 3 — Use the active colour when drawing

Extract the current colour at the top of the frame, then use it in the mouse handler:

```rust
let (r, g, b) = PALETTE[color_idx];
```

Update the mouse-input block to use `r`, `g`, `b` instead of hardcoded `255, 255, 255`, and add the canvas guard:

```rust
let (mx, my) = mouse_position();

if is_mouse_button_down(MouseButton::Left) && my < canvas_h {  // ← guard added
    let ev = DrawEvent { x: mx, y: my, r, g, b, size: 8, pen_down: true };
    local_strokes.push(ev);
    let _ = socket.send_to(&ev.to_bytes(), SERVER);
}
```

The colour is baked into each `DrawEvent`. When the other client receives and draws it, they draw it in the colour embedded in the event — no extra negotiation needed.

---

## Step 4 — Handle palette clicks

Palette clicks are detected with `is_mouse_button_pressed` (fires once per click, not held), checked separately from the stroke-producing mouse handler:

```rust
if is_mouse_button_pressed(MouseButton::Left) {
    let (mpx, mpy) = mouse_position();
    for (i, _) in PALETTE.iter().enumerate() {
        let px = 10.0 + i as f32 * 50.0;
        if mpx >= px && mpx < px + 40.0 && mpy >= palette_y {
            color_idx = i;
        }
    }
}
```

`is_mouse_button_pressed` vs `is_mouse_button_down`:
- `_down` — `true` every frame while held (60+ times per second)
- `_pressed` — `true` only on the single frame the button first goes down

Using `_down` for palette selection would fire 60 times on a slow click and would also land in the stroke handler above (though the `my < canvas_h` guard catches that). Using `_pressed` is cleaner — one click, one selection.

---

## Step 5 — Draw palette buttons

In the render section, after drawing strokes and before `next_frame`:

```rust
// separator between canvas and palette
draw_line(0.0, canvas_h, screen_width(), canvas_h, 1.0, DARKGRAY);

// colour buttons
for (i, &(pr, pg, pb)) in PALETTE.iter().enumerate() {
    let px = 10.0 + i as f32 * 50.0;
    draw_rectangle(px, palette_y, 40.0, 40.0, Color::from_rgba(pr, pg, pb, 255));
    if color_idx == i {
        draw_rectangle_lines(px - 2.0, palette_y - 2.0, 44.0, 44.0, 3.0, WHITE);
    }
}
```

Each button is a 40×40 filled rectangle. The selected button gets a 3-pixel white outline drawn 2 pixels outside its bounds — inside would obscure the colour itself.

---

## Full client

```rust
use macroquad::prelude::*;
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

    let mut local_strokes:  Vec<DrawEvent> = Vec::new();
    let mut remote_strokes: Vec<DrawEvent> = Vec::new();
    let mut connected  = false;
    let mut color_idx: usize = 0;
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
                        connected = true;
                        remote_strokes.push(ev);
                    }
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => break,
                Err(_) => break,
            }
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

        // ── stroke input ──────────────────────────────────────────────────
        let (mx, my) = mouse_position();
        if is_mouse_button_down(MouseButton::Left) && my < canvas_h {
            let ev = DrawEvent { x: mx, y: my, r, g, b, size: 8, pen_down: true };
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

        draw_line(0.0, canvas_h, screen_width(), canvas_h, 1.0, DARKGRAY);

        for (i, &(pr, pg, pb)) in PALETTE.iter().enumerate() {
            let px = 10.0 + i as f32 * 50.0;
            draw_rectangle(px, palette_y, 40.0, 40.0, Color::from_rgba(pr, pg, pb, 255));
            if color_idx == i {
                draw_rectangle_lines(px - 2.0, palette_y - 2.0, 44.0, 44.0, 3.0, WHITE);
            }
        }

        let (label, color) = if connected { ("● LIVE", GREEN) } else { ("○ waiting...", DARKGRAY) };
        draw_text(label, screen_width() - 140.0, 24.0, 20.0, color);

        next_frame().await;
    }
}
```

Run two clients against the server. Each player picks a different colour. Strokes appear in the remote player's chosen colour automatically, because the colour is embedded in every `DrawEvent`.

---

## Exercise

> **TODO 1**: Add keyboard shortcuts so number keys `1`–`8` select palette colours. `is_key_pressed(KeyCode::Key1)` maps to index 0, `Key2` to index 1, and so on.
>
> **TODO 2**: The current palette uses one fixed colour per slot. Allow the user to increase or decrease the brightness of the selected colour by pressing `+` / `-`. Multiply `r`, `g`, `b` by a brightness factor clamped to 0.2–2.0.
>
> **TODO 3**: What happens when one client picks near-black (slot 2, `(20, 20, 20)`) and draws on the dark background? Is it visible? How would you prevent picking a colour that is too close to the background colour?

---

## Key APIs

| API | What it does |
|-----|-------------|
| `is_mouse_button_pressed(Left)` | `true` only on the frame the button first goes down |
| `draw_rectangle(x, y, w, h, color)` | Filled rectangle |
| `draw_rectangle_lines(x, y, w, h, t, color)` | Rectangle outline |
| `draw_line(x1, y1, x2, y2, t, color)` | Line segment |
