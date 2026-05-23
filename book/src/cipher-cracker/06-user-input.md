# Lesson 6 — User Input & Top-3 Guesses

> **Goal**: Build a loop that reads ciphertext from the user, cracks it, and runs until the user quits.
>
> **Concepts**: `Command` enum to model input, `loop` + `match`, finding top-3 with a loop, truncating display output.

---

## The `Command` enum

Instead of reading raw strings and guessing what the user wants, model their choices as an enum:

```rust
enum Command {
    Crack(String),   // a ciphertext to crack
    Quit,            // exit the program
}
```

`Crack` carries the ciphertext as a `String`. `Quit` carries nothing.

Reading a command:

```rust
fn read_command() -> Command {
    print!("> ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if input == "quit" || input == "q" {
        Command::Quit
    } else {
        Command::Crack(input.to_string())
    }
}
```

The main loop becomes clean and explicit:

```rust
loop {
    match read_command() {
        Command::Crack(ciphertext) => {
            // crack and display results
        }
        Command::Quit => {
            println!("Goodbye!");
            break;
        }
    }
}
```

`match` ensures both variants are handled. Adding a new command variant (e.g. `Help`) later will cause a compile error if the `match` doesn't cover it — the compiler enforces completeness.

---

## Finding the top-3 most frequent letters

A single frequency peak can be wrong on shorter texts. The robustness fix: try the **three most frequent letters** as candidates for `'e'`.

Without any fancy sorting, you can find the top-3 with three passes — each pass finds the best letter not already in the result:

```rust
fn top3_letters(freq: &HashMap<char, u32>) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    for _ in 0..3 {
        let mut best = ' ';
        let mut best_count = 0u32;
        for (&ch, &count) in freq {
            if count > best_count && !result.contains(&ch) {
                best_count = count;
                best = ch;
            }
        }
        if best_count > 0 {
            result.push(best);
        }
    }
    result
}
```

- Pass 1: find the most frequent letter overall → push it.
- Pass 2: find the most frequent letter *not already in result* → push it.
- Pass 3: same again.
- Stops early if fewer than 3 distinct letters exist.

---

## Cracking with top-3 candidates

```rust
fn crack_top3(ciphertext: &str) -> Vec<(u8, String)> {
    let freq = count_letters(ciphertext);
    let top = top3_letters(&freq);

    let mut guesses: Vec<(u8, String)> = Vec::new();
    for peak in top {
        let key = guess_key(peak);
        let decrypted = decrypt(ciphertext, key);
        guesses.push((key, decrypted));
    }
    guesses
}
```

For each of the top-3 letters, assume it's `'e'`, compute the key, decrypt, and collect the result.

---

## Truncating long output

Decrypted text can be very long. A preview function cuts it off after `max_chars` characters:

```rust
fn preview(text: &str, max_chars: usize) -> String {
    let mut result = String::new();
    let mut count = 0;
    for ch in text.chars() {
        if count >= max_chars {
            result.push_str("...");
            return result;
        }
        result.push(ch);
        count += 1;
    }
    result
}
```

Loop through each character. Once you've pushed `max_chars` characters, append `"..."` and return early. If the loop finishes before hitting the limit, the text was short enough — return it as-is.

---

## Putting it together

```rust
fn main() {
    println!("=== Cipher Cracker ===");
    println!("Enter ciphertext to crack, or 'quit' to exit.");
    println!();

    loop {
        match read_command() {
            Command::Crack(ciphertext) => {
                let guesses = crack_top3(&ciphertext);
                if guesses.is_empty() {
                    println!("No letters found.\n");
                } else {
                    println!("Top {} guesses:", guesses.len());
                    let mut i = 1;
                    for (key, decrypted) in &guesses {
                        println!("  {}. Key {:2}: {}", i, key, preview(decrypted, 60));
                        i += 1;
                    }
                    println!();
                }
            }
            Command::Quit => {
                println!("Goodbye!");
                break;
            }
        }
    }
}
```

Example session:

```
=== Cipher Cracker ===
Enter ciphertext to crack, or 'quit' to exit.

> dolfh zdv ehjlqqlqj wr jhw yhub wluhg ri vlwwlqj eb khu vlvwhu
Top 3 guesses:
  1. Key  3: alice was beginning to get very tired of sitting by he...
  2. Key 29: xebth wzv beceggbgc kf cek obho kbizy yp pbeebgc wv ad...
  3. Key 10: rogas nzm twceeedce kf awk fwjm kbjwz fy ybeeyme kn zw...

> quit
Goodbye!
```

The first guess is clearly English. The other two are nonsense.

---

## Exercises

Run:

```sh
rbb watch cipher-06
```

Four exercises. Then assemble the full interactive cracker in the project file.
