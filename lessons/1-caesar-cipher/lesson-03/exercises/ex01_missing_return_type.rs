// Exercise 3.1 — declare the return type
//
// This does not compile. The function is supposed to return a char
// but has no `-> char` in its signature.
// Add the return type so it compiles and the test passes.

fn shift_char(c: char, key: u8) {
    (c as u8 + key) as char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shifts_correctly() {
        assert_eq!(shift_char('A', 1), 'B');
        assert_eq!(shift_char('H', 3), 'K');
    }
}

fn main() {
    println!("{}", shift_char('H', 3));
}
