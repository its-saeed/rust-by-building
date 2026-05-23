# Lesson 3 — Operator Overloading

> **Goal**: Make `Vec2 + Vec2`, `Vec2 * f32`, and `-Vec2` work with the standard operators.
>
> **Concepts**: `std::ops` traits, `impl Add for Vec2`, associated types, `type Output`.

---

## Why overload operators?

Right now you'd have to write a helper function to add two vectors:

```rust
fn add(a: Vec2, b: Vec2) -> Vec2 {
    Vec2::new(a.x + b.x, a.y + b.y)
}
```

That works, but physics code is full of vector math. In later lessons you'll write things like:

```
new_position = position + velocity * dt
```

With operator overloading that line looks exactly like math. Without it, it becomes:

```rust
let new_position = add(position, scale(velocity, dt));
```

Readable code matters. Let's fix it.

---

## Traits in Rust

A **trait** defines behaviour that a type can implement. `std::ops::Add` is the trait that powers the `+` operator. When Rust sees `a + b`, it calls `a.add(b)` — which means you can define exactly what `+` does for your type.

To use these traits, bring them into scope:

```rust
use std::ops::{Add, Mul, Neg};
```

---

## Implementing `Add`

```rust
impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
```

Breaking it down:

**`impl Add for Vec2`** — implement the `Add` trait for our `Vec2` type.

**`type Output = Vec2`** — an **associated type**: the result type of the `+` operation. Adding two `Vec2` values gives back a `Vec2`.

**`fn add(self, rhs: Vec2) -> Vec2`** — the method Rust calls when it sees `a + b`. `self` is the left side, `rhs` (right-hand side) is the right. Because `Vec2` is `Copy`, both are copied in — nothing is moved.

Once implemented:

```rust
let a = Vec2::new(1.0, 2.0);
let b = Vec2::new(3.0, 4.0);
let c = a + b;  // Vec2 { x: 4.0, y: 6.0 }
```

---

## Implementing `Mul<f32>`

Scaling a vector by a scalar is different from adding two vectors — the right-hand side is an `f32`, not a `Vec2`. The trait takes a type parameter for this:

```rust
impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Vec2 {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}
```

Now `velocity * dt` works, where `velocity` is a `Vec2` and `dt` is an `f32`.

---

## Implementing `Neg`

`Neg` is the unary minus — the `-` in `-velocity`:

```rust
impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}
```

This will come in handy in the next lesson when we need to reverse a direction.

---

## Your task

Open `lessons/4-ball-moves/lesson-03/project/src/main.rs`.

You'll find `Vec2` already defined. Add:

1. `impl Add for Vec2` — component-wise addition.
2. `impl Mul<f32> for Vec2` — scale both components by a scalar.
3. `impl Neg for Vec2` — negate both components.

To verify your operators work, try this in `main` before the loop:

```rust
let a = Vec2::new(100.0, 200.0);
let b = Vec2::new(50.0, 25.0);
let c = a + b;
println!("{} {}", c.x, c.y);  // should print: 150 225
```

The visual result is still the same (a circle on screen), but the math foundation is now in place.
