# Lifetimes

> **Goal**: Understand what lifetimes are, why Rust requires you to name them when a struct holds a reference, and how to write the annotations confidently.
>
> **Concepts**: borrow validity, the dangling-pointer problem, lifetime elision, lifetime annotations on structs and impls, the `'a` syntax, anonymous lifetime `'_`.

---

## The problem: dangling pointers

In many languages, you can hold a reference to something after it has been destroyed. The reference still exists — it just points to garbage, or to memory that has been repurposed for something else entirely. Using it causes crashes, corrupted state, or security vulnerabilities. This class of bug is called a **dangling pointer** or **use-after-free**.

Rust eliminates this at compile time. Every reference has a **lifetime** — the span of time during which the value it points to is guaranteed to exist. Rust checks, at compile time, that every reference is used only within that span. If the check fails, you get a compiler error rather than a crash at runtime.

---

## Lifetimes you have already used

You have been working with lifetimes throughout this book. You just have not had to write them yet, because in most situations the compiler can figure them out on its own.

Consider `Score::update`:

```rust
fn update(&mut self, ball: &Ball) -> bool {
    let left_exit  = ball.rect.x + ball.rect.w < 0.0;
    let right_exit = ball.rect.x > WINDOW_W;
    ...
}
```

There is a reference here: `ball: &Ball`. It has a lifetime — the duration of this function call. The compiler knows that the borrow is valid because `ball` is alive before the call starts and the call returns before anything can drop it. You did not have to say any of this. The compiler inferred it.

This inference is called **lifetime elision** — the compiler applies a set of built-in rules to fill in lifetimes you did not write. Elision works in the vast majority of functions.

The one place elision cannot help is when you store a reference inside a struct. That is where you have to step in.

---

## A concrete example of the problem

Suppose you wanted a struct that holds a reference to a `String` owned somewhere else:

```rust
struct Greeting {
    text: &str,   // reference to someone else's String
}
```

The compiler rejects this immediately:

```
error[E0106]: missing lifetime specifier
 --> src/main.rs:2:11
  |
2 |     text: &str,
  |           ^ expected named lifetime parameter
```

Why? Because the compiler has no idea how long `Greeting` will live relative to the `String` it borrows. What if you created the `String`, put a reference to it in `Greeting`, then dropped the `String` — but kept the `Greeting` around? You would have a `Greeting` pointing to freed memory:

```rust
let greeting;
{
    let s = String::from("hello");
    greeting = Greeting { text: &s };
    // s is dropped here
}
// greeting is still alive here — but text points to freed memory
println!("{}", greeting.text); // ← disaster
```

Rust refuses to compile this pattern. The lifetime annotation is how you tell the compiler about the constraint that prevents it.

---

## The house-key analogy

Think of it this way: a friend gives you a key to their house. The key (the reference) is only useful while they still live there (while the `String` is still alive). If they move out — if the `String` is dropped — arriving with the key leads nowhere.

When you add a lifetime parameter to `Greeting`, you are writing down this contract:

```rust
struct Greeting<'a> {
    text: &'a str,
}
```

Read this aloud: *"a `Greeting<'a>` holds a reference to a `str` that lives at least as long as `'a`, and a `Greeting` cannot outlive `'a`."*

The compiler now understands the relationship. It can check, everywhere `Greeting` is used, that the referenced data will outlive the struct. The house-key analogy becomes code: the key (`Greeting`) cannot exist after the house (`str`) is gone.

---

## Lifetime elision: when you do not write anything

Before diving further into annotations, it is worth understanding why you rarely need them in functions. The compiler's elision rules handle the most common cases automatically:

**One input reference → output borrows from it:**
```rust
fn first_word(s: &str) -> &str {
    &s[..s.find(' ').unwrap_or(s.len())]
}
```
The compiler reads this as: the returned `&str` lives at most as long as `s`. No annotation needed.

**Method with `&self` → output borrows from `self`:**
```rust
impl Score {
    fn label(&self) -> &str {
        "Score"
    }
}
```
The rule: if there is a `&self` or `&mut self`, the return borrows from it. No annotation needed.

**Multiple input references, no output reference:**
```rust
fn print_both(a: &str, b: &str) {
    println!("{} {}", a, b);
}
```
Nothing is returned, so there is nothing to track. No annotation needed.

Elision fails when the compiler would have to guess which input a returned reference comes from, or — as in our case — when a reference is stored inside a struct where the relationship cannot be derived from function signatures alone.

---

## Adding the annotation step by step

Let's walk through fixing `Greeting`:

**Step 1 — add the lifetime parameter to the struct:**

```rust
struct Greeting<'a> {
    text: &'a str,
}
```

The `<'a>` after `Greeting` introduces the name `'a` (pronounced "tick a" or just "a"). The `'a` in `&'a str` says: "this reference must be valid for at least the lifetime named `'a`."

**Step 2 — add the same parameter to the `impl` block:**

```rust
impl<'a> Greeting<'a> {
    fn new(text: &'a str) -> Self {
        Greeting { text }
    }
}
```

The `<'a>` after `impl` brings the name into scope so `Greeting<'a>` can use it. The `text: &'a str` parameter carries the same constraint: the caller must pass a reference that lives at least as long as `'a`.

**Step 3 — create a `Greeting` with data that lives long enough:**

```rust
fn main() {
    let s = String::from("hello");
    let g = Greeting::new(&s);
    println!("{}", g.text);
    // s is dropped here, after g — that's fine
}
```

The compiler checks: `g` borrows from `s`. `s` is dropped after `g`. The constraint holds. Program compiles.

If you tried to drop `s` first:

```rust
fn main() {
    let g;
    {
        let s = String::from("hello");
        g = Greeting::new(&s);
    } // s dropped here
    println!("{}", g.text); // g is still alive — ERROR
}
```

```
error[E0597]: `s` does not live long enough
  --> src/main.rs:5:26
   |
5  |         g = Greeting::new(&s);
   |                           ^^ borrowed value does not live long enough
6  |     } // s dropped here
   |     - `s` dropped here while still borrowed
7  |     println!("{}", g.text);
   |                    ------ borrow later used here
```

This is exactly the protection lifetimes provide. The error points directly at the problem — no runtime crash, no undefined behavior.

---

## Naming does not matter; the constraint does

`'a` is just a placeholder name. You could write `'texture`, `'game`, or any identifier preceded by a tick:

```rust
struct Paddle<'texture> {
    rect:    Rect,
    texture: &'texture Texture2D,
}
```

Convention is to use single lowercase letters (`'a`, `'b`), but named lifetimes can help readability when there are multiple. The compiler only cares that the names are used consistently, not what they are called.

---

## The anonymous lifetime `'_`

When you reference a struct with a lifetime parameter from a function that does *not* store the reference — it just reads from it momentarily — you do not need to name the lifetime:

```rust
fn update(&mut self, ball: &Ball<'_>) -> bool {
    ...
}
```

`'_` means: "there is a lifetime here, and I know it — please infer it for me." It is different from leaving the lifetime out entirely (which would be an error, since `Ball` requires a lifetime parameter). It is a way of saying: *"I acknowledge this reference has a lifetime, but I do not need to track it further."*

You will see `'_` specifically when you pass a reference to a struct into a function that only reads from it within the call. `Score::update` does not store the `Ball` reference anywhere — it reads two fields and returns. The borrow starts and ends in the same call. `'_` is perfect here.

---

## What the annotation actually means

A lifetime annotation is a **constraint**, not a timer. Writing `'a` does not extend or shorten how long data lives. The data lives exactly as long as it would have anyway. The annotation is purely a statement of a relationship:

> "The `Paddle` holding this reference cannot outlive the `Texture2D` it holds a reference to."

The compiler verifies that relationship everywhere the struct is used. In practice, when the owned data is created before the struct and lives until the program ends — as textures loaded at startup do — the constraint is satisfied without any special effort. The annotation is just the price of admission for storing a reference.

Another way to think about it: you are not *managing* memory with lifetime annotations. Rust manages memory automatically via ownership. Lifetime annotations are a *description* of what the ownership structure already is, written explicitly enough that the compiler can verify it.

---

## Common questions

**"Do I need to worry about this for local variables?"**

No. Lifetimes on local variables are inferred by the compiler and are always correct by construction — you cannot move a local variable into a position where a reference to it would dangle. You only write lifetime annotations on structs and (occasionally) on functions with multiple reference parameters where the compiler cannot determine which input the output borrows from.

**"What if I just store an owned value instead?"**

You can, and often should. If you store `Texture2D` instead of `&Texture2D`, you own the texture and there is no lifetime to track. The reason to use a reference here is that `Texture2D` holds GPU resources — cloning or moving it is expensive and semantically wrong. Two paddles should share one texture, not each own a copy. Storing a reference with a lifetime annotation is the right tool for "borrow this once, use it many times."

**"Will this come up often?"**

In application code: occasionally. Lifetime annotations appear most when you build data structures that hold references (parsers, tree nodes, iterators over borrowed data). In game code like ours, they appear exactly here — when a struct holds a reference to a loaded asset.

---

## What's next

In the next lesson you will add sprite textures to `Ball` and `Paddle`. Each struct will hold a `&'a Texture2D` — a reference to a texture loaded once, before the game loop, shared between instances. You will write the lifetime annotations yourself, guided by the compiler, and see how the steps above map directly onto the code.
