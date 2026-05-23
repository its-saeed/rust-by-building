// Final Exercise 1 — implement the cipher cracker from scratch
//
// Implement all four functions. All tests must pass.
// Use the compiler and test output as your guide.

use std::collections::HashMap;

fn letter_frequencies(text: &str) -> Vec<(char, u32)> {
    todo!()
}

fn guess_key(peak: char) -> u8 {
    todo!()
}

fn decrypt(text: &str, key: u8) -> String {
    todo!()
}

fn crack(ciphertext: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frequencies_sorted_descending() {
        let freq = letter_frequencies("aaabbc");
        assert_eq!(freq[0].0, 'a');
        assert_eq!(freq[0].1, 3);
        assert_eq!(freq[1].1, 2);
    }

    #[test]
    fn frequencies_lowercase_only() {
        let freq = letter_frequencies("AaBb");
        for (ch, _) in &freq {
            assert!(ch.is_lowercase());
        }
    }

    #[test]
    fn frequencies_ignores_non_letters() {
        let freq = letter_frequencies("a!b c1");
        assert_eq!(freq.len(), 2);
    }

    #[test]
    fn guess_key_h_gives_3() {
        assert_eq!(guess_key('h'), 3);
    }

    #[test]
    fn guess_key_l_gives_7() {
        assert_eq!(guess_key('l'), 7);
    }

    #[test]
    fn guess_key_e_gives_0() {
        assert_eq!(guess_key('e'), 0);
    }

    #[test]
    fn decrypt_basic() {
        assert_eq!(decrypt("Khoor, Zruog!", 3), "Hello, World!");
    }

    #[test]
    fn decrypt_preserves_non_letters() {
        assert_eq!(decrypt("khoor, zruog!", 3), "hello, world!");
    }

    #[test]
    fn decrypt_key_26_identity() {
        assert_eq!(decrypt("Hello", 26), "Hello");
    }

    #[test]
    fn crack_long_text() {
        let ciphertext = "dolfh zdv ehjlqqlqj wr jhw yhub wluhg ri vlwwlqj eb khu vlvwhu rq wkh edqn dqg ri kdylqj qrwklqj wr gr rqfh ru wzlfh vkh kdg shhshg lqwr wkh errn khu vlvwhu zdv uhdglqj exw lw kdg qr slfwxuhv ru frqyhuvdwlrqv";
        let result = crack(ciphertext);
        assert!(result.contains("alice"), "expected 'alice', got: {}", result);
        assert!(result.contains("beginning"), "expected 'beginning', got: {}", result);
    }

    #[test]
    fn crack_empty() {
        assert_eq!(crack(""), "");
    }
}

fn main() {
    let ciphertext = "dolfh zdv ehjlqqlqj wr jhw yhub wluhg ri vlwwlqj eb khu vlvwhu rq wkh edqn";
    println!("{}", crack(ciphertext));
}
