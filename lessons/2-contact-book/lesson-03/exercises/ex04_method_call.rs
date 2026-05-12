// Exercise 3.4 — implement `new` and `display`
//
// The `impl Contact` block is empty. Implement:
//   - `new(name: &str, phone: &str, email: &str) -> Contact`
//   - `display(&self)` — prints "  name | phone | email"
//
// All tests must pass.

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_works() {
        let c = Contact::new("Alice", "555-1234", "alice@example.com");
        assert_eq!(c.name, "Alice");
        assert_eq!(c.phone, "555-1234");
        assert_eq!(c.email, "alice@example.com");
    }

    #[test]
    fn new_with_to_string() {
        // new should also work when given owned strings (via deref coercion)
        let name = String::from("Bob");
        let c = Contact::new(&name, "555-5678", "bob@example.com");
        assert_eq!(c.name, "Bob");
    }
}

fn main() {
    let c = Contact::new("Alice", "555-1234", "alice@example.com");
    c.display();
}
