# Lesson 3 — Multiple Tools

> **Goal**: Give the agent several tools with shared state. Observe the LLM picking the right one.
>
> **Concepts**: `Arc<Mutex<T>>` for shared state, multiple tools, complex args with `serde`, error handling from tools.

---

## Step 1 — Multiple tools, one agent

When the agent has several tools, the LLM sees all of their descriptions at once and picks whichever one fits the request. This lesson builds a note-taking agent with three tools: `add_note`, `list_notes`, and `delete_note`.

The challenge: all three tools need to read and write the same list of notes. In lesson 2, `Calculator` was stateless — no fields, no shared data. Here every tool struct must hold a handle to the same storage.

---

## Step 2 — The shared state

```rust
use std::sync::{Arc, Mutex};

type Notes = Arc<Mutex<Vec<String>>>;
```

`type Notes = ...` is a **type alias**. It gives a new name to an existing type — nothing more. After this line, writing `Notes` anywhere is exactly the same as writing `Arc<Mutex<Vec<String>>>`. The compiler treats them as identical; there is no wrapping or conversion.

Type aliases exist purely for readability. `Arc<Mutex<Vec<String>>>` appears in three struct definitions, the function signature, and several `Arc::clone` calls. Writing `Notes` each time is shorter and makes the intent clear: this is the shared note storage, not some incidental use of Arc.

You used `Arc<Mutex<T>>` in the threading chapter for the same reason: multiple owners, one piece of mutable data. The pattern is identical here — the only difference is that the "threads" are tool calls arriving from the LLM rather than OS threads you spawned yourself.

`Arc` lets you clone the handle cheaply. Each tool struct holds one clone. They all point to the same allocation.

`Mutex` ensures only one tool can modify the notes at a time. Rig calls tools sequentially so you will never see two locks contested, but the type system still requires it — you cannot give `&mut Vec<String>` to multiple owners without synchronisation.

You might wonder: if tools are never called concurrently, can we use `Rc<RefCell<T>>` instead of `Arc<Mutex<T>>`? No — and the reason is the `Send` bound, not concurrent access. The `Tool` trait requires `Send + Sync` on tool structs because the agent future must be `Send` to run on tokio's multi-threaded executor (`rt-multi-thread`). `Rc` does not implement `Send`, so the compiler rejects it regardless of whether two threads ever race. `Arc` is required to satisfy the trait bound, not because of concurrent tool calls.

---

## Step 3 — Define all three tools

Notice that the args structs derive `Deserialize` (the LLM's JSON must deserialize into them) but the tool structs themselves do not — because `Arc<Mutex<Vec<String>>>` cannot be serialized. The tool struct just holds a handle; only the args need serde.

```rust
use serde::Deserialize;
use rig::{completion::ToolDefinition, tool::Tool};
use serde_json::json;

// --- add_note ---

#[derive(Deserialize)]
struct AddNoteArgs {
    text: String,
}

struct AddNote {
    notes: Notes,
}

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
struct ToolError(String);

impl Tool for AddNote {
    const NAME: &'static str = "add_note";

    type Error = ToolError;
    type Args = AddNoteArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Save a note. Call this when the user wants to remember something."
                .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "text": {
                        "type": "string",
                        "description": "The note text to save"
                    }
                },
                "required": ["text"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let mut notes = self.notes.lock().unwrap();
        notes.push(args.text);
        let index = notes.len() - 1;
        Ok(format!("Note saved at index {index}."))
    }
}

// --- list_notes ---

#[derive(Deserialize)]
struct ListNotesArgs {}

struct ListNotes {
    notes: Notes,
}

impl Tool for ListNotes {
    const NAME: &'static str = "list_notes";

    type Error = ToolError;
    type Args = ListNotesArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "List all saved notes with their index numbers.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {}
            }),
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        let notes = self.notes.lock().unwrap();
        if notes.is_empty() {
            return Ok("No notes saved.".to_string());
        }
        let lines: Vec<String> = notes
            .iter()
            .enumerate()
            .map(|(i, n)| format!("{i}: {n}"))
            .collect();
        Ok(lines.join("\n"))
    }
}

// --- delete_note ---

#[derive(Deserialize)]
struct DeleteNoteArgs {
    index: usize,
}

struct DeleteNote {
    notes: Notes,
}

impl Tool for DeleteNote {
    const NAME: &'static str = "delete_note";

    type Error = ToolError;
    type Args = DeleteNoteArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Delete a note by its index number.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "index": {
                        "type": "integer",
                        "description": "The index of the note to delete"
                    }
                },
                "required": ["index"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let mut notes = self.notes.lock().unwrap();
        if args.index >= notes.len() {
            return Err(format!(
                "Index {} does not exist. There are {} notes (indices 0–{}).",
                args.index,
                notes.len(),
                notes.len().saturating_sub(1)
            ));
        }
        let removed = notes.remove(args.index);
        Ok(format!("Deleted note {}: \"{removed}\"", args.index))
    }
}
```

Each struct holds a clone of `Notes`. They all point to the same `Vec<String>`. `AddNote::call` pushes; `ListNotes::call` reads; `DeleteNote::call` removes by index.

---

## Step 4 — Wire them up

```rust
use rig::client::CompletionClient;
use rig::providers::openai::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::from_env()?;
    let notes: Notes = Arc::new(Mutex::new(Vec::new()));

    let agent = client
        .agent("gpt-4o-mini")
        .preamble(
            "You are a note-taking assistant. \
             Use the provided tools to add, list, and delete notes. \
             Always confirm what you did.",
        )
        .tool(AddNote { notes: Arc::clone(&notes) })
        .tool(ListNotes { notes: Arc::clone(&notes) })
        .tool(DeleteNote { notes: Arc::clone(&notes) })
        .build();
```

`Arc::clone(&notes)` is cheap — it increments a reference count, not the data. All three tool instances share one `Vec<String>`.

---

## Step 5 — Test the LLM's tool selection

Add a REPL and try these prompts in order:

```rust
    use std::io::{self, BufRead, Write};

    let stdin = io::stdin();

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut line = String::new();
        stdin.lock().read_line(&mut line)?;
        let input = line.trim();

        if input.is_empty() {
            continue;
        }

        let response = agent.prompt(input).await?;
        println!("{response}\n");
    }
}
```

Try these prompts and watch which tools fire:

```
> remember that the meeting is at 3pm
```
The LLM calls `add_note` with `"meeting is at 3pm"`.

```
> what notes do I have?
```
The LLM calls `list_notes` and formats the result.

```
> forget the meeting note
```
This is the interesting case. The LLM does not know the index. It calls `list_notes` first to find it, then calls `delete_note` with the correct index. Two tool calls, one user message. This is the agent loop running automatically — you wrote no control flow for it.

---

## Step 6 — Error handling from tools

Return `Err(ToolError)` from `call()` when something is wrong. Rig feeds the error message back to the LLM as a tool result:

```
> delete note 99
```

`DeleteNote::call` returns:

```
Err("Index 99 does not exist. There are 2 notes (indices 0–1).")
```

Rig sends this to the API. The LLM reads the error and tells the user:

```
There is no note at index 99. You currently have notes at indices 0 and 1.
```

The user gets a sensible explanation. You wrote no error-presentation code — the error message you returned from `call` was enough for the LLM to construct the reply.

The key: write error messages for the LLM to read, not for a human reading a stack trace. Complete sentences work better than codes.

---

## Full code

```rust
use rig::client::CompletionClient;
use rig::providers::openai::Client;
use rig::{completion::ToolDefinition, tool::Tool};
use serde::Deserialize;
use serde_json::json;
use std::io::{self, BufRead, Write};
use std::sync::{Arc, Mutex};

type Notes = Arc<Mutex<Vec<String>>>;

// --- add_note ---

#[derive(Deserialize)]
struct AddNoteArgs {
    text: String,
}

struct AddNote {
    notes: Notes,
}

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
struct ToolError(String);

impl Tool for AddNote {
    const NAME: &'static str = "add_note";

    type Error = ToolError;
    type Args = AddNoteArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Save a note. Call this when the user wants to remember something."
                .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "text": {
                        "type": "string",
                        "description": "The note text to save"
                    }
                },
                "required": ["text"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let mut notes = self.notes.lock().unwrap();
        notes.push(args.text);
        let index = notes.len() - 1;
        Ok(format!("Note saved at index {index}."))
    }
}

// --- list_notes ---

#[derive(Deserialize)]
struct ListNotesArgs {}

struct ListNotes {
    notes: Notes,
}

impl Tool for ListNotes {
    const NAME: &'static str = "list_notes";

    type Error = ToolError;
    type Args = ListNotesArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "List all saved notes with their index numbers.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {}
            }),
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        let notes = self.notes.lock().unwrap();
        if notes.is_empty() {
            return Ok("No notes saved.".to_string());
        }
        let lines: Vec<String> = notes
            .iter()
            .enumerate()
            .map(|(i, n)| format!("{i}: {n}"))
            .collect();
        Ok(lines.join("\n"))
    }
}

// --- delete_note ---

#[derive(Deserialize)]
struct DeleteNoteArgs {
    index: usize,
}

struct DeleteNote {
    notes: Notes,
}

impl Tool for DeleteNote {
    const NAME: &'static str = "delete_note";

    type Error = ToolError;
    type Args = DeleteNoteArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Delete a note by its index number.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "index": {
                        "type": "integer",
                        "description": "The index of the note to delete"
                    }
                },
                "required": ["index"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let mut notes = self.notes.lock().unwrap();
        if args.index >= notes.len() {
            return Err(format!(
                "Index {} does not exist. There are {} notes (indices 0–{}).",
                args.index,
                notes.len(),
                notes.len().saturating_sub(1)
            ));
        }
        let removed = notes.remove(args.index);
        Ok(format!("Deleted note {}: \"{removed}\"", args.index))
    }
}

// --- main ---

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::from_env()?;
    let notes: Notes = Arc::new(Mutex::new(Vec::new()));

    let agent = client
        .agent("gpt-4o-mini")
        .preamble(
            "You are a note-taking assistant. \
             Use the provided tools to add, list, and delete notes. \
             Always confirm what you did.",
        )
        .tool(AddNote { notes: Arc::clone(&notes) })
        .tool(ListNotes { notes: Arc::clone(&notes) })
        .tool(DeleteNote { notes: Arc::clone(&notes) })
        .build();

    let stdin = io::stdin();

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut line = String::new();
        stdin.lock().read_line(&mut line)?;
        let input = line.trim();

        if input.is_empty() {
            continue;
        }

        let response = agent.prompt(input).await?;
        println!("{response}\n");
    }
}
```

---

## Exercises

> **TODO 1**: Add a `clear_notes` tool that removes all notes at once. It takes no arguments. The LLM should call it when asked to "start fresh" or "delete everything."
>
> **TODO 2**: Add an `edit_note` tool that takes an `index: usize` and `text: String` and replaces that note. Test it with a natural language prompt like "change the meeting note to say 4pm instead." The LLM will need to call `list_notes` first to find the right index.
>
> **TODO 3**: `add_note` already returns the index in its output (`"Note saved at index {index}."`). Ask the agent to add a note and then ask "what index did you save that at?" — does the LLM remember and report it correctly?

---

## Key APIs

| API | What it does |
|-----|-------------|
| `Arc::new(Mutex::new(data))` | Creates shared mutable state that can be cloned across owners |
| `Arc::clone(&arc)` | Clones the handle (increments refcount, does not copy data) |
| `mutex.lock().unwrap()` | Acquires the lock; panics if the mutex is poisoned |
| `vec.remove(i)` | Removes and returns the element at index `i`; panics if out of bounds |
| Multiple `.tool(t)` calls | Registers multiple tools on one agent; the LLM sees all descriptions |
| `Err(ToolError)` from `call()` | Rig feeds the error string back to the LLM as a tool result |
