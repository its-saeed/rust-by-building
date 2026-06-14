# Mini Project 2 — Parallel File Search

Build a command-line tool that searches for a word across every `.rs` file in a directory, using one thread per file. Running searches in parallel cuts the total time — especially on an SSD where many files can be read simultaneously.

This is a practical tool you will actually want: a simpler, faster version of `grep -r`.

```sh
cargo run -- "fn main" ./src
```

```
src/main.rs
  1: fn main() {

src/client.rs
  87: fn main() {

src/server.rs
  12: fn main() {
```

---

## What we are parallelising

Each file requires two steps: read from disk, then scan for matches. Both take time. On a spinning disk, parallel reads can be slower (the disk head jumps around), but on an SSD, parallel reads are fine — the drive handles them concurrently.

More importantly, the scanning step is CPU work. On a multi-core machine, eight threads scanning eight files simultaneously uses all cores.

```
time ──────────────────────────────────────────────────────────▶

sequential:
  read+scan file1 [████]
  read+scan file2       [████]
  read+scan file3             [████]

parallel (4 threads):
  read+scan file1 [████]
  read+scan file2 [████]
  read+scan file3 [████]
  read+scan file4 [████]
```

---

## Step 1 — Command-line arguments

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("usage: {} <query> <directory>", args[0]);
        std::process::exit(1);
    }

    let query = args[1].clone();
    let dir   = args[2].clone();
}
```

`env::args()` returns an iterator of strings. `args[0]` is the program name, `args[1]` is the search term, `args[2]` is the directory.

---

## Step 2 — Collect files to search

Walk the directory and collect every `.rs` file:

```rust
use std::fs;
use std::path::PathBuf;

let files: Vec<PathBuf> = fs::read_dir(&dir)
    .expect("could not read directory")
    .filter_map(|entry| entry.ok())
    .map(|entry| entry.path())
    .filter(|path| path.extension().map(|e| e == "rs").unwrap_or(false))
    .collect();

println!("searching {} files for {:?}...", files.len(), query);
```

`fs::read_dir` returns an iterator of `Result<DirEntry>`. `filter_map(|e| e.ok())` skips any entries that errored (permission denied, etc.) without crashing.

---

## Step 3 — The search function

Read one file and return all matching lines with their line numbers:

```rust
use std::path::Path;

fn search(path: &Path, query: &str) -> Vec<(usize, String)> {
    let content = match fs::read_to_string(path) {
        Ok(c)  => c,
        Err(_) => return vec![],  // skip unreadable files silently
    };

    content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(query))
        .map(|(i, line)| (i + 1, line.to_string()))
        .collect()
}
```

`.enumerate()` pairs each line with its index (0-based). Adding 1 gives the conventional 1-based line number editors use.

`.filter(...).map(...).collect()` is the iterator pipeline pattern from earlier projects — filter to matching lines, map to the format we want, collect into a `Vec`.

---

## Step 4 — Spawn one thread per file

Each thread searches one file and sends the results through a channel. Results include the file path so the receiver knows where each match came from:

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel::<(PathBuf, Vec<(usize, String)>)>();

for path in files {
    let tx    = tx.clone();
    let query = query.clone();

    thread::spawn(move || {
        let matches = search(&path, &query);
        if !matches.is_empty() {
            tx.send((path, matches)).unwrap();
        }
        // if no matches, send nothing — receiver sees nothing for this file
    });
}

drop(tx);
```

`query.clone()` is needed because the string moves into each closure. Each thread gets its own copy — cheap for a short search term.

Only files with at least one match send a message. The receiver never sees empty results.

---

## Step 5 — Print results as they arrive

```rust
let mut total_matches = 0;

for (path, matches) in rx {
    println!("\n{}", path.display());
    for (line_num, line) in &matches {
        println!("  {line_num:>4}: {line}");
    }
    total_matches += matches.len();
}

println!("\n{total_matches} match(es) found");
```

Results arrive in whatever order threads finish — faster for small files, slower for large ones. The output order is non-deterministic, just like the weather example.

`path.display()` converts a `PathBuf` to something printable. `{line_num:>4}` right-aligns the line number in a 4-character column.

---

## Full program

```rust
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

fn search(path: &Path, query: &str) -> Vec<(usize, String)> {
    let content = match fs::read_to_string(path) {
        Ok(c)  => c,
        Err(_) => return vec![],
    };
    content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(query))
        .map(|(i, line)| (i + 1, line.to_string()))
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: {} <query> <directory>", args[0]);
        std::process::exit(1);
    }

    let query = args[1].clone();
    let dir   = args[2].clone();

    let files: Vec<PathBuf> = fs::read_dir(&dir)
        .expect("could not read directory")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|e| e == "rs").unwrap_or(false))
        .collect();

    println!("searching {} file(s) for {:?}", files.len(), query);

    let t0 = Instant::now();
    let (tx, rx) = mpsc::channel::<(PathBuf, Vec<(usize, String)>)>();

    for path in files {
        let tx    = tx.clone();
        let query = query.clone();
        thread::spawn(move || {
            let matches = search(&path, &query);
            if !matches.is_empty() {
                tx.send((path, matches)).unwrap();
            }
        });
    }
    drop(tx);

    let mut total = 0;
    for (path, matches) in rx {
        println!("\n{}", path.display());
        for (n, line) in &matches {
            println!("  {n:>4}: {line}");
        }
        total += matches.len();
    }

    println!("\n{total} match(es) in {:.2?}", t0.elapsed());
}
```

---

## Try it on this book's own source

```sh
# search the tele-sketch lesson starters
cargo run -- "DrawEvent" ../../lessons/9-tele-sketch/lesson-01/project/src

# search for a function across a whole project
cargo run -- "fn handle" ../../lessons/
```

---

## Comparing to sequential

To see the speedup, replace the threads with a sequential loop:

```rust
// sequential version for comparison
for path in files {
    let matches = search(&path, &query);
    if !matches.is_empty() {
        tx.send((path, matches)).unwrap();
    }
}
```

On a large codebase (thousands of files) the parallel version is noticeably faster. On a small directory with ten files, the thread creation overhead may actually make it slower.

---

## Extending to subdirectories

`fs::read_dir` only lists the immediate directory. To search recursively, use the `walkdir` crate (already in the workspace):

```rust
use walkdir::WalkDir;

let files: Vec<PathBuf> = WalkDir::new(&dir)
    .into_iter()
    .filter_map(|e| e.ok())
    .map(|e| e.path().to_path_buf())
    .filter(|p| p.extension().map(|e| e == "rs").unwrap_or(false))
    .collect();
```

Same iterator pipeline, different source.

---

## Exercise

> **TODO 1**: Make the file extension configurable. Accept an optional third argument: `cargo run -- "fn" ./src rs`. Default to `"rs"` if not given.
>
> **TODO 2**: Add case-insensitive search: if the query is all lowercase, match regardless of case in the file. `line.to_lowercase().contains(&query.to_lowercase())`.
>
> **TODO 3**: The results arrive in random order. Sort them by filename before printing: collect all `(path, matches)` pairs from `rx` into a `Vec`, sort with `.sort_by_key(|(p, _)| p.clone())`, then print.

---

## Key APIs

| API | What it does |
|-----|-------------|
| `env::args().collect::<Vec<_>>()` | Command-line arguments as a Vec of Strings |
| `fs::read_dir(path)` | List the immediate contents of a directory |
| `entry.path()` | Get the `PathBuf` for a directory entry |
| `path.extension()` | File extension as `Option<&OsStr>` |
| `fs::read_to_string(path)` | Read an entire file as a `String` |
| `str.lines().enumerate()` | Iterator of `(index, line)` pairs |
| `path.display()` | Format a `PathBuf` for printing |
| `walkdir::WalkDir::new(dir)` | Recursive directory traversal |
