use crate::describable::Describable;

// TODO: Define a public struct Book with a public field 'title' of type String
pub struct Book {
    pub title: String
}
// TODO: Define a public struct Movie with a public field 'name' of type String
pub struct Movie {
    pub name: String
}
// TODO: Implement the Describable trait for Book
// The describe method should return "Book: {title}"
impl Describable for Book {
    fn describe(&self) -> String {
        format!("Book: {}",self.title)
    }
}
// TODO: Implement the Describable trait for Movie
// The describe method should return "Film: {name}"
impl Describable for Movie {
    fn describe(&self) -> String {
        format!("Film: {}", self.name)
    }
}
