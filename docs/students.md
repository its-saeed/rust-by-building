# Student guide

Welcome. Your admin has set up a server, created an account for you, and given you:

- A **server address** (e.g. `course.yourorg.internal`)
- A **username** (your name, probably)
- A **password**

This page gets you from those three things to writing your first Rust.

## Step 1 — log in

You have two choices. Both work. Pick one.

### Option A — Zed (recommended if you have your own laptop)

[Zed](https://zed.dev) is a modern code editor with full Rust support. It runs on your laptop, but it can edit files on the server as if they were local.

1. Install Zed on your laptop. One-time.
2. Open Zed, go to **Project → Connect via SSH**.
3. Enter the server address and your username. Zed asks for your password.
4. Open the folder `~/rust-by-building` on the remote.

From now on, Zed's terminal is the server's shell, and the file tree is your server checkout. You get autocomplete, inline compiler errors, jump-to-definition — all via `rust-analyzer` running on the server. Your laptop is just the display.

### Option B — SSH + Helix

If you don't want to install anything locally, or you're on a borrowed machine, just SSH in:

```sh
ssh <you>@<course-server>
cd ~/rust-by-building
hx lessons/03-functions/project/src/lib.rs
```

`hx` is [Helix](https://helix-editor.com). It's preinstalled on the server. It has a built-in tutorial:

```sh
hx --tutor
```

If you're used to `vim`, Helix's keybindings will feel familiar after a short adjustment.

## Step 2 — your first `rbb`

All course interaction goes through one command: `rbb`.

```sh
rbb status
# id  lesson                   ex   proj  status
# 01  hello                    0/0  -     not started
# 03  functions                0/4  -     not started
```

Read the lesson you want to work on:

```sh
rbb open 03
```

This just prints the lesson's `README.md` to your terminal. For the chapter itself, see `book/src/NN-<slug>.md`.

Start the feedback loop:

```sh
rbb watch 03
```

The screen clears and tests run. Every time you save a file in the lesson, they rerun. Fix until green. `Ctrl-C` when done.

When the project tests are green:

```sh
rbb test 03
# all tests passed
```

This records your completion in `~/.rbb/progress.json`.

Check your progress anytime:

```sh
rbb status       # text table
rbb tui          # full-screen dashboard (q to quit)
rbb next         # tells you where to pick up
```

## Step 3 — a typical session

```sh
rbb next
# next up: lesson 03 — functions
#   rbb open 03

rbb open 03             # read the chapter
hx lessons/03-functions/exercises/ex1_return_value.rs
                        # fix the broken exercise
rbb check 03            # compile + run every exercise in lesson 03
hx lessons/03-functions/project/src/lib.rs
                        # work on the lesson's project
rbb watch 03            # feedback loop on the project tests
                        # edit, save, see results, loop. Ctrl-C to stop.
rbb test 03             # final check, records progress
rbb next                # what's next
```

## When you're stuck

In this order:

1. **Read the compiler error from the bottom up.** Rust's errors are exceptionally good. They tell you exactly what to do.
2. Run `rustc --explain EXXXX` where `EXXXX` is the error code (e.g. `E0382`). You get a full tutorial on that specific error.
3. Read the offline Rust book at `/srv/rbb/docs/` on the server (your admin may have served it over HTTP too).
4. **Ask your admin.** They can see the state of your checkout directly.

## Updating the course

When your admin publishes new lessons:

```sh
cd ~/rust-by-building
git pull
rbb status    # new lessons show up as "not started"
```

Nothing of yours gets overwritten — your `~/.rbb/progress.json` is local to your home and not part of the course repo.

## What not to do

- Don't edit `~/.rbb/progress.json` by hand. Use `rbb` instead.
- Don't `cargo update`. The vendor tree is pinned to the exact versions the course expects; updating it offline will fail.
- Don't commit to `main`. If you want to save snapshots of your work:

```sh
git checkout -b my-work
git add -A && git commit -m "my attempt at lesson 05"
```

You can push that branch to the bare repo on the server; check with your admin for the path.

## What `rbb` can do, one-liner per command

| | |
|---|---|
| `rbb status`          | table of all lessons + your progress |
| `rbb tui`             | full-screen interactive dashboard |
| `rbb open <n>`        | print lesson N's README |
| `rbb watch <n>`       | rerun lesson N's project tests on every save |
| `rbb test <n>`        | one-shot: run lesson N's project tests |
| `rbb check <n>`       | compile + run every exercise in lesson N |
| `rbb next`            | tell me what to work on next |

Good luck.
