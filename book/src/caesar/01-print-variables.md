# Lesson 1 — Print & Variables

> **Goal**: Get your first Rust program running. Print the message and key we'll encrypt in later lessons.
>
> **Concepts**: `fn main`, `let`, `println!`, basic types (`&str`, `i32`), comments.

---

## Your first Rust program

Open `lessons/1-caesar-cipher/lesson-01/project/src/main.rs`. You'll see a skeleton. Let's understand every part of it before you touch anything.

Here is the simplest possible Rust program:

```rust
fn main() {
    println!("=== Caesar Cipher ===");
}
```

Run it with:

```sh
cargo run
```

Output:

```
=== Caesar Cipher ===
```

That's a complete, working Rust program. Three tokens, one line of real work. Let's break it down.

---

## `fn main()`

`fn main()` is the **entry point** — the first thing that runs when you execute your program.

```rust
fn main() {
    // everything here runs when the program starts
}
```

- `fn` — the keyword that declares a function. Think of it like `def` in Python or the bare `int main()` in C, except in Rust `main` returns nothing (no `return 0` needed).
- `main` — this exact name is required. Rust looks for `main` when starting your program.
- `()` — no parameters. We'll add parameters in lesson 3.
- `{ }` — the **body** of the function. All the code inside runs top to bottom.

Every Rust program you write will start here.

---

## `println!` — printing to the terminal

`println!` prints a line of text and then moves to the next line.

```rust
println!("Hello, World!");
```

The `!` after `println` is not a typo — it marks this as a **macro**, not a regular function. For now, treat it as a special function call. The rule is simple: always write the `!`. If you forget it:

```rust
println("Hello");   // compile error
```

Rust won't compile this. The compiler will point exactly at the problem and tell you what's wrong. That's a pattern you'll see throughout the course — Rust errors are informative.

### Format strings

You can embed values inside the printed text using `{}` as a placeholder:

```rust
let name = "Alice";
println!("Hello, {}!", name);
```

Output: `Hello, Alice!`

Multiple values — multiple `{}`:

```rust
let message = "Hello";
let key = 3;
println!("Message: {}, Key: {}", message, key);
```

Output: `Message: Hello, Key: 3`

If you've used C before, you might reach for `%s` or `%d`. Don't. Rust uses `{}` for everything, and the compiler enforces it — a mismatched placeholder is a **compile error**, not a runtime surprise.

---

## `let` — binding names to values

```rust
let message = "Hello, World!";
let key = 3;
```

`let` binds a name to a value. A few important things:

### Types are inferred

You don't need to write the type. Rust figures it out from what's on the right side. `"Hello, World!"` is clearly text. `3` is clearly a number. The compiler knows.

If you want to be explicit, you can write the type after a colon:

```rust
let message: &str = "Hello, World!";
let key: i32 = 3;
```

Both forms do the same thing. As a beginner, let Rust infer — only add type annotations when the compiler asks for them or when it makes the code clearer.

### Bindings are immutable by default

Once you write `let x = 5`, you cannot change `x`.

```rust
let key = 3;
key = 7;    // compile error: cannot assign twice to immutable variable
```

This is different from Python (where you can reassign freely) and C (where variables are mutable by default). Rust makes you **opt into** mutability by writing `let mut`:

```rust
let mut key = 3;
key = 7;    // fine
```

We won't need `mut` in this lesson. It shows up in lesson 4.

### The name lives until the end of the block

A `let` binding is visible from the line it's declared until the closing `}` of the block it's in. Trying to use a name before its `let` line is a compile error:

```rust
println!("{}", message);    // compile error: message not yet declared
let message = "Hello";
```

Rust is strict: declare first, use after.

---

## Types — just the two you need today

Rust has a rich type system. For now, two types are enough:

| Example value | Type | What it is |
|---|---|---|
| `"Hello, World!"` | `&str` | A string literal — text baked into the program, read-only |
| `3` | `i32` | A 32-bit signed integer — whole numbers, positive or negative |

You'll learn much more about types in lesson 2. For now: quoted text is `&str`, whole numbers are `i32`.

---

## Comments

```rust
// This is a single-line comment. Rust ignores everything after //.
let key = 3;  // you can put a comment at the end of a line too
```

Same style as C. Comments are for the human reading the code — the compiler skips them entirely.

---

## Putting it together

Here's what the Caesar cipher program looks like after lesson 1:

```rust
fn main() {
    let message = "Hello, World!";
    let key = 3;

    println!("=== Caesar Cipher ===");
    println!("Message : {}", message);
    println!("Key     : {}", key);
}
```

Expected output:

```
=== Caesar Cipher ===
Message : Hello, World!
Key     : 3
```

Nothing is encrypted yet — that starts in lesson 2. But this is the foundation: a program that compiles, runs, and announces what it's going to do.

---

## Exercises

Open `lessons/1-caesar-cipher/lesson-01/` and run:

```sh
rbb watch caesar-01
```

There are four exercises. Each is a small broken program. Read the compiler error, figure out what's wrong, fix it. Move to the next one when it passes.

When all four pass, open `project/src/main.rs` and complete the project step.

Run your finished project with:

```sh
cargo run
```

Verify the output matches what's shown above, then move on to lesson 2.
