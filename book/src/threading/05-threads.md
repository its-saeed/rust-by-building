# Chapter 5 — Threads: Multiple Stacks, One Heap

Analogy: a restaurant with one kitchen and multiple chefs. Each chef has their own workstation (their own stack — their own tools, their own current task) but they all reach into the same refrigerator for ingredients (the shared heap). They can work in parallel, but they can also get in each other's way.

---

## The limitation of processes

Processes are isolated — they cannot share memory. If you want two parts of a program to run in parallel and share data, processes force you to copy that data through the kernel (sockets, pipes, shared memory), which is slow and complex.

Threads solve this by staying *inside* one process.

---

## What a thread is

A **thread** is an independent execution path within a process. It has:

- Its own **stack** — its own function calls, its own local variables
- Its own **program counter** — its own position in the code
- Its own **registers** — its own current CPU state

What it *shares* with every other thread in the process:

- The **heap** — all `Vec`, `String`, `Arc`, `Box` allocations
- The **code segment** — the compiled instructions
- **Global and static variables**
- **Open file handles and sockets**

```
┌────────────────────────────────────────────────────────┐
│                        process                         │
│                                                        │
│   thread 1              thread 2              thread 3 │
│   ┌──────────┐          ┌──────────┐          ┌──────┐ │
│   │  stack   │          │  stack   │          │stack │ │
│   │  PC      │          │  PC      │          │ PC   │ │
│   │  regs    │          │  regs    │          │regs  │ │
│   └──────────┘          └──────────┘          └──────┘ │
│                                                        │
│   ┌────────────────────────────────────────────────┐   │
│   │                    heap                        │   │
│   │   shared Vecs, Strings, Arc-wrapped data       │   │
│   └────────────────────────────────────────────────┘   │
└────────────────────────────────────────────────────────┘
```

The OS schedules threads the same way it schedules processes: each gets a time-slice, runs on a core, and can be preempted at any moment.

---

## Threads vs processes

| | Process | Thread |
|---|---------|--------|
| Memory | Own private address space | Shares the process's heap |
| Isolation | Strong — one crash does not affect others | Weak — a bad write corrupts everyone |
| Communication | Through kernel (slow) | Direct memory access (fast) |
| Creation cost | High — copy address space | Low — just a new stack |
| Crash impact | Only that process dies | Can take down the whole process |

Threads are cheaper to create and faster to communicate between. The price is that isolation disappears — bugs in one thread can corrupt data seen by all threads.

---

## The main thread

Every process has at least one thread: the **main thread**, which starts at `fn main()`. When you call `thread::spawn`, you create an additional thread inside the same process.

```rust
use std::thread;

fn main() {
    // this is the main thread

    let handle = thread::spawn(|| {
        // this is a new thread — same process, new stack
        println!("hello from thread");
    });

    handle.join().unwrap(); // wait for it to finish
    println!("back in main");
}
```

Both threads share the same heap. If you allocate a `Vec` and pass a reference to the spawned thread, they are looking at the same memory.

---

## Thread lifetime

A thread runs until its closure returns. If the main thread finishes before a spawned thread, the process exits and that thread is killed — even if it was mid-operation. This is why `join()` exists: it blocks the calling thread until the target thread finishes.

```
main thread:   ───────────────────────────────── join ── exit
spawned thread:          ════════════════════════╯
```

Without `join()`:

```
main thread:   ───────── exit
spawned thread:          ════════  ← killed here
```

---

## Stack size

Each thread gets its own stack. By default Rust gives each spawned thread a 2 MB stack. On a machine with 8 GB of RAM, that means you could theoretically have thousands of threads — but in practice, the overhead of scheduling thousands of threads makes that number much smaller before performance degrades.

For most programs, tens or low hundreds of threads is the practical ceiling. If you need more concurrent tasks than that, async/await (a different mechanism) is the right tool — but threads are the right starting point for understanding concurrency.

---

## Key ideas

| Concept | What it is |
|---------|-----------|
| Thread | An independent execution path within a process — own stack, shared heap |
| Main thread | The thread that starts at `fn main()` |
| `thread::spawn` | Creates a new thread in the current process |
| `join()` | Waits for a thread to finish before continuing |
| Shared heap | All threads in a process see the same heap allocations |
