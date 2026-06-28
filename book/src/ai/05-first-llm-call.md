# Lesson 1 — First LLM Call

> **Goal**: Send a prompt to an LLM and print the response.
>
> **Concepts**: rig setup, `OPENAI_API_KEY`, `Client`, `Agent`, `.prompt().await`.

The theory chapters explained what LLMs are, how the API works over HTTP, and what tokens cost. Now you write the code that does it.

---

## Step 1 — Setup

Create a new binary project and add these dependencies to `Cargo.toml`:

```toml
[dependencies]
rig = "0.38.2"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
anyhow = "1"
```

You also need an API key from [platform.openai.com](https://platform.openai.com). The key identifies your account — every API request is billed to it. Never put it in source code.

Set it as an environment variable before running your program:

```sh
export OPENAI_API_KEY="sk-..."
```

Or for a single run:

```sh
OPENAI_API_KEY="sk-..." cargo run
```

---

## Step 2 — Hello LLM

```rust
use rig::client::ProviderClient;
use rig::providers::openai::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::from_env()?;

    let agent = client
        .agent(openai::GPT_4O_MINI)
        .preamble("You are a helpful assistant.")
        .build();

    let response = agent.prompt("What is the capital of France?").await?;
    println!("{response}");

    Ok(())
}
```

`Client::from_env()?` reads `OPENAI_API_KEY` from the environment. If the variable is missing it panics with a clear message.

`agent.prompt(text).await` makes an HTTP POST to `https://api.openai.com/v1/chat/completions`, waits for the response, and returns the assistant's reply as a `String`. Everything else — headers, JSON encoding, retries on transient errors — is handled by rig.

---

## Step 3 — The system prompt

The string passed to `.preamble()` is the system prompt. It is the first message the LLM reads, before every user message you send. It sets the agent's behaviour for the entire conversation.

Compare these two agents:

```rust
let generic = client
    .agent(openai::GPT_4O_MINI)
    .preamble("You are a helpful assistant.")
    .build();

let specialist = client
    .agent(openai::GPT_4O_MINI)
    .preamble("You are a Rust expert. Answer every question with Rust code examples.")
    .build();

let prompt = "How do I reverse a list?";

println!("{}", generic.prompt(prompt).await?);
println!("---");
println!("{}", specialist.prompt(prompt).await?);
```

Same question. Very different answers. The preamble is the primary lever for shaping what the model does.

---

## Step 4 — What .prompt() is doing

`.prompt()` returns a `String`. That is the entire public interface — rig handles everything underneath.

Under the hood it posts a JSON body like:

```json
{
  "model": "gpt-4o-mini",
  "messages": [
    { "role": "system",    "content": "You are a helpful assistant." },
    { "role": "user",      "content": "What is the capital of France?" }
  ]
}
```

You learned about this endpoint in the APIs and tokens chapter. Rig constructs this JSON, sends the request over HTTPS, parses the response, and extracts the `content` field from the first choice. The `String` you receive is that content.

---

## Step 5 — A simple REPL

A single prompt is useful for testing. A REPL is the foundation of everything in the following lessons.

```rust
use rig::client::ProviderClient;
use rig::providers::openai::Client;
use std::io::{self, BufRead, Write};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::from_env()?;

    let agent = client
        .agent(openai::GPT_4O_MINI)
        .preamble("You are a helpful assistant.")
        .build();

    let stdin = io::stdin();
    let mut count = 0u32;

    loop {
        count += 1;
        print!("[{count}] > ");
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

`stdin.lock().read_line(&mut line)` blocks until the user presses Enter. The program then sends that line to the LLM and prints the response. The loop repeats until the process is killed.

Note the `count` variable — it tracks how many messages have been sent and prints the number in the prompt. This becomes useful for debugging in later lessons.

---

## Full code

```rust
use rig::client::ProviderClient;
use rig::providers::openai::Client;
use std::io::{self, BufRead, Write};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::from_env()?;

    let agent = client
        .agent(openai::GPT_4O_MINI)
        .preamble("You are a helpful assistant.")
        .build();

    let stdin = io::stdin();
    let mut count = 0u32;

    loop {
        count += 1;
        print!("[{count}] > ");
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

> **TODO 1**: Change the system prompt to make the agent respond only in rhymes. Send a few messages and verify the behaviour changes.
>
> **TODO 2**: The `count` variable already tracks messages sent. Change the display so it shows something like `[message 3 of this session]` instead of just `[3]`.
>
> **TODO 3**: If the user types `quit` or `exit`, break the loop cleanly and print `"Goodbye."` before exiting. Use `break` rather than `std::process::exit`.

---

## Key APIs

| API | What it does |
|-----|-------------|
| `Client::from_env()?` | Creates a client using `OPENAI_API_KEY` from the environment |
| `client.agent(model)` | Starts an agent builder for the given model |
| `.preamble(text)` | Sets the system prompt — sent before every user message |
| `.build()` | Finalises the agent |
| `agent.prompt(text).await` | Sends a user message and returns the assistant's reply as a `String` |
| `openai::GPT_4O_MINI` | Constant for the `"gpt-4o-mini"` model identifier |
