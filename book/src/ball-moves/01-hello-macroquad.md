# Lesson 1 — Hello macroquad

> **Goal**: Open a window, run a game loop, draw a circle on screen.
>
> **Concepts**: external crates, `#[macroquad::main]`, `async fn`, game loop, `draw_circle`, `clear_background`, `next_frame`.

---

## Adding a dependency

Open `lessons/4-ball-moves/lesson-01/project/Cargo.toml`. You'll see:

```toml
[dependencies]
macroquad = "0.4"
```

That one line is all it takes to add macroquad. When you run `cargo run`, Cargo downloads it automatically.

This is how Rust handles external libraries — you declare them in `Cargo.toml` and the toolchain handles the rest.

---

## The game loop

Every game — and every physics engine — runs the same fundamental structure:

```
loop:
  1. clear the screen
  2. update the world
  3. draw the world
  4. wait for the next frame
```

In macroquad, that looks like this:

```rust
use macroquad::prelude::*;

#[macroquad::main("A Ball Moves")]
async fn main() {
    loop {
        clear_background(BLACK);

        // update and draw here

        next_frame().await;
    }
}
```

Let's take it apart.

---

## `use macroquad::prelude::*`

This imports everything macroquad exports into scope — drawing functions, color constants, input helpers. The `*` means "import all public names from this module." You'll see this pattern with macroquad throughout the course.

---

## `#[macroquad::main("A Ball Moves")]`

This line is an **attribute** — extra instructions attached to the item below it. Here it tells macroquad to open a window titled `"A Ball Moves"` and run `main` inside its event system.

Attributes start with `#[` and end with `]`. You'll see `#[derive(...)]` in the next lesson — same syntax, different purpose.

---

## `async fn main()`

The `async` keyword is required by macroquad. It lets macroquad pause your code between frames without blocking the operating system.

You don't need to understand async deeply right now — just know that `next_frame().await` is the pause point. Everything before it runs in one frame; then the OS renders the frame and you start the next iteration.

---

## Inside the loop

```rust
clear_background(BLACK);
```

Paints the entire window black. Without this, each frame draws on top of the last — you'd see trails instead of a moving ball.

```rust
next_frame().await;
```

Hands control back to macroquad. It renders what you drew, handles events, and calls you again for the next frame. This runs ~60 times per second.

---

## Drawing a circle

```rust
draw_circle(x, y, radius, color);
```

`x` and `y` are the centre of the circle in pixels. The origin `(0.0, 0.0)` is the **top-left** corner of the window. `x` increases to the right, `y` increases downward.

The default window is **800 × 600** pixels, so the centre is `(400.0, 300.0)`.

`radius` is in pixels. `color` is one of macroquad's built-in constants: `WHITE`, `RED`, `BLUE`, `GREEN`, `YELLOW`, etc.

Example — a white circle at the centre of the screen:

```rust
draw_circle(400.0, 300.0, 20.0, WHITE);
```

---

## Your task

Open `lessons/4-ball-moves/lesson-01/project/src/main.rs`.

You'll find the game loop skeleton. Draw a white circle at position `(400.0, 300.0)` with radius `20.0`.

Run it:

```sh
cargo run --bin ball-01
```

A window should open with a white dot sitting in the middle. That's your engine, frame one.

Try changing the position, radius, and color. The window updates every time you recompile.
