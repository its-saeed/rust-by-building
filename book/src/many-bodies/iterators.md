# Iterators

An iterator is one of Rust's most important abstractions. You've already used one — the `for` loop — without knowing the machinery underneath. This page explains what iterators actually are, how they work, and how to build your own.

---

## What is an iterator?

An iterator is a value that **produces a sequence of items, one at a time, on demand**.

Think of a ticket dispenser. Each time you press the button, you get the next ticket. When it runs out, it tells you it's empty. You don't need to know in advance how many tickets are inside — you just keep asking until it says "done."

In Rust, "done" is represented by `None`, and "here's the next item" is `Some(item)` — the `Option` type you've already seen.

---

## The `Iterator` trait

Every iterator in Rust implements the `Iterator` trait:

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

- **`type Item`** — what kind of value this iterator produces. For a body iterator, it might be `&Body` or `&mut Body`.
- **`fn next(&mut self)`** — call this to get the next item. Returns `Some(item)` if there is one, `None` when the sequence is exhausted.

That's the entire contract. Anything implementing these two things is an iterator, and anything that works with iterators will accept it.

---

## How `for` loops really work

When you write:

```rust
for body in &self.bodies {
    body.draw();
}
```

Rust desugars this to:

```rust
let mut iter = self.bodies.iter();
loop {
    match iter.next() {
        Some(body) => body.draw(),
        None => break,
    }
}
```

A `for` loop is just syntax sugar over repeatedly calling `next()` until it returns `None`. This means **any type that produces an iterator can be used in a `for` loop** — not just Vec, but ranges, files, custom types, even infinite sequences.

---

## Three ways to iterate a Vec

Given `let bodies: Vec<Body> = ...`, there are three iterator variants:

### `.iter()` → shared references `&Body`

```rust
for body in bodies.iter() {
    body.draw();  // body: &Body — read only
}
```

Use when you only need to read. The Vec is borrowed; multiple shared borrows are fine.

### `.iter_mut()` → mutable references `&mut Body`

```rust
for body in bodies.iter_mut() {
    body.update(dt);  // body: &mut Body — can mutate
}
```

Use when you need to change each element. One `&mut Body` exists at a time — the borrow checker enforces this.

### `.into_iter()` → owned values `Body`

```rust
for body in bodies.into_iter() {
    process(body);  // body: Body — owned, moved out
}
// bodies is gone — the Vec was consumed
```

Use when you want to move elements out. The Vec is consumed and can't be used afterwards.

**Shorthand forms:** `for x in &vec` calls `.iter()`, `for x in &mut vec` calls `.iter_mut()`, `for x in vec` calls `.into_iter()`.

---

## Iterator adapters

The real power of iterators comes from **adapters** — methods on the `Iterator` trait that transform one iterator into another. They're **lazy**: nothing runs until the final step that consumes the iterator.

```rust
// Find the total area of all bodies with radius > 15
let total_area: f32 = bodies
    .iter()
    .filter(|body| body.radius > 15.0)    // keep large bodies
    .map(|body| body.radius * body.radius) // compute radius²
    .sum();                                 // add them up
```

No intermediate Vec is created — Rust compiles the whole chain into a single loop.

Common adapters:

| Adapter | What it does |
|---------|-------------|
| `.map(|x| ...)` | Transform each item into something else |
| `.filter(|x| ...)` | Keep only items where the closure returns `true` |
| `.take(n)` | Stop after `n` items |
| `.skip(n)` | Skip the first `n` items |
| `.enumerate()` | Pair each item with its index: `(0, item), (1, item), ...` |
| `.zip(other)` | Pair items from two iterators side by side |
| `.count()` | Count how many items (consumes the iterator) |
| `.sum()` | Sum all items (consumes the iterator) |
| `.collect::<Vec<_>>()` | Gather items into a collection (consumes the iterator) |

---

## Implementing your own iterator

To make a type iterable, implement `Iterator` — provide `type Item` and `fn next`. Here's a simple countdown:

```rust
struct Countdown {
    remaining: u32,
}

impl Countdown {
    fn new(from: u32) -> Self {
        Countdown { remaining: from }
    }
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.remaining == 0 {
            None                           // sequence exhausted
        } else {
            self.remaining -= 1;
            Some(self.remaining + 1)       // yield the current value
        }
    }
}
```

Using it:

```rust
for n in Countdown::new(3) {
    println!("{}", n);  // prints: 3, then 2, then 1
}

// adapters work too, because Countdown implements Iterator:
let doubled: Vec<u32> = Countdown::new(3).map(|n| n * 2).collect();
// [6, 2, 4] — wait, that's wrong. Actually: [6, 4, 2]
```

Once you implement `next`, all the adapters — `.map`, `.filter`, `.collect`, etc. — are available for free because they're provided by the `Iterator` trait itself.

---

## Connection to our engine

In `World::step`, `iter_mut()` does the heavy lifting:

```rust
for body in self.bodies.iter_mut() {  // yields &mut Body
    body.update(dt);                  // allowed: &mut Body has exclusive access
    body.keep_in_bounds();
}
```

The iterator hands you `&mut Body` values one at a time. Between calls to `next()`, no other code can access those bodies — the borrow checker enforces this, which is why two simultaneous mutable borrows are impossible. Safe concurrent mutation is guaranteed at compile time.

You're ready for lesson 2.
