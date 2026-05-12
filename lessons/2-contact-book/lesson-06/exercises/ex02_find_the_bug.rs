// Final Exercise 2 — find the bug
//
// This is a complete contact book implementation — but something is wrong.
// The tests fail. Find the bug and fix it.
//
// There is exactly one bug. Read the failing test output carefully.

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
            return Some(i + 1);   // BUG: off-by-one
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_first() {
        let contacts = vec![
            Contact::new("Alice", "555-1234", "alice@example.com"),
            Contact::new("Bob",   "555-5678", "bob@example.com"),
        ];
        assert_eq!(find_by_name(&contacts, "Alice"), Some(0));
    }

    #[test]
    fn find_second() {
        let contacts = vec![
            Contact::new("Alice", "555-1234", "alice@example.com"),
            Contact::new("Bob",   "555-5678", "bob@example.com"),
        ];
        assert_eq!(find_by_name(&contacts, "Bob"), Some(1));
    }

    #[test]
    fn not_found() {
        let contacts = vec![
            Contact::new("Alice", "555-1234", "alice@example.com"),
        ];
        assert_eq!(find_by_name(&contacts, "Dave"), None);
    }
}

fn main() {
    let contacts = vec![
        Contact::new("Alice", "555-1234", "alice@example.com"),
        Contact::new("Bob",   "555-5678", "bob@example.com"),
    ];
    println!("{:?}", find_by_name(&contacts, "Alice"));
    println!("{:?}", find_by_name(&contacts, "Bob"));
}
