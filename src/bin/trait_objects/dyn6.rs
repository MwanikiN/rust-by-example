// mod shape;
// mod shapes;

use rust_by_example::lib::shape::Shape;
use rust_by_example::lib::shapes::{Circle, Square};

// TODO: Implement the total_area function
// It should accept a Vec<Box<dyn Shape>> and return the sum of all areas as f64
fn total_area(shapes: Vec<Box<dyn Shape>>) -> f64 {
    // TODO: Calculate and return the sum of all shape areas
    shapes.iter().map(|shape| shape.area()).sum()
}

fn main() {
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Failed to read line");
    let radius: f64 = input1.trim().parse().expect("Invalid number");

    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Failed to read line");
    let side: f64 = input2.trim().parse().expect("Invalid number");

    // TODO: Create a Circle with the given radius
    let circle: Box<dyn Shape> = Box::new(Circle {radius});
    // TODO: Create a Square with the given side
    let square: Box<dyn Shape> = Box::new(Square {side});
    // TODO: Store both shapes in a Vec<Box<dyn Shape>>
    let mut shapes: Vec<Box<dyn Shape>> = vec![];
    shapes.push(circle);
    shapes.push(square);
    // TODO: Call total_area and print the result in the format: "Total area: {total}"
    println!("Total area: {}",total_area(shapes));
}
