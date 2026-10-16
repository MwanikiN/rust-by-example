mod shape;

use shape::Shape;

fn main() {
    // Read inputs
    let mut radius_input = String::new();
    std::io::stdin().read_line(&mut radius_input).expect("Failed to read line");
    let radius: f64 = radius_input.trim().parse().expect("Invalid number");

    let mut width_input = String::new();
    std::io::stdin().read_line(&mut width_input).expect("Failed to read line");
    let width: f64 = width_input.trim().parse().expect("Invalid number");

    let mut height_input = String::new();
    std::io::stdin().read_line(&mut height_input).expect("Failed to read line");
    let height: f64 = height_input.trim().parse().expect("Invalid number");

    // TODO: Create a Circle shape using the radius
    let circle: Shape = Shape::Circle(radius);
    // TODO: Create a Rectangle shape using width and height
    let rectangle: Shape = Shape::Rectangle{width, height};
    // TODO: Print the area of each shape with one decimal place
    // Format: "Circle area: {area}" and "Rectangle area: {area}"
    println!("Circle area: {:.1}",circle.area());
    println!("Rectangle area: {:.1}",rectangle.area());
}
