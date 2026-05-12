// Exercise 4.1 — returning Some and None
//
// `first_name` should return the name of the first contact if the
// slice is non-empty, or None if it is empty.
//
// Fix the function so all tests pass.

struct Contact {
    name: String,
    phone: String,
    email: String,
}

fn first_name(contacts: &[Contact]) -> Option<&str> {
    if contacts.is_empty() {
        None
    } else {
        contacts[0].name.as_str()  // BUG: missing Some(...)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make(name: &str) -> Contact {
        Contact { name: name.to_string(), phone: String::new(), email: String::new() }
    }

    #[test]
    fn empty_slice_gives_none() {
        let contacts: Vec<Contact> = Vec::new();
        assert_eq!(first_name(&contacts), None);
    }

    #[test]
    fn non_empty_gives_some() {
        let contacts = vec![make("Alice"), make("Bob")];
        assert_eq!(first_name(&contacts), Some("Alice"));
    }
}

fn main() {
    let contacts: Vec<Contact> = Vec::new();
    println!("{:?}", first_name(&contacts));
}
