// Exercise 4.4 — fix the encrypt function
//
// The encrypt function below has two bugs. Find and fix both.
// The tests will tell you when you're done.

fn shift_char(c: char, key: u8) -> char {
    if !c.is_alphabetic() {
        return c;
    }
    (c as u8 + key) as char
}

fn encrypt(text: &str, key: u8) -> String {
    let result = String::new();       // bug 1
    for c in text {                   // bug 2
        result.push(shift_char(c, key));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypts_letters() {
        assert_eq!(encrypt("Hello", 3), "Khoor");
    }

    #[test]
    fn leaves_spaces_unchanged() {
        assert_eq!(encrypt("Hi there", 1), "Ij uifsf");
    }
}

fn main() {
    println!("{}", encrypt("Hello, World!", 3));
}
