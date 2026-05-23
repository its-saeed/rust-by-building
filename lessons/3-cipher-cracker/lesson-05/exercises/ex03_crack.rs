// Exercise 4.3 — implement crack using CrackResult
//
// `crack` should:
//   1. Count the letters in `ciphertext` using `count_letters`
//   2. Find the most frequent letter using `top_letter`
//   3. Compute the key with `guess_key`
//   4. Return CrackResult::Success { key, plaintext }
//   5. If there are no letters, return CrackResult::TooFewLetters
//
// All tests must pass.

use std::collections::HashMap;

enum CrackResult {
    Success { key: u8, plaintext: String },
    TooFewLetters,
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
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cracks_longer_text() {
        let ciphertext = "dolfh zdv ehjlqqlqj wr jhw yhub wluhg ri vlwwlqj eb khu vlvwhu rq wkh edqn";
        match crack(ciphertext) {
            CrackResult::Success { key, ref plaintext } => {
                assert_eq!(key, 3);
                assert!(plaintext.contains("alice"), "expected 'alice', got: {}", plaintext);
            }
            CrackResult::TooFewLetters => panic!("expected success"),
        }
    }

    #[test]
    fn empty_input_is_too_few() {
        assert!(matches!(crack(""), CrackResult::TooFewLetters));
    }

    #[test]
    fn no_letters_is_too_few() {
        assert!(matches!(crack("123 !@#"), CrackResult::TooFewLetters));
    }
}

fn main() {
    let ciphertext = "dolfh zdv ehjlqqlqj wr jhw yhub wluhg ri vlwwlqj eb khu vlvwhu rq wkh edqn";
    match crack(ciphertext) {
        CrackResult::Success { key, plaintext } => {
            println!("Key {}: {}", key, plaintext);
        }
        CrackResult::TooFewLetters => {
            println!("Not enough letters to analyse.");
        }
    }
}
