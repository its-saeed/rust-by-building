# Lesson 4 — Velocity Response

> **Goal**: Change velocities so balls bounce off each other correctly.
>
> **Concepts**: `Vec2::dot()`, early return from a loop body, the impulse formula from the primer.

---

## Adding `Vec2::dot()`

The dot product takes two vectors and returns a scalar. It belongs in `impl Vec2`:

```rust
impl Vec2 {
    fn dot(self, other: Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }
}
```

---

## Applying the impulse

Inside `resolve_collisions`, after the positional correction, add the velocity step:

```rust
for (i, j, col) in &collisions {
    // --- positional correction (already written) ---
    let correction = col.normal * (col.penetration / 2.0);
    self.bodies[*i].position = self.bodies[*i].position + (-correction);
    self.bodies[*j].position = self.bodies[*j].position + correction;

    // --- velocity response ---
    let vrel = self.bodies[*i].velocity + (-self.bodies[*j].velocity);
    let vn = vrel.dot(col.normal);

    if vn <= 0.0 {
        continue;  // already separating — skip
    }

    let e = 1.0;  // restitution: 1.0 = perfectly elastic
    let j = -(1.0 + e) * vn / 2.0;  // equal mass: 1/ma + 1/mb = 2.0

    self.bodies[*i].velocity = self.bodies[*i].velocity + col.normal * j;
    self.bodies[*j].velocity = self.bodies[*j].velocity + (-col.normal) * j;
}
```

Let's walk through each part.

---

## `vrel` and `vn`

```rust
let vrel = self.bodies[*i].velocity + (-self.bodies[*j].velocity);
let vn = vrel.dot(col.normal);
```

`vrel` is the velocity of body i relative to body j. `vn` projects that onto the collision normal — it's the speed at which the two balls are approaching each other along the line connecting their centers.

---

## The skip condition

```rust
if vn <= 0.0 {
    continue;
}
```

`vn ≤ 0` means the balls are already moving apart (or moving parallel to the contact). Applying an impulse now would pull them back together. `continue` skips the rest of the loop body and moves on to the next collision.

---

## The impulse

```rust
let e = 1.0;
let j = -(1.0 + e) * vn / 2.0;
```

`e = 1.0` is the restitution coefficient — perfectly elastic means full bounce. `2.0` in the denominator is `1/ma + 1/mb` with both masses equal to 1.

Since `vn > 0` at this point, `j` is negative. Multiplying by the normal and adding to body i's velocity pushes i in the `-normal` direction (away from j), which is correct.

---

## Applying to velocities

```rust
self.bodies[*i].velocity = self.bodies[*i].velocity + col.normal * j;
self.bodies[*j].velocity = self.bodies[*j].velocity + (-col.normal) * j;
```

Body i gets pushed by `j * normal`. Body j gets the equal and opposite push: `j * (-normal)`. Since `j` is negative, body i actually moves in the `-normal` direction and body j in the `+normal` direction — both away from the contact point.

---

## Your task

Open `lessons/6-collisions/lesson-04/project/src/main.rs`.

1. Add `fn dot(self, other: Vec2) -> f32` to `impl Vec2`.
2. In `resolve_collisions`, after the positional correction block, add the velocity response: compute `vrel`, `vn`, check `vn <= 0.0`, compute `j`, apply to both velocities.

Run it. Balls should now bounce off each other. Head-on collisions will look right immediately. Glancing blows work too — the perpendicular component of velocity is unaffected, only the component along the normal changes.

```sh
cargo run --bin coll-04
```

In the next lesson we add mass so larger balls are harder to knock around.
