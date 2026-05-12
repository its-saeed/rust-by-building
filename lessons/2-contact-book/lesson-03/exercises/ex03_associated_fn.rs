// Exercise 3.3 — associated functions are called with `::`
//
// `Contact::new(...)` is an associated function — it does not take `self`.
// It is called with `::`, not with `.`.
// Fix the call in `main` so it compiles.

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
}

fn main() {
    let c = Contact.new("Alice", "555-1234", "alice@example.com");  // BUG: wrong syntax
    println!("{}", c.name);
}
