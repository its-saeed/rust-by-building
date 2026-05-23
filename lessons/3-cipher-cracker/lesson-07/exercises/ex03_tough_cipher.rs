// Final Exercise 3 — crack a tough cipher with brute force
//
// The ciphertext below was written with an unusual letter distribution.
// The most frequent letter in the ciphertext is NOT the encrypted 'e'.
// Simple frequency analysis gets the wrong answer.
//
// Implement `crack_best_of_26` which tries all 26 possible keys and
// returns the decryption whose score is highest.
//
// A scorer is provided: `score(text)` counts how many of the 100 most
// common English words appear in the text. The correct decryption will
// score much higher than the others.

use std::collections::HashMap;

fn decrypt(text: &str, key: u8) -> String {
    let shift = 26 - key % 26;
    text.chars().map(|c| match c {
        'A'..='Z' => (b'A' + (c as u8 - b'A' + shift) % 26) as char,
        'a'..='z' => (b'a' + (c as u8 - b'a' + shift) % 26) as char,
        _ => c,
    }).collect()
}

fn score(text: &str) -> usize {
    // Common English words — if many of these appear, the text is likely English
    const COMMON: &[&str] = &[
        "the", "be", "to", "of", "and", "a", "in", "that", "have", "it",
        "for", "not", "on", "with", "he", "as", "you", "do", "at", "this",
        "but", "his", "by", "from", "they", "we", "say", "her", "she", "or",
        "an", "will", "my", "one", "all", "would", "there", "their", "what",
        "so", "up", "out", "if", "about", "who", "get", "which", "go", "me",
    ];
    let lower = text.to_ascii_lowercase();
    COMMON.iter().filter(|&&word| lower.contains(word)).count()
}

fn crack_best_of_26(ciphertext: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // This text has unusual statistics — 'z', 'x', 'q' dominate the ciphertext
    // because the plaintext uses many of those letters' shifted equivalents.
    const TOUGH: &str = "zhofrph wr uxvw eb exloglqj, d kdqgv-rq frxuvh iru ohduqlqj uxvw wkurxjk surmhfwv";
    // plaintext: "welcome to rust by building, a hands-on course for learning rust through projects"
    // key: 3

    #[test]
    fn cracks_tough_cipher() {
        let result = crack_best_of_26(TOUGH);
        assert!(
            result.contains("welcome") || result.contains("rust") || result.contains("learning"),
            "expected English text, got: {}",
            result
        );
    }

    #[test]
    fn score_english_higher_than_gibberish() {
        assert!(score("the cat sat on the mat") > score("xkq wzo mno bv xkq pzo"));
    }
}

fn main() {
    let ciphertext = "zhofrph wr uxvw eb exloglqj, d kdqgv-rq frxuvh iru ohduqlqj uxvw wkurxjk surmhfwv";
    println!("{}", crack_best_of_26(ciphertext));
}
