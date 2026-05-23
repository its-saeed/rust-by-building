// Exercise 3.2 — iterating over characters
//
// Implement the two functions below using for loops over .chars().
// No closures. Use a for loop and a match (or if) inside.
//
// All tests must pass.

fn count_alphabetic(text: &str) -> u32 {
    // Return how many characters in `text` are alphabetic.
    todo!()
}

fn count_spaces(text: &str) -> u32 {
    // Return how many space characters ' ' are in `text`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alphabetic_in_simple_text() {
        assert_eq!(count_alphabetic("hello world"), 10);
    }

    #[test]
    fn alphabetic_ignores_numbers() {
        assert_eq!(count_alphabetic("abc 123"), 3);
    }

    #[test]
    fn alphabetic_empty() {
        assert_eq!(count_alphabetic(""), 0);
    }

    #[test]
    fn spaces_in_phrase() {
        assert_eq!(count_spaces("one two three"), 2);
    }

    #[test]
    fn spaces_none() {
        assert_eq!(count_spaces("hello"), 0);
    }
}

fn main() {
    let ciphertext = "dolfh zdv ehjlqqlqj wr jhw yhub wluhg";
    println!("alphabetic : {}", count_alphabetic(ciphertext));
    println!("spaces     : {}", count_spaces(ciphertext));
    println!("other      : {}", ciphertext.chars().count() as u32
        - count_alphabetic(ciphertext) - count_spaces(ciphertext));
}
