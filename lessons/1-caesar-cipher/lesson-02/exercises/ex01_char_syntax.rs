// Exercise 2.1 — char uses single quotes
//
// This does not compile.
// "H" is a &str (a string). 'H' is a char.
// You cannot cast a &str to u8. Fix the literal.

fn main() {
    let letter = "H";
    let code = letter as u8;
    println!("ASCII code of H: {}", code);   // expected: 72
}
