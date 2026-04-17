// Exercise 3.3 — early return
//
// Complete `clamp` so that:
//   - values below `lo` are bumped up to `lo`
//   - values above `hi` are capped at `hi`
//   - otherwise the value is returned unchanged
//
// Use `return` for the early-exit cases.

fn clamp(x: i32, lo: i32, hi: i32) -> i32 {
    // TODO
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamps() {
        assert_eq!(clamp(5,   0, 10),  5);
        assert_eq!(clamp(-3,  0, 10),  0);
        assert_eq!(clamp(42,  0, 10), 10);
    }
}

fn main() {
    println!("{}", clamp(42, 0, 10));
}
