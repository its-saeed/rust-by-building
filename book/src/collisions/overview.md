# Project 6 — Balls Collide

> **What you'll build**: Add circle-circle collision detection and impulse-based response so balls bounce off each other, not just off walls.
>
> **Lessons**: 6 lessons.
>
> **Rust concepts covered**: `Option<T>`, `struct` with associated methods, index-based Vec loops, the two-pass borrow pattern.

---

## What changes

Projects 4 and 5 built bodies that bounce off walls. Balls pass straight through each other. This project adds the missing piece: detecting when two balls overlap and computing the correct velocity change so they bounce.

## The collision pipeline

Each frame, after moving all bodies, we run three steps:

```
detect overlaps → separate bodies → change velocities
```

**Detect** — for every pair of balls, check whether they overlap and by how much.

**Separate** — push overlapping balls apart so they don't stick together or tunnel through each other.

**Respond** — compute an impulse (an instantaneous velocity change) so the balls bounce in the physically correct direction.

## What you'll learn

The physics involves a new mathematical tool — the **dot product** — which measures how much two vectors point in the same direction. A dedicated primer page before lesson 4 explains it before the code uses it.

The Rust challenge is accessing two bodies from the same `Vec` at the same time. Lesson 3 shows why the naive approach fails and introduces the **two-pass pattern**: collect collision data in one read-only pass, then apply it in a second pass.
