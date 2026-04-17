#!/usr/bin/env bash
# Orchestrator: build the image once, run every scenario in its own
# container, report a pass/fail table at the end.

set -uo pipefail

IMAGE="${RBB_E2E_IMAGE:-rbb-e2e:latest}"
REPO_ROOT=$(cd "$(dirname "$0")/../.." && pwd)
SCENARIOS_DIR="$REPO_ROOT/tests/e2e/scenarios"

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
    printf '\n%s[run]%s %s\n' "$BOLD" "$RST" "$name"

    # Scenarios whose name hints at offline run without a network.
    docker_args=(run --rm)
    if [[ "$name" == *offline* ]]; then
        docker_args+=(--network=none)
    fi

    if docker "${docker_args[@]}" "$IMAGE" bash "/e2e/scenarios/$(basename "$scenario")"; then
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
