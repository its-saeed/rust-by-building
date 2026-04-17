#!/usr/bin/env bash
# Scenario 07: `rbb-admin progress` discovers student progress files
# under /home/*/.rbb/progress.json and reports one row per student.

source /e2e/lib.sh

LIB=/home/alice/rust-by-building/lessons/03-functions/project/src/lib.rs
cat > "$LIB" <<'EOF'
pub fn add(a: i32, b: i32) -> i32 { a + b }
pub fn sub(a: i32, b: i32) -> i32 { a - b }
pub fn mul(a: i32, b: i32) -> i32 { a * b }
pub fn is_positive(n: i32) -> bool { n > 0 }
pub fn describe_sign(n: i32) -> &'static str {
    if n > 0 { "positive" } else if n < 0 { "negative" } else { "zero" }
}
EOF
chown alice:alice "$LIB"

# alice finishes lesson 03 → progress.json gets written.
as_alice "rbb test 03" >/dev/null 2>&1
assert_file_exists /home/alice/.rbb/progress.json "alice's progress file exists"

# Admin dashboard should surface alice's row.
capture rbb-admin progress
assert_exit_zero "$CAP_CODE" "rbb-admin progress exits 0"
assert_contains  "$CAP_OUT" "alice" "alice appears on the dashboard"
assert_contains  "$CAP_OUT" "1/"    "alice has exactly one lesson done"

# Filtered view should also find alice.
capture rbb-admin progress alice
assert_exit_zero "$CAP_CODE" "filtered view exits 0"
assert_contains  "$CAP_OUT" "alice" "filtered view shows alice"

# A non-existent user filter returns 0 but no rows.
capture rbb-admin progress nobody
assert_exit_zero    "$CAP_CODE" "filter with unknown user exits 0"
assert_not_contains "$CAP_OUT" "alice" "filter excludes other users"

scenario_summary
