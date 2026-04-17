#!/usr/bin/env bash
# Scenario 06: `rbb-admin check` runs the whole-workspace build + tests.
# Passes on the baseline (with lesson 03 implemented), fails when we
# deliberately break a lesson.

source /e2e/lib.sh

cd /opt/rbb

# Baseline: lesson 03 still has todo!() stubs. The workspace BUILDS
# (todo!() is valid rust, it just panics at runtime). But lesson 03's
# tests will FAIL, so `rbb-admin check` must fail.
capture rbb-admin check
assert_exit_nonzero "$CAP_CODE" "baseline with todo!() stubs fails the self-check"

# Now implement lesson 03 so the workspace is healthy.
cat > /opt/rbb/lessons/03-functions/project/src/lib.rs <<'EOF'
pub fn add(a: i32, b: i32) -> i32 { a + b }
pub fn sub(a: i32, b: i32) -> i32 { a - b }
pub fn mul(a: i32, b: i32) -> i32 { a * b }
pub fn is_positive(n: i32) -> bool { n > 0 }
pub fn describe_sign(n: i32) -> &'static str {
    if n > 0 { "positive" }
    else if n < 0 { "negative" }
    else { "zero" }
}
EOF

capture rbb-admin check
assert_exit_zero "$CAP_CODE" "implemented workspace passes the self-check"
assert_contains  "$CAP_OUT" "self-check passed" "prints success banner"

# Now break lesson 03 on purpose and confirm check catches it.
cat > /opt/rbb/lessons/03-functions/project/src/lib.rs <<'EOF'
pub fn add(a: i32, b: i32) -> i32 { a - b }   // intentionally wrong
pub fn sub(a: i32, b: i32) -> i32 { a - b }
pub fn mul(a: i32, b: i32) -> i32 { a * b }
pub fn is_positive(n: i32) -> bool { n > 0 }
pub fn describe_sign(n: i32) -> &'static str { "zero" }
EOF

capture rbb-admin check
assert_exit_nonzero "$CAP_CODE" "broken workspace fails the self-check"

scenario_summary
