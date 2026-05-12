// Contact Book — step 5
//
// Make the contact book interactive.
//
// Add a `read_line()` helper that reads one line from stdin and returns
// the trimmed result as a String.
//
// Then replace the hardcoded contacts with a loop that accepts these commands:
//
//   add   — prompt for name, phone, email, then push a new Contact
//   find  — prompt for a name, search, print result or "Not found."
//   list  — print all contacts, or "No contacts." if empty
//   quit  — print "Goodbye." and exit
//   other — print "  Unknown command. Try: add, find, list, quit"
//
// Run it with `cargo run` and try it yourself.

use std::io::{self, Write};

struct Contact {
    name: String,
    phone: String,
    email: String,
}

impl Contact {
    fn new(name: &str, phone: &str, email: &str) -> Contact {
        Contact {
            name: name.to_string(),
            phone: phone.to_string(),
            email: email.to_string(),
        }
    }

    fn display(&self) {
        println!("  {} | {} | {}", self.name, self.phone, self.email);
    }
}

fn find_by_name(contacts: &[Contact], name: &str) -> Option<usize> {
    for (i, contact) in contacts.iter().enumerate() {
        if contact.name == name {
            return Some(i);
        }
    }
    None
}

// TODO: implement read_line() -> String

fn main() {
    let mut contacts: Vec<Contact> = Vec::new();

    println!("=== Contact Book ===");
    println!("Commands: add, find, list, quit");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        // TODO: read a command and dispatch on it
    }
}
