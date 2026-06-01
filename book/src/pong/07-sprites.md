# Lesson 7 — Sprites

> **Goal**: Replace the rectangle placeholders with real sprite textures loaded from image files.
>
> **Concepts**: `load_texture`, `draw_texture_ex`, `DrawTextureParams`, storing a reference in a struct with a lifetime annotation.

---

## Asset files

Put two PNG images in the project before running:

```
lessons/7-pong/lesson-07/project/assets/
    ball.png
    paddle.png
```

The path is relative to the working directory where you run `cargo run`. macroquad looks there by default.

---

## Loading textures

Like sound, loading a texture reads from disk and is async:

```rust
let paddle_texture = load_texture("assets/paddle.png").await.unwrap();
let ball_texture   = load_texture("assets/ball.png").await.unwrap();
```

Load both **before** the game loop and before creating the structs — the structs will hold references to these values, so the values must already exist.

---

## Storing the texture in the struct

Right now `Paddle` and `Ball` draw themselves as plain rectangles. To draw a sprite instead, each struct needs to know which texture to use. The texture is loaded once and shared — both paddles use the same `paddle_texture`. Storing a reference avoids copying GPU data:

```rust
struct Paddle<'a> {
    rect:    Rect,
    texture: &'a Texture2D,
}
```

The moment you add `texture: &'a Texture2D`, the compiler requires the lifetime annotation. Write `<'a>` after `Paddle` on the struct definition and on every `impl` block:

```rust
impl<'a> Paddle<'a> {
    fn new(x: f32, texture: &'a Texture2D) -> Self {
        Paddle {
            rect: Rect::new(x, WINDOW_H / 2.0 - PADDLE_H / 2.0, PADDLE_W, PADDLE_H),
            texture,
        }
    }
}
```

`'a` is the promise: "this `Paddle` will not outlive the `Texture2D` it holds." Because the textures are loaded before the structs and live for the rest of `main`, this is always satisfied — but the compiler needs you to say it.

Do the same for `Ball`:

```rust
struct Ball<'a> {
    rect:    Rect,
    vel:     Vec2,
    texture: &'a Texture2D,
}

impl<'a> Ball<'a> {
    fn new(texture: &'a Texture2D) -> Self {
        Ball {
            rect: Rect::new(
                WINDOW_W / 2.0 - BALL_SIZE / 2.0,
                WINDOW_H / 2.0 - BALL_SIZE / 2.0,
                BALL_SIZE,
                BALL_SIZE,
            ),
            vel: Vec2::new(300.0, 220.0),
            texture,
        }
    }
}
```

---

## Drawing with `draw_texture_ex`

Replace `draw_rectangle` in both `draw` methods with `draw_texture_ex`:

```rust
fn draw(&self) {
    draw_texture_ex(
        self.texture,
        self.rect.x,
        self.rect.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(self.rect.w, self.rect.h)),
            ..Default::default()
        },
    );
}
```

`dest_size` scales the texture to exactly fit the collision `Rect` — whatever pixel dimensions the image has, it is drawn at `PADDLE_W × PADDLE_H` or `BALL_SIZE × BALL_SIZE`. The `Rect` remains the single source of truth for both physics and visuals.

`WHITE` is the tint color. Passing `WHITE` means no tint — the sprite renders in its original colors. Passing `RED` would multiply the sprite's colors by red.

---

## `Score::update` with a lifetime on `Ball`

Now that `Ball` has a lifetime parameter, passing it to `Score::update` requires the anonymous lifetime:

```rust
fn update(&mut self, ball: &Ball<'_>) -> bool {
```

`'_` tells the compiler: "there is a lifetime here, infer it." You do not need to name it because `Score` does not store the reference — it only reads the ball's position for one call.

---

## Creating the structs in `main`

```rust
let paddle_texture = load_texture("assets/paddle.png").await.unwrap();
let ball_texture   = load_texture("assets/ball.png").await.unwrap();

let mut left  = Paddle::new(PADDLE_OFFSET, &paddle_texture);
let mut right = Paddle::new(WINDOW_W - PADDLE_OFFSET - PADDLE_W, &paddle_texture);
let mut ball  = Ball::new(&ball_texture);
```

Both paddles share `&paddle_texture` — one file, two references. The `draw` calls in the loop require no changes; they still call `left.draw()`, `right.draw()`, `ball.draw()` with no arguments.

---

## Your task

Open `lessons/7-pong/lesson-07/project/src/main.rs`.

1. Add `texture: &'a Texture2D` to `struct Paddle`. Add `<'a>` to the struct and `impl`. Update `Paddle::new` to take `texture: &'a Texture2D`.
2. Replace `draw_rectangle` in `Paddle::draw` with `draw_texture_ex`.
3. Do the same for `struct Ball` — add the field, lifetime, and update `Ball::new` and `Ball::draw`.
4. Update `Score::update` to take `ball: &Ball<'_>`.
5. Load the two textures before the loop. Update the three `::new` calls to pass `&paddle_texture` and `&ball_texture`.

```sh
cargo run --bin pong-07
```

The game should play identically to lesson 5 — same logic, same feel — but paddles and ball are now drawn from your sprite files.
