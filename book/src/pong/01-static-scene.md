# Lesson 1 — Static Scene

> **Goal**: Open a fixed-size window and draw the complete Pong layout — paddles, ball, centre line, score placeholders.
>
> **Concepts**: `Conf` for window setup, `draw_rectangle`, `draw_line`, `draw_text`, `measure_text` for centering, macroquad's `Vec2` and `Rect`.

---

## Fixing the window size

Previous projects used whatever window macroquad opened by default. For a game, we want a specific size. macroquad's `Conf` struct lets you set it at startup:

```rust
fn window_conf() -> Conf {
    Conf {
        window_title: "Pong".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // ...
}
```

`..Default::default()` fills in every field you didn't name with its default value. `Conf` has many fields — fullscreen, high-dpi, resizable — and this pattern lets you set only what you care about.

---

## macroquad's `Rect`

`Rect` is a built-in macroquad type:

```rust
struct Rect {
    x: f32,   // left edge
    y: f32,   // top edge
    w: f32,   // width
    h: f32,   // height
}
```

It represents an axis-aligned rectangle. We'll use it for both paddles and the ball. Later, `rect.overlaps(&other)` will be our entire collision detection.

Create it with `Rect::new(x, y, w, h)` and draw it with `draw_rectangle`:

```rust
let paddle = Rect::new(20.0, 250.0, 12.0, 80.0);
draw_rectangle(paddle.x, paddle.y, paddle.w, paddle.h, WHITE);
```

---

## Laying out the scene

Define constants for the game dimensions at the top of the file:

```rust
const WINDOW_W: f32 = 800.0;
const WINDOW_H: f32 = 600.0;
const PADDLE_W: f32 = 12.0;
const PADDLE_H: f32 = 80.0;
const BALL_SIZE: f32 = 12.0;
const PADDLE_OFFSET: f32 = 20.0;
```

In `main`, create the two paddles and ball as `Rect` values:

```rust
let left_paddle  = Rect::new(PADDLE_OFFSET, WINDOW_H / 2.0 - PADDLE_H / 2.0, PADDLE_W, PADDLE_H);
let right_paddle = Rect::new(WINDOW_W - PADDLE_OFFSET - PADDLE_W, WINDOW_H / 2.0 - PADDLE_H / 2.0, PADDLE_W, PADDLE_H);
let ball         = Rect::new(WINDOW_W / 2.0 - BALL_SIZE / 2.0, WINDOW_H / 2.0 - BALL_SIZE / 2.0, BALL_SIZE, BALL_SIZE);
```

---

## Drawing the centre line

A dashed centre line is traditional. Draw it as a series of short segments:

```rust
let mut y = 10.0;
while y < WINDOW_H {
    draw_line(WINDOW_W / 2.0, y, WINDOW_W / 2.0, y + 15.0, 2.0, DARKGRAY);
    y += 25.0;
}
```

---

## Centering the score text

`draw_text` positions text by its bottom-left corner, which makes centering awkward. `measure_text` tells you the rendered size so you can offset correctly:

```rust
let score_text = "0   0";
let dims = measure_text(score_text, None, 48, 1.0);
draw_text(
    score_text,
    WINDOW_W / 2.0 - dims.width / 2.0,
    48.0,
    48.0,
    WHITE,
);
```

`measure_text(text, font, font_size, font_scale)` returns a `TextDimensions` with `.width` and `.height`. Passing `None` for font uses the built-in default.

---

## The draw loop

Put it all together:

```rust
loop {
    clear_background(BLACK);

    // centre line
    let mut y = 10.0;
    while y < WINDOW_H {
        draw_line(WINDOW_W / 2.0, y, WINDOW_W / 2.0, y + 15.0, 2.0, DARKGRAY);
        y += 25.0;
    }

    // paddles and ball
    draw_rectangle(left_paddle.x,  left_paddle.y,  left_paddle.w,  left_paddle.h,  WHITE);
    draw_rectangle(right_paddle.x, right_paddle.y, right_paddle.w, right_paddle.h, WHITE);
    draw_rectangle(ball.x, ball.y, ball.w, ball.h, WHITE);

    // score
    let score_text = "0   0";
    let dims = measure_text(score_text, None, 48, 1.0);
    draw_text(score_text, WINDOW_W / 2.0 - dims.width / 2.0, 48.0, 48.0, WHITE);

    next_frame().await;
}
```

Nothing moves yet. That's correct — we're building the scene first, then adding behaviour one lesson at a time.

---

## Your task

Open `lessons/7-pong/lesson-01/project/src/main.rs`.

1. Set up `window_conf()` returning a `Conf` with width 800 and height 600.
2. Define the constants for paddle and ball dimensions.
3. Create the three `Rect` values for paddles and ball.
4. In the loop, draw the centre line, the three rectangles, and the centered score text.

```sh
cargo run --bin pong-01
```

You should see a black screen with two white paddles, a white ball in the centre, a dashed grey centre line, and "0   0" at the top.
