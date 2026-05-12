// Exercise 4.2 — match on Option must cover both arms
//
// This does not compile: the match is missing the `None` arm.
// Add it so the function handles both cases.

struct Contact {
    name: String,
    phone: String,
    email: String,
}

fn describe(contacts: &[Contact], index: usize) -> &str {
    let item = contacts.get(index);   // returns Option<&Contact>

    match item {
        Some(c) => c.name.as_str(),
        // BUG: None arm is missing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make(name: &str) -> Contact {
        Contact { name: name.to_string(), phone: String::new(), email: String::new() }
    }

    #[test]
    fn found() {
        let contacts = vec![make("Alice")];
        assert_eq!(describe(&contacts, 0), "Alice");
    }

    #[test]
    fn not_found() {
        let contacts = vec![make("Alice")];
        assert_eq!(describe(&contacts, 99), "not found");
    }
}

fn main() {
    let contacts: Vec<Contact> = Vec::new();
    println!("{}", describe(&contacts, 0));
}
