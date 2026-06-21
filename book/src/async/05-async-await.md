# Chapter 5 — async/await

`async/await` is the reason you do not have to think about state machines every day: the compiler writes them for you, and you write code that looks almost identical to ordinary sequential Rust.

---

## The transformation

When you mark a function `async`, the compiler changes its return type from `T` to `impl Future<Output = T>`. The body is rewritten into a state machine as described in the previous chapter.

```rust
// What you write
async fn fetch(url: &str) -> String {
    // ...
}

// What the compiler produces (approximately)
fn fetch(url: &str) -> impl Future<Output = String> {
    // ...state machine struct...
}
```

The function signature looks the same at the call site. The difference is that calling it gives you a future, not a string.

---

## `async fn` — declaring an async function

Any function can be made async by adding the `async` keyword before `fn`:

```rust
async fn load_user(id: u64) -> User {
    // inside here you can use .await
}
```

Rules:
- An `async fn` always returns a `Future`, even if it looks like it returns `User`.
- Calling an `async fn` does nothing — it builds the future struct. Work starts when it is polled.
- You can only use `.await` inside an `async fn` or an `async` block.

---

## `.await` — suspending until ready

`.await` is how you pause your async function until another future completes:

```rust
let user = load_user(42).await;
```

This means: "poll `load_user(42)` until it returns `Ready`. While it is `Pending`, yield control back to the runtime so other tasks can run. When it is `Ready`, bind the result to `user` and continue."

`.await` is not blocking. The thread is free to run other futures while this one waits. That is the whole point.

---

## Before and after: callback style vs async/await

Async code without `async/await` looked like this — chains of callbacks, each called when the previous step finished:

```rust
// Callback style (not real Rust — illustrative only)
fetch_user(id, |user| {
    fetch_orders(user.id, |orders| {
        fetch_invoice(orders[0].id, |invoice| {
            send_email(user.email, invoice, |result| {
                // finally done — four levels deep
            });
        });
    });
});
```

Each step is a closure passed to the previous step. Error handling must be threaded through every level. The logic is inside-out — the last thing that happens is written first.

With `async/await` the same logic is:

```rust
async fn process_order(id: u64) -> Result<(), Error> {
    let user    = fetch_user(id).await?;
    let orders  = fetch_orders(user.id).await?;
    let invoice = fetch_invoice(orders[0].id).await?;
    send_email(user.email, invoice).await?;
    Ok(())
}
```

Top to bottom. Error propagation with `?` works exactly as in synchronous Rust. No nesting. This is why `async/await` was such a large step forward — it made async code readable.

---

## Common mistake: forgetting `.await`

```rust
async fn get_count() -> u32 { 42 }

async fn example() {
    let count = get_count();   // BUG: count is a Future, not a u32
    println!("{count}");       // compile error: Future doesn't implement Display
}
```

The compiler will warn you:

```
warning: unused implementor of `Future` that must be used
 --> src/main.rs:5:17
  |
5 |     let count = get_count();
  |                 ^^^^^^^^^^^
  = note: futures do nothing unless you `.await` or poll them
```

The fix is always to add `.await`:

```rust
let count = get_count().await;
```

If you see a compiler error about a `Future` where you expected a plain value, the cause is almost always a missing `.await`.

---

## `async` blocks

You can create a future inline without a named function using an `async` block:

```rust
let fut = async {
    let a = step_one().await;
    let b = step_two(a).await;
    b + 1
};

// fut is a Future<Output = i32>
// Nothing has run yet.
let result = fut.await;
```

`async` blocks are useful when you need a small future in the middle of other code — for instance, when spawning a task or selecting between multiple futures. They work exactly like `async fn` bodies.

---

## A complete example

```rust
async fn fetch_greeting() -> String {
    // imagine this does a network call
    String::from("Hello")
}

async fn greet(name: &str) -> String {
    let greeting = fetch_greeting().await;
    format!("{greeting}, {name}!")
}
```

Step by step:
1. `greet("world")` builds a future. Nothing runs.
2. When a runtime polls it, `fetch_greeting()` is called — producing another future.
3. `greet`'s state machine polls `fetch_greeting`'s future.
4. When that returns `Ready("Hello")`, `greet` binds it to `greeting` and moves to the next state.
5. `format!` runs — no await needed, it is synchronous.
6. `greet` returns `Ready("Hello, world!")`.

---

## The golden rule

`.await` can only appear inside an `async fn` or `async` block. Outside that context, there is no state machine to suspend, nowhere for `Pending` to propagate to.

```rust
fn main() {
    let result = greet("world").await;   // compile error: .await outside async
}
```

To drive a future from `main()` you need a runtime. That is the next chapter.

---

## Key ideas

| Concept | What it means |
|---------|---------------|
| `async fn` | Declares a function that returns a `Future` instead of its value directly |
| Calling `async fn` | Builds a future struct — no work runs |
| `.await` | Polls an inner future until `Ready`, yielding the thread while `Pending` |
| Not blocking | The thread runs other tasks while a future is `Pending` — no thread is parked |
| Missing `.await` | Gives you a `Future` instead of its result; the compiler warns you |
| `async` block | Creates an anonymous inline future without a named function |
| Golden rule | `.await` only works inside `async fn` or `async {}` |
