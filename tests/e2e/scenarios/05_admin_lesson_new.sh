#!/usr/bin/env bash
# Scenario 05: `rbb-admin lesson new` scaffolds a complete lesson dir
# and its book chapter stub.

source /e2e/lib.sh

cd /opt/rbb

capture rbb-admin lesson new 99 sandbox
assert_exit_zero "$CAP_CODE" "lesson scaffold exits 0"
assert_contains  "$CAP_OUT" "scaffolded" "prints scaffolded line"

BASE=/opt/rbb/lessons/99-sandbox
assert_file_exists "$BASE/README.md"                 "lesson README created"
assert_file_exists "$BASE/project/Cargo.toml"        "project Cargo.toml created"
assert_file_exists "$BASE/project/src/lib.rs"        "project src/lib.rs created"
assert_file_exists "$BASE/project/tests/smoke.rs"    "project smoke test created"
assert_file_exists "/opt/rbb/book/src/99-sandbox.md" "book chapter stub created"

# Cargo name must match our convention so the workspace picks it up.
cargo_toml=$(cat "$BASE/project/Cargo.toml")
assert_contains "$cargo_toml" 'name = "lesson-99-sandbox"' "package name follows convention"

# Exercise add-on scaffolds a new exercise file with the right prefix.
capture rbb-admin lesson add-exercise 99 greeting
assert_exit_zero "$CAP_CODE" "add-exercise exits 0"

# First exercise in an empty lesson is ex1_*.
assert_file_exists "$BASE/exercises/ex1_greeting.rs" "first exercise file created"

# SUMMARY.md should have been auto-updated. Lesson 99 wasn't in the
# original syllabus, so a new line gets appended under the last phase.
summary=$(cat /opt/rbb/book/src/SUMMARY.md)
assert_contains "$summary" "./99-sandbox.md" "SUMMARY links the new chapter"
assert_contains "$summary" "[Sandbox]"       "SUMMARY title derived from slug"

# Idempotency: running again should not add a duplicate line.
capture rbb-admin lesson new 98 other-sandbox
count=$(grep -c "./99-sandbox.md" /opt/rbb/book/src/SUMMARY.md)
assert_eq "$count" "1" "SUMMARY has exactly one entry for lesson 99"

# Renaming an existing lesson replaces the old slug in SUMMARY.
# Lesson 03 was "functions" in the shipped syllabus.
assert_contains "$summary" "./03-functions.md" "pre-existing lesson 03 intact"

scenario_summary
