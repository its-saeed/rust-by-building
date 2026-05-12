// Final Exercise 3 — crack the cipher
//
// The message below was encrypted with an unknown key (somewhere between 0 and 25).
// Your job: try all 26 possible keys and print each result.
// One of them will be readable English — that's the original message.
//
// Instructions:
//   1. Copy your working shift_char, encrypt, and decrypt from ex01.
//   2. Write a for loop: for key in 0u8..26 { ... }
//   3. Inside the loop, decrypt CIPHERTEXT with the current key and print:
//        Key  3: <decrypted text>
//   4. Run the program. Read the output. Find the key.
//
// There are no tests — your own eyes are the verifier.

const CIPHERTEXT: &str = "Rcple! Jzf ufde nclnvpo esp ntaspc.";

// TODO: paste shift_char, encrypt, decrypt here

fn main() {
    // TODO: loop over all 26 keys and print each decryption candidate
}
