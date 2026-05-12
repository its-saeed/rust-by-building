# Lesson 3 — Functions

> **Goal**: Extract the shift logic into a reusable function. Call it from `main`.
>
> **Concepts**: `fn`, parameters, return types, expressions vs statements, early `return`, unit type `()`.

---

## Why functions?

Right now the shift logic lives directly in `main`:

```rust
let shifted = (first_char as u8 + key) as char;
```

This works for one character. But we want to encrypt a whole message. We'll need to shift every character — and we don't want to repeat that line ten times.

A **function** is a named, reusable piece of logic. We write it once, call it many times.

---

## Anatomy of a function

```rust
fn shift_char(c: char, key: u8) -> char {
    (c as u8 + key) as char
}
```

Read left to right:

- `fn` — keyword that declares a function
- `shift_char` — the name. Rust convention: `snake_case` (lowercase words joined by `_`)
- `(c: char, key: u8)` — two parameters. Each has a name and a type, separated by `:`. Multiple parameters are separated by `,`
- `-> char` — the return type. This function gives back a `char`
- `{ ... }` — the body

Call it like this:

```rust
let result = shift_char('H', 3);
println!("{}", result);   // K
```

---

## The last expression is the return value

Rust has an unusual rule: **the last expression in a function body is automatically returned**. No `return` keyword needed.

```rust
fn shift_char(c: char, key: u8) -> char {
    (c as u8 + key) as char     // no semicolon — this is the return value
}
```

The key detail: **no semicolon** at the end. A line without a semicolon is an **expression** — it evaluates to a value and that value is returned.

Add a semicolon and it becomes a **statement** — it does something but gives back nothing:

```rust
fn shift_char(c: char, key: u8) -> char {
    (c as u8 + key) as char;    // semicolon! now this is a statement
                                // the function returns () — compile error
}
```

The compiler will tell you: _expected `char`, found `()`_. The fix is to remove the semicolon.

This trips up everyone coming from C or Python, where every line ends with a semicolon or nothing. In Rust, the final line of a function body is special: leave off the semicolon to return its value.

### Expressions vs statements

The rule in full:

- **Expression**: evaluates to a value. No semicolon. Examples: `2 + 2`, `'A' as u8`, a function call.
- **Statement**: does something, produces no value. Has a semicolon (or is a `let` binding).

A function body is a list of statements followed by one optional trailing expression. That trailing expression is what gets returned.

```rust
fn describe(n: i32) -> &'static str {
    let doubled = n * 2;              // statement — let bindings always are
    if doubled > 10 { "big" } else { "small" }   // expression — returned
}
```

---

## Early return with `return`

Sometimes you want to exit a function before reaching the end. Use `return` explicitly:

```rust
fn safe_shift(c: char, key: u8) -> char {
    if !c.is_alphabetic() {
        return c;    // leave non-letters unchanged, exit immediately
    }
    (c as u8 + key) as char
}
```

`return` with a semicolon is a statement that exits the function with the given value. Use it for early exits. The trailing-expression style is preferred for the normal return path.

---

## Functions with no return value

Not every function returns something. If there's no `->`, the function returns `()` — the **unit type**, Rust's version of "void":

```rust
fn print_banner() {
    println!("=== Caesar Cipher ===");
}
```

`()` is pronounced "unit". It means "nothing meaningful". You can call these functions, but you can't do anything with their return value.

---

## Parameters are immutable by default

Just like `let`, function parameters are immutable. If you try to change one:

```rust
fn bump(n: i32) -> i32 {
    n += 1;   // compile error: cannot assign to immutable parameter `n`
    n
}
```

Add `mut` to opt in:

```rust
fn bump(mut n: i32) -> i32 {
    n += 1;
    n
}
```

Note: this only affects the local copy of `n`. Calling `bump(5)` does not change the `5` at the call site — Rust copied the value in.

---

## Putting it together

After lesson 3, the project has a `shift_char` function:

```rust
fn shift_char(c: char, key: u8) -> char {
    (c as u8 + key) as char
}

fn main() {
    let message = "Hello, World!";
    let key: u8 = 3;
    let first_char = 'H';

    let shifted = shift_char(first_char, key);

    println!("=== Caesar Cipher ===");
    println!("Message : {}", message);
    println!("Key     : {}", key);
    println!("'{}' + {} = '{}'", first_char, key, shifted);
}
```

Same output as lesson 2 — but the shift logic is now in a named, reusable function. Lesson 4 will call it on every character in the message.

---

## Exercises

Run:

```sh
rbb watch caesar-03
```

Four exercises. Fix each one, then complete the project step: move the shift logic from `main` into a `shift_char` function and call it.
