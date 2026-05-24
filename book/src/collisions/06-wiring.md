# Lesson 6 — Wiring It Up

> **Goal**: Clean up the collision pipeline into a complete, well-organised `World` implementation.
>
> **Concepts**: method decomposition, `Vec<(usize, usize, Collision)>` as a local type.

---

## What we have

After lesson 5, `resolve_collisions` does three things in one block:

1. Detects all collisions (nested loop, shared borrows)
2. Applies positional corrections (sequential mutable access)
3. Applies velocity impulses (sequential mutable access)

It works, but it's getting long. This lesson splits the detection into a dedicated method and reviews the final shape of `World`.

---

## Splitting detection out

Move the detection loop into its own method that returns the list of collisions:

```rust
impl World {
    fn detect_collisions(&self) -> Vec<(usize, usize, Collision)> {
        let mut result = Vec::new();
        for i in 0..self.bodies.len() {
            for j in (i+1)..self.bodies.len() {
                if let Some(col) = detect_collision(&self.bodies[i], &self.bodies[j]) {
                    result.push((i, j, col));
                }
            }
        }
        result
    }
}
```

`&self` (shared borrow) is correct here — detection only reads. The method returns a `Vec` of tuples: indices plus the collision data.

---

## The resolution method

`resolve_collisions` now calls `detect_collisions` and applies the results:

```rust
impl World {
    fn resolve_collisions(&mut self) {
        let collisions = self.detect_collisions();

        for (i, j, col) in &collisions {
            // positional correction
            let correction = col.normal * (col.penetration / 2.0);
            self.bodies[*i].position = self.bodies[*i].position + (-correction);
            self.bodies[*j].position = self.bodies[*j].position + correction;

            // velocity response
            let vrel = self.bodies[*i].velocity + (-self.bodies[*j].velocity);
            let vn = vrel.dot(col.normal);
            if vn <= 0.0 { continue; }

            let inv_mass_sum = 1.0 / self.bodies[*i].mass + 1.0 / self.bodies[*j].mass;
            let j_val = -(1.0 + 1.0) * vn / inv_mass_sum;

            let impulse = col.normal * j_val;
            self.bodies[*i].velocity = self.bodies[*i].velocity + impulse * (1.0 / self.bodies[*i].mass);
            self.bodies[*j].velocity = self.bodies[*j].velocity + (-impulse) * (1.0 / self.bodies[*j].mass);
        }
    }
}
```

Note: `self.detect_collisions()` takes `&self` (shared borrow), which ends before `resolve_collisions` continues with `&mut self` access. No borrow conflict.

---

## The complete `World::step`

```rust
fn step(&mut self, dt: f32) {
    for body in self.bodies.iter_mut() {
        body.update(dt);
        body.keep_in_bounds();
    }
    self.resolve_collisions();
}
```

Three lines. Each does one thing. The game loop stays clean:

```rust
loop {
    let dt = get_frame_time();
    clear_background(BLACK);

    if is_mouse_button_pressed(MouseButton::Left) {
        let (mx, my) = mouse_position();
        world.add_body(Body::new(
            Vec2::new(mx, my),
            Vec2::new(0.0, 0.0),
            macroquad::rand::gen_range(10.0, 40.0),
            random_color(),
        ));
    }

    world.step(dt);
    world.draw_all();
    draw_text(&format!("Bodies: {}", world.bodies.len()), 10.0, 24.0, 24.0, WHITE);

    next_frame().await;
}
```

---

## Your task

Open `lessons/6-collisions/lesson-06/project/src/main.rs`.

1. Extract `fn detect_collisions(&self) -> Vec<(usize, usize, Collision)>` from `resolve_collisions`.
2. Update `resolve_collisions` to call `self.detect_collisions()` and then apply corrections and impulses.
3. Verify the game loop is clean — no collision logic in `main`.

```sh
cargo run --bin coll-06
```

Spawn a crowd of balls. Watch them pile up, push each other, bounce. That's project 6 complete.

**What's next:** Project 7 — Stable World — introduces a restitution coefficient per body (so you can have bouncy and sticky balls), fixes the energy drift from Euler integration, and adds a spatial grid so collision detection stays fast with hundreds of bodies.
