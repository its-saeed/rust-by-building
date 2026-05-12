// Exercise 3.1 — impl block syntax
//
// An `impl` block attaches functions to a type.
// This code does not compile. The `display` function is defined
// outside the `impl` block but uses `self`. Move it inside.

struct Contact {
    name: String,
    phone: String,
    email: String,
}

impl Contact {
    // display should be in here
}

fn display(&self) {   // BUG: this is outside the impl block
    println!("{} | {} | {}", self.name, self.phone, self.email);
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
