#!/usr/bin/env bash
# Scenario 03: after the student implements the lesson 03 project,
# `rbb test 03` passes and progress flips to Done.

source /e2e/lib.sh

LIB=/home/alice/rust-by-building/lessons/03-functions/project/src/lib.rs

# Write a known-good implementation in place of the todo!() stubs.
cat > "$LIB" <<'EOF'
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
chown alice:alice "$LIB"

capture as_alice "rbb test 03"
assert_exit_zero "$CAP_CODE" "rbb test 03 passes with real implementation"
assert_contains  "$CAP_OUT" "all tests passed" "prints success line"

assert_file_exists /home/alice/.rbb/progress.json "progress.json written"
progress=$(cat /home/alice/.rbb/progress.json)
assert_contains "$progress" '"Done"'            "progress contains Done status"
assert_contains "$progress" '"project_passing": true' "project marked passing"

scenario_summary
