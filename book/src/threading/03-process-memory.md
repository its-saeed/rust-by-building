# Chapter 3 — How a Process Lives in Memory

Analogy: when you open a book to read it, you do not read straight from the shelf — you take it to your desk. The OS does the same thing: it loads your program from disk into RAM and sets up a private workspace for it. That workspace is a process.

---

## What a process is

A **process** is a running instance of a program. When you type `cargo run`, the OS:

1. Reads your compiled binary from disk
2. Allocates a region of RAM for it
3. Copies the code into that RAM
4. Sets the program counter to the first instruction
5. Starts executing

Each process gets its own private memory. Process A cannot read or write process B's memory — the kernel enforces this. If it could, a bug in one program could corrupt every other program running on the machine.

---

## Memory layout

A process's memory is divided into regions:

```
high addresses
┌───────────────────────────┐
│          stack            │  ← grows downward
│    (function calls,       │
│     local variables)      │
│                           │
│                           │
│  ↓ grows down             │
│                           │
│  ↑ grows up               │
│                           │
│          heap             │  ← grows upward
│    (dynamic allocations:  │
│     Vec, String, Box)     │
├───────────────────────────┤
│     global / static       │  ← fixed at compile time
├───────────────────────────┤
│          code             │  ← your compiled instructions
└───────────────────────────┘
low addresses
```

---

## The stack

The **stack** is where function calls live. Every time you call a function, the OS pushes a **stack frame** containing:

- The function's local variables
- The return address (where to jump back when the function returns)
- Any arguments passed to it

When the function returns, its frame is popped off. The memory is reclaimed instantly.

```rust
fn add(a: i32, b: i32) -> i32 {
    let result = a + b;   // lives on the stack
    result
}                          // frame is gone after this
```

The stack is fast (just moving a pointer) and automatic (no cleanup needed). Its limitation: the size of each frame must be known at compile time, and the total stack size is limited (typically 8 MB).

---

## The heap

The **heap** is where data with a size unknown at compile time, or data that needs to outlive a function, gets allocated.

```rust
let v = Vec::new();         // the Vec metadata is on the stack
v.push(1);                  // but the actual element storage is on the heap
v.push(2);
// heap allocation grows as needed
```

Allocating on the heap is slower than the stack (the OS must find a free region), and the memory must be freed explicitly — or in Rust's case, when the owner goes out of scope.

`Vec`, `String`, `Box<T>`, `Arc<T>` — anything that stores a variable amount of data or needs to be shared — lives on the heap.

---

## Stack vs Heap at a glance

```
┌────────────────────────────────────────────────────────┐
│                         stack                          │
│                                                        │
│  main()          add()                                 │
│  ┌──────────┐    ┌────────────┐                        │
│  │ x: 5    │    │ a: 5       │                        │
│  │ v: ptr──┼───▶│ b: 3       │                        │
│  └──────────┘    │ result: 8  │                        │
│                  └────────────┘                        │
│                                   (grows ↓)            │
├────────────────────────────────────────────────────────┤
│                         heap                           │
│                                                        │
│  [ 1, 2, 3, 4 ]   ← Vec's element storage             │
│                                                        │
│                                   (grows ↑)            │
└────────────────────────────────────────────────────────┘
```

The `v` variable on the stack is just three words: a pointer to the heap data, a length, and a capacity. The actual integers live on the heap.

---

## Multiple processes

Because each process has its own memory space, they cannot accidentally interfere. But they also cannot easily share data. If two processes need to communicate, they must go through the kernel — via files, pipes, sockets, or shared memory that the kernel sets up explicitly.

This isolation is safe but expensive. Creating a new process involves copying memory regions, setting up a new address space, and loading program code. It takes milliseconds.

Threads solve the communication cost — but give up the isolation. That is the subject of chapter 5.

---

## Key ideas

| Concept | What it is |
|---------|-----------|
| Process | A running program with its own private memory |
| Stack | Fast, automatic memory for function frames and local variables |
| Heap | Dynamic memory for data whose size is not known at compile time |
| Stack frame | The block of memory pushed when a function is called, popped on return |
