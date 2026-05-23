// Exercise 1 — implement Display
//
// `CipherStat` holds information about a ciphertext analysis.
// Implement the `Display` trait so that `println!("{}", stat)` prints:
//
//   Letters: 42, Unique: 18
//
// The numbers should match the actual field values.
//
// No assertions — if it compiles and prints the right format, you're done.

use std::fmt;

struct CipherStat {
    letters: u32,
    unique: u32,
}

// TODO: implement fmt::Display for CipherStat

fn main() {
    let stat = CipherStat { letters: 42, unique: 18 };
    println!("{}", stat);   // Letters: 42, Unique: 18
}
