# Lesson 8 — Ball Trail

> **Goal**: Draw a fading comet tail behind the ball by recording its recent positions and drawing them with decreasing size and transparency.
>
> **Concepts**: `VecDeque`, `push_back`, `pop_front`, `Color` with alpha, drawing order.

---

## What we're building

Right now the ball is a single sprite that teleports from frame to frame — there is no visual sense of direction or speed. A motion trail fixes this: a short history of where the ball just was, drawn as fading circles behind it.

The effect is simple: the most recent position is large and bright, the oldest is tiny and nearly invisible, and everything in between fades smoothly.

---

## `VecDeque` — a queue that grows and shrinks from both ends

A trail is a **fixed-length sliding window** of positions: every frame you add the current position at one end and remove the oldest at the other. This is exactly what `VecDeque` (double-ended queue, pronounced "deck") is for.

```rust
use std::collections::VecDeque;

let mut trail: VecDeque<Vec2> = VecDeque::new();

trail.push_back(Vec2::new(1.0, 2.0));  // add to the back
trail.push_back(Vec2::new(3.0, 4.0));

trail.pop_front();  // remove from the front (oldest out)
```

You could use a `Vec` and call `.remove(0)` to drop the oldest, but `Vec::remove(0)` shifts every element left — O(n). `VecDeque::pop_front` is O(1) because it just moves an index. For a trail of 12 points the difference is negligible, but `VecDeque` is the idiomatically correct choice when the access pattern is "add to one end, remove from the other."

---

## Adding the trail field

Add the import at the top of the file:

```rust
use std::collections::VecDeque;
```

Add a constant for how many positions to keep:

```rust
const TRAIL_LEN: usize = 12;
```

Add the field to `Ball`:

```rust
struct Ball<'a> {
    rect:    Rect,
    vel:     Vec2,
    texture: &'a Texture2D,
    trail:   VecDeque<Vec2>,
}
```

Initialize it in `Ball::new`:

```rust
fn new(texture: &'a Texture2D) -> Self {
    Ball {
        rect: Rect::new(...),
        vel:  Vec2::new(300.0, 220.0),
        texture,
        trail: VecDeque::new(),
    }
}
```

---

## Recording positions each frame

At the start of `Ball::update`, before moving the ball, record where it is now:

```rust
fn update(&mut self, dt: f32) {
    self.trail.push_back(self.rect.center());
    if self.trail.len() > TRAIL_LEN {
        self.trail.pop_front();
    }

    self.rect.x += self.vel.x * dt;
    // ... rest of update
}
```

The trail fills up gradually. After 12 frames it is full, and from then on every push displaces the oldest position. `self.rect.center()` gives the midpoint of the ball's rect — the natural reference point for drawing circles.

---

## Drawing the trail

In `Ball::draw`, draw the trail *before* the ball sprite so the sprite sits on top:

```rust
fn draw(&self) {
    let len = self.trail.len();
    for (i, &pos) in self.trail.iter().enumerate() {
        let t = i as f32 / len as f32;   // 0.0 = oldest, 1.0 = newest
        let alpha  = t * 0.6;
        let radius = t * BALL_SIZE * 0.5;
        draw_circle(pos.x, pos.y, radius, Color::new(1.0, 1.0, 1.0, alpha));
    }

    draw_texture_ex(
        self.texture,
        self.rect.x, self.rect.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(self.rect.w, self.rect.h)),
            ..Default::default()
        },
    );
}
```

`t` is a ratio from 0 (oldest trail point) to 1 (most recent). Both `alpha` and `radius` scale with `t`, so the oldest dot is invisible and tiny, and the newest is solid and half the ball's size.

`Color::new(r, g, b, a)` takes floats in `[0.0, 1.0]`. Alpha 0.0 is fully transparent, 1.0 is fully opaque. The trail peaks at 0.6 opacity so the ball sprite remains clearly dominant.

### Drawing order matters

macroquad draws in call order, with no depth buffer by default. Anything drawn earlier appears underneath anything drawn later. By drawing trail circles first and the sprite second, the sprite always sits on top — even where a trail circle overlaps the ball's current position.

---

## Clearing on reset

When a point is scored, `ball.reset()` teleports the ball to the center. Without clearing the trail, you would see a cluster of ghost circles at the old position for the next 12 frames. Clear it at the start of `reset`:

```rust
fn reset(&mut self) {
    self.trail.clear();
    self.rect.x = WINDOW_W / 2.0 - BALL_SIZE / 2.0;
    // ...
}
```

---

## Your task

Open `lessons/7-pong/lesson-08/project/src/main.rs`.

1. Add `use std::collections::VecDeque;` at the top.
2. Add `const TRAIL_LEN: usize = 12;`.
3. Add `trail: VecDeque<Vec2>` to `struct Ball` and initialize it with `VecDeque::new()` in `Ball::new`.
4. At the start of `Ball::update`, push the current center and pop the front if over `TRAIL_LEN`.
5. In `Ball::draw`, iterate `self.trail` and draw fading circles before the sprite.
6. In `Ball::reset`, call `self.trail.clear()`.

```sh
cargo run --bin pong-08
```

Move the ball — it should leave a short, fading comet trail. A point scored should clear the trail instantly.
