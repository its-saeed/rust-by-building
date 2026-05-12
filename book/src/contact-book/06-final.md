# Final Exercise — Contact Book End-to-End

> **No new concepts.** This is a test of everything from lessons 1–5.
>
> Three exercises. No scaffolding — you write or fix the whole thing.

---

## What you've built

Over the last five lessons you've learned:

| Lesson | Concept |
|--------|---------|
| 1 | `struct`, field definitions, `String::from()` |
| 2 | `Vec<T>`, `.push()`, `for x in &vec`, `.len()` |
| 3 | `impl`, associated functions (`new`), methods (`&self`) |
| 4 | `Option<T>`, `Some` / `None`, `match`, `if let`, `.enumerate()` |
| 5 | `std::io`, `read_line`, `.trim()`, `loop`, `break`, command dispatch |

The result is a full interactive contact book.

---

## Exercise 1 — Implement from scratch

Open `exercises/ex01_implement.rs`. The function signatures are there with `todo!()` bodies. Implement all of them. All tests must pass.

The functions you need to implement:
- `Contact::new` — construct a `Contact` from `&str` arguments
- `Contact::display` — print the contact's details
- `find_by_name` — search a slice by name, return `Option<usize>`

This is closed-book. Use the compiler errors as your guide.

---

## Exercise 2 — Find the bug

Open `exercises/ex02_find_the_bug.rs`. There is a complete implementation — but something is wrong. The tests fail.

Find the bug and fix it. There is exactly one bug. Read the failing test output carefully.

---

## Exercise 3 — Extend it

Open `exercises/ex03_delete.rs`. The contact book is mostly working, but there is no way to remove a contact. Implement the `delete_by_name` function so the tests pass.

Hint: `Vec` has a `.remove(index)` method that removes the element at a given index and shifts the rest.

---

## Run the exercises

```sh
rbb watch contact-06
```

Exercises 1 and 2 are verified by tests. Exercise 3 is verified by tests.
