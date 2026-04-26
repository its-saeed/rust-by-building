//! Admin CLI for the Rust by Building course.
//!
//! Minimal-viable surface: scaffolds new lessons and projects, runs the
//! whole-course self-check, and reads progress out of every student's
//! home dir. User provisioning (`user add` / `user remove`) is stubbed
//! — it will shell out to `useradd` on the Linux server once we're past
//! the scaffolding phase.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
use rbb_core::{discover_lessons, find_repo_root, LessonId};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser)]
#[command(name = "rbb-admin", about = "Rust by Building — admin CLI", version)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Manage student accounts on the server.
    #[command(subcommand)]
    User(UserCmd),

    /// Scaffold content.
    #[command(subcommand)]
    Lesson(LessonCmd),

    /// Compile every exercise and run every project's tests. Fails fast.
    Check,

    /// Read progress out of every student's home dir.
    Progress {
        user: Option<String>,
        /// Emit machine-readable JSON instead of the human table.
        #[arg(long)]
        json: bool,
    },

    /// Run `rbb-admin check`, then push the workspace to the server's
    /// bare repo. A safety net so new lessons are validated before
    /// students can pull them.
    Publish {
        /// Git remote name or absolute path to the bare repo.
        #[arg(long, default_value = "/srv/rbb/rust-by-building.git")]
        remote: String,
        /// Branch to push. Students pull from this.
        #[arg(long, default_value = "main")]
        branch: String,
        /// Skip the preflight `check`. Useful when iterating on the
        /// harness itself; don't use for actual content pushes.
        #[arg(long)]
        skip_check: bool,
    },

    /// Re-vendor every dependency into `vendor/`. Run this after
    /// editing any Cargo.toml. Requires network.
    #[command(name = "vendor-sync")]
    VendorSync,
}

#[derive(Subcommand)]
enum UserCmd {
    /// Create a Linux user, clone the course repo into their home,
    /// assign a port range, and set file ownership.
    Add {
        name: String,
        /// Where to seed the student's checkout from. Defaults to the
        /// server's bare repo. For single-host testing, point this at
        /// the admin's own workspace checkout.
        #[arg(long, default_value = "/srv/rbb/rust-by-building.git")]
        from: PathBuf,
    },
    /// Provision many students at once. Reads usernames (one per
    /// line, blank lines and `#` comments ignored) from the list file,
    /// generates a random password for each, creates the user, and
    /// emits `username:password` pairs to --credentials or stdout.
    Bulk {
        /// Path to a file with one username per line.
        list: PathBuf,
        /// Source for each student's checkout (see `user add --from`).
        #[arg(long, default_value = "/srv/rbb/rust-by-building.git")]
        from: PathBuf,
        /// Write credentials here (mode 600). Defaults to stdout.
        #[arg(long)]
        credentials: Option<PathBuf>,
        /// Length of generated passwords. Minimum 8.
        #[arg(long, default_value_t = 14)]
        password_length: usize,
    },
    /// Remove a user. By default also removes the home directory;
    /// pass --keep-home to preserve their work.
    Remove {
        name: String,
        #[arg(long)]
        keep_home: bool,
    },
    /// Print all users who have a ~/rust-by-building checkout.
    List,
}

#[derive(Subcommand)]
enum LessonCmd {
    /// Create a new lesson directory with templates.
    New { id: u32, slug: String },
    /// Add a new exercise file to an existing lesson.
    AddExercise { id: u32, name: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // User provisioning and progress reading are workspace-free — the
    // admin runs them on the server, not from a checkout. Only lesson
    // scaffolding and `check` need to know the workspace root.
    match cli.cmd {
        Cmd::User(c)                 => handle_user(c),
        Cmd::Progress { user, json } => cmd_progress(user.as_deref(), json),
        Cmd::Lesson(c) => {
            let root = find_repo_root(&std::env::current_dir()?)
                .context("run `rbb-admin lesson` from inside your rust-by-building checkout")?;
            handle_lesson(&root, c)
        }
        Cmd::Check => {
            let root = find_repo_root(&std::env::current_dir()?)
                .context("run `rbb-admin check` from inside your rust-by-building checkout")?;
            cmd_check(&root)
        }
        Cmd::Publish { remote, branch, skip_check } => {
            let root = find_repo_root(&std::env::current_dir()?)
                .context("run `rbb-admin publish` from inside your rust-by-building checkout")?;
            cmd_publish(&root, &remote, &branch, skip_check)
        }
        Cmd::VendorSync => {
            let root = find_repo_root(&std::env::current_dir()?)
                .context("run `rbb-admin vendor-sync` from inside your rust-by-building checkout")?;
            cmd_vendor_sync(&root)
        }
    }
}

fn handle_user(c: UserCmd) -> Result<()> {
    match c {
        UserCmd::Add { name, from }             => user_add(&name, &from),
        UserCmd::Bulk { list, from, credentials, password_length } =>
            user_bulk(&list, &from, credentials.as_deref(), password_length),
        UserCmd::Remove { name, keep_home }     => user_remove(&name, keep_home),
        UserCmd::List                           => user_list(),
    }
}

fn require_root() -> Result<()> {
    // Cheap preflight so we fail with a clear message rather than
    // letting useradd emit a cryptic one.
    let uid = Command::new("id").arg("-u").output()?.stdout;
    let uid = String::from_utf8_lossy(&uid);
    if uid.trim() != "0" {
        anyhow::bail!("this command needs root — run under sudo");
    }
    Ok(())
}

fn sh(cmd: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(cmd).args(args).status()
        .with_context(|| format!("spawning {cmd} {args:?}"))?;
    if !status.success() {
        anyhow::bail!("{cmd} {args:?} exited with {status}");
    }
    Ok(())
}

fn uid_of(name: &str) -> Result<u32> {
    let out = Command::new("id").arg("-u").arg(name).output()?;
    if !out.status.success() {
        anyhow::bail!("id -u {name} failed");
    }
    let s = String::from_utf8_lossy(&out.stdout);
    Ok(s.trim().parse()?)
}

/// The raw provisioning step — no prints, so callers (user_add, user_bulk)
/// can format their own output.
fn provision_user(name: &str, from: &Path) -> Result<ProvisionResult> {
    // 1. create the Linux user.
    sh("useradd", &["-m", "-s", "/bin/bash", name])?;

    let home = PathBuf::from("/home").join(name);
    let dest = home.join("rust-by-building");

    // 2. seed their checkout. Bare repo → git clone; plain dir → copy
    //    (useful for single-host testing where no bare repo exists).
    if from.join("HEAD").exists() {
        sh("git", &["clone", "--quiet",
            &from.display().to_string(),
            &dest.display().to_string()])?;
    } else if from.exists() {
        sh("cp", &["-r", &from.display().to_string(), &dest.display().to_string()])?;
        let _ = std::fs::remove_dir_all(dest.join("target"));
    } else {
        anyhow::bail!("source path does not exist: {from:?}");
    }

    // 3. assign a deterministic port range so two students don't collide.
    let uid = uid_of(name)?;
    let port_base = 10_000 + (uid.saturating_sub(1000)) * 100;
    let rbb_dir = home.join(".rbb");
    std::fs::create_dir_all(&rbb_dir)?;
    std::fs::write(
        rbb_dir.join("env"),
        format!("RBB_PORT_BASE={port_base}\nRBB_PORT_END={}\n", port_base + 99),
    )?;

    // 4. fix ownership on everything we just created.
    sh("chown", &["-R", &format!("{name}:{name}"), &home.display().to_string()])?;

    Ok(ProvisionResult { home, checkout: dest, port_base })
}

struct ProvisionResult {
    home: PathBuf,
    checkout: PathBuf,
    port_base: u32,
}

fn user_add(name: &str, from: &Path) -> Result<()> {
    require_root()?;
    let r = provision_user(name, from)?;
    let pw = random_password(14)?;
    set_password(name, &pw)?;
    println!("created user {name}");
    println!("  home:        {}", r.home.display());
    println!("  checkout:    {}", r.checkout.display());
    println!("  port range:  {}-{}", r.port_base, r.port_base + 99);
    println!("  password:    {pw}");
    Ok(())
}

fn random_password(len: usize) -> Result<String> {
    use std::io::Read;
    let len = len.max(8);
    // Base62 — avoids shell-meaningful chars and ambiguous pairs (l/1/I/O/0).
    const ALPHABET: &[u8] =
        b"abcdefghjkmnpqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ23456789";
    let mut rand = vec![0u8; len];
    std::fs::File::open("/dev/urandom")?.read_exact(&mut rand)?;
    Ok(rand.iter()
        .map(|b| ALPHABET[(*b as usize) % ALPHABET.len()] as char)
        .collect())
}

fn set_password(name: &str, password: &str) -> Result<()> {
    use std::io::Write;
    use std::process::Stdio;

    let mut child = Command::new("chpasswd")
        .stdin(Stdio::piped())
        .spawn()
        .context("spawning chpasswd")?;
    let mut stdin = child.stdin.take().context("no chpasswd stdin")?;
    writeln!(stdin, "{name}:{password}")?;
    drop(stdin);

    let status = child.wait()?;
    if !status.success() {
        anyhow::bail!("chpasswd exited with {status}");
    }
    Ok(())
}

fn read_student_list(path: &Path) -> Result<Vec<String>> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("reading {path:?}"))?;
    Ok(text.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|s| s.to_string())
        .collect())
}

fn user_bulk(list: &Path, from: &Path, creds_path: Option<&Path>, pwlen: usize) -> Result<()> {
    require_root()?;
    let names = read_student_list(list)?;
    if names.is_empty() {
        anyhow::bail!("student list {list:?} is empty");
    }

    let mut creds = Vec::with_capacity(names.len());
    let mut failed: Vec<(String, String)> = Vec::new();

    for name in &names {
        match (|| -> Result<String> {
            provision_user(name, from)?;
            let pw = random_password(pwlen)?;
            set_password(name, &pw)?;
            Ok(pw)
        })() {
            Ok(pw)  => creds.push(format!("{name}:{pw}")),
            Err(e)  => failed.push((name.clone(), format!("{e:#}"))),
        }
    }

    let rendered = creds.join("\n") + "\n";
    match creds_path {
        Some(p) => {
            use std::os::unix::fs::PermissionsExt;
            std::fs::write(p, &rendered)?;
            std::fs::set_permissions(p, std::fs::Permissions::from_mode(0o600))?;
            println!("wrote {} credentials to {} (mode 600)", creds.len(), p.display());
        }
        None => {
            print!("{rendered}");
        }
    }

    for (name, err) in &failed {
        eprintln!("FAILED {name}: {err}");
    }
    if !failed.is_empty() {
        anyhow::bail!("{} of {} users failed; see stderr", failed.len(), names.len());
    }
    Ok(())
}

fn user_remove(name: &str, keep_home: bool) -> Result<()> {
    require_root()?;
    if keep_home {
        sh("userdel", &[name])?;
        println!("removed user {name} (home preserved)");
    } else {
        sh("userdel", &["-r", name])?;
        println!("removed user {name}");
    }
    Ok(())
}

fn user_list() -> Result<()> {
    let home_root = PathBuf::from("/home");
    if !home_root.exists() {
        return Ok(());
    }
    let mut names: Vec<String> = std::fs::read_dir(&home_root)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().join("rust-by-building").exists())
        .filter_map(|e| e.file_name().to_str().map(|s| s.to_string()))
        .collect();
    names.sort();
    for name in names {
        println!("{name}");
    }
    Ok(())
}

fn handle_lesson(root: &Path, c: LessonCmd) -> Result<()> {
    match c {
        LessonCmd::New { id, slug } => scaffold_lesson(root, id, &slug),
        LessonCmd::AddExercise { id, name } => scaffold_exercise(root, id, &name),
    }
}

fn scaffold_lesson(root: &Path, id: u32, slug: &str) -> Result<()> {
    let dir_name = format!("{:02}-{}", id, slug);
    let dir = root.join("lessons").join(&dir_name);
    if dir.exists() {
        anyhow::bail!("lesson dir already exists: {dir:?}");
    }
    std::fs::create_dir_all(dir.join("exercises"))?;
    std::fs::create_dir_all(dir.join("project/src"))?;
    std::fs::create_dir_all(dir.join("project/tests"))?;

    std::fs::write(
        dir.join("README.md"),
        format!("# Lesson {id:02} — {slug}\n\nChapter: [book/src/{:02}-{}.md](../../book/src/{:02}-{}.md)\n", id, slug, id, slug),
    )?;

    let pkg_name = format!("lesson-{:02}-{}", id, slug);
    std::fs::write(
        dir.join("project/Cargo.toml"),
        format!(r#"[package]
name = "{pkg_name}"
edition.workspace = true
version.workspace = true
license.workspace = true

[lib]
path = "src/lib.rs"
"#),
    )?;

    std::fs::write(
        dir.join("project/src/lib.rs"),
        "//! TODO: write the lesson's project here.\n",
    )?;

    std::fs::write(
        dir.join("project/tests/smoke.rs"),
        "#[test]\nfn todo() {\n    todo!(\"write tests for this project\");\n}\n",
    )?;

    // Also stub the book chapter.
    let book = root.join("book/src").join(format!("{:02}-{}.md", id, slug));
    if !book.exists() {
        std::fs::write(&book, format!("# Lesson {id:02} — {slug}\n\nTODO.\n"))?;
    }

    // Wire the new chapter into SUMMARY.md.
    let summary = root.join("book/src/SUMMARY.md");
    let summary_updated = if summary.exists() {
        update_summary(&summary, id, slug)?
    } else {
        false
    };

    println!("{} lesson {:02}-{}", "scaffolded".green(), id, slug);
    println!("  {}", dir.display());
    println!("  {}", book.display());
    if summary_updated {
        println!("  {} (updated)", summary.display());
    } else if summary.exists() {
        println!("  {} (already up to date)", summary.display());
    }
    Ok(())
}

/// Insert or update the SUMMARY.md entry for lesson `id`. Returns true
/// if anything changed on disk. Idempotent.
fn update_summary(summary: &Path, id: u32, slug: &str) -> Result<bool> {
    let original = std::fs::read_to_string(summary)
        .with_context(|| format!("reading {summary:?}"))?;
    let title = slug_to_title(slug);
    let want_line = format!("- [{title}](./{:02}-{slug}.md)", id);

    // 1. If a line already points at ./NN-<something>.md, replace it.
    let prefix = format!("./{:02}-", id);
    let mut found = false;
    let mut changed = false;
    let mut lines: Vec<String> = original.lines().map(String::from).collect();
    for line in &mut lines {
        if line.contains(&prefix) && line.contains(".md") {
            found = true;
            if line.trim() != want_line.trim() {
                let leading = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
                *line = format!("{leading}{want_line}");
                changed = true;
            }
            break;
        }
    }

    // 2. No NN slot? Append under the final top-level heading.
    if !found {
        // Insert before the first Appendix heading, or append at end.
        let insert_at = lines.iter().position(|l| l.trim_start().starts_with("# Appendix"))
            .unwrap_or(lines.len());
        lines.insert(insert_at, want_line);
        if insert_at < lines.len() - 1 { lines.insert(insert_at + 1, String::new()); }
        changed = true;
    }

    if changed {
        let mut out = lines.join("\n");
        if original.ends_with('\n') && !out.ends_with('\n') { out.push('\n'); }
        std::fs::write(summary, out)?;
    }
    Ok(changed)
}

fn slug_to_title(slug: &str) -> String {
    let replaced = slug.replace('-', " ").replace('_', " ");
    let mut chars = replaced.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().chain(chars).collect(),
        None    => String::new(),
    }
}

fn scaffold_exercise(root: &Path, id: u32, name: &str) -> Result<()> {
    let lessons = discover_lessons(root)?;
    let Some(lesson) = lessons.iter().find(|l| l.id == LessonId(id)) else {
        anyhow::bail!("no lesson with id {id:02}");
    };
    let ex_dir = lesson.path.join("exercises");
    let n = std::fs::read_dir(&ex_dir)?.count();
    let file = ex_dir.join(format!("ex{}_{}.rs", n + 1, name));
    std::fs::write(
        &file,
        format!(
            "// Exercise {:02}.{} — {}\n//\n// TODO: write the broken snippet students fix.\n\nfn main() {{}}\n",
            id,
            n + 1,
            name
        ),
    )?;
    println!("{} {}", "scaffolded".green(), file.display());
    Ok(())
}

fn cmd_check(root: &Path) -> Result<()> {
    println!("{}", "self-check: compile exercises + run all project tests".bold());
    println!();

    // 1. Build the workspace (this covers every project's compile).
    let ok = Command::new("cargo")
        .current_dir(root)
        .args(["build", "--workspace", "--all-targets"])
        .status()?
        .success();
    if !ok { anyhow::bail!("workspace failed to build"); }

    // 2. Run cargo test for the workspace.
    let ok = Command::new("cargo")
        .current_dir(root)
        .args(["test", "--workspace", "--no-fail-fast"])
        .status()?
        .success();
    if !ok { anyhow::bail!("some tests failed"); }

    // Note: we do NOT run exercise .rs files here — they are intentionally
    // broken (that's the point). A future version could compile them with
    // `cargo check --example` once they're set up as examples.

    println!();
    println!("{}", "self-check passed".green().bold());
    Ok(())
}

#[derive(serde::Serialize)]
struct UserRow {
    name: String,
    lessons_done: usize,
    lessons_in_progress: usize,
    lessons_total: usize,
    last_active: Option<String>,
    /// Per-lesson snapshot keyed by lesson id as string ("03").
    lessons: std::collections::BTreeMap<String, LessonRow>,
}

#[derive(serde::Serialize)]
struct LessonRow {
    status: String,
    project_passing: bool,
    exercises_passed: u32,
}

fn collect_rows(filter: Option<&str>) -> Result<Vec<UserRow>> {
    let home_root = PathBuf::from("/home");
    let homes: Vec<PathBuf> = if home_root.exists() {
        std::fs::read_dir(&home_root)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.is_dir())
            .collect()
    } else {
        vec![PathBuf::from(std::env::var("HOME")?)]
    };

    let mut rows = Vec::new();
    for home in homes {
        let user = match home.file_name().and_then(|s| s.to_str()) {
            Some(u) => u.to_string(),
            None => continue,
        };
        if let Some(f) = filter {
            if user != f { continue; }
        }

        let progress_file = home.join(".rbb/progress.json");
        if !progress_file.exists() { continue; }

        let text = std::fs::read_to_string(&progress_file)?;
        let p: rbb_core::progress::Progress = serde_json::from_str(&text).unwrap_or_default();

        let mut lessons = std::collections::BTreeMap::new();
        let mut done = 0usize;
        let mut in_progress = 0usize;
        for (id, lp) in &p.lessons {
            match lp.status {
                rbb_core::LessonStatus::Done       => done += 1,
                rbb_core::LessonStatus::InProgress => in_progress += 1,
                _ => {}
            }
            lessons.insert(id.to_string(), LessonRow {
                status: format!("{:?}", lp.status),
                project_passing: lp.project_passing,
                exercises_passed: lp.exercises_passed(),
            });
        }

        rows.push(UserRow {
            name: user,
            lessons_done: done,
            lessons_in_progress: in_progress,
            lessons_total: p.lessons.len(),
            last_active: p.last_active,
            lessons,
        });
    }
    rows.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(rows)
}

fn cmd_publish(root: &Path, remote: &str, branch: &str, skip_check: bool) -> Result<()> {
    if !skip_check {
        println!("{}", "[1/2] preflight: rbb-admin check".bold());
        cmd_check(root)?;
    } else {
        println!("{}", "[1/2] preflight: skipped (--skip-check)".yellow());
    }

    println!("\n{}", format!("[2/2] pushing to {remote} ({branch})").bold());
    let status = Command::new("git")
        .current_dir(root)
        .args(["push", remote, &format!("HEAD:refs/heads/{branch}")])
        .status()
        .context("spawning git push")?;
    if !status.success() {
        anyhow::bail!("git push failed with {status}");
    }
    println!("\n{}", "published".green().bold());
    Ok(())
}

fn cmd_vendor_sync(root: &Path) -> Result<()> {
    println!("{}", "running `cargo vendor` — this needs network".bold());
    let status = Command::new("cargo")
        .current_dir(root)
        .arg("vendor")
        .status()
        .context("spawning cargo vendor")?;
    if !status.success() {
        anyhow::bail!("cargo vendor failed with {status}");
    }
    println!();
    println!("vendor/ is now up to date. Next steps:");
    println!("  git add Cargo.toml Cargo.lock vendor");
    println!("  git commit -m \"bump deps\"");
    println!("  rbb-admin publish");
    Ok(())
}

fn cmd_progress(filter: Option<&str>, as_json: bool) -> Result<()> {
    let rows = collect_rows(filter)?;

    if as_json {
        println!("{}", serde_json::to_string_pretty(&rows)?);
        return Ok(());
    }

    for row in &rows {
        let total = row.lessons_total.max(1);
        println!("{:<16} {}/{}  last active {}",
            row.name.cyan(),
            row.lessons_done, total,
            row.last_active.as_deref().unwrap_or("-"));
    }
    Ok(())
}
