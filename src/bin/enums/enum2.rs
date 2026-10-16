// TODO: Define a public Shape enum with two variants:
// - Circle: holds a single f64 (radius) using tuple syntax
// - Rectangle: holds named fields width (f64) and height (f64)

pub enum Shape {
    // TODO: Define Circle variant with tuple syntax
    Circle(f64),
    // TODO: Define Rectangle variant with named fields
    Rectangle {width: f64, height: f64}
}

impl Shape {
    // TODO: Implement the area method that returns f64
    // Use pattern matching to destructure each variant
    // Circle area: 3.14159 * radius * radius
    // Rectangle area: width * height
    pub fn area(&self) -> f64 {
        // TODO: Match on self and calculate the appropriate area
        match self {
            Shape::Circle(radius) => 3.14159 * radius * radius,
            Shape::Rectangle{width, height} => width * height
        }
    }
}
