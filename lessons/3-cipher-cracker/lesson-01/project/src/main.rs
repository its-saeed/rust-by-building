// Cipher Cracker — step 1
//
// Implement `count_letters` using a HashMap.
// It should count every character in `text` (spaces included).
//
// Then call it on the ciphertext below and print each character and its count.
// The order does not need to be sorted — HashMap order is arbitrary.
//
// Hint: use `*freq.entry(ch).or_insert(0) += 1`

use std::collections::HashMap;

fn count_letters(text: &str) -> HashMap<char, u32> {
    // TODO: implement
    todo!()
}

fn main() {
    let ciphertext = "dolfh zdv ehjlqqlqj wr jhw yhub wluhg ri vlwwlqj eb khu vlvwhu rq wkh edqn";

    println!("=== Cipher Cracker ===");
    println!("Ciphertext: {:?}\n", ciphertext);

    let freq = count_letters(ciphertext);

    println!("Character counts:");
    for (ch, count) in &freq {
        println!("  {:?}: {}", ch, count);
    }
}
