// Exercise 2.3 — you cannot do arithmetic directly on a char
//
// This does not compile.
// In Rust, char is not a number — you must cast it first.
// Fix the expression so it compiles and the test passes.

fn main() {
    let letter = 'A';
    let shifted = letter + 3;        // broken
    println!("{}", shifted as char); // expected: D
}

#[cfg(test)]
mod tests {
    #[test]
    fn a_shifts_to_d() {
        let letter = 'A';
        let shifted = letter + 3;    // same fix needed here
        assert_eq!(shifted as char, 'D');
    }
}
