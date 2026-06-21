# Chapter 3 — The Event Loop

Non-blocking I/O gives us the building block; the event loop is the structure that turns it into a server.

---

## The pattern

An event loop is a single `loop` with three steps, repeated forever:

```
┌─────────────────────────────────────────────────┐
│                 event loop                      │
│                                                 │
│  ┌──────────────────────────────────────────┐   │
│  │  1. ask OS: which sockets are ready?     │   │
│  │     (block here until at least one is)   │   │
│  └─────────────────────┬────────────────────┘   │
│                        │                        │
│  ┌─────────────────────▼────────────────────┐   │
│  │  2. for each ready socket:               │   │
│  │       run its handler                    │   │
│  └─────────────────────┬────────────────────┘   │
│                        │                        │
│  ┌─────────────────────▼────────────────────┐   │
│  │  3. go back to step 1                    │   │
│  └──────────────────────────────────────────┘   │
└─────────────────────────────────────────────────┘
```

In pseudocode:

```
loop {
    events = epoll_wait(watched_fds)   // block until something is ready
    for event in events {
        handle(event)                  // run the handler for that fd
    }
}
```

That is the entire pattern. Every async runtime — Tokio in Rust, libuv in Node.js, asyncio in Python — is a more sophisticated version of this loop.

---

## You already know this pattern: JavaScript

If you have used JavaScript in a browser or Node.js, you have used an event loop. When you write:

```js
fetch("https://example.com/data")
  .then(response => response.json())
  .then(data => console.log(data));
```

The `fetch` call registers interest in a network socket and immediately returns. The JavaScript engine goes back to its event loop. When the OS signals that the socket has data, the engine calls the first `.then` callback.

`setTimeout(fn, 1000)` is the same thing: register a timer, return immediately, let the event loop call `fn` when the timer fires.

Node.js serves millions of connections using this exact mechanism on a single thread. There is no magic — it is one thread plus `epoll` plus the event loop pattern.

---

## The single thread handles everything, interleaved

Here is what the event loop looks like from the outside when handling multiple clients:

```
time ──▶

thread:  ┌────────────┐   ┌───────────────┐   ┌──────────┐   ┌──────────────┐
         │  epoll_wait│   │handle conn A  │   │handle B  │   │handle conn A │
         │  (blocking)│──▶│ (read message)│──▶│(write ok)│──▶│(write reply) │──▶ ...
         └────────────┘   └───────────────┘   └──────────┘   └──────────────┘
              ↑                                                       │
              └───────────────────────────────────────────────────────┘
                                next epoll_wait

connections A, B, C ... are all handled on this one thread's timeline,
interleaved — not in parallel, but fast enough to feel simultaneous
```

This is **concurrency without parallelism** on a single thread. A hundred connections make progress, but the thread handles them one at a time, switching between them as their sockets become ready. Recall from the threading chapter: concurrency and parallelism are different things. The event loop gives you concurrency; it does not give you parallelism on multiple cores.

---

## The critical rule: handlers must not block

The event loop only works if every handler finishes quickly and returns.

If a handler blocks — calls `thread::sleep`, performs a synchronous file read, or spins in a compute loop — the thread is stuck. The event loop cannot proceed to step 1. No other connection gets handled until the blocking handler returns. The whole server freezes.

```
GOOD — handler returns quickly:                BAD — handler blocks the loop:

events = epoll_wait()  ← gets A, B            events = epoll_wait()  ← gets A
handle(A)  ← reads socket, queues work        handle(A)  ← calls thread::sleep(10s)
handle(B)  ← reads socket, queues work        ...
events = epoll_wait()  ← back in 1 ms         ...
                                               ...  (10 seconds pass)
All connections served.                        events = epoll_wait()  ← finally
                                               B has been waiting 10 seconds.
```

This is the hardest discipline of event-loop programming: **every piece of code that runs on the event loop thread must be non-blocking**. You cannot call any function that might wait for I/O or sleep.

---

## What counts as blocking?

| Operation | Blocks? | Effect on event loop |
|-----------|---------|---------------------|
| `std::thread::sleep(Duration::from_secs(1))` | Yes | Stalls loop for 1 second |
| Reading a file with `std::fs::read()` | Yes | Stalls loop until disk responds |
| Calling `.recv()` on a channel with no message | Yes | Stalls loop until message arrives |
| A long CPU computation (e.g., sorting 10M items) | Yes (uses CPU) | Stalls loop until done |
| `TcpStream::read()` on a non-blocking socket | No | Returns immediately |
| Returning from a handler | No | Loop proceeds normally |

Even a computation that does not do I/O blocks the loop if it takes a long time. "Non-blocking" means "returns to the event loop quickly," not just "does not call `read()`."

---

## The limit: CPU-bound work

The event loop is well-matched to I/O-bound work — work that spends most of its time waiting for the network or disk. A web server, a chat server, a database client: these are I/O-bound. The handlers run briefly, the loop spends most of its time in `epoll_wait`, and a single thread can juggle thousands of connections.

CPU-bound work is different. Consider the Mandelbrot renderer from the mandelbrot project. That code has no I/O — it is pure arithmetic on pixels. It will not stall waiting for a network socket. It will burn a CPU core for seconds. If you run that inside an event loop handler, the loop stalls.

```
┌─────────────────────────────────────────────────────┐
│  I/O-bound work          │  CPU-bound work          │
│  (good fit for async)    │  (bad fit for async)     │
├──────────────────────────┼──────────────────────────┤
│  Chat server             │  Image rendering         │
│  HTTP requests           │  Physics simulation      │
│  Database queries        │  Sorting large datasets  │
│  File uploads/downloads  │  Cryptographic hashing   │
└──────────────────────────┴──────────────────────────┘
```

For CPU-bound work, threads remain the right tool. You want multiple cores running in parallel; the event loop cannot give you that. In practice, Tokio provides `spawn_blocking` — a way to hand CPU-heavy work to a thread pool so it does not block the event loop. But that is a detail for later.

---

## Writing handlers without blocking is painful

The event loop pattern solves the scalability problem, but it creates a new programming problem.

In the threaded chat server, a handler looked natural:

```rust
fn handle_client(stream: TcpStream) {
    let mut buf = [0u8; 1024];
    loop {
        let n = stream.read(&mut buf)?;   // block here — that is fine, we own this thread
        let msg = &buf[..n];
        broadcast(msg);
    }
}
```

You write code the way you think about it: read a message, do something with it, loop. The blocking `read()` is fine because the thread is dedicated to this connection.

In an event loop, you cannot write it that way. The `read()` must be non-blocking. But if `read()` returns `EAGAIN`, you need to stop, remember where you were, and let the loop continue. Next time the socket is ready, you pick up where you left off. You have to manually preserve state across handler invocations.

For simple handlers this is manageable. For handlers that do several things in sequence — read a request, look something up, write a response — the code becomes a state machine where each step is a separate callback or enum variant. The logic is spread across many small functions. It is hard to follow.

This is the problem that `async`/`await` solves. `async` functions let you write code that looks sequential and blocking, while the compiler rewrites it into a state machine that is compatible with the event loop. You write:

```rust
async fn handle_client(stream: TcpStream) {
    let msg = stream.read().await;   // yields to the event loop, not blocks the thread
    broadcast(msg);
}
```

The `.await` yields control back to the event loop when there is nothing to do, and resumes where it left off when the socket is ready. The sequential logic stays intact. The compiler generates the state machine.

The event loop is a great idea. The problem is that writing handlers without blocking is painful without language support. `async`/`await` is Rust's solution to that.

But there is one more piece to understand before we get to the syntax. The event loop needs a *type* that represents a handler mid-flight — paused at a wait point, not yet done, but not forgotten either. A value the runtime can hold onto and come back to when the socket is ready.

That type is called a **`Future`**. It is what the compiler generates from every `async fn`. The next chapter explains exactly what it is and how the poll model works.

---

## Key ideas

| Concept | What it means |
|---------|--------------|
| Event loop | A single loop: ask OS what is ready → handle each → repeat |
| Readiness | The OS signals when a socket can be read or written without blocking |
| Non-blocking handlers | Every handler must return quickly; blocking stalls the whole server |
| I/O-bound vs CPU-bound | Event loops excel at I/O-bound work; CPU-bound work still needs threads |
| State machine problem | Without language help, non-blocking handlers require manual state machines |
| `async`/`await` | Rust's language feature that generates those state machines automatically |
