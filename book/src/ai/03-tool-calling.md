# Chapter 3 — Tool Calling

An LLM can only generate text. It cannot open a file, make a network request, or write to a database. It lives entirely inside a text bubble: text goes in, text comes out. Tool calling is the mechanism that breaks that limitation.

---

## The problem: text in, text out

Send an LLM "what files are in the current directory?" and it will produce a fluent, confident-sounding response. But that response is not the real file list — it is a *guess* based on training data. The model has no connection to your filesystem. It cannot run `ls`. It cannot read a socket. It cannot check the time.

This matters the moment you want an AI assistant to do anything real: check a sensor reading, query a database, call an API, write to disk. Pure text generation cannot do any of those things.

The gap is not intelligence — it is execution. The LLM can reason about what to do. It just has no way to actually do it.

---

## The solution: structured output

Instead of generating a freeform response, the LLM can output a *structured* response that describes an action:

```json
{
  "name": "list_files",
  "arguments": { "path": "." }
}
```

Your code receives that JSON, executes `std::fs::read_dir(".")`, and feeds the result back to the LLM as a new message. The LLM reads the real output and continues from there.

The LLM did not execute anything. It produced a description of what to execute. You ran it. The result went back. The LLM kept going.

That is the entire mechanism. Everything else is details.

---

## The sequence

Three parties are involved: the user, the LLM, and your code.

```
User          LLM                        Your Code
  │             │                            │
  │─ "what      │                            │
  │   files     │                            │
  │   are here?"│                            │
  │────────────▶│                            │
  │             │                            │
  │             │  decides to call           │
  │             │  list_files(".")           │
  │             │───────────────────────────▶│
  │             │                            │ executes
  │             │                            │ std::fs::read_dir(".")
  │             │◀── ["main.rs",             │
  │             │     "Cargo.toml"]          │
  │             │                            │
  │             │  generates final response  │
  │◀────────────│                            │
  │  "You have  │                            │
  │   2 files:  │                            │
  │   main.rs,  │                            │
  │   Cargo.toml"                            │
```

The LLM never touches the filesystem. Your code does. The LLM only reads the result and decides what to say next.

---

## How the LLM knows what tools exist

Before the conversation begins, you send a list of available tools as part of the system prompt or the API request. Each tool has three things:

- **Name** — what to call it (`list_files`)
- **Description** — plain English explanation of what it does and when to use it
- **Parameters** — a JSON schema describing the arguments it accepts

The LLM reads these descriptions and uses them the same way a programmer reads documentation. When a user's message matches what a tool does, the LLM decides to call it. When nothing matches, it generates a regular text response.

The quality of your descriptions matters. A vague description produces wrong decisions. A clear one — "lists files in a directory, returns names only, not recursive" — tells the model exactly when to reach for this tool and what to expect back.

---

## The LLM does not execute anything

This is worth repeating because it is easy to misread the mechanism.

When the LLM "calls" a tool, it outputs JSON that *describes* the call. Nothing runs. No function is invoked. No system call is made. The LLM has produced a structured piece of text, the same way it might produce a JSON response in any other context.

Your code receives that JSON, parses it, dispatches to the real function, and sends the result back as another message.

```
LLM output ──▶  { "name": "list_files", "arguments": { "path": "." } }
                          │
                          │  your code reads this
                          ▼
                  call real Rust function
                  std::fs::read_dir(".")
                          │
                          ▼
                  send result back to LLM
                  ["main.rs", "Cargo.toml"]
```

The LLM is a **decision-maker**, not an executor. You write the functions. You decide which ones are available. You run them when asked. The LLM connects natural language to function calls.

---

## Why this is powerful

Natural language is imprecise. Function calls are precise. Tool calling is the bridge.

A user might say "show me the big files." That could mean files over 1 MB, files over 10 MB, the top 10 by size — it is ambiguous. But the LLM can interpret the intent and translate it into a precise sequence: call `list_files(".")`, get the names, call `get_file_size(name)` for each, sort the results, return the top ones.

You write ordinary Rust functions that each do one thing well. The LLM figures out which ones to call and in what order. Your functions know nothing about AI — they just read directories and return sizes. The intelligence is in the orchestration.

This separation is also what keeps the system maintainable. Adding a new capability means writing a new function and adding a description. The LLM learns to use it automatically.

---

## The Rust connection: the `Tool` trait

In rig.rs — the Rust library you will use — each tool is a struct that implements the `Tool` trait. The trait has two required methods:

- `definition()` — returns the JSON description of the tool (name, description, parameter schema). This is what gets sent to the LLM before the conversation starts.
- `call()` — executes the actual logic and returns the result. This is what runs when the LLM decides to use the tool.

If you have read the async chapters, this structure should feel familiar. A trait with required methods — the same pattern as `Iterator` or `Future`. The runtime (in this case rig.rs) drives things by calling your methods at the right time.

```
Tool trait
  │
  ├── definition()  →  sent to LLM before conversation
  │                    "here is what I can do and how to call me"
  │
  └── call()        →  run when LLM decides to use the tool
                       "the LLM chose me; now do the real work"
```

The full code for writing a tool comes in Lesson 2. For now, just notice that the pattern matches what you already know: a trait defines the contract, your struct provides the implementation.

---

## Safety: you are always in control

Because the LLM only outputs JSON describing what to call, your code runs between the decision and the execution. That gap is where you can add:

- **Confirmation prompts** — "The agent wants to delete 47 files. Allow? [y/N]"
- **Logging** — record every tool call and its result for auditing
- **Rate limiting** — prevent the LLM from calling expensive tools too frequently
- **Filtering** — block certain arguments entirely (e.g., reject paths outside the project directory)

The LLM cannot bypass these checks. It has no direct access to your system — only to the JSON it can output and the results you choose to send back.

This is the correct mental model: the LLM is an advisor that proposes actions. You are the system that decides whether to carry them out.

---

## Key ideas

| Concept | What it means |
|---------|---------------|
| Text bubble | LLMs can only read and produce text — they cannot run code or access systems directly |
| Tool call | A structured JSON response the LLM outputs to request a function call |
| Tool description | Name, plain-English purpose, and parameter schema sent to the LLM before the conversation |
| Execution gap | Your code sits between the LLM decision and the real function — you stay in control |
| `Tool` trait | rig.rs interface: `definition()` describes the tool, `call()` runs the logic |
| Decision vs execution | The LLM decides what to call; your code decides whether and how to run it |
