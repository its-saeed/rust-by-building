# Chapter 2 — APIs and Tokens

Talking to an LLM from code is HTTP. That is the entire interface: you send a POST request with a JSON body, you get a JSON response. If you did the networking chapter, you already know the protocol.

---

## How you talk to an LLM

Every major LLM provider — OpenAI, Anthropic, Mistral, all of them — exposes an HTTP API. There is no special protocol, no SDK required to get started, no proprietary format. It is the same HTTP you used to build the TCP server and the download manager.

```
┌──────────────────────────────────────────────────────────────────┐
│  your Rust program                       LLM provider API        │
│                                                                  │
│  POST /v1/chat/completions  ──────────────────────────────────▶  │
│  Content-Type: application/json                                  │
│  Authorization: Bearer sk-...                                    │
│  { "model": "...", "messages": [...] }                           │
│                                                                  │
│  ◀──────────────────────────────────────────────────────────────  │
│  200 OK                                                          │
│  { "choices": [{ "message": { "content": "..." } }] }           │
└──────────────────────────────────────────────────────────────────┘
```

The networking chapter had you send raw HTTP requests and parse raw HTTP responses. The LLM API is exactly that, with a well-defined JSON schema on both ends.

---

## What an API key is

The `Authorization` header carries an API key — a long random string that identifies your account. It functions as a password. Whoever holds the key can make requests billed to your account.

**Never hardcode an API key in source code.** Not even in a private repo. Keys end up in git history, get leaked in screenshots, get committed by accident. The standard practice is to load them from environment variables.

Set one in your shell:

```sh
export OPENAI_API_KEY="sk-..."
```

Read it in Rust:

```rust
let api_key = std::env::var("OPENAI_API_KEY")
    .expect("OPENAI_API_KEY not set");
```

If the variable is missing, this panics with a readable message. Better to fail fast at startup than to get a confusing authentication error from the API later.

For local development, many projects use a `.env` file loaded by the `dotenvy` crate. The important thing is that the key never appears in source code.

---

## The request structure

A chat completion request has three required pieces:

**Model** — which model to use. Different models have different capabilities, context window sizes, and costs.

**Messages** — the conversation history as an array. Each message has a `role` and `content`. There are three roles: `system` (instructions to the model), `user` (what the user said), and `assistant` (what the model said in previous turns).

**System prompt** — technically just the first message with `"role": "system"`. This is where you tell the model what it is, what it should do, and how it should behave. It is instructions, not conversation.

```json
{
  "model": "gpt-4o-mini",
  "messages": [
    {
      "role": "system",
      "content": "You are a helpful assistant."
    },
    {
      "role": "user",
      "content": "What is 2 + 2?"
    }
  ]
}
```

A multi-turn conversation looks like this — you send the whole history every time:

```json
{
  "model": "gpt-4o-mini",
  "messages": [
    { "role": "system",    "content": "You are a helpful assistant." },
    { "role": "user",      "content": "What is 2 + 2?" },
    { "role": "assistant", "content": "2 + 2 = 4." },
    { "role": "user",      "content": "And 4 + 4?" }
  ]
}
```

There is no server-side session. You are reconstructing the full context on every request.

You can send this exact request right now with curl — no SDK, no code:

```sh
curl https://api.openai.com/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -d '{
    "model": "gpt-4o-mini",
    "messages": [
      { "role": "system", "content": "You are a helpful assistant." },
      { "role": "user",   "content": "What is 2 + 2?" }
    ]
  }'
```

The response comes back as raw JSON:

```json
{
  "choices": [{
    "message": {
      "role": "assistant",
      "content": "2 + 2 = 4."
    },
    "finish_reason": "stop"
  }],
  "usage": { "prompt_tokens": 24, "completion_tokens": 8, "total_tokens": 32 }
}
```

This is the whole interface. Everything rig.rs does — tool calling, agent loops, streaming — is built on top of this one HTTP endpoint.

---

## What a token is

The model does not read characters or words. It reads **tokens**.

A tokeniser splits input text into chunks before the model sees it. The chunks do not align with word boundaries — they are optimised for compression and consistency across different languages and writing styles. In English, a token is roughly 3–4 characters, or about 0.75 words.

```
"Hello"       →  1 token
"tokenize"    →  2 tokens:  "token"  +  "ize"
"Rust"        →  1 token
"println!"    →  2–3 tokens: "print"  +  "ln"  +  "!"
"unhappy"     →  2 tokens:  "un"  +  "happy"
```

The exact split depends on the tokeniser the model uses. OpenAI models use a tokeniser called tiktoken. You can test any string at [platform.openai.com/tokenizer](https://platform.openai.com/tokenizer).

Why does this matter? Two reasons:

1. **You pay per token.** Pricing is per million tokens, split between input and output. Every word you send costs tokens. Every word you receive costs tokens.
2. **The context window is measured in tokens.** There is a hard limit on how many tokens the model can process in one request. Send too much and the request fails, or the model truncates your history.

```
┌──────────────────────────────────────────────────────────────────┐
│  rough token counts                                              │
│                                                                  │
│  1 token      ≈  4 characters  ≈  0.75 English words            │
│  100 tokens   ≈  75 words      ≈  a short paragraph             │
│  1,000 tokens ≈  750 words     ≈  two pages of text             │
│  1M tokens    ≈  750,000 words ≈  a full novel                   │
└──────────────────────────────────────────────────────────────────┘
```

---

## Context window

The context window is the maximum number of tokens the model can see in a single request — input tokens plus output tokens combined.

For `gpt-4o-mini`, the context window is **128,000 tokens**, which is roughly 100,000 words, roughly the length of a novel. For most uses — conversations, code generation, tool calling — you will not come close to this limit. A typical tool-calling session for the AI physics sandbox is a few hundred to a few thousand tokens total.

Context window size matters for use cases like: analysing large codebases, processing long documents, maintaining long conversation histories. For this project, it is not a constraint you will run into.

---

## Cost model

Providers charge separately for input tokens (what you send) and output tokens (what the model generates). Output costs more — generating tokens requires running the full model; reading tokens is cheaper.

For `gpt-4o-mini` as of mid-2025:

```
┌──────────────────────────────────────────────────────────────────┐
│  gpt-4o-mini pricing                                             │
│                                                                  │
│  input tokens:   $0.15  per million tokens                       │
│  output tokens:  $0.60  per million tokens                       │
│                                                                  │
│  a typical request in this course:                               │
│    ~700 input tokens   = $0.000105                               │
│    ~300 output tokens  = $0.000180                               │
│    total per request   ≈ $0.0003                                 │
│                                                                  │
│  100 requests (a full day of experiments) ≈ $0.03               │
└──────────────────────────────────────────────────────────────────┘
```

For educational use, the cost of running all the examples in this section is fractions of a cent per session. It is not a meaningful expense.

That said, it is good practice to track `usage` in every response (covered below) so you know what requests actually cost. Runaway loops or very large prompts can add up if you are not paying attention.

---

## The response structure

The API returns JSON. The structure looks like this:

```json
{
  "id": "chatcmpl-abc123",
  "object": "chat.completion",
  "choices": [
    {
      "index": 0,
      "message": {
        "role": "assistant",
        "content": "2 + 2 = 4."
      },
      "finish_reason": "stop"
    }
  ],
  "usage": {
    "prompt_tokens": 24,
    "completion_tokens": 10,
    "total_tokens": 34
  }
}
```

The parts you actually use:

- `choices[0].message.content` — the model's reply
- `choices[0].finish_reason` — why the model stopped: `"stop"` means it finished naturally, `"length"` means it hit the token limit, `"tool_calls"` means it wants to call a function (next chapter)
- `usage.prompt_tokens` and `usage.completion_tokens` — how many tokens this request consumed

The response is the message you append to the conversation history before sending the next request. You build the running context manually.

---

## Why we use rig.rs

You could do all of this by hand with `reqwest`. After the networking chapter you have the skills — serialize the request with `serde_json`, POST it, deserialize the response, extract `choices[0].message.content`. It is maybe 50 lines.

But that approach breaks down quickly as soon as you need **tool calling** — the ability for the LLM to call your Rust functions. Tool calling requires serialising your function signatures into JSON schemas, parsing tool call responses, routing them to the right Rust function, collecting results, re-sending to the model. Writing that correctly by hand is several hundred lines of careful, tedious code.

**`rig.rs`** is a Rust library that wraps all of this:

```
┌──────────────────────────────────────────────────────────────────┐
│  what rig.rs handles for you                                     │
│                                                                  │
│  ✓ HTTP auth and request formatting                              │
│  ✓ JSON serialisation / deserialisation                          │
│  ✓ Tool registration — describing your functions to the model    │
│  ✓ Tool call parsing — reading what function the model picked    │
│  ✓ The agent loop — calling tools and feeding results back       │
│  ✓ Multiple providers — swap OpenAI for Anthropic in one line    │
└──────────────────────────────────────────────────────────────────┘
```

The relationship is the same as `tokio` to raw `epoll`: you could write the event loop yourself, but the library is better-tested, handles edge cases you have not thought of yet, and lets you focus on what you are actually building.

The next chapter covers tool calling — what it is, how the model uses it, and how you wire up Rust functions so the model can invoke them.

---

## Key ideas

| Concept | What it means |
|---------|--------------|
| LLM API | HTTP endpoint — POST JSON in, JSON back; the same protocol from the networking chapter |
| API key | Authentication token; always load from environment variables, never hardcode |
| System prompt | The first `"role": "system"` message — instructions that frame how the model behaves |
| Messages array | Full conversation history sent on every request; the API is stateless |
| Token | The unit the model reads and writes — roughly 0.75 English words or 4 characters |
| Context window | Maximum tokens in one request (input + output); 128k for gpt-4o-mini |
| Usage field | The response field that tells you how many tokens the request consumed |
| rig.rs | Rust library that wraps LLM HTTP APIs and implements tool calling — tokio for AI |
