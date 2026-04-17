// Exercise 3.4 — unit return
//
// `announce` below has no return arrow (`->`). What type does it return?
// It should NOT compile as written — something's wrong. Figure out what
// and fix it. The fix is one character.

fn announce(name: &str) {
    println!("Announcing: {name}")
}

fn main() {
    let _ = announce("World");
}

// There are no assertions here — the goal is to get it to compile.
// If `cargo build` succeeds on this file, you've solved it.
