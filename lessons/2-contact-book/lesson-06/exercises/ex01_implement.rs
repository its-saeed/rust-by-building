// Final Exercise 1 — implement the contact book from scratch
//
// The function signatures are there with `todo!()` bodies.
// Implement all of them. All tests must pass.
//
// This is closed-book. Use the compiler errors and test failures as your guide.

struct Contact {
    name: String,
    phone: String,
    email: String,
}

impl Contact {
    fn new(name: &str, phone: &str, email: &str) -> Contact {
        todo!()
    }

    fn display(&self) {
        todo!()
    }
}

fn find_by_name(contacts: &[Contact], name: &str) -> Option<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_fields() {
        let c = Contact::new("Alice", "555-1234", "alice@example.com");
        assert_eq!(c.name, "Alice");
        assert_eq!(c.phone, "555-1234");
        assert_eq!(c.email, "alice@example.com");
    }

    #[test]
    fn find_returns_correct_index() {
        let contacts = vec![
            Contact::new("Alice", "555-1234", "alice@example.com"),
            Contact::new("Bob",   "555-5678", "bob@example.com"),
            Contact::new("Carol", "555-9012", "carol@example.com"),
        ];
        assert_eq!(find_by_name(&contacts, "Alice"), Some(0));
        assert_eq!(find_by_name(&contacts, "Bob"),   Some(1));
        assert_eq!(find_by_name(&contacts, "Carol"), Some(2));
    }

    #[test]
    fn find_returns_none_when_missing() {
        let contacts = vec![
            Contact::new("Alice", "555-1234", "alice@example.com"),
        ];
        assert_eq!(find_by_name(&contacts, "Dave"), None);
    }

    #[test]
    fn find_empty_slice() {
        let contacts: Vec<Contact> = Vec::new();
        assert_eq!(find_by_name(&contacts, "Alice"), None);
    }

    #[test]
    fn vec_holds_multiple() {
        let mut contacts: Vec<Contact> = Vec::new();
        contacts.push(Contact::new("Alice", "a", "a@a.com"));
        contacts.push(Contact::new("Bob",   "b", "b@b.com"));
        assert_eq!(contacts.len(), 2);
        assert_eq!(contacts[0].name, "Alice");
        assert_eq!(contacts[1].name, "Bob");
    }
}

fn main() {
    let mut contacts: Vec<Contact> = Vec::new();
    contacts.push(Contact::new("Alice", "555-1234", "alice@example.com"));
    contacts.push(Contact::new("Bob",   "555-5678", "bob@example.com"));

    for contact in &contacts {
        contact.display();
    }

    match find_by_name(&contacts, "Alice") {
        Some(i) => {
            print!("Found: ");
            contacts[i].display();
        }
        None => println!("Not found"),
    }
}
