// Caesar Cipher — step 3
//
// Move the shift logic into a function called `shift_char`.
//
// Expected output:
//
//   === Caesar Cipher ===
//   Message : Hello, World!
//   Key     : 3
//   'H' + 3 = 'K'
//
// Instructions:
//   1. Write a function `shift_char(c: char, key: u8) -> char` above main.
//      It should shift a letter by key positions. Non-letters stay unchanged.
//   2. In main, call shift_char('H', key) instead of doing the cast inline.
//   3. Run `cargo run` and confirm the output matches.

fn main() {
    let message = "Hello, World!";
    let key: u8 = 3;
    let first_char = 'H';

    // TODO: replace this line with a call to shift_char
    let shifted = (first_char as u8 + key) as char;

    println!("=== Caesar Cipher ===");
    println!("Message : {}", message);
    println!("Key     : {}", key);
    println!("'{}' + {} = '{}'", first_char, key, shifted);
}
