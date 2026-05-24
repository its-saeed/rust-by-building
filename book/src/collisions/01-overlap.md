# Lesson 1 — Overlap Detection

> **Goal**: Determine whether two circles overlap.
>
> **Concepts**: `Vec2::length()`, `f32::sqrt()`, nested index loops, adding methods to an existing type.

---

## When do two circles overlap?

Two circles overlap when the distance between their centers is less than the sum of their radii.

```
        r_a         r_b
    ◄───────►   ◄───────►
         ●─────────●
              d

  overlap when: d < r_a + r_b
  touching at:  d = r_a + r_b
  gap when:     d > r_a + r_b
```

The distance between two points `(ax, ay)` and `(bx, by)` is:

```
d = sqrt((bx - ax)² + (by - ay)²)
```

In terms of our `Vec2` type: compute the vector from `a` to `b`, then take its length.

---

## Adding `Vec2::length()`

`length()` is the first method we add to `Vec2` that does computation rather than construction. It belongs in the existing `impl Vec2` block:

```rust
impl Vec2 {
    fn new(x: f32, y: f32) -> Self { Vec2 { x, y } }

    fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
```

`f32::sqrt()` is a method on `f32` — it returns the square root. Writing it as `(expression).sqrt()` chains it directly onto the computed value.

`length` takes `self` by value (not `&self`) because `Vec2` is `Copy` — Rust silently copies the two floats. No reference needed.

---

## Writing `overlapping`

```rust
fn overlapping(a: &Body, b: &Body) -> bool {
    let delta = b.position + (-a.position);
    let distance = delta.length();
    distance < a.radius + b.radius
}
```

`b.position + (-a.position)` uses the `Add` and `Neg` implementations from project 4 to get the vector from `a` to `b`. We then take its length and compare.

---

## Checking all pairs

To check every possible pair of bodies, loop over all index combinations where `i < j`:

```rust
for i in 0..world.bodies.len() {
    for j in (i+1)..world.bodies.len() {
        if overlapping(&world.bodies[i], &world.bodies[j]) {
            // ...
        }
    }
}
```

Starting `j` at `i+1` ensures each pair is checked once. Starting at `0` would check `(0,1)` and `(1,0)` — the same collision twice.

For visual feedback, draw a red line between the centers of any overlapping pair:

```rust
let a = &world.bodies[i];
let b = &world.bodies[j];
draw_line(a.position.x, a.position.y, b.position.x, b.position.y, 2.0, RED);
```

The balls still pass through each other — we're only detecting, not resolving. That's the next three lessons.

---

## Your task

Open `lessons/6-collisions/lesson-01/project/src/main.rs`.

1. Add `fn length(self) -> f32` to the `impl Vec2` block.
2. Write `fn overlapping(a: &Body, b: &Body) -> bool` using `length()`.
3. After `world.draw_all()`, add the nested index loop that draws a red line between overlapping centers.

Run it and push two balls together — you should see the red line appear the moment they touch.

```sh
cargo run --bin coll-01
```
