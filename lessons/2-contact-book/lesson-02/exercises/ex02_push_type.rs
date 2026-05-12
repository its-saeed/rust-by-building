// Exercise 2.2 — a Vec<Contact> only accepts Contact values
//
// The type of the vector is `Vec<Contact>`.
// Something wrong is being pushed in. Fix it.

struct Contact {
    name: String,
    phone: String,
    email: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn holds_one_contact() {
        let mut contacts: Vec<Contact> = Vec::new();
        contacts.push(Contact {
            name: String::from("Alice"),
            phone: String::from("555-1234"),
            email: String::from("alice@example.com"),
        });
        assert_eq!(contacts.len(), 1);
        assert_eq!(contacts[0].name, "Alice");
    }
}

fn main() {
    let mut contacts: Vec<Contact> = Vec::new();

    contacts.push("Alice");   // BUG: wrong type

    println!("{} contacts", contacts.len());
}
