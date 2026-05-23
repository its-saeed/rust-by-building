# Lesson 6 — Wall Bouncing

> **Goal**: Keep the ball inside the screen by detecting edge collisions and reflecting its velocity.
>
> **Concepts**: `screen_width`, `screen_height`, boundary conditions, velocity reflection, position correction.

---

## The screen edges

The window is `screen_width()` pixels wide and `screen_height()` pixels tall. The origin is the top-left corner.

A circle with radius `r` at position `(x, y)`:

- hits the **left** wall when `x - r < 0.0`
- hits the **right** wall when `x + r > screen_width()`
- hits the **top** wall when `y - r < 0.0`
- hits the **bottom** wall when `y + r > screen_height()`

---

## Reflecting velocity

When a ball hits a wall, we flip the velocity component that points toward that wall:

- Hit the left or right wall → flip `velocity.x`
- Hit the top or bottom wall → flip `velocity.y`

```rust
self.velocity.x = -self.velocity.x;
```

This reverses only the horizontal component, leaving the vertical unchanged — giving the correct bounce direction.

---

## Position correction

Flipping velocity alone isn't quite enough. If the ball moves fast, it can penetrate the wall before we detect the collision. Next frame it's still inside the wall, gets flipped again, and sticks. We fix this by also **correcting the position** back to the boundary:

```rust
if self.position.x - self.radius < 0.0 {
    self.position.x = self.radius;      // push back to the edge
    self.velocity.x = -self.velocity.x; // reflect
}
```

Setting `self.position.x = self.radius` places the ball's left edge exactly on the wall.

---

## The full `keep_in_bounds` method

```rust
impl Body {
    fn keep_in_bounds(&mut self) {
        let w = screen_width();
        let h = screen_height();

        if self.position.x - self.radius < 0.0 {
            self.position.x = self.radius;
            self.velocity.x = -self.velocity.x;
        }
        if self.position.x + self.radius > w {
            self.position.x = w - self.radius;
            self.velocity.x = -self.velocity.x;
        }
        if self.position.y - self.radius < 0.0 {
            self.position.y = self.radius;
            self.velocity.y = -self.velocity.y;
        }
        if self.position.y + self.radius > h {
            self.position.y = h - self.radius;
            self.velocity.y = -self.velocity.y;
        }
    }
}
```

Four independent checks — one per edge. Each one corrects position and reflects the relevant velocity component.

---

## The complete game loop

```rust
loop {
    let dt = get_frame_time();
    clear_background(BLACK);

    ball.update(dt);
    ball.keep_in_bounds();
    ball.draw();

    next_frame().await;
}
```

Order: move, then clamp, then draw. `keep_in_bounds` runs after `update` so it always sees the new position.

---

## Your task

Open `lessons/4-ball-moves/lesson-06/project/src/main.rs`.

You'll find `Body` with `new`, `update`, and `draw` already in place. Add:

1. `fn keep_in_bounds(&mut self)` to the `impl Body` block — handle all four edges.
2. Call `ball.keep_in_bounds()` in the loop, between `update` and `draw`.

Run it. The ball should bounce indefinitely around the screen.

Try changing the starting velocity — steeper angles, faster speeds. Notice that the ball always stays perfectly inside the window, no matter how fast it moves.

**That's project 4 complete.** You have a ball that moves, bounces, and never escapes. The building blocks — `Vec2`, operators, `Body`, integration, boundaries — are the same ones the rest of the engine will be built on.
