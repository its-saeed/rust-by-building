# Chapter 4 — The Agent Loop

Tool calling (chapter 3) is one round-trip: LLM decides, your code runs, result goes back. An agent is that round-trip running in a loop until the work is done.

---

## One tool call is not enough

Most real requests require several steps. "Rename all .txt files to .md" is not a single action. It is a sequence:

1. List the files in the directory
2. Filter for names ending in `.txt`
3. Rename each one

No single tool call can do this. The LLM needs to see the file list before it knows what to rename. It needs to see the result of the first rename before it runs the next one. Each step depends on information from the previous step.

This is why the single round-trip from chapter 3 is not enough on its own. Real tasks need multiple turns, and something needs to manage those turns. That something is the agent loop.

---

## The agent loop

The loop has one rule: keep calling the LLM until it stops requesting tools.

```
┌─────────────────────────────────────────────────────┐
│                   agent loop                        │
│                                                     │
│   ┌─ LLM receives: prompt + tool results so far ─┐  │
│   │                                              │  │
│   │  Does LLM want to call a tool?               │  │
│   │                                              │  │
│   │   YES → execute tool → append result → loop  │  │
│   │                                              │  │
│   │   NO  → generate final response → done       │  │
│   └──────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────┘
```

On each iteration, the LLM sees everything: the original user message, every tool call it made, and every result that came back. It uses this growing history to decide what to do next. When it has enough information to answer, it produces a regular text response with no tool call — and the loop ends.

---

## A concrete example

"Add milk, eggs, and bread to my todo list."

Three separate items. The LLM does not add them all in one call — `add_todo` takes one item at a time. So it loops:

```
Turn 1:  LLM calls add_todo("milk")
         → result: "added"

Turn 2:  LLM calls add_todo("eggs")
         → result: "added"

Turn 3:  LLM calls add_todo("bread")
         → result: "added"

Turn 4:  LLM sees all three succeeded
         → "Added 3 items to your list."
         no tool call — loop ends
```

From the user's perspective, they typed one sentence and got one reply. From the inside, the loop ran four times over four async HTTP requests.

---

## Observation before action

A useful agent behavior: call a query tool *before* deciding what to do.

"What's running slow in my simulation?" — the LLM cannot answer that without data. So it calls `get_world_stats()` first, reads the result (frame time: 48 ms, active bodies: 3,200), and then decides to call `set_time_scale(0.5)` to slow the simulation down.

The query-then-act pattern shows up constantly in well-designed agents. It means you need two kinds of tools:

- **Query tools** — read state, return information, change nothing
- **Action tools** — modify state, produce effects in the world

When a user says something ambiguous or context-dependent, the agent can call a query tool to orient itself before committing to an action. The LLM decides when to do this on its own — you just make sure the right query tools are available.

---

## When does the loop end?

There are three exit conditions:

- **No tool call in the response** — the LLM produced a regular text reply. Work is done.
- **Unrecoverable tool error** — a tool returned an error the LLM cannot work around. The error propagates up to your code.
- **Maximum iteration limit** — a safety cutoff to prevent infinite loops. rig.rs enforces this automatically.

The third case matters in practice. A poorly described tool or an ambiguous request can put the LLM in a loop where it keeps trying the same tool with slightly different arguments and never converges. The iteration limit is a guardrail, not an edge case.

---

## Why "agent" feels autonomous

From the outside, the agent appears to be thinking — reading your files, making decisions, taking actions, responding to what it finds. It feels like a program with its own judgment.

From the inside: it is a `while` loop calling an HTTP endpoint.

The LLM does not run continuously in the background. It does not have memory between calls beyond what you send it. Each call to the LLM is stateless — it sees only what is in the current message history. The agent *feels* persistent because the loop accumulates results and includes them in every subsequent call. The appearance of memory is constructed by your code, not built into the model.

Understanding this makes the system easier to debug. If the agent does something wrong, you can look at the exact message history it was given at each turn. The decision that produced the bad output is right there in the JSON.

---

## The connection to async

You already know all the mechanics needed to implement this.

Each call to the LLM is an async HTTP request — the same kind you used in the download manager. Each tool execution is an async function. The agent loop is an async loop. Conceptually, it looks like this:

```rust
// conceptually, the agent loop looks like this
loop {
    let response = llm.call(messages).await?;

    match response {
        Response::ToolCall(call) => {
            let result = execute_tool(call).await?;
            messages.push(result);
        }
        Response::Done(text) => {
            println!("{text}");
            break;
        }
    }
}
```

The `llm.call(messages).await?` is an HTTP request, just like `client.get(url).send().await?`. The `.await` yields to the Tokio runtime while the network is busy — exactly the same as every other async operation you have written.

You do not have to write this loop yourself. rig.rs implements it. But knowing what is inside tells you exactly what is happening when the agent runs.

---

## What rig.rs gives you

Three things:

- **Client** — manages authentication and HTTP. Wraps the API key, constructs requests, handles retries. You create it once and hand it to the agent.
- **Agent** — holds your tools, your system prompt, and runs the loop. You give it tools and a prompt; it handles everything else.
- **`Tool` trait** — the interface you implement. `definition()` describes your tool to the LLM; `call()` runs the real logic when the LLM chooses it.

```
rig.rs
  │
  ├── Client      manages auth + HTTP + retries
  │
  ├── Agent       holds tools + prompt + runs the loop
  │                 └── calls Tool::definition() once at startup
  │                 └── calls Tool::call() each time LLM requests it
  │
  └── Tool trait  you implement this
                    ├── definition() → JSON description for the LLM
                    └── call()       → your actual Rust logic
```

The next lesson shows the first real code: a client, an agent, and a tool that interacts with the physics sandbox.

---

## Key ideas

| Concept | What it means |
|---------|---------------|
| Agent loop | Repeatedly call the LLM until it produces a response with no tool call |
| Message history | Every tool call and result is appended and sent on the next turn — this is the agent's "memory" |
| Query tools | Tools that read state without changing it — let the agent observe before acting |
| Action tools | Tools that modify state — the agent calls these once it has enough information |
| Iteration limit | A maximum number of loop turns; prevents runaway agents |
| Stateless LLM | The model has no memory between calls — the loop constructs continuity by accumulating history |
| `Client` / `Agent` / `Tool` | The three rig.rs primitives you build with |
