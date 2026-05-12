// Exercise 5.2 — both arms of an if expression must return the same type
//
// This does not compile.
// When `if` is used as an expression, both branches must produce the same type.
// Fix the `else` branch so both arms return a `&str`.

fn classify(n: i32) -> &'static str {
    let label = if n >= 0 { "non-negative" } else { 0 };
    label
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies() {
        assert_eq!(classify(5),  "non-negative");
        assert_eq!(classify(0),  "non-negative");
        assert_eq!(classify(-1), "negative");
    }
}

fn main() {
    println!("{}", classify(5));
}
