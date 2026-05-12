// Exercise 5.2 — read_line includes a trailing newline
//
// After `read_line`, the buffer ends with '\n'.
// Without trimming, string comparisons fail.
//
// `parse_command` below should return the trimmed command string.
// Fix it so all tests pass.

fn parse_command(input: &str) -> &str {
    input   // BUG: should trim whitespace
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_newline() {
        assert_eq!(parse_command("quit\n"), "quit");
    }

    #[test]
    fn trim_spaces() {
        assert_eq!(parse_command("  add  "), "add");
    }

    #[test]
    fn no_whitespace_unchanged() {
        assert_eq!(parse_command("list"), "list");
    }
}

fn main() {
    println!("'{}'", parse_command("quit\n"));
    println!("'{}'", parse_command("  add  "));
}
