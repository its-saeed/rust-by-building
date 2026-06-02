# Modules

> **Goal**: Split a growing codebase into separate files using Rust's module system.
>
> **Concepts**: `mod`, `pub`, `use`, file-per-module layout, visibility rules.

---

## Why modules

Every project so far has lived in a single `main.rs`. That works fine at a few hundred lines, but Peggle will have several distinct pieces — cannon, ball, pegs, bucket, physics world, score — and keeping them in one file makes it hard to find things and easy to create accidental dependencies between unrelated parts.

Rust's module system lets you split code across files while the compiler treats them as a single program. Each file becomes a **module**: a named scope with controlled visibility.

---

## Declaring a module

In `main.rs`, you declare that a module exists with `mod`:

```rust
mod cannon;
mod peg;
mod ball;
```

Each declaration tells the compiler: "look for a file called `cannon.rs`, `peg.rs`, and `ball.rs` in the same directory, and compile them as modules named `cannon`, `peg`, and `ball`."

The files must exist — the compiler will error if they do not.

---

## Using items from a module

By default, everything inside a module is private — only code inside the same module can see it. To make something accessible from outside, mark it `pub`:

```rust
// peg.rs
pub struct Peg {
    pub pos:  Vec2,
    pub kind: PegKind,
    pub hit:  bool,
}

pub enum PegKind {
    Blue,
    Orange,
}
```

Without `pub` on the struct, `main.rs` cannot name the type. Without `pub` on the fields, it cannot read or write them.

From `main.rs`, bring items into scope with `use`:

```rust
use crate::peg::{Peg, PegKind};
```

`crate` refers to the root of the current crate — in a binary, that is `main.rs`. The path `crate::peg::Peg` reads: "in this crate, in the `peg` module, the item `Peg`."

---

## `pub` on `impl` methods

`pub` on a struct does not automatically make its methods public. Each method needs its own `pub`:

```rust
// peg.rs
impl Peg {
    pub fn new(pos: Vec2, kind: PegKind) -> Self {
        Peg { pos, kind, hit: false }
    }

    pub fn draw(&self) { ... }

    fn flash_color(&self) -> Color { ... }  // private helper
}
```

Private methods are visible within the module but not from `main.rs`. This is how you hide implementation details.

---

## Nested modules with subdirectories

If you want to group several related modules under one name, create a subdirectory and put a `mod.rs` inside it:

```
src/
  main.rs
  entities/
    mod.rs     ← declares the sub-modules
    peg.rs
    ball.rs
    cannon.rs
```

`entities/mod.rs`:
```rust
pub mod peg;
pub mod ball;
pub mod cannon;
```

`main.rs`:
```rust
mod entities;
use crate::entities::peg::Peg;
```

For Peggle we will use a flat layout — all modules directly in `src/`. Subdirectories are useful when a project grows larger, but the flat approach is easier to navigate when learning.

---

## The module layout for this project

```
src/
  main.rs       — game loop, window config, asset loading
  state.rs      — GameState enum
  level.rs      — level layouts, peg starting positions
  peg.rs        — Peg struct, PegKind enum
  cannon.rs     — Cannon struct, aiming, trajectory preview
  ball.rs       — Ball struct (wraps a Rapier body handle)
  bucket.rs     — Bucket struct
  physics.rs    — PhysicsWorld wrapper around Rapier
  score.rs      — Score struct, Display impl
```

Each lesson introduces the relevant file. By the end of the project every file will be in place and `main.rs` will be a thin coordinator that calls into the modules.

---

## Quick reference

| What you write | What it means |
|----------------|---------------|
| `mod foo;` in `main.rs` | compile `src/foo.rs` as module `foo` |
| `pub struct Foo` | `Foo` is visible outside the module |
| `pub fn bar` | `bar` is visible outside the module |
| `use crate::foo::Foo;` | bring `Foo` into the current scope |
| `use crate::foo::*;` | bring everything public from `foo` into scope (use sparingly) |
