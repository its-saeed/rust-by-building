# Lesson 5 — Spawning

> **Goal**: Click anywhere on screen to spawn a new ball at the mouse position.
>
> **Concepts**: `Vec::push` at runtime, reading mouse input, creating values from runtime data.

---

## Mouse input in macroquad

macroquad provides two functions for mouse input:

```rust
mouse_position()                          // returns (f32, f32) — current cursor position
is_mouse_button_pressed(MouseButton::Left) // true on the frame the button is first pressed
```

`is_mouse_button_pressed` returns `true` for exactly one frame — the frame the button goes down. That's what you want for spawning: one click, one ball.

---

## Spawning a body

```rust
if is_mouse_button_pressed(MouseButton::Left) {
    let (mx, my) = mouse_position();
    world.add_body(Body::new(
        Vec2::new(mx, my),
        Vec2::new(0.0, 0.0),
        20.0,
    ));
}
```

`mouse_position()` returns a tuple `(f32, f32)`. Destructuring with `let (mx, my) = ...` unpacks both values at once.

The new body starts at the cursor with zero velocity. Gravity takes over immediately — it falls from wherever you clicked.

---

## Where in the loop

Check for input **before** stepping:

```rust
loop {
    let dt = get_frame_time();
    clear_background(BLACK);

    if is_mouse_button_pressed(MouseButton::Left) {
        let (mx, my) = mouse_position();
        world.add_body(Body::new(Vec2::new(mx, my), Vec2::new(0.0, 0.0), 20.0));
    }

    world.step(dt);
    world.draw_all();

    next_frame().await;
}
```

`world.add_body` calls `Vec::push` under the hood — the Vec grows by one element. There's no limit you need to manage; the Vec allocates more memory automatically as needed.

---

## Your task

Open `lessons/5-many-bodies/lesson-05/project/src/main.rs`.

1. In the game loop, check `is_mouse_button_pressed(MouseButton::Left)`.
2. If pressed, get `mouse_position()` and call `world.add_body(...)` with a new `Body` at that position.

Run it and click around the screen. Each click drops a new ball that falls under gravity and bounces off the walls.

You now have an interactive physics simulation.
