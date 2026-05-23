// Exercise 5.2 — find the three most frequent letters
//
// `top3_letters` takes a frequency map and returns up to 3 chars:
// the most frequent, then 2nd most frequent, then 3rd most frequent.
//
// Strategy: make up to 3 passes. On each pass, find the highest-count
// letter that is not already in your result Vec.
//
// Return fewer than 3 if the map has fewer than 3 entries.
// All tests must pass.

use std::collections::HashMap;

fn top3_letters(freq: &HashMap<char, u32>) -> Vec<char> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_freq(pairs: &[(&str, u32)]) -> HashMap<char, u32> {
        let mut m = HashMap::new();
        for (s, n) in pairs {
            m.insert(s.chars().next().unwrap(), *n);
        }
        m
    }

    #[test]
    fn top3_from_four() {
        let freq = make_freq(&[("h", 10), ("w", 7), ("u", 5), ("l", 3)]);
        let top = top3_letters(&freq);
        assert_eq!(top.len(), 3);
        assert_eq!(top[0], 'h');
        assert_eq!(top[1], 'w');
        assert_eq!(top[2], 'u');
    }

    #[test]
    fn fewer_than_three() {
        let freq = make_freq(&[("h", 5), ("w", 3)]);
        let top = top3_letters(&freq);
        assert_eq!(top.len(), 2);
        assert!(top.contains(&'h'));
        assert!(top.contains(&'w'));
    }

    #[test]
    fn empty_map() {
        let freq: HashMap<char, u32> = HashMap::new();
        assert!(top3_letters(&freq).is_empty());
    }
}

fn main() {
    let mut freq = HashMap::new();
    freq.insert('h', 32u32);
    freq.insert('w', 22);
    freq.insert('u', 18);
    freq.insert('l', 17);

    let top = top3_letters(&freq);
    for (i, ch) in top.iter().enumerate() {
        println!("  {}: '{}'", i + 1, ch);
    }
}
