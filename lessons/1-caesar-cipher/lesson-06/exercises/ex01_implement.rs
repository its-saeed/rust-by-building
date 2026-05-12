// Final Exercise 1 — implement the Caesar cipher from scratch
//
// The three functions below have empty bodies. Implement all of them.
// Do not look at previous lessons. Use the compiler and the tests.
//
// All tests must pass.

fn shift_char(c: char, key: u8) -> char {
    todo!()
}

fn encrypt(text: &str, key: u8) -> String {
    todo!()
}

fn decrypt(text: &str, key: u8) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_encryption() {
        assert_eq!(encrypt("Hello, World!", 3), "Khoor, Zruog!");
        assert_eq!(encrypt("Abc", 1), "Bcd");
        assert_eq!(encrypt("Xyz", 3), "Abc");
    }

    #[test]
    fn preserves_non_letters() {
        assert_eq!(encrypt("Hello, World!", 5), "Mjqqt, Btwqi!");
        assert_eq!(encrypt("a b c", 1), "b c d");
    }

    #[test]
    fn key_zero_is_identity() {
        assert_eq!(encrypt("Hello", 0), "Hello");
        assert_eq!(decrypt("Hello", 0), "Hello");
    }

    #[test]
    fn key_26_is_identity() {
        assert_eq!(encrypt("Hello, World!", 26), "Hello, World!");
    }

    #[test]
    fn round_trip() {
        let msg = "The Quick Brown Fox!";
        assert_eq!(decrypt(&encrypt(msg, 13), 13), msg);
        assert_eq!(decrypt(&encrypt(msg, 7),  7),  msg);
        assert_eq!(decrypt(&encrypt(msg, 25), 25), msg);
    }

    #[test]
    fn large_key_wraps() {
        assert_eq!(encrypt("Hello", 29), encrypt("Hello", 3));
    }
}

fn main() {
    let msg = "Hello, World!";
    let key = 3;
    let enc = encrypt(msg, key);
    let dec = decrypt(&enc, key);
    println!("Original  : {}", msg);
    println!("Encrypted : {}", enc);
    println!("Decrypted : {}", dec);
}
