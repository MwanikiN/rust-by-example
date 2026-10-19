// TODO: Define a public trait called Notify
// It should have a method called message that takes &self and returns a String

pub trait Notify {
    fn message(&self)-> String;
}
// TODO: Define a public struct Email with a public field 'subject' of type String
pub struct Email {
    pub subject: String
}

// TODO: Implement the Notify trait for Email
// The message method should return "Email: {subject}"
impl Notify for Email {
    fn message(&self) -> String {
        format!("Email: {}",self.subject)
    }
}

// TODO: Define a public struct SMS with a public field 'content' of type String
pub struct SMS {
    pub content: String
}

// TODO: Implement the Notify trait for SMS
// The message method should return "SMS: {content}"
impl Notify for SMS {
    fn message(&self) -> String {
        format!("SMS: {}",self.content)
    }
}

// TODO: Create a public generic function called send_notification
// It should accept any type T that implements the Notify trait
// The function should print the result of calling message() on the item

pub fn send_notification<T: Notify>(item: T) {
    println!("{}",item.message())
}
