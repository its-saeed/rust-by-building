#!/usr/bin/env bash
# Scenario 02: `rbb test 03` fails when the lesson project still has
# `todo!()` stubs. Progress must not flip to done.

source /e2e/lib.sh

capture as_alice "rbb test 03"
assert_exit_nonzero "$CAP_CODE" "rbb test 03 fails on stubs"
assert_contains    "$CAP_OUT" "tests failed" "emits the tests-failed line"

# Progress file should either not exist yet or not list lesson 03 as done.
if [[ -f /home/alice/.rbb/progress.json ]]; then
    progress=$(cat /home/alice/.rbb/progress.json)
    assert_not_contains "$progress" '"Done"' "lesson 03 is not marked Done after failure"
else
    pass "progress.json not written on failing run"
fi

scenario_summary
