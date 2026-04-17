// Exercise 3.1 — return value
//
// This function is supposed to return the square of its argument.
// It compiles, but something is wrong. Fix it so the test passes.
//
// Hint: re-read the "Functions return the last expression" section.

fn square(x: i32) -> i32 {
    x * x;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squares_small_numbers() {
        assert_eq!(square(3), 9);
        assert_eq!(square(0), 0);
        assert_eq!(square(-4), 16);
    }
}

fn main() {
    println!("3² = {}", square(3));
}
