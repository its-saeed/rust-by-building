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

# --- JSON output ---

# Install jq would pull from the network; use python instead, which
# the rust image inherits from debian but may not have. Fall back to
# grep-based structural checks.

capture rbb-admin progress --json
assert_exit_zero "$CAP_CODE" "JSON progress exits 0"
assert_contains  "$CAP_OUT" '"name": "alice"'         "JSON has a name field"
assert_contains  "$CAP_OUT" '"lessons_done":'         "JSON has lessons_done"
assert_contains  "$CAP_OUT" '"lessons_in_progress":'  "JSON has lessons_in_progress"
assert_contains  "$CAP_OUT" '"last_active":'          "JSON has last_active"
assert_contains  "$CAP_OUT" '"status": "Done"'        "lesson 03 marked Done in JSON"
assert_contains  "$CAP_OUT" '"project_passing": true' "project passing flag set"

# Validate it's actually parseable JSON. python3 ships with bookworm.
if command -v python3 >/dev/null 2>&1; then
    parsed=$(printf '%s' "$CAP_OUT" | python3 -c 'import sys, json; d=json.load(sys.stdin); print(d[0]["lessons_done"])' 2>&1)
    assert_eq "$parsed" "1" "JSON is parseable and alice has exactly one lesson done"
fi

# Filtered JSON.
capture rbb-admin progress alice --json
assert_exit_zero "$CAP_CODE"             "filtered JSON exits 0"
assert_contains  "$CAP_OUT" '"alice"'    "filtered JSON includes alice"
if command -v python3 >/dev/null 2>&1; then
    count=$(printf '%s' "$CAP_OUT" | python3 -c 'import sys, json; print(len(json.load(sys.stdin)))')
    assert_eq "$count" "1" "filtered JSON has exactly one user"
fi

# Empty filter still produces valid JSON (an empty array).
capture rbb-admin progress nobody --json
assert_exit_zero "$CAP_CODE" "empty-filter JSON exits 0"
if command -v python3 >/dev/null 2>&1; then
    shape=$(printf '%s' "$CAP_OUT" | python3 -c 'import sys, json; print(type(json.load(sys.stdin)).__name__, len(json.load(sys.stdin)) if False else "")' 2>&1 || true)
    # Simpler: just assert it parses and is empty.
    count=$(printf '%s' "$CAP_OUT" | python3 -c 'import sys, json; print(len(json.load(sys.stdin)))')
    assert_eq "$count" "0" "empty filter yields empty JSON array"
fi

scenario_summary
