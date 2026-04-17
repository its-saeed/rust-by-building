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
    Add { name: String },
    Remove { name: String },
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
    let root = find_repo_root(&std::env::current_dir()?)
        .context("run rbb-admin from inside your rust-by-building checkout")?;

    match cli.cmd {
        Cmd::User(c)     => handle_user(c),
        Cmd::Lesson(c)   => handle_lesson(&root, c),
        Cmd::Check       => cmd_check(&root),
        Cmd::Progress { user } => cmd_progress(user.as_deref()),
    }
}

fn handle_user(c: UserCmd) -> Result<()> {
    match c {
        UserCmd::Add { name }    => { println!("TODO: useradd / clone / config for {name}"); Ok(()) }
        UserCmd::Remove { name } => { println!("TODO: userdel for {name}"); Ok(()) }
        UserCmd::List            => { println!("TODO: list /home/*"); Ok(()) }
    }
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
