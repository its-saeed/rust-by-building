# Project 2 — Contact Book

> **What you'll build**: A command-line contact book where you can add contacts, search by name, and list them all.
>
> **Lessons**: 5 lessons + 1 final exercise.
>
> **Rust concepts covered**: `struct`, `impl`, methods, `Vec<T>`, `Option<T>`, user input, `loop`.

## What you'll build

A contact book stores people's details — name, phone number, email address. You can add new contacts, search for someone by name, and list everyone.

By the end of this project you'll have a working interactive program:

```
=== Contact Book ===
Commands: add, find, list, quit
> add
  Name: Alice
  Phone: 555-1234
  Email: alice@example.com
  Added.
> list
  Alice | 555-1234 | alice@example.com
> find
  Name: Alice
  Alice | 555-1234 | alice@example.com
> quit
Goodbye.
```

## What you'll build, lesson by lesson

| Lesson | What gets added |
|--------|-----------------|
| 1 — Structs | Define `Contact`, create one, print its fields |
| 2 — Vec & Collections | Store multiple contacts in a `Vec` |
| 3 — Methods & `impl` | `Contact::new()` and `contact.display()` |
| 4 — Option & Search | `find_by_name` returns `Option<usize>` |
| 5 — User Input | Interactive add / find / list / quit loop |
| Final Exercise | End-to-end: full contact book from scratch |

## How lessons work

Each lesson has two parts:

1. **Read** — this book. Work through the chapter before touching any code.
2. **Exercises** — small broken programs. Fix each one. Run `rbb watch contact-XX` for instant feedback.
3. **Project step** — add the lesson's feature to the contact book program.

Start with the chapter. Then open the exercises. Then the project.
