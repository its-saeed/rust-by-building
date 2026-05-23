# Lesson 5 — Frequency Analysis

> **Goal**: Use the most frequent letter to deduce the key and decrypt the ciphertext automatically.
>
> **Concepts**: English letter frequencies, finding the maximum in a HashMap, computing the Caesar key from a frequency peak, decryption with a `for` loop, returning `CrackResult`.

---

## Why frequency analysis works

Every natural language has patterns. In English:

| Rank | Letter | Frequency |
|------|--------|-----------|
| 1 | e | ~13% |
| 2 | t |  ~9% |
| 3 | a |  ~8% |
| 4 | o |  ~7% |
| 5 | i |  ~7% |
| 6 | n |  ~7% |

`'e'` is by far the most common. In any long enough English text, roughly 1 in 8 letters is `'e'`.

A Caesar cipher shifts every letter by the same amount. It does not change frequencies — it just relabels them. If `'e'` appears 13% of the time in the plaintext, whatever letter `'e'` was shifted to appears 13% of the time in the ciphertext.

**The insight**: the most frequent letter in the ciphertext was almost certainly `'e'` before encryption.

---

## Finding the most frequent letter

From lesson 1, you can count all letters into a `HashMap<char, u32>`. To find the most frequent one, loop through the map and track the maximum:

```rust
fn top_letter(freq: &HashMap<char, u32>) -> Option<char> {
    let mut best = ' ';
    let mut best_count = 0u32;
    for (&ch, &count) in freq {
        if count > best_count {
            best_count = count;
            best = ch;
        }
    }
    if best_count > 0 { Some(best) } else { None }
}
```

- `for (&ch, &count) in freq` — iterates over all `(key, value)` pairs in the map. The `&` in `(&ch, &count)` dereferences the references so `ch` is a `char` and `count` is a `u32`, not references to them.
- Returns `None` if the map is empty (no letters found), or `Some(ch)` with the most frequent letter.

---

## Computing the key

Caesar encryption shifts every letter forward by `key` positions:

```
encrypted = (original + key) mod 26
```

So if you know what a ciphertext letter `peak` was before encryption (we assume `'e'`), you can reverse the formula:

```
peak = (e + key) mod 26
key  = (peak - e) mod 26
```

In code, we work with the ASCII byte values of the characters. The ASCII codes for lowercase letters run from `'a'` = 97 to `'z'` = 122. `b'e'` is just Rust's way of writing the byte value of `'e'` (101):

```rust
fn guess_key(peak: char) -> u8 {
    let peak = peak.to_ascii_lowercase() as u8;
    (peak + 26 - b'e') % 26
}
```

**Line by line:**

`let peak = peak.to_ascii_lowercase() as u8;`

Convert the character to lowercase (in case it's uppercase), then cast it to its ASCII byte value. For example `'H'` → `'h'` → `104`.

`(peak + 26 - b'e') % 26`

This is the formula `(peak - e) mod 26` — but written safely to avoid underflow. Here's why each part is needed:

- `b'e'` is `101` — the ASCII value of `'e'`
- `peak - b'e'` would give the distance between `peak` and `'e'` in the alphabet
- **The problem**: if `peak` comes *before* `'e'` in the alphabet (like `'a'` = 97), then `97 - 101 = -4`, which underflows a `u8`
- **The fix**: add 26 first. Since we take `% 26` at the end, adding 26 doesn't change the final result — it just keeps the arithmetic from going negative

**Step-by-step example with `peak = 'h'`:**

| Step | Expression | Value |
|------|-----------|-------|
| ASCII of `'h'` | `peak` | 104 |
| Add 26 | `peak + 26` | 130 |
| Subtract `b'e'` (101) | `130 - 101` | 29 |
| Modulo 26 | `29 % 26` | **3** ← the key |

**Another example with `peak = 'a'`** (comes before `'e'`):

| Step | Expression | Value |
|------|-----------|-------|
| ASCII of `'a'` | `peak` | 97 |
| Add 26 | `peak + 26` | 123 |
| Subtract `b'e'` (101) | `123 - 101` | 22 |
| Modulo 26 | `22 % 26` | **22** ← the key |

You can verify: `'e'` + 22 positions = `'e'`(4) + 22 = position 26 = `'a'` (wrapping around). ✓

---

## Decryption with a `for` loop

Decryption is the reverse of encryption — instead of shifting forward by `key`, shift backward. Shifting forward by `26 - key` achieves the same result:

```rust
fn decrypt(text: &str, key: u8) -> String {
    let shift = (26 - key % 26) as u8;
    let mut result = String::new();
    for c in text.chars() {
        let ch = match c {
            'A'..='Z' => (b'A' + (c as u8 - b'A' + shift) % 26) as char,
            'a'..='z' => (b'a' + (c as u8 - b'a' + shift) % 26) as char,
            _ => c,
        };
        result.push(ch);
    }
    result
}
```

- `key % 26` handles the edge case where `key == 26` (a 26-shift is the same as no shift).
- Non-letter characters (`' '`, `','`, etc.) fall through the `_` arm and are pushed unchanged.
- Each character is pushed into `result` one at a time.

---

## The `crack` function

Now that you know enums, `crack` returns `CrackResult` instead of a plain `String`. This makes the two outcomes explicit:

```rust
fn crack(ciphertext: &str) -> CrackResult {
    let freq = count_letters(ciphertext);
    match top_letter(&freq) {
        Some(peak) => {
            let key = guess_key(peak);
            let plaintext = decrypt(ciphertext, key);
            CrackResult::Success { key, plaintext }
        }
        None => CrackResult::TooFewLetters,
    }
}
```

The caller is forced by the compiler to handle both variants — there's no way to accidentally ignore the failure case.

---

## Putting it together

After lesson 5, the project cracks the hardcoded ciphertext:

```rust
fn main() {
    let ciphertext = "dolfh zdv ehjlqqlqj wr jhw yhub wluhg ri vlwwlqj eb khu vlvwhu rq wkh edqn dqg ri kdylqj qrwklqj wr gr rqfh ru wzlfh vkh kdg shhshg lqwr wkh errn khu vlvwhu zdv uhdglqj exw lw kdg qr slfwxuhv ru frqyhuvdwlrqv lq lw dqg zkdw lv wkh xvh ri d errn wkrxjkw dolfh zlwkrxw slfwxuhv ru frqyhuvdwlrqv";

    println!("=== Cipher Cracker ===");

    match crack(ciphertext) {
        CrackResult::Success { key, plaintext } => {
            println!("Key {}: {}", key, plaintext);
        }
        CrackResult::TooFewLetters => {
            println!("Not enough letters to analyse.");
        }
    }
}
```

Expected output:

```
=== Cipher Cracker ===
Key 3: alice was beginning to get very tired of sitting by her sister on the bank...
```

---

## When it goes wrong

Frequency analysis needs **enough text** to be reliable. On short texts (fewer than ~50 letters), the most frequent letter might not be the encrypted `'e'`. The fix: try the top 3 most frequent letters as candidates. This is lesson 6.

---

## Exercises

Run:

```sh
rbb watch cipher-05
```

Four exercises. Then add `guess_key`, `decrypt`, and `crack` to the project.
