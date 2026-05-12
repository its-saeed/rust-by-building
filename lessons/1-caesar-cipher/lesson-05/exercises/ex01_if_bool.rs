// Exercise 5.1 — conditions must be bool
//
// This does not compile.
// In Rust, `if` requires a bool condition. Integers are not booleans.
// Fix the condition so it compiles and the test passes.

fn is_big(n: i32) -> &'static str {
    if n {
        "big"
    } else {
        "small"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_correctly() {
        assert_eq!(is_big(10), "big");
        assert_eq!(is_big(0),  "small");
        assert_eq!(is_big(-3), "small");
    }
}

fn main() {
    println!("{}", is_big(10));
}
