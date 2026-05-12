// Exercise 1.3 — field access is case-sensitive
//
// Rust field names are lowercase by convention.
// This program does not compile because the field access uses the wrong case.
// Fix it.
//
// Expected output:
//   alice@example.com

struct Contact {
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
    println!("{}", c.Email);   // wrong case
}

// No assertions — if it compiles and prints the email, you've solved it.
