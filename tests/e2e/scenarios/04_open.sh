#!/usr/bin/env bash
# Scenario 04: `rbb open 03` prints the lesson README to stdout.

source /e2e/lib.sh

capture as_alice "rbb open 03"
assert_exit_zero "$CAP_CODE" "rbb open 03 exits 0"
assert_contains  "$CAP_OUT" "Lesson 03" "README header is present"
assert_contains  "$CAP_OUT" "functions" "README mentions functions"
assert_contains  "$CAP_OUT" "exercises" "README references exercises"
assert_contains  "$CAP_OUT" "project"   "README references the project"

scenario_summary
