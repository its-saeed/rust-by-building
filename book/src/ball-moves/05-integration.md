# Lesson 5 — Integration

> **Goal**: Make the ball actually move by advancing its position each frame based on its velocity.
>
> **Concepts**: Euler integration, delta time, `&mut self`, `get_frame_time`.

---

## Velocity and position

**Position** is where the ball is right now — a point in space.

**Velocity** is how fast and in which direction the ball is moving — a direction with a magnitude, measured in pixels per second.

Each frame we advance position by velocity, scaled by how much time passed:

```
position = position + velocity * dt
```

`dt` is **delta time** — the duration of the previous frame in seconds. On a 60 Hz display, `dt` is about `0.016` (one-sixtieth of a second).

This is called **Euler integration**.

---

## What Euler integration actually does

In the real world, a ball under gravity follows a smooth curve — its velocity changes continuously at every instant. A computer can't compute infinite instants. Instead, it samples the world at discrete moments:

```
time:    t=0          t=0.016       t=0.032       t=0.048
         │             │             │             │
         ●─────────────●─────────────●─────────────●
         │  straight    │  straight   │  straight   │
         │  line step   │  line step  │  line step  │
```

At each step, Euler integration makes a single assumption: **velocity is constant for the duration of this step**. Multiply that velocity by `dt`, add it to position, done.

When velocity truly is constant (no forces), the approximation is exact — the ball moves in a straight line and we compute it perfectly. When velocity is changing (gravity is pulling the ball faster each moment), we're using the velocity from the *start* of the step, ignoring the change that happens *during* it. That introduces a small error.

Smaller `dt` → shorter steps → less error per step. At 60 FPS the steps are small enough that the path looks correct to the eye.

The name comes from Leonhard Euler, the 18th-century mathematician who formalized the method. It's the simplest numerical integrator there is — one addition, one multiplication per axis, per frame. More accurate methods exist (Verlet, Runge-Kutta), but they're more complex and unnecessary for a game at this scale.

---

## Why delta time matters

Without `dt`, your ball moves faster on a 120 Hz monitor than on a 60 Hz one, because the loop runs more often. Multiplying velocity by `dt` makes movement **frame-rate independent**: a ball with velocity `(200.0, 0.0)` moves exactly 200 pixels per second regardless of frame rate.

macroquad gives you `dt` via:

```rust
let dt = get_frame_time();
```

Call this once at the start of each frame, before you update anything.

---

## The `update` method

The `update` method advances the body by one time step. It needs to **change** `position`, so it takes `&mut self`:

```rust
impl Body {
    fn update(&mut self, dt: f32) {
        self.position = self.position + self.velocity * dt;
    }
}
```

`self.position + self.velocity * dt` uses the operators you implemented in lesson 3:
- `self.velocity * dt` calls `Mul<f32>` → scales the velocity vector
- `+ self.position` calls `Add` → advances the position

---

## Why `Vec2` must be `Copy`

Look at this line closely:

```rust
self.position = self.position + self.velocity * dt;
```

The right-hand side reads `self.position` and `self.velocity`. The left-hand side writes to `self.position`. In a language with move semantics, reading a value out of a struct **moves** it — the struct would be left partially initialized, which Rust won't allow.

This is the moment `#[derive(Clone, Copy)]` from lesson 2 pays off. Let's unpack what those two words actually do.

### `Clone` — explicit duplication

`Clone` adds a `.clone()` method that produces an independent copy of the value:

```rust
let a = Vec2::new(1.0, 2.0);
let b = a.clone();   // b is a fresh copy; a is untouched
```

Cloning is always **explicit** — Rust never clones behind your back. This matters for expensive types like `Vec<T>` or `String`, where cloning allocates new heap memory. For those types, you want to see every clone in the source.

`#[derive(Clone)]` generates the implementation automatically: it clones each field. For `Vec2`, that means copying two `f32` values — essentially free.

### `Copy` — implicit duplication

`Copy` changes the default behaviour of field access, assignment, and function calls. Without it, using a value moves it. With it, using a value silently copies the bytes and leaves the original intact.

| | Without `Copy` | With `Copy` |
|---|---|---|
| `let b = a;` | moves `a` — `a` is gone | copies `a` — both valid |
| passing to a fn | moves the value | copies it — original still works |
| reading a struct field | moves out — struct left partial | copies — struct untouched |

That last row is what matters in `update`. When Rust evaluates `self.velocity * dt`, it needs the value of `self.velocity`. With `Copy`, it silently duplicates the two floats. `self` remains fully intact, and the write to `self.position` at the end works fine.

Without `Copy`, the compiler would reject this:

```
error[E0507]: cannot move out of `self.velocity` which is behind a mutable reference
```

### Why both?

`Copy` is a subtrait of `Clone` — it extends it. Rust requires that any `Copy` type also implements `Clone`. If you try to derive only `Copy`:

```
error: the trait `Clone` is not implemented for `Vec2`
```

So the two always appear together.

### When to add `Copy`

The question is: is this type small, free of heap allocations, and cheap to duplicate?

| Type | `Copy`? | Why |
|------|---------|-----|
| `f32`, `i32`, `bool` | yes | primitive, bitwise copy is always correct |
| `Vec2` (two `f32`s) | yes | small, no heap, copying is instant |
| `Body` | no | will grow; one clear owner is the right model |
| `String`, `Vec<T>` | no | own heap memory — a bitwise copy would double-free |

`Vec2` qualifies on all counts. Every physics expression — `position + velocity * dt` — passes `Vec2` values around freely. With `Copy`, the math reads naturally. Without it, you'd need `.clone()` everywhere to avoid moving fields out of structs.

---

## `&mut self` at the call site

When you call a `&mut self` method, Rust requires `mut` on the variable:

```rust
let mut ball = Body::new(...);
ball.update(dt);  // only works because ball is `mut`
```

Without `mut`, the compiler will tell you the variable needs to be mutable. This is intentional — Rust makes mutation explicit so you always know which variables can change.

---

## Putting it together

The game loop now looks like:

```rust
loop {
    let dt = get_frame_time();
    clear_background(BLACK);

    ball.update(dt);
    ball.draw();

    next_frame().await;
}
```

Order matters: update first (move the ball), then draw (show where it ended up).

---

## Your task

Open `lessons/4-ball-moves/lesson-05/project/src/main.rs`.

You'll find `Body` already defined with `new` and `draw`. Add:

1. `fn update(&mut self, dt: f32)` to the `impl Body` block — advance `position` by `velocity * dt`.
2. In `main`, change `ball` to `let mut ball` so you can call `update` on it.
3. Give the ball a non-zero starting velocity — try `Vec2::new(150.0, 100.0)`.
4. Call `ball.update(dt)` inside the loop before `ball.draw()`.

Run it. The ball moves — and flies off the screen. We'll fix that in the next lesson.
