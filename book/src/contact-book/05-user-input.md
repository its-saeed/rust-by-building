# Lesson 5 — User Input

> **Goal**: Make the contact book interactive. Read commands from the terminal.
>
> **Concepts**: `use std::io`, `stdin().read_line()`, `.unwrap()`, `.trim()`, `loop {}`, `break`, matching string commands with `.as_str()`.

---

## Reading from the terminal

So far the data has been hardcoded. To let the user type commands, you need to read from standard input.

Rust's standard library provides this through `std::io`. First, bring it into scope:

```rust
use std::io;
```

Then read a line:

```rust
let mut input = String::new();
io::stdin().read_line(&mut input).unwrap();
```

Two steps:

1. `String::new()` — an empty buffer to read into
2. `io::stdin().read_line(&mut input)` — reads one line from stdin and appends it to `input`

`read_line` returns a `Result` — either success or an error. `.unwrap()` says "I expect this to succeed; if it fails, crash with the error message". For now, this is fine — stdin failure is rare and not worth complex handling.

---

## `.trim()` — removing the trailing newline

After `read_line`, the buffer contains the text the user typed **plus a trailing `\n`** (newline character). If the user typed `add` and pressed Enter, `input` is `"add\n"`, not `"add"`.

This breaks comparisons:

```rust
input == "add"    // false! because input is "add\n"
```

Fix it with `.trim()`:

```rust
let command = input.trim();   // returns &str with leading/trailing whitespace stripped
```

`.trim()` returns a `&str` — it doesn't allocate, it just narrows the view. If you need an owned `String`:

```rust
let command = input.trim().to_string();
```

A convenient pattern: a helper function that reads and trims in one step:

```rust
fn read_line() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
```

---

## `loop {}` — running forever until you say stop

An interactive program runs until the user decides to quit. Rust's `loop` keyword creates an infinite loop:

```rust
loop {
    // runs forever
}
```

To exit, use `break`:

```rust
loop {
    let input = read_line();
    if input == "quit" {
        break;
    }
    println!("You typed: {}", input);
}
```

`break` exits the innermost `loop`, `for`, or `while`. Execution continues after the closing `}`.

---

## Matching string commands

Once you have the trimmed input, dispatch on it with `match`:

```rust
let command = read_line();

match command.as_str() {
    "add"  => { /* add a contact */ }
    "find" => { /* find a contact */ }
    "list" => { /* list all contacts */ }
    "quit" => { break; }
    _      => { println!("  Unknown command. Try: add, find, list, quit"); }
}
```

`.as_str()` converts `String` to `&str`. This is needed because `match` on a `String` doesn't directly support string literal patterns — but `match` on `&str` does.

The `_` arm catches everything else. Always include it: without it, the compiler would reject the `match` because the set of possible strings is infinite.

---

## Prompting the user with `print!`

`println!` always adds a newline. For a prompt on the same line as the cursor:

```rust
print!("> ");
```

But `print!` doesn't flush the terminal buffer immediately. Without flushing, the prompt might not appear until after the user has already typed. Fix it:

```rust
use std::io::Write;   // brings flush() into scope

print!("> ");
io::stdout().flush().unwrap();
```

`io::stdout().flush()` forces the buffer to be written to the terminal now.

---

## Putting it together

The full interactive contact book:

```rust
use std::io::{self, Write};

struct Contact { /* ... */ }
impl Contact { /* ... */ }

fn find_by_name(contacts: &[Contact], name: &str) -> Option<usize> { /* ... */ }

fn read_line() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn main() {
    let mut contacts: Vec<Contact> = Vec::new();

    println!("=== Contact Book ===");
    println!("Commands: add, find, list, quit");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let command = read_line();

        match command.as_str() {
            "add" => {
                print!("  Name: ");  io::stdout().flush().unwrap();
                let name  = read_line();
                print!("  Phone: "); io::stdout().flush().unwrap();
                let phone = read_line();
                print!("  Email: "); io::stdout().flush().unwrap();
                let email = read_line();
                contacts.push(Contact::new(&name, &phone, &email));
                println!("  Added.");
            }
            "find" => {
                print!("  Name: "); io::stdout().flush().unwrap();
                let name = read_line();
                match find_by_name(&contacts, &name) {
                    Some(i) => contacts[i].display(),
                    None    => println!("  Not found."),
                }
            }
            "list" => {
                if contacts.is_empty() {
                    println!("  No contacts.");
                } else {
                    for contact in &contacts {
                        contact.display();
                    }
                }
            }
            "quit" => {
                println!("Goodbye.");
                break;
            }
            _ => println!("  Unknown command. Try: add, find, list, quit"),
        }
    }
}
```

---

## Exercises

Run:

```sh
rbb watch contact-05
```

Four exercises. Then assemble the full interactive program in the project.
