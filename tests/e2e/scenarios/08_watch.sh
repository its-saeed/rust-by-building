#!/usr/bin/env bash
# Scenario 08: `rbb watch` does an initial run, then reruns when a file
# in the lesson changes. Uses fixed sleeps — not ideal but acceptable
# for a filesystem-watcher smoke test.

source /e2e/lib.sh

LIB=/home/alice/rust-by-building/lessons/03-functions/project/src/lib.rs
OUT=/tmp/watch.log
PIDFILE=/tmp/watch.pid

# Start the watcher in the background.
as_alice "nohup rbb watch 03 >$OUT 2>&1 & echo \$! >$PIDFILE; sleep 0.1"

# Initial run is against todo!() stubs — poll until the watcher reports
# failure. Cargo's first-invocation registry fetch can take ~10s, so use
# a generous timeout.
wait_for_substring "$OUT" "watching"     5  "watcher printed startup line"
wait_for_substring "$OUT" "tests failed" 60 "initial run fails on todo!() stubs"

# Flip the implementation to the known-good one. The watcher should
# notice the write (300ms debounce) and rerun automatically.
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

# Diagnostic: is the watcher still alive?
WATCH_PID=$(cat $PIDFILE)
if kill -0 "$WATCH_PID" 2>/dev/null; then
    pass "watcher process still alive (pid $WATCH_PID)"
else
    fail "watcher process died before the edit"
    scenario_summary
fi

# Pipe a debug marker into the log before and after the edit so we can
# see where the watcher hung in the timeline.
echo "=== about to edit lib.rs ===" >> "$OUT"

wait_for_substring "$OUT" "all tests passed" 60 "rerun after edit passes"

# Clean up.
kill "$WATCH_PID" 2>/dev/null || true

scenario_summary
