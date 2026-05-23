# Project 4 — A Ball Moves

> **What you'll build**: A single ball that flies around the screen and bounces off the edges — the foundation of the physics engine.
>
> **Lessons**: 6 lessons.
>
> **Rust concepts covered**: `struct`, `impl`, operator overloading (`Add`, `Mul`, `Neg`), `#[derive]`, method receivers (`&self` vs `&mut self`), Euler integration, external crates.

## What is a physics engine?

A physics engine is a program that simulates how objects move and interact. At its core it's a loop: every frame, it advances time by a small step, moves each object according to its velocity, and handles any collisions.

By the end of this course you'll have built one from scratch — gravity, collisions, stacking, and two shape types. This first project builds the absolute foundation: a single ball that moves and bounces.

## What you'll build, lesson by lesson

| Lesson | What gets added |
|--------|-----------------|
| 1 — Hello macroquad | Game loop, draw a circle on screen |
| 2 — Vec2 | A 2D vector struct to represent positions and directions |
| 3 — Operator Overloading | `+`, `*`, `-` work on Vec2 like real math |
| 4 — The Body | A `Body` struct that groups position, velocity, and radius |
| 5 — Integration | The ball actually moves (`position += velocity * dt`) |
| 6 — Wall Bouncing | The ball bounces off all four screen edges |

## A note on macroquad

All six lessons use [macroquad](https://macroquad.rs) — a minimal Rust game library. It gives you a window, a game loop, and drawing functions. No OpenGL setup, no hundred-line boilerplate. One dependency, one attribute, and you have a running window.

You'll see results from the very first lesson. Every lesson after that builds on a working, visible program.

## How lessons work

Each lesson has two parts:

1. **Read** — this book. Work through the chapter before touching any code.
2. **Project step** — open `lessons/4-ball-moves/lesson-NN/project/src/main.rs` and complete the TODOs.

Run your project at any time with:

```sh
cargo run --bin ball-NN
```

The window opens immediately. You'll see your changes in real time.
