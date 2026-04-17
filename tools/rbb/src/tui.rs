//! Interactive dashboard. A student-friendly alternative to `rbb status`.
//!
//! Layout:
//!
//!   ┌─ Rust by Building ────────────────────────────────────────┐
//!   │  id  lesson               ex   proj  status               │
//!   │> 01  hello                0/0  -     not started           │
//!   │  03  functions            4/4  ✓     done                  │
//!   │                                                            │
//!   │ j/k move  t test  o open  r refresh  q quit                │
//!   └────────────────────────────────────────────────────────────┘

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use rbb_core::{
    discover_lessons, exercises, progress, Lesson, LessonStatus,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    prelude::*,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
    Terminal,
};
use std::io::{self, Stdout};
use std::path::{Path, PathBuf};
use std::time::Duration;

struct App {
    root: PathBuf,
    rows: Vec<LessonRow>,
    table: TableState,
    /// Bottom status line — transient messages after actions.
    status_line: String,
}

struct LessonRow {
    id: String,
    slug: String,
    ex_text: String,
    ex_style: Style,
    proj_text: String,
    proj_style: Style,
    status_text: String,
    status_style: Style,
    lesson: Lesson,
}

impl App {
    fn new(root: PathBuf) -> Result<Self> {
        let mut app = App {
            root,
            rows: Vec::new(),
            table: TableState::default(),
            status_line: "press q to quit, t to run tests, o to open README".into(),
        };
        app.reload()?;
        if !app.rows.is_empty() {
            app.table.select(Some(0));
        }
        Ok(app)
    }

    fn reload(&mut self) -> Result<()> {
        let lessons = discover_lessons(&self.root)?;
        let progress = progress::load().unwrap_or_default();

        self.rows.clear();
        for lesson in lessons {
            let lp = progress.lessons.get(&lesson.id);
            let total_ex = exercises::discover(&lesson.path)
                .map(|v| v.len() as u32)
                .unwrap_or(0);
            let passed_ex = lp.map(|p| p.exercises_passed()).unwrap_or(0);
            let proj = lp.map(|p| p.project_passing).unwrap_or(false);
            let status = lp.map(|p| p.status).unwrap_or_default();

            let (ex_text, ex_style) = if total_ex == 0 {
                ("-".into(), Style::default().fg(Color::DarkGray))
            } else if passed_ex == total_ex {
                (format!("{passed_ex}/{total_ex}"), Style::default().fg(Color::Green))
            } else if passed_ex == 0 {
                (format!("{passed_ex}/{total_ex}"), Style::default().fg(Color::DarkGray))
            } else {
                (format!("{passed_ex}/{total_ex}"), Style::default().fg(Color::Yellow))
            };

            let (proj_text, proj_style) = if proj {
                ("✓".into(), Style::default().fg(Color::Green))
            } else {
                ("-".into(), Style::default().fg(Color::DarkGray))
            };

            let (status_text, status_style) = match status {
                LessonStatus::NotStarted => ("not started".into(), Style::default().fg(Color::DarkGray)),
                LessonStatus::InProgress => ("in progress".into(), Style::default().fg(Color::Yellow)),
                LessonStatus::Done       => ("done".into(),        Style::default().fg(Color::Green)),
            };

            self.rows.push(LessonRow {
                id: format!("{:02}", lesson.id.0),
                slug: lesson.slug.clone(),
                ex_text, ex_style,
                proj_text, proj_style,
                status_text, status_style,
                lesson,
            });
        }
        Ok(())
    }

    fn selected(&self) -> Option<&LessonRow> {
        self.table.selected().and_then(|i| self.rows.get(i))
    }

    fn move_selection(&mut self, delta: isize) {
        if self.rows.is_empty() { return; }
        let cur = self.table.selected().unwrap_or(0) as isize;
        let next = (cur + delta).rem_euclid(self.rows.len() as isize) as usize;
        self.table.select(Some(next));
    }

    fn run_tests(&mut self, term: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        let Some(row) = self.selected() else { return Ok(()); };
        let lesson_id = row.lesson.id.to_string();

        // Suspend the TUI so cargo output lands in the real terminal.
        restore_terminal(term)?;
        println!("\nrunning tests for lesson {lesson_id}...\n");

        let project = row.lesson.path.join("project").join("Cargo.toml");
        let status = std::process::Command::new("cargo")
            .args(["test", "--manifest-path"])
            .arg(&project)
            .status()?;

        println!(
            "\n{}",
            if status.success() { "tests passed — press any key" }
            else                { "tests failed — press any key" }
        );
        let _ = wait_for_key();

        // Bring the TUI back.
        init_terminal(term)?;
        self.reload()?;
        self.status_line = if status.success() {
            format!("lesson {lesson_id}: tests passed")
        } else {
            format!("lesson {lesson_id}: tests failed — see scrollback")
        };
        Ok(())
    }

    fn open_readme(&mut self, term: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        let Some(row) = self.selected() else { return Ok(()); };
        let readme = row.lesson.path.join("README.md");
        let Ok(text) = std::fs::read_to_string(&readme) else {
            self.status_line = format!("no README at {}", readme.display());
            return Ok(());
        };

        restore_terminal(term)?;
        println!("\n{text}\n--- press any key to return ---");
        let _ = wait_for_key();
        init_terminal(term)?;
        Ok(())
    }
}

fn wait_for_key() -> Result<()> {
    loop {
        if let Event::Key(k) = event::read()? {
            if k.kind == KeyEventKind::Press { return Ok(()); }
        }
    }
}

fn init_terminal(term: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    term.clear()?;
    Ok(())
}

fn restore_terminal(term: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(term.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    term.show_cursor()?;
    Ok(())
}

fn draw(f: &mut Frame, app: &mut App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Lesson table.
    let header = Row::new(vec!["id", "lesson", "ex", "proj", "status"])
        .style(Style::default().add_modifier(Modifier::BOLD));

    let rows: Vec<Row> = app.rows.iter().map(|r| {
        Row::new(vec![
            Cell::from(r.id.clone()),
            Cell::from(r.slug.clone()),
            Cell::from(r.ex_text.clone()).style(r.ex_style),
            Cell::from(r.proj_text.clone()).style(r.proj_style),
            Cell::from(r.status_text.clone()).style(r.status_style),
        ])
    }).collect();

    let widths = [
        Constraint::Length(4),
        Constraint::Min(20),
        Constraint::Length(6),
        Constraint::Length(5),
        Constraint::Length(14),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" Rust by Building "))
        .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    f.render_stateful_widget(table, layout[0], &mut app.table);

    // Status bar.
    let hints = "j/k move  t test  o open  r refresh  q quit";
    let bar = Paragraph::new(format!("{hints}\n{}", app.status_line))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(bar, layout[1]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;

    /// Render the dashboard against an in-memory 80x10 backend and
    /// assert the lesson rows come through. Bypasses the event loop
    /// and the real terminal — just feeds our App into `draw`.
    #[test]
    fn renders_lesson_table() -> Result<()> {
        // Walk up from this file to the workspace root.
        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let repo = manifest.parent().unwrap().parent().unwrap().to_path_buf();

        let mut app = App::new(repo)?;
        let backend = TestBackend::new(80, 10);
        let mut term = Terminal::new(backend)?;

        term.draw(|f| draw(f, &mut app))?;

        let buf = term.backend().buffer().clone();
        let w = buf.area.width;
        let h = buf.area.height;
        let mut text = String::new();
        for y in 0..h {
            for x in 0..w {
                text.push_str(buf[(x, y)].symbol());
            }
            text.push('\n');
        }

        assert!(text.contains("Rust by Building"), "missing title:\n{text}");
        assert!(text.contains("functions"), "missing lesson row:\n{text}");
        assert!(text.contains("j/k move"),  "missing hint bar:\n{text}");
        Ok(())
    }
}

pub fn run(root: &Path) -> Result<()> {
    let mut app = App::new(root.to_path_buf())?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut term = Terminal::new(backend)?;
    init_terminal(&mut term)?;

    let result = (|| -> Result<()> {
        loop {
            term.draw(|f| draw(f, &mut app))?;

            if !event::poll(Duration::from_millis(250))? { continue; }
            let Event::Key(key) = event::read()? else { continue; };
            if key.kind != KeyEventKind::Press { continue; }

            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Char('j') | KeyCode::Down => app.move_selection(1),
                KeyCode::Char('k') | KeyCode::Up   => app.move_selection(-1),
                KeyCode::Char('r') => { app.reload()?; app.status_line = "reloaded".into(); }
                KeyCode::Char('t') => app.run_tests(&mut term)?,
                KeyCode::Char('o') => app.open_readme(&mut term)?,
                _ => {}
            }
        }
    })();

    restore_terminal(&mut term)?;
    result
}
