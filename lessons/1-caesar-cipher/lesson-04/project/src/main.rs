// Caesar Cipher — step 4
//
// Add the `encrypt` function and call it on the full message.
//
// Expected output:
//
//   === Caesar Cipher ===
//   Message   : Hello, World!
//   Key       : 3
//   Encrypted : Khoor, Zruog!
//
// Instructions:
//   1. Copy your `shift_char` function from step 3 (it goes above main).
//   2. Write an `encrypt(text: &str, key: u8) -> String` function that
//      loops over text.chars() and pushes each shifted char into a String.
//   3. Call encrypt(message, key) in main and store the result.
//   4. Update the final println! to print the encrypted message.

fn main() {
    let message = "Hello, World!";
    let key: u8 = 3;

    // TODO: call encrypt(message, key) and store the result in `encrypted`

    println!("=== Caesar Cipher ===");
    println!("Message   : {}", message);
    println!("Key       : {}", key);
    println!("Encrypted : {}", message);   // TODO: replace message with encrypted
}
