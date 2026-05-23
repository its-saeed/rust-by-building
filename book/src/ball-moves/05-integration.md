# Lesson 5 — Integration

> **Goal**: Make the ball actually move by advancing its position each frame based on its velocity.
>
> **Concepts**: Euler integration, delta time, `&mut self`, `get_frame_time`.

---

## Velocity and position

**Position** is where the ball is right now — a point in space.

**Velocity** is how fast and in which direction the ball is moving — a direction with a magnitude, measured in pixels per second.

Each frame we advance position by velocity, scaled by how much time passed:

```
position = position + velocity * dt
```

`dt` is **delta time** — the duration of the previous frame in seconds. On a 60 Hz display, `dt` is about `0.016` (one-sixtieth of a second).

This is called **Euler integration** — the simplest way to simulate continuous motion as discrete steps.

---

## Why delta time matters

Without `dt`, your ball moves faster on a 120 Hz monitor than on a 60 Hz one, because the loop runs more often. Multiplying velocity by `dt` makes movement **frame-rate independent**: a ball with velocity `(200.0, 0.0)` moves exactly 200 pixels per second regardless of frame rate.

macroquad gives you `dt` via:

```rust
let dt = get_frame_time();
```

Call this once at the start of each frame, before you update anything.

---

## The `update` method

The `update` method advances the body by one time step. It needs to **change** `position`, so it takes `&mut self`:

```rust
impl Body {
    fn update(&mut self, dt: f32) {
        self.position = self.position + self.velocity * dt;
    }
}
```

`self.position + self.velocity * dt` uses the operators you implemented in lesson 3:
- `self.velocity * dt` calls `Mul<f32>` → scales the velocity vector
- `+ self.position` calls `Add` → advances the position

Because `Vec2` is `Copy`, both reads of `self.position` and `self.velocity` are copies — no borrow conflict.

---

## `&mut self` at the call site

When you call a `&mut self` method, Rust requires `mut` on the variable:

```rust
let mut ball = Body::new(...);
ball.update(dt);  // only works because ball is `mut`
```

Without `mut`, the compiler will tell you the variable needs to be mutable. This is intentional — Rust makes mutation explicit so you always know which variables can change.

---

## Putting it together

The game loop now looks like:

```rust
loop {
    let dt = get_frame_time();
    clear_background(BLACK);

    ball.update(dt);
    ball.draw();

    next_frame().await;
}
```

Order matters: update first (move the ball), then draw (show where it ended up).

---

## Your task

Open `lessons/4-ball-moves/lesson-05/project/src/main.rs`.

You'll find `Body` already defined with `new` and `draw`. Add:

1. `fn update(&mut self, dt: f32)` to the `impl Body` block — advance `position` by `velocity * dt`.
2. In `main`, change `ball` to `let mut ball` so you can call `update` on it.
3. Give the ball a non-zero starting velocity — try `Vec2::new(150.0, 100.0)`.
4. Call `ball.update(dt)` inside the loop before `ball.draw()`.

Run it. The ball moves — and flies off the screen. We'll fix that in the next lesson.
