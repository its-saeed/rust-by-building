// Exercise 4.4 — implement find_by_name
//
// `find_by_name` searches the slice for a contact whose `name` equals
// the given string. If found, return `Some(index)`. If not, return `None`.
//
// Use `.iter().enumerate()` to get both the index and the contact.
// All tests must pass.

struct Contact {
    name: String,
    phone: String,
    email: String,
}

fn find_by_name(contacts: &[Contact], name: &str) -> Option<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make(name: &str) -> Contact {
        Contact { name: name.to_string(), phone: String::new(), email: String::new() }
    }

    #[test]
    fn finds_first() {
        let contacts = vec![make("Alice"), make("Bob"), make("Carol")];
        assert_eq!(find_by_name(&contacts, "Alice"), Some(0));
    }

    #[test]
    fn finds_middle() {
        let contacts = vec![make("Alice"), make("Bob"), make("Carol")];
        assert_eq!(find_by_name(&contacts, "Bob"), Some(1));
    }

    #[test]
    fn finds_last() {
        let contacts = vec![make("Alice"), make("Bob"), make("Carol")];
        assert_eq!(find_by_name(&contacts, "Carol"), Some(2));
    }

    #[test]
    fn not_found_returns_none() {
        let contacts = vec![make("Alice"), make("Bob")];
        assert_eq!(find_by_name(&contacts, "Dave"), None);
    }

    #[test]
    fn empty_slice_returns_none() {
        let contacts: Vec<Contact> = Vec::new();
        assert_eq!(find_by_name(&contacts, "Alice"), None);
    }

    #[test]
    fn case_sensitive() {
        let contacts = vec![make("Alice")];
        assert_eq!(find_by_name(&contacts, "alice"), None);  // lowercase doesn't match
    }
}

fn main() {
    let contacts = vec![
        Contact { name: String::from("Alice"), phone: String::from("555-1234"), email: String::from("a@example.com") },
        Contact { name: String::from("Bob"),   phone: String::from("555-5678"), email: String::from("b@example.com") },
    ];
    match find_by_name(&contacts, "Bob") {
        Some(i) => println!("Found at index {}: {}", i, contacts[i].name),
        None    => println!("Not found"),
    }
}
