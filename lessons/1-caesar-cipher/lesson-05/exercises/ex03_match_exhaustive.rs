// Exercise 5.3 — match must be exhaustive
//
// This does not compile.
// The match covers 'A'..='Z' and 'a'..='z', but `c` is a char —
// there are many other possible values (digits, spaces, punctuation...).
// Add a wildcard arm to handle everything else.

fn describe_char(c: char) -> &'static str {
    match c {
        'A'..='Z' => "uppercase letter",
        'a'..='z' => "lowercase letter",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describes_correctly() {
        assert_eq!(describe_char('H'), "uppercase letter");
        assert_eq!(describe_char('h'), "lowercase letter");
        assert_eq!(describe_char('3'), "other");
        assert_eq!(describe_char(' '), "other");
        assert_eq!(describe_char('!'), "other");
    }
}

fn main() {
    println!("{}", describe_char('H'));
    println!("{}", describe_char('!'));
}
