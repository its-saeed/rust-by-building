# Lesson 2 — World::step()

> **Goal**: Update all bodies each frame by adding a `step` method to `World`.
>
> **Concepts**: `iter_mut`, mutable iteration, the difference between `&self` and `&mut self` in practice.

---

## Why drawing worked with `&self` but updating won't

`draw_all` took `&self` because drawing only reads data. `step` needs to call `body.update(dt)` on every body, which changes `position`. You can't do that through a shared reference.

If you tried:

```rust
fn step(&self, dt: f32) {
    for body in &self.bodies {
        body.update(dt); // error: `body` is `&Body`, update needs `&mut Body`
    }
}
```

Rust rejects it. `&self.bodies` yields `&Body` references — read-only. `update` requires `&mut self` on the body.

---

## `iter_mut` — mutable iteration

`iter_mut()` yields `&mut Body` — one mutable reference to each element in turn:

```rust
fn step(&mut self, dt: f32) {
    for body in self.bodies.iter_mut() {
        body.update(dt);
    }
}
```

With `&mut Body`, calling `body.update(dt)` works because `update` takes `&mut self`.

The method signature becomes `&mut self` because mutably iterating the Vec requires a mutable borrow of `World`.

---

## One `&mut` at a time

Rust's core rule: you can have **one mutable reference** to a value, or **any number of shared references** — never both at once.

`iter_mut` upholds this automatically: it hands you one `&mut Body` per loop iteration, never two references to the same body at the same time. This is what makes the borrow checker useful — code that would cause a data race simply doesn't compile.

---

## The updated game loop

```rust
loop {
    let dt = get_frame_time();
    clear_background(BLACK);

    world.step(dt);
    world.draw_all();

    next_frame().await;
}
```

Step first (move), draw second (show where they ended up).

---

## Your task

Open `lessons/5-many-bodies/lesson-02/project/src/main.rs`.

`World` already has `new`, `add_body`, and `draw_all`. Add:

1. `fn step(&mut self, dt: f32)` — use `self.bodies.iter_mut()` and call `body.update(dt)` on each.
2. In `main`, call `world.step(dt)` before `world.draw_all()`.
3. Give each body a non-zero starting velocity.

Run it. The balls move — and fly off the screen. That's expected. Boundaries come next.
