// Exercise 4.2 — iterate over characters with .chars()
//
// This does not compile.
// You cannot iterate over a &str directly — you need to call .chars() on it first.
// Fix the for loop.

fn main() {
    let text = "Rust";
    for c in text {
        println!("{}", c);
    }
}

// Expected output:
// R
// u
// s
// t
