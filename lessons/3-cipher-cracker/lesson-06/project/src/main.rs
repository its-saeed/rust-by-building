// Cipher Cracker — lesson 5 (User Input)
//
// Assemble the full interactive cracker using a Command loop.
//
// The program runs until the user types "quit":
//
//   === Cipher Cracker ===
//   Enter ciphertext to crack, or 'quit' to exit.
//
//   > dolfh zdv ehjlqqlqj wr jhw yhub wluhg ri vlwwlqj eb khu vlvwhu
//   Top 3 guesses:
//     1. Key  3: alice was beginning to get very tired of sitting...
//     2. Key 29: xebth wzv beceggbgc kf cek obho kbizy yp pbeebgc...
//     3. Key 10: rogas nzm twceedce kf awk fwjm kbjwz fy ybeeyme...
//
//   > quit
//   Goodbye!
//
// TODO 1: Define `enum Command { Crack(String), Quit }`
//
// TODO 2: Implement `fn read_command() -> Command`
//   — print "> " prompt, read a line from stdin
//   — if trimmed input is "quit" or "q", return Command::Quit
//   — otherwise return Command::Crack with the trimmed input as a String
//
// TODO 3: Implement `fn top3_letters(freq: &HashMap<char, u32>) -> Vec<char>`
//   — 3 passes, each finding the highest-count letter not already in result
//
// TODO 4: Implement `fn crack_top3(ciphertext: &str) -> Vec<(u8, String)>`
//   — return up to 3 (key, decrypted) pairs
//
// TODO 5: Implement `fn preview(text: &str, max_chars: usize) -> String`
//   — return at most `max_chars` characters, appending "..." if truncated
//
// TODO 6: Wire everything together in `main` using a loop + match on Command

use std::collections::HashMap;
use std::io::Write;

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

fn main() {
    println!("=== Cipher Cracker ===");
    println!("Enter ciphertext to crack, or 'quit' to exit.");
    println!();

    // TODO: loop, read_command(), match on Command::Crack / Command::Quit
}
