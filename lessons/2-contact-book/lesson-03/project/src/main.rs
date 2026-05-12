// Contact Book — step 3
//
// Add an `impl Contact` block with two functions:
//
//   fn new(name: &str, phone: &str, email: &str) -> Contact
//     — constructs a Contact from &str arguments
//
//   fn display(&self)
//     — prints "  name | phone | email"
//
// Then use Contact::new(...) instead of struct literals,
// and call contact.display() inside the loop.
//
// Expected output:
//
//   === Contact Book (3 contacts) ===
//     Alice | 555-1234 | alice@example.com
//     Bob | 555-5678 | bob@example.com
//     Carol | 555-9012 | carol@example.com

struct Contact {
    name: String,
    phone: String,
    email: String,
}

impl Contact {
    // TODO: add new() and display()
}

fn main() {
    let mut contacts: Vec<Contact> = Vec::new();

    // TODO: use Contact::new() to push three contacts

    println!("=== Contact Book ({} contacts) ===", contacts.len());
    for contact in &contacts {
        // TODO: call contact.display()
    }
}
