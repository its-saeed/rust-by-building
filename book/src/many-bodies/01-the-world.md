# Lesson 1 — The World

> **Goal**: Create a `World` struct that owns a collection of bodies and draws them all.
>
> **Concepts**: `Vec<T>`, ownership of collections, `push`, iterating with `for`, `&self` on read-only methods.

---

## From one ball to many

In project 4 you had one `Body` living in `main`. That was fine for one ball. For many balls you need a container — something that owns a list of bodies and knows how to operate on all of them.

That's `World`.

---

## `Vec<T>` — a growable list

`Vec<T>` is Rust's dynamic array. The `T` is a type parameter — `Vec<Body>` is a list of `Body` values, `Vec<f32>` is a list of floats.

```rust
let mut bodies: Vec<Body> = Vec::new();
```

The `mut` is required because adding to a `Vec` is mutation. Key operations:

```rust
bodies.push(body);   // add to the end — Body is moved in
bodies.len();        // number of elements
bodies[0];           // index — gives a reference, not ownership
```

`push` **moves** the value into the Vec. After pushing, you no longer own `body` — the Vec does. This is the same ownership rule you've seen before: one owner at a time.

---

## The `World` struct

```rust
struct World {
    bodies: Vec<Body>,
}
```

`World` owns its `Vec<Body>`, and the Vec owns each `Body` inside it. The ownership chain: `World` → `Vec` → each `Body`.

The constructor:

```rust
impl World {
    fn new() -> Self {
        World {
            bodies: Vec::new(),
        }
    }
}
```

Adding a body:

```rust
impl World {
    fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }
}
```

`add_body` takes `&mut self` because pushing changes the Vec. It takes `body: Body` by value — the body is moved into the Vec.

---

## Drawing all bodies

To draw every body, iterate over the Vec and call `draw` on each:

```rust
impl World {
    fn draw_all(&self) {
        for body in &self.bodies {
            body.draw();
        }
    }
}
```

`&self.bodies` borrows the Vec as a sequence of shared references. Each `body` in the loop is a `&Body`. Since `Body::draw` takes `&self`, calling `body.draw()` works directly.

`draw_all` takes `&self` because it only reads — nothing changes.

---

## Your task

Open `lessons/5-many-bodies/lesson-01/project/src/main.rs`.

1. Add a `World` struct with a `bodies: Vec<Body>` field.
2. Implement `World::new()` and `World::add_body(&mut self, body: Body)`.
3. Implement `World::draw_all(&self)` — loop over `&self.bodies` and call `draw` on each.
4. In `main`, create a `World` and add three or more bodies at different positions.

Run it — several static circles should appear on screen.
