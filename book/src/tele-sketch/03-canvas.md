# Lesson 3 — Local Canvas

Before adding any networking to the client, get drawing working locally. A canvas that responds to your mouse is the foundation — once that works, wiring it to the server is just plumbing.

---

## Canvas as a list of events

The canvas is not a pixel buffer or an image. It is a `Vec<DrawEvent>`:

```rust
let mut local_strokes:  Vec<DrawEvent> = Vec::new();
let mut remote_strokes: Vec<DrawEvent> = Vec::new();
```

Every frame, replay all events by drawing a circle at each recorded position. This is the simplest possible approach: no render targets, no image API, no state beyond the event list. The downside is that drawing cost grows with the number of strokes — fine for a classroom demo, something to fix in a production app.

Two separate lists keep your strokes visually distinct from remote ones and make the clear-canvas operation in lesson 5 straightforward.

---

## Mouse input

```rust
let (mx, my) = mouse_position();
let canvas_h  = screen_height() - 60.0; // bottom 60 px reserved for palette

if is_mouse_button_down(MouseButton::Left) && my < canvas_h {
    let ev = DrawEvent {
        x: mx, y: my,
        r: current_r, g: current_g, b: current_b,
        size: brush_size,
        pen_down: true,
    };
    local_strokes.push(ev);
}
```

`is_mouse_button_down` is true every frame the button is held. Each frame with the button down while inside the canvas area produces a new event at the current cursor position. At 60 fps, fast mouse movement produces ~60 events per second — enough to look like a continuous stroke when drawn as overlapping circles.

---

## Colour palette

Eight buttons along the bottom of the window. Each is a 40×40 rectangle. A white border on the selected colour.

```rust
const PALETTE: [(u8, u8, u8); 8] = [
    (255, 255, 255), // white
    (20,  20,  20),  // near-black
    (220, 60,  60),  // red
    (60,  200, 80),  // green
    (60,  110, 240), // blue
    (240, 165, 30),  // orange
    (200, 60,  200), // magenta
    (40,  200, 210), // cyan
];
```

```rust
let palette_y = screen_height() - 52.0;
for (i, &(r, g, b)) in PALETTE.iter().enumerate() {
    let px    = 10.0 + i as f32 * 50.0;
    let color = Color::from_rgba(r, g, b, 255);
    draw_rectangle(px, palette_y, 40.0, 40.0, color);

    if color_idx == i {
        draw_rectangle_lines(px - 2.0, palette_y - 2.0, 44.0, 44.0, 3.0, WHITE);
    }
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mpx, mpy) = mouse_position();
        if mpx >= px && mpx < px + 40.0 && mpy >= palette_y {
            color_idx = i;
        }
    }
}
```

`is_mouse_button_pressed` fires once on the frame the button goes down — unlike `_down` which fires every frame it is held. Use `_pressed` for UI clicks to avoid registering multiple selections.

---

## Drawing the canvas

```rust
clear_background(Color::from_rgba(30, 30, 35, 255));

for ev in local_strokes.iter().filter(|e| e.pen_down) {
    draw_circle(ev.x, ev.y, ev.size as f32, Color::from_rgba(ev.r, ev.g, ev.b, 255));
}
for ev in remote_strokes.iter().filter(|e| e.pen_down) {
    draw_circle(ev.x, ev.y, ev.size as f32, Color::from_rgba(ev.r, ev.g, ev.b, 200));
}
```

Remote strokes are drawn at alpha 200 instead of 255 — subtly translucent, so you can always tell whose strokes are whose even when colours overlap.

---

## Brush size preview

Draw a small circle next to the palette showing the current brush at the current colour:

```rust
let (r, g, b) = PALETTE[color_idx];
draw_circle(
    screen_width() - 50.0,
    screen_height() - 30.0,
    brush_size as f32,
    Color::from_rgba(r, g, b, 255),
);
```

---

## Exercise

> **TODO 1**: Implement scroll-wheel brush size control. `mouse_wheel()` returns `(horizontal, vertical)` — positive vertical means scroll up. Clamp size between 2 and 40.
>
> ```rust
> let scroll = mouse_wheel().1;
> if scroll > 0.0 { brush_size = (brush_size + 2).min(40); }
> if scroll < 0.0 { brush_size = brush_size.saturating_sub(2).max(2); }
> ```
>
> **TODO 2**: Draw a separator line between the canvas area and the palette row.
>
> **TODO 3**: Press `C` to clear `local_strokes`. (Remote strokes stay — you can only clear your own for now.)

---

## Key APIs

| API | What it does |
|-----|-------------|
| `mouse_position()` | Current cursor position as `(f32, f32)` |
| `is_mouse_button_down(Left)` | True every frame the button is held |
| `is_mouse_button_pressed(Left)` | True only on the frame the button first goes down |
| `mouse_wheel()` | `(horizontal, vertical)` scroll delta this frame |
| `draw_circle(x, y, r, color)` | Filled circle |
| `draw_rectangle(x, y, w, h, color)` | Filled rectangle |
| `draw_rectangle_lines(x, y, w, h, thickness, color)` | Rectangle outline |
| `Color::from_rgba(r, g, b, a)` | Construct a colour with alpha |
