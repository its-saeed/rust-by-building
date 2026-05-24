# Ownership in Plain English

Rust's ownership system is the thing that makes it different from every other mainstream language. It can feel strange at first, but it exists for a concrete reason — and once you understand the reason, the rules click into place.

---

## The problem

Programs need memory to store data. When you're done with some data, that memory should be freed. The question is: who decides when you're done?

**Manual memory management (C, C++):** The programmer frees memory by hand. Powerful, but easy to get wrong. Freeing too early leaves a dangling pointer — accessing freed memory causes crashes or silent corruption. Freeing the same memory twice also crashes. Forgetting to free causes leaks.

**Garbage collection (Python, JavaScript, Go):** A runtime periodically scans for data no one is using and frees it automatically. Safe, but you pay a performance cost, and you can't predict exactly when memory is freed.

**Rust's approach:** The compiler tracks who owns each value and inserts the cleanup automatically — with zero runtime overhead and guaranteed safety. If your code compiles, an entire class of memory bugs cannot occur.

---

## Rule 1: Every value has exactly one owner

```rust
let a = Body::new(...);  // `a` owns this Body
let b = a;               // ownership moves to `b`
                         // `a` no longer exists
```

After `let b = a;`, the variable `a` is gone. The Body now belongs to `b`. Using `a` after this point is a compile error.

This prevents **double-free** — two variables both thinking they own the same memory and both trying to free it.

---

## Rule 2: When the owner goes out of scope, the value is dropped

```rust
{
    let ball = Body::new(...);
    ball.draw();
} // `ball` goes out of scope here — Body is automatically cleaned up
```

Rust inserts the cleanup automatically at the closing brace. You never call `free()` — cleanup happens deterministically, every time, exactly when the owner goes away.

---

## Rule 3: Ownership can be transferred (moved)

```rust
fn process(body: Body) {  // `process` takes ownership
    body.draw();
} // `body` is dropped here

let ball = Body::new(...);
process(ball);             // ownership moved into `process`
// `ball` no longer usable here
```

Passing a value to a function transfers ownership to that function. This is called a **move**.

### The Copy exception

Small, cheap types like `f32`, `i32`, `bool`, and our `Vec2` implement `Copy`. For these, assignment and function calls make a **bitwise copy** — both the original and the copy are valid afterwards.

```rust
let x: f32 = 3.14;
let y = x;   // copied, not moved
println!("{}", x);  // still valid
```

That's why we added `#[derive(Clone, Copy)]` to `Vec2` in project 4: it's just two floats, copying is instant, and we want to use it freely in math expressions without ownership concerns.

`Body` is not `Copy` — it's larger and we want one clear owner (the `World`'s Vec).

---

## Borrowing: using without owning

Most of the time you don't want to transfer ownership — you just want to look at a value, or temporarily change it. That's **borrowing**.

### Shared borrow `&T` — read-only access

```rust
fn draw(body: &Body) {   // borrows Body, does not own it
    // can read body's fields
    // cannot change them
}

let ball = Body::new(...);
draw(&ball);   // lend ball — ball still owned here
draw(&ball);   // can lend again — multiple shared borrows allowed
```

`&Body` is a **shared reference**. Many shared borrows can exist at the same time, as long as nobody is mutating.

### Mutable borrow `&mut T` — read-write access

```rust
fn update(body: &mut Body, dt: f32) {  // borrows mutably
    body.position = body.position + body.velocity * dt;
}

let mut ball = Body::new(...);  // variable must be `mut` too
update(&mut ball, dt);          // lend mutably — ball still owned here
```

`&mut Body` gives exclusive access. While it exists, no other borrows of `ball` are allowed.

---

## The borrow rules

**Either** any number of shared borrows — **or** exactly one mutable borrow. Never both at the same time.

```
──────────────────────────────────────────────
 Many shared (&T) borrows   |  allowed ✓
 One mutable (&mut T) borrow |  allowed ✓
 Shared AND mutable at once  |  not allowed ✗
 Two mutable borrows at once |  not allowed ✗
──────────────────────────────────────────────
```

This rule is what makes Rust's concurrency safe: if mutation requires exclusive access, two threads can never race to write the same data simultaneously.

---

## How it maps to our engine

| Code | What's happening |
|------|-----------------|
| `world.bodies.push(body)` | Body is **moved** into the Vec — Vec owns it |
| `fn draw(&self)` | `&self` is a shared borrow — read-only |
| `fn update(&mut self, dt: f32)` | `&mut self` is a mutable borrow — can change fields |
| `self.bodies.iter_mut()` | Yields one `&mut Body` at a time — enforces the one-writer rule |
| `#[derive(Clone, Copy)]` on Vec2 | Vec2 copies on assignment — safe to use in expressions freely |

---

## When the compiler complains

Ownership errors can be frustrating at first. But the compiler's messages are unusually helpful — they name the conflicting borrows and where they come from.

A useful framing: when the borrow checker rejects your code, it has spotted a potential problem. It's worth pausing to understand the message rather than trying to work around it. Most of the time, the fix also makes the code's intent clearer.

You now have the model. The next lesson puts it into practice with `Vec<Body>`.
