# Lesson 3 — Basic Canvas

Get drawing working locally with the least possible code. One colour, one brush size, no palette, no networking. The goal is a window you can scribble on — everything else is added in later lessons.

---

## Before the loop — declarations

```rust
let mut local_strokes:  Vec<DrawEvent> = Vec::new();
let mut remote_strokes: Vec<DrawEvent> = Vec::new();
```

`local_strokes` holds every stroke you draw. `remote_strokes` holds strokes received from other players — it stays empty until lesson 4, but declaring it now means the render code does not need to change when networking arrives.

Both are `Vec<DrawEvent>`. The canvas is not a pixel buffer — it is a list of events replayed every frame. Adding a stroke means pushing to the vec; clearing the canvas means clearing the vec. No render targets, no image API needed.

---

## Inside the loop — mouse input

At the top of every frame, check whether the mouse button is held:

```rust
let (mx, my) = mouse_position();

if is_mouse_button_down(MouseButton::Left) {
    let ev = DrawEvent {
        x: mx, y: my,
        r: 255, g: 255, b: 255,   // white stroke
        size: 8,
        pen_down: true,
    };
    local_strokes.push(ev);
}
```

`is_mouse_button_down` is `true` every frame the button is held. At 60 fps, dragging the mouse records ~60 events per second. When drawn as overlapping circles they form a continuous stroke.

Colour is fixed at white for now. Brush size is fixed at 8 px. Both become variables in lesson 5 and 6.

---

## Inside the loop — render

After input, clear the window and replay all events:

```rust
clear_background(Color::from_rgba(30, 30, 35, 255));

for ev in local_strokes.iter().filter(|e| e.pen_down) {
    draw_circle(ev.x, ev.y, ev.size as f32, Color::from_rgba(ev.r, ev.g, ev.b, 255));
}
for ev in remote_strokes.iter().filter(|e| e.pen_down) {
    draw_circle(ev.x, ev.y, ev.size as f32, Color::from_rgba(ev.r, ev.g, ev.b, 200));
}
```

`clear_background` redraws the whole window each frame — macroquad does not retain what was drawn last frame.

The `.filter(|e| e.pen_down)` guard is not needed yet (all events have `pen_down: true`), but it will matter once clear-canvas events appear in lesson 6.

Remote strokes are drawn at alpha 200 — slightly translucent — so you can always tell whose is whose when colours overlap. For now both lists draw in white at different opacities, which is already distinguishable.

---

## Full client

```rust
use macroquad::prelude::*;
use tele_sketch::event::DrawEvent;

#[macroquad::main("Tele-Sketch")]
async fn main() {
    let mut local_strokes:  Vec<DrawEvent> = Vec::new();
    let mut remote_strokes: Vec<DrawEvent> = Vec::new();

    loop {
        // ── input ─────────────────────────────────────────────────────────
        let (mx, my) = mouse_position();

        if is_mouse_button_down(MouseButton::Left) {
            let ev = DrawEvent {
                x: mx, y: my,
                r: 255, g: 255, b: 255,
                size: 8,
                pen_down: true,
            };
            local_strokes.push(ev);
        }

        // ── render ────────────────────────────────────────────────────────
        clear_background(Color::from_rgba(30, 30, 35, 255));

        for ev in local_strokes.iter().filter(|e| e.pen_down) {
            draw_circle(ev.x, ev.y, ev.size as f32, Color::from_rgba(ev.r, ev.g, ev.b, 255));
        }
        for ev in remote_strokes.iter().filter(|e| e.pen_down) {
            draw_circle(ev.x, ev.y, ev.size as f32, Color::from_rgba(ev.r, ev.g, ev.b, 200));
        }

        next_frame().await;
    }
}
```

Run it: `cargo run --bin client`. Draw on the dark window. The strokes appear immediately and stay until you close the window.

---

## Exercise

> **TODO 1**: Change the stroke colour to red (`r: 220, g: 60, b: 60`). Run it. Then switch back to white. Notice that the colour is baked into each `DrawEvent` — changing it only affects new events, not existing ones.
>
> **TODO 2**: Change `size: 8` to `size: 20`. Run it. Then try `size: 2`. You are changing the brush radius in pixels — this becomes a scroll-wheel control in lesson 6.
>
> **TODO 3**: Add `is_key_pressed(KeyCode::C)` to clear `local_strokes` when `C` is pressed. (`remote_strokes` stays — you can only erase your own side for now.)

---

## Key APIs

| API | What it does |
|-----|-------------|
| `mouse_position()` | Current cursor as `(f32, f32)` |
| `is_mouse_button_down(Left)` | `true` every frame the button is held |
| `draw_circle(x, y, r, color)` | Filled circle |
| `clear_background(color)` | Fill the whole window — must call every frame |
| `Color::from_rgba(r, g, b, a)` | Colour with alpha (0–255) |
