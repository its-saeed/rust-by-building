#!/usr/bin/env bash
# Rust by Building — one-shot server provisioning.
# Run as root on a fresh Ubuntu/Debian box.

set -euo pipefail

SRV=/srv/rbb
REPO_SRC="${REPO_SRC:-/opt/rust-by-building}"

echo "[1/6] install system toolchain"
apt-get update
apt-get install -y build-essential curl git pkg-config libssl-dev

echo "[2/6] install rustup system-wide"
if ! command -v rustup >/dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
        | sh -s -- -y --no-modify-path --default-toolchain stable
    ln -sf "$HOME/.cargo/bin/rustc" /usr/local/bin/rustc
    ln -sf "$HOME/.cargo/bin/cargo" /usr/local/bin/cargo
    ln -sf "$HOME/.cargo/bin/rustup" /usr/local/bin/rustup
fi

echo "[3/6] preinstall helix editor"
# Replace with apt-get install helix when available on your distro.
# On recent Debian / Ubuntu 24.04+:
apt-get install -y helix || echo "install helix manually if not in apt"

echo "[4/6] create /srv/rbb layout"
mkdir -p "$SRV"
mkdir -p "$SRV/docs"

# Bare repo students pull from
if [[ ! -d "$SRV/rust-by-building.git" ]]; then
    git init --bare "$SRV/rust-by-building.git"
fi

# Seed it from the admin's checkout
if [[ -d "$REPO_SRC/.git" ]]; then
    git --git-dir="$REPO_SRC/.git" --work-tree="$REPO_SRC" push \
        "$SRV/rust-by-building.git" +main 2>/dev/null || true
fi

echo "[5/6] vendor crates"
if [[ -d "$REPO_SRC" ]]; then
    pushd "$REPO_SRC" >/dev/null
    cargo vendor --locked "$SRV/vendor"
    popd >/dev/null
fi

echo "[6/6] install rbb + rbb-admin"
if [[ -d "$REPO_SRC" ]]; then
    pushd "$REPO_SRC" >/dev/null
    cargo build --release -p rbb -p rbb-admin
    install -m 0755 target/release/rbb        /usr/local/bin/rbb
    install -m 0755 target/release/rbb-admin  /usr/local/bin/rbb-admin
    popd >/dev/null
fi

echo
echo "✓ server is ready"
echo "  add your first student with: rbb-admin user add <name>"
