# Lesson 3 — The Ball

> **Goal**: Add velocity to `Ball` and an `update` method that moves it and bounces it off the top and bottom walls.
>
> **Concepts**: adding a field to an existing struct, Euler integration revisited, wall bounce logic.

---

## Adding velocity to `Ball`

The ball needs a direction and a speed. Add a `vel` field:

```rust
struct Ball {
    rect: Rect,
    vel:  Vec2,   // pixels per second
}
```

macroquad's `Vec2` is already available from `use macroquad::prelude::*`. It supports addition, scalar multiplication, negation — the same operations as the `Vec2` you built in project 4. The difference: this one comes from the library, so we get it for free.

Update `Ball::new` to initialise the velocity:

```rust
impl Ball {
    fn new() -> Self {
        Ball {
            rect: Rect::new(
                WINDOW_W / 2.0 - BALL_SIZE / 2.0,
                WINDOW_H / 2.0 - BALL_SIZE / 2.0,
                BALL_SIZE,
                BALL_SIZE,
            ),
            vel: Vec2::new(300.0, 220.0),
        }
    }
}
```

`Vec2::new(300.0, 220.0)` means 300 px/s to the right and 220 px/s downward. The ball immediately starts moving when the game begins.

---

## Adding `Ball::update`

```rust
impl Ball {
    fn update(&mut self, dt: f32) {
        self.rect.x += self.vel.x * dt;
        self.rect.y += self.vel.y * dt;

        // bounce off top wall
        if self.rect.y < 0.0 {
            self.rect.y = 0.0;
            self.vel.y = self.vel.y.abs();
        }
        // bounce off bottom wall
        if self.rect.y + self.rect.h > WINDOW_H {
            self.rect.y = WINDOW_H - self.rect.h;
            self.vel.y = -self.vel.y.abs();
        }
        // bounce off left wall (temporary — replaced by scoring in lesson 5)
        if self.rect.x < 0.0 {
            self.rect.x = 0.0;
            self.vel.x = self.vel.x.abs();
        }
        // bounce off right wall (temporary)
        if self.rect.x + self.rect.w > WINDOW_W {
            self.rect.x = WINDOW_W - self.rect.w;
            self.vel.x = -self.vel.x.abs();
        }
    }
}
```

### Movement

```rust
self.rect.x += self.vel.x * dt;
self.rect.y += self.vel.y * dt;
```

Euler integration from project 4 — sample velocity, assume it's constant for this step, advance position. `Rect` doesn't support vector math directly, so we update `x` and `y` separately.

### Wall bouncing

When the ball hits a wall, flip the relevant velocity component and correct the position:

```
before bounce:          after bounce:
  ┌──────────┐            ┌──────────┐
  │ ↙        │            │ ↖        │
  │     □    │    →       │     □    │
y=0──────────┘          y=0──────────┘
  ball.y < 0              vel.y = vel.y.abs() (force downward)
```

Using `.abs()` rather than `vel.y *= -1.0` is safer: if the ball overshoots the wall by several pixels, the sign-flip approach could flip back on the next frame and leave the ball stuck. `.abs()` always forces the velocity in the correct direction regardless.

The position correction (`self.rect.y = 0.0`) prevents the ball from sitting partially outside the wall after a fast frame.

---

## The game loop

```rust
let mut left  = Paddle::new(PADDLE_OFFSET);
let mut right = Paddle::new(WINDOW_W - PADDLE_OFFSET - PADDLE_W);
let mut ball  = Ball::new();

loop {
    let dt = get_frame_time();

    left.update(dt, KeyCode::W, KeyCode::S);
    right.update(dt, KeyCode::Up, KeyCode::Down);
    ball.update(dt);

    clear_background(BLACK);
    draw_centre_line();
    left.draw();
    right.draw();
    ball.draw();
    draw_score(0, 0);

    next_frame().await;
}
```

Each type is responsible for updating and drawing itself. `main` just coordinates the sequence.

---

## Your task

Open `lessons/7-pong/lesson-03/project/src/main.rs`.

1. Add `vel: Vec2` to `struct Ball`.
2. Update `Ball::new` to initialise `vel: Vec2::new(300.0, 220.0)`.
3. Add `fn update(&mut self, dt: f32)` to `impl Ball` — move by velocity, then apply the four wall-bounce checks.
4. Change `let ball` to `let mut ball` and call `ball.update(dt)` in the loop.

```sh
cargo run --bin pong-03
```

The ball bounces around all four walls. Paddles remain controllable. The ball passes through paddles — collision is next.
