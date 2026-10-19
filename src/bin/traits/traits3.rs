mod notifiable;
mod alerts;

use notifiable::Notifiable;
use alerts::{EmailAlert, SystemAlert};

fn main() {
    // TODO: Create an instance of EmailAlert
    let email_alert = EmailAlert;
    // TODO: Create an instance of SystemAlert
    let system_alert = SystemAlert;
    // TODO: Print the notification from EmailAlert
    println!("{}", email_alert.notify());
    // TODO: Print the notification from SystemAlert
    println!("{}", system_alert.notify());
}
