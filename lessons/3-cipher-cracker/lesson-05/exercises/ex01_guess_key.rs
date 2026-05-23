// Exercise 4.1 — compute the Caesar key from a frequency peak
//
// If the most frequent letter in the ciphertext is `peak`, and we assume
// it was `'e'` before encryption, the key is:
//
//   key = (peak as u8 + 26 - b'e') % 26
//
// Implement `guess_key` so all tests pass.
// Work with lowercase characters only.

fn guess_key(peak: char) -> u8 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peak_h_gives_key_3() {
        // 'e' + 3 = 'h'
        assert_eq!(guess_key('h'), 3);
    }

    #[test]
    fn peak_l_gives_key_7() {
        // 'e' + 7 = 'l'
        assert_eq!(guess_key('l'), 7);
    }

    #[test]
    fn peak_e_gives_key_0() {
        // 'e' + 0 = 'e' (no shift)
        assert_eq!(guess_key('e'), 0);
    }

    #[test]
    fn peak_a_wraps_correctly() {
        // 'e' + 22 = 'a' (wraps around)
        assert_eq!(guess_key('a'), 22);
    }

    #[test]
    fn peak_b_gives_key_23() {
        // 'e' + 23 = 'b'
        assert_eq!(guess_key('b'), 23);
    }
}

fn main() {
    println!("peak 'h' → key {}", guess_key('h'));
    println!("peak 'l' → key {}", guess_key('l'));
    println!("peak 'a' → key {}", guess_key('a'));
}
