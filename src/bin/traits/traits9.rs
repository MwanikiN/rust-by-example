mod product1;

use product1::{Gadget, full_report};

fn main() {
    // Read inputs
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim().to_string();
    
    let mut serial_input = String::new();
    std::io::stdin().read_line(&mut serial_input).expect("Failed to read line");
    let serial: u32 = serial_input.trim().parse().expect("Failed to parse serial");
    
    // TODO: Create a Gadget instance with the name and serial
    let gadget: Gadget = Gadget {name, serial};
    // TODO: Call full_report with your gadget
    full_report(gadget);
}
