// Caesar Cipher — step 5 (final)
//
// Replace your shift_char from step 4 with the correct version that:
//   - wraps around (Z+3 = C, not ])
//   - handles uppercase and lowercase separately
//
// Then add a `decrypt` function and call both in main.
//
// Expected output:
//
//   === Caesar Cipher ===
//   Original  : Hello, World!
//   Key       : 3
//   Encrypted : Khoor, Zruog!
//   Decrypted : Hello, World!
//
// Instructions:
//   1. Rewrite shift_char using match with 'A'..='Z', 'a'..='z', and _ arms.
//      Each letter arm: subtract the base (b'A' or b'a'), add key, mod 26, add base back.
//   2. Keep encrypt as-is from step 4.
//   3. Write decrypt(text: &str, key: u8) -> String.
//      Hint: decrypt is encrypt with key (26 - key % 26).
//   4. Call both in main and print all four lines.

fn shift_char(c: char, key: u8) -> char {
    // TODO: replace this with the correct match-based implementation
    if !c.is_alphabetic() {
        return c;
    }
    (c as u8 + key) as char   // broken — no wrap-around
}

fn encrypt(text: &str, key: u8) -> String {
    let mut result = String::new();
    for c in text.chars() {
        result.push(shift_char(c, key));
    }
    result
}

// TODO: add decrypt here

fn main() {
    let message = "Hello, World!";
    let key: u8 = 3;

    let encrypted = encrypt(message, key);
    // TODO: decrypt encrypted and store in `decrypted`

    println!("=== Caesar Cipher ===");
    println!("Original  : {}", message);
    println!("Key       : {}", key);
    println!("Encrypted : {}", encrypted);
    // TODO: println! for Decrypted
}
