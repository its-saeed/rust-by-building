# Rust by Building

A learn-Rust-from-scratch course designed for offline use on a shared server. No internet access required — everything runs on a single Linux box students SSH into.

Two kinds of people work with this repo:

- **Students** — log into the server, follow lessons, do exercises and projects. Read [`docs/students.md`](./docs/students.md).
- **Admins** — set up the server, onboard students, write and publish new lessons. Read [`docs/admins.md`](./docs/admins.md).

## What's in here

```
book/          Lesson text (rendered with mdbook, served locally on the server)
lessons/       Per-lesson exercises + small projects with boilerplate and tests
capstones/     Phase-end larger projects
tools/         Rust workspace: rbb (student CLI), rbb-admin (admin CLI), rbb-core (shared)
server/        Provisioning scripts and systemd units
vendor/        Vendored crates so students can build offline (populated by admin)
```

## Quick start

If you're a student: your admin has already created your account and set up your environment. Just log in and run `rbb status`.

If you're an admin setting up for the first time: run `server/setup.sh` on a fresh Linux box, then add your first student with `rbb-admin user add <name>`.
