#!/usr/bin/env bash
# Scenario 12: `rbb tui` is registered and launches. We don't drive
# the full interactive UI here (that's manual-test territory) — we
# smoke-test that the subcommand exists and can open a terminal
# session through a pty, then quit cleanly when sent 'q'.

source /e2e/lib.sh

# 1. Subcommand is registered.
capture as_alice "rbb --help"
assert_contains "$CAP_OUT" "tui" "tui subcommand appears in --help"

# 2. `rbb tui --help` doesn't crash.
capture as_alice "rbb tui --help"
assert_exit_zero "$CAP_CODE" "rbb tui --help exits 0"

# 3. Full run with a pty, feeding 'q' to quit. `script` from bsdutils
#    is present on debian-bookworm; it creates a pty for us so crossterm
#    can enter raw mode.
if command -v script >/dev/null 2>&1; then
    # Run as alice. `script -q` suppresses its own banner. Give the
    # UI a beat to draw its first frame, THEN send 'q' — otherwise
    # the keystroke is consumed before anything renders.
    capture su -l alice -c "
        export PATH=/usr/local/cargo/bin:/usr/local/bin:\$PATH
        export CARGO_HOME=/usr/local/cargo
        export RUSTUP_HOME=/usr/local/rustup
        cd ~/rust-by-building
        (sleep 1 && printf 'q') | timeout 10 script -qec 'rbb tui' /tmp/tui.typescript
    "
    assert_exit_zero "$CAP_CODE" "rbb tui launches and quits on 'q'"
    # Rendering correctness is asserted by the TestBackend unit tests
    # in tools/rbb/src/tui.rs. `script` cannot reliably capture the
    # alt-screen buffer ratatui writes to.
else
    printf '  (skipping pty test — `script` not installed)\n'
fi

# 4. Without a pty, it must fail with a clean error (not a panic).
capture as_alice "rbb tui </dev/null"
assert_exit_nonzero "$CAP_CODE" "rbb tui without a pty fails fast"
assert_not_contains "$CAP_OUT" "panicked" "no panic traceback leaked"

scenario_summary
