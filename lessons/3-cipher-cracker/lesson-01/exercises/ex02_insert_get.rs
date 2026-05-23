// Exercise 1.2 — .get() takes a reference to the key
//
// `HashMap::get` takes `&K` — a reference to the key type.
// This does not compile because the key is passed by value, not by reference.
// Fix the `.get()` call.

use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_existing_key() {
        let mut map = HashMap::new();
        map.insert('e', 5u32);
        assert_eq!(map.get(&'e'), Some(&5));
    }

    #[test]
    fn get_missing_key() {
        let map: HashMap<char, u32> = HashMap::new();
        assert_eq!(map.get(&'z'), None);
    }
}

fn main() {
    let mut map: HashMap<char, u32> = HashMap::new();
    map.insert('h', 8);
    map.insert('w', 5);

    let count = map.get('h');   // BUG: should be &'h'
    println!("{:?}", count);
}
