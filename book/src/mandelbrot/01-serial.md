# Lesson 1 — Serial Renderer

Build the Mandelbrot renderer from scratch on a single thread. By the end of this lesson you have a working, colourful fractal — and a timing number that will motivate the next lesson.

---

## Step 1 — Project setup

One binary, two dependencies:

```toml
[package]
name = "mandelbrot"
edition = "2021"

[dependencies]
macroquad   = "0.4"
num-complex = "0.4"
```

`num-complex` provides a `Complex<T>` type with arithmetic operators already implemented — so `z * z + c` works directly, matching the formula from the primer exactly.

```rust
use macroquad::prelude::*;

#[macroquad::main("Mandelbrot")]
async fn main() {
    loop {
        clear_background(BLACK);
        next_frame().await;
    }
}
```

Run it — a black window. Now fill it.

---

## Step 2 — Mapping pixels to the complex plane

The screen has pixels. The Mandelbrot set lives in the complex plane — roughly between `-2.5` and `1.0` on the real axis and `-1.2` and `1.2` on the imaginary axis.

```rust
const WIDTH:  usize = 800;
const HEIGHT: usize = 600;

// complex plane bounds
const RE_MIN: f64 = -2.5;
const RE_MAX: f64 =  1.0;
const IM_MIN: f64 = -1.2;
const IM_MAX: f64 =  1.2;
```

Convert pixel `(px, py)` to a point on the complex plane:

```rust
use num_complex::Complex;

fn pixel_to_complex(px: usize, py: usize) -> Complex<f64> {
    let re = RE_MIN + (px as f64 / WIDTH  as f64) * (RE_MAX - RE_MIN);
    let im = IM_MIN + (py as f64 / HEIGHT as f64) * (IM_MAX - IM_MIN);
    Complex::new(re, im)
}
```

This is linear interpolation — pixel 0 maps to the minimum, pixel WIDTH maps to the maximum. The result is already a `Complex<f64>`, ready to pass straight into the iteration function.

---

## Step 3 — The iteration function

```rust
use num_complex::Complex;

const MAX_ITER: u32 = 256;

fn mandelbrot(c: Complex<f64>) -> u32 {
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..MAX_ITER {
        if z.norm_sqr() > 4.0 {
            return i;
        }
        z = z * z + c;
    }
    MAX_ITER
}
```

Returns the iteration count at escape, or `MAX_ITER` if the point never escaped.

Compare this with the formula from the primer: `z = z² + c`. The code says exactly that. `num-complex` implements `*` and `+` for `Complex<f64>` so the arithmetic works without any manual unpacking.

`z.norm_sqr()` returns `x² + y²` — the squared magnitude — which is cheaper than computing the square root and equivalent to checking `|z| > 2`.

---

## Step 4 — Colour mapping

Map the escape count to a colour. A simple gradient from deep blue (escapes fast) to bright white (escapes slowly), with black for points inside the set:

```rust
fn iter_to_color(iter: u32) -> Color {
    if iter == MAX_ITER {
        return BLACK;  // inside the set
    }
    let t = iter as f32 / MAX_ITER as f32;  // 0.0 → 1.0
    // blue → cyan → white gradient
    Color::from_rgba(
        (t * 255.0) as u8,
        (t * 180.0) as u8,
        255,
        255,
    )
}
```

Feel free to experiment — colour mapping is entirely cosmetic. Changing it does not affect correctness.

---

## Step 5 — Rendering with Image

macroquad's `Image` lets you set individual pixel colours, then upload the whole image to the GPU as a `Texture2D`:

```rust
let mut image = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, BLACK);

for py in 0..HEIGHT {
    for px in 0..WIDTH {
        let c     = pixel_to_complex(px, py);
        let color = iter_to_color(mandelbrot(c));
        image.set_pixel(px as u32, py as u32, color);
    }
}

let texture = Texture2D::from_image(&image);
```

Then in the game loop:

```rust
loop {
    clear_background(BLACK);
    draw_texture(&texture, 0.0, 0.0, WHITE);
    next_frame().await;
}
```

Note: we compute the image *before* the loop and draw the same texture every frame. There is no reason to recompute every frame — the image is static.

---

## Step 6 — Timing it

Wrap the computation in a timer:

```rust
use std::time::Instant;

let t0 = Instant::now();

// ... the double loop ...

let elapsed = t0.elapsed();
println!("rendered in {:.2?}", elapsed);
```

Run it. Note the time. On a typical laptop this takes somewhere between 0.5 and 3 seconds depending on `MAX_ITER` and screen size. That number is what the next lesson will cut in half (or more).

---

## Full code

```rust
use macroquad::prelude::*;
use num_complex::Complex;
use std::time::Instant;

const WIDTH:    usize = 800;
const HEIGHT:   usize = 600;
const MAX_ITER: u32   = 256;

const RE_MIN: f64 = -2.5;
const RE_MAX: f64 =  1.0;
const IM_MIN: f64 = -1.2;
const IM_MAX: f64 =  1.2;

fn pixel_to_complex(px: usize, py: usize) -> (f64, f64) {
    let cx = RE_MIN + (px as f64 / WIDTH  as f64) * (RE_MAX - RE_MIN);
    let cy = IM_MIN + (py as f64 / HEIGHT as f64) * (IM_MAX - IM_MIN);
    (cx, cy)
}

fn mandelbrot(c: Complex<f64>) -> u32 {
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..MAX_ITER {
        if z.norm_sqr() > 4.0 { return i; }
        z = z * z + c;
    }
    MAX_ITER
}

fn iter_to_color(iter: u32) -> Color {
    if iter == MAX_ITER { return BLACK; }
    let t = iter as f32 / MAX_ITER as f32;
    Color::from_rgba((t * 255.0) as u8, (t * 180.0) as u8, 255, 255)
}

#[macroquad::main("Mandelbrot")]
async fn main() {
    let t0 = Instant::now();

    let mut image = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, BLACK);

    for py in 0..HEIGHT {
        for px in 0..WIDTH {
            let c     = pixel_to_complex(px, py);
            let color = iter_to_color(mandelbrot(c));
            image.set_pixel(px as u32, py as u32, color);
        }
    }

    println!("serial: {:.2?}", t0.elapsed());

    let texture = Texture2D::from_image(&image);

    loop {
        clear_background(BLACK);
        draw_texture(&texture, 0.0, 0.0, WHITE);
        next_frame().await;
    }
}
```

---

## Exercise

> **TODO 1**: Change `MAX_ITER` to 64 and to 512. How does the image quality change? How does the render time change?
>
> **TODO 2**: Try a different colour mapping — for example, use `iter` to cycle through hues using HSV, or create a three-colour gradient (black → red → yellow). The function signature stays the same.
>
> **TODO 3**: Change the complex plane bounds to zoom into the area around `(-0.75, 0.1)` with a width of `0.01`. What do you see?

---

## Key APIs

| API | What it does |
|-----|-------------|
| `Image::gen_image_color(w, h, color)` | Create a CPU-side pixel buffer filled with one colour |
| `image.set_pixel(x, y, color)` | Set one pixel in the buffer |
| `Texture2D::from_image(&image)` | Upload the pixel buffer to the GPU |
| `draw_texture(&tex, x, y, tint)` | Draw the texture on screen |
| `Instant::now()` / `.elapsed()` | Wall-clock timer |
