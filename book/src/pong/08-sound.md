# Lesson 6 — Sound and Polish

> **Goal**: Load and play sounds on paddle hits and scoring, add a start screen, and make the ball accelerate with each rally.
>
> **Concepts**: `async` resource loading, `Option<T>` for optional values, new enum variant for a start screen, speed progression.

---

## Loading sounds

Sounds live on disk, so loading them takes time — macroquad's `load_sound` is `async`:

```rust
let bounce_sound = load_sound("assets/bounce.wav").await.ok();
let score_sound  = load_sound("assets/score.wav").await.ok();
```

Two things worth noting:

**`.await`** — everything up to this point has happened synchronously inside `async fn main`. `.await` yields to the executor while the file loads, then resumes with the result. You can only `.await` inside an `async` function, which is why `main` carries that keyword.

**`.ok()`** — `load_sound` returns `Result<Sound, _>`. Calling `.ok()` converts that into `Option<Sound>`: `Some(sound)` if the file loaded, `None` if it didn't. The game still runs without the files — the sounds are simply silent.

Place both lines **before** the game loop, right after the window opens. Loading inside the loop would re-read the file every frame.

### Providing the audio files

Put two WAV files in the project alongside `Cargo.toml`:

```
lesson-08/project/
    assets/
        bounce.wav    ← played on paddle hits and wall bounces
        score.wav     ← played when a point is scored
    src/
        main.rs
    Cargo.toml
```

Any short mono WAV files work. A 50ms sine-wave beep for `bounce.wav` and a slightly lower one for `score.wav` are typical. The macroquad asset path is relative to the working directory where you run `cargo run`.

---

## Playing a sound

```rust
if let Some(ref s) = bounce_sound {
    play_sound_once(s);
}
```

`play_sound_once` takes `&Sound`. The `ref s` in the pattern borrows out of the `Option` so you get a reference without moving the value. This pattern appears every time you trigger a sound, so it's worth getting comfortable with it.

---

## A start screen — `WaitingToStart`

Right now the game launches and the ball moves immediately. A start screen gives players a moment to get ready and makes the game feel finished.

Add a third variant to `State`:

```rust
enum State {
    WaitingToStart,
    Playing,
    GameOver,
}
```

Change the initial state in `main`:

```rust
let mut state = State::WaitingToStart;
```

Add the new arm to the `match`:

```rust
State::WaitingToStart => {
    let title = "PONG";
    let tdims = measure_text(title, None, 96, 1.0);
    draw_text(title, WINDOW_W / 2.0 - tdims.width / 2.0, WINDOW_H / 2.0 - 20.0, 96.0, WHITE);

    let hint = "Press Space to start";
    let hdims = measure_text(hint, None, 24, 1.0);
    draw_text(hint, WINDOW_W / 2.0 - hdims.width / 2.0, WINDOW_H / 2.0 + 50.0, 24.0, GRAY);

    if is_key_pressed(KeyCode::Space) {
        state = State::Playing;
    }
}
```

The ball and paddles draw every frame (outside the match), so the player sees the starting positions while waiting. Only the state transition is gated behind Space.

When restarting after a game over, go directly to `State::Playing` — the player already knows how to play:

```rust
if is_key_pressed(KeyCode::R) {
    score = Score::new();
    ball.reset();
    left  = Paddle::new(PADDLE_OFFSET);
    right = Paddle::new(WINDOW_W - PADDLE_OFFSET - PADDLE_W);
    state = State::Playing;   // skip start screen on restart
}
```

---

## Ball acceleration

Each rally should feel more intense than the last. Multiply `vel` by a small factor every time the ball hits a paddle:

```rust
fn check_paddles(&mut self, left: &Paddle, right: &Paddle) -> bool {
    let mut hit = false;
    if self.rect.overlaps(&left.rect) {
        self.rect.x = left.rect.x + left.rect.w;
        self.deflect(left);
        self.vel.x = self.vel.x.abs();
        self.vel  *= 1.05;
        hit = true;
    }
    if self.rect.overlaps(&right.rect) {
        self.rect.x = right.rect.x - self.rect.w;
        self.deflect(right);
        self.vel.x = -self.vel.x.abs();
        self.vel  *= 1.05;
        hit = true;
    }
    hit
}
```

The method now returns `bool` so the caller knows whether to play a sound. `self.vel *= 1.05` scales both components — the ball keeps its angle but moves 5% faster.

`ball.reset()` already sets a fixed speed, so acceleration resets automatically when a point is scored.

Use the return value in the game loop:

```rust
let hit = ball.check_paddles(&left, &right);
if hit {
    if let Some(ref s) = bounce_sound { play_sound_once(s); }
}
```

For the score sound, `score.update` doesn't tell you *which* event happened — only whether the game is over. Compare the total before and after the call to detect a new point:

```rust
if score.update(&ball) {
    ball.reset();
    if let Some(ref s) = score_sound { play_sound_once(s); }
    if score.left  >= WIN_SCORE { winner = "Left player wins!";  state = State::GameOver; }
    if score.right >= WIN_SCORE { winner = "Right player wins!"; state = State::GameOver; }
}
```

---

## Your task

Open `lessons/7-pong/lesson-08/project/src/main.rs`.

1. Add `WaitingToStart` to `enum State`. Change the initial state to `State::WaitingToStart`.
2. Before the game loop, load `bounce_sound` and `score_sound` using `load_sound(...).await.ok()`.
3. Add the `WaitingToStart` match arm with the title text and Space-to-start transition.
4. Change `check_paddles` to return `bool` and multiply `self.vel *= 1.05` on each hit.
5. Capture the return value of `check_paddles` and play `bounce_sound` when it's `true`.
6. Inside the `if score.update(&ball)` block, play `score_sound`.

```sh
cargo run --bin pong-08
```

Put two WAV files in `lessons/7-pong/lesson-08/project/assets/` before running. Play a rally — the ball should get noticeably faster after a few hits, and each event should produce a sound.

