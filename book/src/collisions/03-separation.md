# Lesson 3 — Separating Bodies

> **Goal**: Push overlapping balls apart so they don't stick or tunnel through each other.
>
> **Concepts**: the two-pass borrow pattern, why simultaneous `&mut` borrows of a `Vec` fail.

---

## Positional correction

Once we know two circles overlap by `penetration` pixels along `normal`, the fix is straightforward: push each ball half the overlap distance in opposite directions.

```
before:            after:
  ●──────●    →    ●    ●
  (overlap)        (gap = 0)
```

Ball A moves `penetration / 2` in the `-normal` direction (away from B).  
Ball B moves `penetration / 2` in the `+normal` direction (away from A).

```rust
let correction = col.normal * (col.penetration / 2.0);
body_a.position = body_a.position + (-correction);
body_b.position = body_b.position + correction;
```

Simple. But getting two `&mut Body` references from the same `Vec` is where Rust pushes back.

---

## The problem: two mutable borrows

The natural approach is to take mutable references to both bodies in the loop:

```rust
// This does NOT compile:
for i in 0..self.bodies.len() {
    for j in (i+1)..self.bodies.len() {
        let a = &mut self.bodies[i];  // mutable borrow of self.bodies
        let b = &mut self.bodies[j];  // second mutable borrow — REJECTED
        // ...
    }
}
```

```
error[E0499]: cannot borrow `self.bodies` as mutable more than once at a time
```

Rust's rule: **at most one mutable borrow of a value at a time.** Both `&mut self.bodies[i]` and `&mut self.bodies[j]` borrow the whole `Vec` mutably. Even though they target different elements, the compiler doesn't track index-level disjointness — it only sees two exclusive borrows of the same `Vec`.

This isn't a flaw in the borrow checker. If `i == j`, allowing both borrows would let you alias a single value through two `&mut` pointers — undefined behaviour.

---

## The solution: two passes

The fix is to separate reading from writing. In the first pass, detect all collisions using only shared borrows (`&Body`). In the second pass, apply corrections using mutable borrows — one body at a time, sequentially.

```rust
// Pass 1: detect (shared borrows — multiple allowed)
let mut collisions: Vec<(usize, usize, Collision)> = Vec::new();
for i in 0..self.bodies.len() {
    for j in (i+1)..self.bodies.len() {
        if let Some(col) = detect_collision(&self.bodies[i], &self.bodies[j]) {
            collisions.push((i, j, col));
        }
    }
}

// Pass 2: apply (sequential mutable access — no overlap)
for (i, j, col) in &collisions {
    let correction = col.normal * (col.penetration / 2.0);
    self.bodies[*i].position = self.bodies[*i].position + (-correction);
    self.bodies[*j].position = self.bodies[*j].position + correction;
}
```

In Pass 2, each line borrows from `self.bodies` and releases the borrow before the next line begins. The borrows are sequential, never simultaneous — so the borrow checker accepts it.

---

## Where to put this

Add a method to `World` that runs both passes:

```rust
impl World {
    fn resolve_collisions(&mut self) {
        let mut collisions: Vec<(usize, usize, Collision)> = Vec::new();
        for i in 0..self.bodies.len() {
            for j in (i+1)..self.bodies.len() {
                if let Some(col) = detect_collision(&self.bodies[i], &self.bodies[j]) {
                    collisions.push((i, j, col));
                }
            }
        }
        for (i, j, col) in &collisions {
            let correction = col.normal * (col.penetration / 2.0);
            self.bodies[*i].position = self.bodies[*i].position + (-correction);
            self.bodies[*j].position = self.bodies[*j].position + correction;
        }
    }
}
```

Call it inside `World::step`, after updating all bodies:

```rust
fn step(&mut self, dt: f32) {
    for body in self.bodies.iter_mut() {
        body.update(dt);
        body.keep_in_bounds();
    }
    self.resolve_collisions();
}
```

---

## Your task

Open `lessons/6-collisions/lesson-03/project/src/main.rs`.

1. Add `fn resolve_collisions(&mut self)` to `impl World` using the two-pass pattern above.
2. Call `self.resolve_collisions()` at the end of `World::step`.
3. Remove the visual debug lines from the game loop — they're no longer needed.

Run it. Balls now push each other apart when they overlap. They don't bounce yet — that's the next lesson. But they should no longer overlap or stick.
