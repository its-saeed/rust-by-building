// Exercise 2.4 — implement a function that counts contacts
//
// `count_contacts` should return the number of contacts in the slice.
// Implement it so all tests pass.
//
// Hint: slices have a `.len()` method.

struct Contact {
    name: String,
    phone: String,
    email: String,
}

fn count_contacts(contacts: &[Contact]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make(name: &str) -> Contact {
        Contact {
            name: name.to_string(),
            phone: String::from("555-0000"),
            email: String::from("x@x.com"),
        }
    }

    #[test]
    fn empty() {
        let contacts: Vec<Contact> = Vec::new();
        assert_eq!(count_contacts(&contacts), 0);
    }

    #[test]
    fn one() {
        let contacts = vec![make("Alice")];
        assert_eq!(count_contacts(&contacts), 1);
    }

    #[test]
    fn three() {
        let contacts = vec![make("Alice"), make("Bob"), make("Carol")];
        assert_eq!(count_contacts(&contacts), 3);
    }
}

fn main() {
    let contacts = vec![
        Contact { name: String::from("Alice"), phone: String::from("555-1234"), email: String::from("a@example.com") },
        Contact { name: String::from("Bob"),   phone: String::from("555-5678"), email: String::from("b@example.com") },
    ];
    println!("{} contacts", count_contacts(&contacts));
}
