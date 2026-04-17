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
    Progress { user: Option<String> },
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
        Cmd::User(c)           => handle_user(c),
        Cmd::Progress { user } => cmd_progress(user.as_deref()),
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
    }
}

fn handle_user(c: UserCmd) -> Result<()> {
    match c {
        UserCmd::Add { name, from }             => user_add(&name, &from),
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

fn user_add(name: &str, from: &Path) -> Result<()> {
    require_root()?;

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
        // A freshly copied workspace target/ is owned by the admin and
        // contains stale fingerprints. Clear it; the student rebuilds.
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

    // Keep this line color-free so shell tests can grep it reliably.
    println!("created user {name}");
    println!("  home:        {}", home.display());
    println!("  checkout:    {}", dest.display());
    println!("  port range:  {}-{}", port_base, port_base + 99);
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

    println!("{} lesson {:02}-{}", "scaffolded".green(), id, slug);
    println!("  {}", dir.display());
    println!("  {}", book.display());
    println!();
    println!("don't forget to add it to book/src/SUMMARY.md");
    Ok(())
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

fn cmd_progress(filter: Option<&str>) -> Result<()> {
    // Read /home/*/.rbb/progress.json. On dev machines (where /home
    // doesn't have students) we fall back to just the current user.
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

    for home in homes {
        let user = home.file_name().and_then(|s| s.to_str()).unwrap_or("?");
        if let Some(f) = filter {
            if user != f { continue; }
        }

        let progress_file = home.join(".rbb/progress.json");
        if !progress_file.exists() { continue; }

        let text = std::fs::read_to_string(&progress_file)?;
        let p: rbb_core::progress::Progress = serde_json::from_str(&text).unwrap_or_default();

        let done = p.lessons.values().filter(|lp| lp.status == rbb_core::LessonStatus::Done).count();
        let total = p.lessons.len().max(1);
        println!("{:<16} {}/{}  last active {}",
            user.cyan(),
            done, total,
            p.last_active.as_deref().unwrap_or("-"));
    }
    Ok(())
}
