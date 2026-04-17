// Exercise 3.2 — write the signature
//
// Write the function signature (the `fn NAME(...) -> ...` line) for
// a function named `is_even` that takes an i32 and returns a bool.
// Implement the body so the tests pass.

// TODO: write the function here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn even_and_odd() {
        assert_eq!(is_even(2), true);
        assert_eq!(is_even(3), false);
        assert_eq!(is_even(0), true);
        assert_eq!(is_even(-4), true);
    }
}

fn main() {
    println!("{}", is_even(42));
}
