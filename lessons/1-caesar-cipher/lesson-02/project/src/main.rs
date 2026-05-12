// Caesar Cipher — step 2
//
// Add a single-character shift to the program.
//
// Expected output:
//
//   === Caesar Cipher ===
//   Message : Hello, World!
//   Key     : 3
//   'H' + 3 = 'K'
//
// Steps:
//   1. Declare `first_char` as the char 'H'.
//   2. Cast it to u8, add the key, cast back to char — store in `shifted`.
//   3. The final println! is already written; just make it compile.

fn main() {
    let message = "Hello, World!";
    let key: u8 = 3;

    let first_char = '?';           // TODO: change to the first letter of message
    let shifted = first_char;       // TODO: apply the shift

    println!("=== Caesar Cipher ===");
    println!("Message : {}", message);
    println!("Key     : {}", key);
    println!("'{}' + {} = '{}'", first_char, key, shifted);
}
