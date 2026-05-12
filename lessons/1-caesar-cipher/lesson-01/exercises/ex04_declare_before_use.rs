// Exercise 1.4 — declare before you use
//
// Rust reads your code top to bottom. You must declare a variable
// before you can use it. Something is out of order here.
//
// Fix it so the program compiles and prints: Key: 7

fn main() {
    println!("Key: {}", shift);
    let shift = 7;
}
