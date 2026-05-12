// Exercise 1.1 — struct definition syntax
//
// This program does not compile.
// A struct body must be wrapped in `{ }`.
// Fix the definition so it compiles and prints the name.

struct Contact
    name: String,
    phone: String,
    email: String,
}

fn main() {
    let c = Contact {
        name: String::from("Alice"),
        phone: String::from("555-1234"),
        email: String::from("alice@example.com"),
    };
    println!("{}", c.name);
}

// No assertions — if it compiles and prints "Alice", you've solved it.
