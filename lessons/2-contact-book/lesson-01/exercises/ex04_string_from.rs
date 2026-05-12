// Exercise 1.4 — struct fields that are `String` need owned strings
//
// The field type is `String`, but `"Alice"` is a `&str`.
// Rust will not automatically convert between them.
// Fix the struct literal so it compiles.
//
// Hint: use `String::from(...)` or `"...".to_string()`.

struct Contact {
    name: String,
    phone: String,
    email: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fields_accessible() {
        let c = Contact {
            name: String::from("Bob"),
            phone: String::from("555-5678"),
            email: String::from("bob@example.com"),
        };
        assert_eq!(c.name, "Bob");
        assert_eq!(c.phone, "555-5678");
        assert_eq!(c.email, "bob@example.com");
    }
}

fn main() {
    let c = Contact {
        name: "Alice",                   // BUG: wrong type
        phone: "555-1234",               // BUG: wrong type
        email: "alice@example.com",      // BUG: wrong type
    };
    println!("{} — {} — {}", c.name, c.phone, c.email);
}
