# Project 5 — Many Bodies

> **What you'll build**: A simulation of dozens of balls falling under gravity and bouncing off the screen edges. Click anywhere to spawn more.
>
> **Lessons**: 6 lessons.
>
> **Rust concepts covered**: `Vec<T>`, `iter_mut`, struct ownership, `&mut self` vs `&self`, force accumulation, mouse input.

## What's new in this project

Project 4 gave you one ball. This project gives you many — all managed by a single `World` struct that owns a `Vec<Body>`.

Adding a container for your objects forces you to deal with Rust's ownership rules in practice. How do you loop over all bodies and update each one? How do you add a new body while the simulation is running? These are the questions this project answers.

By the end, you'll have a proper physics world: bodies fall, bounce, and can be spawned interactively.

## What you'll build, lesson by lesson

| Lesson | What gets added |
|--------|-----------------|
| 1 — The World | `World` struct holding `Vec<Body>` |
| 2 — World::step() | Update all bodies each frame with `iter_mut` |
| 3 — Boundaries | World enforces screen edges for every body |
| 4 — Gravity | Bodies accelerate downward each step |
| 5 — Spawning | Click to add a ball at the mouse position |
| 6 — Polish | Random colors, body count display |

## A note on the code structure

Each lesson's starter file includes the complete, working code from the previous lesson. The foundation — `Vec2` with operator overloading and `Body` — comes from project 4 and is already in place. Your job each lesson is to add one layer on top.
