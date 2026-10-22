// TODO: Define a public Notify trait with a send method
// The send method should take &self and return a String
pub trait Notify {
    fn send(&self) -> String;
}

// TODO: Create a public Email struct with a public recipient field (String)
// Implement the Notify trait for Email
// send() should return "Email to: {recipient}"
pub struct Email {
    pub recipient: String
}

impl Notify for Email {
    fn send(&self) -> String {
        format!("Email to: {}",self.recipient)
    }
}

// TODO: Create a public Sms struct with a public phone field (String)
// Implement the Notify trait for Sms
// send() should return "SMS to: {phone}"
pub struct Sms {
    pub phone: String
}

impl Notify for Sms {
    fn send(&self) -> String {
        format!("SMS to: {}",self.phone)
    }
}

// TODO: Create a public Push struct with a public device field (String)
// Implement the Notify trait for Push
// send() should return "Push to: {device}"
pub struct Push {
    pub device: String
}

impl Notify for Push {
    fn send(&self) -> String {
        format!("Push to: {}",self.device)
    }
}
