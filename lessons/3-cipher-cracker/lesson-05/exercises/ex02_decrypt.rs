// Exercise 4.2 — implement decrypt
//
// Decryption is the reverse of encryption.
// Instead of shifting forward by `key`, shift backward.
// The trick: shifting forward by `26 - key` is the same as shifting backward by `key`.
//
// Implement `decrypt` so all tests pass.
// Non-letter characters must pass through unchanged.
// Handle both uppercase and lowercase.

fn decrypt(text: &str, key: u8) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decrypt_basic() {
        assert_eq!(decrypt("Khoor", 3), "Hello");
    }

    #[test]
    fn decrypt_with_punctuation() {
        assert_eq!(decrypt("Khoor, Zruog!", 3), "Hello, World!");
    }

    #[test]
    fn decrypt_lowercase() {
        assert_eq!(decrypt("khoor", 3), "hello");
    }

    #[test]
    fn decrypt_key_zero_is_identity() {
        assert_eq!(decrypt("Hello", 0), "Hello");
    }

    #[test]
    fn decrypt_key_26_is_identity() {
        assert_eq!(decrypt("Hello", 26), "Hello");
    }

    #[test]
    fn decrypt_wraparound() {
        // 'A' + 3 = 'D', so decrypt('D', 3) = 'A'
        assert_eq!(decrypt("Def", 3), "Abc");
    }
}

fn main() {
    println!("{}", decrypt("Khoor, Zruog!", 3));
}
