#!/usr/bin/env bash
# Scenario 16: `rbb-admin vendor-sync` re-runs cargo vendor and prints
# the follow-up commit instructions.

source /e2e/lib.sh

cd /opt/rbb

# Snapshot vendor/ so we can verify it's still coherent after the sync.
before=$(find vendor -maxdepth 2 -type d | wc -l | tr -d ' ')

capture rbb-admin vendor-sync
assert_exit_zero "$CAP_CODE" "vendor-sync exits 0"
assert_contains  "$CAP_OUT" "cargo vendor"       "mentions what it's running"
assert_contains  "$CAP_OUT" "vendor/ is now up to date" "prints success note"
assert_contains  "$CAP_OUT" "rbb-admin publish"  "hints at the next step"

assert_file_exists vendor/serde/Cargo.toml "a well-known vendored crate survived"

after=$(find vendor -maxdepth 2 -type d | wc -l | tr -d ' ')
assert_eq "$before" "$after" "vendor tree shape unchanged on a clean sync"

# The workspace must still build --frozen after a vendor-sync — anything
# else means vendor-sync corrupted the tree.
capture cargo build --frozen -p rbb --manifest-path Cargo.toml
assert_exit_zero "$CAP_CODE" "workspace still builds --frozen after vendor-sync"

scenario_summary
