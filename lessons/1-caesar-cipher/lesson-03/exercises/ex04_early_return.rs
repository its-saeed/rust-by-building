// Exercise 3.4 — early return for non-letters
//
// The function below should leave any non-alphabetic character unchanged
// (spaces, punctuation, digits). For letters, it shifts normally.
//
// Right now it shifts everything, including spaces and punctuation.
// Add an early return: if `c` is not alphabetic, return `c` immediately.
//
// Hint: `c.is_alphabetic()` returns true if c is a letter.

fn shift_char(c: char, key: u8) -> char {
    (c as u8 + key) as char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shifts_letters() {
        assert_eq!(shift_char('A', 3), 'D');
        assert_eq!(shift_char('H', 3), 'K');
    }

    #[test]
    fn leaves_non_letters_unchanged() {
        assert_eq!(shift_char(' ', 3), ' ');
        assert_eq!(shift_char('!', 3), '!');
        assert_eq!(shift_char('3', 3), '3');
    }
}

fn main() {
    println!("{}", shift_char('H', 3));
    println!("{}", shift_char(' ', 3));
}
