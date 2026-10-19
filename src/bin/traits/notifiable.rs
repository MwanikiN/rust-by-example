// TODO: Define a public trait called Notifiable
// The trait should have a method: notify(&self) -> String
// This method should have a DEFAULT implementation
// that returns "You have a new notification!"
pub trait Notifiable {
    fn notify(&self) -> String {
        String::from("You have a new notification!")
    }
}