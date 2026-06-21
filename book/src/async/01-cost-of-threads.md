# Chapter 1 — The Cost of Threads at Scale

Your threaded chat server from the last project works — but "works" and "scales" are different things.

---

## What we built

The chat server spawns one OS thread per connected client:

```rust
loop {
    let (stream, _) = listener.accept()?;
    thread::spawn(move || {
        handle_client(stream);
    });
}
```

This is the **one thread per connection** pattern. It is simple, correct, and performs well at small scale. At 10 users it is perfectly fine. At 10,000 users it collapses.

---

## Memory: the stack problem

Every OS thread needs its own stack. On Linux and macOS, Rust threads start with a default stack of **2 MB** (the OS may grow it, but 2 MB is reserved up front).

The math is straightforward:

```
  10 users        ×  2 MB  =     20 MB   ← fine
 100 users        ×  2 MB  =    200 MB   ← fine
1,000 users       ×  2 MB  =      2 GB   ← uncomfortable
10,000 users      ×  2 MB  =     20 GB   ← exceeds most server RAM
100,000 users     ×  2 MB  =    200 GB   ← impossible
```

Most of that stack is empty. A thread handling a chat client spends nearly all its time blocked on a `read()` call waiting for a message. The stack is allocated and idle.

You cannot fix this by making stacks smaller — there is a minimum size below which programs crash with stack overflows. The limit is physical: one thread per connection simply does not scale.

### Check it yourself

Rust's default stack size for spawned threads is documented, but you can confirm it experimentally. `thread::Builder` lets you set the stack size explicitly — so you can also read back what the default would be:

```rust
use std::thread;

fn main() {
    // Spawn with the default stack (2 MB)
    thread::spawn(|| {
        println!("default stack thread running");
    }).join().unwrap();

    // Spawn with an explicit small stack (64 KB) — overflows much sooner
    thread::Builder::new()
        .stack_size(64 * 1024)
        .spawn(|| {
            println!("tiny stack thread running");
        })
        .unwrap()
        .join()
        .unwrap();
}
```

To see what the OS reports for the current process's stack limit:

```sh
ulimit -s        # prints stack size in KB (typically 8192 = 8 MB on Linux/macOS)
```

Note the difference: `ulimit -s` shows the OS limit for the *main thread*. Rust sets spawned threads to **2 MB** regardless of that limit — it is hardcoded in the standard library (`std::thread::DEFAULT_STACK_SIZE = 2 * 1024 * 1024`).

---

## CPU: context switching cost

Memory is not the only problem. The OS scheduler must share CPU time across all threads — including threads that are blocked waiting for I/O, which is most of them.

When the scheduler switches from one thread to another, it must:

```
┌─────────────────────────────────────────────────────┐
│             context switch                          │
│                                                     │
│  1. save registers of the outgoing thread           │
│     (program counter, stack pointer, general regs)  │
│                                                     │
│  2. update the kernel's thread table                │
│                                                     │
│  3. load registers of the incoming thread           │
│                                                     │
│  4. invalidate CPU cache lines that the new thread  │
│     probably does not share with the old one        │
│                                                     │
│  cost: roughly 1–10 µs per switch                   │
└─────────────────────────────────────────────────────┘
```

With 10,000 threads, the math gets ugly fast. The OS gives each thread a time-slice (typically ~1 ms) before switching to the next. One full cycle through all 10,000 threads takes:

```
10,000 threads × 1 ms time-slice = 10 seconds per full cycle
10,000 switches × 10 µs switch cost = 100 ms of pure overhead per cycle
```

That is **100 ms out of every 10 seconds** — 1% of all CPU time — spent saving and restoring registers, not running your program. And that is the optimistic case where every thread actually has work to do. When most threads are blocked on `read()`, the scheduler still has to visit them to discover they have nothing to do.

---

## The C10K problem

In 1999, web engineer Dan Kegel published a paper asking why web servers could not handle 10,000 simultaneous connections — the "C10K problem." The hardware was fast enough in theory; the software model was the bottleneck. Servers using one thread (or one process) per connection hit exactly the limits described above: memory exhaustion and context-switching overhead.

The C10K paper motivated the design of `epoll` on Linux and similar APIs on other operating systems. These became the foundation for every modern high-performance server. Async I/O in Rust, Node.js, Go's goroutines, and nginx's worker model all trace their lineage to this problem.

---

## What the threads are actually doing

Here is what 10,000 chat server threads look like at any given moment:

```
┌────────────────────────────────────────────────────────────┐
│  server with 10,000 threads — snapshot at one instant      │
│                                                            │
│  thread   0001  [blocked — waiting for read()]  ░░░░░░░░  │
│  thread   0002  [blocked — waiting for read()]  ░░░░░░░░  │
│  thread   0003  [blocked — waiting for read()]  ░░░░░░░░  │
│  thread   0004  [RUNNING — processing message]  ████████  │
│  thread   0005  [blocked — waiting for read()]  ░░░░░░░░  │
│  thread   0006  [blocked — waiting for read()]  ░░░░░░░░  │
│  thread   0007  [blocked — waiting for read()]  ░░░░░░░░  │
│  thread   0008  [RUNNING — writing response ]   ████████  │
│  ...                                                       │
│  thread   9999  [blocked — waiting for read()]  ░░░░░░░░  │
│  thread  10000  [blocked — waiting for read()]  ░░░░░░░░  │
│                                                            │
│  ░ = thread exists, consumes stack RAM, costs scheduling   │
│      overhead — but is doing absolutely nothing right now  │
└────────────────────────────────────────────────────────────┘
```

At any instant, only a few clients are sending data. The rest are idle — connected but waiting. Yet every idle client owns a thread, a stack, and a slot in the scheduler's queue.

The server is paying the full cost of 10,000 threads to serve the two or three that have work to do.

---

## The waste made concrete

Think of it this way. If a chat client sends a message once every 10 seconds, its thread is doing useful work for maybe 1 millisecond out of every 10,000 milliseconds — that is **0.01% utilisation**. The other 99.99% of the time, the thread is sleeping on a `read()` call.

One thread per connection is a terrible match for I/O-bound workloads precisely because I/O is slow. Network round-trips take milliseconds; local disk reads take microseconds. In both cases the CPU can execute millions of instructions in the time a thread spends waiting. Parking a thread — and its stack — for that entire duration is wasteful.

---

## The insight

The bottleneck is not that the machine cannot handle 10,000 chat clients. The bottleneck is that 10,000 threads is the wrong tool.

At any given instant, only a handful of connections have data ready to process. We do not need 10,000 threads. We need a way to watch thousands of connections simultaneously and only do work when there is actually something to do.

The solution is to stop giving each connection its own thread.

---

## Key ideas

| Problem | Detail |
|---------|--------|
| Stack memory | Each thread needs ~2 MB; 10,000 threads = 20 GB |
| Context switching | Switching threads costs ~1–10 µs; with thousands of threads this dominates CPU time |
| Utilisation | I/O-bound threads are blocked and idle almost all the time |
| C10K problem | Thread-per-connection cannot scale to 10,000 concurrent clients |
| Root cause | We are paying for 10,000 threads to serve the few connections that have data right now |
