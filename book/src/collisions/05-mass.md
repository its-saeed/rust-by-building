# Lesson 5 — Mass

> **Goal**: Make heavier balls harder to push.
>
> **Concepts**: adding a field to an existing struct, deriving mass from radius, updating the impulse formula.

---

## The problem with equal mass

Right now every ball has the same mass. A tiny ball colliding with a giant ball sends the giant flying just as hard — which looks wrong.

Mass determines how much an impulse changes a body's velocity: `Δv = j / m`. A larger mass means a smaller velocity change for the same impulse.

---

## Adding `mass` to `Body`

```rust
struct Body {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
    color: Color,
    mass: f32,    // new
}
```

Update the constructor to compute mass from radius. We use `radius²` — proportional to the circle's area — which gives a satisfying physical feel without needing `π`:

```rust
impl Body {
    fn new(position: Vec2, velocity: Vec2, radius: f32, color: Color) -> Self {
        let mass = radius * radius;
        Body { position, velocity, radius, color, mass }
    }
}
```

A ball with radius 30 has mass 900. A ball with radius 10 has mass 100. The big ball is nine times heavier.

---

## Updating the impulse formula

The denominator in the impulse formula is `1/ma + 1/mb` — the combined **inverse mass**. Previously we hardcoded `2.0` (both masses = 1). Now we use the actual values:

```rust
let inv_mass_sum = 1.0 / self.bodies[*i].mass + 1.0 / self.bodies[*j].mass;
let j = -(1.0 + e) * vn / inv_mass_sum;
```

The velocity change for each body scales by its own inverse mass:

```rust
let impulse = col.normal * j;
self.bodies[*i].velocity = self.bodies[*i].velocity + impulse * (1.0 / self.bodies[*i].mass);
self.bodies[*j].velocity = self.bodies[*j].velocity + (-impulse) * (1.0 / self.bodies[*j].mass);
```

A heavier body has a smaller `1/mass`, so it gets a proportionally smaller velocity change from the same impulse.

---

## What you'll see

Spawn a large ball (radius 50+) and a small ball (radius 10). Roll the small one into the large one — the large one barely moves. Roll the large one into the small one — the small one flies off.

The system conserves momentum: the total `mass × velocity` before and after each collision is the same.

---

## Your task

Open `lessons/6-collisions/lesson-05/project/src/main.rs`.

1. Add `mass: f32` to `Body` and compute it in `new` as `radius * radius`.
2. In `resolve_collisions`, replace the hardcoded `2.0` denominator with `1.0 / bodies[i].mass + 1.0 / bodies[j].mass`.
3. Apply the velocity delta scaled by `1.0 / mass` for each body.
4. Spawn a few balls of different sizes to verify the mass ratios feel correct.

```sh
cargo run --bin coll-05
```
