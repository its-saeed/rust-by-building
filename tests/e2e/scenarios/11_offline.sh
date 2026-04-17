#!/usr/bin/env bash
# Scenario 11: the whole flow runs with zero network access.
# Driven by run.sh: the container is launched with --network=none.

source /e2e/lib.sh

# Sanity: network really is off.
if curl -sS --max-time 2 https://crates.io/ >/dev/null 2>&1; then
    fail "network is reachable — the harness forgot --network=none"
    scenario_summary
fi
pass "outbound network is blocked"

# rbb check 03 compiles and runs four exercises from scratch. If
# anything under rbb needed to fetch a crate, this would fail.
capture as_alice "rbb check 03"
assert_exit_nonzero "$CAP_CODE" "baseline check returns non-zero (broken exercises)"
assert_contains    "$CAP_OUT" "ex1_return_value" "exercises discovered"

# Implement lesson 03 project and run tests. cargo test goes through
# the full build graph — serde, clap transitively. All must resolve
# locally.
# Write the implementation to both /opt/rbb (for rbb-admin check) and
# /home/alice/rust-by-building (for rbb test). Quoted heredoc so the
# apostrophes in `&'static` come through untouched.
cat > /tmp/impl.rs <<'EOF'
pub fn add(a: i32, b: i32) -> i32 { a + b }
pub fn sub(a: i32, b: i32) -> i32 { a - b }
pub fn mul(a: i32, b: i32) -> i32 { a * b }
pub fn is_positive(n: i32) -> bool { n > 0 }
pub fn describe_sign(n: i32) -> &'static str {
    if n > 0 { "positive" } else if n < 0 { "negative" } else { "zero" }
}
EOF
cp /tmp/impl.rs /home/alice/rust-by-building/lessons/03-functions/project/src/lib.rs
cp /tmp/impl.rs /opt/rbb/lessons/03-functions/project/src/lib.rs
chown alice:alice /home/alice/rust-by-building/lessons/03-functions/project/src/lib.rs

capture as_alice "rbb test 03"
assert_exit_zero "$CAP_CODE" "rbb test 03 passes offline"
assert_contains  "$CAP_OUT" "all tests passed" "tests ran"

# rbb-admin self-check builds the whole workspace from vendor/.
capture bash -c "cd /opt/rbb && rbb-admin check"
assert_exit_zero "$CAP_CODE" "admin self-check passes offline"

# Belt-and-braces: an explicit cargo build with --offline must succeed.
capture su -l alice -c "
    export PATH=/usr/local/cargo/bin:/usr/local/bin:\$PATH
    export CARGO_HOME=/usr/local/cargo
    export RUSTUP_HOME=/usr/local/rustup
    cd ~/rust-by-building
    cargo build --offline -p rbb 2>&1
"
assert_exit_zero "$CAP_CODE" "cargo build --offline -p rbb works"

scenario_summary
