//! Student CLI for the Rust by Building course.
//!
//! Minimal-viable surface: status / open / test / next.
//! `watch` is a thin wrapper around `test` on a timer for now — full
//! filesystem-watcher support is a later polish step.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
use rbb_core::{discover_lessons, find_lesson, find_repo_root, LessonId, LessonStatus};
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(name = "rbb", about = "Rust by Building — student CLI", version)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Show all lessons and your progress.
    Status,
    /// Print a lesson's README to stdout.
    Open { lesson: String },
    /// Run tests for a lesson's project.
    Test { lesson: String },
    /// Rerun tests every time something changes (simple poll for now).
    Watch { lesson: String },
    /// Jump to the next lesson you haven't finished.
    Next,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = find_repo_root(&std::env::current_dir()?)
        .context("run rbb from inside your rust-by-building checkout")?;

    match cli.cmd {
        Cmd::Status       => cmd_status(&root),
        Cmd::Open { lesson }  => cmd_open(&root, &lesson),
        Cmd::Test { lesson }  => cmd_test(&root, &lesson),
        Cmd::Watch { lesson } => cmd_watch(&root, &lesson),
        Cmd::Next         => cmd_next(&root),
    }
}

fn parse_id(s: &str) -> Result<LessonId> { s.parse() }

fn cmd_status(root: &PathBuf) -> Result<()> {
    let lessons = discover_lessons(root)?;
    let progress = rbb_core::progress::load()?;

    println!("{:>3}  {:<24} {}", "id", "lesson", "status");
    for l in lessons {
        let status = progress.lessons.get(&l.id)
            .map(|lp| lp.status)
            .unwrap_or_default();

        let status_str = match status {
            LessonStatus::NotStarted => "not started".dimmed().to_string(),
            LessonStatus::InProgress => "in progress".yellow().to_string(),
            LessonStatus::Done       => "done".green().to_string(),
        };
        println!("{:>3}  {:<24} {}", l.id, l.slug, status_str);
    }
    Ok(())
}

fn cmd_open(root: &PathBuf, lesson: &str) -> Result<()> {
    let id = parse_id(lesson)?;
    let l = find_lesson(root, id)?;
    let readme = l.path.join("README.md");
    let text = std::fs::read_to_string(&readme)
        .with_context(|| format!("reading {readme:?}"))?;
    println!("{text}");
    Ok(())
}

fn cmd_test(root: &PathBuf, lesson: &str) -> Result<()> {
    let id = parse_id(lesson)?;
    let l = find_lesson(root, id)?;
    let project = l.path.join("project").join("Cargo.toml");
    if !project.exists() {
        anyhow::bail!("lesson {id} has no project at {project:?}");
    }

    let status = Command::new("cargo")
        .args(["test", "--manifest-path"])
        .arg(&project)
        .status()?;

    if status.success() {
        mark_project_passing(id)?;
        println!("{}", "all tests passed".green());
        Ok(())
    } else {
        println!("{}", "tests failed".red());
        std::process::exit(1);
    }
}

fn cmd_watch(root: &PathBuf, lesson: &str) -> Result<()> {
    // Simplest possible version: poll every 2s. Replace with `notify`
    // once we're past the scaffolding phase.
    println!("watching lesson {lesson} — Ctrl-C to stop");
    loop {
        let _ = cmd_test(root, lesson);
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}

fn cmd_next(root: &PathBuf) -> Result<()> {
    let lessons = discover_lessons(root)?;
    let progress = rbb_core::progress::load()?;

    for l in lessons {
        let done = progress.lessons.get(&l.id).map(|lp| lp.status) == Some(LessonStatus::Done);
        if !done {
            println!("next up: lesson {} — {}", l.id, l.slug);
            println!("  rbb open {}", l.id);
            return Ok(());
        }
    }
    println!("{}", "all lessons complete".green());
    Ok(())
}

fn mark_project_passing(id: LessonId) -> Result<()> {
    let mut p = rbb_core::progress::load()?;
    let lp = p.lessons.entry(id).or_default();
    lp.project_passing = true;
    lp.status = LessonStatus::Done;
    p.last_active = Some(now_rfc3339());
    rbb_core::progress::save(&p)
}

fn now_rfc3339() -> String {
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("@{t}")
}
