# Lesson 3 — Boundaries

> **Goal**: Keep every body on screen by having `World::step()` call `keep_in_bounds` on each body.
>
> **Concepts**: calling multiple methods in a single `iter_mut` loop, responsibility of the World vs the Body.

---

## Where should boundary checking live?

`Body` already has `keep_in_bounds` from project 4. The question is who calls it.

You could call `ball.keep_in_bounds()` in `main` — but `main` shouldn't know about every body. That's `World`'s job. The World knows all its bodies; it should be the one enforcing their boundaries.

---

## Extending `step`

The fix is one line added to the loop you wrote in the last lesson:

```rust
fn step(&mut self, dt: f32) {
    for body in self.bodies.iter_mut() {
        body.update(dt);
        body.keep_in_bounds();
    }
}
```

Both `update` and `keep_in_bounds` take `&mut self` on `Body`. Since `iter_mut` gives you `&mut Body`, both calls work in the same loop — no extra iteration needed.

Order matters: update first (move the body), then clamp (fix any boundary violation). If you clamp before updating, the ball would teleport back to its pre-move position every frame.

---

## `main` stays clean

The game loop in `main` doesn't change at all:

```rust
world.step(dt);
world.draw_all();
```

`main` calls one method. `World` handles all the bodies. This separation — `World` manages its bodies, `main` manages the `World` — will keep scaling cleanly as the engine grows.

---

## Your task

Open `lessons/5-many-bodies/lesson-03/project/src/main.rs`.

`World::step()` already calls `body.update(dt)`. Add:

1. Call `body.keep_in_bounds()` inside the same `iter_mut` loop, after `update`.

That's the whole change. Run it — all balls should now bounce off all four edges indefinitely.

Try adding more bodies with different radii and velocities. They all bounce correctly without any extra code in `main`.
