// Exercise 1.2 — field names must match the struct definition exactly
//
// There is a typo in the struct literal below.
// Find it and fix it so the program compiles.

struct Contact {
    name: String,
    phone: String,
    email: String,
}

fn main() {
    let c = Contact {
        name: String::from("Alice"),
        fone: String::from("555-1234"),   // typo
        email: String::from("alice@example.com"),
    };
    println!("{}", c.phone);
}

// No assertions — if it compiles and prints "555-1234", you've solved it.
