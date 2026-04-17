#!/usr/bin/env bash
# Scenario 09: `rbb check 03` compiles and runs every exercise,
# reports per-exercise pass/fail, and records it in progress.json.

source /e2e/lib.sh

# The shipped exercises are intentionally broken in different ways
# (ex1 wrong trailing semicolon, ex2 missing function, ex3 stub,
# ex4 compiles fine). Baseline check should return non-zero and
# discover all 4 exercises.
capture as_alice "rbb check 03"
assert_exit_nonzero "$CAP_CODE" "check 03 fails while exercises are broken"
assert_contains "$CAP_OUT" "ex1_return_value" "lists ex1"
assert_contains "$CAP_OUT" "ex2_signature"    "lists ex2"
assert_contains "$CAP_OUT" "ex3_early_return" "lists ex3"
assert_contains "$CAP_OUT" "ex4_unit_return"  "lists ex4"

# Progress file records the baseline (status InProgress, partial exercises).
assert_file_exists /home/alice/.rbb/progress.json "progress.json written"
progress=$(cat /home/alice/.rbb/progress.json)
assert_contains "$progress" '"InProgress"' "status flips to InProgress after first check"

# Replace every exercise with a known-good version.
EX=/home/alice/rust-by-building/lessons/03-functions/exercises

cat > "$EX/ex1_return_value.rs" <<'EOF'
fn square(x: i32) -> i32 { x * x }
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn squares_small_numbers() {
        assert_eq!(square(3), 9);
        assert_eq!(square(0), 0);
        assert_eq!(square(-4), 16);
    }
}
fn main() { println!("{}", square(3)); }
EOF

cat > "$EX/ex2_signature.rs" <<'EOF'
fn is_even(n: i32) -> bool { n % 2 == 0 }
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn even_and_odd() {
        assert_eq!(is_even(2), true);
        assert_eq!(is_even(3), false);
        assert_eq!(is_even(0), true);
        assert_eq!(is_even(-4), true);
    }
}
fn main() { println!("{}", is_even(42)); }
EOF

cat > "$EX/ex3_early_return.rs" <<'EOF'
fn clamp(x: i32, lo: i32, hi: i32) -> i32 {
    if x < lo { return lo; }
    if x > hi { return hi; }
    x
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn clamps() {
        assert_eq!(clamp(5, 0, 10), 5);
        assert_eq!(clamp(-3, 0, 10), 0);
        assert_eq!(clamp(42, 0, 10), 10);
    }
}
fn main() { println!("{}", clamp(42, 0, 10)); }
EOF

# ex4 already compiles — keep as-is.

chown -R alice:alice "$EX"

capture as_alice "rbb check 03"
assert_exit_zero "$CAP_CODE" "check 03 passes after fixing all exercises"
assert_contains "$CAP_OUT" "4 of 4 exercises passing" "summary shows full count"

# Status output should now show 4/4 for lesson 03.
capture as_alice "rbb status"
assert_contains "$CAP_OUT" "4/4" "rbb status reflects 4/4 exercises"

scenario_summary
