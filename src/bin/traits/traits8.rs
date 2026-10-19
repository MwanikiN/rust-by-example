mod notification;

use notification::{Email, SMS, send_notification};

fn main() {
    // Read inputs
    let mut subject = String::new();
    std::io::stdin().read_line(&mut subject).expect("Failed to read line");
    let subject = subject.trim().to_string();
    
    let mut content = String::new();
    std::io::stdin().read_line(&mut content).expect("Failed to read line");
    let content = content.trim().to_string();
    
    // TODO: Create an Email instance with the subject
    let email: Email = Email{subject};
    // TODO: Create an SMS instance with the content
    let sms: SMS = SMS {content};
    // TODO: Call send_notification with the email
    send_notification(email);
    // TODO: Call send_notification with the sms
    send_notification(sms);
}
