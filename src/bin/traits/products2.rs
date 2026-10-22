// mod catalog;
// mod items;

use rust_by_example::lib::catalog::CatalogItem;
use rust_by_example::lib::items1::{Book, Magazine};

fn main() {
    // Read inputs
    let mut book_title = String::new();
    std::io::stdin().read_line(&mut book_title).expect("Failed to read line");
    let book_title = book_title.trim().to_string();

    let mut book_author = String::new();
    std::io::stdin().read_line(&mut book_author).expect("Failed to read line");
    let book_author = book_author.trim().to_string();

    let mut magazine_name = String::new();
    std::io::stdin().read_line(&mut magazine_name).expect("Failed to read line");
    let magazine_name = magazine_name.trim().to_string();

    let mut issue_input = String::new();
    std::io::stdin().read_line(&mut issue_input).expect("Failed to read line");
    let magazine_issue: u32 = issue_input.trim().parse().expect("Failed to parse issue number");

    // TODO: Create a Book instance using book_title and book_author
    let book: Book = Book {title: book_title, author: book_author};
    // TODO: Create a Magazine instance using magazine_name and magazine_issue
    let magazine: Magazine = Magazine {name: magazine_name, issue: magazine_issue};
    // TODO: Call description() on each item and print the results
    println!("{}",book.description());
    println!("{}",magazine.description());
}
