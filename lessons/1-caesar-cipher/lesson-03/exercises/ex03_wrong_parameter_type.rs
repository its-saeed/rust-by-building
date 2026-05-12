// Exercise 3.3 — parameter types must match
//
// This does not compile.
// The function signature says `key` is an `i32`,
// but `u8` arithmetic is needed to cast back to `char`.
// Change the parameter type of `key` to `u8`.

fn shift_char(c: char, key: i32) -> char {
    (c as u8 + key) as char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shifts_correctly() {
        assert_eq!(shift_char('B', 2), 'D');
        assert_eq!(shift_char('Z', 0), 'Z');
    }
}

fn main() {
    println!("{}", shift_char('B', 2));
}
