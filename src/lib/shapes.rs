use super::shape::Shape;

// TODO: Define the Circle struct with a radius field (f64)
pub struct Circle {
    // TODO: Add the radius field
    pub radius: f64
}

// TODO: Implement the Shape trait for Circle
// Area formula: 3.14159 × radius × radius
impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }
}

// TODO: Define the Square struct with a side field (f64)
pub struct Square {
    // TODO: Add the side field
    pub side: f64
}

// TODO: Implement the Shape trait for Square
// Area formula: side × side
impl Shape for Square {
    // TODO: Implement the area method
    fn area(&self) -> f64 {
        self.side * self.side
    }
}
