# Project 8 — Peggle Nights

Peggle is a ball-launcher game. A cannon at the top of the screen fires a ball that bounces through a field of coloured pegs. Blue pegs are worth points. Orange pegs are the objective — clear every orange peg and you win the level. A moving bucket at the bottom catches the ball for a free shot.

This project introduces two ideas that have not appeared in the course before:

**Rapier** — a 2D physics engine. We have been writing collision detection and velocity response by hand since Project 4. Rapier handles all of that for us. The ball bounces off pegs, walls, and the bucket without any manual collision code. You describe the shapes and Rapier does the rest.

**Modules** — the code is split across multiple files from Lesson 1 onward. Each file is a module: `peg.rs`, `cannon.rs`, `ball.rs`, and so on. `main.rs` becomes a thin coordinator that calls into the modules.

---

## What you will build

- A cannon that rotates to follow the mouse and shows a dotted trajectory preview
- A ball that launches, bounces realistically off pegs and walls, and exits at the bottom
- A field of pegs (blue and orange) that disappear when hit and flash before vanishing
- A bucket that oscillates at the bottom — catch the ball for an extra shot
- A score display and win/lose states based on orange pegs cleared and balls remaining

---

## New Rust concepts

| Concept | Where it appears |
|---------|-----------------|
| `mod`, `pub`, `use` | Modules primer, then every lesson |
| `#[derive(Debug, Clone, Copy)]` | `peg.rs` from Lesson 1 |
| `impl Default` | structs that have sensible zero values |
| Newtype wrapper | `BallHandle(RigidBodyHandle)` in `ball.rs` |
| `Vec::retain` | removing hit pegs without an index loop |
| `impl std::fmt::Display` | `Score` in `score.rs` |
| `?` operator | asset loading in `main.rs` |

---

## Project layout

```
lessons/8-peggle/
  lesson-01/project/    ← static scene, no physics
  lesson-02/project/    ← Rapier world, static peg colliders
  lesson-03/project/    ← mouse aiming
  lesson-04/project/    ← trajectory preview
  lesson-05/project/    ← ball launch and physics
  lesson-06/project/    ← collision events, peg removal
  lesson-07/project/    ← moving bucket
  lesson-08/project/    ← score, win/lose, game states
  lesson-09/project/    ← polish: flash, trail, fever mode
```

Each lesson's project is the previous lesson's solution with the next lesson's work left as TODOs.
