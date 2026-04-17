# Admin guide

You are the person who runs the course server, onboards students, and writes content. There are two separate concerns: **infrastructure** (done once per server) and **content** (ongoing).

## One-time server setup

Fresh Linux box (Ubuntu/Debian tested). You have sudo.

```sh
git clone <your-mirror> /opt/rust-by-building
cd /opt/rust-by-building
sudo bash server/setup.sh
```

The script installs rustup system-wide, creates `/srv/rbb/` (shared docs, vendored crates, bare git repo), installs the `rbb` and `rbb-admin` binaries into `/usr/local/bin/`, and starts the local docs HTTP server on port 8000.

## Onboarding a student

```sh
rbb-admin user add alice
# Creates Linux user 'alice', home at /home/alice,
# clones /srv/rbb/rust-by-building.git into /home/alice/rust-by-building,
# writes .cargo/config.toml to use /srv/rbb/vendor,
# assigns port range 10100-10199,
# prints initial SSH setup instructions.
```

Remove a student:

```sh
rbb-admin user remove alice        # --keep-home to preserve their work
```

## Authoring new lessons

```sh
# 1. Scaffold:
rbb-admin lesson new 07 "error-handling"
# Creates:
#   book/src/07-error-handling.md           (with frontmatter)
#   lessons/07-error-handling/
#     exercises/                            (empty)
#     project/                              (Cargo.toml + src/lib.rs + tests/)
# And appends an entry to book/src/SUMMARY.md.

# 2. Add an exercise:
rbb-admin lesson add-exercise 07 "question-mark"
# Creates lessons/07-error-handling/exercises/ex1_question_mark.rs
# with `// TODO` markers and a `#[test]` so the student gets a clear signal.

# 3. Write the lesson text in book/src/07-error-handling.md.

# 4. Validate the whole course still compiles:
rbb-admin check
# Builds every exercise, runs every project's tests. Fails fast if you
# broke something.

# 5. Publish:
rbb-admin publish
# Pushes to /srv/rbb/rust-by-building.git on the server (via SSH remote).
# Students can now `git pull` to pick it up.
```

## Observing progress

```sh
rbb-admin progress
# alice       05/27 lessons   last active 2h ago
# bob         12/27 lessons   last active 5m ago
# carol        3/27 lessons   last active yesterday

rbb-admin progress bob --detailed
# Per-lesson status for bob.
```

This just reads `/home/*/.rbb/progress.json` — admins have filesystem read on student homes.

## Vendored dependencies

When you add a new crate to a lesson's `Cargo.toml`, rerun the vendor:

```sh
rbb-admin vendor sync
# Runs `cargo vendor --locked` against the workspace, copies to /srv/rbb/vendor/
# so student offline builds pick up the new crate.
```

This is the one step that requires the *admin's* machine to have internet. Students never need it.

## Ports

Each student gets a port range at onboarding: `RBB_PORT_BASE` in their `~/.rbb/env`. Lessons that need a port (capstones, mostly) pick `RBB_PORT_BASE + offset`. Tests that just need *any* free port bind `127.0.0.1:0`.

## File layout on the server

```
/opt/rust-by-building/                     # your source-of-truth checkout (for admin use)
/srv/rbb/
    rust-by-building.git                   # bare repo students pull from
    vendor/                                # shared vendored crates
    docs/                                  # rendered book + stdlib docs
/usr/local/bin/
    rbb                                    # student CLI
    rbb-admin                              # admin CLI
/home/<student>/
    rust-by-building/                      # their clone
    .rbb/
        progress.json                      # their state
        env                                # their port range
```
