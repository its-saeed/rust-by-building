# Chapter 1 — The CPU and Memory

Analogy: imagine a chef working at a single counter. The chef can only do one thing at a time — chop, stir, taste — but they do each step so fast it *looks* continuous. A CPU works the same way.

---

## The CPU

A CPU (Central Processing Unit) is the part of the computer that actually runs your code. It is a chip that does one thing, over and over, billions of times per second:

```
┌─────────────────────────────────────┐
│  fetch    read the next instruction │
│     ↓     from memory               │
│  decode   figure out what it means  │
│     ↓                               │
│  execute  do it (add, compare, etc) │
│     ↓                               │
│  repeat                             │
└─────────────────────────────────────┘
```

Every line of your Rust program — every `+`, every `if`, every function call — eventually becomes a sequence of these simple instructions.

---

## Registers

Inside the CPU are a handful of tiny storage slots called **registers**. They hold the values the CPU is currently working with — the two numbers being added, the result, the address of the next instruction.

Registers are extremely fast (one CPU cycle to read or write) but there are very few of them — typically 16 to 32 general-purpose registers.

---

## RAM

When a program runs, its code and data live in **RAM** (Random Access Memory). RAM is much larger than registers — gigabytes — but also much slower (hundreds of CPU cycles to read from).

The CPU fetches each instruction from RAM, executes it (possibly using values also fetched from RAM), and writes results back. The gap in speed between the CPU and RAM is one of the central constraints of computing.

```
┌───────────┐        ┌──────────────────────────┐
│    CPU    │◀──────▶│           RAM             │
│           │        │                          │
│ registers │        │  your program's code     │
│  (tiny,   │        │  your program's data     │
│   fast)   │        │  (large, slower)         │
└───────────┘        └──────────────────────────┘
```

---

## The Program Counter

One register has a special job: the **program counter** (also called the instruction pointer). It holds the address in RAM of the *next* instruction to execute.

After each instruction, the program counter advances automatically. When your code calls a function, the program counter jumps to that function's first instruction. When the function returns, it jumps back.

This is all your program is, at the lowest level: a stream of instructions, a counter pointing to the current one, and some memory holding values.

---

## Cores

Modern CPUs have multiple **cores** — essentially multiple independent fetch-decode-execute units on the same chip, each with their own registers and program counter.

A 4-core CPU can run 4 instruction streams simultaneously — truly in parallel, at the same instant.

```
┌──────────────────────────────────────────┐
│                  CPU                     │
│  ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐ │
│  │ core 0 │ │ core 1 │ │ core 2 │ │ core 3 │ │
│  └────────┘ └────────┘ └────────┘ └────────┘ │
└──────────────────────────────────────────┘
                     │
              shared RAM
```

All cores share the same RAM. This is both what makes multi-core useful and, as we will see later, what makes concurrent programming tricky.

---

## Key ideas

| Concept | What it is |
|---------|-----------|
| CPU | Executes instructions one at a time, per core |
| Register | Tiny, fast storage inside the CPU |
| RAM | Large, slower storage holding code and data |
| Program counter | Points to the next instruction to run |
| Core | An independent execution unit; modern CPUs have several |
