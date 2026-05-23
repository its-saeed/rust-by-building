use std::fmt;
use std::collections::HashMap;

// --- Summary trait ---
//
// TODO 1: Define a `Summary` trait with one required method:
//   fn summarize(&self) -> String;

// --- CipherSummary struct ---

struct CipherSummary {
    total_letters: u32,
    unique_letters: u32,
    top_letter: Option<char>,
}

// TODO 2: Implement `fmt::Display` for `CipherSummary`.
//
// It should print like:
//   Letters: 42, Unique: 18, Top: 'h'
//
// When `top_letter` is None, print:
//   Letters: 0, Unique: 0, Top: (none)

// TODO 3: Implement `Summary` for `CipherSummary`.
//
// `summarize()` should return a String like:
//   "Analysed 42 letters (18 unique)"

// --- LetterIter ---

struct LetterIter {
    chars: Vec<char>,
    pos: usize,
}

impl LetterIter {
    fn new(text: &str) -> LetterIter {
        LetterIter {
            chars: text.chars().collect(),
            pos: 0,
        }
    }
}

// TODO 4: Implement `Iterator` for `LetterIter`.
//
// - `type Item = char`
// - `next` returns each alphabetic character as lowercase, skipping
//   non-alphabetic characters.
// - Returns `None` when all characters have been visited.

// --- Helper: build a CipherSummary from ciphertext ---

fn analyse(text: &str) -> CipherSummary {
    let mut freq: HashMap<char, u32> = HashMap::new();
    // TODO 5: Use LetterIter to iterate over the letters in `text`
    //   and count each character using the entry API.
    //   Hint: `for ch in LetterIter::new(text) { ... }`

    let total_letters = freq.values().sum();
    let unique_letters = freq.len() as u32;
    let top_letter = freq.iter().max_by_key(|(_, &v)| v).map(|(&k, _)| k);

    CipherSummary { total_letters, unique_letters, top_letter }
}

fn main() {
    let ciphertext = "dolfh zdv ehjlqqlqj wr jhw yhub wluhg ri vlwwlqj eb khu vlvwhu rq wkh edqn";

    let summary = analyse(ciphertext);

    // Uses Display — TODO 2
    println!("{}", summary);

    // Uses Summary trait — TODO 3
    // println!("{}", summary.summarize());
}
