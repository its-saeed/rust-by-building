// Final Exercise 3 — implement delete_by_name
//
// `delete_by_name` should remove the first contact whose name matches
// the given string. If the contact was found and removed, return `true`.
// If no contact had that name, return `false`.
//
// Hint: `Vec` has a `.remove(index)` method that removes the element at
// the given index and shifts everything after it down.
// Combine it with `find_by_name` from lesson 4.
//
// All tests must pass.

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
}

fn find_by_name(contacts: &[Contact], name: &str) -> Option<usize> {
    for (i, contact) in contacts.iter().enumerate() {
        if contact.name == name {
            return Some(i);
        }
    }
    None
}

fn delete_by_name(contacts: &mut Vec<Contact>, name: &str) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(contacts: &[Contact]) -> Vec<&str> {
        contacts.iter().map(|c| c.name.as_str()).collect()
    }

    #[test]
    fn delete_existing() {
        let mut contacts = vec![
            Contact::new("Alice", "555-1234", "alice@example.com"),
            Contact::new("Bob",   "555-5678", "bob@example.com"),
            Contact::new("Carol", "555-9012", "carol@example.com"),
        ];
        assert_eq!(delete_by_name(&mut contacts, "Bob"), true);
        assert_eq!(names(&contacts), vec!["Alice", "Carol"]);
    }

    #[test]
    fn delete_first() {
        let mut contacts = vec![
            Contact::new("Alice", "555-1234", "alice@example.com"),
            Contact::new("Bob",   "555-5678", "bob@example.com"),
        ];
        assert_eq!(delete_by_name(&mut contacts, "Alice"), true);
        assert_eq!(names(&contacts), vec!["Bob"]);
    }

    #[test]
    fn delete_missing_returns_false() {
        let mut contacts = vec![
            Contact::new("Alice", "555-1234", "alice@example.com"),
        ];
        assert_eq!(delete_by_name(&mut contacts, "Dave"), false);
        assert_eq!(contacts.len(), 1);
    }

    #[test]
    fn delete_from_empty() {
        let mut contacts: Vec<Contact> = Vec::new();
        assert_eq!(delete_by_name(&mut contacts, "Alice"), false);
    }
}

fn main() {
    let mut contacts = vec![
        Contact::new("Alice", "555-1234", "alice@example.com"),
        Contact::new("Bob",   "555-5678", "bob@example.com"),
        Contact::new("Carol", "555-9012", "carol@example.com"),
    ];
    println!("before: {} contacts", contacts.len());
    delete_by_name(&mut contacts, "Bob");
    println!("after:  {} contacts", contacts.len());
    for c in &contacts {
        println!("  {}", c.name);
    }
}
