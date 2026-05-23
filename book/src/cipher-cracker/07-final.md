# Final Exercise — Cipher Cracker End-to-End

> **No new concepts.** This is a test of everything from lessons 1–6.

---

## What you've built

| Lesson | Concept |
|--------|---------|
| 1 | `HashMap<char, u32>`, entry API, counting pattern |
| 2 | Traits: contracts, `Display`/`Debug`, custom traits, `Iterator` trait |
| 3 | Iterators: ranges, `.chars()`, `.count()`, `.sum()`, `.enumerate()`, `.take()`, `.skip()` |
| 4 | Enums: unit variants, data variants, `match`, `if let`, `CrackResult` |
| 5 | Frequency analysis: `top_letter`, `guess_key`, `decrypt`, `crack` |
| 6 | `Command` enum, interactive loop, top-3 guesses, preview |

---

## Exercise 1 — Implement from scratch

`exercises/ex01_implement.rs` has empty function bodies. Implement:

- `count_letters(text: &str) -> HashMap<char, u32>` — count lowercase alphabetic chars
- `top_letter(freq: &HashMap<char, u32>) -> Option<char>` — find the most frequent
- `guess_key(peak: char) -> u8` — compute the Caesar key assuming `peak` is encrypted `'e'`
- `crack(ciphertext: &str) -> CrackResult` — crack using the most frequent letter

All tests must pass. Use the compiler and test failures as your guide.

---

## Exercise 2 — Find the bug

`exercises/ex02_find_the_bug.rs` has a complete implementation with exactly one bug. The tests fail. Find it and fix it.

---

## Exercise 3 — Crack a tough one

`exercises/ex03_tough_cipher.rs` provides a ciphertext encrypted with an unusual key. The most frequent letter is **not** the encrypted `'e'` — the text has unusual statistics.

Implement `crack_best_of_26` which tries **all 26 possible keys** and returns the decryption that contains the most common English words (a short list is provided).

This shows the limits of single-frequency analysis and a brute-force alternative.

---

## Run the exercises

```sh
rbb watch cipher-07
```
