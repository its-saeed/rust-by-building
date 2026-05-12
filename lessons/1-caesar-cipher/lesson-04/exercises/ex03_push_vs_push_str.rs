// Exercise 4.3 — push() vs push_str()
//
// This does not compile.
// `.push()` appends a single char.
// `.push_str()` appends a &str.
// The code is using the wrong one in each case. Swap them.

fn main() {
    let mut result = String::new();

    let letter = 'K';
    let word = "hoor";

    result.push_str(letter);   // wrong method for a char
    result.push(word);         // wrong method for a &str

    println!("{}", result);    // expected: Khoor
}
