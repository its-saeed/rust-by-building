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

scenario_summary
