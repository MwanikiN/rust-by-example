mod point1;

use point1::Point;

fn main() {
    // Read four inputs: x1, y1, x2, y2
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let x1: i32 = input.trim().parse().expect("Invalid input");

    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let y1: i32 = input.trim().parse().expect("Invalid input");

    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let x2: i32 = input.trim().parse().expect("Invalid input");

    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let y2: i32 = input.trim().parse().expect("Invalid input");

    // TODO: Create two Point instances using the inputs
    let point1: Point = Point{x: x1, y: y1};
    let point2: Point = Point{x: x2, y: y2};
    // TODO: Print the first point using debug format {:?}
    // Format: "Debug: Point { x: ..., y: ... }"
    println!("Debug: {:?}",point1);
    // TODO: Print the second point using display format {}
    // Format: "Display: (..., ...)"
    println!("Display: {}", point2);
    // TODO: Compare the two points and print the result
    // Format: "Equal: true" or "Equal: false"
    println!("Equal: {}",point1 == point2);
}
