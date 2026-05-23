// Cipher Cracker — lesson 4 (Frequency Analysis)
//
// This project builds on the letter-counting from lesson 1.
// Add three functions and wire them together to crack the ciphertext.
//
//   fn guess_key(peak: char) -> u8
//     — computes the key assuming `peak` is encrypted 'e'
//     — formula: (peak as u8 + 26 - b'e') % 26
//
//   fn decrypt(text: &str, key: u8) -> String
//     — shifts every letter backward by `key` (non-letters pass through)
//     — use a for loop with a match on each character
//
//   fn crack(ciphertext: &str) -> CrackResult
//     — uses count_letters + top_letter to find the peak, then guess_key + decrypt
//     — returns CrackResult::Success { key, plaintext } on success
//     — returns CrackResult::TooFewLetters if no letters found
//
// Expected output:
//
//   === Cipher Cracker ===
//   Key 3: alice was beginning to get very tired...

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

// TODO: implement guess_key(peak: char) -> u8

// TODO: implement decrypt(text: &str, key: u8) -> String

// TODO: implement crack(ciphertext: &str) -> CrackResult

fn main() {
    let ciphertext = "dolfh zdv ehjlqqlqj wr jhw yhub wluhg ri vlwwlqj eb khu vlvwhu rq wkh edqn dqg ri kdylqj qrwklqj wr gr rqfh ru wzlfh vkh kdg shhshg lqwr wkh errn khu vlvwhu zdv uhdglqj exw lw kdg qr slfwxuhv ru frqyhuvdwlrqv lq lw dqg zkdw lv wkh xvh ri d errn wkrxjkw dolfh zlwkrxw slfwxuhv ru frqyhuvdwlrqv";

    println!("=== Cipher Cracker ===");

    // TODO: call crack(), then match on CrackResult to print the result
}
