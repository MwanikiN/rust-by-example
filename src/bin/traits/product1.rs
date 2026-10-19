use std::fmt;
use std::fmt::Display;

// TODO: Define a public Inspectable trait with an inspect method
// The inspect method should take &self and return a String
pub trait Inspectable {
    fn inspect(&self) -> String;
}

// TODO: Define a public Gadget struct with public fields:
// - name: String
// - serial: u32
pub struct Gadget {
    pub name: String,
    pub serial: u32
}

// TODO: Implement Inspectable for Gadget
// The inspect method should return "Inspecting: {name}"
impl Inspectable for Gadget {
    fn inspect(&self) -> String {
        format!("Inspecting: {}",self.name)
    }
}

// TODO: Implement std::fmt::Display for Gadget
// Format as "{name} (SN: {serial})"
impl fmt::Display for Gadget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (SN: {})",self.name, self.serial)
    }
}

// TODO: Create a public generic function full_report<T>
// The function should accept any type T that implements both Display and Inspectable
// It should print two lines:
// 1. The item using {} formatter (Display)
// 2. The result of calling inspect()
pub fn full_report<T: Display + Inspectable>(item: T){
    println!("{}",item);
    println!("{}",item.inspect());
}

