// Exercise 2.4 — know your ASCII values
//
// The tests below have wrong expected values.
// Using what you know about ASCII (A=65, a=97, each letter is +1),
// fix the numbers in the assertions. Do not change anything else.

fn main() {
    println!("'E' as u8 = {}", 'E' as u8);
    println!("'e' as u8 = {}", 'e' as u8);
    println!("'Z' as u8 = {}", 'Z' as u8);
}

#[cfg(test)]
mod tests {
    #[test]
    fn uppercase_e() {
        assert_eq!('E' as u8, 60);  // wrong — fix this
    }

    #[test]
    fn lowercase_e() {
        assert_eq!('e' as u8, 92);  // wrong — fix this
    }

    #[test]
    fn uppercase_z() {
        assert_eq!('Z' as u8, 80);  // wrong — fix this
    }
}
