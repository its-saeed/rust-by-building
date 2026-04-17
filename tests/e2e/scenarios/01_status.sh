#!/usr/bin/env bash
# Scenario 01: `rbb status` lists the discovered lessons for the student.

source /e2e/lib.sh

capture as_alice "rbb status"
assert_exit_zero "$CAP_CODE" "rbb status exits 0"
assert_contains  "$CAP_OUT" "01"       "lists lesson 01"
assert_contains  "$CAP_OUT" "hello"    "shows lesson 01 slug"
assert_contains  "$CAP_OUT" "03"       "lists lesson 03"
assert_contains  "$CAP_OUT" "functions" "shows lesson 03 slug"
assert_contains  "$CAP_OUT" "not started" "marks fresh lessons as not started"

scenario_summary
