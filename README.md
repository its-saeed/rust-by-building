# Rust by Building

**[Read the book →](https://its-saeed.github.io/rust-by-building/)**

Learn Rust from scratch by writing small programs that get progressively more interesting. Designed to run on a single offline Linux server: an admin sets it up once, students SSH in and work.

No crates.io access, no browser full of tabs — just a book, exercises, and projects, all local.

There are two kinds of people here:

| | You are… | Start with |
|---|---|---|
| 🧑‍🏫 | An **admin** standing up a course server | [`docs/admins.md`](./docs/admins.md) — TL;DR below |
| 🧑‍💻 | A **student** with login credentials | [`docs/students.md`](./docs/students.md) — TL;DR below |

---

## Admin TL;DR

```sh
# On a fresh Debian/Ubuntu server, as root:
git clone <wherever-you-host-this> /opt/rust-by-building
cd /opt/rust-by-building
sudo bash server/setup.sh
```

`setup.sh` is interactive. It asks which editor to install (Helix, Zed Remote, or both) and optionally runs bulk onboarding against a student list.

Then provision students. Either one-off:

```sh
rbb-admin user add alice
# Prints home / checkout / port range
```

…or in batch from a `students.txt` (one name per line):

```sh
rbb-admin user bulk students.txt --credentials creds.txt
# wrote 12 credentials to creds.txt (mode 600)
```

`creds.txt` is a mode-600 file with `username:password` pairs — hand each row to the corresponding student however you normally share credentials.

To publish new or updated lessons to students:

```sh
rbb-admin publish -m "add lesson 05: closures"
```

One command: validates, commits, pushes the admin repo, and exports filtered content (no tool source) to the student-facing repo. Students `git pull` to get it.

See **[`docs/admins.md`](./docs/admins.md)** for writing new lessons, adding crates, observing progress, and rotating the student list.

## Student TL;DR
## Student TL;DR
## Student TL;DR

Your admin gave you a username, password, and a server address. Log in:

```sh
ssh <you>@<course-server>
```

…then use either editor:

- **Helix** (preinstalled): `hx lessons/03-functions/project/src/lib.rs`
- **[Zed](https://zed.dev)** (install locally on your laptop): open the project's "Connect via SSH", pick this server, open `~/rust-by-building/`. Zed does the rest.

First thing to run, either way:

```sh
rbb status      # list lessons, see what you've done
rbb open 01     # read lesson 01
rbb watch 01    # feedback loop — edit, save, see tests rerun
rbb next        # jump to the next unfinished lesson
```

For a full-screen dashboard instead of text:

```sh
rbb tui
```

See **[`docs/students.md`](./docs/students.md)** for the longer story.

---

## What's in this repo

```
book/          lesson chapters (rendered with mdbook, served locally)
lessons/       per-lesson exercises + projects with boilerplate + tests
tools/         rust workspace:
                 rbb         student CLI
                 rbb-admin   admin CLI
                 rbb-core    shared types & helpers
vendor/        checked-in cargo vendor tree (offline builds)
server/        setup.sh — provisions a fresh server in one shot
tests/e2e/     dockerized regression suite (12 scenarios)
```

## Development

If you're the admin *building this*, not deploying it, regression tests live in Docker:

```sh
bash tests/e2e/run.sh           # build image, run all scenarios
bash tests/e2e/run.sh 03_test   # run just matching scenarios
```

12 scenarios cover status / test / watch / check / open / next on the student side, and lesson scaffolding / user lifecycle / progress / offline builds on the admin side.

## Why offline

This course was built for environments where crates.io and the wider internet are unreliable or unavailable. Everything a student needs — the toolchain, docs, every crate the lessons depend on — lives on the server.

Consequences:

- `vendor/` is committed (~120 MB, 70+ crates). Adding a dependency means re-running `cargo vendor` and committing the result. See [`docs/admins.md`](./docs/admins.md).
- `.cargo/config.toml` hard-redirects `crates-io` to the vendor tree.
- `cargo build --frozen` is the norm; `cargo update` is never run on the server.
- The docs/admins.md and docs/students.md assume the server is reachable but nothing beyond it is.
