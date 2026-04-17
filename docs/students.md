# Student guide

## Logging in

You have two options:

### Option A — Zed (recommended)

1. Install [Zed](https://zed.dev) on your laptop (one-time, requires download).
2. Open Zed → Project → Connect via SSH → enter the server your admin gave you.
3. Open the folder `~/rust-by-building/` on the remote.
4. You now have a full editor with rust-analyzer, running on the server.

Your laptop is just the display. All code, all tests, all `cargo` commands run on the server.

### Option B — SSH + editor

```sh
ssh <you>@<course-server>
cd ~/rust-by-building
hx lessons/01-hello/project/src/lib.rs    # helix is preinstalled
```

Helix has an interactive tutorial: `hx --tutor`.

## Your first session

```sh
rbb status                  # see all lessons and your progress
rbb open 01                 # read lesson 01
rbb watch 01                # start the feedback loop on lesson 01 exercises
                            # edit exercises/ex1_*.rs, save, see results
rbb test 01                 # when exercises pass, run the lesson's project tests
rbb next                    # jumps to the next thing you haven't finished
```

Your progress is stored in `~/.rbb/progress.json`. It's local to your account — no one else sees it unless you push a branch.

## Getting unstuck

1. **Read the compiler error.** Rust's errors are famously good. If there's a code like `E0382`, run `rustc --explain E0382` for a full tutorial.
2. **Look at the offline docs.** The Rust book and stdlib docs are served at `http://<server>:8000` — open in your browser. Also: `rustup doc` from the terminal.
3. **Check your progress.** `rbb status` will show you what you've completed and what's next.
4. **Ask your admin.** If you're really stuck, they can see the state of your repo directly.

## Updating the course

When the admin publishes new lessons:

```sh
cd ~/rust-by-building
git pull
rbb status                  # new lessons show as "new"
```

## What not to do

- Don't edit files under `.rbb/` directly — use the `rbb` CLI.
- Don't `cargo update` in the course projects — the vendored crates are pinned for offline use.
- Don't commit to `main`. If you want to save your work, push to a personal branch: `git checkout -b alice/lesson-03 && git push origin alice/lesson-03`.
