// mod notifications;

use rust_by_example::lib::notifications::{Notify, Email, Sms, Push};

fn main() {
    // Read inputs
    let mut recipient = String::new();
    std::io::stdin().read_line(&mut recipient).expect("Failed to read line");
    let recipient = recipient.trim().to_string();

    let mut phone = String::new();
    std::io::stdin().read_line(&mut phone).expect("Failed to read line");
    let phone = phone.trim().to_string();

    let mut device = String::new();
    std::io::stdin().read_line(&mut device).expect("Failed to read line");
    let device = device.trim().to_string();

    // TODO: Create a Vec<Box<dyn Notify>> to hold different notification types
    let mut notification: Vec<Box<dyn Notify>> = vec![];
    // TODO: Create Email, Sms, and Push instances using the inputs
    let email: Box<dyn Notify> = Box::new(Email {recipient});
    notification.push(email);
    let sms: Box<dyn Notify> = Box::new(Sms {phone});
    notification.push(sms);
    let push: Box<dyn Notify> = Box::new(Push {device});
    notification.push(push);
    // TODO: Add them to the vector in order (Email, Sms, Push) using Box::new()
    // TODO: Print the result of calling send() on the first element

    println!("{}",notification[0].send());
}
