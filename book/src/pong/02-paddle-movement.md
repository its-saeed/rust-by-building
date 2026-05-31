# Lesson 2 — Paddle Movement

> **Goal**: Add an `update` method to `Paddle` so both paddles respond to held keys and stay within the screen.
>
> **Concepts**: `is_key_down` vs `is_key_pressed`, `KeyCode` as a parameter, `f32::clamp`, `&mut self`.

---

## `is_key_down` vs `is_key_pressed`

macroquad gives you two ways to read the keyboard:

| Function | When it returns `true` |
|----------|----------------------|
| `is_key_pressed(key)` | Only on the first frame the key is pressed |
| `is_key_down(key)` | Every frame the key is held down |

Paddle movement should happen continuously while the key is held — so `is_key_down` is the right choice. `is_key_pressed` fires once and stops, which would make the paddle snap on each tap rather than move smoothly. That's useful for menus and jumps, not continuous movement.

---

## Adding `Paddle::update`

Add a speed constant:

```rust
const PADDLE_SPEED: f32 = 400.0; // pixels per second
```

Then add `update` to `impl Paddle`. It takes the two key codes as parameters so the same method works for both paddles:

```rust
impl Paddle {
    fn update(&mut self, dt: f32, up: KeyCode, down: KeyCode) {
        if is_key_down(up)   { self.rect.y -= PADDLE_SPEED * dt; }
        if is_key_down(down) { self.rect.y += PADDLE_SPEED * dt; }
        self.rect.y = self.rect.y.clamp(0.0, WINDOW_H - PADDLE_H);
    }
}
```

Passing `KeyCode` values as arguments means we write the logic once and reuse it for both players. The alternative — two separate methods or two separate structs — would be needless repetition.

---

## Clamping to the screen

`f32::clamp(min, max)` constrains a value to a range:

```
self.rect.y = self.rect.y.clamp(0.0, WINDOW_H - PADDLE_H);
```

The upper bound is `WINDOW_H - PADDLE_H`, not `WINDOW_H`. We clamp the **top edge** of the paddle, so the bottom edge (`y + PADDLE_H`) stays within the screen:

```
y=0   ┌──────────────┐  ← clamp min (paddle top can't go above this)
      │   ┌──────┐   │
      │   │      │   │
      │   └──────┘   │
y=520 │              │  ← clamp max = WINDOW_H - PADDLE_H = 600 - 80
      │              │    (paddle top can't go below this)
y=600 └──────────────┘  ← paddle bottom = y + 80 = 600 ✓
```

---

## Calling `update` in the game loop

Make both paddles `mut`, then call `update` on each:

```rust
let mut left  = Paddle::new(PADDLE_OFFSET);
let mut right = Paddle::new(WINDOW_W - PADDLE_OFFSET - PADDLE_W);

loop {
    let dt = get_frame_time();

    left.update(dt, KeyCode::W, KeyCode::S);
    right.update(dt, KeyCode::Up, KeyCode::Down);

    clear_background(BLACK);
    // ... draw ...
    next_frame().await;
}
```

The game loop stays readable — each line does one clear thing. The details of key codes and clamping live inside `Paddle`, not scattered through `main`.

---

## Your task

Open `lessons/7-pong/lesson-02/project/src/main.rs`.

1. Add `const PADDLE_SPEED: f32 = 400.0;`.
2. Add `fn update(&mut self, dt: f32, up: KeyCode, down: KeyCode)` to `impl Paddle`.
3. Change `let left` and `let right` to `let mut`.
4. In the loop, call `left.update(dt, KeyCode::W, KeyCode::S)` and `right.update(dt, KeyCode::Up, KeyCode::Down)`.

```sh
cargo run --bin pong-02
```

Both paddles should move smoothly — W/S for the left, ↑/↓ for the right — and stop at the screen edges.
