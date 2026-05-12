// Exercise 2.1 — Vec requires `mut` to grow
//
// You cannot push into a Vec that was not declared with `let mut`.
// Fix this so it compiles and prints "1 contacts".

struct Contact {
    name: String,
    phone: String,
    email: String,
}

fn main() {
    let contacts: Vec<Contact> = Vec::new();   // BUG: missing mut

    contacts.push(Contact {
        name: String::from("Alice"),
        phone: String::from("555-1234"),
        email: String::from("alice@example.com"),
    });

    println!("{} contacts", contacts.len());
}

// No assertions — if it compiles and prints "1 contacts", you've solved it.
