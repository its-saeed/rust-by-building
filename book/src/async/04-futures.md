# Chapter 4 — Futures

In the last chapter, the event loop needed handlers that could be *paused*. When a socket has no data yet, the handler cannot block the thread — it has to stop, hand control back to the loop, and be resumed later when data arrives. The loop needs something to hold onto: a value representing "this handler started, it paused, it will finish later."

That value is a `Future`.

Think of it like a ticket at a deli counter: you hand in your order and get a small numbered slip back. The food is not ready yet — the ticket just represents "food that will exist at some point." You can come back and check, or do other things while you wait. The event loop is the person checking the tickets. When one is ready, it gets handled; the others wait.

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

Think of a pizza restaurant with a buzzer system. You order at the counter, the cashier hands you a small plastic buzzer, and you go sit wherever you like. The kitchen watches the oven. When your pizza is ready, the kitchen triggers your buzzer. You get up and collect your order.

Notice what the restaurant did *not* do: they did not follow you around watching you. They did not send someone to find your table. They handed you the buzzer — and the buzzer is the only thing they need to reach you, no matter where you are sitting.

The `Waker` is that buzzer. The future is you. The reactor is the kitchen.

There are three parties involved:

- **Runtime** — the cashier, drives futures by calling `poll()`
- **Future** — you, the state machine doing the work
- **Reactor** — the kitchen, watches OS events (epoll/kqueue) and fires wakers when ready

```
   Runtime              Future              Reactor / OS
      │                    │                     │
      │── poll(cx) ────────▶│                     │
      │                    │                     │
      │                    │─── register fd ────▶│  "tell me when socket 7 is ready"
      │                    │─── store waker ────▶│  future hands waker to reactor
      │                    │                     │
      │◀── Pending ─────────│                     │
      │                    │                     │
      │  (polls other       │                     │
      │   futures ...)      │                     │
      │                    │      data arrives   │
      │                    │                ◀────│
      │                    │                     │
      │◀──────────────────────── waker.wake() ───│  reactor fires the stored waker
      │                    │                     │
      │── poll(cx) ────────▶│                     │
      │                    │─── read data ──────▶│
      │                    │◀── bytes ───────────│
      │◀── Ready(data) ─────│                     │
```

### Why does the future store the waker?

Back to the restaurant: the kitchen cannot come find you — they do not know your table. The only way they can reach you is through the buzzer in your hand. So when you sit down, you keep hold of it.

The same logic applies here. By the time data arrives on the socket, `poll()` has already returned and the runtime has moved on to other work. The runtime has no memory of this particular future — it is just a struct on the heap somewhere.

The reactor is the only thing that will notice when the socket is ready. But the reactor does not know which future to reschedule. The future solves this by handing its waker to the reactor when it registers the socket:

```
future → reactor: "here is my waker. when socket 7 has data, call waker.wake()"
```

When the OS fires, the reactor calls `waker.wake()`, which puts the future back on the runtime's queue to be polled. Without the stored waker, the reactor would have no way to reach the runtime on behalf of this specific future — the buzzer would be sitting on the counter with no one holding it.

The `Context` passed into `poll()` carries the waker for the current task. The future clones it — `cx.waker().clone()` — because `poll()` only lends `cx` temporarily, but the reactor needs to hold onto the waker until the OS fires. One buzzer per table; the kitchen keeps the trigger, you keep the receiver.

No thread is parked between the `Pending` and the next `poll()`. The future is just a struct on the heap, waiting.

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
