# Lesson 3 — The Ball

> **Goal**: Make the ball move and bounce off the top and bottom walls.
>
> **Concepts**: macroquad's built-in `Vec2`, Euler integration revisited, wall bounce logic.

---

## Velocity as a `Vec2`

The ball needs a direction and a speed — a velocity vector. In projects 4 and 5 we built `Vec2` from scratch. Here we use macroquad's built-in version, already imported via `use macroquad::prelude::*`:

```rust
let mut vel = Vec2::new(300.0, 220.0); // pixels per second
```

macroquad's `Vec2` supports all the same operations as the one you built — addition, multiplication by scalar, negation — but it's fully featured (dot product, length, normalize, and more) and requires no code from us.

---

## Moving the ball

Each frame, advance the ball's position by velocity × dt. `Rect` doesn't support vector arithmetic directly, so we update `x` and `y` separately:

```rust
ball.x += vel.x * dt;
ball.y += vel.y * dt;
```

This is the same Euler integration from project 4 — sample velocity, assume it's constant for this step, advance position. At 60 FPS the step is 0.016 s, small enough that the ball's path looks smooth.

---

## Bouncing off the top and bottom walls

When the ball hits a horizontal wall, flip its vertical velocity. Also correct the position so the ball doesn't overlap the wall:

```rust
if ball.y < 0.0 {
    ball.y = 0.0;
    vel.y = vel.y.abs(); // force downward
}
if ball.y + ball.h > WINDOW_H {
    ball.y = WINDOW_H - ball.h;
    vel.y = -vel.y.abs(); // force upward
}
```

Using `.abs()` and `-.abs()` instead of just `vel.y *= -1.0` is safer: if the ball somehow ends up multiple pixels past the wall in one frame, flipping the sign won't help if the check fires twice — it would flip back. `.abs()` always pushes the velocity in the correct direction regardless of how many times the check runs.

---

## Bouncing off the left and right walls (temporary)

For now, also bounce off the left and right edges. This keeps the ball visible while we build up to proper scoring:

```rust
if ball.x < 0.0 {
    ball.x = 0.0;
    vel.x = vel.x.abs();
}
if ball.x + ball.w > WINDOW_W {
    ball.x = WINDOW_W - ball.w;
    vel.x = -vel.x.abs();
}
```

In lesson 5 these two checks will be replaced with score logic — when the ball exits left or right, a point is awarded instead.

---

## Update order

Always update state before drawing. The loop should look like:

```rust
loop {
    let dt = get_frame_time();

    // 1. input
    // 2. move paddles, move ball
    // 3. wall bounces
    clear_background(BLACK);
    // 4. draw everything
    next_frame().await;
}
```

Clearing the screen comes after all updates and before all draws. Everything drawn between `clear_background` and `next_frame` appears in the same frame.

---

## Your task

Open `lessons/7-pong/lesson-03/project/src/main.rs`.

1. Add `let mut vel = Vec2::new(300.0, 220.0);` after the ball `Rect`.
2. Inside the loop, after paddle movement, add the two lines that advance `ball.x` and `ball.y`.
3. Add the four wall-bounce blocks (top, bottom, left, right).

```sh
cargo run --bin pong-03
```

The ball should bounce around all four walls while both paddles remain controllable. The ball passes straight through the paddles — collision detection is next.
