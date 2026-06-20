# What we left out

This course teaches Rust by building real programs, not by surveying the language. That means some topics were skipped entirely, some were touched lightly, and some were deferred to later in the curriculum. This page names them and points you where to go next.

---

## Async/await

Every network program in this course uses blocking I/O — `TcpStream::connect` blocks until the connection is open, `read_to_string` blocks until the response arrives. This is simple and correct for a course server where concurrency is handled with threads.

The real world often uses **async Rust** instead: functions that can be paused while waiting for I/O, letting a single thread handle thousands of concurrent connections. The `async`/`await` keywords, the `Future` trait, and runtimes like `tokio` are the dominant way to write production network servers in Rust.

We used `macroquad`'s async loop, but that is a thin wrapper — we never wrote `Future` implementations or used `tokio`. *The Rust Async Book* (available at `rustup doc`) is the starting point.

---

## Macros

We called macros throughout the course — `println!`, `vec!`, `format!`, `assert_eq!` — but never wrote one.

Rust has two kinds:

- **Declarative macros** (`macro_rules!`) — pattern-matching on token trees. Good for reducing repetition. Covered in chapter 19 of *The Rust Programming Language*.
- **Procedural macros** — Rust code that runs at compile time and generates more Rust code. Used to implement `#[derive(Debug)]`, `#[derive(Serialize)]`, and similar attributes. Requires a separate crate and is significantly more complex.

---

## `unsafe`

Every line of code in this course is safe Rust. The compiler guarantees memory safety for all of it.

`unsafe` blocks opt out of those guarantees. They are needed when calling C libraries (FFI), implementing low-level data structures, or writing OS/embedded code. The *Rustonomicon* (`rustup doc --nomicon`) covers unsafe Rust in depth.

You should not need `unsafe` for typical application code.

---

## `Arc` and `Mutex`

The threading lessons used channels for all communication between threads. Channels are the right tool when threads produce results and hand them off.

Sometimes threads need to *share* mutable state — a counter, a cache, a list of connections. That requires:

- **`Arc<T>`** — a reference-counted smart pointer that can be cloned across threads (`Rc<T>` is the single-threaded version)
- **`Mutex<T>`** — a lock that ensures only one thread accesses the data at a time

`Arc<Mutex<T>>` is the standard pattern for shared mutable state. It was left out because channels cover most beginner use cases cleanly, and `Mutex` introduces deadlock risks that require careful discipline.

---

## Lifetimes

Lifetimes appeared once — in the Pong lesson, where a struct held a reference and needed an annotation. We treated them as "something the compiler asks for" without fully explaining the rules.

Lifetimes are Rust's way of tracking how long references are valid. They are most visible in structs that hold references, in functions that return references, and in some trait bounds. Chapter 10 of *The Rust Programming Language* covers them fully.

---

## Generics in depth

We wrote generic functions (`fn search<T>`) and used generic types (`Vec<T>`, `Option<T>`) throughout, but never wrote generic structs or explored trait bounds deeply. Topics not covered:

- `where` clauses
- Associated types (vs type parameters)
- Generic structs and `impl` blocks
- Turbofish syntax (`::<>`)
- Const generics

---

## Error handling with `?`

We used `.unwrap()` and `.expect()` throughout the course. These are fine for learning — they crash loudly on failure. Production code uses the `?` operator to propagate errors up the call stack:

```rust
fn read_config(path: &Path) -> Result<Config, io::Error> {
    let text = fs::read_to_string(path)?;  // returns early on error
    // ...
}
```

The `?` operator works with `Result` and `Option`, and requires that error types are compatible. The `thiserror` and `anyhow` crates handle the common cases for library and application error types respectively.

---

## Traits in depth

We defined and implemented traits, but some advanced uses were skipped:

- **Trait objects** (`dyn Trait`) — dynamic dispatch, where the concrete type is not known at compile time
- **Blanket implementations** — implementing a trait for all types that implement another trait
- **Operator overloading** — touched in the physics projects but not generalised
- **Default method implementations** — mentioned but not explored
- **`impl Trait` in return position** — returning "some type that implements Trait" without naming it

---

## Testing

We wrote tests in every project using `#[test]` and `assert_eq!`, but Rust's testing infrastructure has more:

- **`#[should_panic]`** — tests that expect a panic
- **Doc tests** — examples in doc comments that run as tests
- **Integration tests** — in a `tests/` directory, testing the public API as an external user would
- **Test organisation** — the `#[cfg(test)]` module pattern

`cargo test -- --help` lists all the test runner flags (filtering by name, running ignored tests, controlling output).

---

## What to read next

If you want to continue learning Rust after this course:

- **[*The Rust Programming Language*](https://doc.rust-lang.org/book/)** — the official book, available offline via `rustup doc --book`. Fill in the gaps left by this course.
- **[*Rust by Example*](https://doc.rust-lang.org/rust-by-example/)** — short code examples for every language feature. Available at `rustup doc --example` if your toolchain includes it.
- **[*Rustlings*](https://github.com/rust-lang/rustlings)** — small exercises that cover everything the Rust book covers. Good for drilling the concepts that felt shaky.
- **[*Zero To Production In Rust*](https://www.zero2prod.com/)** — a book-length project building a real email newsletter service. Covers async, error handling, databases, testing, and deployment in depth.
