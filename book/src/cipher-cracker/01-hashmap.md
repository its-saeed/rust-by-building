# Lesson 1 — HashMap

> **Goal**: Count how many times each letter appears in the ciphertext.
>
> **Concepts**: `HashMap<K, V>`, `HashMap::new()`, `.insert()`, `.get()`, the entry API, iterating over a HashMap.

---

## The right tool for counting

To count letter frequencies, you need to answer: "how many times have I seen the letter `'e'`?"

A `Vec` doesn't work well here — you'd have to store 26 entries, one per letter of the alphabet, and manually compute the index for each letter. A `HashMap` is the right choice.

A **HashMap** stores **key-value pairs**. Given a key, it finds the value in O(1) time — immediately, regardless of how many items are stored.

For counting letters:
- **Key**: a `char` — the letter
- **Value**: a `u32` — how many times it has appeared

---

## Bringing HashMap into scope

HashMap lives in the standard library's `collections` module. You must bring it into scope before using it:

```rust
use std::collections::HashMap;
```

This is a `use` declaration — it makes `HashMap` available by its short name. Without it, you'd have to write `std::collections::HashMap` everywhere.

---

## Creating a HashMap

```rust
let mut freq: HashMap<char, u32> = HashMap::new();
```

- `HashMap<char, u32>` — the type: keys are `char`, values are `u32`
- `HashMap::new()` — creates an empty map
- `let mut` — the map will change (we'll insert into it), so it must be mutable

---

## Inserting and reading

```rust
freq.insert('e', 1);      // add the entry 'e' → 1
freq.insert('t', 3);      // add 't' → 3

let count = freq.get(&'e');  // look up 'e'
```

`.get()` returns `Option<&u32>` — either `Some(&1)` if `'e'` is in the map, or `None` if it isn't. You already know `Option` from project 2.

`.insert()` returns the **old value** if the key already existed (wrapped in `Option`), or `None` if the key was new. Usually you ignore this return value.

---

## The counting pattern — the entry API

Inserting one-by-one doesn't work for counting. If you call `.insert('e', 1)` twice, the second call overwrites the first and you still have `1`.

What you need: "if `'e'` is already in the map, add 1 to its value; otherwise, insert it with value 1."

Rust provides this with the **entry API**:

```rust
*freq.entry(ch).or_insert(0) += 1;
```

This is a single line that does all the work. Let's break it down:

```rust
freq.entry(ch)          // look at the slot for key `ch` in the map
    .or_insert(0)       // if the slot is empty, put 0 in it; return &mut u32
```

`.or_insert(0)` returns `&mut u32` — a mutable reference to the value (whether it was just inserted or already existed).

```rust
* ... += 1;             // dereference the reference, then add 1
```

The `*` dereferences the `&mut u32` to get the `u32`, then `+= 1` increments it.

You can also split this into two lines for clarity:

```rust
let count = freq.entry(ch).or_insert(0);  // &mut u32
*count += 1;
```

Both forms do the same thing.

### Why `*` is needed

`.or_insert(0)` gives back a **reference** (`&mut u32`), not the value itself. You can't do arithmetic directly on a reference — you must dereference it first.

This is the same rule you saw with `&mut String` in project 2's user input: to change the value behind a reference, you go through `*`.

---

## Counting all letters in a string

```rust
use std::collections::HashMap;

fn count_letters(text: &str) -> HashMap<char, u32> {
    let mut freq: HashMap<char, u32> = HashMap::new();
    for ch in text.chars() {
        *freq.entry(ch).or_insert(0) += 1;
    }
    freq
}
```

This loops over every character. For each one, it increments (or initialises) the count in the map.

---

## Iterating over a HashMap

To print all entries, use `.iter()`:

```rust
for (ch, count) in &freq {
    println!("  '{}': {}", ch, count);
}
```

`&freq` borrows the map. Each iteration gives a `(&char, &u32)` pair — references to the key and value. The `(ch, count)` pattern destructures the pair into two variables.

One important note: **HashMap does not preserve insertion order**. If you iterate over it, the entries come out in an arbitrary order. You'll fix this in lesson 2 by sorting.

---

## Putting it together

After lesson 1, the project counts every character in the ciphertext:

```rust
use std::collections::HashMap;

fn count_letters(text: &str) -> HashMap<char, u32> {
    let mut freq: HashMap<char, u32> = HashMap::new();
    for ch in text.chars() {
        *freq.entry(ch).or_insert(0) += 1;
    }
    freq
}

fn main() {
    let ciphertext = "dolfh zdv ehjlqqlqj wr jhw yhub wluhg ri vlwwlqj eb khu vlvwhu rq wkh edqn";

    println!("=== Cipher Cracker ===");
    println!("Ciphertext: {:?}\n", ciphertext);

    let freq = count_letters(ciphertext);

    println!("Character counts:");
    for (ch, count) in &freq {
        println!("  {:?}: {}", ch, count);
    }
}
```

The output will be unsorted (HashMap order is arbitrary). Sorting comes in lesson 2.

---

## Exercises

Run:

```sh
rbb watch cipher-01
```

Four exercises. Then add `count_letters` to the project and print the raw counts.
