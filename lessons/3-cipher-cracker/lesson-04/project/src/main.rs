// Cipher Cracker — lesson 3 (Enums)
//
// Define the two enums that the rest of the project will use.
//
// TODO 1: Define `CrackResult` with two variants:
//   Success { key: u8, plaintext: String }
//   TooFewLetters
//
// TODO 2: Define `Command` with two variants:
//   Crack(String)   — a ciphertext to crack
//   Quit            — exit the program
//
// TODO 3: Implement `fn describe(result: &CrackResult) -> String`
//   - Success: "Key N: <plaintext>"
//   - TooFewLetters: "Not enough letters to analyse."
//
// TODO 4: Implement `fn parse_command(input: &str) -> Command`
//   - If input (trimmed) is "quit" or "q", return Command::Quit
//   - Otherwise return Command::Crack with the trimmed input as a String
//
// When done, main should print:
//   Key 3: alice was beginning to get very tired
//   Not enough letters to analyse.
//   Got command: crack "hello"
//   Got command: quit

fn main() {
    // Test CrackResult::describe
    let success = CrackResult::Success {
        key: 3,
        plaintext: String::from("alice was beginning to get very tired"),
    };
    println!("{}", describe(&success));

    let failure = CrackResult::TooFewLetters;
    println!("{}", describe(&failure));

    // Test Command::parse_command
    let cmd1 = parse_command("hello");
    let cmd2 = parse_command("quit");

    match cmd1 {
        Command::Crack(text) => println!("Got command: crack {:?}", text),
        Command::Quit        => println!("Got command: quit"),
    }
    match cmd2 {
        Command::Crack(text) => println!("Got command: crack {:?}", text),
        Command::Quit        => println!("Got command: quit"),
    }
}
