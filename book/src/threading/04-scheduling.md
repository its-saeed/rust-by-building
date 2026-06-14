# Chapter 4 — Scheduling: The Illusion of Simultaneity

Analogy: a radio DJ handles listener calls, plays songs, reads the news, and runs commercials — apparently all at once. In reality they rapidly switch between tasks. The OS scheduler does the same thing with your programs.

---

## The single-core problem

A CPU core executes one instruction at a time. If you have 200 processes running on a laptop with 8 cores, most processes cannot be running at any given instant — yet the system feels responsive. How?

The answer is **time-slicing**: the OS lets each process run for a short burst (a *time-slice*, typically 1–10 ms), then pauses it and switches to another. The switching happens so fast that everything appears to run simultaneously.

---

## Context switching

When the OS pauses a process and runs another, it performs a **context switch**:

1. Save the current process's CPU state — all registers, the program counter, everything — to RAM
2. Load the saved state of the next process
3. Set the program counter to where that process left off
4. Resume executing

```
time ────────────────────────────────────────▶

process A  ██████░░░░░░████░░░░░░██████░░░░░░████
process B  ░░░░░░█████░░░░░████░░░░░░░░█████░░░░░

           ↑     ↑
       A runs   switch to B
```

From the process's perspective nothing happened. Its registers, program counter, and memory are exactly as it left them. It simply did not notice the pause.

---

## The scheduler

The **scheduler** is the part of the kernel that decides which process runs next. It weighs many factors:

- **Priority**: higher-priority processes (system tasks, real-time workloads) run more often
- **Fairness**: no process should starve — every process gets some CPU time
- **I/O state**: a process waiting for a disk read or network packet is not given CPU time — it is marked *blocked* and woken up when the data arrives

Blocked processes do not waste CPU time. This is important: when your program calls `socket.recv_from()` on a *blocking* socket and no data is ready, the kernel marks your process as blocked and runs something else. When data arrives, the kernel marks it runnable again.

---

## Multi-core scheduling

With N cores, N processes can run truly in parallel — at the same physical instant.

```
         core 0      core 1      core 2      core 3
time ─────────────────────────────────────────────▶

         process A   process B   process C   process D
         ██████████  ██████████  ██████████  ██████████
         process E   process F
         ░░░████░░░  ░░░████░░░
```

The scheduler assigns work to cores. If you have 8 cores and 200 processes, it multiplexes 200 processes across 8 slots using time-slicing on each core.

---

## Why this matters for programs

**Blocking is free**: if your program waits for a file or network packet, it uses no CPU. The kernel suspends it and runs something else. This is efficient.

**But**: if you have only one thread and it blocks, your *entire program* stops. The game loop freezes. The UI hangs. The only way around this — with threads — is to let other threads keep running while one is blocked.

**Time-slices are not predictable**: the scheduler can pause your thread at any point between any two instructions. This is harmless for a single-threaded program but becomes dangerous when two threads share data — which is the subject of chapter 7.

---

## Voluntary vs preemptive scheduling

**Preemptive** scheduling (what all modern OSes do): the kernel can pause a thread at any moment, without asking. Your thread does not need to cooperate.

**Cooperative** scheduling (older OSes, async runtimes): a thread runs until it voluntarily yields. Every `await` in an async Rust program is a yield point — the async runtime is a cooperative scheduler inside your process.

The distinction matters when reading about `async`/`await` later. For threads, always assume preemptive: the OS can interrupt you at any time.

---

## Key ideas

| Concept | What it is |
|---------|-----------|
| Time-slice | A short burst of CPU time given to one process |
| Context switch | Saving one process's CPU state and restoring another's |
| Scheduler | The kernel component that decides what runs next |
| Blocked | A process waiting for I/O — not given CPU time until data arrives |
| Preemptive | The OS can pause any thread at any moment |
