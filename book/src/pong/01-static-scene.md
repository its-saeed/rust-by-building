# Lesson 1 — Static Scene

> **Goal**: Open a fixed-size window and draw the complete Pong layout — paddles, ball, centre line, score placeholders.
>
> **Concepts**: `Conf` for window setup, game entities as structs, `draw_rectangle`, `draw_line`, `draw_text`, `measure_text`.

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
        window_width:  WINDOW_W as i32,
        window_height: WINDOW_H as i32,
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

## Constants

The window size appears in multiple places — in `window_conf`, in every positioning calculation, and in the collision checks coming in later lessons. Hardcoding `800.0` and `600.0` everywhere makes those numbers easy to get out of step. Define them once as constants instead:

```rust
const WINDOW_W: f32 = 800.0;
const WINDOW_H: f32 = 600.0;
const PADDLE_W: f32 = 12.0;
const PADDLE_H: f32 = 80.0;
const BALL_SIZE: f32 = 12.0;
const PADDLE_OFFSET: f32 = 20.0;
```

Then `window_conf` uses them directly:

```rust
fn window_conf() -> Conf {
    Conf {
        window_title: "Pong".to_owned(),
        window_width:  WINDOW_W as i32,
        window_height: WINDOW_H as i32,
        ..Default::default()
    }
}
```

`Conf` expects `i32` fields, so the `as i32` cast is required. The `f32` constants are what the rest of the game uses — all the positioning math is floating point.

---

## macroquad's `Rect`

`Rect` is a built-in macroquad type representing an axis-aligned rectangle:

```rust
struct Rect {
    x: f32,   // left edge
    y: f32,   // top edge
    w: f32,   // width
    h: f32,   // height
}
```

`x` and `y` are the **top-left corner**, not the centre:

```
(x, y) ┌──────────┐
       │          │  h
       │          │
       └──────────┘
            w
```

We'll use `Rect` inside our game structs to represent both paddles and the ball. Later, `rect.overlaps(&other)` will be our entire collision detection — one method call.

---

## Game entities as structs

In projects 4 and 5, we defined `Body` as a struct that owns its position and velocity and knows how to `draw` and `update` itself. The same pattern applies here.

Instead of keeping loose variables like `left_paddle_x`, `left_paddle_y`, we define proper types:

```rust
struct Paddle {
    rect: Rect,
}

struct Ball {
    rect: Rect,
}
```

Each struct owns a `Rect` that holds its position and size. Later lessons will add more fields — velocity for the ball, key bindings for the paddles — but for now we just need to draw them.

---

## Implementing `Paddle`

The `new` constructor handles all the positioning math. The `draw` method calls macroquad's drawing function:

```rust
impl Paddle {
    fn new(x: f32) -> Self {
        Paddle {
            rect: Rect::new(x, WINDOW_H / 2.0 - PADDLE_H / 2.0, PADDLE_W, PADDLE_H),
        }
    }

    fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE);
    }
}
```

`new` takes only `x` — the horizontal position — because the vertical position is always centred on screen. Create both paddles:

```rust
let left  = Paddle::new(PADDLE_OFFSET);
let right = Paddle::new(WINDOW_W - PADDLE_OFFSET - PADDLE_W);
```

**Left paddle x** — `PADDLE_OFFSET = 20`:

```
x=0          x=20
 │←─ 20 ──►│┃
 │           ┃
```

**Right paddle x** — `WINDOW_W - PADDLE_OFFSET - PADDLE_W = 800 - 20 - 12 = 768`. We subtract `PADDLE_W` because `x` is the *left* edge. Without it, the paddle would hang 12 pixels off-screen:

```
              x=768  x=800
                ┃◄12►│←20─│
                ┃          │
```

**Both paddles y** — `WINDOW_H / 2.0 - PADDLE_H / 2.0 = 300 - 40 = 260`. Half the height subtracted moves the top edge up so the centre of the paddle sits at the screen midpoint:

```
y=0   ┌──────────────┐
      │              │
y=260 │   ┌──────┐   │  ← paddle top (y)
      │   │      │   │
y=300 │   │  ────┤   │  ← screen centre
      │   │      │   │
y=340 │   └──────┘   │  ← paddle bottom (y + PADDLE_H)
      │              │
y=600 └──────────────┘
```

---

## Implementing `Ball`

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
        }
    }

    fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE);
    }
}
```

The ball starts centred on screen. `BALL_SIZE / 2.0` offsets the top-left corner so the ball's visual centre lands on `(WINDOW_W / 2, WINDOW_H / 2)`:

```
       x=394  x=400  x=406
         │◄─6─►│
         ┌─────┐  ← ball top-left at (394, 294)
         │     │
         └─────┘
               ↑
         screen centre (400, 300)
```

---

## Drawing the centre line

A dashed centre line is traditional. Draw it as a series of short segments spaced down the screen:

```rust
fn draw_centre_line() {
    let mut y = 10.0;
    while y < WINDOW_H {
        draw_line(WINDOW_W / 2.0, y, WINDOW_W / 2.0, y + 15.0, 2.0, DARKGRAY);
        y += 25.0;
    }
}
```

Each iteration draws a 15 px segment, then skips 10 px before the next:

```
y=10  ┊  ← segment (15 px tall)
y=25  ·  ← gap (10 px)
y=35  ┊
y=50  ·
y=60  ┊
      ...
```

---

## Centering the score text

`draw_text` places text by its **bottom-left** corner. To centre it, measure its width first:

```rust
fn draw_score(left: u32, right: u32) {
    let text = format!("{}   {}", left, right);
    let dims = measure_text(&text, None, 48, 1.0);
    draw_text(&text, WINDOW_W / 2.0 - dims.width / 2.0, 48.0, 48.0, WHITE);
}
```

Without the offset, `x = WINDOW_W / 2.0` places the *left edge* of the text at centre — it would appear right-heavy. Subtracting half the text width shifts it left so it's visually centred:

```
            x=400 (centre)
              │
  ┌───────────┼───────────┐
  │  0   0    │            │  ← wrong: left-aligned at centre
  │      0   0             │  ← correct: centred
  │      ↑                 │
  │  WINDOW_W/2 - dims.width/2
  └────────────────────────┘
```

---

## The game loop

With the structs and helpers in place, the loop is clean:

```rust
let left  = Paddle::new(PADDLE_OFFSET);
let right = Paddle::new(WINDOW_W - PADDLE_OFFSET - PADDLE_W);
let ball  = Ball::new();

loop {
    clear_background(BLACK);

    draw_centre_line();
    left.draw();
    right.draw();
    ball.draw();
    draw_score(0, 0);

    next_frame().await;
}
```

Each type draws itself. `main` doesn't need to know anything about pixels or Rect fields.

---

## Your task

Open `lessons/7-pong/lesson-01/project/src/main.rs`.

1. Define the constants and `window_conf()`.
2. Define `struct Paddle { rect: Rect }` and `impl Paddle` with `new(x: f32)` and `draw`.
3. Define `struct Ball { rect: Rect }` and `impl Ball` with `new()` and `draw`.
4. Add `draw_centre_line()` and `draw_score(left: u32, right: u32)` as free functions.
5. In `main`, create the paddles and ball, then loop with all draws.

```sh
cargo run --bin pong-01
```

You should see a black screen with two white paddles, a white ball in the centre, a dashed grey centre line, and "0   0" at the top.
