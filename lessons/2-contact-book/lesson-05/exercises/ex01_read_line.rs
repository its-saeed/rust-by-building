// Exercise 5.1 — read_line requires a mutable buffer
//
// `read_line` appends characters into its argument.
// To do that, it needs `&mut String` — a mutable reference.
// This does not compile. Fix it.

fn append_greeting(buf: &mut String) {
    buf.push_str("hello");
}

fn main() {
    let buf = String::new();   // BUG: missing mut
    append_greeting(&mut buf);
    println!("{}", buf);
}

// No assertions — if it compiles and prints "hello", you've solved it.
// This is the same constraint that `read_line` has: it needs `&mut String`.
