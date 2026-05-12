// Exercise 5.4 — dispatching string commands with match
//
// `run_command` receives a trimmed command string and returns a description
// of what that command does. Complete the match arms for "find" and "quit".
// The wildcard arm is already there.
//
// All tests must pass.

fn run_command(cmd: &str) -> &str {
    match cmd {
        "add"  => "adding a contact",
        "list" => "listing all contacts",
        // TODO: handle "find" → "finding a contact"
        // TODO: handle "quit" → "quitting"
        _      => "unknown command",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_commands() {
        assert_eq!(run_command("add"),  "adding a contact");
        assert_eq!(run_command("list"), "listing all contacts");
        assert_eq!(run_command("find"), "finding a contact");
        assert_eq!(run_command("quit"), "quitting");
        assert_eq!(run_command("nope"), "unknown command");
        assert_eq!(run_command(""),     "unknown command");
    }
}

fn main() {
    println!("{}", run_command("add"));
    println!("{}", run_command("quit"));
    println!("{}", run_command("??"));
}
