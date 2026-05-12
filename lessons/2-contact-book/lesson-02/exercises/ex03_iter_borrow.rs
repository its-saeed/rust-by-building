// Exercise 2.3 — iterate by borrowing, not by moving
//
// `for contact in contacts` moves each Contact out of the Vec.
// After the loop, `contacts` is gone and `.len()` fails.
//
// Fix the loop so the Vec is still usable after it.
// Hint: borrow the Vec with `&`.

struct Contact {
    name: String,
    phone: String,
    email: String,
}

fn main() {
    let mut contacts: Vec<Contact> = Vec::new();
    contacts.push(Contact {
        name: String::from("Alice"),
        phone: String::from("555-1234"),
        email: String::from("alice@example.com"),
    });
    contacts.push(Contact {
        name: String::from("Bob"),
        phone: String::from("555-5678"),
        email: String::from("bob@example.com"),
    });

    for contact in contacts {   // BUG: moves contacts
        println!("{}", contact.name);
    }

    println!("total: {}", contacts.len());   // fails because contacts was moved
}

// No assertions — if it compiles and prints all names then "total: 2", you've solved it.
