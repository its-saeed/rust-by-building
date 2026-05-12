// Final Exercise 2 — find the bug
//
// This is a complete Caesar cipher implementation — but something is wrong.
// The tests fail. Find the single bug and fix it.
//
// Read the test output carefully: it shows you which inputs produce wrong results.
// Think about what the numbers mean before changing anything.

fn shift_char(c: char, key: u8) -> char {
    match c {
        'A'..='Z' => (b'A' + (c as u8 - b'A' + key) % 25) as char,
        'a'..='z' => (b'a' + (c as u8 - b'a' + key) % 26) as char,
        _ => c,
    }
}

fn encrypt(text: &str, key: u8) -> String {
    let mut result = String::new();
    for c in text.chars() {
        result.push(shift_char(c, key));
    }
    result
}

fn decrypt(text: &str, key: u8) -> String {
    encrypt(text, 26 - key % 26)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_encryption() {
        assert_eq!(encrypt("Hello, World!", 3), "Khoor, Zruog!");
    }

    #[test]
    fn uppercase_wraparound() {
        assert_eq!(encrypt("Xyz", 3), "Abc");
        assert_eq!(encrypt("Z", 1), "A");
    }

    #[test]
    fn round_trip() {
        let msg = "Attack at Dawn!";
        assert_eq!(decrypt(&encrypt(msg, 11), 11), msg);
    }
}

fn main() {
    println!("{}", encrypt("Hello, World!", 3));
}
