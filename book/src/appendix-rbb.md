# Using `rbb` effectively

`rbb` is the student CLI. Every course interaction goes through it. This page covers all commands and some habits that make the feedback loop faster.

---

## Command reference

| Command | What it does |
|---------|-------------|
| `rbb status` | Table of all lessons — title, exercise progress, project status |
| `rbb tui` | Full-screen dashboard, updated live. Press `q` to quit |
| `rbb next` | Print the next unfinished lesson and the command to open it |
| `rbb open <n>` | Print lesson N's README to the terminal |
| `rbb watch <n>` | Run lesson N's project tests on every file save — the main feedback loop |
| `rbb test <n>` | Run lesson N's project tests once and record completion |
| `rbb check <n>` | Compile and run every exercise file in lesson N |

---

## The feedback loop

The single most important command is `rbb watch`:

```sh
rbb watch 05
```

The screen clears. Tests run. Every time you save any file in lesson 05, they run again. You see the compiler output immediately — red until you fix it, green when you are done. Press `Ctrl-C` to stop.

Use `rbb watch` while working. Use `rbb test` once at the end to record your completion.

---

## Exercises vs projects

Each lesson has two kinds of work:

- **Exercises** — small isolated files in `lessons/NN-slug/exercises/`. Each one compiles independently. `rbb check <n>` runs all of them.
- **Project** — a full Cargo project in `lessons/NN-slug/project/`. `rbb watch <n>` and `rbb test <n>` run its tests.

Exercises come first. They are short drills on the lesson's concept. The project applies everything together.

---

## Seeing where you are

```sh
rbb status
```

```
id  lesson              exercises  project   status
01  caesar              4/4        done      complete
02  contact-book        0/3        -         not started
03  cipher-cracker      2/6        in prog   in progress
```

`rbb next` cuts straight to the point:

```sh
rbb next
# next up: lesson 02 — contact-book
#   rbb open 02
```

---

## Progress is local

Your progress is stored in `~/.rbb/progress.json` on the course server, inside your home directory. It is not part of the course repo — `git pull` never touches it and you cannot accidentally lose it by updating lessons.

Do not edit it by hand. If something looks wrong, ask your admin.

---

## A typical session

```sh
rbb next                        # where am I?
rbb open 04                     # read the lesson chapter
rbb check 04                    # work through exercises until green
rbb watch 04                    # open the project, edit, watch tests
rbb test 04                     # record completion
rbb next                        # what is next
```

---

## Tips

**Start with `rbb open`**, not by jumping straight into code. The chapter explains the concept; the exercises and project assume you have read it.

**Read the full compiler error.** Rust's errors point at the exact line and often suggest the exact fix. Read from the bottom — the last message is usually the root cause.

**`rustc --explain EXXXX`** gives a deep dive on any error code. If you see `error[E0382]`, run `rustc --explain E0382` for a full explanation with examples.

**Do not `cargo update`**. The vendor tree is pinned. Updating it offline will break things.

**Save your own work on a branch** if you want snapshots:

```sh
git checkout -b my-progress
git add -A && git commit -m "lesson 03 done"
```

The `main` branch belongs to the course. Your branch is yours.
