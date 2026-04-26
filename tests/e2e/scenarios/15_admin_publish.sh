#!/usr/bin/env bash
# Scenario 15: `rbb-admin publish` runs a preflight check, pushes to
# the admin bare repo, and exports filtered content to the student repo.
# Also tests -m (stage + commit as part of publish) and --skip-check.

source /e2e/lib.sh

cd /opt/rbb

# /opt/rbb isn't a git repo in the baseline image — set one up.
git init --quiet --initial-branch=main
git config user.email admin@test
git config user.name  admin
git add -A >/dev/null
git commit --quiet -m "seed"

# Two bare remotes: admin repo and student repo.
BARE=/tmp/rbb-bare.git
STUDENT_BARE=/tmp/rbb-student.git
git init --quiet --bare --initial-branch=main "$BARE"
git init --quiet --bare --initial-branch=main "$STUDENT_BARE"
git config --system --add safe.directory "$BARE"
git config --system --add safe.directory "$STUDENT_BARE"
git config --system --add safe.directory /opt/rbb

# Baseline: lesson 03 has todo!() stubs, so preflight must fail.
capture rbb-admin publish --remote "$BARE" --student-repo "$STUDENT_BARE"
assert_exit_nonzero "$CAP_CODE" "publish refuses when preflight check fails"
assert_contains    "$CAP_OUT" "preflight" "mentions the preflight step"

# --skip-check bypass works when you truly need it.
capture rbb-admin publish --remote "$BARE" --student-repo "$STUDENT_BARE" --skip-check
assert_exit_zero "$CAP_CODE" "publish --skip-check exits 0"
assert_contains  "$CAP_OUT" "published" "prints published banner"

# Admin repo received the commit.
ref=$(git --git-dir="$BARE" show-ref --hash refs/heads/main | head -1)
if [[ -n "$ref" ]]; then
    pass "admin bare remote has refs/heads/main"
else
    fail "admin bare remote did not receive main"
fi

# Student repo also received filtered content.
student_ref=$(git --git-dir="$STUDENT_BARE" show-ref --hash refs/heads/main | head -1)
if [[ -n "$student_ref" ]]; then
    pass "student bare remote has refs/heads/main"
else
    fail "student bare remote did not receive main"
fi

# Student repo has lessons/ but NOT tools/.
student_tree=$(git --git-dir="$STUDENT_BARE" ls-tree --name-only HEAD)
assert_contains "$student_tree" "lessons" "student repo contains lessons/"
if echo "$student_tree" | grep -q "^tools$"; then
    fail "student repo must not contain tools/"
else
    pass "student repo does not contain tools/"
fi

# -m flag: make an uncommitted change, publish with -m, verify a commit is created.
echo "// extra comment" >> /opt/rbb/lessons/03-functions/exercises/ex1_args.rs
capture rbb-admin publish -m "test commit via -m" \
    --remote "$BARE" --student-repo "$STUDENT_BARE" --skip-check
assert_exit_zero "$CAP_CODE" "publish -m exits 0"
assert_contains  "$CAP_OUT" "published" "publish -m prints banner"
last_msg=$(git log --format=%s -1)
assert_eq "$last_msg" "test commit via -m" "-m created a commit with the right message"

# -m with nothing to commit still succeeds.
capture rbb-admin publish -m "no changes here" \
    --remote "$BARE" --student-repo "$STUDENT_BARE" --skip-check
assert_exit_zero "$CAP_CODE" "publish -m with clean tree exits 0"
assert_contains  "$CAP_OUT" "nothing to commit" "reports clean tree"

# Fix lesson 03 — this time the preflight should pass end-to-end.
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

capture rbb-admin publish --remote "$BARE" --student-repo "$STUDENT_BARE"
assert_exit_zero "$CAP_CODE" "full publish with passing preflight"
assert_contains  "$CAP_OUT" "self-check passed" "preflight output reported"
assert_contains  "$CAP_OUT" "published"         "push reported"

scenario_summary
