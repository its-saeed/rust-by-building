// Exercise 2.2 — cast back to char
//
// This compiles, but the test fails.
// `shifted` is a u8 (a number). The test expects a char.
// Add one cast to fix it.

fn main() {
    let letter = 'H';
    let shifted = letter as u8 + 3;
    println!("Shifted: {}", shifted);   // prints 75, should print K
}

#[cfg(test)]
mod tests {
    #[test]
    fn h_shifts_to_k() {
        let letter = 'H';
        let shifted = letter as u8 + 3;
        assert_eq!(shifted as char, 'K');
    }
}
