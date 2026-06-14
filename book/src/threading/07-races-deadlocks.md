# Chapter 7 — The Problems: Races and Deadlocks

Threads are powerful but introduce two classes of bugs that do not exist in single-threaded programs. Both are notoriously hard to reproduce because they depend on the exact timing of the OS scheduler — and schedulers are not predictable.

---

## The bank account problem

Consider two threads both transferring money from the same account:

```rust
// Thread A: withdraw 100
let balance = account.balance;   // reads 1000
let new_balance = balance - 100; // computes 900
account.balance = new_balance;   // writes 900

// Thread B: withdraw 100
let balance = account.balance;   // reads 1000
let new_balance = balance - 100; // computes 900
account.balance = new_balance;   // writes 900
```

If these run sequentially, the final balance is 800 — correct. But if they interleave:

```
Thread A: let balance = account.balance;   // reads 1000
Thread B: let balance = account.balance;   // reads 1000  ← same value!
Thread A: account.balance = 1000 - 100;   // writes 900
Thread B: account.balance = 1000 - 100;   // writes 900  ← overwrites A!

final balance: 900  ← wrong, should be 800
```

Both threads read the same starting value, both compute the same result, and one write disappears. $100 was never deducted.

This is a **data race**: two threads access the same memory, at least one is writing, and there is no coordination between them.

---

## Why this is so hard to debug

The bug only appears when Thread A is interrupted *between its read and its write*. Whether that happens depends entirely on the OS scheduler — which changes behaviour based on system load, other processes, timer interrupts, and factors outside your control.

On a quiet machine during a test run, Thread A might always complete before Thread B even starts — the bug is invisible. Under production load, with hundreds of requests per second, the interleaving occurs and money vanishes. The test suite passes; production is broken.

---

## Data race vs race condition

A **data race** is a specific technical term: two threads access the same memory location simultaneously, at least one is writing, and there is no synchronisation. Data races are undefined behaviour in C and C++ — the CPU is allowed to produce any result whatsoever.

A **race condition** is broader: any bug where the outcome depends on the order of thread operations. A race condition can exist even without a data race (for example, checking a file for existence and then creating it — another process could create it in between).

Rust's type system prevents data races completely. It cannot prevent all race conditions, but eliminating data races removes the most dangerous class.

---

## Deadlock

A **deadlock** occurs when two or more threads are each waiting for something the other holds.

Classic example with two locks:

```
Thread A:               Thread B:
lock(lock_1)            lock(lock_2)
...                     ...
lock(lock_2) ← waits   lock(lock_1) ← waits
```

Thread A holds lock 1 and waits for lock 2. Thread B holds lock 2 and waits for lock 1. Neither can proceed. Both wait forever — the program hangs.

```
Thread A ──holds──▶ lock 1
Thread A ──wants──▶ lock 2
                       │
Thread B ──holds──▶ lock 2
Thread B ──wants──▶ lock 1
                       │
         (cycle → both blocked forever)
```

Deadlocks do not cause crashes — the program simply stops making progress. They can be very hard to spot because the threads are not *doing* anything wrong — they are just waiting.

---

## Starvation

A third class of problem, less severe: **starvation**. A thread that is perpetually denied resources (CPU time, access to a lock, data from a channel) because other threads always get priority. The thread is runnable but never actually runs.

---

## What Rust prevents

Rust's ownership and type system prevents **data races** at compile time — you cannot compile code that has a data race. The mechanism for this (the `Send` and `Sync` traits, the borrow checker applied to cross-thread shared references) is what makes Rust unique in the systems language space.

Rust does **not** prevent:
- **Deadlocks** — these are a logic error that the compiler cannot detect
- **Race conditions** that do not involve data races
- **Starvation**

These still require careful design and testing. But eliminating data races removes the most common, most destructive, and hardest-to-reproduce class of threading bug.

---

## A note on the next chapters

The practical chapters do not use locks at all — they use **message passing** via channels. When data is sent through a channel, ownership moves with it: at any point, exactly one thread owns the data, so there is no sharing and no races. This is the cleanest threading model and the right place to start.

Locks (`Mutex`) are a separate tool for when you genuinely need multiple threads to share and update the same value. That comes later.

---

## Key ideas

| Problem | What it is | Rust prevents it? |
|---------|-----------|------------------|
| Data race | Two threads read/write the same memory without coordination | Yes — compile error |
| Race condition | Outcome depends on thread ordering | Partially |
| Deadlock | Two threads each wait for what the other holds | No |
| Starvation | A thread is never given its turn | No |
