# Lesson 2 — Parallel Renderer

The serial renderer times itself. Now cut that time down by spreading the work across all CPU cores. The algorithm does not change — only *who* runs it.

---

## The plan

Every row of pixels is independent of every other row. Row 42 does not need to know what row 41 computed — it just needs to know its own `py` coordinate. This makes the problem **embarrassingly parallel**: split the image into horizontal strips, give each strip to a thread, collect the results.

```
image (600 rows)
├── strip 0  rows   0–149  → thread 0 ─╮
├── strip 1  rows 150–299  → thread 1  ├─ channel ─→ main thread → texture
├── strip 2  rows 300–449  → thread 2  │
└── strip 3  rows 450–599  → thread 3 ─╯
```

Each thread computes its strip and sends the finished pixel data through a channel. The main thread receives all strips, assembles the image, and uploads it once.

---

## Step 1 — Decide how many threads

```rust
use std::thread;

let num_threads = thread::available_parallelism()
    .map(|n| n.get())
    .unwrap_or(4);
```

`available_parallelism()` returns the number of logical cores the OS is willing to give you — usually the number of CPU cores (or double, with hyperthreading). Using exactly as many threads as cores is the sweet spot: more threads than cores means context-switching overhead with no extra parallelism.

---

## Step 2 — The channel

Each thread sends one message: a `(strip_index, Vec<Color>)` pair — the strip's position and its computed pixels, row by row.

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel::<(usize, Vec<Color>)>();
```

---

## Step 3 — Spawn the threads

Divide the image into strips, one per thread. Each thread gets a clone of `tx`, computes its rows, and sends the result:

```rust
let rows_per_strip = HEIGHT / num_threads;

for t in 0..num_threads {
    let tx = tx.clone();

    thread::spawn(move || {
        let row_start = t * rows_per_strip;
        let row_end   = if t == num_threads - 1 { HEIGHT } else { row_start + rows_per_strip };

        let mut pixels = Vec::with_capacity((row_end - row_start) * WIDTH);

        for py in row_start..row_end {
            for px in 0..WIDTH {
                let c = pixel_to_complex(px, py);
                pixels.push(iter_to_color(mandelbrot(c)));
            }
        }

        tx.send((t, pixels)).unwrap();
    });
}

drop(tx); // close the original sender so rx knows when all threads are done
```

Three things to notice:

**`tx.clone()`** — each thread gets its own sender. The channel stays open until every clone is dropped (when each thread finishes). Dropping the original `tx` after the loop ensures the channel closes as soon as the last thread sends.

**`move` closure** — the thread owns `t`, `tx`, and accesses `WIDTH`, `HEIGHT`, etc. which are `const` (always available). The functions `pixel_to_complex`, `mandelbrot`, `iter_to_color` are also free functions — no ownership issues.

**Last strip gets the remainder** — `HEIGHT` might not divide evenly into `num_threads`. The `if t == num_threads - 1` guard gives the last thread any leftover rows.

---

## Step 4 — Collect results

The main thread receives strips as they arrive and writes them into the image:

```rust
let mut image = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, BLACK);

for (strip_idx, pixels) in rx {
    let row_start = strip_idx * rows_per_strip;

    for (i, color) in pixels.into_iter().enumerate() {
        let py = row_start + i / WIDTH;
        let px = i % WIDTH;
        image.set_pixel(px as u32, py as u32, color);
    }
}
```

`for (strip_idx, pixels) in rx` iterates until all senders are dropped — i.e., until all threads have sent their result. No explicit `join()` needed; the channel acts as the synchronisation point.

---

## Step 5 — Timing and comparison

Wrap everything in a timer and print both serial and parallel times:

```rust
let t0 = Instant::now();
// ... spawn threads, collect via rx ...
println!("parallel ({num_threads} threads): {:.2?}", t0.elapsed());
```

On a 4-core machine you should see roughly a 3–4× speedup. On an 8-core machine, 6–7×. The speedup is not perfectly linear because there is overhead in creating threads and sending data through the channel, and the OS may not schedule all threads on separate cores simultaneously.

---

## Full code

```rust
use macroquad::prelude::*;
use num_complex::Complex;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

const WIDTH:    usize = 800;
const HEIGHT:   usize = 600;
const MAX_ITER: u32   = 256;

const RE_MIN: f64 = -2.5;
const RE_MAX: f64 =  1.0;
const IM_MIN: f64 = -1.2;
const IM_MAX: f64 =  1.2;

fn pixel_to_complex(px: usize, py: usize) -> Complex<f64> {
    let re = RE_MIN + (px as f64 / WIDTH  as f64) * (RE_MAX - RE_MIN);
    let im = IM_MIN + (py as f64 / HEIGHT as f64) * (IM_MAX - IM_MIN);
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

#[macroquad::main("Mandelbrot")]
async fn main() {
    let num_threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);

    println!("rendering with {num_threads} threads...");

    let t0 = Instant::now();

    let (tx, rx) = mpsc::channel::<(usize, Vec<Color>)>();
    let rows_per_strip = HEIGHT / num_threads;

    for t in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            let row_start = t * rows_per_strip;
            let row_end   = if t == num_threads - 1 { HEIGHT } else { row_start + rows_per_strip };
            let mut pixels = Vec::with_capacity((row_end - row_start) * WIDTH);
            for py in row_start..row_end {
                for px in 0..WIDTH {
                    let (cx, cy) = pixel_to_complex(px, py);
                    pixels.push(iter_to_color(mandelbrot(cx, cy)));
                }
            }
            tx.send((t, pixels)).unwrap();
        });
    }
    drop(tx);

    let mut image = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, BLACK);

    for (strip_idx, pixels) in rx {
        let row_start = strip_idx * rows_per_strip;
        for (i, color) in pixels.into_iter().enumerate() {
            let py = row_start + i / WIDTH;
            let px = i % WIDTH;
            image.set_pixel(px as u32, py as u32, color);
        }
    }

    println!("parallel ({num_threads} threads): {:.2?}", t0.elapsed());

    let texture = Texture2D::from_image(&image);

    loop {
        clear_background(BLACK);
        draw_texture(&texture, 0.0, 0.0, WHITE);
        next_frame().await;
    }
}
```

---

## What the speedup tells you

If 4 threads give you a 3.8× speedup, that is close to the theoretical maximum for 4 cores. If you get less — say 2.5× — there are a few common reasons:

- **Hyperthreading**: two logical cores share one physical core and compete for the same execution units
- **Memory bandwidth**: all threads read/write to the same RAM — contention on the memory bus
- **Amdahl's law**: the serial parts of the program (thread creation, channel overhead, texture upload) cannot be parallelised and set a ceiling on speedup

The Mandelbrot computation is mostly independent per thread, so you should get close to linear speedup up to the physical core count.

---

## Exercise

> **TODO 1**: Add a command-line argument (or a `const`) for thread count. Run with 1, 2, 4, 8, and 16 threads and record the time. Plot the speedup. At what thread count does adding more threads stop helping?
>
> **TODO 2**: Add keyboard zoom: press `+`/`-` to zoom in/out at the center. On each zoom, re-render in parallel. You will need to move the render code into a function that takes the bounds as arguments.
>
> **TODO 3**: Add click-to-zoom: clicking a point makes it the new center and halves the width/height of the complex plane window. Re-render after each click.

---

## Key APIs

| API | What it does |
|-----|-------------|
| `thread::available_parallelism()` | Number of logical cores available |
| `mpsc::channel::<T>()` | Channel where messages are of type `T` |
| `tx.clone()` | Second sender to the same channel |
| `drop(tx)` | Close the original sender so `rx` ends when threads finish |
| `for item in rx` | Receive messages until all senders are dropped |
| `Vec::with_capacity(n)` | Pre-allocate to avoid reallocations in the hot loop |
