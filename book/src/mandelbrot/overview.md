# Project 10 — Mandelbrot

You have two CPU cores doing nothing. This project puts them to work.

The Mandelbrot set is one of the most famous images in mathematics — an infinitely detailed fractal that reveals new structure no matter how far you zoom in. Rendering it is pure computation: no files, no network, no user input. Every pixel is independent of every other pixel. That makes it a perfect problem for threads.

---

## What you are building

A macroquad window that renders the Mandelbrot set. The first version is single-threaded — it computes every pixel one after another on the main thread, which you will time. The second version splits the image into horizontal strips and computes each strip on a separate thread in parallel. On a multi-core machine, the speedup is dramatic and immediate.

```
┌──────────────────────────────────────────────┐
│                                              │
│         Mandelbrot Set                       │
│                                              │
│   ░░░░░░░░░░████████████░░░░░░░░░░░░░░░░    │
│   ░░░░░░███████████████████░░░░░░░░░░░░░    │
│   ░░░░████████████████████████░░░░░░░░░░    │
│   ░░██████████████████████████████░░░░░░    │
│   ░░░░████████████████████████░░░░░░░░░░    │
│   ░░░░░░███████████████████░░░░░░░░░░░░░    │
│   ░░░░░░░░░░████████████░░░░░░░░░░░░░░░░    │
│                                              │
└──────────────────────────────────────────────┘
```

---

## Why this project

Chapter 6 gave two reasons to use threads: blocking I/O and CPU-bound parallelism. Tele-Sketch (and the later Chat Server) cover the I/O side. Mandelbrot is the pure CPU case — work that has no waiting, only computing. Threads spread that computation across all available cores and the result is a measurable, visible speedup.

---

## Concepts covered

| Concept | Where it appears |
|---------|-----------------|
| `thread::spawn` + `move` | One thread per image strip |
| `mpsc::channel` | Threads send completed strips to the main thread |
| `tx.clone()` | Each thread gets its own sender |
| Timing with `Instant` | Measure and compare serial vs parallel |
| `Texture2D` / `Image` | macroquad pixel-level rendering |
| f64 arithmetic | Complex number iteration |
