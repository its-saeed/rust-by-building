// Exercise 3 — implement the Iterator trait
//
// `Countdown` counts DOWN from `start` to 1 (inclusive), then stops.
//
// Implement `Iterator` for `Countdown`.
// - `type Item` should be `u32`
// - `next` should return Some(n) for each value from `start` down to 1,
//   then None.
//
// Example: Countdown::new(3) should yield 3, 2, 1, then stop.

struct Countdown {
    current: u32,
}

impl Countdown {
    fn new(start: u32) -> Countdown {
        Countdown { current: start }
    }
}

// TODO: implement Iterator for Countdown

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_down() {
        let v: Vec<u32> = Countdown::new(3).collect();
        assert_eq!(v, vec![3, 2, 1]);
    }

    #[test]
    fn zero_yields_nothing() {
        let v: Vec<u32> = Countdown::new(0).collect();
        assert_eq!(v, vec![]);
    }

    #[test]
    fn works_with_sum() {
        let total: u32 = Countdown::new(4).sum();
        assert_eq!(total, 10);  // 4 + 3 + 2 + 1
    }

    #[test]
    fn works_with_filter() {
        let evens: Vec<u32> = Countdown::new(6)
            .filter(|n| n % 2 == 0)
            .collect();
        assert_eq!(evens, vec![6, 4, 2]);
    }
}

fn main() {
    for n in Countdown::new(5) {
        print!("{} ", n);
    }
    println!();  // 5 4 3 2 1
}
