// Exercise 3.3 — .enumerate() for position + value
//
// Implement the two functions below using .enumerate().
// No closures.
//
// All tests must pass.

fn first_position(text: &str, target: char) -> Option<usize> {
    // Return the index (0-based) of the first occurrence of `target`
    // in `text`. Return None if not found.
    todo!()
}

fn char_at(text: &str, index: usize) -> Option<char> {
    // Return the character at position `index` (0-based).
    // Return None if the index is out of range.
    // Hint: use .enumerate() in a for loop
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_first_e() {
        assert_eq!(first_position("hello", 'l'), Some(2));
    }

    #[test]
    fn not_found() {
        assert_eq!(first_position("hello", 'z'), None);
    }

    #[test]
    fn first_occurrence_not_second() {
        // 'l' appears at index 2, not 3
        assert_eq!(first_position("hello", 'l'), Some(2));
    }

    #[test]
    fn char_at_valid() {
        assert_eq!(char_at("hello", 1), Some('e'));
    }

    #[test]
    fn char_at_out_of_range() {
        assert_eq!(char_at("hello", 10), None);
    }

    #[test]
    fn char_at_zero() {
        assert_eq!(char_at("hello", 0), Some('h'));
    }
}

fn main() {
    let text = "cipher";
    println!("first 'p' at: {:?}", first_position(text, 'p'));
    println!("char at 3  : {:?}", char_at(text, 3));
}
