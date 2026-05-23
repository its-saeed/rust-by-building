// Exercise 5.1 — read lines until an empty line
//
// `collect_lines` receives a slice of string lines.
// It should join them into a single space-separated String,
// stopping when it encounters an empty line.
//
// Implement it so all tests pass.

fn collect_lines(lines: &[&str]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stops_at_empty_line() {
        let input = ["hello world", "foo bar", "", "ignored line"];
        let result = collect_lines(&input);
        assert_eq!(result, "hello world foo bar");
    }

    #[test]
    fn single_line() {
        let input = ["only line", ""];
        assert_eq!(collect_lines(&input), "only line");
    }

    #[test]
    fn immediately_empty() {
        let input = [""];
        assert_eq!(collect_lines(&input), "");
    }

    #[test]
    fn no_empty_line_reads_all() {
        let input = ["one", "two", "three"];
        assert_eq!(collect_lines(&input), "one two three");
    }
}

fn main() {
    let lines = ["Khoor, Zruog!", "Wkh txlfn eurzq ira.", ""];
    println!("{}", collect_lines(&lines));
}
