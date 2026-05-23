// Exercise 3.3 — if let with enums
//
// `safe_divide(a: f64, b: f64) -> Option<f64>`
// Returns None if b is 0.0, otherwise Some(a / b).
//
// `show_result(result: Option<f64>)`
// Uses `if let` to print "Result: X" (one decimal place) if Some,
// or "Cannot divide by zero" if None.
//
// All tests must pass.

fn safe_divide(a: f64, b: f64) -> Option<f64> {
    todo!()
}

fn show_result(result: Option<f64>) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divides_normally() {
        assert_eq!(safe_divide(10.0, 4.0), Some(2.5));
    }

    #[test]
    fn divide_by_zero_is_none() {
        assert_eq!(safe_divide(5.0, 0.0), None);
    }

    #[test]
    fn divide_by_zero_exact() {
        assert_eq!(safe_divide(1.0, 0.0), None);
    }
}

fn main() {
    show_result(safe_divide(10.0, 4.0));   // Result: 2.5
    show_result(safe_divide(5.0, 0.0));    // Cannot divide by zero
}
