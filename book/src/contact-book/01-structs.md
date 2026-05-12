# Lesson 1 — Structs

> **Goal**: Define a `Contact` type with three fields. Create one contact and print its details.
>
> **Concepts**: `struct`, field definitions, creating instances, field access with `.`, `String::from()`.

---

## Why structs?

In the Caesar cipher, data was simple: a message (`&str`) and a key (`u8`). Two separate variables worked fine.

A contact is different. A contact has a name, a phone number, and an email address. They belong together — they describe one person. Passing three separate variables around would be messy and error-prone. A **struct** solves this by bundling related data under one name.

---

## Defining a struct

```rust
struct Contact {
    name: String,
    phone: String,
    email: String,
}
```

Read this top to bottom:

- `struct` — the keyword that declares a new type
- `Contact` — the name. Rust convention: `PascalCase` (each word capitalised, no underscores)
- `{ ... }` — the body, containing a list of **fields**
- Each field: `name: Type` — a name, a colon, a type, a trailing comma

This defines a new type called `Contact`. It does not create any value yet — it just describes the shape.

---

## Creating an instance

To make an actual `Contact`, you write a **struct literal**:

```rust
let contact = Contact {
    name: String::from("Alice"),
    phone: String::from("555-1234"),
    email: String::from("alice@example.com"),
};
```

You must provide every field. The order doesn't need to match the definition, but by convention you follow the same order. If you forget a field, the compiler tells you which one is missing.

---

## Accessing fields with `.`

Once you have a `Contact`, read its fields with `.`:

```rust
println!("Name:  {}", contact.name);
println!("Phone: {}", contact.phone);
println!("Email: {}", contact.email);
```

`contact.name` means "the `name` field of `contact`". This is the same dot notation used in C, Go, and Python.

---

## `String::from()` — creating owned strings

You might wonder: why not just write `"Alice"` directly?

```rust
let contact = Contact {
    name: "Alice",   // compile error!
    ...
};
```

This fails because `"Alice"` is a `&str` — a reference to text baked into the program. But `name` is typed as `String`, which is an owned, heap-allocated string. They are different types and Rust won't silently convert between them.

`String::from("Alice")` takes the `&str` and creates an owned `String` from it:

```rust
let s: String = String::from("Alice");
```

There is an equivalent shorthand: `"Alice".to_string()`. Both do the same thing.

| Expression | Type | What it means |
|---|---|---|
| `"Alice"` | `&str` | A reference to a string literal in the binary |
| `String::from("Alice")` | `String` | An owned, heap-allocated copy |
| `"Alice".to_string()` | `String` | Same as above, different syntax |

For struct fields that store text, always use `String`. Use `&str` for function parameters where you just need to read the text (more on this in lesson 3).

---

## Why `String` in struct fields?

A struct must own its data. Consider: if `name` were a `&str` — a reference — it would point to text that lives somewhere else. That "somewhere else" could be freed while the `Contact` still exists. Rust would reject this at compile time.

`String` avoids the problem: the `Contact` owns its text directly. When the `Contact` is dropped (goes out of scope), its strings are freed with it. Clean, safe, no dangling references.

---

## Putting it together

After lesson 1, the project creates and prints one contact:

```rust
struct Contact {
    name: String,
    phone: String,
    email: String,
}

fn main() {
    let contact = Contact {
        name: String::from("Alice"),
        phone: String::from("555-1234"),
        email: String::from("alice@example.com"),
    };

    println!("=== Contact Book ===");
    println!("Name  : {}", contact.name);
    println!("Phone : {}", contact.phone);
    println!("Email : {}", contact.email);
}
```

Expected output:

```
=== Contact Book ===
Name  : Alice
Phone : 555-1234
Email : alice@example.com
```

---

## Exercises

Run:

```sh
rbb watch contact-01
```

Four exercises. Each targets one concept from this lesson. When all pass, open `project/src/main.rs` and complete the project step.
