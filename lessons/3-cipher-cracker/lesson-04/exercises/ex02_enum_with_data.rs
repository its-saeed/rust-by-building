// Exercise 3.2 — enum variants with data
//
// Define an enum `Coin` with three variants:
//   Penny           — worth 1
//   Nickel          — worth 5
//   Dime            — worth 10
//   Quarter         — worth 25
//
// Implement `fn value(c: Coin) -> u32` that returns the cent value.
//
// Then implement `fn total(coins: &[Coin]) -> u32` that sums a slice of coins.
// Hint: you can't use `Coin` after moving it into `value`, so match directly
// in `total` instead of calling `value`.
//
// All tests must pass.

// TODO: define Coin enum

// TODO: implement value(c: Coin) -> u32

// TODO: implement total(coins: &[Coin]) -> u32

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn penny_is_1() {
        assert_eq!(value(Coin::Penny), 1);
    }

    #[test]
    fn quarter_is_25() {
        assert_eq!(value(Coin::Quarter), 25);
    }

    #[test]
    fn total_mixed() {
        let coins = [Coin::Quarter, Coin::Dime, Coin::Nickel, Coin::Penny];
        assert_eq!(total(&coins), 41);
    }

    #[test]
    fn total_empty() {
        assert_eq!(total(&[]), 0);
    }
}

fn main() {
    // No assertions — if it compiles and tests pass, you're done.
}
