#!/usr/bin/env bash
# Scenario 13: `rbb-admin user bulk` onboards many students at once
# and emits username:password credentials, with the right file mode.

source /e2e/lib.sh

STUDENTS=/tmp/students.txt
CREDS=/tmp/creds.txt

# Whitespace + comments must be ignored.
cat > "$STUDENTS" <<'EOF'
# a class roster
bob

carol
dave
EOF

capture rbb-admin user bulk "$STUDENTS" --from /opt/rbb --credentials "$CREDS"
assert_exit_zero "$CAP_CODE" "bulk provisioning exits 0"
assert_contains  "$CAP_OUT" "wrote 3 credentials" "emits summary line"

assert_file_exists "$CREDS" "credentials file written"

# Mode is 600 — credentials aren't world-readable.
mode=$(stat -c '%a' "$CREDS")
assert_eq "$mode" "600" "credentials file is mode 600"

# Format: exactly three lines, each `name:password`.
line_count=$(wc -l < "$CREDS" | tr -d ' ')
assert_eq "$line_count" "3" "three credential rows emitted"

for name in bob carol dave; do
    row=$(grep "^$name:" "$CREDS" || true)
    assert_contains "$row" "$name:" "credentials contain $name"
    # password portion is >=8 chars (we defaulted to 14).
    password=${row#*:}
    if [[ ${#password} -ge 8 ]]; then
        pass "$name password is $((${#password})) chars"
    else
        fail "$name password too short: '$password'"
    fi
done

# Each user was actually created.
for name in bob carol dave; do
    assert_file_exists "/home/$name/rust-by-building/Cargo.toml" "$name has checkout"
    assert_file_exists "/home/$name/.rbb/env"                    "$name has .rbb/env"
done

# The generated passwords actually authenticate (verify via `su` with
# the password piped in — requires `-c` and a way to send a password.
# `chpasswd` set it; `su` with `SU_PAM_HIDE_PROMPT=1` and the `--preserve-environment`
# tricks are flaky. Easiest portable check: verify the hashed password
# field in /etc/shadow is non-empty and not `!` or `*`.
for name in bob carol dave; do
    hashed=$(grep "^$name:" /etc/shadow | cut -d: -f2)
    if [[ -n "$hashed" && "$hashed" != "!" && "$hashed" != "*" && "$hashed" != "!!" ]]; then
        pass "$name has a set password hash in /etc/shadow"
    else
        fail "$name has no password hash (got '$hashed')"
    fi
done

# Stdout mode (no --credentials): pairs go to stdout.
cat > "$STUDENTS" <<'EOF'
eve
EOF
capture rbb-admin user bulk "$STUDENTS" --from /opt/rbb
assert_exit_zero "$CAP_CODE" "stdout mode exits 0"
assert_contains  "$CAP_OUT" "eve:" "stdout output contains eve:..."

# Empty list is an error, not a silent success.
: > /tmp/empty.txt
capture rbb-admin user bulk /tmp/empty.txt --from /opt/rbb
assert_exit_nonzero "$CAP_CODE" "empty list fails"

# Non-root is rejected (consistency with user add).
capture su -l alice -c "rbb-admin user bulk $STUDENTS --from /opt/rbb 2>&1"
assert_exit_nonzero "$CAP_CODE" "non-root bulk is rejected"
assert_contains    "$CAP_OUT" "root" "error mentions root"

scenario_summary
