mod coordinates;

use coordinates::{GridPoint, NamedLocation};

fn main() {
    // Read inputs
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Failed to read line");
    let x: i32 = input1.trim().parse().expect("Invalid number");

    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Failed to read line");
    let y: i32 = input2.trim().parse().expect("Invalid number");

    let mut input3 = String::new();
    std::io::stdin().read_line(&mut input3).expect("Failed to read line");
    let name = input3.trim().to_string();

    // TODO: Create a GridPoint instance with x and y
    let grid_point: GridPoint = GridPoint {x, y};
    // TODO: Demonstrate implicit copy by assigning to another variable
    // (GridPoint implements Copy, so this creates a copy automatically)
    let grid_point2: GridPoint = grid_point;
    // TODO: Print original and copied point
    // Format: "Original point: ({x}, {y})"
    // Format: "Copied point: ({x}, {y})"
    println!("Original point: ({}, {})",grid_point.x, grid_point.y);
    println!("Copied point: ({}, {})",grid_point2.x, grid_point2.y);
    // TODO: Create a NamedLocation instance with name, x, and y
    let named_location: NamedLocation = NamedLocation {name, x, y};
    // TODO: Demonstrate explicit clone (NamedLocation only implements Clone, not Copy)
    // Use .clone() to create a duplicate
    let named_location2: NamedLocation = named_location.clone();
    // TODO: Print original and cloned location
    // Format: "Original location: {name} at ({x}, {y})"
    // Format: "Cloned location: {name} at ({x}, {y})"
    println!("Original location: {} at ({}, {})",named_location.name, named_location.x, named_location.y);
    println!("Cloned location: {} at ({}, {})",named_location2.name, named_location2.x, named_location2.y);
}
