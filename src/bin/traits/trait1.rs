mod point;

use point::Point;

fn main() {
    // Read inputs
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let x1: i32 = input.trim().parse().expect("Invalid input");
    
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let y1: i32 = input.trim().parse().expect("Invalid input");
    
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let dx: i32 = input.trim().parse().expect("Invalid input");
    
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let dy: i32 = input.trim().parse().expect("Invalid input");
    
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let x2: f64 = input.trim().parse().expect("Invalid input");
    
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let y2: f64 = input.trim().parse().expect("Invalid input");
    
    // TODO: Create an integer point using x1 and y1
    let mut int_point: Point<i32> = Point::new(x1, y1);
    // TODO: Translate the integer point by dx and dy
    println!("Integer point: ({}, {})", int_point.x, int_point.y);
    let result = int_point.translate(dx, dy);
    println!("After translation: ({}, {})", result.x, result.y);
    // TODO: Create a floating-point point using x2 and y2
    let float_point: Point<f64> = Point::new(x2, y2);
    // TODO: Print the results in the required format:
    
    println!("Float point: ({}, {})", float_point.x, float_point.y);
}
