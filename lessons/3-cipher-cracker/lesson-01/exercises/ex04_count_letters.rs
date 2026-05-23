// Exercise 1.4 — implement count_letters
//
// Implement `count_letters` so it counts every character in `text`
// (including spaces and punctuation) and returns the HashMap.
//
// Use the entry API: `*freq.entry(ch).or_insert(0) += 1`
//
// All tests must pass.

use std::collections::HashMap;

fn count_letters(text: &str) -> HashMap<char, u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_single_chars() {
        let freq = count_letters("aaa");
        assert_eq!(freq[&'a'], 3);
    }

    #[test]
    fn counts_multiple_chars() {
        let freq = count_letters("abcabc");
        assert_eq!(freq[&'a'], 2);
        assert_eq!(freq[&'b'], 2);
        assert_eq!(freq[&'c'], 2);
    }

    #[test]
    fn counts_spaces_too() {
        let freq = count_letters("a b");
        assert_eq!(freq[&'a'], 1);
        assert_eq!(freq[&' '], 1);
        assert_eq!(freq[&'b'], 1);
    }

    #[test]
    fn empty_returns_empty_map() {
        let freq = count_letters("");
        assert!(freq.is_empty());
    }
}

fn main() {
    let freq = count_letters("hello world");
    let mut pairs: Vec<(char, u32)> = freq.into_iter().collect();
    pairs.sort_by_key(|p| p.0);
    for (ch, count) in pairs {
        println!("  {:?}: {}", ch, count);
    }
}
