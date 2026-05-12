// Exercise 4.3 — if let for optional values
//
// `print_if_found` should print the contact's name if `result` is Some,
// and print "not found" otherwise.
//
// Use `if let` to implement it. All tests must pass.

struct Contact {
    name: String,
    phone: String,
    email: String,
}

fn print_if_found(contacts: &[Contact], result: Option<usize>) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make(name: &str) -> Contact {
        Contact { name: name.to_string(), phone: String::new(), email: String::new() }
    }

    #[test]
    fn some_prints_name() {
        let contacts = vec![make("Alice"), make("Bob")];
        assert_eq!(print_if_found(&contacts, Some(0)), "Alice");
        assert_eq!(print_if_found(&contacts, Some(1)), "Bob");
    }

    #[test]
    fn none_prints_not_found() {
        let contacts = vec![make("Alice")];
        assert_eq!(print_if_found(&contacts, None), "not found");
    }
}

fn main() {
    let contacts = vec![
        Contact { name: String::from("Alice"), phone: String::new(), email: String::new() },
    ];
    println!("{}", print_if_found(&contacts, Some(0)));
    println!("{}", print_if_found(&contacts, None));
}
