// Exercise 2 — define and implement a custom trait
//
// Define a trait `Summary` with one required method:
//
//   fn summarize(&self) -> String;
//
// Then implement it for both `Article` and `Tweet` below.
//
// `Article::summarize` should return:
//   "<title>, by <author>"
//
// `Tweet::summarize` should return:
//   "@<username>: <content>"

struct Article {
    title: String,
    author: String,
}

struct Tweet {
    username: String,
    content: String,
}

// TODO: define the Summary trait

// TODO: implement Summary for Article

// TODO: implement Summary for Tweet

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn article_summary() {
        let a = Article {
            title: String::from("Rust is great"),
            author: String::from("Alice"),
        };
        assert_eq!(a.summarize(), "Rust is great, by Alice");
    }

    #[test]
    fn tweet_summary() {
        let t = Tweet {
            username: String::from("bob"),
            content: String::from("Hello world"),
        };
        assert_eq!(t.summarize(), "@bob: Hello world");
    }
}

fn main() {
    let a = Article {
        title: String::from("Rust is great"),
        author: String::from("Alice"),
    };
    let t = Tweet {
        username: String::from("bob"),
        content: String::from("Hello world"),
    };
    println!("{}", a.summarize());
    println!("{}", t.summarize());
}
