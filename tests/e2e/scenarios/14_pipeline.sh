#!/usr/bin/env bash
# Scenario 14: the whole pipeline the README promises.
#
#   fresh server  →  setup.sh  →  bulk onboard from a roster  →
#   student can log in + run rbb  →  admin scaffolds a new lesson
#   and pushes  →  student pulls and sees it.
#
# Runs on the `rbb-e2e-bare` image (just a rust:1-bookworm with the
# course checkout at /opt/rust-by-building; no prebuilt binaries, no
# /srv/rbb, no users). setup.sh is expected to create all of that.

source /e2e/lib.sh

# --- 1. setup.sh runs non-interactively ---------------------------------

cat > /tmp/roster.txt <<'EOF'
alice
bob
carol
EOF

# EDITOR_CHOICE=3 (both), STUDENTS_FILE sends us down the bulk-onboard path.
export EDITOR_CHOICE=3
export STUDENTS_FILE=/tmp/roster.txt
export REPO_SRC=/opt/rust-by-building

# Capture so we can grep the "Credentials written" line for the path.
setup_log=$(bash /opt/rust-by-building/server/setup.sh 2>&1)
setup_rc=$?
assert_exit_zero "$setup_rc" "setup.sh exits 0"
assert_contains "$setup_log" "Rust by Building server is ready" "setup prints ready banner"

# --- 2. the post-setup layout ------------------------------------------

assert_file_exists /usr/local/bin/rbb        "rbb installed"
assert_file_exists /usr/local/bin/rbb-admin  "rbb-admin installed"
assert_file_exists /srv/rbb/rust-by-building.git/HEAD "bare repo created"
assert_file_exists /srv/rbb/vendor           "shared vendor tree seeded"

# --- 3. students were provisioned --------------------------------------

for name in alice bob carol; do
    assert_file_exists "/home/$name/rust-by-building/Cargo.toml" "$name has checkout"
    assert_file_exists "/home/$name/.rbb/env"                    "$name has port env"
done

# setup.sh writes credentials to /root/rbb-credentials-<ts>.txt (mode 600).
creds_file=$(ls -1t /root/rbb-credentials-*.txt 2>/dev/null | head -1 || true)
if [[ -z "$creds_file" ]]; then
    fail "no credentials file found under /root/"
    scenario_summary
fi
pass "credentials file written: $(basename "$creds_file")"
mode=$(stat -c '%a' "$creds_file")
assert_eq "$mode" "600" "credentials file is mode 600"
lines=$(wc -l < "$creds_file" | tr -d ' ')
assert_eq "$lines" "3" "three credentials rows"

# Each student has an actual password hash in /etc/shadow.
for name in alice bob carol; do
    hashed=$(grep "^$name:" /etc/shadow | cut -d: -f2)
    if [[ -n "$hashed" && "$hashed" != "!" && "$hashed" != "*" ]]; then
        pass "$name has password hash set"
    else
        fail "$name password hash missing (got '$hashed')"
    fi
done

# --- 4. a student can actually run rbb ---------------------------------

# Log in as alice via su (SSH would add flakiness without coverage).
capture su -l alice -c "
    export PATH=/usr/local/cargo/bin:/usr/local/bin:\$PATH
    export CARGO_HOME=/usr/local/cargo
    export RUSTUP_HOME=/usr/local/rustup
    cd ~/rust-by-building && rbb status
"
assert_exit_zero "$CAP_CODE" "alice can run rbb status"
assert_contains  "$CAP_OUT" "functions" "alice sees lesson 03"

# --- 5. content update propagation -------------------------------------

# Admin adds a new lesson in /opt (their working checkout).
cd /opt/rust-by-building
rbb-admin lesson new 99 pipeline-test >/dev/null
git -c user.email=admin@test -c user.name=admin add -A
git -c user.email=admin@test -c user.name=admin commit --quiet -m "add lesson 99"
git push --quiet /srv/rbb/rust-by-building.git main

# Alice pulls and sees the new lesson.
capture su -l alice -c "
    export PATH=/usr/local/cargo/bin:/usr/local/bin:\$PATH
    export CARGO_HOME=/usr/local/cargo
    export RUSTUP_HOME=/usr/local/rustup
    cd ~/rust-by-building && git pull --quiet && rbb status
"
assert_exit_zero "$CAP_CODE" "alice can git pull + rbb status after update"
assert_contains  "$CAP_OUT" "99"             "new lesson id shows up"
assert_contains  "$CAP_OUT" "pipeline-test"  "new lesson slug shows up"

scenario_summary
