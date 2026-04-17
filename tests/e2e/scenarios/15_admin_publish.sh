#!/usr/bin/env bash
# Scenario 15: `rbb-admin publish` runs a preflight check and then
# pushes to a bare remote. Skipping the preflight works too.

source /e2e/lib.sh

cd /opt/rbb

# /opt/rbb isn't a git repo in the baseline image — set one up.
git init --quiet --initial-branch=main
git config user.email admin@test
git config user.name  admin
git add -A >/dev/null
git commit --quiet -m "seed"

# Bare remote we push to.
BARE=/tmp/rbb-bare.git
git init --quiet --bare --initial-branch=main "$BARE"
git config --system --add safe.directory "$BARE"
git config --system --add safe.directory /opt/rbb

# Baseline: lesson 03 has todo!() stubs, so preflight must fail.
capture rbb-admin publish --remote "$BARE"
assert_exit_nonzero "$CAP_CODE" "publish refuses when preflight check fails"
assert_contains    "$CAP_OUT" "preflight" "mentions the preflight step"

# --skip-check bypass works when you truly need it.
capture rbb-admin publish --remote "$BARE" --skip-check
assert_exit_zero "$CAP_CODE" "publish --skip-check exits 0"
assert_contains  "$CAP_OUT" "published" "prints published banner"

# Remote actually received the commit.
ref=$(git --git-dir="$BARE" show-ref --hash refs/heads/main | head -1)
if [[ -n "$ref" ]]; then
    pass "bare remote has refs/heads/main"
else
    fail "bare remote did not receive main"
fi

# Fix lesson 03, this time the preflight should pass end-to-end.
cat > /opt/rbb/lessons/03-functions/project/src/lib.rs <<'EOF'
pub fn add(a: i32, b: i32) -> i32 { a + b }
pub fn sub(a: i32, b: i32) -> i32 { a - b }
pub fn mul(a: i32, b: i32) -> i32 { a * b }
pub fn is_positive(n: i32) -> bool { n > 0 }
pub fn describe_sign(n: i32) -> &'static str {
    if n > 0 { "positive" } else if n < 0 { "negative" } else { "zero" }
}
EOF
git add -A >/dev/null
git commit --quiet -m "implement lesson 03"

capture rbb-admin publish --remote "$BARE"
assert_exit_zero "$CAP_CODE" "full publish with passing preflight"
assert_contains  "$CAP_OUT" "self-check passed" "preflight output reported"
assert_contains  "$CAP_OUT" "published"         "push reported"

scenario_summary
