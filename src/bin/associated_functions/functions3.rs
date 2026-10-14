use std::io;

// TODO: Define the Rectangle struct here
struct Rectangle {
    width: u32,
    height: u32
}


// TODO: Add the implementation block for Rectangle with:
// - new: associated function that takes width and height, returns Rectangle
// - area: method that takes &self, returns u32
// - scale: method that takes &mut self and factor, multiplies width and height
impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}

fn main() {
    let mut input = String::new();
    
    // Read width
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let width: u32 = input.trim().parse().expect("Invalid number");
    
    input.clear();
    
    // Read height
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let height: u32 = input.trim().parse().expect("Invalid number");
    
    input.clear();
    
    // Read scale factor
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let factor: u32 = input.trim().parse().expect("Invalid number");
    
    // TODO: Create a mutable Rectangle using Rectangle::new
    let mut rect: Rectangle = Rectangle::new(width, height);
    
    // TODO: Print the initial area
    println!("{}", rect.area());
    
    // TODO: Scale the rectangle by the factor
    rect.scale(factor);
    let new_area = rect.area();
    // TODO: Print the new area
    println!("{}",new_area);
}