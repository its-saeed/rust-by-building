use lesson_03_calculator::*;

#[test]
fn addition() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(-1, 1), 0);
    assert_eq!(add(0, 0), 0);
}

#[test]
fn subtraction() {
    assert_eq!(sub(5, 3), 2);
    assert_eq!(sub(0, 4), -4);
}

#[test]
fn multiplication() {
    assert_eq!(mul(3, 4), 12);
    assert_eq!(mul(-2, 5), -10);
    assert_eq!(mul(0, 99), 0);
}

#[test]
fn positive_check() {
    assert_eq!(is_positive(1), true);
    assert_eq!(is_positive(-1), false);
    assert_eq!(is_positive(0), false);
}

#[test]
fn describe_sign_positive() {
    assert_eq!(describe_sign(42), "positive");
}

#[test]
fn describe_sign_negative_and_zero() {
    assert_eq!(describe_sign(-3), "negative");
    assert_eq!(describe_sign(0), "zero");
}
