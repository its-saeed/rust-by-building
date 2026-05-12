// Exercise 1.3 — Rust format strings use {}, not %s or %d
//
// This program does not compile.
// The format string uses C-style placeholders. Replace them with Rust ones.
//
// Expected output:
//   Message: Hello, World!
//   Key: 3

fn main() {
    let message = "Hello, World!";
    let key = 3;
    println!("Message: %s", message);
    println!("Key: %d", key);
}
