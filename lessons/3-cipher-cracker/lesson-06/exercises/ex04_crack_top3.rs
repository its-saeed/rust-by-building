// Exercise 5.4 — implement crack_top3
//
// `crack_top3` should return up to 3 candidate decryptions,
// each as (key, decrypted_text).
//
// Steps:
//   1. Count letter frequencies with `count_letters`
//   2. Find the top-3 most frequent letters with `top3_letters`
//   3. For each: compute the key with `guess_key`, decrypt the text
//   4. Return a Vec<(u8, String)>
//
// All tests must pass.

use std::collections::HashMap;

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

fn top3_letters(freq: &HashMap<char, u32>) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    for _ in 0..3 {
        let mut best = ' ';
        let mut best_count = 0u32;
        for (&ch, &count) in freq {
            if count > best_count && !result.contains(&ch) {
                best_count = count;
                best = ch;
            }
        }
        if best_count > 0 {
            result.push(best);
        }
    }
    result
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

fn crack_top3(ciphertext: &str) -> Vec<(u8, String)> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_guess_is_correct() {
        let ciphertext = "dolfh zdv ehjlqqlqj wr jhw yhub wluhg ri vlwwlqj eb khu vlvwhu rq wkh edqn";
        let guesses = crack_top3(ciphertext);
        assert!(!guesses.is_empty());
        let (key, ref text) = guesses[0];
        assert_eq!(key, 3);
        assert!(text.contains("alice"), "expected 'alice', got: {}", text);
    }

    #[test]
    fn returns_up_to_three() {
        let ciphertext = "dolfh zdv ehjlqqlqj wr jhw yhub wluhg ri vlwwlqj eb khu vlvwhu rq wkh edqn";
        let guesses = crack_top3(ciphertext);
        assert!(guesses.len() <= 3);
        assert!(!guesses.is_empty());
    }

    #[test]
    fn empty_input_returns_empty() {
        assert!(crack_top3("").is_empty());
    }
}

fn main() {
    let ciphertext = "dolfh zdv ehjlqqlqj wr jhw yhub wluhg ri vlwwlqj eb khu vlvwhu rq wkh edqn";
    let guesses = crack_top3(ciphertext);
    let mut i = 1;
    for (key, text) in &guesses {
        let mut preview = String::new();
        let mut count = 0;
        for ch in text.chars() {
            if count >= 40 { break; }
            preview.push(ch);
            count += 1;
        }
        println!("  {}. Key {:2}: {}...", i, key, preview);
        i += 1;
    }
}
