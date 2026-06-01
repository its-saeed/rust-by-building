# Lifetimes

> **Goal**: Understand what lifetimes are and why Rust requires you to name them when a struct holds a reference.
>
> **Concepts**: borrow validity, lifetime annotations on structs and impls, the `'a` syntax, anonymous lifetime `'_`.

---

## The problem lifetimes solve

Rust guarantees that a reference never outlives the data it points to. A dangling pointer — a reference to memory that has already been freed — is one of the most common sources of crashes and security bugs in other languages. Rust makes it impossible at compile time.

You have already used references everywhere — `&Ball`, `&mut Paddle`, `&str` — and Rust has quietly verified their validity without requiring you to write anything extra. In most cases the compiler can figure out how long a borrow is valid on its own. Lifetimes only become visible when the compiler cannot infer the relationship — most commonly when you store a reference inside a struct.

---

## Three analogies

### House key

A friend gives you a key to their house. That key is only useful while they still live there. If they move out — if they *drop* the house — arriving with the key leads nowhere.

Rust enforces the same rule: a reference (the key) is only valid while the owner (your friend, living in the house) is still alive. The lifetime `'a` is the duration of their tenancy. If the owner is gone and you still hold a reference, the compiler rejects your program.

### Library book

You borrow a book. The library still owns it. You can read it (`&`), and so can others at the same time — multiple shared references are fine. But if you want to annotate it, you need exclusive access (`&mut`) and nobody else can borrow it while you do. When the library closes (the scope ends), the book is returned and your borrow expires.

This is exactly Rust's borrow rules: any number of shared references *or* one exclusive reference, never both, and never past the owner's lifetime.

### Name badge

A name badge grants access to a building. The badge points to a person's identity, which is stored in the HR system (the owned data). If that person leaves the company and their record is deleted, the badge points to nothing. Lifetimes are the compiler's way of guaranteeing: *the record exists at least as long as any badge pointing to it*.

---

## When Rust makes you write lifetimes

In functions, the compiler applies **lifetime elision** — a set of rules that infer lifetimes automatically. You have been benefiting from this all along:

```rust
fn first_word(s: &str) -> &str { ... }
```

The compiler reads this as: *the returned reference lives at most as long as `s`*. No annotation needed.

Elision cannot work when you store a reference in a struct. The compiler has no way to know how long the struct will live relative to the data it borrows, so you must say it explicitly:

```rust
struct Highlighted<'a> {
    text: &'a str,
}
```

The `'a` is a **lifetime parameter** — a name for the relationship. It reads: "a `Highlighted` cannot outlive the `str` it holds a reference to."

---

## The syntax

A lifetime parameter is introduced with `<'a>` after the type name, then used wherever a reference is stored:

```rust
struct Foo<'a> {
    value: &'a SomeType,
}
```

`impl` blocks need the same parameter in scope:

```rust
impl<'a> Foo<'a> {
    fn new(value: &'a SomeType) -> Self {
        Foo { value }
    }
}
```

When you pass a reference to a function and the lifetime does not need a name — it just needs to exist — you can use the **anonymous lifetime** `'_`:

```rust
fn print(foo: &Foo<'_>) { ... }
```

`'_` tells the compiler: "there is a lifetime here, please infer it."

---

## What the annotation means

A lifetime annotation is a **constraint**, not a timer. Writing `'a` does not extend or shorten how long data lives — it only tells the compiler about a relationship that must hold. The data lives as long as it would have anyway; the annotation just makes the dependency explicit so the compiler can verify it.

In practice, the data you reference will usually outlive the struct with no trouble at all — the annotation is just the price of admission for storing a reference.

---

## What's next

In the next lesson you will add sprite textures to `Ball` and `Paddle`. Each struct will hold a `&'a Texture2D` — a reference to a texture loaded before the game loop. You will write the lifetime annotations yourself and see the compiler guide you to the right form.
