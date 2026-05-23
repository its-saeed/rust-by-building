// Exercise 3.4 — .take(), .skip(), and .rev()
//
// Implement the three functions below.
// Use .take(), .skip(), .rev() — no closures needed.
//
// All tests must pass.

fn first_n_chars(text: &str, n: usize) -> String {
    // Return the first `n` characters of `text` as a new String.
    // If text is shorter than n, return the whole string.
    todo!()
}

fn chars_from(text: &str, start: usize) -> String {
    // Return all characters of `text` starting at index `start`.
    // If start >= text length, return an empty String.
    todo!()
}

fn reversed(text: &str) -> String {
    // Return the characters of `text` in reverse order as a String.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_3() {
        assert_eq!(first_n_chars("hello", 3), "hel");
    }

    #[test]
    fn first_n_longer_than_text() {
        assert_eq!(first_n_chars("hi", 10), "hi");
    }

    #[test]
    fn from_index_2() {
        assert_eq!(chars_from("hello", 2), "llo");
    }

    #[test]
    fn from_index_beyond() {
        assert_eq!(chars_from("hi", 5), "");
    }

    #[test]
    fn reverse_word() {
        assert_eq!(reversed("hello"), "olleh");
    }

    #[test]
    fn reverse_empty() {
        assert_eq!(reversed(""), "");
    }
}

fn main() {
    let text = "cipher";
    println!("first 3   : {}", first_n_chars(text, 3));
    println!("from idx 2: {}", chars_from(text, 2));
    println!("reversed  : {}", reversed(text));
}
