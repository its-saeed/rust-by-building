#!/usr/bin/env bash
# Scenario 10: `rbb-admin user add/remove/list` actually provision
# Linux users, seed their checkout, and clean up on removal.

source /e2e/lib.sh

# Add bob, seeded from the admin's own checkout (no bare repo in the
# test container).
capture rbb-admin user add bob --from /opt/rbb
assert_exit_zero "$CAP_CODE" "user add exits 0"
assert_contains "$CAP_OUT" "created user bob" "prints created line"
assert_contains "$CAP_OUT" "port range"       "prints port range"
assert_contains "$CAP_OUT" "password"         "prints generated password"

# Password hash must be set in /etc/shadow.
hashed=$(grep "^bob:" /etc/shadow | cut -d: -f2)
if [[ -n "$hashed" && "$hashed" != "!" && "$hashed" != "*" && "$hashed" != "!!" ]]; then
    pass "bob has password hash in /etc/shadow"
else
    fail "bob has no password hash (got '$hashed')"
fi

# User exists at the OS level.
assert_file_exists /home/bob "home dir created"
assert_file_exists /home/bob/rust-by-building "checkout populated"
assert_file_exists /home/bob/rust-by-building/Cargo.toml "workspace Cargo.toml"
assert_file_exists /home/bob/.rbb/env "env file with port range"

env_content=$(cat /home/bob/.rbb/env)
assert_contains "$env_content" "RBB_PORT_BASE=" "env has RBB_PORT_BASE"
assert_contains "$env_content" "RBB_PORT_END="  "env has RBB_PORT_END"

# Ownership: bob owns their own home.
owner=$(stat -c '%U' /home/bob/rust-by-building)
assert_eq "$owner" "bob" "bob owns the checkout"

# bob can actually run rbb status without errors.
capture su -l bob -c "
    export PATH=/usr/local/cargo/bin:/usr/local/bin:\$PATH
    export CARGO_HOME=/usr/local/cargo
    export RUSTUP_HOME=/usr/local/rustup
    cd ~/rust-by-building && rbb status
"
assert_exit_zero "$CAP_CODE" "bob can run rbb status"
assert_contains "$CAP_OUT" "functions" "bob sees lesson 03"

# List shows both alice (seeded in the image) and bob.
capture rbb-admin user list
assert_contains "$CAP_OUT" "alice" "list shows alice"
assert_contains "$CAP_OUT" "bob"   "list shows bob"

# Remove bob. Default removes home.
capture rbb-admin user remove bob
assert_exit_zero "$CAP_CODE" "user remove exits 0"
assert_contains "$CAP_OUT" "removed user bob" "prints removed line"

# Home is gone.
if [[ -d /home/bob ]]; then
    fail "/home/bob still exists after remove"
else
    pass "/home/bob removed"
fi

# User account is gone.
if id bob >/dev/null 2>&1; then
    fail "bob account still exists"
else
    pass "bob account removed"
fi

# Add + --keep-home variant.
capture rbb-admin user add carol --from /opt/rbb
assert_exit_zero "$CAP_CODE" "add carol"
capture rbb-admin user remove carol --keep-home
assert_exit_zero "$CAP_CODE" "remove carol --keep-home"
assert_file_exists /home/carol "carol's home preserved"
if id carol >/dev/null 2>&1; then
    fail "carol account should be removed"
else
    pass "carol account removed, home kept"
fi

# Non-root can't add (useful for catching sudo-less mistakes).
capture su -l alice -c "rbb-admin user add mallory --from /opt/rbb 2>&1"
assert_exit_nonzero "$CAP_CODE" "non-root user add is rejected"
assert_contains "$CAP_OUT" "root" "error mentions root"

scenario_summary
