// Contact Book — step 4
//
// Add a `find_by_name` function:
//
//   fn find_by_name(contacts: &[Contact], name: &str) -> Option<usize>
//
// It should search the slice and return Some(index) if found, None if not.
//
// Then demonstrate it in main: search for a name that exists and one that
// doesn't, and print the result of each search.
//
// Expected output (example):
//
//   === Contact Book ===
//     Alice | 555-1234 | alice@example.com
//     Bob | 555-5678 | bob@example.com
//   ---
//   Search: Alice
//   Found:
//     Alice | 555-1234 | alice@example.com
//   ---
//   Search: Dave
//     'Dave' not found.

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

// TODO: implement find_by_name

fn main() {
    let mut contacts: Vec<Contact> = Vec::new();
    contacts.push(Contact::new("Alice", "555-1234", "alice@example.com"));
    contacts.push(Contact::new("Bob",   "555-5678", "bob@example.com"));

    println!("=== Contact Book ===");
    for contact in &contacts {
        contact.display();
    }

    // TODO: search for "Alice" and "Dave", print results
}
