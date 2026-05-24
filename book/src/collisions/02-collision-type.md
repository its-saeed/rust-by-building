# Lesson 2 — The Collision Type

> **Goal**: Replace the `bool` with a value that carries the direction and depth of the collision.
>
> **Concepts**: `Option<T>`, `Some` / `None`, early return, `Vec2::normalize()`.

---

## Why `bool` isn't enough

Knowing two circles overlap is only the first step. To push them apart and change their velocities, we need:

- **Which direction** — the line from center A to center B, as a unit vector
- **How much** — the overlap depth, so we know how far to push

We encode both in a struct:

```rust
struct Collision {
    normal: Vec2,       // unit vector from a's center toward b's center
    penetration: f32,   // how much the circles overlap, in pixels
}
```

The `normal` tells us the direction of impact. The `penetration` tells us how much to correct.

---

## `Option<T>` — a value that might not exist

`detect_collision` either finds a collision or it doesn't. Rust represents this with `Option<T>`:

```rust
enum Option<T> {
    Some(T),   // there is a value
    None,      // there isn't
}
```

Our function signature:

```rust
fn detect_collision(a: &Body, b: &Body) -> Option<Collision>
```

When there's no overlap, return `None`. When there is, return `Some(collision)`. The caller can't accidentally use a collision value that doesn't exist — `Option` forces them to check.

---

## The collision normal

The normal is the unit vector pointing from A's center to B's center. "Unit vector" means its length is exactly 1.0 — it encodes direction only, with no magnitude.

To get a unit vector from any vector: divide each component by the vector's length. This is called **normalizing**:

```rust
impl Vec2 {
    fn normalize(self) -> Vec2 {
        let len = self.length();
        Vec2::new(self.x / len, self.y / len)
    }
}
```

After normalizing, the result always has length 1.0 regardless of the original length.

---

## `detect_collision`

```rust
fn detect_collision(a: &Body, b: &Body) -> Option<Collision> {
    let delta = b.position + (-a.position);
    let distance = delta.length();
    let radii_sum = a.radius + b.radius;

    if distance >= radii_sum {
        return None;
    }

    Some(Collision {
        normal: delta.normalize(),
        penetration: radii_sum - distance,
    })
}
```

`return None` exits the function early — the Rust way of "bail out if there's nothing to do." The last expression, `Some(...)`, is the return value when there is a collision.

`penetration` is `radii_sum - distance`: if the circles are supposed to be 50 pixels apart but are only 46, they overlap by 4 pixels.

---

## Calling it

Replace the `overlapping` call with `detect_collision`:

```rust
for i in 0..world.bodies.len() {
    for j in (i+1)..world.bodies.len() {
        if let Some(col) = detect_collision(&world.bodies[i], &world.bodies[j]) {
            let a = &world.bodies[i];
            let b = &world.bodies[j];
            draw_line(a.position.x, a.position.y, b.position.x, b.position.y, 2.0, RED);
            let _ = col; // will use in the next lesson
        }
    }
}
```

`if let Some(col) = ...` unwraps the `Option` — the body runs only when a collision is found, and `col` is the `Collision` value inside.

---

## Your task

Open `lessons/6-collisions/lesson-02/project/src/main.rs`.

1. Add `fn normalize(self) -> Vec2` to the `impl Vec2` block.
2. Define the `Collision` struct above `Body`.
3. Write `fn detect_collision(a: &Body, b: &Body) -> Option<Collision>`.
4. Update the nested loop to use `if let Some(col) = detect_collision(...)`.

The visual result is the same as lesson 1 — red lines on overlap. The difference is that `col` now carries the information we need to fix the collision in the next lesson.
