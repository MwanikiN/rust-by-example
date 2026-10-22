use super::catalog::CatalogItem;

// TODO: Define the Book struct with 'title' and 'author' fields (both String)
pub struct Book {
    // TODO: Add fields
    pub title: String,
    pub author: String
}

// TODO: Define the Magazine struct with 'name' (String) and 'issue' (u32) fields
pub struct Magazine {
    // TODO: Add fields
    pub name: String,
    pub issue: u32
}

// TODO: Implement CatalogItem trait for Book
// The description should return: "Book: {title} by {author}"
impl CatalogItem for Book {
    // TODO: Implement the description method
    fn description(&self) -> String {
        format!("Book: {} by {}",self.title, self.author)
    }
}

// TODO: Implement CatalogItem trait for Magazine
// The description should return: "Magazine: {name}, Issue {issue}"
impl CatalogItem for Magazine {
    // TODO: Implement the description method
    fn description(&self) -> String {
        format!("Magazine: {}, Issue {}",self.name, self.issue)
    }
}
