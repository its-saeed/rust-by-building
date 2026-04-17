#!/usr/bin/env bash
# Orchestrator: build the image once, run every scenario in its own
# container, report a pass/fail table at the end.

set -uo pipefail

IMAGE="${RBB_E2E_IMAGE:-rbb-e2e:latest}"
IMAGE_BARE="${RBB_E2E_IMAGE_BARE:-rbb-e2e-bare:latest}"
REPO_ROOT=$(cd "$(dirname "$0")/../.." && pwd)
SCENARIOS_DIR="$REPO_ROOT/tests/e2e/scenarios"

# Pick the image for a scenario by name. The bare image skips the
# provisioning the main image pre-bakes — used by pipeline tests
# that exercise setup.sh end to end.
image_for() {
    case "$1" in
        *pipeline*) echo "$IMAGE_BARE" ;;
        *)          echo "$IMAGE"      ;;
    esac
}

# Colors for this script's own output.
if [[ -t 1 ]]; then
    RED=$'\033[31m'
    GREEN=$'\033[32m'
    BOLD=$'\033[1m'
    DIM=$'\033[2m'
    RST=$'\033[0m'
else
    RED=''; GREEN=''; BOLD=''; DIM=''; RST=''
fi

printf '%s[build]%s %s\n' "$BOLD" "$RST" "$IMAGE"
if ! docker build -q -t "$IMAGE" -f "$REPO_ROOT/tests/e2e/Dockerfile" "$REPO_ROOT"; then
    printf '%sbuild failed%s\n' "$RED" "$RST" >&2
    exit 2
fi

# Build the bare image too — cheap when cached, skipped if no
# pipeline scenarios exist yet.
if ls "$SCENARIOS_DIR"/*pipeline*.sh >/dev/null 2>&1; then
    printf '%s[build]%s %s\n' "$BOLD" "$RST" "$IMAGE_BARE"
    if ! docker build -q -t "$IMAGE_BARE" -f "$REPO_ROOT/tests/e2e/Dockerfile.bare" "$REPO_ROOT"; then
        printf '%sbare build failed%s\n' "$RED" "$RST" >&2
        exit 2
    fi
fi

declare -a results

# Let the user run a single scenario by name: `./run.sh 03_test_passing`
if [[ $# -gt 0 ]]; then
    filter="$1"
    scenarios=()
    for s in "$SCENARIOS_DIR"/*.sh; do
        if [[ "$s" == *"$filter"* ]]; then
            scenarios+=("$s")
        fi
    done
    if [[ ${#scenarios[@]} -eq 0 ]]; then
        printf '%sno scenarios match%s %q\n' "$RED" "$RST" "$filter" >&2
        exit 2
    fi
else
    scenarios=("$SCENARIOS_DIR"/*.sh)
fi

for scenario in "${scenarios[@]}"; do
    name=$(basename "$scenario" .sh)
    img=$(image_for "$name")
    printf '\n%s[run]%s %s (image: %s)\n' "$BOLD" "$RST" "$name" "$img"

    # Scenarios whose name hints at offline run without a network.
    docker_args=(run --rm)
    if [[ "$name" == *offline* ]]; then
        docker_args+=(--network=none)
    fi

    if docker "${docker_args[@]}" "$img" bash "/e2e/scenarios/$(basename "$scenario")"; then
        results+=("$GREEN✓$RST  $name")
    else
        results+=("$RED✗$RST  $name")
    fi
done

printf '\n%s[summary]%s\n' "$BOLD" "$RST"
for r in "${results[@]}"; do
    printf '  %s\n' "$r"
done

# Exit non-zero if any scenario failed.
for r in "${results[@]}"; do
    if [[ "$r" == *"✗"* ]]; then
        exit 1
    fi
done
