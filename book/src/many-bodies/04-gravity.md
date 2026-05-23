# Lesson 4 — Gravity

> **Goal**: Make bodies accelerate downward each frame, simulating gravity.
>
> **Concepts**: force accumulation, constants, velocity as accumulated acceleration.

---

## How gravity works in a physics engine

Gravity is a constant downward **acceleration** — it changes velocity, not position directly.

Each frame:

```
velocity.y += GRAVITY * dt
position   += velocity  * dt
```

Velocity grows downward over time. Position follows velocity. The ball speeds up as it falls, and slows down if it bounces upward — exactly like real gravity.

---

## Defining the constant

```rust
const GRAVITY: f32 = 500.0;
```

`const` declares a value that never changes. By convention, constants are written in `SCREAMING_SNAKE_CASE`. The type annotation (`f32`) is required for constants — Rust won't infer it.

`500.0` is in pixels per second squared. It gives a satisfying, game-like fall speed. You can tune it freely.

---

## Applying gravity in `Body::update`

Gravity is applied before the position update:

```rust
fn update(&mut self, dt: f32) {
    self.velocity.y += GRAVITY * dt;
    self.position = self.position + self.velocity * dt;
}
```

`self.velocity.y += GRAVITY * dt` — this is **field mutation**. You're not replacing the whole `velocity` vector, just its `y` component. Because `Vec2` is `Copy`, the `x` component is untouched.

The order matters: accumulate forces first, then advance position with the already-updated velocity.

---

## What you'll see

Balls now fall toward the floor and bounce back up. Because velocity accumulates, balls that bounce off the top edge have less upward velocity each time and eventually settle near the floor — exactly as you'd expect.

---

## Your task

Open `lessons/5-many-bodies/lesson-04/project/src/main.rs`.

1. Add `const GRAVITY: f32 = 500.0;` near the top of the file.
2. In `Body::update`, add `self.velocity.y += GRAVITY * dt;` before the position line.
3. Give your bodies small or zero initial vertical velocity — gravity will do the work.

Run it. Balls should fall, hit the floor, and bounce. Adjust `GRAVITY` to feel the difference between light and heavy gravity.
