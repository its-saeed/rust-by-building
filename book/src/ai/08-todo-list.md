# Mini Project — AI Todo List

## What you'll build

A command-line todo list where you talk to an AI agent in plain English. The agent decides which tools to call — you never write parsing logic.

```
> add buy milk, eggs, and bread
Added 3 items: milk (id 1), eggs (id 2), bread (id 3).

> what's on my list?
  1. [ ] buy milk
  2. [ ] buy eggs
  3. [ ] buy bread

> I got the milk and eggs
Marked milk and eggs as complete.

  1. [x] buy milk
  2. [x] buy eggs
  3. [ ] buy bread

> clean up finished items
Removed 2 completed items. 1 item remaining.

> add dentist appointment and call mom
Added: dentist appointment (id 4), call mom (id 5).

> quit
```

Notice "I got the milk and eggs" — the LLM called `list_todos()` first to find the ids, then called `complete_todo()` twice. One vague message, three tool calls. That's the agent loop at work.

---

## The State

Start with the data structures:

```rust
struct Todo {
    id: u32,
    text: String,
    done: bool,
}

struct TodoList {
    items: Vec<Todo>,
    next_id: u32,
}
```

Add methods on `TodoList`:

```rust
impl TodoList {
    fn new() -> Self {
        TodoList { items: Vec::new(), next_id: 1 }
    }

    fn add(&mut self, text: String) -> u32 {
        let id = self.next_id;
        self.items.push(Todo { id, text, done: false });
        self.next_id += 1;
        id
    }

    fn complete(&mut self, id: u32) -> Result<(), String> {
        match self.items.iter_mut().find(|t| t.id == id) {
            Some(todo) => { todo.done = true; Ok(()) }
            None => Err(format!("No item with id {id}")),
        }
    }

    fn uncomplete(&mut self, id: u32) -> Result<(), String> {
        match self.items.iter_mut().find(|t| t.id == id) {
            Some(todo) => { todo.done = false; Ok(()) }
            None => Err(format!("No item with id {id}")),
        }
    }

    fn clear_done(&mut self) -> usize {
        let before = self.items.len();
        self.items.retain(|t| !t.done);
        before - self.items.len()
    }

    fn display(&self) -> String {
        if self.items.is_empty() {
            return "Your list is empty.".to_string();
        }
        self.items
            .iter()
            .map(|t| {
                let mark = if t.done { "x" } else { " " };
                format!("  {}. [{}] {}", t.id, mark, t.text)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
```

---

## The Tools

Six tools. Each one holds `Arc<Mutex<TodoList>>` so they share the same list.

### `add_todo`

Adds a single item. Use this when the user mentions one thing.

```rust
use std::sync::{Arc, Mutex};
use rig::{completion::ToolDefinition, tool::Tool};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct AddTodoArgs {
    text: String,
}

struct AddTodo {
    list: Arc<Mutex<TodoList>>,
}

impl Tool for AddTodo {
    const NAME: &'static str = "add_todo";
    type Error = String;
    type Args = AddTodoArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Add a single todo item.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "text": { "type": "string", "description": "The todo item text" }
                },
                "required": ["text"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let mut list = self.list.lock().unwrap();
        let id = list.add(args.text.clone());
        Ok(format!("Added: {} (id {})", args.text, id))
    }
}
```

### `add_many`

Batch-adds multiple items from one request. This is what handles "add milk, eggs, and bread" in a single tool call.

The JSON schema uses an array type. serde deserializes it into `Vec<String>` automatically.

```rust
#[derive(Deserialize)]
struct AddManyArgs {
    items: Vec<String>,
}

struct AddMany {
    list: Arc<Mutex<TodoList>>,
}

impl Tool for AddMany {
    const NAME: &'static str = "add_many";
    type Error = String;
    type Args = AddManyArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Add multiple todo items at once.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "items": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "List of todo items to add"
                    }
                },
                "required": ["items"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let mut list = self.list.lock().unwrap();
        let mut results = Vec::new();
        for text in args.items {
            let id = list.add(text.clone());
            results.push(format!("{} (id {})", text, id));
        }
        Ok(format!("Added {}: {}", results.len(), results.join(", ")))
    }
}
```

### `complete_todo` and `uncomplete_todo`

Mark items done or undone by id.

```rust
#[derive(Deserialize)]
struct TodoIdArgs {
    id: u32,
}

struct CompleteTodo {
    list: Arc<Mutex<TodoList>>,
}

impl Tool for CompleteTodo {
    const NAME: &'static str = "complete_todo";
    type Error = String;
    type Args = TodoIdArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Mark a todo item as complete by id.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "id": { "type": "integer", "description": "The todo item id" }
                },
                "required": ["id"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let mut list = self.list.lock().unwrap();
        list.complete(args.id)?;
        Ok(format!("Marked item {} as complete.", args.id))
    }
}

struct UncompleteTodo {
    list: Arc<Mutex<TodoList>>,
}

impl Tool for UncompleteTodo {
    const NAME: &'static str = "uncomplete_todo";
    type Error = String;
    type Args = TodoIdArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Mark a todo item as not complete by id.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "id": { "type": "integer", "description": "The todo item id" }
                },
                "required": ["id"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let mut list = self.list.lock().unwrap();
        list.uncomplete(args.id)?;
        Ok(format!("Marked item {} as incomplete.", args.id))
    }
}
```

### `clear_done`

Removes all completed items. Takes no arguments — the schema is an empty object.

```rust
#[derive(Deserialize)]
struct NoArgs {}

struct ClearDone {
    list: Arc<Mutex<TodoList>>,
}

impl Tool for ClearDone {
    const NAME: &'static str = "clear_done";
    type Error = String;
    type Args = NoArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Remove all completed todo items.".to_string(),
            parameters: json!({ "type": "object", "properties": {} }),
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        let mut list = self.list.lock().unwrap();
        let removed = list.clear_done();
        Ok(format!("Removed {} completed item(s). {} remaining.", removed, list.items.len()))
    }
}
```

### `list_todos`

Returns the current list as a formatted string. This is a read-only query tool — the LLM calls it to see what's there before deciding what to do.

```rust
struct ListTodos {
    list: Arc<Mutex<TodoList>>,
}

impl Tool for ListTodos {
    const NAME: &'static str = "list_todos";
    type Error = String;
    type Args = NoArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "List all current todo items with their ids and completion status.".to_string(),
            parameters: json!({ "type": "object", "properties": {} }),
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        let list = self.list.lock().unwrap();
        Ok(list.display())
    }
}
```

---

## Wiring the Agent

Create one `Arc<Mutex<TodoList>>` and clone it into each tool:

```rust
use rig::client::ProviderClient;
use rig::providers::openai::Client;

let client = Client::from_env()?;
let list = Arc::new(Mutex::new(TodoList::new()));

let agent = client
    .agent(openai::GPT_4O_MINI)
    .preamble("You manage a todo list. Use the available tools to add, complete, and remove items. When the user mentions completing items by name, first call list_todos to find their ids.")
    .tool(AddTodo { list: list.clone() })
    .tool(AddMany { list: list.clone() })
    .tool(CompleteTodo { list: list.clone() })
    .tool(UncompleteTodo { list: list.clone() })
    .tool(ClearDone { list: list.clone() })
    .tool(ListTodos { list: list.clone() })
    .build();
```

The preamble tells the LLM to call `list_todos` when it needs to resolve item names to ids. Without this instruction the model might hallucinate ids. This is prompt engineering — the right instruction makes the agent reliable without any extra code.

---

## The Main Loop

```rust
use std::io::Write;

loop {
    print!("> ");
    std::io::stdout().flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let input = input.trim();

    if input == "quit" || input == "exit" { break; }
    if input.is_empty() { continue; }

    let response = agent.prompt(input).await?;
    println!("{response}\n");
}
```

---

## Full code

```rust
use std::sync::{Arc, Mutex};
use std::io::Write;

use anyhow::Result;
use rig::client::ProviderClient;
use rig::providers::openai::Client;
use rig::{completion::ToolDefinition, tool::Tool};
use serde::Deserialize;
use serde_json::json;

struct Todo {
    id: u32,
    text: String,
    done: bool,
}

struct TodoList {
    items: Vec<Todo>,
    next_id: u32,
}

impl TodoList {
    fn new() -> Self {
        TodoList { items: Vec::new(), next_id: 1 }
    }

    fn add(&mut self, text: String) -> u32 {
        let id = self.next_id;
        self.items.push(Todo { id, text, done: false });
        self.next_id += 1;
        id
    }

    fn complete(&mut self, id: u32) -> Result<(), String> {
        match self.items.iter_mut().find(|t| t.id == id) {
            Some(todo) => { todo.done = true; Ok(()) }
            None => Err(format!("No item with id {id}")),
        }
    }

    fn uncomplete(&mut self, id: u32) -> Result<(), String> {
        match self.items.iter_mut().find(|t| t.id == id) {
            Some(todo) => { todo.done = false; Ok(()) }
            None => Err(format!("No item with id {id}")),
        }
    }

    fn clear_done(&mut self) -> usize {
        let before = self.items.len();
        self.items.retain(|t| !t.done);
        before - self.items.len()
    }

    fn display(&self) -> String {
        if self.items.is_empty() {
            return "Your list is empty.".to_string();
        }
        self.items
            .iter()
            .map(|t| {
                let mark = if t.done { "x" } else { " " };
                format!("  {}. [{}] {}", t.id, mark, t.text)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[derive(Deserialize)]
struct AddTodoArgs {
    text: String,
}

struct AddTodo {
    list: Arc<Mutex<TodoList>>,
}

impl Tool for AddTodo {
    const NAME: &'static str = "add_todo";
    type Error = String;
    type Args = AddTodoArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Add a single todo item.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "text": { "type": "string", "description": "The todo item text" }
                },
                "required": ["text"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let mut list = self.list.lock().unwrap();
        let id = list.add(args.text.clone());
        Ok(format!("Added: {} (id {})", args.text, id))
    }
}

#[derive(Deserialize)]
struct AddManyArgs {
    items: Vec<String>,
}

struct AddMany {
    list: Arc<Mutex<TodoList>>,
}

impl Tool for AddMany {
    const NAME: &'static str = "add_many";
    type Error = String;
    type Args = AddManyArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Add multiple todo items at once.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "items": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "List of todo items to add"
                    }
                },
                "required": ["items"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let mut list = self.list.lock().unwrap();
        let mut results = Vec::new();
        for text in args.items {
            let id = list.add(text.clone());
            results.push(format!("{} (id {})", text, id));
        }
        Ok(format!("Added {}: {}", results.len(), results.join(", ")))
    }
}

#[derive(Deserialize)]
struct TodoIdArgs {
    id: u32,
}

struct CompleteTodo {
    list: Arc<Mutex<TodoList>>,
}

impl Tool for CompleteTodo {
    const NAME: &'static str = "complete_todo";
    type Error = String;
    type Args = TodoIdArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Mark a todo item as complete by id.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "id": { "type": "integer", "description": "The todo item id" }
                },
                "required": ["id"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let mut list = self.list.lock().unwrap();
        list.complete(args.id)?;
        Ok(format!("Marked item {} as complete.", args.id))
    }
}

struct UncompleteTodo {
    list: Arc<Mutex<TodoList>>,
}

impl Tool for UncompleteTodo {
    const NAME: &'static str = "uncomplete_todo";
    type Error = String;
    type Args = TodoIdArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Mark a todo item as not complete by id.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "id": { "type": "integer", "description": "The todo item id" }
                },
                "required": ["id"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let mut list = self.list.lock().unwrap();
        list.uncomplete(args.id)?;
        Ok(format!("Marked item {} as incomplete.", args.id))
    }
}

#[derive(Deserialize)]
struct NoArgs {}

struct ClearDone {
    list: Arc<Mutex<TodoList>>,
}

impl Tool for ClearDone {
    const NAME: &'static str = "clear_done";
    type Error = String;
    type Args = NoArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Remove all completed todo items.".to_string(),
            parameters: json!({ "type": "object", "properties": {} }),
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        let mut list = self.list.lock().unwrap();
        let removed = list.clear_done();
        Ok(format!("Removed {} completed item(s). {} remaining.", removed, list.items.len()))
    }
}

struct ListTodos {
    list: Arc<Mutex<TodoList>>,
}

impl Tool for ListTodos {
    const NAME: &'static str = "list_todos";
    type Error = String;
    type Args = NoArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "List all current todo items with their ids and completion status.".to_string(),
            parameters: json!({ "type": "object", "properties": {} }),
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        let list = self.list.lock().unwrap();
        Ok(list.display())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::from_env()?;
    let list = Arc::new(Mutex::new(TodoList::new()));

    let agent = client
        .agent(openai::GPT_4O_MINI)
        .preamble("You manage a todo list. Use the available tools to add, complete, and remove items. When the user mentions completing items by name, first call list_todos to find their ids.")
        .tool(AddTodo { list: list.clone() })
        .tool(AddMany { list: list.clone() })
        .tool(CompleteTodo { list: list.clone() })
        .tool(UncompleteTodo { list: list.clone() })
        .tool(ClearDone { list: list.clone() })
        .tool(ListTodos { list: list.clone() })
        .build();

    println!("AI Todo List — type 'quit' to exit\n");

    loop {
        print!("> ");
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "quit" || input == "exit" { break; }
        if input.is_empty() { continue; }

        let response = agent.prompt(input).await?;
        println!("{response}\n");
    }

    Ok(())
}
```

Your `Cargo.toml`:

```toml
[package]
name = "ai-todo"
version = "0.1.0"
edition = "2021"

[dependencies]
rig = "0.38.2"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
```

Set your API key and run:

```sh
export OPENAI_API_KEY=sk-...
cargo run
```

---

## What to notice

"I got the milk and eggs" produced three tool calls from one sentence: `list_todos()` to find the ids, then `complete_todo(1)` and `complete_todo(2)`. The agent loop ran inside rig, invisibly. The code you wrote didn't parse the sentence or figure out the ids — the LLM did.

`list_todos` is a query tool. It has no side effects. The LLM uses it to read state before deciding what mutations to make. If you build anything where the agent needs to reference current state before acting — a physics sandbox, a calendar, a file system — you'll reach for this same pattern.

`add_many` takes `Vec<String>`. The JSON schema declares `"type": "array"` and serde handles the rest. JSON arrays become Rust Vecs with no extra code on your side.

The state lives in Rust — `Arc<Mutex<TodoList>>` — not in the LLM. The LLM has no memory of previous turns beyond what the tools return. If you restart the program, the list is empty. The LLM decides what mutations to make; Rust owns the data.
