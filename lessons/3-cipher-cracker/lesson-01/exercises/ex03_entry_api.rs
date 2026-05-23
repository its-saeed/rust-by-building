// Exercise 1.3 — the entry API needs a dereference
//
// `.or_insert(0)` returns `&mut u32` — a mutable reference to the value.
// To add 1 to the actual value, you must dereference it with `*`.
//
// Fix the counting loop so the tests pass.

use std::collections::HashMap;

fn count_chars(text: &str) -> HashMap<char, u32> {
    let mut freq: HashMap<char, u32> = HashMap::new();
    for ch in text.chars() {
        freq.entry(ch).or_insert(0) += 1;   // BUG: missing dereference
    }
    freq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_correctly() {
        let freq = count_chars("aabbc");
        assert_eq!(freq[&'a'], 2);
        assert_eq!(freq[&'b'], 2);
        assert_eq!(freq[&'c'], 1);
    }

    #[test]
    fn empty_string() {
        let freq = count_chars("");
        assert!(freq.is_empty());
    }
}

fn main() {
    let freq = count_chars("hello");
    for (ch, count) in &freq {
        println!("  '{}': {}", ch, count);
    }
}
