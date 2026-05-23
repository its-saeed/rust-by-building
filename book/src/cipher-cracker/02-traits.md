# Lesson 2 — Traits

> **Goal**: Understand what a trait is — and see that iterators work because they implement the `Iterator` trait.
>
> **Concepts**: traits as contracts, `Display` and `Debug`, defining a custom trait, the `Iterator` trait, implementing `Iterator` on your own type.

---

## The problem traits solve

Imagine you have a `Contact` struct and a `Product` struct. Both have a `name` field. You want to write one function that can print the name of *either* one — without writing two separate functions.

Or imagine you're sorting a list. How does Rust know how to compare two elements? It relies on the elements implementing a specific interface.

Rust's answer to both problems is the **trait**.

---

## What is a trait?

A **trait** is a contract. It says: "any type that wants to be *X* must provide these methods."

```rust
trait Greet {
    fn hello(&self) -> String;
}
```

This defines a trait called `Greet`. Any type that *implements* `Greet` must provide a `hello` method that takes a reference to `self` and returns a `String`.

Now implement it for a couple of types:

```rust
struct English;
struct Spanish;

impl Greet for English {
    fn hello(&self) -> String {
        String::from("Hello!")
    }
}

impl Greet for Spanish {
    fn hello(&self) -> String {
        String::from("¡Hola!")
    }
}
```

Each type provides its own `hello`. Both satisfy the `Greet` contract.

---

## Why this is powerful

Once a type implements a trait, you can write one function that works for *all* types that implement it:

```rust
fn print_greeting(greeter: &impl Greet) {
    println!("{}", greeter.hello());
}

fn main() {
    print_greeting(&English);   // Hello!
    print_greeting(&Spanish);   // ¡Hola!
}
```

`&impl Greet` means "a reference to any type that implements `Greet`". You don't need to know the concrete type — you only need the trait guarantee.

---

## Traits from the standard library: `Display` and `Debug`

You've already been using traits. When you write `println!("{}", value)`, Rust calls the `Display` trait's method on `value`. When you write `println!("{:?}", value)`, it calls `Debug`.

The `Display` and `Debug` traits are defined in `std::fmt`. Rust implements them automatically for built-in types (`i32`, `String`, `bool`, etc.), but for your own types you have to implement them yourself.

**The easy way — `Debug`**: derive it automatically:

```rust
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let p = Point { x: 1.0, y: 2.5 };
    println!("{:?}", p);    // Point { x: 1.0, y: 2.5 }
    println!("{:#?}", p);   // pretty-printed
}
```

`#[derive(Debug)]` tells the compiler to generate a `Debug` implementation automatically. It works as long as all fields also implement `Debug`.

**The manual way — `Display`**: you write the formatting yourself:

```rust
use std::fmt;

struct Point {
    x: f64,
    y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 1.0, y: 2.5 };
    println!("{}", p);   // (1.0, 2.5)
}
```

`impl fmt::Display for Point` says "I'm implementing the `Display` trait for `Point`". The required method is `fmt`, which writes into a formatter `f`. `write!(f, ...)` works just like `println!` but writes to `f` instead of stdout.

---

## Default method implementations

A trait can provide a **default** implementation for a method. Types can use the default or override it:

```rust
trait Describe {
    fn name(&self) -> &str;

    // Default: uses `name()` — no override required
    fn describe(&self) {
        println!("I am a {}", self.name());
    }
}

struct Cat;
struct Dog;

impl Describe for Cat {
    fn name(&self) -> &str { "cat" }
    // `describe` uses the default
}

impl Describe for Dog {
    fn name(&self) -> &str { "dog" }

    fn describe(&self) {
        println!("Woof! I am a dog.");   // overrides default
    }
}
```

A required method (like `name` here) has no body — you *must* implement it. An optional method (like `describe`) has a default body — you may override it.

---

## The `Iterator` trait

Here is the key insight for this project: **all of Rust's iterator methods — `.filter()`, `.map()`, `.collect()`, `.take()`, etc. — work because they are methods on the `Iterator` trait.**

The `Iterator` trait is defined in the standard library as:

```rust
trait Iterator {
    type Item;   // the type of values produced

    fn next(&mut self) -> Option<Self::Item>;

    // `.filter()`, `.map()`, `.collect()`, `.take()`, ...
    // are all provided as default methods — hundreds of them
}
```

`type Item` is an **associated type** — the type of value this iterator produces. `next()` is the one required method: return `Some(value)` for the next item, or `None` when finished.

All the other methods (`.filter`, `.map`, `.collect`, etc.) are provided as defaults in terms of `next`. You implement `next` once, and you get them all for free.

---

## Implementing `Iterator` yourself

Here is a `Counter` that counts from 1 up to `max`:

```rust
struct Counter {
    current: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { current: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.current < self.max {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}
```

That's all. Now because `Counter` implements `Iterator`, it works directly in `for` loops, and also gains methods like `.sum()`:

```rust
fn main() {
    // Works in a for loop — just like any built-in iterator
    for n in Counter::new(5) {
        print!("{} ", n);   // 1 2 3 4 5
    }
    println!();

    // .sum() is provided by Iterator — no extra code needed
    let total: u32 = Counter::new(5).sum();
    println!("{}", total);   // 15
}
```

One method — `next` — and the `for` loop, `.sum()`, and many other features work automatically.

---

## Why this matters for the cipher cracker

In lesson 3, when you write:

```rust
for ch in text.chars() {
    // ...
}
```

This works because `.chars()` returns a value that implements `Iterator<Item = char>`. The `for` loop calls `next()` on it repeatedly until `None` is returned.

Likewise, when you loop over a `HashMap`:

```rust
for (&ch, &count) in &freq {
    // ...
}
```

This works because `HashMap` also implements a form of iteration.

Understanding traits means understanding *why* `for` loops work on so many different types — they all implement the same `Iterator` contract. And it means if you ever want to make your own iterable type, you now know exactly what to implement.

---

## Summary

| Concept | What it means |
|---------|--------------|
| `trait Foo { fn bar(&self); }` | Define a contract |
| `impl Foo for MyType { fn bar(&self) { ... } }` | Fulfill the contract |
| `fn f(x: &impl Foo)` | Accept any type that fulfills the contract |
| `#[derive(Debug)]` | Auto-implement the `Debug` trait |
| `impl Iterator for T` | Make `T` iterable — all iterator methods come free |

---

## Exercises

Run:

```sh
rbb watch cipher-02
```

Four exercises. Then add a `Summary` struct to the project that implements `Display`, and a `LetterIter` that implements `Iterator` over the letters of a string.
