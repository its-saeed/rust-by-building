# Admin guide

You are the person who runs the course server, onboards students, writes content, and publishes updates. This guide is the reference; the [README](../README.md) has the quickstart.

## One-time server setup

Fresh Debian/Ubuntu box. You have sudo.

```sh
git clone <wherever-you-host-this> /opt/rust-by-building
cd /opt/rust-by-building
sudo bash server/setup.sh
```

`setup.sh` is interactive and will ask:

1. **Editor choice** — Helix (SSH-only students), Zed Remote (students use Zed on their laptops), or both.
   - Helix installs from apt and is tiny.
   - Zed Remote needs **nothing on the server** beyond SSH. Zed auto-installs its remote agent over SSH on first connect.
   - "Both" is the default: install Helix, trust students to pick.
2. **Student list** (optional) — a path to a file with one username per line. If you provide one, `setup.sh` runs `rbb-admin user bulk` against it at the end and writes credentials to `/root/rbb-credentials-*.txt` (mode 600).

What the script does, in order:
- apt-installs `build-essential`, `git`, `curl`, `openssh-server`, and (optionally) `helix`
- installs `rustup` into `/usr/local/cargo` system-wide
- builds + installs `rbb` and `rbb-admin` into `/usr/local/bin/`
- sets up `/srv/rbb/` with a bare git repo + shared vendor tree
- enables and starts `sshd`
- (optional) provisions the student list

Re-running is safe: every step is idempotent.

## Onboarding students

### One at a time

```sh
rbb-admin user add alice
# created user alice
#   home:        /home/alice
#   checkout:    /home/alice/rust-by-building
#   port range:  10100-10199
```

`user add` does NOT set a password. Set one with `passwd alice`, or use bulk mode below (which generates one).

### In a batch

Write a text file with one username per line (blank lines and `# comments` are ignored):

```
# students.txt
alice
bob
carol
dave
```

Then:

```sh
rbb-admin user bulk students.txt --credentials creds.txt
# wrote 4 credentials to creds.txt (mode 600)

cat creds.txt
# alice:XjK4mpQr3wVzNb
# bob:9HfAqWsD8eTcPx
# carol:...
# dave:...
```

`--credentials` is optional; if omitted, `username:password` pairs go to stdout. Passwords are 14-char base62 by default (`--password-length N` to change), generated from `/dev/urandom`, and ambiguous glyphs (l, 1, I, O, 0) are excluded.

Share `creds.txt` with students however you'd normally share a credentials list. **Delete it after distribution** — it's mode 600 but still lives on disk.

### Removing students

```sh
rbb-admin user remove alice              # userdel -r, home gone
rbb-admin user remove alice --keep-home  # userdel only, home preserved
```

### Listing students

```sh
rbb-admin user list
# alice
# bob
```

Scans `/home/*/rust-by-building/` — any Linux user whose home has a checkout shows up.

## Observing progress

```sh
rbb-admin progress
# alice      5/27  last active @1710000000
# bob       12/27  last active @1710000500
# carol      3/27  last active @1709999500
```

The `@NNN` timestamp is a Unix-epoch seconds marker — it's roughly the last time the student ran `rbb test` or `rbb check` successfully.

Filter to one user:

```sh
rbb-admin progress alice
```

Machine-readable JSON:

```sh
rbb-admin progress --json
rbb-admin progress alice --json
```

Per-user JSON includes `lessons_done`, `lessons_in_progress`, `lessons_total`, `last_active`, and a per-lesson map of `{status, project_passing, exercises_passed}`.

Under the hood this just reads `/home/*/.rbb/progress.json`. You have filesystem access, so nothing fancy is needed.

## Writing new lessons

### Scaffold

```sh
rbb-admin lesson new 07 error-handling
# creates:
#   lessons/07-error-handling/
#     README.md
#     exercises/                       (empty)
#     project/Cargo.toml
#     project/src/lib.rs
#     project/tests/smoke.rs
#   book/src/07-error-handling.md      (stub — write the chapter here)
```

Then add exercises one by one:

```sh
rbb-admin lesson add-exercise 07 question-mark
# creates lessons/07-error-handling/exercises/ex1_question-mark.rs
```

Write the chapter in `book/src/07-error-handling.md`. Model it on `book/src/03-functions.md`: explain a concept, point at exercises and project, keep it short.

`lesson new` also wires the chapter into `book/src/SUMMARY.md` automatically — it replaces any pre-listed entry for that lesson number, or appends a new one if none exists.

### Validate before publishing

```sh
rbb-admin check
```

Builds every package in the workspace and runs every project's test suite. Fails fast if any lesson's stubs are still `todo!()` or if a new lesson broke something.

### Publish

```sh
# From the admin checkout (commits must already be made):
rbb-admin publish
# [1/2] preflight: rbb-admin check
#   ...
# [2/2] pushing to /srv/rbb/rust-by-building.git (main)
# published
```

`publish` runs `check` as a preflight, then `git push` to the bare repo. `--skip-check` bypasses the preflight when you're iterating on the harness; never use it for content pushes. `--remote <path-or-name>` overrides the default (`/srv/rbb/rust-by-building.git`).

Students pick up changes with `git pull` in their checkout. If they're running `rbb watch`, tests rerun automatically.

## Adding or bumping dependencies

This is the one step that requires internet access on the admin's machine (or, in principle, a private crates mirror).

```sh
# 1. Edit the Cargo.toml that needs the change.
# 2. Re-vendor — runs cargo vendor and tells you what to commit next:
rbb-admin vendor-sync
# 3. Commit + publish:
git add Cargo.toml Cargo.lock vendor
git commit -m "bump: <what changed>"
rbb-admin publish
```

If `cargo build --offline` fails after a vendor-sync, the vendor tree is out of sync with `Cargo.lock` — re-run `rbb-admin vendor-sync`.

## Ports

Each student gets a deterministic port range at onboarding, written to `~/.rbb/env`:

```sh
cat /home/alice/.rbb/env
# RBB_PORT_BASE=10100
# RBB_PORT_END=10199
```

Formula: `10000 + (uid - 1000) * 100`, 100 ports per student. Enough headroom for any lesson's TCP server without collisions. Lessons that just need *any* free port should bind `127.0.0.1:0` and let the kernel pick.

## File layout on the server

```
/opt/rust-by-building/                  your source-of-truth checkout (for admin work)
/srv/rbb/
    rust-by-building.git                bare repo students pull from
    vendor/                             shared vendored crates (setup.sh copies from /opt)
    docs/                               rendered book + stdlib docs (optional HTTP serve)
/usr/local/bin/
    rbb                                 student CLI
    rbb-admin                           admin CLI
/home/<student>/
    rust-by-building/                   their clone
    .rbb/
        progress.json                   their state
        env                             their port range
```

## Regression tests

Before shipping changes to the admin CLI or lesson harness, run the e2e suite:

```sh
bash tests/e2e/run.sh
```

12 scenarios, docker-based, no network leaks. Scenario 10 covers the full user lifecycle; scenarios 01–09 cover the student flow; 11 runs offline; 12 smoke-tests the TUI.
