# Lesson 2 — Defining a Tool

> **Goal**: Define a Rust struct as a tool the LLM can call.
>
> **Concepts**: `Tool` trait, `ToolDefinition`, JSON schema for parameters, `serde::Deserialize` for args.

---

## Step 1 — The problem with text-only agents

The REPL from lesson 1 can answer questions but it cannot *do* anything. Ask it "what is 47 times 83?" and it will produce the right answer — but only because the model has seen arithmetic in its training data. It is pattern-matching, not computing.

Ask it something it cannot pattern-match — "what is `sha256('hello world')`?" — and it will guess wrong with confidence. The model has no access to runtime functions. It only knows what is in its weights.

Tool calling is the fix. You define a function in Rust. You describe it in a format the LLM can read. The LLM decides when to call it and what arguments to pass. Rig executes the function and feeds the result back to the model, which then writes its final reply.

---

## Step 2 — Define the tool struct

Add `serde` to `Cargo.toml`:

```toml
[dependencies]
rig = "0.38.2"
thiserror = "1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
```

A tool is a struct that implements the `Tool` trait. Start with the args type — the data the LLM will send when it calls the tool:

```rust
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CalculatorArgs {
    a: f64,
    b: f64,
    operation: String,
}
```

`#[derive(Deserialize)]` tells serde how to convert the JSON the LLM produces into this struct. The field names must match what you declare in the JSON schema (next step).

Then the tool struct itself:

```rust
#[derive(Deserialize, Serialize)]
struct Calculator;
```

No fields — this tool is stateless. It just does arithmetic.

---

## Step 3 — Implement the Tool trait

The `Tool` trait requires an associated `Error` type. That type must implement `std::error::Error` — Rust's standard trait for anything that represents an error. The trait has two required supertraits (`Debug` and `Display`) and no required methods of its own, but implementing it manually means writing `impl Display` and `impl Error` boilerplate for every error type you create.

**`thiserror`** is a crate that generates that boilerplate from a derive macro:

```rust
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
struct ToolError(String);
```

- `#[derive(thiserror::Error)]` — generates the `impl std::error::Error for ToolError` block.
- `#[derive(Debug)]` — satisfies the `Debug` supertrait requirement.
- `#[error("{0}")]` — generates `impl Display for ToolError` using the format string. `{0}` refers to the first field, so `ToolError("division by zero".to_string())` displays as `division by zero`.

Without `thiserror`, you would write this manually every time:

```rust
use std::fmt;

struct ToolError(String);

impl fmt::Display for ToolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ToolError {}
```

`thiserror` collapses all of that into two lines. You will see it in almost every Rust project that defines custom error types.

```rust
use rig::{completion::ToolDefinition, tool::Tool};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
struct ToolError(String);

impl Tool for Calculator {
    const NAME: &'static str = "calculator";

    type Error = ToolError;
    type Args = CalculatorArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Performs arithmetic. Use this for any calculation.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "a": {
                        "type": "number",
                        "description": "The first operand"
                    },
                    "b": {
                        "type": "number",
                        "description": "The second operand"
                    },
                    "operation": {
                        "type": "string",
                        "description": "One of: add, subtract, multiply, divide"
                    }
                },
                "required": ["a", "b", "operation"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let result = match args.operation.as_str() {
            "add"      => args.a + args.b,
            "subtract" => args.a - args.b,
            "multiply" => args.a * args.b,
            "divide" => {
                if args.b == 0.0 {
                    return Err(ToolError("division by zero".to_string()));
                }
                args.a / args.b
            }
            op => return Err(ToolError(format!("unknown operation: {op}"))),
        };
        Ok(result.to_string())
    }
}
```

The four associated types tell rig what flows through this tool:

- `Error` — the error type returned when `call` fails; must implement `std::error::Error`
- `Args` — the struct that `Deserialize` populates from the LLM's JSON
- `Output` — what `call` returns on success

`definition` is called once when the agent is built. Rig sends the `ToolDefinition` to the model as part of every request so the LLM knows the tool exists.

`call` is called by rig whenever the LLM decides to invoke the tool.

---

## Step 4 — The JSON schema

The `parameters` field in `ToolDefinition` is a JSON Schema object. The LLM reads it to know what arguments it must supply. Get it wrong and the LLM either does not call the tool or passes bad arguments.

The schema follows the [JSON Schema specification](https://json-schema.org/). For most tools you only need a small subset:

```json
{
  "type": "object",
  "properties": {
    "field_name": {
      "type": "string | number | boolean | array | object",
      "description": "What the LLM should put here"
    }
  },
  "required": ["field_name"]
}
```

The `description` on each property is what the LLM actually reads to understand how to fill it. Make it precise. "The first operand" is fine. "A number" is not — the LLM already knows it is a number from `type`.

---

## Step 5 — Add the tool to the agent

```rust
use rig::client::CompletionClient;
use rig::providers::openai::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::from_env()?;

    let agent = client
        .agent("gpt-4o-mini")
        .preamble("You are a calculator assistant. Use the calculator tool for all arithmetic.")
        .tool(Calculator)
        .build();

    let response = agent.prompt("What is 47 times 83?").await?;
    println!("{response}");

    Ok(())
}
```

`.tool(Calculator)` registers the tool. You can chain multiple `.tool()` calls (lesson 3 does this).

The prompt call works identically to lesson 1 — one `await`, one `String`. The tool loop happens inside rig, invisibly.

---

## Step 6 — What happens under the hood

When you call `agent.prompt("What is 47 times 83?")`:

1. Rig sends the user message to the API along with the tool definition.
2. The LLM recognises this requires arithmetic and outputs a tool call instead of plain text:
   ```json
   { "name": "calculator", "arguments": { "a": 47, "b": 83, "operation": "multiply" } }
   ```
3. Rig receives the tool call, deserialises the arguments into `CalculatorArgs`, and calls `Calculator::call()`.
4. `call()` returns `"3901"`.
5. Rig sends the tool result back to the API as a new message.
6. The LLM reads the result and writes its final reply: `"47 times 83 is 3901."`
7. Rig returns that string from `.prompt()`.

This is what the agent loop chapter described. Steps 2–5 can repeat multiple times in a single `.prompt()` call if the LLM needs to chain several tool calls.

---

## Full code

```rust
use rig::client::CompletionClient;
use rig::providers::openai::Client;
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
struct CalculatorArgs {
    a: f64,
    b: f64,
    operation: String,
}

#[derive(Deserialize, Serialize)]
struct Calculator;

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
struct ToolError(String);

impl Tool for Calculator {
    const NAME: &'static str = "calculator";

    type Error = ToolError;
    type Args = CalculatorArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Performs arithmetic. Use this for any calculation.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "a": {
                        "type": "number",
                        "description": "The first operand"
                    },
                    "b": {
                        "type": "number",
                        "description": "The second operand"
                    },
                    "operation": {
                        "type": "string",
                        "description": "One of: add, subtract, multiply, divide"
                    }
                },
                "required": ["a", "b", "operation"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let result = match args.operation.as_str() {
            "add"      => args.a + args.b,
            "subtract" => args.a - args.b,
            "multiply" => args.a * args.b,
            "divide" => {
                if args.b == 0.0 {
                    return Err(ToolError("division by zero".to_string()));
                }
                args.a / args.b
            }
            op => return Err(ToolError(format!("unknown operation: {op}"))),
        };
        Ok(result.to_string())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::from_env()?;

    let agent = client
        .agent("gpt-4o-mini")
        .preamble("You are a calculator assistant. Use the calculator tool for all arithmetic.")
        .tool(Calculator)
        .build();

    let response = agent.prompt("What is 47 times 83?").await?;
    println!("{response}");

    Ok(())
}
```

---

## Exercises

> **TODO 1**: Add a `power` operation that computes `a^b`. Use `f64::powf`. Ask the agent "what is 2 to the power of 10?" — it should use the tool and return 1024.
>
> **TODO 2**: Add a second tool `WordCounter` that takes a single `text: String` argument and returns the number of words in it as a string. Register both tools on the same agent. Ask "how many words are in 'the quick brown fox'?" — verify the LLM calls the right tool.
>
> **TODO 3**: Ask the agent something that requires no calculation — "what is the capital of France?". Does the LLM answer directly, or does it try to call the calculator? Confirm that tool definitions do not force the LLM to use them.

---

## Key APIs

| API | What it does |
|-----|-------------|
| `Tool` trait | The interface a type must implement to be used as an LLM tool |
| `Tool::NAME` | A `&'static str` the model uses to identify and invoke the tool |
| `Tool::Args` | The `Deserialize` type that receives the LLM's argument JSON |
| `Tool::Output` | The return type of a successful `call()` |
| `ToolDefinition` | Struct holding the tool's name, description, and parameter schema |
| `json!({ ... })` | `serde_json` macro for building JSON values inline |
| `.tool(t)` | Registers a tool on the agent builder |
