# Lesson 2 — Vec & Collections

> **Goal**: Store multiple contacts in a list. Print all of them.
>
> **Concepts**: `Vec<T>`, `Vec::new()`, `.push()`, `let mut`, iterating with `for .. in &vec`, `.len()`, `.is_empty()`.

---

## The problem with one contact

After lesson 1 you can represent one contact as a `Contact` value. But a contact book holds many contacts. You need a collection.

Rust's standard growable list is `Vec<T>` — a **vector**. It stores any number of values of the same type `T`, in order, on the heap.

---

## Creating a `Vec`

```rust
let mut contacts: Vec<Contact> = Vec::new();
```

Breaking this down:

- `let mut` — the vector will grow, so it must be mutable
- `contacts` — the name
- `: Vec<Contact>` — the type annotation. `Vec<Contact>` means "a vector of `Contact` values". The `<Contact>` is the **type parameter** — it tells Rust what kind of things are inside.
- `Vec::new()` — creates an empty vector

You can also write it without the annotation if Rust can infer the type from the first `.push()`:

```rust
let mut contacts = Vec::new();
contacts.push(Contact { ... });   // Rust infers Vec<Contact>
```

Both forms are correct.

---

## Adding elements with `.push()`

```rust
contacts.push(Contact {
    name: String::from("Alice"),
    phone: String::from("555-1234"),
    email: String::from("alice@example.com"),
});

contacts.push(Contact {
    name: String::from("Bob"),
    phone: String::from("555-5678"),
    email: String::from("bob@example.com"),
});
```

`.push(value)` adds `value` to the end of the vector. The vector **takes ownership** of the value you push in.

`contacts` must be declared `let mut` for `.push()` to work. Trying to push into an immutable vector is a compile error.

---

## Length and emptiness

```rust
println!("{} contacts", contacts.len());   // number of elements

if contacts.is_empty() {
    println!("No contacts yet.");
}
```

`.len()` returns the number of elements as a `usize`. `.is_empty()` returns `true` if the vector has no elements — it's equivalent to `contacts.len() == 0` but more readable.

---

## Iterating over a Vec

To visit every contact and print it:

```rust
for contact in &contacts {
    println!("{} | {} | {}", contact.name, contact.phone, contact.email);
}
```

The `&` before `contacts` is critical. Let's understand why.

### `for contact in &contacts` — borrowing

`&contacts` borrows the vector. The loop gives you `contact: &Contact` — a reference to each element. You can read the contact's fields but you don't own them. The vector stays intact after the loop.

### `for contact in contacts` — moving

Without `&`, you **move** the contacts out of the vector. After the loop, `contacts` is gone — Rust considers it moved. You cannot use it again:

```rust
for contact in contacts {        // moves each Contact out
    println!("{}", contact.name);
}
println!("{}", contacts.len());  // compile error: value moved
```

**Rule of thumb**: when looping to read, always write `for x in &collection`.

---

## The heap and ownership

`Vec<T>` allocates memory on the heap to store its elements. When the `Vec` is dropped (goes out of scope), it frees that memory — and every `Contact` inside it is dropped too, freeing their `String` fields in turn.

This is Rust's ownership system at work: no garbage collector needed. You get predictable cleanup, automatically.

---

## Putting it together

After lesson 2, the project holds and prints multiple contacts:

```rust
struct Contact {
    name: String,
    phone: String,
    email: String,
}

fn main() {
    let mut contacts: Vec<Contact> = Vec::new();

    contacts.push(Contact {
        name: String::from("Alice"),
        phone: String::from("555-1234"),
        email: String::from("alice@example.com"),
    });
    contacts.push(Contact {
        name: String::from("Bob"),
        phone: String::from("555-5678"),
        email: String::from("bob@example.com"),
    });
    contacts.push(Contact {
        name: String::from("Carol"),
        phone: String::from("555-9012"),
        email: String::from("carol@example.com"),
    });

    println!("=== Contact Book ({} contacts) ===", contacts.len());
    for contact in &contacts {
        println!("  {} | {} | {}", contact.name, contact.phone, contact.email);
    }
}
```

Expected output:

```
=== Contact Book (3 contacts) ===
  Alice | 555-1234 | alice@example.com
  Bob | 555-5678 | bob@example.com
  Carol | 555-9012 | carol@example.com
```

---

## Exercises

Run:

```sh
rbb watch contact-02
```

Four exercises. When all pass, add the `Vec` and the loop to your project.
