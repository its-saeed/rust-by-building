# Rust by Building

**[Read the book →](https://its-saeed.github.io/rust-by-building/)**

Learn Rust by writing programs that actually do things. Each project introduces new concepts through a concrete problem — no toy snippets, no isolated exercises disconnected from a goal.

---

## What you'll build

### Command-line tools

| Project | What you build | Concepts |
|---------|---------------|----------|
| 1 — Caesar Cipher | Encrypt and decrypt messages | Variables, `char`, ASCII arithmetic, functions, loops, control flow |
| 2 — Contact Book | Store and search contacts | Structs, `Vec`, `impl`, `Option`, user input |
| 3 — Cipher Cracker | Break a Caesar cipher with frequency analysis | `HashMap`, traits, iterators, enums |

### Graphics & physics

| Project | What you build | Concepts |
|---------|---------------|----------|
| 4 — A Ball Moves | A ball bouncing around a window | macroquad, `Vec2`, operator overloading, physics integration |
| 5 — Many Bodies | Many balls under gravity | Ownership, `World` struct, gravity, spawning |
| 6 — Balls Collide | Elastic collisions between balls | Overlap detection, velocity response, mass |
| 7 — Pong | A playable Pong game | Lifetimes, sprites, sound, game states |
| 8 — Peggle Nights | A Peggle-style physics game | Rapier physics engine, modules, aiming, trajectory preview |

### Networking

> **How Networks Work** — HTTP, TCP, UDP, sockets, the layered model

| Project | What you build | Concepts |
|---------|---------------|----------|
| 9 — Tele-Sketch | A shared drawing canvas over the network | Protocol design, TCP server, async I/O, real-time sync |
| Mini project | HTTP over raw TCP | Parsing HTTP by hand, `reqwest` |

### Concurrency & async

> **How Concurrency Works** — CPUs, scheduling, threads, races, deadlocks
>
> **How Async Works** — futures, event loops, `async`/`await`, the tokio runtime

| Project | What you build | Concepts |
|---------|---------------|----------|
| 10 — Mandelbrot | A parallel fractal renderer | Thread pools, work splitting, shared buffers |
| 11 — Chat Server | A multi-user terminal chat | `Arc<Mutex<T>>`, broadcast channels, message passing |
| 12 — Async Chat Server | Same chat server, async | tokio tasks, async channels, `select!` |
| 13 — Download Manager | Concurrent file downloads with progress | `join!`, racing mirrors, progress bars |

### AI

> **How AI Works** — LLMs, tokens, tool calling, the agent loop

| Project | What you build | Concepts |
|---------|---------------|----------|
| Mini project — AI Todo List | A plain-English todo list backed by an LLM | rig.rs, tool traits, `Arc<Mutex<T>>`, prompt engineering |

---

## How the course works

The book lives alongside exercises and starter projects. A small CLI called `rbb` drives the workflow:

```sh
rbb watch caesar-02    # run tests for lesson 2 in watch mode
rbb open caesar-02     # open the lesson in the book
rbb next               # jump to the next unfinished lesson
```

The course is designed to run on a single offline Linux server — all crates are vendored, nothing requires internet access during class.

- **Admins**: see [`docs/admins.md`](./docs/admins.md) for setup, student provisioning, and publishing lessons.
- **Students**: see [`docs/students.md`](./docs/students.md) for logging in and getting started.
