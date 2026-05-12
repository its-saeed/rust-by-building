// Exercise 5.4 — fix the wrap-around
//
// The shift_char below does not wrap around.
// Shifting 'Z' by 3 should give 'C', not ']'.
// Shifting 'z' by 3 should give 'c', not '}'.
//
// Fix both arms of the match using modulo and the base letter offset.
// Hint: position = c as u8 - b'A', then (position + key) % 26, then add b'A' back.

fn shift_char(c: char, key: u8) -> char {
    match c {
        'A'..='Z' => (c as u8 + key) as char,   // broken: no wrap-around
        'a'..='z' => (c as u8 + key) as char,   // broken: no wrap-around
        _ => c,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_uppercase() {
        assert_eq!(shift_char('Z', 3), 'C');
        assert_eq!(shift_char('X', 5), 'C');
        assert_eq!(shift_char('A', 0), 'A');
    }

    #[test]
    fn wraps_lowercase() {
        assert_eq!(shift_char('z', 3), 'c');
        assert_eq!(shift_char('a', 1), 'b');
    }

    #[test]
    fn leaves_non_letters_unchanged() {
        assert_eq!(shift_char(' ', 5), ' ');
        assert_eq!(shift_char('!', 5), '!');
    }
}

fn main() {
    println!("{}", shift_char('Z', 3));   // should print C
    println!("{}", shift_char('z', 3));   // should print c
}
