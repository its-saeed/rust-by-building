# Reading stdlib docs offline

The course server has no internet connection — and you do not need one. The full Rust documentation is available locally.

---

## The standard library

```sh
rustup doc --std
```

This opens the standard library reference in your browser (or prints the path if you are in a headless SSH session). Every type, trait, and function in `std` is documented there.

Common starting points:

| What you need | Where to look |
|---------------|--------------|
| String methods | `std::str` / `std::string::String` |
| Vec methods | `std::vec::Vec` |
| HashMap | `std::collections::HashMap` |
| File I/O | `std::fs` / `std::io` |
| Threads | `std::thread` / `std::sync` |
| Error handling | `std::result` / `std::option` |
| Time | `std::time` |
| Network | `std::net` |

---

## Compiler error explanations

When the compiler gives you an error code like `E0382`, run:

```sh
rustc --explain E0382
```

You get a full tutorial on that specific error — what causes it, why Rust rejects it, and how to fix it. These explanations are better than most Stack Overflow answers.

---

## The Rust book

```sh
rustup doc --book
```

Opens *The Rust Programming Language* (the official book) locally. If you want to look up how ownership works, what lifetimes mean, or how traits are defined, it is here.

---

## Listing all available docs

```sh
rustup doc
```

Without arguments, this opens an index of everything installed: the book, the reference, the standard library, Rustonomicon, and more.

---

## Finding the path

If you are on a headless SSH session and cannot open a browser:

```sh
rustup doc --path
```

This prints the filesystem path to the docs root. You can then either:

- Open the HTML files directly if you have a local mount
- Ask your admin whether docs are served over HTTP on the course server (they may be at something like `http://course-server:8080/doc`)

---

## Searching within the docs

The browser-based docs have a search bar (press `S` to focus it). Type any type or function name to jump straight to it. This is faster than navigating the module tree.

---

## Crate docs

The course's vendor tree ships with documentation for every crate in the workspace. To read them:

```sh
cargo doc --open
```

Run this from inside a project directory. It builds and opens docs for all dependencies used by that project — `macroquad`, `num-complex`, `reqwest`, etc.
