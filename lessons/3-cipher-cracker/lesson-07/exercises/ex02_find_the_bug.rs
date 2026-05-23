// Final Exercise 2 — find the bug
//
// This implementation compiles and runs, but the tests fail.
// There is exactly one bug. Find it and fix it.

use std::collections::HashMap;

fn letter_frequencies(text: &str) -> Vec<(char, u32)> {
    let mut freq: HashMap<char, u32> = HashMap::new();
    for ch in text.chars().filter(|c| c.is_alphabetic()) {
        *freq.entry(ch.to_ascii_lowercase()).or_insert(0) += 1;
    }
    let mut pairs: Vec<(char, u32)> = freq.into_iter().collect();
    pairs.sort_by(|a, b| a.1.cmp(&b.1));   // BUG: ascending instead of descending
    pairs
}

fn guess_key(peak: char) -> u8 {
    (peak.to_ascii_lowercase() as u8 + 26 - b'e') % 26
}

fn decrypt(text: &str, key: u8) -> String {
    let shift = 26 - key % 26;
    text.chars().map(|c| match c {
        'A'..='Z' => (b'A' + (c as u8 - b'A' + shift) % 26) as char,
        'a'..='z' => (b'a' + (c as u8 - b'a' + shift) % 26) as char,
        _ => c,
    }).collect()
}

fn crack(ciphertext: &str) -> String {
    let pairs = letter_frequencies(ciphertext);
    if pairs.is_empty() { return String::new(); }
    let key = guess_key(pairs[0].0);
    decrypt(ciphertext, key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frequencies_most_frequent_first() {
        let freq = letter_frequencies("aaabbc");
        assert_eq!(freq[0].1, 3, "first element should have count 3, got {}", freq[0].1);
        assert_eq!(freq[1].1, 2);
        assert_eq!(freq[2].1, 1);
    }

    #[test]
    fn crack_succeeds() {
        let ciphertext = "dolfh zdv ehjlqqlqj wr jhw yhub wluhg ri vlwwlqj eb khu vlvwhu rq wkh edqn";
        let result = crack(ciphertext);
        assert!(result.contains("alice"), "expected 'alice', got: {}", result);
    }
}

fn main() {
    let ciphertext = "dolfh zdv ehjlqqlqj wr jhw yhub wluhg ri vlwwlqj eb khu vlvwhu rq wkh edqn";
    println!("{}", crack(ciphertext));
}
