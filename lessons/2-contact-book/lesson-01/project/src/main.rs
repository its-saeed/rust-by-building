// Contact Book — step 1
//
// Define a struct called `Contact` with three String fields:
//   name, phone, email
//
// Then create one contact and make this program print:
//
//   === Contact Book ===
//   Name  : Alice
//   Phone : 555-1234
//   Email : alice@example.com
//
// Fill in the struct definition and the values below.
// Do not change the println! lines.

struct Contact {
    // TODO: add three fields here
}

fn main() {
    let contact = Contact {
        // TODO: fill in the fields
    };

    println!("=== Contact Book ===");
    println!("Name  : {}", contact.name);
    println!("Phone : {}", contact.phone);
    println!("Email : {}", contact.email);
}
