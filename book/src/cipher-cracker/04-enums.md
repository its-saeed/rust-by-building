# Lesson 4 — Enums

> **Goal**: Learn to model "one of several possibilities" with enums. Use `CrackResult` to make the cracker's success and failure cases explicit.
>
> **Concepts**: enum definition, unit variants, variants with data, `match`, `if let`, `Option<T>` as an enum.

---

## You've already used an enum

Every time you've written `Some(value)` or `None`, you've been using an enum. Here is how `Option` is actually defined in the standard library:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

An **enum** (short for *enumeration*) defines a type that can be exactly **one of several named variants**. `Option<T>` can be either `Some(value)` or `None` — never both, never neither.

---

## Defining your own enum

```rust
enum Direction {
    North,
    South,
    East,
    West,
}
```

Each name (`North`, `South`, etc.) is a **variant**. These are **unit variants** — they carry no data, just an identity.

Create a value:

```rust
let heading = Direction::North;
```

---

## `match` with enums

`match` is how you act on which variant you have. Every variant must be handled:

```rust
fn describe(d: Direction) -> &'static str {
    match d {
        Direction::North => "heading north",
        Direction::South => "heading south",
        Direction::East  => "heading east",
        Direction::West  => "heading west",
    }
}
```

If you forget a variant, the compiler tells you. There is no way to accidentally miss a case.

---

## Variants with data

Variants can carry values. This is what makes enums powerful:

```rust
enum Shape {
    Circle(f64),              // one f64: the radius
    Rectangle(f64, f64),      // two f64s: width, height
}
```

Create them:

```rust
let c = Shape::Circle(3.0);
let r = Shape::Rectangle(4.0, 6.0);
```

Extract the data in a `match`:

```rust
fn area(s: Shape) -> f64 {
    match s {
        Shape::Circle(r)       => 3.14159 * r * r,
        Shape::Rectangle(w, h) => w * h,
    }
}
```

The variable names in the pattern (`r`, `w`, `h`) are bound to the values stored in the variant. You name them whatever makes sense.

---

## Variants with named fields

You can also give fields names, like a struct inside a variant:

```rust
enum CrackResult {
    Success { key: u8, plaintext: String },
    TooFewLetters,
}
```

`Success` carries two named pieces of data. `TooFewLetters` carries nothing — it's a unit variant.

Create a `Success`:

```rust
let result = CrackResult::Success {
    key: 3,
    plaintext: String::from("alice was beginning to get very tired"),
};
```

Match on it:

```rust
match result {
    CrackResult::Success { key, plaintext } => {
        println!("Key {}: {}", key, plaintext);
    }
    CrackResult::TooFewLetters => {
        println!("Not enough letters to analyse.");
    }
}
```

The names in the pattern (`key`, `plaintext`) must match the field names in the definition.

---

## Why `CrackResult` is better than returning a `String`

Consider two ways to write `crack`:

```rust
// Old approach — empty String means "failure"
fn crack(text: &str) -> String { ... }

// New approach — explicit success/failure
fn crack(text: &str) -> CrackResult { ... }
```

With the old approach, every caller has to remember to check whether the String is empty. It's easy to forget. With `CrackResult`, the compiler forces you to handle both cases in a `match` — you can't accidentally ignore the failure.

This pattern — using an enum to make success and failure explicit — is fundamental in Rust.

---

## `if let` — matching one variant

When you only care about one variant and want to ignore the rest:

```rust
if let CrackResult::Success { key, plaintext } = result {
    println!("Key {}: {}", key, plaintext);
}
// if it's TooFewLetters, nothing happens
```

`if let` is shorthand for a `match` with one real arm and a `_` catch-all. Use it when you only care about success (or any single variant).

You've seen this before with `Option`:

```rust
if let Some(peak) = top_letter(&freq) {
    println!("Most frequent: '{}'", peak);
}
```

Same idea — `if let` unwraps `Some` and binds the value, ignoring `None`.

---

## Putting it together

After lesson 3, the project defines `CrackResult` and `Command`, and uses `match` to handle each:

```rust
enum CrackResult {
    Success { key: u8, plaintext: String },
    TooFewLetters,
}

enum Command {
    Crack(String),
    Quit,
}
```

`CrackResult` is used in lesson 4 when you implement `crack`. `Command` is used in lesson 5 when you build the interactive loop.

---

## Exercises

Run:

```sh
rbb watch cipher-04
```

Four exercises. Then define both enums in the project and implement a `describe` method on `CrackResult` using `match`.
