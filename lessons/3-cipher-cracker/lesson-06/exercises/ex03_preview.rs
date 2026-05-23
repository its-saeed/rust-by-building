// Exercise 5.3 — truncate long strings for display
//
// `preview` should return up to `max_chars` characters of `text`.
// If the text is longer, append "..." to the truncated result.
// If the text is short enough, return it unchanged.
//
// All tests must pass.

fn preview(text: &str, max_chars: usize) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_text_unchanged() {
        assert_eq!(preview("hello", 10), "hello");
    }

    #[test]
    fn exact_length_unchanged() {
        assert_eq!(preview("hello", 5), "hello");
    }

    #[test]
    fn long_text_truncated() {
        assert_eq!(preview("hello world", 5), "hello...");
    }

    #[test]
    fn empty_text() {
        assert_eq!(preview("", 5), "");
    }

    #[test]
    fn max_zero_gives_dots() {
        assert_eq!(preview("hello", 0), "...");
    }
}

fn main() {
    println!("{}", preview("alice was beginning to get very tired", 20));
    println!("{}", preview("short", 20));
}
