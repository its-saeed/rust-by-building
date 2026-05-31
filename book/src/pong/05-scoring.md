# Lesson 5 — Scoring

> **Goal**: Award points when the ball exits the screen, reset the ball, and end the game when someone reaches 5.
>
> **Concepts**: `enum` for game state, `match`, `u32` score fields, `Ball::reset`.

---

## The scoring zones

The ball exits play when it fully leaves the left or right edge of the screen:

```
  ball.rect.x + ball.rect.w < 0        ball.rect.x > WINDOW_W
           │                                   │
           ▼                                   ▼
  ┌────────╫──────────────────────────────────╫────────┐
  │ right  ║                                  ║  left  │
  │ scores ║         game area                ║ scores │
  └────────╫──────────────────────────────────╫────────┘
     ←ball exits                        ball exits→
```

- Ball exits **left**: right player scores (the left player missed).
- Ball exits **right**: left player scores (the right player missed).

These checks replace the temporary left/right wall bounces from lesson 3.

---

## `Score` struct

Group both scores into a struct so they're easy to pass around and draw:

```rust
struct Score {
    left:  u32,
    right: u32,
}

impl Score {
    fn new() -> Self {
        Score { left: 0, right: 0 }
    }

    fn update(&mut self, ball: &mut Ball) -> Option<&'static str> {
        if ball.rect.x + ball.rect.w < 0.0 {
            self.right += 1;
            ball.reset();
        }
        if ball.rect.x > WINDOW_W {
            self.left += 1;
            ball.reset();
        }
        if self.left  >= WIN_SCORE { return Some("Left player wins!"); }
        if self.right >= WIN_SCORE { return Some("Right player wins!"); }
        None
    }

    fn draw(&self) {
        let text = format!("{}   {}", self.left, self.right);
        let dims = measure_text(&text, None, 48, 1.0);
        draw_text(&text, WINDOW_W / 2.0 - dims.width / 2.0, 48.0, 48.0, WHITE);
    }
}
```

`update` takes `&mut Ball` because it calls `ball.reset()` when a point is scored. It returns `Option<&'static str>` — `None` while the game continues, `Some("Left player wins!")` or `Some("Right player wins!")` when someone hits `WIN_SCORE`. The caller gets the winner string for free; no re-inspection of the score needed.

Replace the `draw_score(0, 0)` free function call in the loop with `score.draw()`.

---

## `enum State`

Use an enum to track whether the game is running or over:

```rust
enum State {
    Playing,
    GameOver,
}
```

`match` on it each frame to decide what to update and draw:

```rust
match state {
    State::Playing => {
        // update paddles, ball, check scoring
    }
    State::GameOver => {
        // draw win message, wait for restart
    }
}
```

This is cleaner than a boolean `game_over` flag — `enum` makes all valid states explicit, and `match` forces you to handle every one.

---

## `Ball::reset`

When a point is scored, the ball returns to the centre with a randomised direction:

```rust
impl Ball {
    fn reset(&mut self) {
        self.rect.x = WINDOW_W / 2.0 - BALL_SIZE / 2.0;
        self.rect.y = WINDOW_H / 2.0 - BALL_SIZE / 2.0;
        let dir_x = if macroquad::rand::gen_range(0, 2) == 0 { 1.0_f32 } else { -1.0 };
        let dir_y = if macroquad::rand::gen_range(0, 2) == 0 { 1.0_f32 } else { -1.0 };
        self.vel = Vec2::new(dir_x * 300.0, dir_y * 180.0);
    }
}
```

`macroquad::rand::gen_range(0, 2)` returns either `0` or `1` — a coin flip for direction. This keeps each serve unpredictable without adding a dependency on a separate random crate.

Also **remove** the two temporary left/right wall bounce blocks from `Ball::update`. Those lines kept the ball on screen during earlier lessons, but now the ball is allowed to exit.

---

## Calling `Score::update` in the game loop

Declare `winner` alongside `state` before the loop:

```rust
let mut winner = "";
let mut state  = State::Playing;
```

Inside the `State::Playing` arm, after `ball.update(dt)` and `ball.check_paddles(...)`:

```rust
if let Some(w) = score.update(&mut ball) {
    winner = w;
    state  = State::GameOver;
}
```

`if let` destructures the `Option`: if `update` returned `Some(w)`, store the string and change state. If it returned `None`, skip the block entirely.

---

## Game over screen

In the `State::GameOver` arm:

```rust
State::GameOver => {
    let dims = measure_text(winner, None, 48, 1.0);
    draw_text(winner, WINDOW_W / 2.0 - dims.width / 2.0, WINDOW_H / 2.0, 48.0, WHITE);

    let hint = "Press R to restart";
    let hdims = measure_text(hint, None, 24, 1.0);
    draw_text(hint, WINDOW_W / 2.0 - hdims.width / 2.0, WINDOW_H / 2.0 + 40.0, 24.0, GRAY);

    if is_key_pressed(KeyCode::R) {
        score = Score::new();
        ball.reset();
        left  = Paddle::new(PADDLE_OFFSET);
        right = Paddle::new(WINDOW_W - PADDLE_OFFSET - PADDLE_W);
        state = State::Playing;
    }
}
```

`is_key_pressed` (not `is_key_down`) fires only on the frame the key is first pressed — right for a restart trigger.

---

## Your task

Open `lessons/7-pong/lesson-05/project/src/main.rs`.

1. Define `struct Score` with `new`, `update`, and `draw`.
2. Define `enum State { Playing, GameOver }`.
3. Add `fn reset(&mut self)` to `impl Ball`. Remove the left/right wall bounces from `Ball::update`.
4. In `main`, create `let mut score = Score::new()` and `let mut state = State::Playing`.
5. Wrap the update logic in `match state { State::Playing => { ... } State::GameOver => { ... } }`.
6. Use `if let Some(w) = score.update(&mut ball)` to capture the winner and transition to `GameOver`.
7. Add the game over screen inside `State::GameOver`.

```sh
cargo run --bin pong-05
```

Play to 5 — the game should stop, declare a winner, and restart cleanly on `R`.
