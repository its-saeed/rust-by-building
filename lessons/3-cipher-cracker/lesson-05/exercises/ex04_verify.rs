// Exercise 4.4 — crack and verify a round-trip
//
// `encrypt_and_crack` should encrypt the plaintext with the given key,
// then crack the ciphertext. For a long enough text, it should return
// CrackResult::Success with the original plaintext.
//
// Implement `encrypt` (shift each letter forward by key) and use `crack`
// (provided). All tests must pass.

use std::collections::HashMap;

enum CrackResult {
    Success { key: u8, plaintext: String },
    TooFewLetters,
}

fn encrypt(text: &str, key: u8) -> String {
    todo!()
}

fn count_letters(text: &str) -> HashMap<char, u32> {
    let mut freq: HashMap<char, u32> = HashMap::new();
    for ch in text.chars() {
        if ch.is_alphabetic() {
            let ch = ch.to_ascii_lowercase();
            *freq.entry(ch).or_insert(0) += 1;
        }
    }
    freq
}

fn top_letter(freq: &HashMap<char, u32>) -> Option<char> {
    let mut best = ' ';
    let mut best_count = 0u32;
    for (&ch, &count) in freq {
        if count > best_count {
            best_count = count;
            best = ch;
        }
    }
    if best_count > 0 { Some(best) } else { None }
}

fn guess_key(peak: char) -> u8 {
    (peak.to_ascii_lowercase() as u8 + 26 - b'e') % 26
}

fn decrypt(text: &str, key: u8) -> String {
    let shift = (26 - key % 26) as u8;
    let mut result = String::new();
    for c in text.chars() {
        let ch = match c {
            'A'..='Z' => (b'A' + (c as u8 - b'A' + shift) % 26) as char,
            'a'..='z' => (b'a' + (c as u8 - b'a' + shift) % 26) as char,
            _ => c,
        };
        result.push(ch);
    }
    result
}

fn crack(ciphertext: &str) -> CrackResult {
    let freq = count_letters(ciphertext);
    match top_letter(&freq) {
        Some(peak) => CrackResult::Success {
            key: guess_key(peak),
            plaintext: decrypt(ciphertext, guess_key(peak)),
        },
        None => CrackResult::TooFewLetters,
    }
}

fn encrypt_and_crack(plaintext: &str, key: u8) -> CrackResult {
    let ciphertext = encrypt(plaintext, key);
    crack(&ciphertext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_shifts_letters() {
        assert_eq!(encrypt("Hello, World!", 3), "Khoor, Zruog!");
        assert_eq!(encrypt("abc", 1), "bcd");
        assert_eq!(encrypt("xyz", 3), "abc");
    }

    #[test]
    fn round_trip_long_text() {
        let text = "alice was beginning to get very tired of sitting by her sister on the bank";
        match encrypt_and_crack(text, 3) {
            CrackResult::Success { plaintext, .. } => assert_eq!(plaintext, text),
            CrackResult::TooFewLetters => panic!("expected success"),
        }
    }
}

fn main() {
    let plaintext = "alice was beginning to get very tired of sitting by her sister on the bank";
    println!("original : {}", plaintext);
    let ciphertext = encrypt(plaintext, 7);
    println!("encrypted: {}", ciphertext);
    match crack(&ciphertext) {
        CrackResult::Success { key, plaintext } => println!("Key {}: {}", key, plaintext),
        CrackResult::TooFewLetters => println!("Not enough letters."),
    }
}
