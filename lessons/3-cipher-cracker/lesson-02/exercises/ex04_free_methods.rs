// Exercise 4 — iterator methods come for free
//
// `LetterIter` iterates over the alphabetic characters of a string,
// converting each to lowercase.
//
// The `chars` field holds the characters as a Vec<char> (already collected).
// `pos` is the current position.
//
// 1. Implement `Iterator` for `LetterIter`.
//    - `type Item = char`
//    - `next` returns the next lowercase alphabetic char, skipping
//      non-alphabetic characters.
//
// 2. The tests below use `.collect()`, `.count()`, and `.map()` on
//    a `LetterIter`. Make them pass — you only need to implement `next`.

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

// TODO: implement Iterator for LetterIter

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collects_letters_only() {
        let v: Vec<char> = LetterIter::new("Hello, World!").collect();
        assert_eq!(v, vec!['h', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd']);
    }

    #[test]
    fn counts_letters() {
        assert_eq!(LetterIter::new("abc 123 def").count(), 6);
    }

    #[test]
    fn maps_to_uppercase() {
        let v: Vec<char> = LetterIter::new("hi")
            .map(|c| c.to_ascii_uppercase())
            .collect();
        assert_eq!(v, vec!['H', 'I']);
    }
}

fn main() {
    let letters: Vec<char> = LetterIter::new("Rust, by Building!").collect();
    println!("{:?}", letters);
}
