# Functions

> **Prerequisites**: Lessons 01 (Hello, Rust) and 02 (Values and types).
> **You'll write**: ~4 exercises and a small calculator project.

A function in Rust is a named chunk of code that takes some inputs, maybe returns a value, and can be called from elsewhere. You've already seen one on every exercise — `fn main()`. Now let's write our own.

## The shape of a function

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Read that left to right:

- `fn` — this is a function.
- `add` — its name. Use `snake_case` for function names in Rust.
- `(a: i32, b: i32)` — two parameters, both 32-bit signed integers.
- `-> i32` — returns a 32-bit signed integer.
- `{ ... }` — the body.

You call it the same way you'd expect:

```rust
let sum = add(2, 3);
println!("{sum}");     // 5
```

## Functions return the last expression

Rust does not require `return`. The final expression in the body — the one with no semicolon — is the return value.

```rust
fn double(x: i32) -> i32 {
    x * 2       // no semicolon: this is returned
}
```

```rust
fn double(x: i32) -> i32 {
    x * 2;      // semicolon: this is a statement, and nothing is returned.
                // Compile error: expected `i32`, found `()`.
}
```

The semicolon turns an expression into a statement. A statement has no value. A function declared to return `i32` that ends in a statement won't compile.

`return` exists for early exit:

```rust
fn clamp(x: i32) -> i32 {
    if x < 0 { return 0; }
    if x > 100 { return 100; }
    x
}
```

## No return type means no return value

```rust
fn greet(name: &str) {
    println!("Hello, {name}!");
}
```

No `->` means the function returns `()` (the "unit" type, written `()` — Rust's version of "void"). You can still call it, you just can't assign its result to something meaningful:

```rust
let nothing = greet("world");   // nothing is of type ()
```

## Expressions vs statements, one more time

This is where most people trip. Here's the rule:

- **Expression**: evaluates to a value (`2 + 2`, `if x > 0 { "pos" } else { "neg" }`, a function call).
- **Statement**: does something, evaluates to nothing (a `let` binding, an expression with `;` on the end).

A function body is a list of statements followed by one optional trailing expression. That trailing expression is what's returned.

```rust
fn sign(x: i32) -> &'static str {
    let label = if x > 0 {
        "positive"
    } else if x < 0 {
        "negative"
    } else {
        "zero"
    };                                  // statement: semicolon ends the `let`
    label                               // expression: returned
}
```

The `if` itself is an expression. Remember that — it'll come back when we cover control flow.

## Parameters are immutable by default

```rust
fn bump(n: i32) -> i32 {
    n += 1;         // compile error: cannot assign to immutable parameter
    n
}
```

To mutate, mark the parameter `mut`:

```rust
fn bump(mut n: i32) -> i32 {
    n += 1;
    n
}
```

This is only about the local name — `bump(5)` still leaves the `5` at the caller untouched, because `n` is a copy of it.

## Exercises

Find them under `lessons/03-functions/exercises/`. Open `rbb watch 03` and work through them in order.

## Project

When the exercises pass, build the project at `lessons/03-functions/project/`: a tiny calculator module that you export from a library, with tests that verify each function works. The project brief is in `lessons/03-functions/project/README.md`.
