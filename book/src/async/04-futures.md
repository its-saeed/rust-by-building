# Chapter 4 — Futures

Think of a ticket at a deli counter: you hand in your order and get a small numbered slip back. The food isn't ready yet — the ticket just represents "food that will exist at some point." You can check on it, wait by the counter, or go browse the shop while you wait. A `Future` in Rust is that ticket.

---

## What a Future is

A `Future` is a value that represents a computation that has not finished yet. It might complete in a millisecond, or it might take minutes. The key point: the future *is not the result* — it is a placeholder for a result that will come.

```rust
// This does NOT fetch anything yet.
// It produces a Future — a description of what to do.
let fut = fetch_page("https://example.com");

// Nothing happens until someone drives the future to completion.
```

Futures are **lazy**. Creating one does zero work. If you never drive it, the computation never runs. This is different from most languages where calling an async function immediately starts a background task.

---

## The `Future` trait

Every async value in Rust implements the `Future` trait. The whole trait is one method:

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

`poll()` asks the future: "are you done yet?" The answer is one of two things:

```
Poll::Ready(value)   — done, here is the result
Poll::Pending        — not yet, try again later
```

The runtime calls `poll()` repeatedly until it gets `Ready`. In between, the future does not consume any thread.

---

## The state machine model

When you write an `async fn`, the compiler transforms it into a **state machine** — a struct that implements `Future`. Each `.await` point becomes a state boundary.

Here is a simple async function:

```rust
async fn send_report(url: &str) -> Result<(), Error> {
    let data = fetch_data().await;      // await point 1
    let resp = post(url, data).await;   // await point 2
    Ok(())
}
```

The compiler generates something like this (simplified):

```
┌─────────────────────────────────────────────────────────┐
│  SendReport state machine                               │
│                                                         │
│   State S0 ──► poll fetch_data()                        │
│       │            │                                    │
│       │      Ready(data)                                │
│       ▼            │                                    │
│   State S1 ◄───────┘                                    │
│       │                                                 │
│       ├──► poll post(url, data)                         │
│       │            │                                    │
│       │      Ready(resp)                                │
│       ▼            │                                    │
│   State S2 ◄───────┘  (done — returns Ready(Ok(())))   │
└─────────────────────────────────────────────────────────┘
```

- **S0**: initial — start polling `fetch_data()`
- **S1**: `fetch_data()` returned `Ready`; now polling `post()`
- **S2**: `post()` returned `Ready`; we are done

If `fetch_data()` returns `Pending`, the state machine saves where it is (still S0) and returns `Pending` upward. Next time `poll()` is called, it picks up exactly where it left off. No stack needed between polls — all state lives in the struct on the heap.

---

## Wakers — how a future asks to be polled again

Returning `Pending` is only useful if the future gets polled again at the right moment. That is what the `Waker` is for.

The `Context` passed into `poll()` carries a `Waker`. When a future cannot make progress — for instance, it is waiting for data on a socket — it stores a clone of the waker and returns `Pending`. Later, when the OS reports that data has arrived on that socket, the event loop calls `waker.wake()`. This signals the runtime to schedule another `poll()` on that future.

```
┌─────────────────────────────────────────────────────────────┐
│                      waker flow                             │
│                                                             │
│  1. Runtime calls poll(cx) on a future                      │
│                                                             │
│  2. Future asks OS for data — not ready                     │
│     Future stores cx.waker().clone() inside itself          │
│     Future returns Poll::Pending                            │
│                                                             │
│  3. Runtime does other work                                 │
│                                                             │
│  4. OS: data arrives on socket                              │
│     Event loop fires the stored waker                       │
│                                                             │
│  5. Runtime schedules the future for polling again          │
│     Runtime calls poll(cx) — this time data is available    │
│     Future returns Poll::Ready(data)                        │
└─────────────────────────────────────────────────────────────┘
```

No thread is parked between steps 2 and 5. The future just sits as a struct on the heap, holding its state.

---

## Futures compose

You can build large futures out of small ones. The outer future drives inner futures by polling them. When you write:

```rust
let data = fetch_data().await;
```

…the outer future calls `poll()` on the inner `fetch_data()` future on your behalf. If `fetch_data()` returns `Pending`, the outer future returns `Pending` too — propagating the "not yet" signal up the chain until it reaches the runtime.

This is why async Rust is **zero-cost**: there is no hidden thread pool, no callback registration, no allocations per await point. It compiles down to a state machine that gets polled. The overhead is a function call and an enum comparison.

---

## Why you rarely implement `Future` directly

You will almost never write an `impl Future` block by hand. The `async fn` keyword does it for you. But knowing the poll model tells you three important things:

1. **Why futures are lazy** — creating a future is just building a struct; `poll()` is never called until a runtime does it.
2. **Why `.await` is required** — without it, you have a `Future` value sitting there, never polled, never progressing. The compiler warns you about this exactly because it is always a mistake.
3. **Why the compiler complains about `Pin`** — the state machine holds references to its own fields across await points; `Pin` ensures it cannot be moved in memory after polling starts.

---

## Key ideas

| Concept | What it means |
|---------|---------------|
| `Future` | A value representing a computation that may not be done yet |
| Lazy | Creating a future does no work; work starts only when polled |
| `poll()` | The single method on `Future`; returns `Ready(value)` or `Pending` |
| State machine | An `async fn` compiles into a struct; each `.await` is a state boundary |
| `Waker` | Lets a future tell the runtime "poll me again when I might be ready" |
| Composition | Outer futures drive inner futures by calling their `poll()` |
| Zero-cost | No hidden threads, no allocations per await — just a struct and a function call |
