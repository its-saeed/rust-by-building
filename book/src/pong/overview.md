# Project 7 — Pong

> **What you'll build**: A complete two-player Pong game — paddles, a bouncing ball, scoring, and sound.
>
> **Lessons**: 6 lessons.
>
> **Rust concepts covered**: `enum` for game state, `is_key_down` vs `is_key_pressed`, `f32::clamp`, using library types (`Rect`, `Vec2`) instead of building your own.

---

## Why Pong?

Pong is the simplest game that has everything: input, movement, collision, state, and feedback. Each element maps cleanly to one lesson, so by the end you have a complete, playable game — not just a demo.

It's also a different kind of collision than project 6. There's no physics formula, no impulse, no dot product. A ball hitting a paddle means one thing: flip the horizontal velocity. Collision detection is handled by macroquad's built-in `Rect::overlaps()` — one method call.

## What macroquad provides

In earlier projects we built `Vec2` from scratch to understand vectors. Here we use macroquad's built-in types directly:

- **`Vec2`** — position and velocity, with all math operators built in
- **`Rect`** — a rectangle with an `overlaps` method for collision detection
- **`is_key_down`** — true every frame the key is held (right for continuous paddle movement)
- **`play_sound_once`** — one-line audio
- **`Conf`** — window configuration at startup

The lesson: knowing when to build a type and when to reach for the library.

## The game

Two paddles, one ball. First to five wins. Controls: `W`/`S` for the left paddle, `↑`/`↓` for the right. Press `Space` to start, `R` to restart after the game ends.
