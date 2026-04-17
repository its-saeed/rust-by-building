#!/usr/bin/env bash
# Rust by Building — one-shot server provisioning.
#
# Run as root on a fresh Debian / Ubuntu box:
#
#   sudo bash server/setup.sh
#
# What it does:
#   1. Installs build prerequisites and the Rust toolchain system-wide.
#   2. Builds and installs rbb / rbb-admin into /usr/local/bin.
#   3. Sets up /srv/rbb/ with a bare git repo and a seeded vendor tree.
#   4. Installs a student-facing editor (helix — and/or leaves room for
#      Zed Remote, which needs nothing on the server beyond SSH).
#   5. Ensures sshd is running so students can log in.
#   6. Prints onboarding instructions.

set -euo pipefail

REPO_SRC="${REPO_SRC:-$(cd "$(dirname "$0")/.." && pwd)}"
SRV=/srv/rbb

if [[ $EUID -ne 0 ]]; then
    echo "setup.sh needs root — rerun with sudo."
    exit 1
fi

if [[ ! -f "$REPO_SRC/Cargo.toml" ]]; then
    echo "Can't find rust-by-building workspace at $REPO_SRC."
    echo "Run this script from the repo, or set REPO_SRC=/path/to/checkout."
    exit 1
fi

# ── interactive prompts ──────────────────────────────────────────────

# Editor choice. Both options use the same SSH path; the question is
# whether we pre-install helix on the server for students who prefer an
# SSH-and-terminal workflow.
EDITOR_CHOICE="${EDITOR_CHOICE:-}"
if [[ -z "$EDITOR_CHOICE" && -t 0 ]]; then
    cat <<'EOF'

Which editor setup should students use?

  [1] Helix (installed on the server, runs inside SSH)
      Students SSH in and use `hx`. No local install on their machine.

  [2] Zed Remote (students run Zed on their laptop, connect via SSH)
      Needs nothing extra on the server — Zed auto-installs its remote
      agent over SSH the first time a student connects.

  [3] Both — install Helix AND document the Zed path (recommended)

EOF
    read -rp "Choice [1/2/3, default 3]: " EDITOR_CHOICE
    EDITOR_CHOICE="${EDITOR_CHOICE:-3}"
fi

# Whether to run the initial student-list provisioning after setup.
STUDENTS_FILE="${STUDENTS_FILE:-}"
if [[ -z "$STUDENTS_FILE" && -t 0 ]]; then
    read -rp "Path to a students list file (blank to skip): " STUDENTS_FILE
fi

# ── the work ──────────────────────────────────────────────────────────

echo
echo "[1/6] install apt prerequisites"
export DEBIAN_FRONTEND=noninteractive
apt-get update -q
apt-get install -y -q \
    build-essential \
    ca-certificates \
    curl \
    git \
    openssh-server

if [[ "$EDITOR_CHOICE" == "1" || "$EDITOR_CHOICE" == "3" ]]; then
    # Helix landed in debian/ubuntu repos in 2024-ish. If the package isn't
    # available the admin can install it manually later.
    apt-get install -y -q helix || echo "  (couldn't find helix in apt — install it manually if students want it)"
fi

echo "[2/6] install rustup system-wide"
if ! command -v rustup >/dev/null; then
    export CARGO_HOME=/usr/local/cargo
    export RUSTUP_HOME=/usr/local/rustup
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
        | sh -s -- -y --no-modify-path --default-toolchain stable
    ln -sf /usr/local/cargo/bin/rustc  /usr/local/bin/rustc
    ln -sf /usr/local/cargo/bin/cargo  /usr/local/bin/cargo
    ln -sf /usr/local/cargo/bin/rustup /usr/local/bin/rustup
    # Make the env available to new shells.
    cat > /etc/profile.d/rbb-cargo.sh <<'EOF'
export CARGO_HOME=/usr/local/cargo
export RUSTUP_HOME=/usr/local/rustup
export PATH=/usr/local/cargo/bin:$PATH
EOF
fi

echo "[3/6] build + install rbb / rbb-admin"
pushd "$REPO_SRC" >/dev/null
cargo build --frozen --release -p rbb -p rbb-admin
install -m 0755 target/release/rbb        /usr/local/bin/rbb
install -m 0755 target/release/rbb-admin  /usr/local/bin/rbb-admin
popd >/dev/null

echo "[4/6] create $SRV layout"
mkdir -p "$SRV/docs" "$SRV/vendor"
# Seed the shared vendor/ so per-student clones build offline without
# each one duplicating 100+ MB of crates.
cp -r "$REPO_SRC/vendor/." "$SRV/vendor/"

if [[ ! -d "$SRV/rust-by-building.git" ]]; then
    git init --quiet --bare --initial-branch=main "$SRV/rust-by-building.git"
fi

# Git's safe.directory check rejects a repo whose owner UID differs
# from the caller's. On the course server the admin's checkout and the
# bare repo are often touched by different accounts (student clones,
# root pushes, admin publishes). Whitelist them system-wide so nobody
# has to remember the incantation.
git config --system --add safe.directory "$REPO_SRC"
git config --system --add safe.directory "$SRV/rust-by-building.git"

# Seed the bare repo from the admin's checkout if it's a git repo.
# Let errors surface — a silent failure leaves the bare repo empty,
# which breaks every student clone downstream.
if [[ -d "$REPO_SRC/.git" ]]; then
    git --git-dir="$REPO_SRC/.git" --work-tree="$REPO_SRC" \
        push --quiet "$SRV/rust-by-building.git" +HEAD:refs/heads/main
fi

echo "[5/6] enable sshd"
systemctl enable --now ssh >/dev/null 2>&1 || systemctl enable --now sshd >/dev/null 2>&1 || true

echo "[6/6] done"

# ── optional: bulk-onboard students ──────────────────────────────────

if [[ -n "$STUDENTS_FILE" && -f "$STUDENTS_FILE" ]]; then
    creds=/root/rbb-credentials-$(date +%Y%m%d-%H%M%S).txt
    echo
    echo "Provisioning students from $STUDENTS_FILE..."
    rbb-admin user bulk "$STUDENTS_FILE" \
        --from "$SRV/rust-by-building.git" \
        --credentials "$creds"
    echo "Credentials written to $creds (mode 600)."
fi

# ── summary ──────────────────────────────────────────────────────────

cat <<EOF

✓ Rust by Building server is ready.

Repo (bare):   $SRV/rust-by-building.git
Admin CLI:     /usr/local/bin/rbb-admin
Student CLI:   /usr/local/bin/rbb

Next steps:

  # Add one student (interactive):
  rbb-admin user add alice

  # Add many at once, output credentials:
  rbb-admin user bulk students.txt --credentials creds.txt

  # Push new or updated lessons to the bare repo:
  cd $REPO_SRC
  git push $SRV/rust-by-building.git main

Editor for students:
EOF

case "$EDITOR_CHOICE" in
    1)  echo "  Helix is installed. Students run 'hx <file>' over SSH."                ;;
    2)  echo "  Students install Zed on their laptop and use 'Connect via SSH'."        ;;
    3|*)echo "  Helix installed for SSH-only students; Zed Remote also supported."     ;;
esac

echo
