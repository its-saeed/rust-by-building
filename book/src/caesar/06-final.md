# Final Exercise — Caesar Cipher End-to-End

> **No new concepts.** This is a test of everything from lessons 1–5.
>
> Three exercises. No scaffolding — you write or fix the whole thing.

---

## What you've built

Over the last five lessons you've learned:

| Lesson | Concept |
|--------|---------|
| 1 | Variables, `let`, `println!` |
| 2 | `char`, ASCII, type casting with `as` |
| 3 | Functions, return types, expressions vs statements |
| 4 | `for` loops, `.chars()`, numeric ranges, `String`, `let mut` |
| 5 | `if`/`else`, `match`, modulo, wrap-around |

The result is a working Caesar cipher:

```rust
fn shift_char(c: char, key: u8) -> char {
    match c {
        'A'..='Z' => (b'A' + (c as u8 - b'A' + key) % 26) as char,
        'a'..='z' => (b'a' + (c as u8 - b'a' + key) % 26) as char,
        _ => c,
    }
}

fn encrypt(text: &str, key: u8) -> String {
    let mut result = String::new();
    for c in text.chars() {
        result.push(shift_char(c, key));
    }
    result
}

fn decrypt(text: &str, key: u8) -> String {
    encrypt(text, 26 - key % 26)
}
```

Now use it.

---

## Exercise 1 — Implement from scratch

Open `exercises/ex01_implement.rs`. The three function signatures are there with `todo!()` bodies. Implement all three. All tests must pass.

This is closed-book. No looking at previous steps. Use the compiler errors as your guide.

---

## Exercise 2 — Find the bug

Open `exercises/ex02_find_the_bug.rs`. There is a complete cipher implementation — but something is wrong. The tests fail. Find the bug and fix it.

There is exactly one bug. Read the failing test output carefully: it tells you which inputs produce wrong results.

---

## Exercise 3 — Crack it

Open `exercises/ex03_crack.rs`. You are given a ciphertext that was encrypted with an unknown key. You don't know the key.

Your job: try all 26 possible keys and print the result of each. One of them will be readable English — that's the original message.

This exercise has no tests. Run the program, read the output, and write down the key.

---

## Run the exercises

```sh
rbb watch caesar-06
```

Exercises 1 and 2 are verified by tests. Exercise 3 is verified by your own eyes.
