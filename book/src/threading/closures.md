# Closures

You have used closures throughout this book without a formal introduction — in `map`, `filter`, `retain_mut`, and now `thread::spawn`. Before going further with threads, it is worth understanding exactly what closures are and why the `move` keyword exists.

---

## What is a closure

A **closure** is an anonymous function that can capture variables from the scope where it is defined.

```rust
let greeting = String::from("hello");

let say_hi = |name: &str| {
    println!("{greeting}, {name}!");  // captures `greeting` from outside
};

say_hi("Alice");  // hello, Alice!
say_hi("Bob");    // hello, Bob!
```

`say_hi` is a closure. It looks like a function but it does not have a name and it can use `greeting` even though `greeting` was defined outside it. A regular function cannot do this — it can only use its own parameters and globals.

---

## Syntax

```rust
// with parameter types annotated (optional when the compiler can infer them)
let add = |a: i32, b: i32| -> i32 { a + b };

// types inferred, single expression (no braces needed)
let add = |a, b| a + b;

// no parameters
let greet = || println!("hello");

// multiple statements
let describe = |x: i32| {
    let doubled = x * 2;
    println!("{x} doubled is {doubled}");
};
```

The `|params|` is the parameter list. Everything after it is the body — either a single expression or a `{ block }`.

---

## How closures capture variables

When a closure uses a variable from the surrounding scope, it **captures** it. There are three ways this can happen, and Rust chooses the least restrictive one that works:

### 1. By shared reference (`&T`)

If the closure only reads the variable, it borrows it:

```rust
let text = String::from("hello");

let print_it = || println!("{text}");  // borrows text

print_it();
print_it();
println!("{text}");  // text is still available here
```

`text` is borrowed while the closure exists. The closure does not own it.

### 2. By mutable reference (`&mut T`)

If the closure modifies the variable, it borrows it mutably:

```rust
let mut count = 0;

let mut increment = || { count += 1; };  // mutably borrows count

increment();
increment();
println!("{count}");  // 2
```

No other code can access `count` while the closure holds the mutable borrow.

### 3. By move (`T`)

The `move` keyword forces the closure to take **ownership** of captured variables instead of borrowing them:

```rust
let text = String::from("hello");

let print_it = move || println!("{text}");  // takes ownership of text

// text is gone — moved into the closure
// println!("{text}");  ← would not compile
```

After a `move` closure is created, the original variable is gone. The closure owns the data entirely.

---

## Why `thread::spawn` requires `move`

`thread::spawn` requires a `move` closure. Here is why:

```rust
let name = String::from("Alice");

// this does NOT compile:
thread::spawn(|| println!("{name}"));
// error: closure may outlive the current function,
//        but it borrows `name`, which is owned by the current function
```

Without `move`, the closure would borrow `name` from the current function's stack frame. But the spawned thread might still be running after the current function returns — at which point the stack frame is gone and `name` no longer exists. The compiler refuses this.

With `move`, the closure takes ownership of `name`, so it lives as long as the closure does — as long as the thread runs:

```rust
let name = String::from("Alice");

thread::spawn(move || println!("{name}")).join().unwrap();
// compiles: the thread owns `name`
```

The rule: **if a spawned thread uses a variable from outside, that variable must be moved in.**

---

## Closures as arguments

Functions that accept closures use the `Fn` trait family as their parameter types:

| Trait | What it means |
|-------|--------------|
| `Fn` | Closure that can be called multiple times; captures by `&T` |
| `FnMut` | Closure that can be called multiple times; may mutate captures (`&mut T`) |
| `FnOnce` | Closure that can be called only once; takes ownership of captures |

You will rarely write these trait bounds yourself — the compiler infers them. But you will see them in error messages and in the signatures of standard library functions:

```rust
// thread::spawn's signature (simplified):
pub fn spawn<F: FnOnce() -> T + Send + 'static, T>(f: F) -> JoinHandle<T>
```

`FnOnce` — the closure is called once (the thread runs it and it is done).
`Send` — the closure can be sent to another thread (its captures are thread-safe).
`'static` — the closure does not borrow anything that might go away (hence `move`).

You do not need to memorise this. The compiler's error messages tell you exactly which constraint is violated when something goes wrong.

---

## Where closures appear in this project

| Location | Closure | What it captures |
|----------|---------|-----------------|
| `thread::spawn(move \|\| handle_client(stream))` | `move` — owns `stream` | `stream: TcpStream` |
| `thread::spawn(move \|\| { ... tx_reader ... })` | `move` — owns `tx_reader` | `tx_reader: Sender<Event>` |
| `writers.retain_mut(\|w\| writeln!(w, ...).is_ok())` | by `&mut` — borrows each writer | nothing from outer scope |

The pattern is consistent: anything passed to `thread::spawn` needs `move`; short closures passed to iterator methods capture by reference.
