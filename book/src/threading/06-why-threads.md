# Chapter 6 — Why We Need Threads

Understanding *what* a thread is does not immediately explain *when* you need one. There are two distinct reasons to reach for threads, and confusing them leads to using the wrong tool.

---

## Reason 1 — Blocking I/O

Recall from chapter 4: when a process calls a blocking operation (reading a file, waiting for a network packet, sleeping), the kernel marks it as *blocked* and gives its CPU time to someone else.

This is efficient for the machine. But it is a problem for your program if there is more work to do.

**The single-threaded server problem:**

```rust
// this server can only handle one client at a time
loop {
    let (stream, _) = listener.accept()?;  // blocks until a client connects
    handle_client(stream);                  // blocks until the client is done
    // second client has to wait through all of this
}
```

While `handle_client` is running, `accept` is not called. A second client trying to connect gets no response until the first is finished. With 100 clients connecting simultaneously, the 100th waits for clients 1 through 99 to finish.

**The fix: spawn a thread per client:**

```rust
loop {
    let (stream, _) = listener.accept()?;
    thread::spawn(move || {
        handle_client(stream);  // runs on its own thread
    });
    // main thread immediately loops back to accept()
}
```

Now each client gets its own thread. The kernel schedules them independently — while one thread is blocked waiting for data from its client, others keep running. The program makes progress on all clients simultaneously.

This pattern — **one thread per connection** — is exactly what the chat server project in this section uses.

---

## Reason 2 — CPU-bound parallelism

Some work has no I/O at all — it is pure computation. Sorting a large dataset, rendering an image, simulating physics. This work does not block on anything; it just uses CPU.

On a single thread, this work runs on one core. On a 4-core machine, 3 cores sit idle. Threads let you split the work:

```rust
let data = vec![...very large vec...];
let half = data.len() / 2;

let left  = data[..half].to_vec();
let right = data[half..].to_vec();

let h1 = thread::spawn(move || process(left));
let h2 = thread::spawn(move || process(right));

let result_left  = h1.join().unwrap();
let result_right = h2.join().unwrap();
```

Both halves run on separate cores at the same time. Wall-clock time roughly halves (minus overhead). This is **parallelism** — not just concurrency.

---

## Concurrency vs Parallelism

These two words are often used interchangeably but mean different things:

**Concurrency** — multiple tasks are *in progress* at the same time, but they may not be running at the same physical instant. The OS switches between them rapidly. One core is enough.

**Parallelism** — multiple tasks are running at the same physical instant, on separate cores. Requires multiple cores.

```
concurrency (1 core):
time ──▶
core 0:  AAABBBAAABBBAAABBB
         switching rapidly — feels simultaneous

parallelism (2 cores):
time ──▶
core 0:  AAAAAAAAAAAA
core 1:  BBBBBBBBBBBB
         actually simultaneous
```

A single-threaded program with non-blocking I/O can be concurrent but not parallel. A multi-threaded program can be both.

Threads buy you both: the OS can switch between threads on one core (concurrency) and run threads on separate cores (parallelism).

---

## When threads are *not* the answer

Threads are not always the right choice:

- **Too many tasks**: if you need 10,000 concurrent connections, spawning 10,000 threads is impractical (2 MB stack each = 20 GB of virtual memory). Async/await with a small fixed thread pool handles this better.
- **Shared mutable state**: if many threads need to read and write the same data constantly, the coordination overhead can be worse than single-threaded. Sometimes a single-threaded event loop is faster.
- **Simple scripts**: the complexity of threading is not worth it for a 50-line script that runs for 200 ms.

The right mental model: reach for threads when you have a *small number* of long-running concurrent tasks (like one per connected client, or one per CPU core of heavy computation). For massive concurrency or reactive event handling, async is usually better — but understanding threads first makes async much easier to grasp.

---

## Key ideas

| Scenario | Why threads help |
|----------|-----------------|
| Server handling multiple clients | One thread per client — each blocks independently |
| Reading files while computing | I/O thread blocks; compute thread keeps running |
| CPU-bound computation | Spread across multiple cores |
| Waiting for multiple network sockets | Each waits independently without freezing the others |
