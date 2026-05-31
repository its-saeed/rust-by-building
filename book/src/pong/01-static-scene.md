# Lesson 1 — Static Scene

> **Goal**: Open a fixed-size window and draw the complete Pong layout — paddles, ball, centre line, score placeholders.
>
> **Concepts**: `Conf` for window setup, `draw_rectangle`, `draw_line`, `draw_text`, `measure_text` for centering, macroquad's `Vec2` and `Rect`.

---

## The game board

Here's what we're building — a fixed 800×600 window with a score display, two paddles, a ball, and a dashed centre line:

```
(0,0)                   (800,0)
  ┌──────────────────────────────┐
  │           0   0              │  ← score, centred
  │                              │
  │  ┃    ┊              ┊    ┃  │
  │  ┃    ┊      □       ┊    ┃  │  ← ball (□)
  │  ┃    ┊              ┊    ┃  │
  │        ┊              ┊       │
  │        ┊              ┊       │
  └──────────────────────────────┘
(0,600)                 (800,600)

  ↑            ↑
left          centre
paddle         line
```

The origin `(0, 0)` is the top-left corner. `x` increases to the right, `y` increases **downward** — the same screen convention from project 4.

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

`x` and `y` are the **top-left corner** of the rectangle, not its centre:

```
(x, y) ┌──────────┐
       │          │  h
       │          │
       └──────────┘
            w
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

**Left paddle x** — `PADDLE_OFFSET = 20`. The paddle starts 20 px from the left edge:

```
x=0          x=20
 │←─ 20 ──►│┃
 │           ┃
```

**Right paddle x** — `WINDOW_W - PADDLE_OFFSET - PADDLE_W = 800 - 20 - 12 = 768`. We subtract `PADDLE_W` because `x` is the *left* edge of the paddle — if we only subtracted the offset, the paddle would hang 12 pixels off the right side of the screen:

```
                x=768  x=800
                  ┃◄12►│←20─│
                  ┃          │
```

**Both paddles y** — `WINDOW_H / 2.0 - PADDLE_H / 2.0 = 300 - 40 = 260`. Subtracting half the paddle height shifts the top edge up so the paddle is centred on the midpoint:

```
y=0   ┌──────────────┐
      │              │
y=260 │   ┌──────┐   │  ← paddle top (y)
      │   │      │   │
y=300 │   │  ────┤   │  ← screen centre (WINDOW_H / 2)
      │   │      │   │
y=340 │   └──────┘   │  ← paddle bottom (y + PADDLE_H)
      │              │
y=600 └──────────────┘
```

**Ball position** — `WINDOW_W / 2.0 - BALL_SIZE / 2.0` for both axes. Same idea: the ball's top-left corner is offset by half its size so its centre lands on the screen centre:

```
         x=394  x=400 x=406
           │◄6►│  │
           ┌───┐   ← ball (12×12), top-left at (394, 294)
           │   │
           └───┘
                ↑
          screen centre (400, 300)
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

Each iteration draws a 15 px segment, then jumps 25 px before the next — leaving a 10 px gap:

```
y=10  ┊  ← draw_line y to y+15
y=25  ·  ← gap (no draw)
y=35  ┊  ← draw_line y to y+15
y=50  ·
y=60  ┊
      ...
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

Without the offset, `draw_text` at `x = WINDOW_W / 2.0` would place the *left edge* of the text at the centre — making it look right-heavy. Subtracting half the text width shifts the whole string left so it's visually centred:

```
              x=400 (centre)
                │
  ┌─────────────┼─────────────┐
  │  ←dims.width/2→  0   0    │  ← wrong: left-aligned at centre
  │        0   0              │  ← correct: centred
  │        ↑                  │
  │   WINDOW_W/2 - dims.width/2
  └───────────────────────────┘
```

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
