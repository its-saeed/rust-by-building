// Exercise 4.1 — let mut
//
// This does not compile.
// You cannot modify a variable that wasn't declared with `mut`.
// Add `mut` in the right place so the program compiles.
//
// Expected output: Hi

fn main() {
    let greeting = String::new();
    greeting.push('H');
    greeting.push('i');
    println!("{}", greeting);
}
