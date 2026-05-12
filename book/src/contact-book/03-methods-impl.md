# Lesson 3 — Methods & `impl`

> **Goal**: Add a constructor and a display method to `Contact`. Replace the verbose struct literals with `Contact::new(...)`.
>
> **Concepts**: `impl` blocks, methods (`&self`), associated functions, `.to_string()`, calling methods vs calling functions.

---

## The problem with struct literals everywhere

After lesson 2, every time you create a contact you write out all three fields explicitly:

```rust
contacts.push(Contact {
    name: String::from("Alice"),
    phone: String::from("555-1234"),
    email: String::from("alice@example.com"),
});
```

And every time you print one:

```rust
println!("  {} | {} | {}", contact.name, contact.phone, contact.email);
```

Both details are repeated everywhere. If the format changes, you update dozens of lines. An `impl` block solves this.

---

## `impl` blocks — attaching behaviour to a type

An `impl` block lets you define functions that belong to a type:

```rust
impl Contact {
    // functions go here
}
```

Functions inside an `impl Contact` block are called **associated with** the `Contact` type. There are two flavours:

1. **Methods** — take `&self` as the first parameter. Called with dot notation: `contact.display()`.
2. **Associated functions** — take no `self`. Called with `::`: `Contact::new(...)`.

---

## Associated functions — `Contact::new`

An associated function is just a regular function that lives inside an `impl` block. By convention, `new` creates a value of the type:

```rust
impl Contact {
    fn new(name: &str, phone: &str, email: &str) -> Contact {
        Contact {
            name: name.to_string(),
            phone: phone.to_string(),
            email: email.to_string(),
        }
    }
}
```

Notice the parameters are `&str` — string slices. The caller passes string literals, which are `&str`. Inside, `.to_string()` converts each `&str` into an owned `String` for the struct.

`.to_string()` is equivalent to `String::from(...)`. Both produce an owned `String` from a `&str`. `.to_string()` is idiomatic in method chains.

Call it with `::`:

```rust
let alice = Contact::new("Alice", "555-1234", "alice@example.com");
```

Clean and concise.

---

## Methods — `contact.display()`

A method is a function whose first parameter is `self` (or `&self`, or `&mut self`). This gives the function access to the value it was called on.

```rust
impl Contact {
    fn display(&self) {
        println!("  {} | {} | {}", self.name, self.phone, self.email);
    }
}
```

- `&self` — borrows the `Contact` for reading. The method can read fields but not modify them.
- `self.name` — accesses the `name` field of the contact the method was called on.

Call it with dot notation:

```rust
contact.display();
```

Rust automatically passes the contact as `&self`. You don't write `Contact::display(&contact)` (though that also works).

---

## `&self` vs `&mut self` vs `self`

| Receiver | What it means | When to use |
|---|---|---|
| `&self` | Borrow immutably | Reading fields |
| `&mut self` | Borrow mutably | Modifying fields |
| `self` | Take ownership | Consuming the value |

`display` only reads fields, so `&self` is correct. We'll see `&mut self` in lesson 5 when we add a method that modifies the list.

---

## Multiple functions in one `impl` block

You can put as many functions as you want in one `impl` block:

```rust
impl Contact {
    fn new(name: &str, phone: &str, email: &str) -> Contact {
        Contact {
            name: name.to_string(),
            phone: phone.to_string(),
            email: email.to_string(),
        }
    }

    fn display(&self) {
        println!("  {} | {} | {}", self.name, self.phone, self.email);
    }
}
```

---

## Putting it together

After lesson 3, creation and printing go through the `impl` block:

```rust
struct Contact {
    name: String,
    phone: String,
    email: String,
}

impl Contact {
    fn new(name: &str, phone: &str, email: &str) -> Contact {
        Contact {
            name: name.to_string(),
            phone: phone.to_string(),
            email: email.to_string(),
        }
    }

    fn display(&self) {
        println!("  {} | {} | {}", self.name, self.phone, self.email);
    }
}

fn main() {
    let mut contacts: Vec<Contact> = Vec::new();

    contacts.push(Contact::new("Alice", "555-1234", "alice@example.com"));
    contacts.push(Contact::new("Bob",   "555-5678", "bob@example.com"));
    contacts.push(Contact::new("Carol", "555-9012", "carol@example.com"));

    println!("=== Contact Book ({} contacts) ===", contacts.len());
    for contact in &contacts {
        contact.display();
    }
}
```

Same output as lesson 2 — but the code is cleaner and the details are in one place.

---

## Exercises

Run:

```sh
rbb watch contact-03
```

Four exercises. Then update the project: add the `impl Contact` block with `new` and `display`, and use them in `main`.
