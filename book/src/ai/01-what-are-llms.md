# Chapter 1 — What LLMs Are

An LLM is a function that takes text in and produces text out. Everything else — the sense that it understands you, the feeling that it reasons — emerges from that one mechanical operation done at scale.

---

## Not magic

"Large language model" sounds technical and opaque. Strip the jargon and it is this: a very large mathematical function, trained on a very large amount of text, that predicts what word comes next.

That is the whole thing. Not a database. Not a reasoner. Not a mind. A next-word predictor, run in a loop.

This framing is not reductive — it is accurate, and it keeps you from being surprised when LLMs fail in specific, predictable ways. Understanding the mechanism makes the tool more useful, not less.

---

## What training means

An LLM is trained on an enormous corpus of text: web pages, books, code repositories, academic papers, forum posts, documentation. Hundreds of billions of words.

During training, the model is shown a sequence of text and asked a single question: *given everything that came before, what token comes next?* It makes a guess, checks against the actual next token, and adjusts its internal parameters to do better next time. This happens billions of times across the corpus until the model's predictions stop improving much.

The result is a file of numbers — billions of floating-point weights — that encode statistical relationships between tokens across all that text. Not facts. Not knowledge. Statistical patterns. The model has learned that certain sequences of tokens tend to follow other sequences.

---

## Prediction, not understanding

Think of the autocomplete on your phone. Type "I'll be there" and it suggests "soon" or "at 5" based on what you usually type next. It is not thinking. It is pattern-matching on your message history.

An LLM is autocomplete trained on all of human writing, able to predict paragraphs at a time instead of words. The output looks like understanding because human text contains human understanding, and the model has learned to reproduce those patterns faithfully.

```
┌──────────────────────────────────────────────────────────────┐
│  prompt:  "The capital of France is"                         │
│                                                              │
│  model sees:   [The] [capital] [of] [France] [is]           │
│                                                              │
│  question asked internally:  what token comes next?          │
│                                                              │
│  high probability:  " Paris"                                 │
│  lower probability: " Lyon"                                  │
│  very low probability: " a city"                             │
│                                                              │
│  output:  "Paris"   ← the highest-probability continuation  │
└──────────────────────────────────────────────────────────────┘
```

It produced "Paris" not because it knows Paris is the capital of France, but because in billions of sentences about France, "Paris" followed this pattern more than anything else.

---

## What a token is

The model does not operate on characters or words — it operates on **tokens**. A token is roughly a word-piece: somewhere between a syllable and a short word. "tokenization" splits text into these chunks before the model ever sees it.

For example: `"unhappy"` might become `["un", "happy"]`, and `"println!"` might become `["print", "ln", "!"]`.

Tokens are the atomic unit of everything the model reads and writes. Chapter 2 covers tokens in detail — how they are counted, what they cost, and why the size of the model's context window is measured in them.

---

## Why this produces useful behavior

Human knowledge is embedded in human text. Every tutorial, every Stack Overflow answer, every textbook, every paper — the model trained on all of it.

A model trained on thousands of Rust programming tutorials has seen how to explain ownership, borrowing, and lifetimes thousands of times. When you ask it to explain ownership, it synthesises a response that statistically resembles the best explanations in its training data. It does not understand ownership. But the output is often indistinguishable from output produced by someone who does.

This is genuinely useful. The model is, in a real sense, distilling patterns from an enormous amount of human expertise. That the mechanism is statistical rather than cognitive does not make the output less valuable.

---

## What LLMs are not

Three things LLMs are commonly mistaken for:

**Not a database.** An LLM does not look facts up. There is no index, no key-value store, no query engine. When you ask it who wrote a poem, it produces the most statistically likely answer based on patterns in training data. It might be wrong.

**Not deterministic.** The same prompt can produce different outputs on different runs. The model samples from a probability distribution over tokens — there is randomness in the selection. You can tune how random (via a parameter called *temperature*), but the output is not fixed.

**Not always right.** The model will produce confident-sounding text about things it is wrong about. This is called **hallucination** — the model generated a plausible-looking token sequence that happens to be false. It cannot tell the difference between a true statement and a false one that fits the statistical pattern. Verify outputs that matter.

```
┌──────────────────────────────────────────────────────────────┐
│  what LLMs are not                                           │
│                                                              │
│  ✗ database        — it does not "look things up"           │
│  ✗ deterministic   — same prompt, different outputs possible │
│  ✗ always correct  — hallucination is real and common        │
│                                                              │
│  ✓ next-token predictor                                      │
│  ✓ distilled from human text                                 │
│  ✓ useful when you know the limits                           │
└──────────────────────────────────────────────────────────────┘
```

---

## The context window

Every request to an LLM is stateless. The model only sees what you send it in that one request — there is no persistent memory between requests.

If you have a conversation with an LLM across ten messages, the client is sending all ten messages every time. The model does not remember your previous request. It reads the entire conversation history from the payload you submit.

This maps directly to something you built in the networking chapter. An LLM API endpoint behaves like a stateless HTTP handler:

```
┌───────────────────────────────────────────────────────────────┐
│  async fn handle_request(req: Request) -> Response            │
│                                                               │
│  ← request arrives with full conversation history in body    │
│                                                               │
│  handler:  reads prompt, generates next tokens, returns them  │
│                                                               │
│  → response leaves                                            │
│                                                               │
│  handler forgets everything  ←  no state persists            │
└───────────────────────────────────────────────────────────────┘
```

The maximum amount of text the model can see in one request — all the history you send plus the response it generates — is called the **context window**. It is measured in tokens and has a fixed upper limit that depends on the model. Chapter 2 has numbers.

Statelessness has an important implication: memory is your responsibility. If you want the model to remember something across sessions, you have to re-send it. If the conversation grows longer than the context window, something has to be dropped or summarised. There is no background persistence happening for you.

---

## Why this is relevant to Rust

In this section, you will give an LLM a set of Rust functions and let it decide which ones to call. You will write functions like `apply_force(body_id, direction, magnitude)` and `get_position(body_id)`. The LLM will read the names and descriptions of those functions, understand from the pattern what they do, and emit structured calls to them.

The "understanding" is the same statistical pattern-matching described above. But it works — reliably — because function names and documentation strings in code follow consistent patterns across all the code the model was trained on. It has seen thousands of physics APIs, thousands of game engine APIs, thousands of Rust APIs. The patterns are strong.

What you are building is not a chatbot. It is a system where natural language instructions get translated into real function calls on a live physics simulation. The LLM is the translator. The physics engine does the actual work.

---

## Key ideas

| Concept | What it means |
|---------|--------------|
| Next-token prediction | The one operation LLMs perform — predict what token comes next, given all previous tokens |
| Training | Adjusting billions of weights by repeatedly predicting next tokens across an enormous text corpus |
| Token | The unit the model reads and writes — roughly a word-piece, not a full word |
| Statistical patterns | The model learned that certain token sequences follow others; it has no explicit knowledge store |
| Hallucination | The model generating confident-sounding text that is factually wrong |
| Stateless | The model has no memory between requests; you must send the full context each time |
| Context window | The maximum tokens the model can see in one request — input plus output combined |
