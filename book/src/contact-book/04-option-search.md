# Lesson 4 — Option & Search

> **Goal**: Search for a contact by name. Return its position if found, or signal that it wasn't found.
>
> **Concepts**: `Option<T>`, `Some(value)`, `None`, `match` on `Option`, `if let`, `.iter().enumerate()`.

---

## The problem: a search might fail

Suppose you want to find the contact named "Dave". If Dave exists in the list, great — return his contact details. If not, what do you return?

In C, you might return a null pointer. In Python, you might return `None`. Rust has a built-in type for this: `Option<T>`.

---

## `Option<T>` — a value that might not exist

`Option<T>` is an enum with two variants:

```rust
enum Option<T> {
    Some(T),   // there is a value, and here it is
    None,      // there is no value
}
```

`T` is a placeholder for any type. `Option<usize>` either holds `Some(3)` (the number 3) or `None` (no number).

You've seen Rust's `match` used on characters and ranges. `Option` works the same way:

```rust
let result: Option<usize> = Some(2);

match result {
    Some(i) => println!("found at index {}", i),
    None    => println!("not found"),
}
```

`match` on `Option` is **exhaustive** — you must handle both arms. Forgetting `None` is a compile error.

---

## Returning `Option` from a function

Here is a function that searches contacts by name and returns the index if found:

```rust
fn find_by_name(contacts: &[Contact], name: &str) -> Option<usize> {
    for (i, contact) in contacts.iter().enumerate() {
        if contact.name == name {
            return Some(i);
        }
    }
    None
}
```

Key parts:

- `&[Contact]` — a **slice** of contacts. A slice is a view into a contiguous sequence. You can pass a `&Vec<Contact>` where a `&[Contact]` is expected — Rust converts automatically.
- `contacts.iter().enumerate()` — iterates, giving `(index, &Contact)` pairs. `i` is the position (0-based), `contact` is a reference to the element.
- `Some(i)` — wraps the found index in `Option`.
- `None` at the end — the trailing expression. If no contact matched, the function returns `None`.

---

## `if let` — a shorter way to unwrap

When you only care about the `Some` case, `if let` is more concise than `match`:

```rust
if let Some(i) = find_by_name(&contacts, "Alice") {
    contacts[i].display();
} else {
    println!("  Not found.");
}
```

`if let Some(i) = expr` means: "if `expr` is `Some(i)`, bind `i` and run the block". If it's `None`, fall through to `else`.

`if let` is not more powerful than `match` — it's just shorter when you only care about one arm.

---

## Using the found index

Once you have an index, use it to access the element:

```rust
match find_by_name(&contacts, "Bob") {
    Some(i) => contacts[i].display(),
    None    => println!("  Not found."),
}
```

`contacts[i]` gives you the `Contact` at position `i`. If `i` is out of bounds, Rust panics — but since `find_by_name` only returns indices that came from the same vector, this is always safe.

---

## Putting it together

After lesson 4, the project can search:

```rust
fn find_by_name(contacts: &[Contact], name: &str) -> Option<usize> {
    for (i, contact) in contacts.iter().enumerate() {
        if contact.name == name {
            return Some(i);
        }
    }
    None
}

fn main() {
    let mut contacts: Vec<Contact> = Vec::new();
    contacts.push(Contact::new("Alice", "555-1234", "alice@example.com"));
    contacts.push(Contact::new("Bob",   "555-5678", "bob@example.com"));

    println!("=== Contact Book ===");
    for contact in &contacts {
        contact.display();
    }

    println!("---");
    let query = "Alice";
    match find_by_name(&contacts, query) {
        Some(i) => {
            println!("Found: ");
            contacts[i].display();
        }
        None => println!("  '{}' not found.", query),
    }
}
```

Expected output:

```
=== Contact Book ===
  Alice | 555-1234 | alice@example.com
  Bob | 555-5678 | bob@example.com
---
Found:
  Alice | 555-1234 | alice@example.com
```

---

## Exercises

Run:

```sh
rbb watch contact-04
```

Four exercises. Then add `find_by_name` to the project and demonstrate a search.
