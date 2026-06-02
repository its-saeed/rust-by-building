# Lesson 1 — Static Scene

> **Goal**: Draw the cannon, a field of pegs, and the bucket. Split the code into modules from the start.
>
> **Concepts**: `mod`, `pub`, `use`, `#[derive(Debug, Clone, Copy)]`, `impl Default`, level layout as `Vec<Peg>`.

---

## Project structure

Open `lessons/8-peggle/lesson-01/project/src/`. You will find several files already created:

```
src/
  main.rs
  state.rs
  level.rs
  peg.rs
  cannon.rs
  bucket.rs
```

Each file is a module. `main.rs` declares them all with `mod` and uses items from them with `use`. Read through each file to see what is already there and what the TODOs ask you to add.

---

## Constants

All sizing constants live in `main.rs` and are `pub` so other modules can read them:

```rust
pub const WINDOW_W:    f32 = 800.0;
pub const WINDOW_H:    f32 = 600.0;
pub const PEG_RADIUS:  f32 = 8.0;
pub const BALL_RADIUS: f32 = 6.0;
pub const BUCKET_W:    f32 = 80.0;
pub const BUCKET_H:    f32 = 16.0;
pub const CANNON_LEN:  f32 = 40.0;
```

Other modules import them with `use crate::{WINDOW_W, WINDOW_H, ...}` or refer to them as `crate::WINDOW_W`.

---

## `PegKind` and `Peg`

In `peg.rs`:

```rust
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PegKind {
    Blue,
    Orange,
}

#[derive(Debug, Clone)]
pub struct Peg {
    pub pos:  Vec2,
    pub kind: PegKind,
    pub hit:  bool,
}

impl Peg {
    pub fn new(pos: Vec2, kind: PegKind) -> Self {
        Peg { pos, kind, hit: false }
    }

    pub fn draw(&self) {
        let color = match self.kind {
            PegKind::Blue   => BLUE,
            PegKind::Orange => ORANGE,
        };
        draw_circle(self.pos.x, self.pos.y, crate::PEG_RADIUS, color);
    }
}
```

`#[derive(Debug, Clone, Copy, PartialEq)]` on `PegKind` gives you:
- `Debug` — lets you print the value with `{:?}` for debugging
- `Clone` / `Copy` — `PegKind` is small (one byte), so copying it is cheap; `Copy` means it moves like integers rather than requiring `.clone()`
- `PartialEq` — lets you compare with `==`

`Peg` derives `Debug` and `Clone` but not `Copy`, because `Vec2` does not implement `Copy` in all contexts and we may want to add fields later.

---

## Level layout

In `level.rs`, define the peg positions for one level as a function that returns `Vec<Peg>`:

```rust
use macroquad::prelude::*;
use crate::peg::{Peg, PegKind};

pub fn level_one() -> Vec<Peg> {
    let mut pegs = Vec::new();

    // 5 rows of 9 pegs, centred on the screen
    for row in 0..5_u32 {
        for col in 0..9_u32 {
            let x = 100.0 + col as f32 * 75.0;
            let y = 180.0 + row as f32 * 70.0;
            // every third peg is orange
            let kind = if (row * 9 + col) % 3 == 0 {
                PegKind::Orange
            } else {
                PegKind::Blue
            };
            pegs.push(Peg::new(Vec2::new(x, y), kind));
        }
    }

    pegs
}
```

---

## Cannon

In `cannon.rs`:

```rust
use macroquad::prelude::*;

pub struct Cannon {
    pub pos:   Vec2,    // base position (top-centre of screen)
    pub angle: f32,     // radians; 0 = straight down
}

impl Default for Cannon {
    fn default() -> Self {
        Cannon {
            pos:   Vec2::new(crate::WINDOW_W / 2.0, 30.0),
            angle: std::f32::consts::FRAC_PI_2,  // pointing straight down
        }
    }
}

impl Cannon {
    pub fn draw(&self) {
        let tip = self.pos + Vec2::new(
            self.angle.cos() * crate::CANNON_LEN,
            self.angle.sin() * crate::CANNON_LEN,
        );
        draw_line(self.pos.x, self.pos.y, tip.x, tip.y, 6.0, GRAY);
        draw_circle(self.pos.x, self.pos.y, 10.0, DARKGRAY);
    }
}
```

`impl Default` gives `Cannon::default()` — a standard Rust convention for "a sensible starting value". `main.rs` creates the cannon with `Cannon::default()` rather than manually setting fields.

---

## Bucket

In `bucket.rs`:

```rust
use macroquad::prelude::*;

pub struct Bucket {
    pub x:   f32,
    pub dir: f32,   // 1.0 = moving right, -1.0 = moving left
}

impl Default for Bucket {
    fn default() -> Self {
        Bucket {
            x:   crate::WINDOW_W / 2.0 - crate::BUCKET_W / 2.0,
            dir: 1.0,
        }
    }
}

impl Bucket {
    pub fn draw(&self) {
        let y = crate::WINDOW_H - 30.0;
        draw_rectangle(self.x, y, crate::BUCKET_W, crate::BUCKET_H, GREEN);
    }
}
```

---

## `main.rs`

```rust
use macroquad::prelude::*;

mod state;
mod level;
mod peg;
mod cannon;
mod bucket;

use cannon::Cannon;
use bucket::Bucket;

pub const WINDOW_W:    f32 = 800.0;
pub const WINDOW_H:    f32 = 600.0;
pub const PEG_RADIUS:  f32 = 8.0;
pub const BALL_RADIUS: f32 = 6.0;
pub const BUCKET_W:    f32 = 80.0;
pub const BUCKET_H:    f32 = 16.0;
pub const CANNON_LEN:  f32 = 40.0;

fn window_conf() -> Conf {
    Conf {
        window_title:  "Peggle Nights".to_owned(),
        window_width:  WINDOW_W as i32,
        window_height: WINDOW_H as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut pegs   = level::level_one();
    let     cannon = Cannon::default();
    let     bucket = Bucket::default();

    loop {
        clear_background(Color::from_hex(0x0a0a1a));

        for peg in &pegs {
            peg.draw();
        }
        cannon.draw();
        bucket.draw();

        next_frame().await;
    }
}
```

---

## Your task

Open each file in `lessons/8-peggle/lesson-01/project/src/` and follow the TODOs:

1. In `peg.rs`: add `#[derive]` attributes, implement `Peg::new` and `Peg::draw`.
2. In `level.rs`: implement `level_one()` returning a `Vec<Peg>` with at least 20 pegs, some orange.
3. In `cannon.rs`: implement `Default` and `draw`.
4. In `bucket.rs`: implement `Default` and `draw`.
5. In `main.rs`: add the `mod` declarations, create the objects, and draw them each frame.

```sh
cargo run --bin peggle-01
```

You should see a dark background, a grey cannon at the top centre, rows of blue and orange pegs, and a green bucket at the bottom. Nothing moves yet.
