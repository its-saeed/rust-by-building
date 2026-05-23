// Exercise 3.1 — ranges and basic iterator methods
//
// Implement the three functions below using iterator methods.
// No for loops needed — just .sum(), .count(), .min(), .max().
//
// All tests must pass.

fn sum_to(n: u32) -> u32 {
    // Return the sum of 1 + 2 + ... + n (inclusive).
    // Hint: use a range and .sum()
    todo!()
}

fn count_in_range(from: u32, to: u32) -> usize {
    // Return how many integers are in the range from..=to (inclusive).
    // Hint: use a range and .count()
    todo!()
}

fn largest_below(limit: u32) -> Option<u32> {
    // Return the largest integer strictly below `limit`, starting from 1.
    // Return None if limit is 0 or 1 (no integers in range).
    // Hint: use a range and .max()
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_to_5() {
        assert_eq!(sum_to(5), 15);   // 1+2+3+4+5
    }

    #[test]
    fn sum_to_100() {
        assert_eq!(sum_to(100), 5050);
    }

    #[test]
    fn sum_to_0() {
        assert_eq!(sum_to(0), 0);
    }

    #[test]
    fn count_range() {
        assert_eq!(count_in_range(3, 7), 5);   // 3,4,5,6,7
    }

    #[test]
    fn largest_below_5() {
        assert_eq!(largest_below(5), Some(4));
    }

    #[test]
    fn largest_below_1() {
        assert_eq!(largest_below(1), None);
    }
}

fn main() {
    println!("sum 1..=10  = {}", sum_to(10));
    println!("count 1..=5 = {}", count_in_range(1, 5));
    println!("max < 8     = {:?}", largest_below(8));
}
