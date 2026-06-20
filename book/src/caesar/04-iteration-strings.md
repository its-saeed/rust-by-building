# Lesson 4 — Iteration & Strings

> **Goal**: Encrypt the entire message, not just one character.
>
> **Concepts**: `&str` vs `String`, `String::new()`, `.push()`, `let mut`, `for` loops, `.chars()`, numeric ranges.

---

## Two kinds of strings

Rust has two string types. You've been using one of them since lesson 1.

### `&str` — a string you borrow and read

```rust
let message = "Hello, World!";
```

`"Hello, World!"` is a **string literal** — it's baked into the compiled program, stored in read-only memory. `message` is a `&str`: a reference to that text. You can read it, pass it around, but you cannot grow or modify it.

Think of `&str` as a window into existing text. It's always a reference to something else — hence the `&`.

### `String` — a string you own and can grow

```rust
let mut result = String::new();
```

`String` is a heap-allocated, growable string. You own it. You can add characters to it, clear it, modify it. It lives on the heap (unlike `&str` literals, which are in the binary).

Think of `String` like a dynamic buffer — similar to `std::string` in C++, or a list of characters you control.

### When to use which

| | `&str` | `String` |
|---|---|---|
| Input to a function | ✓ (read-only, efficient) | |
| Building a new string | | ✓ (growable) |
| String literals | ✓ | |
| Return a constructed string | | ✓ |

Our `encrypt` function will **take** a `&str` (the original message) and **return** a `String` (the encrypted result).

---

## Building a string character by character

```rust
let mut result = String::new();   // start empty
result.push('K');                 // append a char
result.push('h');
result.push('o');
result.push('o');
result.push('r');
println!("{}", result);           // Khoor
```

Two things to notice:

**`String::new()`** creates an empty `String`. The `::` means we're calling `new` on the `String` type itself (not on a value). You'll see this pattern often in Rust.

**`.push(c)`** appends one `char` to the end. There's also `.push_str(s)` for appending a whole `&str`:

```rust
result.push_str(" World");   // appends multiple characters at once
```

Don't mix them up — `.push()` takes a `char`, `.push_str()` takes a `&str`.

---

## `let mut` — opting into mutation

In lesson 1 you learned that `let` bindings are immutable by default. To modify a `String` (or any variable), you must declare it with `let mut`:

```rust
let result = String::new();
result.push('K');   // compile error: cannot borrow `result` as mutable
```

```rust
let mut result = String::new();
result.push('K');   // fine
```

`mut` is explicit in Rust. If you see `let mut`, you know that variable will change somewhere. If you see `let`, you know it won't. This makes code easier to reason about.

---

## Iterating over characters with `.chars()`

To loop over every character in a `&str`, call `.chars()` on it. This gives you an iterator — a sequence of `char` values you can loop over.

```rust
let text = "Hello";

for c in text.chars() {
    println!("{}", c);
}
```

Output:
```
H
e
l
l
o
```

The `for c in ...` loop binds each character to `c` in turn and runs the body once per character.

You cannot iterate over a `&str` directly — `for c in text` won't compile. You must call `.chars()` first. The reason: a `&str` is a sequence of bytes, not characters, and Rust makes you be explicit about the distinction.

---

## Iterating over a range of numbers

The same `for ... in ...` syntax works with numbers. A **range** `0..n` produces the integers 0, 1, 2, … n−1:

```rust
for i in 0..5 {
    println!("{}", i);
}
```

Output:
```
0
1
2
3
4
```

`0..5` means "from 0 up to but not including 5". To include the endpoint, write `0..=5` (0 through 5 inclusive).

This comes up whenever you need to repeat something a fixed number of times or try every value in a known set:

```rust
for key in 0u8..26 {
    println!("key {}: {}", key, encrypt("Khoor", key));
}
```

Notice the `0u8` — this tells Rust the range produces `u8` values, matching the type our functions expect. Without it, Rust infers `i32` by default.

---

## The `encrypt` function

Now we have everything we need to encrypt a full message:

```rust
fn encrypt(text: &str, key: u8) -> String {
    let mut result = String::new();
    for c in text.chars() {
        result.push(shift_char(c, key));
    }
    result
}
```

Read it top to bottom:

1. `let mut result = String::new()` — start with an empty output string
2. `for c in text.chars()` — visit every character in the input
3. `result.push(shift_char(c, key))` — shift the character and append it to the result
4. `result` — return the finished string (trailing expression, no semicolon)

This calls `shift_char` from lesson 3, which already handles non-alphabetic characters correctly (leaves them unchanged). So spaces, punctuation and digits pass through untouched.

---

## A glimpse of what's coming

The `encrypt` function above can actually be written in one line:

```rust
fn encrypt(text: &str, key: u8) -> String {
    text.chars().map(|c| shift_char(c, key)).collect()
}
```

`.map()` transforms each character. `.collect()` gathers the results back into a `String`. `|c| shift_char(c, key)` is a **closure** — an inline function. These are powerful tools we'll cover in a later project. For now, the explicit `for` loop is clearer and equally correct.

---

## Putting it together

After lesson 4, the program encrypts the full message:

```rust
fn shift_char(c: char, key: u8) -> char {
    if !c.is_alphabetic() {
        return c;
    }
    (c as u8 + key) as char
}

fn encrypt(text: &str, key: u8) -> String {
    let mut result = String::new();
    for c in text.chars() {
        result.push(shift_char(c, key));
    }
    result
}

fn main() {
    let message = "Hello, World!";
    let key: u8 = 3;

    let encrypted = encrypt(message, key);

    println!("=== Caesar Cipher ===");
    println!("Message   : {}", message);
    println!("Key       : {}", key);
    println!("Encrypted : {}", encrypted);
}
```

Output:

```
=== Caesar Cipher ===
Message   : Hello, World!
Key       : 3
Encrypted : Khoor, Zruog!
```

Spaces and punctuation pass through unchanged. Letters are shifted. The cipher works — on letters that don't wrap around the alphabet. Lesson 5 fixes that.

---

## Exercises

Run:

```sh
rbb watch caesar-04
```

Four exercises, then add `encrypt` to the project.
