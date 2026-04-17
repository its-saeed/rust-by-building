# Shared helpers for e2e scenarios.
# Source at the top of each scenario: `source /e2e/lib.sh`

set -uo pipefail

# Escape codes (only applied if stdout is a tty; otherwise plain).
if [[ -t 1 ]]; then
    RED=$'\033[31m'
    GREEN=$'\033[32m'
    YELLOW=$'\033[33m'
    DIM=$'\033[2m'
    RST=$'\033[0m'
else
    RED=''; GREEN=''; YELLOW=''; DIM=''; RST=''
fi

SCENARIO="${SCENARIO:-${0##*/}}"

_fail_count=0
_pass_count=0

# Asserts. Each prints one line; accumulated failures cause the scenario
# to exit non-zero at the end.

assert_eq() {
    # assert_eq <actual> <expected> <message>
    local actual="$1" expected="$2" msg="${3:-values equal}"
    if [[ "$actual" == "$expected" ]]; then
        pass "$msg"
    else
        fail "$msg"
        printf '  %sexpected%s: %q\n' "$DIM" "$RST" "$expected"
        printf '  %s  actual%s: %q\n' "$DIM" "$RST" "$actual"
    fi
}

assert_contains() {
    # assert_contains <haystack> <needle> <message>
    local haystack="$1" needle="$2" msg="${3:-contains substring}"
    if [[ "$haystack" == *"$needle"* ]]; then
        pass "$msg"
    else
        fail "$msg"
        printf '  %sexpected substring%s: %q\n' "$DIM" "$RST" "$needle"
        printf '  %s           haystack%s:\n' "$DIM" "$RST"
        printf '%s\n' "$haystack" | sed 's/^/    /'
    fi
}

assert_not_contains() {
    local haystack="$1" needle="$2" msg="${3:-does not contain substring}"
    if [[ "$haystack" != *"$needle"* ]]; then
        pass "$msg"
    else
        fail "$msg"
        printf '  %sunexpected substring%s: %q\n' "$DIM" "$RST" "$needle"
    fi
}

assert_exit_zero() {
    local code="$1" msg="${2:-exit 0}"
    assert_eq "$code" "0" "$msg"
}

assert_exit_nonzero() {
    local code="$1" msg="${2:-exit != 0}"
    if [[ "$code" != "0" ]]; then
        pass "$msg"
    else
        fail "$msg (got exit 0)"
    fi
}

assert_file_exists() {
    local path="$1" msg="${2:-file exists: $1}"
    if [[ -e "$path" ]]; then
        pass "$msg"
    else
        fail "$msg"
    fi
}

pass() {
    _pass_count=$((_pass_count + 1))
    printf '  %s✓%s %s\n' "$GREEN" "$RST" "$1"
}

fail() {
    _fail_count=$((_fail_count + 1))
    printf '  %s✗%s %s\n' "$RED" "$RST" "$1"
}

scenario_summary() {
    printf '\n[%s] %d passed, %d failed\n' "$SCENARIO" "$_pass_count" "$_fail_count"
    if (( _fail_count > 0 )); then
        exit 1
    fi
}

# Convenience: capture stdout + exit code from a command.
# Usage:
#   capture some command --with --args
#   echo "$CAP_OUT"
#   echo "$CAP_CODE"
capture() {
    CAP_OUT=$("$@" 2>&1)
    CAP_CODE=$?
    return 0
}

# Run a command as alice (the fixture student). `su -l` resets the
# environment, so we restore PATH, CARGO_HOME, and RUSTUP_HOME — the
# rust:bookworm image puts cargo/rustup under /usr/local/ and expects
# those vars to point at them.
as_alice() {
    su -l alice -c "
        export PATH=/usr/local/cargo/bin:/usr/local/bin:\$PATH
        export CARGO_HOME=/usr/local/cargo
        export RUSTUP_HOME=/usr/local/rustup
        cd ~/rust-by-building && $*
    "
}

# Poll a file (or command output) for a substring, up to `timeout` seconds.
# Usage:
#   wait_for_substring <file> <needle> <timeout_secs> <message>
wait_for_substring() {
    local file="$1" needle="$2" timeout="$3" msg="${4:-wait for substring}"
    local waited=0
    while (( waited < timeout )); do
        if [[ -f "$file" ]] && grep -qF "$needle" "$file" 2>/dev/null; then
            pass "$msg (after ${waited}s)"
            return 0
        fi
        sleep 1
        waited=$((waited + 1))
    done
    fail "$msg (timed out after ${timeout}s)"
    if [[ -f "$file" ]]; then
        printf '  %scurrent file contents%s:\n' "$DIM" "$RST"
        sed 's/^/    /' "$file" | tail -30
    fi
    return 1
}
