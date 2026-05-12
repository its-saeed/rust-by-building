// Exercise 3.2 — the semicolon problem
//
// This does not compile.
// The function is declared to return a char, but the last line
// has a semicolon — turning it into a statement that returns nothing.
// Remove the semicolon.

fn shift_char(c: char, key: u8) -> char {
    (c as u8 + key) as char;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shifts_correctly() {
        assert_eq!(shift_char('A', 3), 'D');
        assert_eq!(shift_char('M', 2), 'O');
    }
}

fn main() {
    println!("{}", shift_char('A', 3));
}
