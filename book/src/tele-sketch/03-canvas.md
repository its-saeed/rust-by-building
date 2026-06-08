# Lesson 3 — Local Canvas

Before adding any networking to the client, get drawing working locally. A canvas that responds to your mouse is the foundation — wiring it to the server in the next lesson is just one extra line per event.

---

## Step 1 — Why a list of events, not a pixel buffer

The most obvious way to store a drawing is a 2D array of pixels — a bitmap. macroquad does support render textures (offscreen pixel buffers), but using one here would require learning a new API just to implement "draw a circle."

Instead the canvas is two `Vec<DrawEvent>` lists:

```rust
let mut local_strokes:  Vec<DrawEvent> = Vec::new();
let mut remote_strokes: Vec<DrawEvent> = Vec::new();
```

Every frame, **replay** all events by drawing a circle at each recorded position. New event? Push it. Clear the canvas? Clear the vec. Display? Loop through and draw.

The downside: rendering cost grows as strokes accumulate. For a classroom demo lasting 10–20 minutes, this is fine. A production app would bake finished strokes into a texture — lesson 5 discusses that trade-off.

Two separate lists — `local_strokes` and `remote_strokes` — keep your strokes visually distinct from the remote player's and make the clear-canvas feature in lesson 5 straightforward: clearing your canvas should not erase what the other person drew.

---

## Step 2 — Reserving space for the palette

The bottom 60 pixels of the window belong to the palette. The canvas occupies everything above it:

```rust
let canvas_h = screen_height() - 60.0;
```

This value is used in two places: to guard mouse input (clicks below `canvas_h` are palette clicks, not strokes) and to draw the separator line. Compute it once at the top of each frame from `screen_height()` so it adapts if the window is resized.

---

## Step 3 — Mouse input

```rust
let (mx, my) = mouse_position();

if is_mouse_button_down(MouseButton::Left) && my < canvas_h {
    let ev = DrawEvent {
        x:        mx,
        y:        my,
        r:        current_r,
        g:        current_g,
        b:        current_b,
        size:     brush_size,
        pen_down: true,
    };
    local_strokes.push(ev);
}
```

`is_mouse_button_down` returns `true` every frame the button is held. So holding the mouse button while moving produces a new event each frame — at 60 fps that is 60 events per second. When drawn as overlapping circles, consecutive events are close enough together that they appear as a continuous stroke, even during fast movement.

The guard `my < canvas_h` prevents stroke events from being recorded when clicking the palette buttons.

At this point, only local drawing works. The sent events do not exist yet — that comes in lesson 4.

---

## Step 4 — The colour palette

Eight constant colours at the bottom of the window:

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
```

A `usize` index tracks the currently selected colour:

```rust
let mut color_idx: usize = 0;
```

Each frame, extract the active colour from the palette before the mouse-input block:

```rust
let (r, g, b) = PALETTE[color_idx];
```

Then `r`, `g`, `b` are used when creating `DrawEvent`s. The colour is baked into each event — which is why remote strokes display in the remote player's colour automatically, without any extra metadata.

---

## Step 5 — Drawing palette buttons

```rust
let palette_y = screen_height() - 52.0;

for (i, &(pr, pg, pb)) in PALETTE.iter().enumerate() {
    let px    = 10.0 + i as f32 * 50.0;
    let color = Color::from_rgba(pr, pg, pb, 255);

    draw_rectangle(px, palette_y, 40.0, 40.0, color);

    if color_idx == i {
        draw_rectangle_lines(px - 2.0, palette_y - 2.0, 44.0, 44.0, 3.0, WHITE);
    }
}
```

Each button is a 40×40 filled rectangle. The selected one gets a 3-pixel white outline, drawn slightly outside the button bounds so the outline does not cover the colour itself.

---

## Step 6 — Handling palette clicks

```rust
if is_mouse_button_pressed(MouseButton::Left) {
    let (mpx, mpy) = mouse_position();
    if mpx >= px && mpx < px + 40.0 && mpy >= palette_y {
        color_idx = i;
    }
}
```

Notice `is_mouse_button_pressed`, not `is_mouse_button_down`. The difference matters:

- `_down` — `true` every frame the button is held (60 times per second while held)
- `_pressed` — `true` only on the single frame the button first goes down

Using `_down` for palette selection would work visually, but it would also create a stroke event on that same frame (the mouse input block runs in the same loop). Using `_pressed` fires once, selects the colour, and that is it.

The hit test is also checking `mpy >= palette_y` rather than `mpy >= palette_y && mpy < screen_height()`. Because the canvas guard already uses `my < canvas_h`, any click in the palette area is guaranteed not to produce a stroke.

---

## Step 7 — Scroll-wheel brush size

```rust
let mut brush_size: u8 = 8;

// each frame:
let scroll = mouse_wheel().1;
if scroll > 0.0 { brush_size = (brush_size + 2).min(40); }
if scroll < 0.0 { brush_size = brush_size.saturating_sub(2).max(2); }
```

`mouse_wheel()` returns `(horizontal_delta, vertical_delta)`. Positive vertical = scroll up = larger brush. Negative = scroll down = smaller brush.

`saturating_sub` prevents underflow — subtracting from a `u8` that is already 0 would wrap to 255. It saturates at 0 instead. Then `.max(2)` sets the effective minimum to 2 pixels.

Brush size is stored as a `u8` because it lives in `DrawEvent.size`. Keeping it the same type means no conversion when constructing events.

---

## Step 8 — Rendering the canvas

After handling all input, clear and redraw everything:

```rust
clear_background(Color::from_rgba(30, 30, 35, 255));

for ev in local_strokes.iter().filter(|e| e.pen_down) {
    draw_circle(ev.x, ev.y, ev.size as f32, Color::from_rgba(ev.r, ev.g, ev.b, 255));
}
for ev in remote_strokes.iter().filter(|e| e.pen_down) {
    draw_circle(ev.x, ev.y, ev.size as f32, Color::from_rgba(ev.r, ev.g, ev.b, 200));
}
```

Two passes: local strokes at full opacity (alpha 255), remote strokes slightly translucent (alpha 200). The difference is subtle but lets you visually separate contributions — your strokes look crisp; the other person's are lightly washed. When colours overlap, the partial transparency lets both show through.

`.filter(|e| e.pen_down)` skips any events where the pen was lifted. Right now all pushed events have `pen_down: true`, so the filter does nothing — but when the clear feature arrives in lesson 5, lift events may appear in the list and you want to skip them.

---

## Step 9 — Palette separator and brush preview

Draw a line between the canvas and the palette to make the boundary visible:

```rust
draw_line(0.0, canvas_h, screen_width(), canvas_h, 1.0, DARKGRAY);
```

Draw a circle at the right end of the palette row showing the current brush size in the current colour:

```rust
draw_circle(
    screen_width() - 50.0,
    screen_height() - 30.0,
    brush_size as f32,
    Color::from_rgba(r, g, b, 255),
);
```

This gives immediate feedback when scrolling — you see the brush grow or shrink in real time, at the actual radius, in the colour you are about to use.

---

## Frame order

The order of operations inside the loop matters:

```
1. extract active colour from palette
2. handle mouse input → push to local_strokes
3. handle palette clicks → update color_idx
4. handle scroll → update brush_size
5. clear_background
6. draw local_strokes
7. draw remote_strokes
8. draw separator line + palette buttons + brush preview
9. next_frame().await
```

Input before drawing: events added this frame are visible this frame. Drawing before UI overlays: strokes appear behind the palette, not on top of it.

---

## Exercise

> **TODO 1**: At the moment, `color_idx` is set by palette clicks that happen anywhere in the `for` loop over PALETTE. But there is a subtle problem: the hit test inside the loop runs the same check for every button. Explain why this still works correctly even though the check does not `break` out of the loop after finding a match.
>
> **TODO 2**: Add a keyboard shortcut: number keys `1`–`8` select palette colours directly. `is_key_pressed(KeyCode::Key1)` through `KeyCode::Key8`.
>
> **TODO 3**: The brush preview circle at the bottom right can overlap the rightmost palette button if the window is too narrow. Add a check: if `screen_width() - 50.0 < 10.0 + 8.0 * 50.0 + 40.0`, skip drawing the preview.

---

## Key APIs

| API | What it does |
|-----|-------------|
| `mouse_position()` | Current cursor as `(f32, f32)` |
| `is_mouse_button_down(Left)` | `true` every frame the button is held |
| `is_mouse_button_pressed(Left)` | `true` only on the frame the button first goes down |
| `mouse_wheel()` | `(horizontal, vertical)` scroll delta this frame |
| `u8::saturating_sub(n)` | Subtract without underflowing — stops at 0 |
| `draw_circle(x, y, r, color)` | Filled circle |
| `draw_rectangle(x, y, w, h, color)` | Filled rectangle |
| `draw_rectangle_lines(x, y, w, h, t, color)` | Rectangle outline |
| `draw_line(x1, y1, x2, y2, t, color)` | Line segment |
| `Color::from_rgba(r, g, b, a)` | Colour with alpha (0–255) |
