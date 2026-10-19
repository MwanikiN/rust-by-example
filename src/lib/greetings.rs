// TODO: Define a public Greet trait with a greet method
// The greet method should take &self and return a String
pub trait Greet {
    fn greet(&self) -> String;
}

// TODO: Create a Card struct (doesn't need to be public!)
// It should have a message field of type String
struct Card {
    message: String
}

// TODO: Implement the Greet trait for Card
// Return the message when greet is called
impl Greet for Card {
    fn greet(&self) -> String {
        format!("{}",self.message)
    }
}

// TODO: Create a public function called create_greeting
// It should take a String parameter and return impl Greet
// This function creates and returns a Card with the given message
pub fn create_greeting(parameter: String) -> impl Greet {
    Card {message: parameter}
}
