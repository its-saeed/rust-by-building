# Lesson 3 — Local Canvas

Before adding any networking to the client, get drawing working locally. A canvas that responds to your mouse is the foundation — wiring it to the server in the next lesson is just one extra line per event.

The client is a macroquad program: one `async fn main` decorated with `#[macroquad::main]`, and a loop body that runs every frame. Some state is declared once before the loop; everything else happens inside it.

---

## Before the loop — declarations

### Canvas storage

The most obvious way to store a drawing is a 2D array of pixels — a bitmap. macroquad does support render textures (offscreen pixel buffers), but using one here would require learning a new API just to draw a circle.

Instead the canvas is two `Vec<DrawEvent>` lists:

```rust
let mut local_strokes:  Vec<DrawEvent> = Vec::new();
let mut remote_strokes: Vec<DrawEvent> = Vec::new();
```

Every frame, **replay** all events by drawing a circle at each recorded position. New event? Push it. Clear the canvas? Clear the vec. Display? Loop and draw.

Two separate lists keep your strokes visually distinct from the remote player's and make the clear-canvas feature in lesson 5 straightforward — clearing your canvas should not erase what the other person drew.

### Palette and brush

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

let mut color_idx:  usize = 0;   // currently selected palette entry
let mut brush_size: u8    = 8;   // brush radius in pixels
```

`color_idx` is a `usize` because it indexes directly into `PALETTE`. `brush_size` is a `u8` because it maps directly into `DrawEvent.size` — same type, no conversion.

---

## Inside the loop — top of frame

### Derived values

Compute these at the very top of the loop body, before anything else uses them:

```rust
let (r, g, b) = PALETTE[color_idx];
let canvas_h  = screen_height() - 60.0; // bottom 60 px reserved for palette
```

`canvas_h` is recomputed every frame from `screen_height()` so it stays correct if the window is resized. `(r, g, b)` is unpacked once here and reused in both the mouse handler and the render section.

### Mouse input

```rust
let (mx, my) = mouse_position();

if is_mouse_button_down(MouseButton::Left) && my < canvas_h {
    let ev = DrawEvent {
        x: mx, y: my,
        r, g, b,
        size: brush_size,
        pen_down: true,
    };
    local_strokes.push(ev);
}
```

`is_mouse_button_down` returns `true` every frame the button is held. At 60 fps, holding and dragging produces ~60 events per second. When drawn as overlapping circles they look like a continuous stroke.

The guard `my < canvas_h` prevents stroke events from being recorded when the user clicks a palette button. Networking is not wired up yet — `send_to` will be added in lesson 4 immediately after the `push`.

### Colour palette — clicks

Handle palette clicks in the same top-of-frame section, before rendering:

```rust
let palette_y = screen_height() - 52.0;

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

`is_mouse_button_pressed` fires once on the frame the button first goes down — unlike `_down`, which fires every frame while held. Use `_pressed` for UI clicks. If you used `_down` here, a slow click on a palette button would also spam draw events into `local_strokes` on every held frame.

### Scroll-wheel brush size

```rust
let scroll = mouse_wheel().1;
if scroll > 0.0 { brush_size = (brush_size + 2).min(40); }
if scroll < 0.0 { brush_size = brush_size.saturating_sub(2).max(2); }
```

`mouse_wheel()` returns `(horizontal_delta, vertical_delta)`. Positive vertical = scroll up = larger brush.

`saturating_sub` prevents underflow — subtracting 2 from a `u8` that is already 0 would wrap to 254. `.saturating_sub(2)` stops at 0, then `.max(2)` lifts the floor to 2 px.

---

## Inside the loop — render section

All rendering happens after input handling. macroquad redraws the whole window every frame, so the first call is always `clear_background`:

### Background

```rust
clear_background(Color::from_rgba(30, 30, 35, 255));
```

### Canvas strokes

```rust
for ev in local_strokes.iter().filter(|e| e.pen_down) {
    draw_circle(ev.x, ev.y, ev.size as f32, Color::from_rgba(ev.r, ev.g, ev.b, 255));
}
for ev in remote_strokes.iter().filter(|e| e.pen_down) {
    draw_circle(ev.x, ev.y, ev.size as f32, Color::from_rgba(ev.r, ev.g, ev.b, 200));
}
```

Two passes: local strokes at full opacity (alpha 255), remote strokes slightly translucent (alpha 200). The difference is subtle — your strokes look crisp; the other person's are lightly washed. When colours overlap, the partial transparency lets both show through.

`.filter(|e| e.pen_down)` skips lift events. Every event pushed so far has `pen_down: true`, so the filter does nothing right now — but when lesson 5 introduces the clear command, events with `pen_down: false` may appear and must be skipped.

### Palette separator line

```rust
draw_line(0.0, canvas_h, screen_width(), canvas_h, 1.0, DARKGRAY);
```

A thin line at `canvas_h` makes the boundary between canvas and palette visually explicit.

### Palette buttons

```rust
for (i, &(pr, pg, pb)) in PALETTE.iter().enumerate() {
    let px = 10.0 + i as f32 * 50.0;
    draw_rectangle(px, palette_y, 40.0, 40.0, Color::from_rgba(pr, pg, pb, 255));
    if color_idx == i {
        draw_rectangle_lines(px - 2.0, palette_y - 2.0, 44.0, 44.0, 3.0, WHITE);
    }
}
```

Each button is a 40×40 filled rectangle. The selected one gets a 3-pixel white outline drawn slightly outside its bounds so it does not obscure the colour.

### Brush preview

```rust
draw_circle(
    screen_width() - 50.0,
    screen_height() - 30.0,
    brush_size as f32,
    Color::from_rgba(r, g, b, 255),
);
```

A circle at the right end of the palette row showing the current brush radius in the current colour. Scroll the wheel and watch it grow or shrink in real time.

---

## Full client

```rust
use macroquad::prelude::*;
use tele_sketch::event::DrawEvent;

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
    let mut local_strokes:  Vec<DrawEvent> = Vec::new();
    let mut remote_strokes: Vec<DrawEvent> = Vec::new();
    let mut color_idx:  usize = 0;
    let mut brush_size: u8    = 8;

    loop {
        // ── derived values ────────────────────────────────────────────────
        let (r, g, b) = PALETTE[color_idx];
        let canvas_h  = screen_height() - 60.0;
        let palette_y = screen_height() - 52.0;

        // ── mouse input ───────────────────────────────────────────────────
        let (mx, my) = mouse_position();
        if is_mouse_button_down(MouseButton::Left) && my < canvas_h {
            let ev = DrawEvent { x: mx, y: my, r, g, b, size: brush_size, pen_down: true };
            local_strokes.push(ev);
            // lesson 4: add send_to here
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

        draw_circle(
            screen_width() - 50.0,
            screen_height() - 30.0,
            brush_size as f32,
            Color::from_rgba(r, g, b, 255),
        );

        next_frame().await;
    }
}
```

Run it: `cargo run --bin client`. Draw on the dark canvas. Switch colours. Scroll to resize the brush. The `remote_strokes` list stays empty until lesson 4 — that is fine, the render loop just draws nothing for it.

---

## Exercise

> **TODO 1**: The palette click loop iterates all 8 buttons every time and does not break early. Explain why this still selects the correct colour even without a `break`.
>
> **TODO 2**: Add keyboard shortcuts: number keys `1`–`8` select palette colours directly using `is_key_pressed(KeyCode::Key1)` through `KeyCode::Key8`.
>
> **TODO 3**: Press `C` to clear `local_strokes`. (`is_key_pressed(KeyCode::C)` — `remote_strokes` stays, since you can only clear your own side for now.)

---

## Key APIs

| API | What it does |
|-----|-------------|
| `mouse_position()` | Current cursor as `(f32, f32)` |
| `is_mouse_button_down(Left)` | `true` every frame the button is held |
| `is_mouse_button_pressed(Left)` | `true` only on the frame the button first goes down |
| `mouse_wheel()` | `(horizontal, vertical)` scroll delta this frame |
| `u8::saturating_sub(n)` | Subtract without underflowing — clamps to 0 |
| `draw_circle(x, y, r, color)` | Filled circle |
| `draw_rectangle(x, y, w, h, color)` | Filled rectangle |
| `draw_rectangle_lines(x, y, w, h, t, color)` | Rectangle outline |
| `draw_line(x1, y1, x2, y2, t, color)` | Line segment |
| `Color::from_rgba(r, g, b, a)` | Colour with alpha (0–255) |
