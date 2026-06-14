# Chapter 2 — What the Operating System Does

Analogy: you do not drive to the airport by negotiating directly with traffic lights, fuel pumps, and runway controllers. You call a taxi, book a ticket, and the system handles the rest. The operating system is that system — it sits between your program and the hardware.

---

## The problem without an OS

Imagine you wrote a program that needed to read a file. Without an OS you would have to:

- Know the exact memory addresses of the disk controller chip
- Speak the controller's hardware protocol (send commands, wait for interrupts)
- Make sure no other program was talking to the disk at the same time
- Manage RAM yourself, making sure you did not overwrite another program

This was how early computers worked. Every program had to handle hardware directly. It was fragile, slow to write, and programs could not safely run alongside each other.

---

## The kernel

The **kernel** is the core of the operating system. It runs at a privileged level — it can do things normal programs cannot, like talking to hardware directly.

When your program wants to read a file, create a network connection, or allocate memory, it makes a **system call** — a request to the kernel to do it on its behalf.

```
┌──────────────────────────────────┐
│         your program             │  ← user space
│                                  │
│   open("data.txt")               │
│         │                        │
│         ▼  (system call)         │
├──────────────────────────────────┤  ← boundary
│            kernel                │  ← kernel space
│                                  │
│   talks to the disk controller   │
│   returns a file handle          │
└──────────────────────────────────┘
         │
    disk hardware
```

The boundary matters: normal programs cannot cross it accidentally. If your code tries to access hardware directly or touch memory that belongs to another program, the CPU raises an exception and the kernel terminates your process. This is what keeps one buggy program from corrupting another.

---

## What the kernel manages

**Memory**: gives each program its own region of RAM and prevents programs from reading each other's memory.

**Files**: maintains the filesystem, handles concurrent access, translates paths like `/home/user/data.txt` into disk locations.

**Devices**: provides a uniform interface to keyboards, screens, network cards, USB devices — you call `read()`, the kernel handles the rest.

**Processes**: creates, pauses, resumes, and terminates programs. Decides which program runs on which core and for how long.

**Network**: manages TCP connections, UDP sockets, routing — your program hands data to the kernel and the kernel handles getting it across the network.

---

## System calls from Rust

In Rust you rarely see system calls directly — the standard library wraps them:

```rust
use std::fs;
let contents = fs::read_to_string("data.txt").unwrap();
//                   ↑
//    this calls open() and read() system calls under the hood
```

`std::net::TcpStream::connect`, `std::thread::spawn`, `Vec::with_capacity` — all of these eventually make system calls. The kernel does the real work.

---

## User space vs kernel space

```
┌─────────────────────────────────────────┐
│              user space                  │
│                                         │
│   your program   other programs         │
│                                         │
│   (cannot touch hardware directly)      │
├─────────────────────────────────────────┤
│              kernel space                │
│                                         │
│   scheduler   memory manager            │
│   filesystem  network stack             │
│                                         │
│   (can do anything)                     │
├─────────────────────────────────────────┤
│              hardware                    │
│   CPU   RAM   disk   network card       │
└─────────────────────────────────────────┘
```

Everything in this book's networking and threading chapters goes through this boundary. `UdpSocket::send_to`, `TcpListener::accept`, `thread::spawn` — all of them ask the kernel to do something on your behalf.

---

## Key ideas

| Concept | What it is |
|---------|-----------|
| Kernel | The privileged core of the OS — manages hardware on behalf of programs |
| System call | A program's request to the kernel (read file, allocate memory, etc.) |
| User space | Where your program runs — restricted, safe |
| Kernel space | Where the OS runs — has full hardware access |
