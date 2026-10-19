mod processor;

use processor::{Summarize, Validate, process_items};
use std::io;

// TODO: Define a public struct Article with a public title field (String)
// Derive Clone for Article
#[derive(Clone)]
pub struct Article {
    pub title: String
}
// TODO: Implement Summarize for Article
// The summary method should return "Article: {title}"
impl Summarize for Article {
    fn summary(&self) -> String {
        format!("{}",self.title)
    }
}
// TODO: Define a public struct Form with a public filled field (bool)
pub struct Form {
    pub filled: bool
}
// TODO: Implement Validate for Form
// The validate method should return the value of filled
impl Validate for Form {
    fn validate(&self) -> bool {
        self.filled
    }
}

fn main() {
    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Failed to read line");
    let title = title.trim().to_string();

    let mut filled_input = String::new();
    io::stdin().read_line(&mut filled_input).expect("Failed to read line");
    let filled: bool = filled_input.trim().parse().expect("Failed to parse bool");

    // TODO: Create an Article with the given title
    let article: Article = Article {title};
    // TODO: Create a Form with the given filled value
    let form: Form = Form {filled};
    // TODO: Call process_items with the article and form
    process_items(article.clone(), form);
}
