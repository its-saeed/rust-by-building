// Contact Book — step 2
//
// Extend the program from step 1.
// Add a Vec<Contact> and push three contacts into it.
// Then loop over the Vec and print each contact.
//
// Expected output:
//
//   === Contact Book (3 contacts) ===
//     Alice | 555-1234 | alice@example.com
//     Bob | 555-5678 | bob@example.com
//     Carol | 555-9012 | carol@example.com
//
// Fill in the TODOs. Do not change the println! header line format.

struct Contact {
    name: String,
    phone: String,
    email: String,
}

fn main() {
    let mut contacts: Vec<Contact> = Vec::new();

    // TODO: push three Contact values into `contacts`

    println!("=== Contact Book ({} contacts) ===", contacts.len());

    for contact in &contacts {
        // TODO: print each contact in the format:  Name | Phone | Email
    }
}
