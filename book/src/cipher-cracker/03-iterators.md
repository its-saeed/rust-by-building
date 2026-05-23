# Lesson 3 — Iterators

> **Goal**: Use iterator methods to work with sequences of characters and numbers — without closures.
>
> **Concepts**: ranges, `.chars()`, `.count()`, `.sum()`, `.min()`, `.max()`, `.enumerate()`, `.take()`, `.skip()`, `.rev()`.

---

## What is an iterator?

In lesson 2 you learned that iterators work because types implement the `Iterator` trait. The `for` loop is the most common way to drive one:

```rust
for ch in "hello".chars() {
    println!("{}", ch);   // h, e, l, l, o
}
```

`.chars()` returns a value implementing `Iterator<Item = char>`. The `for` loop calls `.next()` on it until `None` is returned.

But iterators can do more than drive `for` loops. They come with methods — and many of those methods don't need closures at all.

---

## Ranges

A range is the simplest iterator. `0..5` produces `0, 1, 2, 3, 4`. `0..=5` includes `5`.

```rust
for n in 0..5 {
    print!("{} ", n);   // 0 1 2 3 4
}

for n in 1..=3 {
    print!("{} ", n);   // 1 2 3
}
```

---

## `.count()` — how many items

```rust
let total = "hello world".chars().count();
println!("{}", total);   // 11
```

`.count()` drives the iterator to completion and returns the number of items produced.

---

## `.sum()` — add them all up

Works on iterators of numbers:

```rust
let total: u32 = (1..=10).sum();
println!("{}", total);   // 55
```

You must annotate the type (`: u32`) so Rust knows what to sum into.

---

## `.min()` and `.max()`

Find the smallest or largest item. Both return `Option` — `None` if the iterator is empty:

```rust
let smallest = (3u32..=7).min();   // Some(3)
let largest  = (3u32..=7).max();   // Some(7)

if let Some(n) = largest {
    println!("largest: {}", n);
}
```

---

## `.enumerate()` — items with their index

`.enumerate()` wraps each item with a counter, giving `(index, item)` pairs:

```rust
for (i, ch) in "abc".chars().enumerate() {
    println!("{}: {}", i, ch);
}
// 0: a
// 1: b
// 2: c
```

This is the Rust way to loop when you need the position as well as the value — no manual counter variable needed.

---

## `.skip(n)` and `.take(n)`

`.skip(n)` discards the first `n` items; `.take(n)` stops after `n` items:

```rust
// skip the first 2, take the next 3
for ch in "abcdefg".chars().skip(2).take(3) {
    print!("{}", ch);   // cde
}
```

Chain them to extract a "window" from any sequence.

---

## `.rev()` — iterate in reverse

Works on ranges and slices:

```rust
for n in (1..=5).rev() {
    print!("{} ", n);   // 5 4 3 2 1
}
```

---

## Counting specific characters

You can't use `.filter()` without a closure — but you can count manually with a `for` loop:

```rust
fn count_vowels(text: &str) -> u32 {
    let mut count = 0;
    for ch in text.chars() {
        match ch {
            'a' | 'e' | 'i' | 'o' | 'u' => count += 1,
            _ => {}
        }
    }
    count
}
```

This pattern — iterate, inspect each item, accumulate — is the foundation of everything you build in this project.

---

## Putting it together

After lesson 3, the project reports basic stats about the ciphertext:

```rust
fn main() {
    let ciphertext = "dolfh zdv ehjlqqlqj wr jhw yhub wluhg ri vlwwlqj eb khu vlvwhu rq wkh edqn";

    let total_chars = ciphertext.chars().count();

    let mut letter_count = 0u32;
    for ch in ciphertext.chars() {
        if ch.is_alphabetic() {
            letter_count += 1;
        }
    }

    println!("Total characters : {}", total_chars);
    println!("Alphabetic       : {}", letter_count);
    println!("Other            : {}", total_chars as u32 - letter_count);

    print!("First 10 chars   : ");
    for ch in ciphertext.chars().take(10) {
        print!("{}", ch);
    }
    println!();
}
```

Expected output:

```
Total characters : 72
Alphabetic       : 61
Other            : 11
First 10 chars   : dolfh zdv e
```

---

## Exercises

Run:

```sh
rbb watch cipher-03
```

Four exercises. Then compute the stats above in the project.
