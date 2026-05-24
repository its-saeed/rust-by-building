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

Balls now fall toward the floor and bounce back up. Because velocity accumulates, balls fall faster each frame — they accelerate, just like real gravity.

---

## Why do balls eventually stop bouncing?

You might notice that after many bounces, balls settle near the floor and stop — even though we flip velocity perfectly and add no friction. This seems wrong. Here's what's actually happening.

Our bounce reverses velocity exactly: `velocity.y = -velocity.y`. In theory, a ball should bounce to the same height forever. But two things work against this:

**1. Euler integration is an approximation.** We advance position in discrete steps. The formula `position += velocity * dt` isn't exact — it slightly overestimates or underestimates the true position depending on how velocity is changing. These tiny errors accumulate over many frames. In our case, gravity is always pulling down, and Euler integration is biased slightly toward energy loss rather than gain.

**2. Floating-point numbers aren't exact.** `f32` represents numbers in binary, and most decimal fractions (like `0.016`) can't be represented exactly. Each arithmetic operation introduces a rounding error at the last decimal place. After thousands of frames and millions of operations, these micro-errors add up.

The result: each bounce loses a tiny amount of energy, and eventually the losses outpace the ball's ability to escape gravity.

This isn't a bug — it's a known limitation of simple Euler integration with fixed timesteps. In Project 7 (Stable World) we'll introduce **restitution** — a bounce coefficient that gives us explicit control over energy loss per bounce. For now, the settling behaviour is expected and even looks physically plausible.

---

## Your task

Open `lessons/5-many-bodies/lesson-04/project/src/main.rs`.

1. Add `const GRAVITY: f32 = 500.0;` near the top of the file.
2. In `Body::update`, add `self.velocity.y += GRAVITY * dt;` before the position line.
3. Give your bodies small or zero initial vertical velocity — gravity will do the work.

Run it. Balls should fall, hit the floor, and bounce. Adjust `GRAVITY` to feel the difference between light and heavy gravity.
