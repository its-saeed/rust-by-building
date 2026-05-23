// Exercise 3.4 — the CrackResult enum
//
// Define `CrackResult` with two variants:
//   Success { key: u8, plaintext: String }
//   TooFewLetters
//
// Implement `fn describe(result: &CrackResult) -> String`:
//   - For Success: "Key N: <plaintext>"  (e.g. "Key 3: hello world")
//   - For TooFewLetters: "Not enough letters to analyse."
//
// All tests must pass.

// TODO: define CrackResult

// TODO: implement describe

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describes_success() {
        let r = CrackResult::Success {
            key: 3,
            plaintext: String::from("hello world"),
        };
        assert_eq!(describe(&r), "Key 3: hello world");
    }

    #[test]
    fn describes_too_few() {
        let r = CrackResult::TooFewLetters;
        assert_eq!(describe(&r), "Not enough letters to analyse.");
    }

    #[test]
    fn key_zero() {
        let r = CrackResult::Success {
            key: 0,
            plaintext: String::from("abc"),
        };
        assert_eq!(describe(&r), "Key 0: abc");
    }
}

fn main() {
    let success = CrackResult::Success {
        key: 3,
        plaintext: String::from("alice was beginning to get very tired"),
    };
    let failure = CrackResult::TooFewLetters;

    println!("{}", describe(&success));
    println!("{}", describe(&failure));
}
