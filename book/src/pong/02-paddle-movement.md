# Lesson 2 — Paddle Movement

> **Goal**: Make both paddles move in response to held keys, clamped to the screen.
>
> **Concepts**: `is_key_down` vs `is_key_pressed`, `KeyCode`, `f32::clamp`, mutating a `Rect` field.

---

## `is_key_down` vs `is_key_pressed`

macroquad gives you two ways to read the keyboard:

| Function | When it returns `true` |
|----------|----------------------|
| `is_key_pressed(key)` | Only on the first frame the key is pressed |
| `is_key_down(key)` | Every frame the key is held down |

Paddle movement should happen continuously while the key is held — so `is_key_down` is the right choice. `is_key_pressed` fires once and then stops, which would make the paddle snap a fixed distance on each tap. That's useful for menus and jumps, not smooth movement.

---

## Moving a paddle

Add a speed constant near the top:

```rust
const PADDLE_SPEED: f32 = 400.0; // pixels per second
```

Make the paddles mutable — they'll change position every frame:

```rust
let mut left_paddle  = Rect::new(...);
let mut right_paddle = Rect::new(...);
```

Inside the loop, read `dt` and move each paddle:

```rust
let dt = get_frame_time();

if is_key_down(KeyCode::W) { left_paddle.y -= PADDLE_SPEED * dt; }
if is_key_down(KeyCode::S) { left_paddle.y += PADDLE_SPEED * dt; }

if is_key_down(KeyCode::Up)   { right_paddle.y -= PADDLE_SPEED * dt; }
if is_key_down(KeyCode::Down) { right_paddle.y += PADDLE_SPEED * dt; }
```

`left_paddle.y` is just an `f32` field on the `Rect` — you mutate it directly. The draw call reads the updated value each frame.

---

## Clamping to the screen

Without a limit, paddles can move off screen. `f32::clamp(min, max)` constrains a value to a range:

```rust
left_paddle.y  = left_paddle.y.clamp(0.0, WINDOW_H - PADDLE_H);
right_paddle.y = right_paddle.y.clamp(0.0, WINDOW_H - PADDLE_H);
```

The upper bound is `WINDOW_H - PADDLE_H`, not `WINDOW_H` — we're clamping the top edge of the paddle, so the bottom edge (`y + PADDLE_H`) stays within the window.

---

## Why `dt` again

At 60 FPS, `PADDLE_SPEED * dt = 400 × 0.016 = 6.4` pixels per frame. At 120 FPS, each frame is shorter so the step is smaller, but the paddle still travels 400 pixels per second. The speed is frame-rate independent.

Without `dt`, the paddle would move twice as fast on a 120 Hz display as on 60 Hz.

---

## Your task

Open `lessons/7-pong/lesson-02/project/src/main.rs`.

1. Add `const PADDLE_SPEED: f32 = 400.0;`.
2. Change `let left_paddle` and `let right_paddle` to `let mut`.
3. Inside the loop, read `dt` with `get_frame_time()`.
4. Add four `if is_key_down(...)` blocks to move each paddle.
5. After moving, clamp both `paddle.y` values to `0.0..=WINDOW_H - PADDLE_H`.

```sh
cargo run --bin pong-02
```

Both paddles should move smoothly — W/S for the left, ↑/↓ for the right — and stop at the screen edges.
