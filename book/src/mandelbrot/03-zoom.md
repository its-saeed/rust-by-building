# Lesson 3 — Zoom

The image is static — it always shows the same region of the complex plane. This lesson makes it interactive: scroll up to zoom in, scroll down to zoom out, always centred on the mouse cursor.

The key insight is that zooming is just changing the *bounds* of the complex plane window. Everything else — the iteration, the colouring, the threading — stays the same.

---

## Step 1 — Make the bounds mutable

In lesson 2, the bounds are `const`:

```rust
const RE_MIN: f64 = -2.5;
const RE_MAX: f64 =  1.0;
const IM_MIN: f64 = -1.2;
const IM_MAX: f64 =  1.2;
```

Constants cannot change. Replace them with mutable variables inside `main`:

```rust
let mut re_min = -2.5_f64;
let mut re_max =  1.0_f64;
let mut im_min = -1.2_f64;
let mut im_max =  1.2_f64;
```

Now the bounds can be updated each time the user scrolls.

---

## Step 2 — Extract a `render` function

Right now the parallel rendering code lives inline in `main`. As soon as the bounds change, we need to re-run it. Extract it into a function:

```rust
fn render(re_min: f64, re_max: f64, im_min: f64, im_max: f64) -> Texture2D {
    let num_threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);

    let (tx, rx) = mpsc::channel::<Vec<PixelData>>();
    let rows_per_strip = HEIGHT / num_threads;

    for t in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            let row_start = t * rows_per_strip;
            let row_end   = if t == num_threads - 1 { HEIGHT } else { row_start + rows_per_strip };
            let mut pixels = Vec::with_capacity((row_end - row_start) * WIDTH);
            for py in row_start..row_end {
                for px in 0..WIDTH {
                    let c     = pixel_to_complex(px, py, re_min, re_max, im_min, im_max);
                    let color = iter_to_color(mandelbrot(c));
                    pixels.push(PixelData { px: px as u32, py: py as u32, color });
                }
            }
            tx.send(pixels).unwrap();
        });
    }
    drop(tx);

    let mut image = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, BLACK);
    for pixels in rx {
        for PixelData { px, py, color } in pixels {
            image.set_pixel(px, py, color);
        }
    }
    Texture2D::from_image(&image)
}
```

`render` takes the four bounds, does all the parallel work, and returns a ready `Texture2D`. `main` calls it once at startup, then again whenever the bounds change.

`pixel_to_complex` also needs to take the bounds as parameters now, since the global constants are gone:

```rust
fn pixel_to_complex(
    px: usize, py: usize,
    re_min: f64, re_max: f64,
    im_min: f64, im_max: f64,
) -> Complex<f64> {
    let re = re_min + (px as f64 / WIDTH  as f64) * (re_max - re_min);
    let im = im_min + (py as f64 / HEIGHT as f64) * (im_max - im_min);
    Complex::new(re, im)
}
```

---

## Step 3 — Read the scroll wheel

macroquad's `mouse_wheel()` returns `(horizontal, vertical)` scroll deltas. Positive vertical means scroll up:

```rust
let (_, scroll_y) = mouse_wheel();
if scroll_y != 0.0 {
    // user scrolled — zoom and re-render
}
```

Only re-render when the scroll delta is non-zero. Rendering takes real time; doing it every frame would freeze the window.

---

## Step 4 — Zoom math

Zooming in means *shrinking* the complex plane window — you are looking at a smaller region, so each pixel covers less area and shows more detail. Zooming out means *expanding* the window.

```rust
let zoom = if scroll_y > 0.0 { 0.85 } else { 1.0 / 0.85 };
```

`0.85` is the zoom factor: each scroll step makes the window 85% of its current size (zoom in) or ~118% (zoom out).

The tricky part is keeping the point *under the mouse cursor* fixed as you zoom. If you always zoom around the centre, the image jumps when you scroll over a detail you want to explore.

Here is how to zoom around the cursor:

```rust
let (mx, my) = mouse_position();

// 1. find the complex point currently under the mouse
let cx = re_min + (mx as f64 / WIDTH  as f64) * (re_max - re_min);
let cy = im_min + (my as f64 / HEIGHT as f64) * (im_max - im_min);

// 2. scale the window
let new_width  = (re_max - re_min) * zoom;
let new_height = (im_max - im_min) * zoom;

// 3. reposition so that (cx, cy) is still under pixel (mx, my)
re_min = cx - (mx as f64 / WIDTH  as f64) * new_width;
re_max = re_min + new_width;
im_min = cy - (my as f64 / HEIGHT as f64) * new_height;
im_max = im_min + new_height;
```

Step 3 is just the `pixel_to_complex` formula solved for `re_min`:

```
cx = re_min + (mx / WIDTH) * width
re_min = cx - (mx / WIDTH) * new_width
```

The pixel ratio `mx / WIDTH` stays the same before and after the zoom — the cursor does not move. Solving for `re_min` with the new width guarantees the complex point under the cursor is unchanged.

---

## Step 5 — Re-render and the game loop

Call `render` with the updated bounds, then draw every frame as usual:

```rust
#[macroquad::main("Mandelbrot")]
async fn main() {
    let mut re_min = -2.5_f64;
    let mut re_max =  1.0_f64;
    let mut im_min = -1.2_f64;
    let mut im_max =  1.2_f64;

    let mut texture = render(re_min, re_max, im_min, im_max);

    loop {
        let (_, scroll_y) = mouse_wheel();
        if scroll_y != 0.0 {
            let zoom = if scroll_y > 0.0 { 0.85 } else { 1.0 / 0.85 };

            let (mx, my) = mouse_position();
            let cx = re_min + (mx as f64 / WIDTH  as f64) * (re_max - re_min);
            let cy = im_min + (my as f64 / HEIGHT as f64) * (im_max - im_min);

            let new_width  = (re_max - re_min) * zoom;
            let new_height = (im_max - im_min) * zoom;

            re_min = cx - (mx as f64 / WIDTH  as f64) * new_width;
            re_max = re_min + new_width;
            im_min = cy - (my as f64 / HEIGHT as f64) * new_height;
            im_max = im_min + new_height;

            texture = render(re_min, re_max, im_min, im_max);
        }

        clear_background(BLACK);
        draw_texture(&texture, 0.0, 0.0, WHITE);
        next_frame().await;
    }
}
```

The texture variable is overwritten on each zoom — the old texture is dropped and a new one uploaded to the GPU.

---

## Full code

```rust
use macroquad::prelude::*;
use num_complex::Complex;
use std::sync::mpsc;
use std::thread;

const WIDTH:    usize = 800;
const HEIGHT:   usize = 600;
const MAX_ITER: u32   = 256;

struct PixelData {
    px: u32,
    py: u32,
    color: Color,
}

fn pixel_to_complex(
    px: usize, py: usize,
    re_min: f64, re_max: f64,
    im_min: f64, im_max: f64,
) -> Complex<f64> {
    let re = re_min + (px as f64 / WIDTH  as f64) * (re_max - re_min);
    let im = im_min + (py as f64 / HEIGHT as f64) * (im_max - im_min);
    Complex::new(re, im)
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

fn render(re_min: f64, re_max: f64, im_min: f64, im_max: f64) -> Texture2D {
    let num_threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);

    let (tx, rx) = mpsc::channel::<Vec<PixelData>>();
    let rows_per_strip = HEIGHT / num_threads;

    for t in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            let row_start = t * rows_per_strip;
            let row_end   = if t == num_threads - 1 { HEIGHT } else { row_start + rows_per_strip };
            let mut pixels = Vec::with_capacity((row_end - row_start) * WIDTH);
            for py in row_start..row_end {
                for px in 0..WIDTH {
                    let c     = pixel_to_complex(px, py, re_min, re_max, im_min, im_max);
                    let color = iter_to_color(mandelbrot(c));
                    pixels.push(PixelData { px: px as u32, py: py as u32, color });
                }
            }
            tx.send(pixels).unwrap();
        });
    }
    drop(tx);

    let mut image = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, BLACK);
    for pixels in rx {
        for PixelData { px, py, color } in pixels {
            image.set_pixel(px, py, color);
        }
    }
    Texture2D::from_image(&image)
}

#[macroquad::main("Mandelbrot")]
async fn main() {
    let mut re_min = -2.5_f64;
    let mut re_max =  1.0_f64;
    let mut im_min = -1.2_f64;
    let mut im_max =  1.2_f64;

    let mut texture = render(re_min, re_max, im_min, im_max);

    loop {
        let (_, scroll_y) = mouse_wheel();
        if scroll_y != 0.0 {
            let zoom = if scroll_y > 0.0 { 0.85 } else { 1.0 / 0.85 };

            let (mx, my) = mouse_position();
            let cx = re_min + (mx as f64 / WIDTH  as f64) * (re_max - re_min);
            let cy = im_min + (my as f64 / HEIGHT as f64) * (im_max - im_min);

            let new_width  = (re_max - re_min) * zoom;
            let new_height = (im_max - im_min) * zoom;

            re_min = cx - (mx as f64 / WIDTH  as f64) * new_width;
            re_max = re_min + new_width;
            im_min = cy - (my as f64 / HEIGHT as f64) * new_height;
            im_max = im_min + new_height;

            texture = render(re_min, re_max, im_min, im_max);
        }

        clear_background(BLACK);
        draw_texture(&texture, 0.0, 0.0, WHITE);
        next_frame().await;
    }
}
```

---

## Exercise

> **TODO 1**: Print the current bounds and zoom level in the window using `draw_text`. Display the centre of the view (`(re_min + re_max) / 2`, `(im_min + im_max) / 2`) and the width of the current window.
>
> **TODO 2**: Add keyboard pan — arrow keys shift the view left/right/up/down by 10% of the current window size. Re-render after each keypress. Use `is_key_pressed(KeyCode::Right)` etc.
>
> **TODO 3**: Add a reset key — pressing `R` restores the original bounds and re-renders. Store the initial bounds in constants and restore from them.

---

## Key APIs

| API | What it does |
|-----|-------------|
| `mouse_wheel()` | `(h, v)` scroll delta this frame — positive v is scroll up |
| `mouse_position()` | `(x, y)` cursor position in pixels |
| `is_key_pressed(KeyCode::R)` | True on the frame the key is first pressed |
