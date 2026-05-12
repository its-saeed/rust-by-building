// Exercise 3.2 — methods need `&self`
//
// A method must take `&self` as its first parameter to access the
// struct's fields. This function is missing it.
// Fix it so `c.display()` compiles.

struct Contact {
    name: String,
    phone: String,
    email: String,
}

impl Contact {
    fn display() {   // BUG: missing &self
        println!("{} | {} | {}", self.name, self.phone, self.email);
    }
}

fn main() {
    let c = Contact {
        name: String::from("Alice"),
        phone: String::from("555-1234"),
        email: String::from("alice@example.com"),
    };
    c.display();
}

// No assertions — if it compiles and prints the contact, you've solved it.
