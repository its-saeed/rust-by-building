# Lesson 4 — The Body

> **Goal**: Create a `Body` struct that groups everything a physics object needs: position, velocity, radius.
>
> **Concepts**: struct composition, `&self` vs `&mut self`, `fn draw`, method calls with dot notation.

---

## Grouping related state

A physics body has several pieces of state that always travel together:

- Where is it? → `position: Vec2`
- How fast and in which direction is it moving? → `velocity: Vec2`
- How big is it? → `radius: f32`

Instead of passing these around as separate arguments everywhere, we bundle them into a struct:

```rust
struct Body {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
}
```

This is **struct composition** — a struct whose fields are themselves structs. `position` and `velocity` are `Vec2` values. Because `Vec2` is `Copy`, `Body` can store them directly by value with no extra complexity.

---

## A constructor

```rust
impl Body {
    fn new(position: Vec2, velocity: Vec2, radius: f32) -> Self {
        Body { position, velocity, radius }
    }
}
```

The pattern is the same as `Vec2::new`. Field shorthand keeps it clean — when the parameter name matches the field name, you write it once.

Creating a body:

```rust
let ball = Body::new(
    Vec2::new(400.0, 300.0),
    Vec2::new(0.0, 0.0),
    20.0,
);
```

---

## `&self` and `&mut self`

Methods can borrow `self` in two ways:

**`&self`** — read-only borrow. The method can look at the data but not change it.

**`&mut self`** — mutable borrow. The method can change the struct's fields.

A `draw` method only needs to read the body's data — it never changes it. So it takes `&self`:

```rust
impl Body {
    fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, WHITE);
    }
}
```

Inside the method, `self` refers to the current instance. `self.position` accesses its `position` field.

Calling it:

```rust
ball.draw();
```

Rust automatically borrows `ball` as `&ball` when calling a `&self` method. You don't need to write `&` at the call site.

In the next lesson, `update` will need `&mut self` because it changes `position` — that's the difference in practice.

---

## Your task

Open `lessons/4-ball-moves/lesson-04/project/src/main.rs`.

You'll find `Vec2` with all three operators already implemented. Add:

1. A `Body` struct with `position: Vec2`, `velocity: Vec2`, and `radius: f32`.
2. `impl Body` with a `new(position, velocity, radius) -> Self` constructor.
3. `impl Body` with a `draw(&self)` method that calls `draw_circle`.
4. In `main`, replace the raw `draw_circle` call with a `Body` and `body.draw()`.

The result is still the same circle — but the code is now organised around the object that will grow into a full physics body over the next two lessons.
