// Exercise 1.1 — HashMap requires a `use` declaration
//
// `HashMap` lives in `std::collections`. Without bringing it into scope,
// Rust does not know what `HashMap` means.
//
// Add the missing `use` line at the top so this compiles.

fn main() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("hello", 1);
    println!("{:?}", map.get("hello"));
}

// No assertions — if it compiles and prints Some(1), you've solved it.
