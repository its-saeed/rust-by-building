# Project 15 — Download Manager

> **What you'll build**: A command-line download manager that accepts a list of URLs, downloads them all concurrently, and saves each file to disk with a live progress bar.
>
> **Lessons**: 4 lessons.
>
> **Rust concepts covered**: `reqwest` streaming, `tokio::spawn`, `tokio::sync::Semaphore`, `tokio::select!`, `mpsc` channels for progress reporting, async I/O.

---

## What you are building

A program that reads a list of URLs from a file and downloads them all at the same time:

```sh
cargo run -- urls.txt
```

```
downloading 5 files...
[████████████████████] file1.zip   4.2 MB  done  1.2s
[████████████░░░░░░░░] file2.tar   8.1 MB   52%  ...
[███░░░░░░░░░░░░░░░░░] file3.pdf   1.1 MB   18%  ...
[████████████████████] file4.png  82.4 KB  done  0.3s
[█████████████████░░░] file5.mp3   6.7 MB   87%  ...

5 files, 20.5 MB, completed in 4.3s
```

Each row shows a live progress bar that updates as bytes arrive. Completed downloads show their time. The program exits once every download has finished or failed.

---

## Why downloads are the perfect async problem

Downloads are **I/O-bound**: the program spends almost all its time waiting for bytes to arrive over the network. The CPU is idle. A thread doing a download is mostly sleeping.

With threads this is wasteful — each waiting thread still occupies stack memory and an OS scheduling slot. With async, a waiting task costs almost nothing; the runtime runs other tasks on the same thread while yours waits.

More concretely:

```
sequential downloads:
  file1 [────────── 2.1s ──────────]
  file2                              [────── 1.4s ──────]
  file3                                                   [─── 0.9s ───]
  total: 4.4s

concurrent downloads:
  file1 [────────── 2.1s ──────────]
  file2 [────── 1.4s ──────]
  file3 [─── 0.9s ───]
  total: 2.1s  ← limited only by the slowest
```

That is the core win. You will measure this difference in Lesson 2.

---

## Why this is also the perfect project for channels

Progress bars require a specific pattern: each download task knows how many bytes it has received, but only one task should be printing to the terminal at a time (otherwise the output would be garbled).

The solution is async channels:

```
download task 1  ──► Progress::Chunk { bytes: 4096 } ──╮
download task 2  ──► Progress::Done  { url: "..." }  ──┤──► display task ──► terminal
download task 3  ──► Progress::Chunk { bytes: 8192 } ──╯
```

Each download sends events. One display task receives them all and updates the screen. This is the producer/consumer pattern — and it shows up in nearly every real async application.

---

## How this builds on what you already know

You have already learned:

- `tokio::spawn` and `JoinHandle` (Async in Rust, Lesson 2)
- Async I/O with `tokio::fs` (Async in Rust, Lesson 3)
- `mpsc` channels (Async in Rust, Lesson 4)
- `tokio::join!` and `tokio::select!` (Async in Rust, Lesson 5)

This project uses all of them together in a realistic program. Nothing here is conceptually new — this is the capstone where the pieces combine.

---

## What is new: `reqwest`

You have used `reqwest` in the networking section for simple HTTP requests. Here you use its **streaming** mode: instead of reading the entire response into memory, you process it as a stream of chunks as they arrive. This is essential for large files.

```rust
let mut stream = response.bytes_stream();
while let Some(chunk) = stream.next().await {
    // process chunk as it arrives
}
```

---

## What you will build, lesson by lesson

| Lesson | What gets added |
|--------|-----------------|
| 1 — Single File | Download one URL, stream chunks to disk, handle errors |
| 2 — Concurrent Downloads | Spawn one task per URL, limit concurrency with a Semaphore |
| 3 — Racing Mirrors | `select!` to take the fastest of several mirror URLs |
| 4 — Progress Bars | `mpsc` channels to report progress; live display task |

Start with Lesson 1.
